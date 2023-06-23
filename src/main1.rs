#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin, PinState, ToggleableOutputPin};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp2040_hal as bsp;

use bsp::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    pins.gpio15.into_push_pull_output_in_state(PinState::High);
    let col0 = pins.gpio29.into_floating_input();
    let mut leds = pins.gpio7.into_push_pull_output();

    // let mut oldstate = false;
    // let mut count: u32 = 0;

    loop {
        leds.toggle().unwrap();
        // delay.delay_ms(1_u32);
        // if count > 1000 {
        //     leds.toggle().unwrap();
        //     count = 0;
        // }
        // count += 1;
    }
}
