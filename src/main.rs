#![no_std]
#![no_main]
#![allow(non_snake_case)]

mod key;
mod key_codes;
mod key_mapping;
mod keyscanning;
mod mods;

// use rp2040_hal::usb::UsbBus;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{StatefulOutputPin, ToggleableOutputPin};
use heapless::String;
use keyscanning::{Row, Col};
use panic_probe as _;

use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use crate::keyscanning::Matrix;
use crate::keyscanning::StateType;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[rp2040_hal::entry]
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

    // let bus_allocator = unsafe {
    //     USB_ALLOCATOR = Some(UsbBus::new(usb));
    //     USB_ALLOCATOR.as_ref().unwrap()
    // };

    // unsafe {
    //     USB_HID = Some(HIDClass::new(bus_allocator, KeyboardReport::desc(), 60));
    //     USB_SERIAL = Some(SerialPort::new(bus_allocator));
    //     HID_BUS = Some(
    //         UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
    //             .manufacturer("HisZd")
    //             .product("avrkey")
    //             .serial_number("000001")
    //             .supports_remote_wakeup(true)
    //             .build(),
    //     );
    // HID_BUS.as_mut().unwrap().force_reset().ok();
    // }

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
        let mut str: String<30> = String::from("row: ");
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
        println!("{}, {}", codes, modifier);
        // let key_report = KeyboardReport {
        //     modifier,
        //     reserved: 0,
        //     leds: 0,
        //     keycodes: codes,
        // };
        // unsafe {
        //     let usb_hid = USB_HID.as_ref().unwrap_unchecked();
        //     usb_hid.push_input(&key_report).unwrap_unchecked();
        //     // macOS doesn't like it when you don't pull this, apparently.
        //     // TODO: maybe even parse something here
        //     usb_hid.pull_raw_output(&mut [0; 64]).ok();
        //     // Wake the host if a key is pressed and the device supports
        //     // remote wakeup.
        //     // if !report_is_empty(&key_report)
        //     //     && keyboard_usb_device.state() == UsbDeviceState::Suspend
        //     //     && keyboard_usb_device.remote_wakeup_enabled()
        //     // {
        //     // usb_hid.
        //     // }
        // }
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

    // let mut countinit: usize = 0;

    let mut polldly: usize = 0;

    let mut cnt: usize = 0;
    let mut ledpin = pins.gpio7.into_push_pull_output();
    let mut cntfn = || {
        if cnt < 1 {
            cnt += 1;
        } else {
            ledpin.toggle().unwrap();
            cnt = 0;
            // if ledpin.is_set_high().unwrap() {
            //     println!("high\n");
            // } else {
            //     println!("low\n");
            // }
        }
    };

    info!("Loop starting!");
    loop {
        cntfn();

        if polldly < 250000 {
            polldly += 1;
        } else {
            matrix.poll();
            polldly = 0;
        }
    }
}

// fn report_is_empty(report: &KeyboardReport) -> bool {
//     report.modifier != 0
//         || report
//             .keycodes
//             .iter()
//             .any(|key| *key != key_codes::KeyCode::Emp as u8)
// }

// static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
// static mut HID_BUS: Option<UsbDevice<UsbBus>> = None;
// static mut USB_HID: Option<HIDClass<UsbBus>> = None;
// static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

// fn poll_usb() -> bool {
//     unsafe {
//         if let (Some(usb_dev), Some(hid), Some(serial)) =
//             (HID_BUS.as_mut(), USB_HID.as_mut(), USB_SERIAL.as_mut())
//         {
//             usb_dev.poll(&mut [hid, serial]);
//             return false;
//         }
//         true
//     }
// }
