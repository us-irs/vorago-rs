//! API for the TIM peripherals
//!
//! ## Examples
//!
//! - [MS and second tick implementation](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va416xx/examples/simple/examples/timer-ticks.rs)
//! - [Cascade feature example](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va416xx/examples/simple/examples/cascade.rs)
pub use vorago_shared_hal::timer::*;

pub const TIM_IRQ_OFFSET: usize = 48;
