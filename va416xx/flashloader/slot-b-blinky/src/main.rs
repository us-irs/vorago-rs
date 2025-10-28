//! Simple blinky example using the HAL
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::StatefulOutputPin;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use va416xx_hal::{gpio::PinsG, pac};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("VA416xx HAL blinky example for App Slot B");

    let mut dp = pac::Peripherals::take().unwrap();
    let portg = PinsG::new(&mut dp.sysconfig, dp.portg);
    let mut led = portg.pg5.into_readable_push_pull_output();
    loop {
        cortex_m::asm::delay(8_000_000);
        led.toggle().ok();
    }
}
