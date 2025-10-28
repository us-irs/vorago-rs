#![no_std]
use va108xx_hal::time::Hertz;

pub const SYSCLK_FREQ: Hertz = Hertz::from_raw(50_000_000);
