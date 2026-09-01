//! # Async API for the UART peripheral
//!
//! This module provides the [Tx] driver, which implements [embedded_io_async::Write]. It exposes
//! a `bank_id` accessor and an `on_interrupt` associated function: the token returned by
//! `bank_id` is meant to be stashed somewhere reachable from the interrupt handler (e.g. a
//! `Mutex<Cell<_>>`) and passed to `on_interrupt` there, so the handler does not need access to
//! the driver instance itself.
//!
//! There is no equivalent async RX driver here. Waiting for a specific transfer to complete, as
//! TX does, is naturally a one-shot [core::future::Future]. For reception, this gets a bit more
//! complicated. Users might care about receiving all data, and a user API driven reception
//! might miss data received between API read calls. The HAL support was kept simple and only
//! exposes an [crate::uart::RxWithInterrupt::on_interrupt] API to drain the current bytes in the
//! RX FIFO into a buffer from your own interrupt handler. You can then forward the bytes into a
//! queue of your choice e.g. an `embassy_sync::pipe::Pipe`, which already gives you an async
//! `read`. See [crate::uart::RxWithInterrupt] for details and the `async-uart-rx` examples for a
//! complete pattern.

pub mod tx;
pub use tx::*;
