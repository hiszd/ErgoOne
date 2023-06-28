#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod key;
mod key_codes;
mod key_mapping;
mod keyscanning;
mod mods;

use core::sync::atomic::AtomicBool;
use core::{
    cell::RefCell,
    sync::atomic::{AtomicU8, Ordering},
};
use heapless::spsc::Queue;

use crate::{key_codes::KeyCode, pac::interrupt};
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use heapless::String;
use keyscanning::{Col, Row};
use panic_probe as _;

use critical_section::Mutex;
use usb_device::{
    class_prelude::{UsbBusAllocator, UsbClass},
    prelude::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    UsbError,
};
use usbd_hid::{
    descriptor::KeyboardReport,
    hid_class::{HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig},
};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

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

use crate::keyscanning::Matrix;
use crate::keyscanning::StateType;

static mut CORE1_STACK: Stack<4096> = Stack::new();

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    info!("Program start");
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

    unsafe {
        USB_HID = Some(HIDClass::new_with_settings(
            bus_allocator,
            KeyboardReport::desc(),
            1,
            HidClassSettings {
                subclass: HidSubClass::NoSubClass,
                protocol: HidProtocol::Keyboard,
                config: ProtocolModeConfig::ForceReport,
                locale: HidCountryCode::US,
            },
        ));
        HID_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("HisZd")
                .product("ErgoOne")
                .serial_number("000001")
                .supports_remote_wakeup(true)
                .build(),
        );
        HID_BUS.as_mut().unwrap().force_reset().ok();
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(rp2040_hal::pac::Interrupt::USBCTRL_IRQ);
    };

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

        for code in keycodes {
            if code == KeyCode::Led_Col1 {
                RCOL.store(255, Ordering::Relaxed);
                GCOL.store(0, Ordering::Relaxed);
                BCOL.store(0, Ordering::Relaxed);
            } else if code == KeyCode::Led_Col2 {
                RCOL.store(0, Ordering::Relaxed);
                GCOL.store(0, Ordering::Relaxed);
                BCOL.store(255, Ordering::Relaxed);
            }
        }
    }

    // TODO create way to handle more than 6 codes per poll
    unsafe fn push_input(c: (KeyCode, StateType)) {
        if c != KeyCode::________ && !KEY_QUEUE.is_full() {
            match KEY_QUEUE.enqueue(c) {
                Ok(_) => {}
                Err(e) => error!("{}: {:?}", c, e),
            };
        }
    }

    // update MODIFIERS with a new value based on what is presed, or released
    unsafe fn mod_update(c: KeyCode) {
        let mods = MODIFIERS.load(Ordering::Relaxed);
        MODIFIERS.
        mods |= c.modifier_bitmask();

    }

    let mut matrix: Matrix<5, 16> = Matrix::new(
        rows,
        cols,
        callback,
        push_input,
        mod_update,
        key_mapping::FancyAlice66(),
    );
    // TODO reboot into bootloader if started while escape is pressed.

    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);

    let cores = mc.cores();
    let core1 = &mut cores[1];

    let _ledcore = core1.spawn(unsafe { &mut CORE1_STACK.mem }, move || {
        use smart_leds::{SmartLedsWrite, RGB8};
        let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
        let mut ws = Ws2812::new(
            pins.gpio7.into_mode(),
            &mut pio,
            sm0,
            clocks.peripheral_clock.freq(),
            timer.count_down(),
        );
        let mut color: RGB8 = RGB8::new(0, 0, 0);
        loop {
            let R = RCOL.load(Ordering::Relaxed);
            let G = GCOL.load(Ordering::Relaxed);
            let B = BCOL.load(Ordering::Relaxed);
            let curcol = RGB8::new(R, G, B);
            let basecol = RGB8::new(0, 0, 0);
            if curcol != color {
                ws.write(
                    [
                        basecol, basecol, basecol, basecol, basecol, basecol, basecol, basecol,
                    ]
                    .iter()
                    .copied(),
                )
                .unwrap();
                color = curcol;
            }
            ws.write(
                [color, color, color, color, color, color, color, color]
                    .iter()
                    .copied(),
            )
            .unwrap();
        }
    });

    info!("Loop starting!");
    loop {
        delay.delay_us(1000u32);
        matrix.poll();
    }
}

static RCOL: AtomicU8 = AtomicU8::new(0);
static GCOL: AtomicU8 = AtomicU8::new(0);
static BCOL: AtomicU8 = AtomicU8::new(0);

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut HID_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;
static mut REPORTSENT: AtomicBool = AtomicBool::new(false);
static mut READYTOSEND: AtomicBool = AtomicBool::new(false);
static mut KEY_QUEUE: Queue<KeyCode, 16> = Queue::new();
static mut MODIFIERS: AtomicU8 = AtomicU8::new(0);
// TODO create way to handle more than 6 codes per poll
fn prepare_report() {
    let mut keycodes = [0u8; 6];
    let mut keycode_index = 0;
    let mut modifier = 0;

    let mut push_keycode = |key| {
        if keycode_index < keycodes.len() {
            keycodes[keycode_index] = key;
            keycode_index += 1;
        }
    };

    let mut codes: [KeyCode; 6] = [
        KeyCode::________,
        KeyCode::________,
        KeyCode::________,
        KeyCode::________,
        KeyCode::________,
        KeyCode::________,
    ];
    for i in 0..6 {
        match unsafe { KEY_QUEUE.dequeue() } {
            Some(code) => {
                codes[i] = code;
            }
            None => {
                break;
            }
        };
    }

    codes.iter().for_each(|k| {
        if k != &KeyCode::________ {
            if let Some(bitmask) = k.modifier_bitmask() {
                // modifier |= bitmask;
                // info!("modifier: {=u8:#x}", modifier);
            } else {
                push_keycode(k.into());
            }
        }
    });
    // println!("{=[u8]:#x}", keycodes);

    critical_section::with(|cs| {
        KEYBOARD_REPORT.replace(
            cs,
            KeyboardReport {
                modifier: unsafe { MODIFIERS.load(Ordering::Relaxed) },
                reserved: 0,
                leds: 0,
                keycodes,
            },
        )
    });
}

static KEYBOARD_REPORT: Mutex<RefCell<KeyboardReport>> = Mutex::new(RefCell::new(KeyboardReport {
    modifier: 0,
    reserved: 0,
    leds: 0,
    keycodes: [0u8; 6],
}));

const BLANK_REPORT: KeyboardReport = KeyboardReport {
    modifier: 0,
    reserved: 0,
    leds: 0,
    keycodes: [0u8; 6],
};

/// Handle USB interrupts, used by the host to "poll" the keyboard for new inputs.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_dev = HID_BUS.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();

    if usb_dev.poll(&mut [usb_hid]) {
        usb_hid.poll();
    }

    prepare_report();

    let report = critical_section::with(|cs| *KEYBOARD_REPORT.borrow_ref(cs));
    println!("m: {=u8:x}", report.modifier);
    match usb_hid.push_input(&report) {
        Ok(_) => {
            critical_section::with(|cs| KEYBOARD_REPORT.replace(cs, BLANK_REPORT));
            REPORTSENT.store(true, Ordering::Relaxed);
            READYTOSEND.store(true, Ordering::Relaxed);
        }
        Err(UsbError::WouldBlock) => {
            REPORTSENT.store(false, Ordering::Relaxed);
            READYTOSEND.store(false, Ordering::Relaxed);
        }
        Err(UsbError::ParseError) => error!("UsbError::ParseError"),
        Err(UsbError::BufferOverflow) => error!("UsbError::BufferOverflow"),
        Err(UsbError::EndpointOverflow) => error!("UsbError::EndpointOverflow"),
        Err(UsbError::EndpointMemoryOverflow) => error!("UsbError::EndpointMemoryOverflow"),
        Err(UsbError::InvalidEndpoint) => error!("UsbError::InvalidEndpoint"),
        Err(UsbError::Unsupported) => error!("UsbError::Unsupported"),
        Err(UsbError::InvalidState) => error!("UsbError::InvalidState"),
    }

    // macOS doesn't like it when you don't pull this, apparently.
    // TODO: maybe even parse something here
    let mut out: [u8; 64] = [0; 64];
    let siz = usb_hid.pull_raw_output(&mut out).unwrap_unchecked();
    // if siz > 8 {
    //     println!("outty: {:a}, {:?}", out, siz);
    // }

    // Wake the host if a key is pressed and the device supports
    // remote wakeup.
    if !report_is_empty(&report)
        && usb_dev.state() == UsbDeviceState::Suspend
        && usb_dev.remote_wakeup_enabled()
    {
        usb_dev.bus().remote_wakeup();
    }
}

fn report_is_empty(report: &KeyboardReport) -> bool {
    report.modifier != 0
        || report
            .keycodes
            .iter()
            .any(|key| *key != key_codes::KeyCode::________ as u8)
}
