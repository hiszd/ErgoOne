#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

mod key;
mod key_codes;
mod key_mapping;
mod keyscanning;
mod mods;

use arduino_hal::Peripherals;
use atmega_usbd::UsbBus;
use heapless::String;
use keyscanning::{Col, Row};
use panic_halt as _;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    UsbError,
};
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};
use usbd_serial::SerialPort;

use crate::keyscanning::Matrix;
use crate::keyscanning::StateType;

#[allow(dead_code)]
fn println(msg: &[u8]) -> bool {
    unsafe {
        if let Some(ser) = USB_SERIAL.as_mut() {
            match ser.write(msg) {
                Ok(count) => count == msg.len(),
                Err(UsbError::WouldBlock) => false, // No data could be written (buffers full)
                Err(_err) => false,                 // An error occurred
            }
        } else {
            false
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let pll = dp.PLL;
    let usb = dp.USB_DEVICE;

    // Configure PLL interface
    // prescale 16MHz crystal -> 8MHz ** actually I got rid of the scaling
    // pll.pllcsr.write(|w| unsafe { w.bits(0_u8) });
    pll.pllcsr.write(|w| w.pindiv().set_bit());
    // 96MHz PLL output; /1.5 for 64MHz timers, /2 for 48MHz USB
    pll.pllfrq
        .write(|w| w.pdiv().mhz96().plltm().factor_15().pllusb().set_bit());

    // Enable PLL
    pll.pllcsr.modify(|_, w| w.plle().set_bit());

    // Check PLL lock
    while pll.pllcsr.read().plock().bit_is_clear() {}

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(UsbBus::new(usb));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_HID = Some(HIDClass::new(bus_allocator, KeyboardReport::desc(), 60));
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        HID_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("HisZd")
                .product("avrkey")
                .serial_number("000001")
                .supports_remote_wakeup(true)
                .build(),
        );
        // HID_BUS.as_mut().unwrap().force_reset().ok();
    }

    let rows: [Row; 5] = [
        Row::new(pins.a3.into_output().downgrade()),
        Row::new(pins.a2.into_output().downgrade()),
        Row::new(pins.a1.into_output().downgrade()),
        Row::new(pins.a0.into_output().downgrade()),
        Row::new(pins.d13.into_output().downgrade()),
    ];

    let cols: [Col; 16] = [
        Col::new(pins.d5.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d7.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d9.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d8.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d6.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d12.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d4.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.led_tx.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d1.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d0.into_floating_input().downgrade().forget_imode()),
        // Col::new(pins.d2.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d10.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d3.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.d11.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.miso.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.mosi.into_floating_input().downgrade().forget_imode()),
        Col::new(pins.sck.into_floating_input().downgrade().forget_imode()),
    ];

    fn info(info: &str) {
        println(info.as_bytes());
    }

    fn callback(row: usize, col: usize, state: StateType, prevstate: StateType) {
        // let blank: String<20> = String::from("                    \n");
        let rowstr: String<2> = String::from(row as u32);
        let colstr: String<2> = String::from(col as u32);
        let mut str: String<30> = String::from("row: ");
        str.push_str(rowstr.as_str()).unwrap();
        str.push_str(", col: ").unwrap();
        str.push_str(colstr.as_str()).unwrap();
        str.push_str(match prevstate {
            StateType::Tap => "p: Tap",
            StateType::Hold => "p: Hold",
            StateType::Off => "p: Off",
            StateType::Idle => "p: Idle",
        })
        .unwrap();
        str.push_str(match state {
            StateType::Tap => "c: Tap",
            StateType::Hold => "c: Hold",
            StateType::Off => "c: Off",
            StateType::Idle => "c: Idle",
        })
        .unwrap();
        str.push_str("\n").unwrap();
        // println(&blank.into_bytes());
        println(&str.into_bytes());
    }

    // TOTO create way to handle more than 6 codes per poll
    fn push_input(codes: [u8; 6], modifier: u8) {
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

    let matrix: Matrix<5, 16> = Matrix::new(
        rows,
        cols,
        callback,
        info,
        push_input,
        key_mapping::FancyAlice66(),
    );
    // TODO reboot into bootloader if started while escape is pressed.
    // ISSUE there doesn't appear to be any way of doing this in the HAL currently
    // let scan = matrix.poll().unwrap();
    // if scan[0][0] >= 4 {}

    let mut countinit: usize = 0;

    let mut cnt: usize = 0;
    let mut ledpin = pins.d2.into_output();
    let mut cntfn = || {
        if cnt < 3000 {
            cnt += 1;
        } else {
            ledpin.toggle();
            cnt = 0;
            if ledpin.is_set_high() {
                println(b"high\n");
            } else {
                println(b"low\n");
            }
        }
    };

    loop {
        unsafe {
            if poll_usb() || HID_BUS.as_mut().unwrap().state() != UsbDeviceState::Configured {
                continue;
            }
        }

        cntfn();

        // matrix.poll();

        unsafe {
            if countinit <= 5 && USB_SERIAL.as_mut().unwrap().dtr() {
                println(b"heyonce ");
                countinit += 1;
            }
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

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut HID_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn poll_usb() -> bool {
    unsafe {
        if let (Some(usb_dev), Some(hid), Some(serial)) =
            (HID_BUS.as_mut(), USB_HID.as_mut(), USB_SERIAL.as_mut())
        {
            usb_dev.poll(&mut [hid, serial]);
            return false;
        }
        true
    }
}
