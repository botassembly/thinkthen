//! A test child's whole environment, built from nothing (ticket 0127).
//!
//! The child gets `PATH` and the parent's value of each name the caller
//! keeps. A secret-shaped or `THINKTHEN_` name is never kept: a test sets a
//! `THINKTHEN_` value or a fake key explicitly with `.env`.
use std::ffi::OsStr;
use std::process::Command;

/// Name parts that mark a secret, matched without regard to case.
const SECRET: [&str; 6] = ["KEY", "TOKEN", "SECRET", "PASSWORD", "CREDENTIAL", "AUTH"];

/// A `Command` for `program` whose environment holds `PATH` and `keep` only.
pub(crate) fn command(program: impl AsRef<OsStr>, keep: &[&str]) -> Command {
    let mut command = Command::new(program);
    command.env_clear();
    for name in std::iter::once(&"PATH").chain(keep) {
        let upper = name.to_uppercase();
        assert!(
            *name == "PATH"
                || !(upper.starts_with("THINKTHEN_")
                    || SECRET.iter().any(|part| upper.contains(part))),
            "a test child may not keep {name} from the parent: set a THINKTHEN_ value or a fake key explicitly"
        );
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
    command
}
