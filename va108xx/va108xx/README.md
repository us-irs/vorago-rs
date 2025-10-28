[![Crates.io](https://img.shields.io/crates/v/va108xx)](https://crates.io/crates/va108xx)
[![docs.rs](https://img.shields.io/docsrs/va108xx)](https://docs.rs/va108xx)

# PAC for the Vorago VA108xx microcontroller family

This repository contains the Peripheral Access Crate (PAC) for
Voragos VA108xx series of Cortex-M0 based microcontrollers.

The crate was generated using [`svd2rust`](https://github.com/rust-embedded/svd2rust).

If you are interested in higher-level abstractions, it is recommended you visit
the `va108xx-hal` HAL crate and the `vorago-reb1` BSP crate which build on top of this PAC.

## Usage

To use this crate, add this to your `Cargo.toml`

```toml
[dependencies.va108xx]
version = "<Most Recent Version>"
features = ["rt"]
```

The `rt` feature is optional and recommended. It brings in support for `cortex-m-rt`.

For full details on the autgenerated API, please see the
[svd2rust documentation](https://docs.rs/svd2rust/latest/svd2rust/#peripheral-api).

## Optional Features

- [`defmt`](https://defmt.ferrous-systems.com/): Add support for `defmt` by adding the
  [`defmt::Format`](https://defmt.ferrous-systems.com/format) derive on many types.
- `debug`: Add `Debug` derives for various structures

## Regenerating the PAC

If you want to re-generate the PAC, for example if the register file `va416xx.svd` changes
or the `svd2rust` version is updated, you can do some using the following these steps:

1. Make sure all necessary tools are installed: [`svd2rust`](https://docs.rs/svd2rust/latest/svd2rust/),
   [`svdtools`](https://github.com/rust-embedded/svdtools) and [`form`](https://crates.io/crates/form).
   You can install all tools with `cargo`:

   ```sh
   cargo install --locked svd2rust svdtools form
   ```

2. Patch the vendor-provided SVD file `svd/va41xx.svd`. This can be done using `svdtools` in
   conjunction with the `svd/va108xx-patch.yml` file.

   ```sh
   svdtools patch svd/va108xx-patch.yml
   ```

3. Use `svd2rust` to generate the Rust library

   ```sh
   svd2rust -i svd/va108xx.svd.patched
   ```

4. Use the `form` tool to split the generated `lib.rs` into individual modules.

    ```sh
    form -i lib.rs -o src/
    ```

The `gen-helper.sh` automates steps 2-4.
