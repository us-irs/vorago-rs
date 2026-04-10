//! GPIO support module.
//!
//! Contains abstractions to use the pins provided by the [crate::pins] module as GPIO or
//! IO peripheral pins.
//!
//! The core data structures provided for this are the
//!
//! - [Output] for push-pull output pins.
//! - [Input] for input pins.
//! - [Flex] for pins with flexible configuration requirements.
//! - [IoPeriphPin] for IO peripheral pins.
//!
//! The [crate::pins] module exposes singletons to access the [Pin]s required by this module
//! in a type-safe way.
pub use vorago_shared_hal::gpio::*;
