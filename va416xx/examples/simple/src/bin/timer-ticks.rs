//! MS and Second counter implemented using the TIM0 and TIM1 peripheral
#![no_main]
#![no_std]
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::asm;
use cortex_m_rt::entry;
use simple_examples::peb1;
use va416xx_hal::{
    clock::ClockConfigurator,
    irq_router::enable_and_init_irq_router,
    pac::{self, interrupt},
    prelude::*,
    timer::CountdownTimer,
};

#[allow(dead_code)]
enum LibType {
    Pac,
    Hal,
}

static MS_COUNTER: AtomicU32 = AtomicU32::new(0);
static SEC_COUNTER: AtomicU32 = AtomicU32::new(0);

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut last_ms = 0;
    defmt::println!("-- Vorago system ticks using timers --");
    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();
    enable_and_init_irq_router();
    let mut ms_timer = CountdownTimer::new(dp.tim0, &clocks);
    ms_timer.enable_interrupt(true);
    ms_timer.start(1.Hz());
    let mut second_timer = CountdownTimer::new(dp.tim1, &clocks);
    second_timer.enable_interrupt(true);
    second_timer.start(1.Hz());
    loop {
        let current_ms = MS_COUNTER.load(Ordering::Relaxed);
        if current_ms >= last_ms + 1000 {
            // To prevent drift.
            last_ms += 1000;
            defmt::info!("MS counter: {}", current_ms);
            let second = SEC_COUNTER.load(Ordering::Relaxed);
            defmt::info!("Second counter: {}", second);
        }
        asm::delay(1000);
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn TIM0() {
    MS_COUNTER.fetch_add(1, Ordering::Relaxed);
}

#[interrupt]
#[allow(non_snake_case)]
fn TIM1() {
    SEC_COUNTER.fetch_add(1, Ordering::Relaxed);
}
