//! # API for the UART peripheral
//!
//! The core of this API are the [Uart], [Rx] and [Tx] structures.
//! The RX structure also has a dedicated [RxWithInterrupt] variant which allows reading the receiver
//! using interrupts.
//!
//! The [rx_async] and [tx_async] modules provide an asynchronous non-blocking API for the UART
//! peripheral.
//!
//! ## Examples
//!
//! - [UART simple example](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/examples/simple/examples/uart.rs)
//! - [UART with IRQ and RTIC](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/examples/rtic/src/bin/uart-echo-rtic.rs)
//! - [Flashloader exposing a CCSDS interface via UART](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/flashloader)
//! - [Async UART RX example](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/examples/embassy/src/bin/async-uart-rx.rs)
//! - [Async UART TX example](https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/branch/main/va108xx/examples/embassy/src/bin/async-uart-tx.rs)
pub use vorago_shared_hal::uart::*;
