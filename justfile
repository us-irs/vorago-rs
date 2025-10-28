all: check-all \
    build-all \
    fmt-all \
    clippy-all \
    docs-all

check-all: check-va108xx check-va416xx
build-all: build-va108xx build-va416xx
fmt-all: fmt-va108xx fmt-va416xx
clippy-all: clippy-va108xx clippy-va416xx
docs-all: docs-va108xx docs-va416xx
clean-all: clean-va108xx clean-va416xx

[working-directory: 'va108xx']
check-va108xx:
  cargo check --target thumbv6m-none-eabi
  cargo check --target thumbv6m-none-eabi --examples
  cargo check -p va108xx --target thumbv6m-none-eabi --all-features
  cargo check -p va108xx-hal --target thumbv6m-none-eabi --features "defmt"

[working-directory: 'va416xx']
check-va416xx:
  cargo check --target thumbv7em-none-eabihf
  cargo check --target thumbv7em-none-eabihf --examples
  cargo check -p va416xx --target thumbv7em-none-eabihf --all-features
  cargo check -p va416xx-hal --target thumbv7em-none-eabihf --features "defmt va41630"

[working-directory: 'va108xx']
build-va108xx:
  cargo build --target thumbv6m-none-eabi

[working-directory: 'va416xx']
build-va416xx:
  cargo build --target thumbv7em-none-eabihf

[working-directory: 'va108xx']
fmt-va108xx:
  cargo fmt --all -- --check

[working-directory: 'va416xx']
fmt-va416xx:
  cargo fmt --all -- --check

[working-directory: 'va108xx']
clippy-va108xx:
  cargo clippy --target thumbv6m-none-eabi -- -D warnings

[working-directory: 'va416xx']
clippy-va416xx:
  cargo clippy --target thumbv7em-none-eabihf -- -D warnings

[working-directory: 'va108xx']
docs-va108xx:
  RUSTDOCFLAGS="--cfg docsrs --generate-link-to-definition -Z unstable-options" cargo +nightly doc -p va108xx --all-features
  RUSTDOCFLAGS="--cfg docsrs --generate-link-to-definition -Z unstable-options" cargo +nightly doc -p va108xx-hal --all-features
  RUSTDOCFLAGS="--cfg docsrs --generate-link-to-definition -Z unstable-options" cargo +nightly doc -p vorago-reb1

[working-directory: 'va416xx']
docs-va416xx:
  RUSTDOCFLAGS="--cfg docsrs --generate-link-to-definition -Z unstable-options" cargo +nightly doc -p vorago-peb1
  RUSTDOCFLAGS="--cfg docsrs --generate-link-to-definition -Z unstable-options" cargo +nightly doc -p va416xx-hal --features va41630
  RUSTDOCFLAGS="--cfg docsrs --generate-link-to-definition -Z unstable-options" cargo +nightly doc -p va416xx

[working-directory: 'va108xx']
clean-va108xx:
  cargo clean

[working-directory: 'va416xx']
clean-va416xx:
  cargo clean
