#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod actions;
mod key;
mod key_codes;
mod key_mapping;
mod keyscanning;
mod macros;
mod mods;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::{AtomicU8, Ordering};

use cortex_m_rt::entry;
use critical_section::Mutex;
use defmt::*;
use defmt_rtt as _;
use heapless::spsc::{Producer, Queue};
use heapless::{String, Vec};
use keyscanning::{Col, Row};
use kiibohd_hid_io::{CommandInterface, HidIoCommandId, KiibohdCommandInterface};
use kiibohd_usb::KeyState;
use panic_probe as _;
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
use usb_device::prelude::{UsbDeviceBuilder, UsbVidPid};
use usb_device::{class_prelude::UsbBusAllocator, prelude::UsbDevice, UsbError};
use usbd_hid::hid_class::HidCountryCode;
use ws2812_pio::Ws2812;

use self::actions::CallbackActions;
use self::keyscanning::{KeyQueue, KeyQueueMulti};
use crate::keyscanning::Matrix;
use crate::keyscanning::StateType;
use crate::{key_codes::KeyCode, pac::interrupt};

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
  fn new() -> Self { Self {} }
}

impl<const H: usize> KiibohdCommandInterface<H> for HidioInterface<H> {
  fn h0001_device_name(&self) -> Option<&str> { Some("ErgoOne") }
  fn h0001_firmware_name(&self) -> Option<&str> { Some("ErgoOne") }
}

static mut KBD_LAYER: AtomicU8 = AtomicU8::new(0);
static mut SENDINGSTRING: AtomicBool = AtomicBool::new(false);

#[derive(Clone, PartialEq, PartialOrd)]
pub enum ARGS {
  KS { code: KeyCode },
  RGB { r: u8, g: u8, b: u8 },
  STR { s: String<30> },
  LYR { l: usize },
  BLN { b: bool },
  NON {},
}

/// execute function for key code
pub fn action(action: CallbackActions, ops: ARGS) {
  match action {
    CallbackActions::Press => match ops {
      ARGS::KS { code } => {
        if unsafe { !SENDINGSTRING.load(Ordering::Relaxed) } {
          critical_section::with(|_| {
            let kbd = unsafe { KBD_PRODUCER.get_mut() };
            if code != KeyCode::________ {
              if kbd.is_some() {
                match kbd
                  .as_mut()
                  .unwrap()
                  .enqueue(kiibohd_usb::KeyState::Press(code.into()))
                {
                  Ok(_) => {
                    warn!("Key IN  {:?}", code);
                    unsafe { ACTIVE_QUEUE.enqueue(code) };
                  }
                  Err(err) => error!("{}", err),
                }
              } else {
                error!("KBD_PRODUCER is None");
              }
            }
          });
        }
      }
      _ => {
        error!("Expected ARGS::KS but got something else");
      }
    },
    CallbackActions::Release => match ops {
      ARGS::KS { code } => {
        critical_section::with(|_| {
          let kbd = unsafe { KBD_PRODUCER.get_mut() };
          if code != KeyCode::________ {
            if kbd.is_some() {
              match kbd
                .as_mut()
                .unwrap()
                .enqueue(kiibohd_usb::KeyState::Release(code.into()))
              {
                Ok(_) => {
                  warn!("Key OUT {:?}", code);
                  unsafe { ACTIVE_QUEUE.dequeue(code) };
                }
                Err(err) => error!("{}", err),
              }
            } else {
              error!("KBD_PRODUCER is None");
            }
          }
        });
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
    // TODO: add support for sending strings
    CallbackActions::SendString => match ops {
      ARGS::STR { s: strng } => {
        // start sending string and block other keys sending until complete
        unsafe { SENDINGSTRING.store(true, Ordering::Relaxed) };
        strng.chars().for_each(|e| {
          let kbd = unsafe { KBD_PRODUCER.get_mut() };
          let code = KeyCode::from_char(e);
          if code.1 != 0 {
            if kbd.is_some() {
              code.0.iter().for_each(|x| {
                if x.is_some() {
                  let code = x.unwrap();
                  match kbd
                    .as_mut()
                    .unwrap()
                    .enqueue(kiibohd_usb::KeyState::Press(code.into()))
                  {
                    Ok(_) => {
                      warn!("Key OUT {:?}", code);
                      unsafe { STRING_QUEUE.push(code) };
                    }
                    Err(err) => error!("{}", err),
                  }
                }
              });
            } else {
              error!("KBD_PRODUCER is None");
            }
          }
        })
      }
      _ => {
        error!("Expected ARGS::STR but got something else");
      }
    },
    CallbackActions::SetLayer => match ops {
      ARGS::LYR { l } => {
        println!("Layer: {}", l);
        unsafe { KBD_LAYER.store(l as u8, Ordering::Relaxed) };
      }
      _ => {
        error!("Expected ARGS::LYR but got something else");
      }
    },
    CallbackActions::IncLayer => match ops {
      ARGS::NON {} => {
        unsafe {
          let l = KBD_LAYER.load(Ordering::Relaxed);
          KBD_LAYER.store((l + 1) as u8, Ordering::Relaxed);
        };
      }
      _ => {
        error!("Expected ARGS::LYR but got something else");
      }
    },
    CallbackActions::DecLayer => match ops {
      ARGS::NON {} => {
        unsafe {
          let l = KBD_LAYER.load(Ordering::Relaxed);
          KBD_LAYER.store((l - 1) as u8, Ordering::Relaxed);
        };
      }
      _ => {
        error!("Expected ARGS::LYR but got something else");
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
  unsafe {
    USB_ALLOCATOR = Some(UsbBusAllocator::new(UsbBus::new(
      pac.USBCTRL_REGS,
      pac.USBCTRL_DPRAM,
      clocks.usb_clock,
      true,
      &mut pac.RESETS,
    )));
    USB_ALLOCATOR.as_ref().unwrap();
    // Setup the interface
    let (kbd_producer, kbd_consumer) = KBD_QUEUE.split();
    let (kbd_led_producer, _kbd_led_consumer) = KBD_LED_QUEUE.split();
    let (_mouse_producer, mouse_consumer) = MOUSE_QUEUE.split();
    let (_ctrl_producer, ctrl_consumer) = CTRL_QUEUE.split();
    KBD_PRODUCER = Mutex::new(Some(kbd_producer));
    USB_HID = Some(HidInterface::new(
      USB_ALLOCATOR.as_ref().unwrap(),
      HidCountryCode::US,
      kbd_consumer,
      kbd_led_producer,
      mouse_consumer,
      ctrl_consumer,
    ));
    warn!("USB_HID is setup");
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
    warn!("HIDIO_INTF is setup");
    HID_BUS = Some(
      UsbDeviceBuilder::new(USB_ALLOCATOR.as_ref().unwrap(), UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("HisZd")
        .product("ErgoOne")
        .serial_number("000001")
        .supports_remote_wakeup(false)
        .build(),
    );
    warn!("HID_BUS is setup");
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
    layer: usize,
    state: StateType,
    prevstate: StateType,
    keycodes: [KeyCode; 2],
  ) {
    let rowstr: String<2> = String::from(row as u32);
    let colstr: String<2> = String::from(col as u32);
    let laystr: String<2> = String::from(layer as u32);
    let mut str: String<40> = String::from("row: ");
    str.push_str(rowstr.as_str()).unwrap();
    str.push_str(", col: ").unwrap();
    str.push_str(colstr.as_str()).unwrap();
    str.push_str(", lay: ").unwrap();
    str.push_str(laystr.as_str()).unwrap();
    str
      .push_str(match prevstate {
        StateType::Tap => " p: Tap",
        StateType::Hold => " p: Hold",
        StateType::Off => " p: Off",
        StateType::Idle => " p: Idle",
      })
      .unwrap();
    str
      .push_str(match state {
        StateType::Tap => " c: Tap",
        StateType::Hold => " c: Hold",
        StateType::Off => " c: Off",
        StateType::Idle => " c: Idle",
      })
      .unwrap();
    info!("{}, c1: {}, c2: {}", str, keycodes[0], keycodes[1]);
  }

  let mut matrix: Matrix<5, 16> = Matrix::new(rows, cols, callback, [
    key_mapping::ERGOONE_RSTLNE.into(),
    key_mapping::ERGOONE_1.into(),
  ]);

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
      if (curcol[0].r != color[0].r) || (curcol[0].g != color[0].g) || (curcol[0].b != color[0].b) {
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
    // if SENDINGSTRING is true then queue the next key
    if unsafe { SENDINGSTRING.load(Ordering::Relaxed) } {
      let kbd = unsafe { KBD_PRODUCER.get_mut() };
      let codes = unsafe { STRING_QUEUE.pop().unwrap() };
      if codes.0.is_some() {
        if kbd.is_some() {
          match kbd
            .as_mut()
            .unwrap()
            .enqueue(kiibohd_usb::KeyState::Press(codes.0.unwrap().into()))
          {
            Ok(_) => {
              warn!("Key OUT {:?}", codes.0.unwrap());
            }
            Err(err) => error!("{}", err),
          }
        } else {
          error!("KBD_PRODUCER is None");
        }
      }
    }
    // Delay by 1ms
    delay.delay_us(1000u32);
    unsafe {
      if let Some(usb_hid) = USB_HID.as_mut() {
        usb_hid.update();
        match usb_hid.push() {
          Ok(_) => {
            REPORTSENT.store(true, Ordering::Relaxed);
          }
          Err(UsbError::WouldBlock) => {
            REPORTSENT.store(false, Ordering::Relaxed);
          }
          Err(UsbError::ParseError) => error!("UsbError::ParseError"),
          Err(UsbError::BufferOverflow) => error!("UsbError::BufferOverflow"),
          Err(UsbError::EndpointOverflow) => error!("UsbError::EndpointOverflow"),
          Err(UsbError::EndpointMemoryOverflow) => {
            error!("UsbError::EndpointMemoryOverflow")
          }
          Err(UsbError::InvalidEndpoint) => error!("UsbError::InvalidEndpoint"),
          Err(UsbError::Unsupported) => error!("UsbError::Unsupported"),
          Err(UsbError::InvalidState) => error!("UsbError::InvalidState"),
        }
      }
    }
    matrix.poll(Context {
      key_queue: unsafe { ACTIVE_QUEUE.get_keys() },
    });
    matrix.set_layer(unsafe { KBD_LAYER.load(Ordering::Relaxed) as usize });
  }
}

static RCOL: AtomicU8 = AtomicU8::new(0);
static GCOL: AtomicU8 = AtomicU8::new(0);
static BCOL: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Context {
  key_queue: [Option<KeyCode>; 10],
}

static mut KBD_PRODUCER: Mutex<Option<Producer<'_, KeyState, KBD_QUEUE_SIZE>>> = Mutex::new(None);
static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut HID_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HidInterface> = None;
static mut REPORTSENT: AtomicBool = AtomicBool::new(false);
static mut ACTIVE_QUEUE: KeyQueue<10> = KeyQueue::new();
static mut STRING_QUEUE: KeyQueueMulti<30> = KeyQueueMulti::new();

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
    }
  }
}
