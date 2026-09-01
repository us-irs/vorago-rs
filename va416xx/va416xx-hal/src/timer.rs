//! API for the TIM peripherals
pub use vorago_shared_hal::timer::*;

/// Offset of the first timer interrupt in the NVIC interrupt table.
pub const TIM_IRQ_OFFSET: usize = 48;
