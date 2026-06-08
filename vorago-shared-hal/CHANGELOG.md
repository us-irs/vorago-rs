Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

### Changed

- Async TX UART and Async SPI driver constructor is explicitely marked `unsafe`.
- Async TX UART `write` now returns a `TxFuture`
- Empty async TX write resolves to `Poll::Ready(0)` immediately.
- Async SPI API now always returns futures instead of optional futures.

### Fixed

- Asynch drivers now borrow the buffers properly for the lifetime of the future.
- Asynch UART TX driver now borrows the TX peripheral for the duration of the future.

## [v0.4.0] 2026-05-19

### Changed

- Naming improvements for UART register module
- Improved UART Async TX module. Only enable TX below threshold interrupts if the FIFO
  actually needs to be refilled.

## [v0.3.0] 2026-05-18

### Added

- Add `is_high` and `is_low` for `InputPinAsync`.
- Add `InputPin` impl for `InputPinAsync`.
- `HwCsPin` in SPI module for easer usage of HW CS pins as `Output` CS pins

### Changed

- Bumped `fugit` from v0.3 to v0.4
- Added `RxWithInterrupt::steal`.
- Renamed UART `Data` register `value` field to `data`
- Improved type level support for resource management for SPI, PWM, UART.
- Renamed `tx_asynch` and `rx_asynch` module name to `*_async`
- Naming improvements in SPI module: replaced `cfg` by `config*`
- UART configuration now expects an explicit clock configuration structure and does not
  calculate it itself anymore.

### Fixed

- `Pull::Up` and `Pull::High` were inverted.
- Removed HW CS pin provider implementation for PA23, PA22 and PA21, which are multi HW CS pins.
- Added missing `AnyPin` trait impl for Multi HW CS pins.
- Expose inner `Input` pin for `InputPinAsync`.
- Bugfix for UART clock calculation with 8x baud mode.
- Possible bugfix for Asynch GPIO where the interrupt handler could become stuck in a loop.
- Robustness improvements for the Asynch GPIO driver code.

## [v0.2.0] 2025-09-03

Renamed to `vorago-shared-hal`

### Changed

- Various renaming to be more in-line with common Embedded Rust naming conventions.
  - `PinId` -> `DynPinId`
  - `PinIdProvider` -> `PinId`
  - `FunSel` -> `FunctionSelect`
  - `PinMarker` -> `AnyPin`
  - Peripheral traits renamed from `*Marker` to `*Instance`
  - `Clk` abbreviation in names changed to `Clock`
  - `Cmd` abbreviation in names changed to `Command`
  - `Irq` abbreviation in names changed to `Interrupt`

## [v0.1.0] 2025-09-02

Init commit.

[unreleased]: https://egit.irs.uni-stuttgart.de/rust/vorago-rs/compare/vorago-shared-hal-v0.4.0...HEAD
[v0.4.0]: https://egit.irs.uni-stuttgart.de/rust/vorago-rs/compare/vorago-shared-hal-v0.3.0...vorago-shared-hal-v0.4.0
[v0.3.0]: https://egit.irs.uni-stuttgart.de/rust/vorago-rs/src/tag/vorago-shared-hal-v0.3.0
[v0.2.0]: https://egit.irs.uni-stuttgart.de/rust/vorago-shared-hal/compare/v0.1.0...v0.2.0
[v0.1.0]: https://egit.irs.uni-stuttgart.de/rust/vorago-shared-hal/src/tag/v0.1.0
