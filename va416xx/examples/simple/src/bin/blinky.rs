//! Simple blinky example using the HAL
#![no_main]
#![no_std]

// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use cortex_m_rt::entry;
use va416xx_hal::{
    gpio::{Output, PinState},
    pac,
    pins::PinsG,
};

#[entry]
fn main() -> ! {
    defmt::println!("VA416xx HAL blinky example");

    let dp = pac::Peripherals::take().unwrap();
    let portg = PinsG::new(dp.portg);
    let mut led = Output::new(portg.pg5, PinState::Low);
    loop {
        cortex_m::asm::delay(2_000_000);
        led.toggle();
    }
}
