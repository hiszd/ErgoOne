#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![feature(generic_const_exprs)]

mod actions;
mod key;
mod key_codes;
mod key_mapping;
mod keyscanning;
mod macros;
mod mods;
mod util;

use core::sync::atomic::AtomicBool;
use core::{
    cell::RefCell,
    sync::atomic::{AtomicU8, Ordering},
};

use crate::{key_codes::KeyCode, pac::interrupt};
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use heapless::String;
use keyscanning::{Col, Row};
use kiibohd_hid_io::{CommandInterface, HidIoCommandId, KiibohdCommandInterface};
use kiibohd_usb::KeyState;
use panic_probe as _;
use util::hid_descriptor::KeyboardNkroReport;

use critical_section::Mutex;
use heapless::spsc::{Producer, Queue};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::UsbDevice,
    UsbError,
};
use usbd_hid::hid_class::HidCountryCode;

use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    multicore::{Multicore, Stack},
    pac,
    prelude::_rphal_pio_PIOExt,
    sio::Sio,
    usb::UsbBus,
    watchdog::Watchdog,
    Timer,
};
use ws2812_pio::Ws2812;

use crate::keyscanning::StateType;
use crate::keyscanning::{Matrix, Operation};

use self::actions::CallbackActions;
use self::keyscanning::KeyQueue;

// These define the maximum pending items in each queue
const KBD_QUEUE_SIZE: usize = 10; // This would limit NKRO mode to 10KRO
const KBD_LED_QUEUE_SIZE: usize = 3;
const MOUSE_QUEUE_SIZE: usize = 5;
const CTRL_QUEUE_SIZE: usize = 2;

type HidInterface = kiibohd_usb::HidInterface<
    'static,
    UsbBus,
    KBD_QUEUE_SIZE,
    KBD_LED_QUEUE_SIZE,
    MOUSE_QUEUE_SIZE,
    CTRL_QUEUE_SIZE,
>;

pub struct HidioInterface<const H: usize> {}

#[allow(dead_code)]
impl<const H: usize> HidioInterface<H> {
    fn new() -> Self {
        Self {}
    }
}

impl<const H: usize> KiibohdCommandInterface<H> for HidioInterface<H> {
    fn h0001_device_name(&self) -> Option<&str> {
        Some("ErgoOne")
    }

    fn h0001_firmware_name(&self) -> Option<&str> {
        Some("ErgoOne")
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum ARGS {
    KS {
        code: KeyCode,
        op: Operation,
        st: StateType,
    },
    RGB {
        r: u8,
        g: u8,
        b: u8,
    },
    STR {
        s: String<30>,
    },
}

/// execute function for key code
pub fn action(action: CallbackActions, ops: ARGS) {
    match action {
        CallbackActions::Push => match ops {
            ARGS::KS { code, op: _, st } => {
                if code != KeyCode::________ {
                    unsafe {
                        match st {
                            StateType::Tap => {}
                            StateType::Hold => {}
                            StateType::Off => {}
                            StateType::Idle => {}
                        }
                        critical_section::with(|_| {
                            let kbd = KBD_PRODUCER.get_mut();
                            if kbd.is_some() {
                                match kbd
                                    .as_mut()
                                    .unwrap()
                                    .enqueue(kiibohd_usb::KeyState::Press(code.into()))
                                {
                                    Ok(_) => (),
                                    Err(err) => error!("{}", err),
                                }
                            } else {
                                error!("KBD_PRODUCER is None");
                            }
                        })
                        // let mm = KEY_QUEUE.enqueue((code, op, st));
                        // println!("push: {:?}, {:?} :: {:?}", code, st, mm);
                    }
                }
            }
            _ => {
                error!("Expected ARGS::KS but got something else");
            }
        },
        CallbackActions::RGBSet => match ops {
            ARGS::RGB { r, g, b } => {
                println!("RGB: {} {} {}", r, g, b);
                RCOL.store(r, Ordering::Relaxed);
                GCOL.store(g, Ordering::Relaxed);
                BCOL.store(b, Ordering::Relaxed);
            }
            _ => {
                error!("Expected ARGS::RGB but got something else");
            }
        },
        CallbackActions::SendString => match ops {
            ARGS::STR { s: _ } => {}
            _ => {
                error!("Expected ARGS::STR but got something else");
            }
        },
    }
}

static mut CORE1_STACK: Stack<4096> = Stack::new();

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

// Setup the queues used to generate the input reports (ctrl, keyboard and mouse)
static mut CTRL_QUEUE: Queue<kiibohd_usb::CtrlState, CTRL_QUEUE_SIZE> = Queue::new();
static mut KBD_QUEUE: Queue<kiibohd_usb::KeyState, KBD_QUEUE_SIZE> = Queue::new();
static mut KBD_LED_QUEUE: Queue<kiibohd_usb::LedState, KBD_LED_QUEUE_SIZE> = Queue::new();
static mut MOUSE_QUEUE: Queue<kiibohd_usb::MouseState, MOUSE_QUEUE_SIZE> = Queue::new();
static mut HIDIO_INTF: Mutex<
    Option<CommandInterface<HidioInterface<256>, 8, 8, 64, 256, 277, 10>>,
> = Mutex::new(None);

#[entry]
fn main() -> ! {
    info!("Program start");
    // Initialize everything
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let mut sio = Sio::new(pac.SIO);
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Initialize USB
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(UsbBusAllocator::new(UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        )));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    // Setup the interface
    // NOTE: Ignoring usb_bus setup in this example, use a compliant usb-device UsbBus interface
    unsafe {
        let (kbd_producer, kbd_consumer) = KBD_QUEUE.split();
        let (kbd_led_producer, kbd_led_consumer) = KBD_LED_QUEUE.split();
        let (mouse_producer, mouse_consumer) = MOUSE_QUEUE.split();
        let (ctrl_producer, ctrl_consumer) = CTRL_QUEUE.split();
        KBD_PRODUCER = Mutex::new(Some(kbd_producer));
        USB_HID = Some(HidInterface::new(
            bus_allocator,
            HidCountryCode::NotSupported,
            kbd_consumer,
            kbd_led_producer,
            mouse_consumer,
            ctrl_consumer,
        ));

        // Basic CommandInterface
        HIDIO_INTF = Mutex::new(Some(
            CommandInterface::<HidioInterface<256>, 8, 8, 64, 256, 277, 10>::new(
                &[
                    HidIoCommandId::SupportedIds,
                    HidIoCommandId::GetInfo,
                    HidIoCommandId::TestPacket,
                ],
                HidioInterface::<256>::new(),
            )
            .unwrap(),
        ));
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(rp2040_hal::pac::Interrupt::USBCTRL_IRQ);
    };

    // Initialize Keyscanning
    let rows: [Row; 5] = [
        Row::new(pins.gpio15.into()),
        Row::new(pins.gpio14.into()),
        Row::new(pins.gpio13.into()),
        Row::new(pins.gpio12.into()),
        Row::new(pins.gpio11.into()),
    ];
    let cols: [Col; 16] = [
        Col::new(pins.gpio29.into()),
        Col::new(pins.gpio28.into()),
        Col::new(pins.gpio27.into()),
        Col::new(pins.gpio26.into()),
        Col::new(pins.gpio25.into()),
        Col::new(pins.gpio24.into()),
        Col::new(pins.gpio23.into()),
        Col::new(pins.gpio17.into()),
        Col::new(pins.gpio16.into()),
        Col::new(pins.gpio6.into()),
        Col::new(pins.gpio5.into()),
        Col::new(pins.gpio4.into()),
        Col::new(pins.gpio3.into()),
        Col::new(pins.gpio2.into()),
        Col::new(pins.gpio1.into()),
        Col::new(pins.gpio0.into()),
    ];
    /// callback to print a report of what happened during the scan
    fn callback(
        row: usize,
        col: usize,
        state: StateType,
        prevstate: StateType,
        keycodes: [KeyCode; 2],
    ) {
        let rowstr: String<2> = String::from(row as u32);
        let colstr: String<2> = String::from(col as u32);
        let mut str: String<35> = String::from("row: ");
        str.push_str(rowstr.as_str()).unwrap();
        str.push_str(", col: ").unwrap();
        str.push_str(colstr.as_str()).unwrap();
        str.push_str(match prevstate {
            StateType::Tap => " p: Tap",
            StateType::Hold => " p: Hold",
            StateType::Off => " p: Off",
            StateType::Idle => " p: Idle",
        })
        .unwrap();
        str.push_str(match state {
            StateType::Tap => " c: Tap",
            StateType::Hold => " c: Hold",
            StateType::Off => " c: Off",
            StateType::Idle => " c: Idle",
        })
        .unwrap();
        info!("{}, c1: {}, c2: {}", str, keycodes[0], keycodes[1]);
    }

    let mut matrix: Matrix<5, 16> =
        Matrix::new(rows, cols, callback, key_mapping::ERGOONE_RSTLNE.into());
    let poll1 = matrix.poll(Context {
        key_queue: unsafe { ACTIVE_QUEUE.get_keys() },
    });

    if poll1 {
        // let gpio_activity_pin_mask = 0;
        // let disable_interface_mask = 0;
        info!("Escape key detected on boot, going into bootloader mode.");
        // rp2040_hal::rom_data::reset_to_usb_boot(gpio_activity_pin_mask, disable_interface_mask);
        rp2040_hal::rom_data::reset_to_usb_boot(0, 0);
    }

    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);

    let cores = mc.cores();
    let core1 = &mut cores[1];

    let _ledcore = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || {
        use smart_leds::{SmartLedsWrite, RGB8};
        let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
        let empty: [RGB8; 8] = [RGB8::default(); 8];
        let mut ws = Ws2812::new(
            pins.gpio7.into_mode(),
            &mut pio,
            sm0,
            clocks.peripheral_clock.freq(),
            timer.count_down(),
        );
        let mut R = RCOL.load(Ordering::Relaxed);
        let mut G = GCOL.load(Ordering::Relaxed);
        let mut B = BCOL.load(Ordering::Relaxed);
        ws.write(empty.iter().copied()).unwrap();
        loop {
            let color = [RGB8::new(R, G, B); 8];
            R = RCOL.load(Ordering::Relaxed);
            G = GCOL.load(Ordering::Relaxed);
            B = BCOL.load(Ordering::Relaxed);
            let curcol = [RGB8::new(R, G, B); 8];
            if (curcol[0].r != color[0].r)
                || (curcol[0].g != color[0].g)
                || (curcol[0].b != color[0].b)
            {
                println!(
                    "{},{},{}  {},{},{}",
                    curcol[0].r, curcol[0].g, curcol[0].b, color[0].r, color[0].g, color[0].b
                );
                // ws.write(empty.iter().copied()).unwrap();
                // color = curcol;
            }
            ws.write(color.iter().copied()).unwrap();
        }
    });

    info!("Loop starting!");
    println!("thg = {}", 0 * 1);
    loop {
        delay.delay_us(1000u32);
        matrix.poll(Context {
            key_queue: unsafe { KEY_QUEUE.get_keys() },
        });
    }
}

static RCOL: AtomicU8 = AtomicU8::new(0);
static GCOL: AtomicU8 = AtomicU8::new(0);
static BCOL: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Context {
    key_queue: [Option<KeyCode>; 29],
}

fn nkro_push(key: (KeyCode, Operation, StateType)) {
    let mut keybitmap = critical_section::with(|cs| KEYBOARD_REPORT.borrow_ref(cs).keybitmap);
    let mut k: u8 = key.0.into();

    // NOTE: The indexing actually starts from 1 (not 0), so position 0 represents 1
    //       0 in USB HID represents no keys pressed, so it's meaningless in a bitmask

    //       Ignore any keys over 231/0xE7
    if k == 0 || k > 0xE7 {
        warn!(
            "Invalid key for nkro_push({}, {}), ignored.",
            key,
            key.2 == StateType::Tap || key.2 == StateType::Hold
        );
        return;
    }

    k = k - 1;

    // Determine position
    let byte: usize = (k / 8).into();
    let bit: usize = (k % 8).into();

    // Set/Unset
    if key.2 == StateType::Tap || key.2 == StateType::Hold {
        debug!("Key {} in  ACTION", key);
        unsafe {
            ACTIVE_QUEUE.enqueue(key);
        }
        keybitmap[byte] |= 1 << bit;
    } else {
        debug!("Key {} out ACTION", key);
        unsafe {
            ACTIVE_QUEUE.dequeue(key);
        }
        keybitmap[byte] &= !(1 << bit);
    }
    critical_section::with(|cs| {
        KEYBOARD_REPORT.replace(cs, KeyboardNkroReport { leds: 0, keybitmap })
    });
}

static mut KBD_PRODUCER: Mutex<Option<Producer<'_, KeyState, KBD_QUEUE_SIZE>>> = Mutex::new(None);
static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut HID_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HidInterface> = None;
static mut REPORTSENT: AtomicBool = AtomicBool::new(false);
static mut READYTOSEND: AtomicBool = AtomicBool::new(false);
static mut ACTIVE_QUEUE: KeyQueue<29> = KeyQueue::new();
static mut KEY_QUEUE: KeyQueue<29> = KeyQueue::new();
// static mut STRING_QUEUE: KeyQueue<30> = KeyQueue::new();

fn prepare_report() {
    critical_section::with(|_| unsafe {
        // println!("{}", KEY_QUEUE.keys.iter().find(|k| k.is_some()).is_some());

        // iterate over the queue
        KEY_QUEUE.keys.iter().for_each(|k| {
            if k.is_some() {
                // println!("{:?}", k.unwrap());
                let kr = k.unwrap();
                // Push the key to the report bitmap
                nkro_push(kr);
                KEY_QUEUE.dequeue(kr);
                // Add the key to the pull queue to be pulled if not pressed next time
                // PULL_QUEUE.enqueue(kr);

                // if kr.1 == Operation::SendOff && kr.2 == StateType::Tap || kr.2 == StateType::Hold {
                // }
            }
        });
    });
}

// fn prepare_report() {
//     let mut keybitmap = [0u8; 29];
//     let mut keycode_index = 0;
//
//     let mut push_keycode = |key: u8, press: bool| {
//         // NOTE: The indexing actually starts from 1 (not 0), so position 0 represents 1
//         //       0 in USB HID represents no keys pressed, so it's meaningless in a bitmask
//         //       Ignore any keys over 231/0xE7
//         if key == 0 || key > 0xE7 {
//             warn!("Invalid key for nkro_bit({}, {}), ignored.", key, press);
//             return;
//         }
//
//         let key = key - 1;
//
//         // Determine position
//         let byte: usize = (key / 8).into();
//         let bit: usize = (key % 8).into();
//
//         // Set/Unset
//         if press {
//             keybitmap[byte] |= 1 << bit;
//         } else {
//             keybitmap[byte] &= !(1 << bit);
//         }
//     };
//
//     unsafe {
//         // if the string queue is not empty then iterate through the queue
//         if !STRING_QUEUE.is_empty() {
//             // pull from the queue and process the keycode
//             return;
//         }
//     }
//
//     let mut dq: [Option<KeyCode>; 29] = [None; 29];
//     critical_section::with(|_| unsafe {
//         KEY_QUEUE.keys.iter().for_each(|k| {
//             if k.is_some() {
//                 let kr = k.unwrap();
//                 push_keycode(kr.0.into());
//                 if kr.1 == Operation::SendOff {
//                     dq[dq.iter().position(|x| x.is_none()).unwrap()] = Some(kr.0);
//                 }
//             }
//         });
//     });
//     unsafe {
//         dq.iter().for_each(|k| {
//             if k.is_some() {
//                 let kr = k.unwrap();
//                 KEY_QUEUE.dequeue(kr);
//             }
//         });
//     }
//
//     critical_section::with(|cs| {
//         KEYBOARD_REPORT.replace(cs, KeyboardNkroReport { leds: 0, keybitmap })
//     });
// }

static KEYBOARD_REPORT: Mutex<RefCell<KeyboardNkroReport>> =
    Mutex::new(RefCell::new(KeyboardNkroReport {
        leds: 0,
        keybitmap: [0u8; 29],
    }));

const BLANK_REPORT: KeyboardNkroReport = KeyboardNkroReport {
    leds: 0,
    keybitmap: [0u8; 29],
};

/// Handle USB interrupts, used by the host to "poll" the keyboard for new inputs.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    if let Some(usb_dev) = HID_BUS.as_mut() {
        if let Some(usb_hid) = USB_HID.as_mut() {
            if usb_dev.poll(&mut usb_hid.interfaces()) {
                usb_hid.pull();
                let hidio_intf = critical_section::with(|_| HIDIO_INTF.get_mut().as_mut());
                if hidio_intf.is_some() {
                    let hidio = hidio_intf.unwrap();
                    usb_hid.pull_hidio(hidio);
                }
            }

            usb_hid.update();

            match usb_hid.push() {
                Ok(_) => {
                    // critical_section::with(|cs| KEYBOARD_REPORT.replace(cs, BLANK_REPORT));
                    REPORTSENT.store(true, Ordering::Relaxed);
                }
                Err(UsbError::WouldBlock) => {
                    REPORTSENT.store(false, Ordering::Relaxed);
                }
                Err(UsbError::ParseError) => error!("UsbError::ParseError"),
                Err(UsbError::BufferOverflow) => error!("UsbError::BufferOverflow"),
                Err(UsbError::EndpointOverflow) => error!("UsbError::EndpointOverflow"),
                Err(UsbError::EndpointMemoryOverflow) => error!("UsbError::EndpointMemoryOverflow"),
                Err(UsbError::InvalidEndpoint) => error!("UsbError::InvalidEndpoint"),
                Err(UsbError::Unsupported) => error!("UsbError::Unsupported"),
                Err(UsbError::InvalidState) => error!("UsbError::InvalidState"),
            }
        }
    }

    // if REPORTSENT.load(Ordering::Relaxed) {
    //     prepare_report();
    // }

    // let report: KeyboardNkroReport;
    // if READYTOSEND.load(Ordering::Relaxed) {
    // report = critical_section::with(|cs| *KEYBOARD_REPORT.borrow_ref(cs));
    // } else {
    // report = BLANK_REPORT;
    // }

    // let report = critical_section::with(|cs| *KEYBOARD_REPORT.borrow_ref(cs));

    // macOS doesn't like it when you don't pull this, apparently.
    // TODO: maybe even parse something here
    // let mut out: [u8; 64] = [0; 64];
    // let siz = usb_hid.pull_raw_output(&mut out).unwrap_unchecked();
    // if siz > 8 {
    //     println!("outty: {:a}, {:?}", out, siz);
    // }

    // Wake the host if a key is pressed and the device supports
    // remote wakeup.
    // if !report_is_empty(&report)
    //     && usb_dev.state() == UsbDeviceState::Suspend
    //     && usb_dev.remote_wakeup_enabled()
    // {
    //     usb_dev.bus().remote_wakeup();
    // }
}

fn report_is_empty(report: &KeyboardNkroReport) -> bool {
    report
        .keybitmap
        .iter()
        .any(|key| *key != key_codes::KeyCode::________ as u8)
}
