//! A test child's whole environment, built from nothing, and the names cargo reads.
#![allow(dead_code, reason = "each test file uses part of the helper")]
use std::process::Command;

pub(crate) const CARGO: &[&str] = &[
    "HOME",
    "CARGO_HOME",
    "RUSTUP_HOME",
    "RUSTUP_TOOLCHAIN",
    "CARGO_TARGET_DIR",
    "RUSTC_WRAPPER",
];

/// A `Command` whose environment holds `PATH` and `keep`; a secret or `THINKTHEN_` name panics.
pub(crate) fn command(program: &str, keep: &[&str]) -> Command {
    let mut command = Command::new(program);
    command.env_clear();
    for name in std::iter::once(&"PATH").chain(keep) {
        let upper = name.to_uppercase();
        let secret = ["KEY", "TOKEN", "SECRET", "PASSWORD", "CREDENTIAL", "AUTH"];
        assert!(
            !upper.starts_with("THINKTHEN_") && !secret.iter().any(|part| upper.contains(part)),
            "a test child may not keep {name} from the parent: set a THINKTHEN_ value or a fake key explicitly"
        );
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    command
}
