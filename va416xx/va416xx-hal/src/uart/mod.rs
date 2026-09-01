//! # API for the UART peripheral
//!
//! The core of this API are the [Uart], [Rx] and [Tx] structures.
//! The RX structure also has a dedicated [RxWithInterrupt] variant which allows reading the receiver
//! using interrupts.
//!
//! The [asynch] module provides an asynchronous, non-blocking TX driver. See its docs for how
//! to build async RX reception on top of [RxWithInterrupt] instead.
pub use vorago_shared_hal::uart::*;
