//! Test image
//!
//! It would be nice to use a test framework like defmt-test, but I have issues
//! with probe run and it would be better to make the RTT work first
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
// Logging provider
use defmt_rtt as _;
// Panic provider
use panic_probe as _;
use va108xx_hal::{
    gpio::{regs::Gpio, Input, Output, PinState, Pull},
    pac,
    pins::{PinsA, PinsB, Port},
    prelude::*,
    time::Hertz,
    timer::CountdownTimer,
};

#[allow(dead_code)]
#[derive(Debug, defmt::Format)]
enum TestCase {
    // Tie PORTA[0] to PORTA[1] for these tests!
    TestBasic,
    TestPullup,
    TestPulldown,
    TestMask,
    // Tie PORTB[22] to PORTB[23] for this test
    PortB,
    Perid,
    // Tie PA0 to an oscilloscope and configure pulse detection
    Pulse,
    // Tie PA0, PA1 and PA3 to an oscilloscope
    DelayGpio,
    // PA0 can be checked with an oscillsope to verify timing correctness.
    DelayMs,
}

#[entry]
fn main() -> ! {
    defmt::println!("-- VA108xx Test Application --");
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let pinsa = PinsA::new(dp.porta);
    let pinsb = PinsB::new(dp.portb);
    let mut led1 = Output::new(pinsa.pa10, PinState::Low);
    let test_case = TestCase::DelayMs;

    match test_case {
        TestCase::TestBasic
        | TestCase::TestPulldown
        | TestCase::TestPullup
        | TestCase::TestMask => {
            defmt::info!(
                "Test case {:?}. Make sure to tie PORTA[0] to PORTA[1]",
                test_case
            );
        }
        _ => {
            defmt::info!("Test case {:?}", test_case);
        }
    }
    match test_case {
        TestCase::TestBasic => {
            // Tie PORTA[0] to PORTA[1] for these tests!
            let mut out = Output::new(pinsa.pa0, PinState::Low);
            let input = Input::new_floating(pinsa.pa1);
            out.set_high();
            assert!(input.is_high());
            out.set_low();
            assert!(input.is_low());
        }
        TestCase::TestPullup => {
            // Tie PORTA[0] to PORTA[1] for these tests!
            let input = Input::new_with_pull(pinsa.pa1, Pull::Up);
            assert!(input.is_high());
            let mut out = Output::new(pinsa.pa0, PinState::Low);
            out.set_low();
            assert!(input.is_low());
            out.set_high();
            assert!(input.is_high());
        }
        TestCase::TestPulldown => {
            // Tie PORTA[0] to PORTA[1] for these tests!
            let input = Input::new_with_pull(pinsa.pa1, Pull::Down);
            assert!(input.is_low());
            let mut out = Output::new(pinsa.pa0, PinState::Low);
            out.set_low();
            assert!(input.is_low());
            out.set_high();
            assert!(input.is_high());
        }
        TestCase::TestMask => {
            // Tie PORTA[0] to PORTA[1] for these tests!
            // Need to test this low-level..
            /*
            let mut input = Input::new_with_pull(pinsa.pa1, Pull::Down);
            input.clear_datamask();
            assert!(!input.datamask());
            let mut out = pinsa.pa0.into_push_pull_output();
            out.clear_datamask();
            assert!(input.is_low_masked().is_err());
            assert!(out.set_high_masked().is_err());
            */
        }
        TestCase::PortB => {
            // Tie PORTB[22] to PORTB[23] for these tests!
            let mut out = Output::new(pinsb.pb22, PinState::Low);
            let input = Input::new_floating(pinsb.pb23);
            out.set_high();
            assert!(input.is_high());
            out.set_low();
            assert!(input.is_low());
        }
        TestCase::Perid => {
            let mmio_porta = Gpio::new_mmio(Port::A);
            assert_eq!(mmio_porta.read_perid(), 0x004007e1);
            let mmio_porta = Gpio::new_mmio(Port::B);
            assert_eq!(mmio_porta.read_perid(), 0x004007e1);
        }
        TestCase::Pulse => {
            let mut output_pulsed = Output::new(pinsa.pa0, PinState::Low);
            output_pulsed.configure_pulse_mode(true, PinState::Low);
            defmt::info!("Pulsing high 10 times..");
            output_pulsed.set_low();
            for _ in 0..10 {
                output_pulsed.set_high();
                cortex_m::asm::delay(25_000_000);
            }
            output_pulsed.configure_pulse_mode(true, PinState::High);
            defmt::info!("Pulsing low 10 times..");
            for _ in 0..10 {
                output_pulsed.set_low();
                cortex_m::asm::delay(25_000_000);
            }
        }
        TestCase::DelayGpio => {
            let mut out_0 = Output::new(pinsa.pa0, PinState::Low);
            out_0.configure_delay(true, false);
            let mut out_1 = Output::new(pinsa.pa1, PinState::Low);
            out_1.configure_delay(false, true);
            let mut out_2 = Output::new(pinsa.pa3, PinState::Low);
            out_2.configure_delay(true, true);
            for _ in 0..20 {
                out_0.toggle();
                out_1.toggle();
                out_2.toggle();
                cortex_m::asm::delay(25_000_000);
            }
        }
        TestCase::DelayMs => {
            let mut delay_timer = CountdownTimer::new(dp.tim1, 50.MHz());
            let mut pa0 = Output::new(pinsa.pa0, PinState::Low);
            for _ in 0..5 {
                led1.toggle();
                delay_timer.delay_ms(500);
                led1.toggle();
                delay_timer.delay_ms(500);
            }
            let ahb_freq: Hertz = 50.MHz();
            let mut syst_delay = cortex_m::delay::Delay::new(cp.SYST, ahb_freq.raw());
            // Release image should be used to verify timings for pin PA0
            for _ in 0..5 {
                pa0.toggle();
                syst_delay.delay_us(50);
                pa0.toggle();
                syst_delay.delay_us(50);
                pa0.toggle();
                delay_timer.delay_us(50);
                pa0.toggle();
                delay_timer.delay_us(50);
            }
        }
    }

    defmt::info!("Test success");
    loop {
        led1.toggle();
        cortex_m::asm::delay(25_000_000);
    }
}
