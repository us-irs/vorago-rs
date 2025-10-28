Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

## [v0.6.0] 2025-09-03

- Re-generated PAC with `svd2rust` v0.37.0

## [v0.5.1] 2025-07-22

defmt version v1

## [v0.5.0] 2025-02-17

- Re-generated PAC with `svd2rust` v0.35.0 and added optional `defmt` and `Debug` implementations

## [v0.4.0] 2025-02-12

- Re-generated PAC with `svd2rust` v0.35.0

## [v0.3.0] 2024-06-16

- Re-generated PAC with `svd2rust` v0.33.3

## [v0.2.4]

- Added missing bitfield `CSDTRG2` in `CSD_CTRL` register of `TIM0` peripheral

## [v0.2.3]

- Added peripheral reset fields for `PERIPHERAL_RESET` register

## [v0.2.2]

- README tweks

## [v0.2.1]

- Some README and Manifest weaks

## [v0.2.0]

- Authorative repository was transferred to https://egit.irs.uni-stuttgart.de/rust/va108xx-rs but
  there still will be a GitHub mirror. Project relicensed as Apache-2.0 only

## [v0.1.3]

### Added

- Added two missing bit fields for I2CA STATUS register: I2CIDLE and IDLE

### Fixed

- Made I2CA STATUS register read-only

## [v0.1.2]

### Fixed

- Generated with patched version of `svd2rust`: See
  https://github.com/rust-embedded/svd2rust/pull/549 for more details.
  Some bitmasks were missing from register reader definitions.

## [v0.1.1]

- Relicensed under dual Apache-2.0 / MIT license

### Changed

- SVD file handling improved and new fields added for the peripheral
  clock enable register

### Added

- Helper script to automate all steps for PAC generation
- Added badges for README

## [v0.1.0]

### Added

- First version of the PAC which builds. Uses a patched version
  of `svd2rust`: https://github.com/rust-embedded/svd2rust

[unreleased]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/va108xx-v0.6.0...HEAD
[v0.6.0]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/va108xx-v0.5.1...va108xx-v0.6.0
[v0.5.1]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/va108xx-v0.5.0...va108xx-v0.5.1
[v0.5.0]: https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/compare/va108xx-v0.4.0...va108xx-v0.5.0
