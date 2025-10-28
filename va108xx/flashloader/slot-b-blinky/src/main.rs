//! Simple blinky example using the HAL
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va108xx_hal::{gpio::PinsA, pac, prelude::*, timer::CountdownTimer};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("VA108xx HAL blinky example for App Slot B");

    let mut dp = pac::Peripherals::take().unwrap();
    let mut timer = CountdownTimer::new(&mut dp.sysconfig, 50.MHz(), dp.tim0);
    let porta = PinsA::new(&mut dp.sysconfig, dp.porta);
    let mut led2 = porta.pa7.into_readable_push_pull_output();

    loop {
        led2.toggle();
        timer.delay_ms(1000);
    }
}
