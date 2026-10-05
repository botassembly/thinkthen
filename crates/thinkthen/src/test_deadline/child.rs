//! A test child's whole environment, built from nothing, and the names cargo reads.
#![allow(dead_code, reason = "each test file uses part of the helper")]
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(windows)]
#[path = "run.rs"]
mod run;
#[cfg(windows)]
#[path = "wait.rs"]
pub(crate) mod wait;

pub(crate) const CARGO: &[&str] = &[
    "HOME",
    "CARGO_HOME",
    "CARGO_BUILD_JOBS",
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

/// A default folder the command picks when no setting names one (ticket 0379).
///
/// Linux reads each folder from its own XDG variable, so a case's root holds
/// one subfolder per kind. Windows reads the configuration from `APPDATA` and
/// the other two from `LOCALAPPDATA`. macOS reads all three from `HOME`, so
/// there the root is the home and every folder moves with it.
#[derive(Clone, Copy)]
pub(crate) enum Folder {
    /// The folder that holds `config.json`.
    Config,
    /// The answer cache.
    Cache,
    /// The usage totals.
    Usage,
}

const MACOS: bool = cfg!(target_os = "macos");
const WINDOWS_HOST: bool = cfg!(windows);

impl Folder {
    /// The variable that puts this folder under `root`, and its value.
    pub(crate) fn variable(self, root: &Path) -> (&'static str, String) {
        let (name, value) = match self {
            _ if MACOS => ("HOME", root.to_owned()),
            Self::Config if WINDOWS_HOST => ("APPDATA", root.join("config")),
            _ if WINDOWS_HOST => ("LOCALAPPDATA", root.join("local")),
            Self::Config => ("XDG_CONFIG_HOME", root.join("config")),
            Self::Cache => ("XDG_CACHE_HOME", root.join("cache")),
            Self::Usage => ("XDG_STATE_HOME", root.join("state")),
        };
        (name, value.to_string_lossy().into_owned())
    }

    /// Write `text` as the configuration file under `root`, and return the
    /// variable that points the command at it.
    pub(crate) fn configure(root: &Path, text: &str) -> std::io::Result<(&'static str, String)> {
        let folder = Self::Config.under(root);
        std::fs::create_dir_all(&folder)?;
        let path = folder.join("config.json");
        std::fs::write(&path, text)?;
        private_file(&path)?;
        Ok(Self::Config.variable(root))
    }

    /// Where the command keeps this folder once `variable` names `root`.
    pub(crate) fn under(self, root: &Path) -> PathBuf {
        let mut folder = PathBuf::from(self.variable(root).1);
        folder.extend(self.below());
        folder
    }

    /// Where the command keeps this folder when only the child's home, as
    /// `ChildEnvironment::home` sets it, names it.
    pub(crate) fn in_home(self, home: &Path) -> PathBuf {
        let base: &[&str] = match self {
            _ if MACOS => &[],
            Self::Config if WINDOWS_HOST => &["AppData", "Roaming"],
            _ if WINDOWS_HOST => &["AppData", "Local"],
            Self::Config => &[".config"],
            Self::Cache => &[".cache"],
            Self::Usage => &[".local", "state"],
        };
        let mut folder = home.to_owned();
        folder.extend(base.iter().chain(self.below()));
        folder
    }

    /// The path from the folder its variable names down to this folder.
    fn below(self) -> &'static [&'static str] {
        match self {
            Self::Config if MACOS => &["Library", "Application Support", "thinkthen"],
            Self::Cache if MACOS => &["Library", "Caches", "thinkthen"],
            Self::Usage if MACOS => &["Library", "Application Support", "thinkthen", "usage"],
            Self::Cache if WINDOWS_HOST => &["thinkthen", "cache"],
            Self::Usage if WINDOWS_HOST => &["thinkthen", "usage"],
            _ => &["thinkthen"],
        }
    }
}

/// Restrict an owned fixture file before asking the product to trust it.
pub(crate) fn private_file(path: &Path) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        let mut command = command("powershell.exe", &[]);
        command.args(["-NoProfile", "-NonInteractive", "-Command", "$ErrorActionPreference='Stop'; $sid=[System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value; $acl=Get-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH; $acl.SetSecurityDescriptorSddlForm(\"O:${sid}D:P(A;;FA;;;${sid})(A;;FA;;;SY)\"); Set-Acl -LiteralPath $env:THINKTHEN_FIXTURE_PATH -AclObject $acl"])
            .env("THINKTHEN_FIXTURE_PATH", path);
        let output = run::output(&mut command)?;
        if !output.status.success() {
            return Err(std::io::Error::other(format!(
                "private fixture ACL: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
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
