[![Crates.io](https://img.shields.io/crates/v/va416xx)](https://crates.io/crates/va416xx)
[![docs.rs](https://img.shields.io/docsrs/va416xx)](https://docs.rs/va416xx)

# PAC for the Vorago VA416xx microcontroller family

This repository contains the Peripheral Access Crate (PAC) for
Voragos VA416xx series of Cortex-M4 based microcontrollers.

The crate was generated using [`svd2rust`](https://github.com/rust-embedded/svd2rust).

## Usage

To use this crate, add this to your `Cargo.toml`

```toml
[dependencies.va416xx]
version = "<MostRecentVersion>"
features = ["rt"]
```

The `rt` feature is optional and recommended. It brings in support for `cortex-m-rt`.

For full details on the autgenerated API, you can read the
[svd2rust documentation](https://docs.rs/svd2rust/latest/svd2rust/).

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
   conjunction with the `svd/va416xx-patch.yml` file.

   ```sh
   svdtools patch svd/va416xx-patch.yml
   ```

3. Use `svd2rust` to generate the Rust library

   ```sh
   svd2rust -i svd/va416xx.svd.patched
   ```

4. Use the `form` tool to split the generated `lib.rs` into individual modules.

    ```sh
    form -i lib.rs -o src/
    ```

The `gen-helper.sh` automates steps 2-4.
