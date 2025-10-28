VA108xx Example Applications
========

This folder contains various examples
Consult the main README first for setup of the repository.

## Simple examples

```rs
cargo run --example blinky
```

You can have a look at the `simple/examples` folder to see all available simple examples

## RTIC example

```rs
cargo run --bin rtic-example
```

## Embassy example

Blinky with time driver IRQs in library

```rs
cargo run --bin embassy-example
```

Blinky with custom time driver IRQs

```rs
cargo run --bin embassy-example --no-default-features --features custom-irqs
```
