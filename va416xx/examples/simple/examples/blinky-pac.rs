//! Simple blinky example
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use va416xx_hal::pac;

// Mask for the LED
const LED_PG5: u32 = 1 << 5;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // Enable all peripheral clocks
    dp.sysconfig
        .peripheral_clk_enable()
        .modify(|_, w| unsafe { w.bits(0xffffffff) });
    dp.portg.dir().modify(|_, w| unsafe { w.bits(LED_PG5) });
    dp.portg
        .datamask()
        .modify(|_, w| unsafe { w.bits(LED_PG5) });
    for _ in 0..10 {
        dp.portg.clrout().write(|w| unsafe { w.bits(LED_PG5) });
        cortex_m::asm::delay(2_000_000);
        dp.portg.setout().write(|w| unsafe { w.bits(LED_PG5) });
        cortex_m::asm::delay(2_000_000);
    }
    loop {
        dp.portg.togout().write(|w| unsafe { w.bits(LED_PG5) });
        cortex_m::asm::delay(2_000_000);
    }
}
