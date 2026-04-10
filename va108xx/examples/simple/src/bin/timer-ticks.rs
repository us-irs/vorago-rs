//! MS and Second counter implemented using the TIM0 and TIM1 peripheral
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use portable_atomic::AtomicU32;
use va108xx_hal::{
    pac::{self, interrupt},
    prelude::*,
    time::Hertz,
    timer::{CountdownTimer, InterruptConfig},
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
    let mut delay = CountdownTimer::new(dp.tim2, 50.MHz());
    let mut last_ms = 0;
    defmt::info!("-- Vorago system ticks using timers --");
    let lib_type = LibType::Hal;
    match lib_type {
        LibType::Pac => {
            unsafe {
                dp.sysconfig
                    .peripheral_clk_enable()
                    .modify(|_, w| w.irqsel().set_bit());
                dp.sysconfig
                    .tim_clk_enable()
                    .modify(|r, w| w.bits(r.bits() | (1 << 0) | (1 << 1)));
                dp.irqsel.tim(0).write(|w| w.bits(0x00));
                dp.irqsel.tim(1).write(|w| w.bits(0x01));
            }

            let sys_clk: Hertz = 50.MHz();
            let cnt_ms = sys_clk.raw() / 1000 - 1;
            let cnt_sec = sys_clk.raw() - 1;
            unsafe {
                dp.tim0.cnt_value().write(|w| w.bits(cnt_ms));
                dp.tim0.rst_value().write(|w| w.bits(cnt_ms));
                dp.tim0.ctrl().write(|w| {
                    w.enable().set_bit();
                    w.irq_enb().set_bit()
                });
                dp.tim1.cnt_value().write(|w| w.bits(cnt_sec));
                dp.tim1.rst_value().write(|w| w.bits(cnt_sec));
                dp.tim1.ctrl().write(|w| {
                    w.enable().set_bit();
                    w.irq_enb().set_bit()
                });
                unmask_irqs();
            }
        }
        LibType::Hal => {
            let mut ms_timer = CountdownTimer::new(dp.tim0, 50.MHz());
            ms_timer.enable_interrupt(InterruptConfig::new(interrupt::OC0, true, true));
            ms_timer.start(1.kHz());
            let mut second_timer = CountdownTimer::new(dp.tim1, 50.MHz());
            second_timer.enable_interrupt(InterruptConfig::new(interrupt::OC1, true, true));
            second_timer.start(1.Hz());
        }
    }
    loop {
        let current_ms = MS_COUNTER.load(portable_atomic::Ordering::Relaxed);
        if current_ms - last_ms >= 1000 {
            // To prevent drift.
            last_ms += 1000;
            defmt::info!("MS counter: {}", current_ms);
            let second = SEC_COUNTER.load(portable_atomic::Ordering::Relaxed);
            defmt::info!("Second counter: {}", second);
        }
        delay.delay_ms(50);
    }
}

fn unmask_irqs() {
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC1);
    }
}

#[interrupt]
#[allow(non_snake_case)]
fn OC0() {
    MS_COUNTER.fetch_add(1, portable_atomic::Ordering::Relaxed);
}

#[interrupt]
#[allow(non_snake_case)]
fn OC1() {
    SEC_COUNTER.fetch_add(1, portable_atomic::Ordering::Relaxed);
}
