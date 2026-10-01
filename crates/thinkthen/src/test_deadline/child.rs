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

/// A default folder the command picks when no setting names one.
///
/// Linux reads each from its own XDG variable, so a case's root holds one
/// subfolder per kind. macOS reads all three from `HOME`, so there the root is
/// the home (ticket 0379). Windows reads `APPDATA` and `LOCALAPPDATA`; its cases
/// wait on finding W6 in `sdlc/planning/windows.md`.
#[cfg(unix)]
#[derive(Clone, Copy)]
pub(crate) enum Folder {
    /// The folder that holds `config.json`.
    Config,
    /// The answer cache.
    Cache,
    /// The usage totals.
    Usage,
}

#[cfg(unix)]
impl Folder {
    /// The variable that puts this folder under `root`, and its value.
    pub(crate) fn variable(self, root: &Path) -> (&'static str, String) {
        let (name, value) = if cfg!(target_os = "macos") {
            ("HOME", root.to_owned())
        } else {
            match self {
                Self::Config => ("XDG_CONFIG_HOME", root.join("config")),
                Self::Cache => ("XDG_CACHE_HOME", root.join("cache")),
                Self::Usage => ("XDG_STATE_HOME", root.join("state")),
            }
        };
        (name, value.to_str().expect("a UTF-8 test folder").to_owned())
    }

    /// Where the command keeps this folder once `variable` names `root`.
    pub(crate) fn under(self, root: &Path) -> std::path::PathBuf {
        let mac = cfg!(target_os = "macos");
        match self {
            Self::Config if mac => root.join("Library/Application Support/thinkthen"),
            Self::Cache if mac => root.join("Library/Caches/thinkthen"),
            Self::Usage if mac => root.join("Library/Application Support/thinkthen/usage"),
            Self::Config => root.join("config/thinkthen"),
            Self::Cache => root.join("cache/thinkthen"),
            Self::Usage => root.join("state/thinkthen"),
        }
    }
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
