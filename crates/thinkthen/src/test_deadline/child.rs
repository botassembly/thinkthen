//! A test child's whole environment, built from nothing, and the names cargo reads.
#![allow(dead_code, reason = "each test file uses part of the helper")]
use std::path::Path;
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
    command.clear_environment();
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

/// What Windows needs in a child's environment to start it and to open a
/// socket: without `SystemRoot` the socket library cannot load (ticket 0373).
#[cfg(windows)]
const WINDOWS: [&str; 4] = ["SystemRoot", "SystemDrive", "TEMP", "TMP"];

/// A test child's environment, the same on every platform that runs the tests.
pub(crate) trait ChildEnvironment {
    /// Clear the environment. Windows keeps only what it needs to run;
    /// Linux and macOS keep nothing.
    fn clear_environment(&mut self) -> &mut Self;
    /// Name the child's home. Windows reads its default folders from
    /// `APPDATA` and `LOCALAPPDATA`, so they go under the home there.
    fn home(&mut self, home: impl AsRef<Path>) -> &mut Self;
}

impl ChildEnvironment for Command {
    fn clear_environment(&mut self) -> &mut Self {
        self.env_clear();
        #[cfg(windows)]
        for name in WINDOWS {
            if let Some(value) = std::env::var_os(name) {
                self.env(name, value);
            }
        }
        self
    }

    fn home(&mut self, home: impl AsRef<Path>) -> &mut Self {
        let home = home.as_ref();
        #[cfg(windows)]
        self.env("APPDATA", home.join("AppData").join("Roaming"))
            .env("LOCALAPPDATA", home.join("AppData").join("Local"));
        self.env("HOME", home)
    }
}
