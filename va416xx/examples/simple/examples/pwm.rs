//! Simple PWM example
//!
//! Outputs a PWM waveform on pin PG2.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, pwm::SetDutyCycle};
// Import panic provider.
use panic_probe as _;
// Import logger.
use defmt_rtt as _;
use simple_examples::peb1;
use va416xx_hal::{
    clock::ClockConfigurator,
    pac,
    pins::PinsG,
    prelude::*,
    pwm::{get_duty_from_percent, PwmA, PwmB, PwmPin},
    timer::CountdownTimer,
};

#[entry]
fn main() -> ! {
    defmt::println!("-- VA108xx PWM example application--");
    let dp = pac::Peripherals::take().unwrap();

    // Use the external clock connected to XTAL_N.
    let clocks = ClockConfigurator::new(dp.clkgen)
        .xtal_n_clk_with_src_freq(peb1::EXTCLK_FREQ)
        .freeze()
        .unwrap();

    let pinsg = PinsG::new(dp.portg);
    let mut pwm = PwmPin::new(pinsg.pg2, dp.tim9, &clocks, 10.Hz()).unwrap();
    let mut delay_timer = CountdownTimer::new(dp.tim0, &clocks);
    let mut current_duty_cycle = 0.0;
    pwm.set_duty_cycle(get_duty_from_percent(current_duty_cycle))
        .unwrap();
    pwm.enable();

    // Delete type information, increased code readibility for the rest of the code
    loop {
        let mut counter = 0;
        // Increase duty cycle continuously
        while current_duty_cycle < 1.0 {
            delay_timer.delay_ms(400);
            current_duty_cycle += 0.02;
            counter += 1;
            if counter % 10 == 0 {
                defmt::info!("current duty cycle: {}", current_duty_cycle);
            }

            pwm.set_duty_cycle(get_duty_from_percent(current_duty_cycle))
                .unwrap();
        }

        // Switch to PWMB and decrease the window with a high signal from 100 % to 0 %
        // continously
        current_duty_cycle = 0.0;
        let mut upper_limit = 1.0;
        let mut lower_limit = 0.0;
        let mut pwmb: PwmPin<PwmB> = PwmPin::from(pwm);
        pwmb.set_pwmb_lower_limit(get_duty_from_percent(lower_limit));
        pwmb.set_pwmb_upper_limit(get_duty_from_percent(upper_limit));
        while lower_limit < 0.5 {
            delay_timer.delay_ms(400);
            lower_limit += 0.01;
            upper_limit -= 0.01;
            pwmb.set_pwmb_lower_limit(get_duty_from_percent(lower_limit));
            pwmb.set_pwmb_upper_limit(get_duty_from_percent(upper_limit));
            defmt::info!("Lower limit: {}", pwmb.pwmb_lower_limit());
            defmt::info!("Upper limit: {}", pwmb.pwmb_upper_limit());
        }
        pwm = PwmPin::<PwmA>::from(pwmb);
    }
}
