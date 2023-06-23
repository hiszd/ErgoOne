#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod key;
mod key_codes;
mod key_mapping;
mod keyscanning;
mod mods;

use crate::pac::interrupt;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use heapless::String;
use keyscanning::{Col, Row};
use panic_probe as _;

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;
use usb_device::{
    class_prelude::{UsbBusAllocator, UsbClass},
    prelude::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_hid::{
    descriptor::KeyboardReport,
    hid_class::{HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig},
};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
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
    let sio = Sio::new(pac.SIO);

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

    let _delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

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

    fn callback(row: usize, col: usize, state: StateType, prevstate: StateType) {
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
        str.push_str("\n").unwrap();
        info!("{}", str);
    }

    // TOTO create way to handle more than 6 codes per poll
    fn push_input(codes: [u8; 6], modifier: u8) {
        // println!("{}, {}", codes, modifier);
        let key_report = KeyboardReport {
            modifier,
            reserved: 0,
            leds: 0,
            keycodes: codes,
        };
        unsafe {
            let usb_hid = USB_HID.as_ref().unwrap_unchecked();
            usb_hid.push_input(&key_report).unwrap_unchecked();
            // macOS doesn't like it when you don't pull this, apparently.
            // TODO: maybe even parse something here
            usb_hid.pull_raw_output(&mut [0; 64]).ok();
            // Wake the host if a key is pressed and the device supports
            // remote wakeup.
            // if !report_is_empty(&key_report)
            //     && keyboard_usb_device.state() == UsbDeviceState::Suspend
            //     && keyboard_usb_device.remote_wakeup_enabled()
            // {
            // usb_hid.
            // }
        }
    }

    let mut matrix: Matrix<5, 16> = Matrix::new(
        rows,
        cols,
        callback,
        push_input,
        key_mapping::FancyAlice66(),
    );
    // TODO reboot into bootloader if started while escape is pressed.
    // ISSUE there doesn't appear to be any way of doing this in the HAL currently
    // let scan = matrix.poll().unwrap();
    // if scan[0][0] >= 4 {}

    let mut polldly: usize = 0;

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        pins.gpio7.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );
    let mut ledfn = || {
        use smart_leds::{SmartLedsWrite, RGB8};
        let color: RGB8 = (255, 0, 0).into();

        ws.write(
            [color, color, color, color, color, color, color, color]
                .iter()
                .copied(),
        )
        .unwrap();
    };

    info!("Loop starting!");
    loop {
        ledfn();

        if polldly < 1 {
            polldly += 1;
        } else {
            matrix.poll();
            polldly = 0;
        }
    }
}

fn report_is_empty(report: &KeyboardReport) -> bool {
    report.modifier != 0
        || report
            .keycodes
            .iter()
            .any(|key| *key != key_codes::KeyCode::________ as u8)
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut HID_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    if let (Some(usb_dev), Some(hid)) = (HID_BUS.as_mut(), USB_HID.as_mut()) {
        if usb_dev.poll(&mut [hid]) {
            hid.poll();
        }
    }
}
