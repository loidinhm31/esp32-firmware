#![no_std]
#![no_main]

use esp_hal::{
    peripherals::Peripherals,
    clock::ClockControl,
    prelude::*,
    delay::Delay,
    gpio::{Io, Level, Output},
    system::SystemControl,
};
use fugit::ExtU64;
use esp_println::println;

#[allow(unused_imports)]
use esp_backtrace as _;

#[entry]
fn main() -> ! {
    // https://github.com/danclive/esp-examples/blob/main/blinky/src/main.rs
    // https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/wifi_access_point.rs
    #[cfg(feature = "log")]
    {
        // The default log level can be specified here.
        // You can see the esp-println documentation： https://docs.rs/esp-println
        esp_println::logger::init_logger(log::LevelFilter::Info);
    }

    println!("Init!");

    // Take Peripherals, Initialize Clocks, and Create a Handle for Each
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Instantiate and Create Handle for IO
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // use esp_println
    println!("hello world!");

    // use log
    #[cfg(feature = "log")]
    {
        log::error!("this is error message");
        log::warn!("this is warn message");
        log::info!("this is info message");
        log::debug!("this is debug message");
        log::trace!("this is trace message");
    }

    // Instantiate and Create Handle for LED output
    let mut led = Output::new(io.pins.gpio4, Level::Low);

    // Create and initialize a delay variable to manage delay loop
    // loop.
    let delay = Delay::new(&clocks);

    // Initialize LED to on or off
    led.set_low();


    // Application Loop
    loop {
        led.toggle();
        delay.delay_millis(500);
        led.toggle();
        // or using `fugit` duration
        delay.delay(2.secs());
    }
}