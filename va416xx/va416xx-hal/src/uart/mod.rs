//! # API for the UART peripheral
//!
//! The core of this API are the [Uart], [Rx] and [Tx] structures.
//! The RX structure also has a dedicated [RxWithInterrupt] variant which allows reading the receiver
//! using interrupts.
//!
//! The [rx_async] and [tx_async] modules provide an asynchronous non-blocking API for the UART
//! peripheral.
pub use vorago_shared_hal::uart::*;
