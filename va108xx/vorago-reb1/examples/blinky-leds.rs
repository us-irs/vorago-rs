//! Blinky examples using the PAC directly, the HAL, or the BSP
//!
//! Additional note on LEDs:
//! Be not afraid: Pulling the GPIOs low makes the LEDs blink. See REB1
//! schematic for more details.
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use panic_halt as _;
use va108xx_hal::{
    gpio::{Output, PinState},
    pac,
    pins::PinsA,
    prelude::*,
    timer::CountdownTimer,
};
use vorago_reb1::leds::Leds;

// REB LED pin definitions. All on port A
const LED_D2: u32 = 1 << 10;
const LED_D3: u32 = 1 << 7;
const LED_D4: u32 = 1 << 6;

#[allow(dead_code)]
enum LibType {
    Pac,
    Hal,
    Bsp,
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let lib_type = LibType::Bsp;

    match lib_type {
        LibType::Pac => {
            // Enable all peripheral clocks
            dp.sysconfig
                .peripheral_clk_enable()
                .modify(|_, w| unsafe { w.bits(0xffffffff) });
            dp.porta
                .dir()
                .modify(|_, w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
            dp.porta
                .datamask()
                .modify(|_, w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
            for _ in 0..10 {
                dp.porta
                    .clrout()
                    .write(|w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
                cortex_m::asm::delay(5_000_000);
                dp.porta
                    .setout()
                    .write(|w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
                cortex_m::asm::delay(5_000_000);
            }
            loop {
                dp.porta
                    .togout()
                    .write(|w| unsafe { w.bits(LED_D2 | LED_D3 | LED_D4) });
                cortex_m::asm::delay(25_000_000);
            }
        }
        LibType::Hal => {
            let pins = PinsA::new(dp.porta);
            let mut led1 = Output::new(pins.pa10, PinState::Low);
            let mut led2 = Output::new(pins.pa7, PinState::Low);
            let mut led3 = Output::new(pins.pa6, PinState::Low);
            let mut delay = CountdownTimer::new(dp.tim0, 50.MHz());
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
            }
        }
        LibType::Bsp => {
            let pinsa = PinsA::new(dp.porta);
            let mut leds = Leds::new(pinsa.pa10, pinsa.pa7, pinsa.pa6);
            let mut delay = CountdownTimer::new(dp.tim0, 50.MHz());
            for _ in 0..10 {
                // Blink all LEDs quickly
                for led in leds.iter_mut() {
                    led.toggle();
                }
                delay.delay_ms(500);
            }
            // Now use a wave pattern
            loop {
                for led in leds.iter_mut() {
                    led.toggle();
                    delay.delay_ms(200);
                }
            }
        }
    }
}
