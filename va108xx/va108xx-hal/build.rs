//! Copies `.cargo/config.toml.template` to `.cargo/config.toml` for the `va108xx` workspace on
//! first build, so a fresh clone gets a working target/runner configuration without a manual
//! `cp` step. Best-effort only: if the template is not found (e.g. this crate was pulled in as a
//! regular dependency rather than built from within the `va108xx` workspace) or the copy fails
//! for any reason, this silently does nothing rather than failing the build.
//!
//! Note that this can not fix the *current* `cargo build` invocation if `config.toml` was still
//! missing when it started, since Cargo resolves the build target before any build script runs.
//! It only prepares the config for the next invocation.
use std::path::Path;

fn main() {
    let template = Path::new("../.cargo/config.toml.template");
    let config = Path::new("../.cargo/config.toml");
    if template.is_file() && !config.exists() {
        let _ = std::fs::copy(template, config);
    }
    println!("cargo:rerun-if-changed={}", template.display());
}
