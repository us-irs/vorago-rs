[![Crates.io](https://img.shields.io/crates/v/va108xx-hal)](https://crates.io/crates/va108xx-hal)
[![docs.rs](https://img.shields.io/docsrs/va108xx-hal)](https://docs.rs/va108xx-hal)

# HAL for the Vorago VA108xx MCU family

This repository contains the **H**ardware **A**bstraction **L**ayer (HAL), which is an additional
hardware abstraction on top of the [peripheral access API](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/va108xx).

It is the result of reading the datasheet for the device and encoding a type-safe layer over the
raw PAC. This crate also implements traits specified by the
[embedded-hal](https://github.com/rust-embedded/embedded-hal) project, making it compatible with
various drivers in the embedded rust ecosystem.

In contrats to other HAL implementations, there is only one chip variant available here so there
is no need to pass the chip variant as a feature.

## Building

Building an application requires the `thumbv6m-none-eabi` cross-compiler toolchain.
If you have not installed it yet, you can do so with

```sh
rustup target add thumbv6m-none-eabi
```

After that, you can use `cargo build` to build the development version of the crate.

## Setting up your own binary crate

If you have a custom board, you might be interested in setting up a new binary crate for your
project. These steps aim to provide a complete list to get a binary crate working to flash
your custom board.

The hello world of embedded development is usually to blinky a LED. This example
is contained within the
[examples folder](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/examples/simple/examples/blinky.rs).

1. Set up your Rust cross-compiler if you have not done so yet. See more in the [build chapter](#Building)
2. Create a new binary crate with `cargo init`
3. To ensure that `cargo build` cross-compiles, it is recommended to create a `.cargo/config.toml`
   file. A sample `.cargo/config.toml` file is provided in this repository as well
4. Copy the `memory.x` file into your project. This file contains information required by the linker.
5. Copy the `blinky.rs` file to the `src/main.rs` file in your binary crate
6. You need to add some dependencies to your `Cargo.toml` file

   ```toml
	[dependencies]
	cortex-m = "<Compatible Version>"
	cortex-m-rt = "<Compatible Version>"
	panic-halt = "<Compatible Version>"
	embedded-hal = "<Compatible Version>"

	[dependencies.va108xx-hal]
	version = "<Most Recent Version>"
	features = ["rt"]
   ```

6. Build the application with `cargo build`

7. Flashing the board might work differently for different boards and there is usually
   more than one way. You can find example instructions in primary README.

## Embedded Rust

If you have not done this yet, it is recommended to read some of the excellent resources available
to learn Rust:

- [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- [Rust Discovery Book](https://docs.rust-embedded.org/discovery/)
