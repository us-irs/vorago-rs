//! Simple blinky example
//!
//! Additional note on LEDs when using the REB1 development board:
//! Be not afraid: Pulling the GPIOs low makes the LEDs blink. See REB1
//! schematic for more details.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use va108xx_hal::{
    gpio::{Output, PinState},
    pac::{self},
    pins::PinsA,
    prelude::*,
    timer::CountdownTimer,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut delay = CountdownTimer::new(dp.tim1, 50.MHz());
    let porta = PinsA::new(dp.porta);
    let mut led1 = Output::new(porta.pa10, PinState::Low);
    let mut led2 = Output::new(porta.pa7, PinState::Low);
    let mut led3 = Output::new(porta.pa6, PinState::Low);
    for _ in 0..10 {
        led1.set_low();
        led2.set_low();
        led3.set_low();
        delay.delay_ms(200);
        led1.set_high();
        led2.set_high();
        led3.set_high();
        delay.delay_ms(200);
    }
    loop {
        led1.toggle();
        delay.delay_ms(200);
        led2.toggle();
        delay.delay_ms(200);
        led3.toggle();
        delay.delay_ms(200);
    }
}
