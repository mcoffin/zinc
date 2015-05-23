#![feature(plugin, start, asm)]
#![no_std]

extern crate zinc;

use zinc::hal::sam3x::{pin, system_clock, watchdog};
use zinc::hal::pin::Gpio;
use zinc::hal::pin::GpioDirection;
use zinc::hal::pin::GpioConf;
use zinc::hal::cortex_m3::systick;
use core::option::Option::*;
use core::convert::From;

#[path="../../../src/util/wait_for.rs"]
#[macro_use] mod wait_for;

const WAIT_TIME: u32 = 1000;

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    zinc::hal::mem_init::init_stack();
    zinc::hal::mem_init::init_data();

    let mck_freq = system_clock::init_clock(system_clock::ClockSource::Main(Some(12_000_000)),
                             Some(system_clock::Pll {
                                 mul: 0x1,
                                 div: 0x1,
                                 count: 0x3f,
                             }));
    systick::setup(mck_freq / 1000);
    systick::enable();

    watchdog::watchdog_disable();

    main();
    0
}

fn wait(mut ticks: u32) {
    systick::tick();
    while ticks > 0 {
        wait_for!(systick::tick());
        ticks -= 1;
    }
}

const LED_CONF: GpioConf = GpioConf {
    index: 27 + 32,
    direction: GpioDirection::Out,
};
pub fn main() {
    let led = pin::Pin::from(LED_CONF);
    led.set_high();
    loop {
        wait(WAIT_TIME);
        led.set_low();
        wait(WAIT_TIME);
        led.set_high();
    }
}
