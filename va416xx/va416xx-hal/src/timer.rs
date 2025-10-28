//! API for the TIM peripherals
//!
//! ## Examples
//!
//! - [MS and second tick implementation](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/timer-ticks.rs)
//! - [Cascade feature example](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/examples/simple/examples/cascade.rs)
pub use vorago_shared_hal::timer::*;

pub const TIM_IRQ_OFFSET: usize = 48;
