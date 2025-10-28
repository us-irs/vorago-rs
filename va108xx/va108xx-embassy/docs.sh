#!/bin/bash
export RUSTDOCFLAGS="--cfg docsrs --generate-link-to-definition -Z unstable-options"
cargo +nightly doc --open
