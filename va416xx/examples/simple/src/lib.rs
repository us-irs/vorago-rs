#![no_std]

/// PEB1 board specific configuration.
pub mod peb1 {
    use va416xx_hal::time::Hertz;

    // The clock on the PEB1 board has a 20 MHz clock which is increased to 40 MHz with a configurable
    // PLL by default.
    pub const EXTCLK_FREQ: Hertz = Hertz::from_raw(40_000_000);
    pub const XTAL_FREQ: Hertz = Hertz::from_raw(10_000_000);
}
