Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

## [v0.9.0] 2025-09-03

- Bumped `va108xx-hal` dependency to 0.12

## [v0.8.1] 2025-03-07

- Bumped allowed `va108xx-hal` dependency to 0.11
- Bumped `bitfield` dependency

## [v0.8.0] 2025-02-17

- Bumped `va108xx-hal` dependency to 0.10

## [v0.7.0] 2025-02-13

- Bumped `va108xx-hal` dependency to 0.9
- Minor adjustments to `Button` API.
- `Button`, `Led` and `Leds` now simply wrap a type using a tuple struct.

## [v0.6.0] 2024-09-30

- Added M95M01 EEPROM module/API
- Update `va108xx-hal` dependency to range >=v0.8, <0.9

## [v0.5.1] 2024-07-04

- Update `va108xx-hal` dependency to v0.7.0

## [v0.5.0] 2024-06-16

- Updated `va108xx` to v0.3.0 and `va108xx-hal` dependency to v0.6.0

## [v0.4.0]

- Update manifest file to have correct links and license
- Update some dependencies
  - `cortex-m-rtic` (dev-depencency) to 1.1.2
  - Other dependencies: Only revision has changed

## [v0.3.2]

- Bump HAL dependency to v0.5.0. Changed API, especially for IRQ handling

## [v0.3.1]

- Updated ADC code and dependency

## [v0.3.0]

- Completed baseline features to support all sensors on the REB1 sevice
- Relicensed as Apache-2.0 and moved to https://egit.irs.uni-stuttgart.de/rust/vorago-reb1

## [v0.2.3]

- Added basic accelerometer example. Board in not populated so it is not complete, but
  it provides a starting point
- Added ADC base library and example building on the new max116xx-10bit device driver crate

[unreleased]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/vorago-reb1-v0.9.0...HEAD
[v0.9.0]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/vorago-reb1-v0.8.1...vorago-reb1-v0.9.0
[v0.8.1]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/vorago-reb1-v0.8.0...vorago-reb1-v0.8.1
[v0.8.0]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/vorago-reb1-v0.7.0...vorago-reb1-v0.8.0
[v0.7.0]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/vorago-reb1-v0.6.0...vorago-reb1-v0.7.0
[v0.6.0]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/vorago-reb1-v0.5.0...vorago-reb1-v0.6.0
