//! Pin resource management singletons.
//!
//! This module contains the pin singletons. It allows creating those singletons
//! to access the [Pin] structures of individual ports in a safe way with checked ownership
//! rules.
pub use vorago_shared_hal::pins::*;
