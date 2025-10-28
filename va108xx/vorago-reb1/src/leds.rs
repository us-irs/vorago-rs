//! # API for using the REB1 LEDs
//!
//! ## Examples
//!
//! - [LED example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/examples/blinky-leds.rs)
//! - [Button Blinky using IRQs](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/examples/blinky-button-irq.rs)
//! - [Button Blinky using IRQs and RTIC](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/examples/blinky-button-rtic.rs)
use va108xx_hal::{
    gpio::{Output, PinState},
    pins::{Pa10, Pa6, Pa7, Pin},
};

#[derive(Debug)]
pub struct Leds(pub [Led; 3]);

impl Leds {
    pub fn new(led_pin1: Pin<Pa10>, led_pin2: Pin<Pa7>, led_pin3: Pin<Pa6>) -> Leds {
        Leds([
            Led(Output::new(led_pin1, PinState::Low)),
            Led(Output::new(led_pin2, PinState::Low)),
            Led(Output::new(led_pin3, PinState::Low)),
        ])
    }
}

impl core::ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.0
    }
}

impl core::ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.0
    }
}

impl core::ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.0[i]
    }
}

impl core::ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.0[i]
    }
}

#[derive(Debug)]
pub struct Led(Output);

impl Led {
    /// Turns the LED off. Setting the pin high actually turns the LED off
    #[inline]
    pub fn off(&mut self) {
        self.0.set_high();
    }

    /// Turns the LED on. Setting the pin low actually turns the LED on
    #[inline]
    pub fn on(&mut self) {
        self.0.set_low();
    }

    /// Toggles the LED
    #[inline]
    pub fn toggle(&mut self) {
        self.0.toggle();
    }
}
