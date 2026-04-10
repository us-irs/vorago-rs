//! Simple Cascade example
//!
//! A timer will be periodically started which starts another timer via the cascade feature.
//! This timer will then start another timer with the cascade feature as well.
#![no_main]
#![no_std]
#![allow(non_snake_case)]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use va108xx_hal::{
    pac::{self, interrupt},
    prelude::*,
    timer::{CascadeControl, CascadeSelect, CascadeSource, CountdownTimer, InterruptConfig},
};

#[entry]
fn main() -> ! {
    defmt::println!("-- VA108xx Cascade example application--");

    let dp = pac::Peripherals::take().unwrap();
    let mut delay = CountdownTimer::new(dp.tim0, 50.MHz());

    // Will be started periodically to trigger a cascade
    let mut cascade_triggerer = CountdownTimer::new(dp.tim3, 50.MHz());
    cascade_triggerer.auto_disable(true);
    cascade_triggerer.enable_interrupt(InterruptConfig::new(pac::Interrupt::OC1, true, false));
    cascade_triggerer.enable();

    // First target for cascade
    let mut cascade_target_1 = CountdownTimer::new(dp.tim4, 50.MHz());
    cascade_target_1.auto_deactivate(true);
    cascade_target_1
        .cascade_source(CascadeSelect::Csd0, CascadeSource::Tim(3))
        .unwrap();
    let mut csd_cfg = CascadeControl {
        enable_src_0: true,
        trigger_mode_0: true,
        ..Default::default()
    };
    cascade_target_1.cascade_control(csd_cfg);
    // Normally it should already be sufficient to activate IRQ in the CTRL
    // register but a full interrupt is use here to display print output when
    // the timer expires
    cascade_target_1.enable_interrupt(InterruptConfig::new(pac::Interrupt::OC2, true, false));
    // The counter will only activate when the cascade signal is coming in so
    // it is okay to call start here to set the reset value
    cascade_target_1.start(1.Hz());

    // Activated by first cascade target
    let mut cascade_target_2 = CountdownTimer::new(dp.tim5, 50.MHz());
    cascade_target_2.auto_deactivate(true);
    // Set TIM4 as cascade source
    cascade_target_2
        .cascade_source(CascadeSelect::Csd1, CascadeSource::Tim(4))
        .unwrap();

    csd_cfg = CascadeControl::default();
    csd_cfg.enable_src_1 = true;
    // Use trigger mode here
    csd_cfg.trigger_mode_1 = true;
    cascade_target_2.cascade_control(csd_cfg);
    // Normally it should already be sufficient to activate IRQ in the CTRL
    // register but a full interrupt is use here to display print output when
    // the timer expires
    cascade_target_2.enable_interrupt(InterruptConfig::new(pac::Interrupt::OC3, true, false));
    // The counter will only activate when the cascade signal is coming in so
    // it is okay to call start here to set the reset value
    cascade_target_2.start(1.Hz());

    // Unpend all IRQs
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC1);
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC2);
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC3);
    }

    loop {
        defmt::info!("-- Triggering cascade in 0.5 seconds --");
        cascade_triggerer.start(2.Hz());
        delay.delay_ms(5000);
    }
}

#[interrupt]
fn OC1() {
    static mut IDX: u32 = 0;
    defmt::info!("{}: Cascade trigger timed out", &IDX);
    *IDX += 1;
}

#[interrupt]
fn OC2() {
    static mut IDX: u32 = 0;
    defmt::info!("{}: First cascade target timed out", &IDX);
    *IDX += 1;
}

#[interrupt]
fn OC3() {
    static mut IDX: u32 = 0;
    defmt::info!("{}: Second cascade target timed out", &IDX);
    *IDX += 1;
}
