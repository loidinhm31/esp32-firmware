#![no_std]
#![no_main]

#[allow(unused_imports)]
use esp_backtrace as _;

use esp_hal::{
    clock::ClockControl,
    gpio::{Io},
    ledc::{
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
        LSGlobalClkSource,
        Ledc,
        LowSpeed,
    },
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    delay::Delay,
};

use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    println!("Init");

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let led = io.pins.gpio7;

    let mut ledc = Ledc::new(peripherals.LEDC, &clocks);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.get_timer::<LowSpeed>(timer::Number::Timer0);

    let duty_cycle_resolution = timer::config::Duty::Duty14Bit;

    lstimer0
        .configure(timer::config::Config {
            duty: duty_cycle_resolution,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 50.Hz(),
        })
        .unwrap();

    let mut channel0 = ledc.get_channel(channel::Number::Channel0, led);


    let max_duty = get_max_duty(duty_cycle_resolution); // Get max duty value
    println!("Max Duty {}", max_duty);

    let neutral_duty = max_duty * 75 / 1000; // ~7.5% duty (neutral, stop)

    let min_limit = max_duty * 50 / 1000; // ~5% duty
    println!("Min Limit {}", min_limit);
    let max_limit = max_duty * 100 / 1000; // ~10% duty
    println!("Max Limit {}", max_limit);

    channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: 5,
            pin_config: channel::config::PinConfig::PushPull,
        })
        .unwrap();

    let delay = Delay::new(&clocks);

    loop {
        for angle in 0..=180 {
            channel0.set_duty_hw(map(angle, 0, 180, min_limit, max_limit));
            delay.delay_millis(12);
        }

        for angle in (0..=180).rev() {
            channel0.set_duty_hw(map(angle, 0, 180, min_limit, max_limit));
            delay.delay_millis(12);
        }
    }
}

fn get_max_duty(duty_cycle_resolution: timer::config::Duty) -> u32 {
    // For 14-bit resolution (Duty14Bit), the max duty value is 16383
    (1 << duty_cycle_resolution as u32) - 1
}

// Function that maps one range to another
fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

fn map_duty_for_angle(angle: u32) -> u32 {
    let ms = 1000 + (angle * 1000) / 90;
    let duty = (16383 * ms) / 20000;
    duty
}