Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

### Added

### Changed

- Added `RxWithInterrupt::steal`.
- Improved type level support for resource management for SPI, PWM, UART.
- Removed HW CS pin provider implementation for PA23, PA22 and PA21, which are multi HW CS pins.

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

[unreleased]: https://egit.irs.uni-stuttgart.de/rust/vorago-shared-hal/compare/v0.2.0...HEAD
[v0.2.0]: https://egit.irs.uni-stuttgart.de/rust/vorago-shared-hal/compare/v0.1.0...v0.2.0
[v0.1.0]: https://egit.irs.uni-stuttgart.de/rust/vorago-shared-hal/src/tag/v0.1.0
