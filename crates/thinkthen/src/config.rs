//! Read-only process configuration and independent platform paths.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::core::{Backend, DEFAULT_MODEL, Named, Prices};

mod backends;

/// Why the configuration file was refused. The message names the file and a
/// field, never a value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConfigError {
    pub(crate) message: std::borrow::Cow<'static, str>,
    /// The file exists but could not be read, which is a local failure.
    pub(crate) unreadable: bool,
    pub(crate) price: Option<&'static str>,
}

const fn refused(message: &'static str) -> ConfigError {
    ConfigError {
        message: std::borrow::Cow::Borrowed(message),
        unreadable: false,
        price: None,
    }
}

const PRICE_VALUE: &str = "configuration prices are two decimal strings from 0 through 1000000 with at most six fractional digits";
const PRICE_PAIR: &str = "configuration prices require both input and output fields";

const fn refused_price(message: &'static str) -> ConfigError {
    ConfigError {
        message: std::borrow::Cow::Borrowed(message),
        unreadable: false,
        price: Some(message),
    }
}

pub(crate) const DEFAULT_CACHE_BYTES: u64 = 100_000_000;

#[cfg(not(windows))]
const SHARED_BACKENDS: &str = "the configuration file is writable by another user, so its `backends` are refused; keep it writable by its owner alone";
#[cfg(windows)]
const SHARED_BACKENDS: &str = "another user owns the configuration file or its Windows access permissions allow another user to change it, so its `backends` are refused; keep it owned by your user and writable only by your user and Windows SYSTEM";
const UNKNOWN: &str = "the configuration file holds a field other than `schema`, `url`, `model`, `cache`, `cache_bytes`, `usd_per_million_input`, `usd_per_million_output`, `backend`, and `backends`";
const SCHEMA: &str = "configuration field `schema` must be `thinkthen.config/1`";
const CACHE_BYTES: &str =
    "configuration field `cache_bytes` must be a whole number greater than zero";

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(skip)]
    present: bool,
    #[serde(skip)]
    shared: bool,
    schema: String,
    url: Option<String>,
    model: Option<String>,
    cache: Option<bool>,
    cache_bytes: Option<u64>,
    usd_per_million_input: Option<String>,
    usd_per_million_output: Option<String>,
    backend: Option<String>,
    backends: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(skip)]
    prices: Option<Prices>,
    #[serde(skip)]
    named: Vec<Named>,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Config")
            .field("present", &self.present)
            .field("shared", &self.shared)
            .field("schema", &self.schema)
            .field("url", &self.url.as_ref().map(|_| "<withheld>"))
            .field("model", &self.model)
            .field("cache", &self.cache)
            .field("cache_bytes", &self.cache_bytes)
            .field("backend", &self.backend)
            .field("backends", &self.named)
            .finish()
    }
}

impl Config {
    pub(crate) fn read(path: Option<&Path>) -> Result<Self, ConfigError> {
        let Some(path) = path else {
            return Ok(Self::default());
        };
        #[cfg(not(windows))]
        let read = fs::read(path).map(|bytes| (bytes, ()));
        #[cfg(windows)]
        let read = crate::windows::files::configuration(path);
        let (bytes, _shared) = match read {
            Ok(read) => read,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(_) => {
                return Err(ConfigError {
                    message: "the configuration file could not be read".into(),
                    unreadable: true,
                    price: None,
                });
            }
        };
        let mut parsed = Self::parse(&bytes)?;
        #[cfg(not(windows))]
        let shared = fs::metadata(path).is_ok_and(|metadata| writable_by_another(&metadata));
        #[cfg(windows)]
        let shared = _shared;
        parsed.shared = shared;
        if parsed.shared && !parsed.named.is_empty() {
            // Another user could name any variable here, and so send any secret in the environment.
            return Err(refused(SHARED_BACKENDS));
        }
        Ok(parsed)
    }

    fn parse(bytes: &[u8]) -> Result<Self, ConfigError> {
        let mut parsed: Self = serde_json::from_slice(bytes).map_err(|_| {
            let fault = shape_fault(bytes);
            if fault == PRICE_VALUE {
                refused_price(fault)
            } else {
                refused(fault)
            }
        })?;
        parsed.present = true;
        parsed.validate()?;
        parsed.prices = price_pair(bytes, &parsed)?;
        parsed.named = backends::read(parsed.backend.as_deref(), parsed.backends.as_ref())?;
        Ok(parsed)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.schema != "thinkthen.config/1" {
            return Err(refused(SCHEMA));
        }
        if self
            .model
            .as_ref()
            .is_some_and(|model| model.trim().is_empty())
        {
            return Err(refused("configuration field `model` must not be blank"));
        }
        if self.cache_bytes == Some(0) {
            return Err(refused(CACHE_BYTES));
        }
        if let Some(url) = self.url.as_deref()
            && Backend::resolve(Some(url), None, DEFAULT_MODEL).is_err()
        {
            return Err(refused(
                "configuration field `url` must be a safe backend base",
            ));
        }
        Ok(())
    }

    pub(crate) fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    /// The backend the file names, which `backends::read` checked.
    pub(crate) fn backend(&self) -> Option<&str> {
        self.backend.as_deref()
    }
    /// The backends the file names beside the built-ins.
    pub(crate) fn named(&self) -> &[Named] {
        &self.named
    }
    pub(crate) const fn present(&self) -> bool {
        self.present
    }
    /// Whether another user can change the file, which decides where the key
    /// and the evidence go.
    pub(crate) const fn shared(&self) -> bool {
        self.shared
    }
    pub(crate) const fn has_model(&self) -> bool {
        self.model.is_some()
    }
    pub(crate) const fn has_cache(&self) -> bool {
        self.cache.is_some()
    }
    pub(crate) const fn has_cache_bytes(&self) -> bool {
        self.cache_bytes.is_some()
    }
    pub(crate) fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }
    pub(crate) fn cache_enabled(&self) -> bool {
        self.cache.unwrap_or(true)
    }
    pub(crate) fn cache_bytes(&self) -> u64 {
        self.cache_bytes.unwrap_or(DEFAULT_CACHE_BYTES)
    }
    pub(crate) const fn prices(&self) -> Option<Prices> {
        self.prices
    }
}

fn price_pair(bytes: &[u8], config: &Config) -> Result<Option<Prices>, ConfigError> {
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|_| refused("the configuration file is not valid closed JSON"))?;
    let Some(fields) = value.as_object() else {
        return Err(refused("the configuration file is not valid closed JSON"));
    };
    match (
        fields.get("usd_per_million_input"),
        fields.get("usd_per_million_output"),
    ) {
        (None, None) => Ok(None),
        (Some(input), Some(output)) => {
            if !input.is_string() || !output.is_string() {
                return Err(refused_price(PRICE_VALUE));
            }
            let pair = config
                .usd_per_million_input
                .as_deref()
                .zip(config.usd_per_million_output.as_deref())
                .and_then(|(input, output)| Prices::parse(input, output));
            pair.map(Some).ok_or_else(|| refused_price(PRICE_VALUE))
        }
        _ => Err(refused_price(PRICE_PAIR)),
    }
}

/// Names the first field in name order that breaks the closed shape, and never its value.
fn shape_fault(bytes: &[u8]) -> &'static str {
    use serde_json::Value;
    let Ok(Value::Object(fields)) = serde_json::from_slice::<Value>(bytes) else {
        return "the configuration file is not valid closed JSON";
    };
    if !fields.contains_key("schema") {
        return SCHEMA;
    }
    for (name, value) in &fields {
        let fault = match (name.as_str(), value) {
            ("schema", Value::String(_))
            | ("url" | "model", Value::String(_) | Value::Null)
            | ("cache", Value::Bool(_) | Value::Null) => continue,
            ("cache_bytes", value) if value.is_null() || value.is_u64() => continue,
            ("usd_per_million_input" | "usd_per_million_output", Value::String(_))
            | ("backend", Value::String(_) | Value::Null)
            | ("backends", Value::Object(_) | Value::Null) => continue,
            ("schema", _) => SCHEMA,
            ("url", _) => "configuration field `url` must be a string",
            ("model", _) => "configuration field `model` must be a string",
            ("cache", _) => "configuration field `cache` must be true or false",
            ("cache_bytes", _) => CACHE_BYTES,
            ("usd_per_million_input" | "usd_per_million_output", _) => PRICE_VALUE,
            ("backend", _) => "configuration field `backend` must be a string",
            ("backends", _) => "configuration field `backends` must be an object",
            _ => UNKNOWN,
        };
        return fault;
    }
    "the configuration file is not valid closed JSON"
}

/// Whether another user owns or may write this file or folder. A group bit
/// alone stays quiet, because the usual 002 umask sets it on everything a
/// user saves. The configuration file and a named cache folder share this rule.
#[cfg(unix)]
pub(crate) fn writable_by_another(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::{MetadataExt as _, PermissionsExt as _};
    metadata_writable_by_another(
        metadata.permissions().mode(),
        metadata.uid(),
        nix::unistd::geteuid().as_raw(),
    )
}

#[cfg(unix)]
fn metadata_writable_by_another(mode: u32, owner: u32, effective: u32) -> bool {
    owner != effective || mode & 0o002 != 0
}

#[cfg(not(unix))]
pub(crate) const fn writable_by_another(_metadata: &fs::Metadata) -> bool {
    false
}

#[derive(Clone, Copy)]
enum Platform {
    Linux,
    Macos,
    Windows,
}

impl Platform {
    /// The variables that hold the configuration, cache and usage bases. On
    /// Windows, ADR 0017 and tickets 0062 and 0360 chose `%APPDATA%` and
    /// `%LOCALAPPDATA%` (ticket 0373).
    const fn bases(self) -> [&'static str; 3] {
        match self {
            Self::Linux | Self::Macos => ["XDG_CONFIG_HOME", "XDG_CACHE_HOME", "XDG_STATE_HOME"],
            Self::Windows => ["APPDATA", "LOCALAPPDATA", "LOCALAPPDATA"],
        }
    }
}

pub(crate) fn path() -> Option<PathBuf> {
    let platform = current_platform();
    resolve_config(platform, variable(platform.bases()[0]), variable("HOME"))
}

pub(crate) fn cache_path() -> Option<PathBuf> {
    let platform = current_platform();
    resolve_cache(platform, variable(platform.bases()[1]), variable("HOME"))
}

/// The usage totals are state. Clearing the cache keeps them (ticket 0360).
pub(crate) fn usage_path() -> Option<PathBuf> {
    let platform = current_platform();
    resolve_usage(platform, variable(platform.bases()[2]), variable("HOME"))
}

fn resolve_config(
    platform: Platform,
    base: Option<String>,
    home: Option<String>,
) -> Option<PathBuf> {
    match platform {
        Platform::Macos => absolute(home)
            .map(|home| home.join("Library/Application Support/thinkthen/config.json")),
        Platform::Linux => absolute(base)
            .map(|home| home.join("thinkthen/config.json"))
            .or_else(|| absolute(home).map(|home| home.join(".config/thinkthen/config.json"))),
        Platform::Windows => absolute(base).map(|base| base.join("thinkthen").join("config.json")),
    }
}

fn resolve_cache(
    platform: Platform,
    base: Option<String>,
    home: Option<String>,
) -> Option<PathBuf> {
    match platform {
        Platform::Macos => absolute(home).map(|home| home.join("Library/Caches/thinkthen")),
        Platform::Linux => absolute(base)
            .map(|home| home.join("thinkthen"))
            .or_else(|| absolute(home).map(|home| home.join(".cache/thinkthen"))),
        Platform::Windows => absolute(base).map(|base| base.join("thinkthen").join("cache")),
    }
}

fn resolve_usage(
    platform: Platform,
    base: Option<String>,
    home: Option<String>,
) -> Option<PathBuf> {
    match platform {
        // macOS has no state folder. The configuration file shares
        // `Application Support/thinkthen`, so usage takes a private folder below it.
        Platform::Macos => {
            absolute(home).map(|home| home.join("Library/Application Support/thinkthen/usage"))
        }
        Platform::Linux => absolute(base)
            .map(|home| home.join("thinkthen"))
            .or_else(|| absolute(home).map(|home| home.join(".local/state/thinkthen"))),
        // The cache shares `%LOCALAPPDATA%\thinkthen`, so usage takes its own folder.
        Platform::Windows => absolute(base).map(|base| base.join("thinkthen").join("usage")),
    }
}

fn absolute(value: Option<String>) -> Option<PathBuf> {
    value.map(PathBuf::from).filter(|path| path.is_absolute())
}

const fn current_platform() -> Platform {
    if cfg!(target_os = "macos") {
        Platform::Macos
    } else if cfg!(windows) {
        Platform::Windows
    } else {
        Platform::Linux
    }
}

fn variable(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "config/backend_setups_tests.rs"]
mod backend_setups;
