//! API for the SPI peripheral.
//!
//! The main abstraction provided by this module is the [Spi] an structure.
//! It provides the [SpiBus trait](https://docs.rs/embedded-hal/latest/embedded_hal/spi/trait.SpiBus.html),
//! but also offer a low level interface via the [SpiLowLevel] trait.
//!
//! ## Examples
//!
//! - [Blocking SPI example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/examples/simple/examples/spi.rs)
//! - [REB1 ADC example](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/examples/max11519-adc.rs)
//! - [REB1 EEPROM library](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/vorago-reb1/src/m95m01.rs)
pub use vorago_shared_hal::spi::*;
