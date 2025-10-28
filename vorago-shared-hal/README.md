[![Crates.io](https://img.shields.io/crates/v/vorago-shared-hal)](https://crates.io/crates/vorago-shared-hal)

Vorago Shared Peripherals
========

Peripheral drivers shared between Vorago families.

This library should not used directly. Instead, use the re-exported modules of the repective
[VA108xx HAL](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/va108xx-hal) and
[VA416xx HAL](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs).

## Check / Build for VA1XXX family

```sh
cargo check --features "vor1x, defmt"
```

## Check / Build for VA4XXX family

```sh
cargo check --features "vor4x, defmt"
```
