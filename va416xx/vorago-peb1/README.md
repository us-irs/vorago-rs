[![Crates.io](https://img.shields.io/crates/v/vorago-peb1)](https://crates.io/crates/vorago-peb1)
[![docs.rs](https://img.shields.io/docsrs/vorago-peb1)](https://docs.rs/vorago-peb1)

# Rust BSP for the Vorago PEB1 development board

This is the Rust **B**oard **S**upport **P**ackage crate for the Vorago PEB1 development board.
Its aim is to provide drivers for the board features of the PEB1 board.

The BSP builds on top of the [HAL crate for VA416xx devices](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/va416xx-hal).

## Notes on board revisions

On RevA, issuing the `monitor reset` command in the GDB application is problematic and will prevent
the flashed binary from working properly. On board revision B, this was not an issue.
For that reason, two different `*.gdb` files were provided in the `jlink` folder for each
board revision. If you are not using these files, make sure to correctly configure your flash
tools depending on which tool or board your are using.
