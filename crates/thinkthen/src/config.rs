//! Read-only process configuration and independent platform paths.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::core::{Backend, DEFAULT_MODEL};

/// Why the configuration file was refused. The message names the file and a
/// field, never a value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConfigError {
    pub(crate) message: &'static str,
    /// The file exists but could not be read, which is a local failure.
    pub(crate) unreadable: bool,
}

const fn refused(message: &'static str) -> ConfigError {
    ConfigError {
        message,
        unreadable: false,
    }
}

pub(crate) const DEFAULT_CACHE_BYTES: u64 = 100_000_000;

const SCHEMA: &str = "configuration field `schema` must be `thinkthen.config/1`";
const CACHE_BYTES: &str =
    "configuration field `cache_bytes` must be a whole number greater than zero";

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(skip)]
    present: bool,
    schema: String,
    url: Option<String>,
    model: Option<String>,
    cache: Option<bool>,
    cache_bytes: Option<u64>,
}

impl Config {
    pub(crate) fn read(path: Option<&Path>) -> Result<Self, ConfigError> {
        let Some(path) = path else {
            return Ok(Self::default());
        };
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(_) => {
                return Err(ConfigError {
                    message: "the configuration file could not be read",
                    unreadable: true,
                });
            }
        };
        Self::parse(&bytes)
    }

    fn parse(bytes: &[u8]) -> Result<Self, ConfigError> {
        let mut parsed: Self =
            serde_json::from_slice(bytes).map_err(|_| refused(shape_fault(bytes)))?;
        parsed.present = true;
        parsed.validate()?;
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
    pub(crate) const fn present(&self) -> bool {
        self.present
    }
    pub(crate) const fn has_url(&self) -> bool {
        self.url.is_some()
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
}

/// Names the first field that breaks the closed shape, and never its value.
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
            ("schema", _) => SCHEMA,
            ("url", _) => "configuration field `url` must be a string",
            ("model", _) => "configuration field `model` must be a string",
            ("cache", _) => "configuration field `cache` must be true or false",
            ("cache_bytes", _) => CACHE_BYTES,
            _ => {
                "the configuration file holds a field other than `schema`, `url`, `model`, `cache`, and `cache_bytes`"
            }
        };
        return fault;
    }
    "the configuration file is not valid closed JSON"
}

#[derive(Clone, Copy)]
enum Platform {
    Linux,
    Macos,
}

pub(crate) fn path() -> Option<PathBuf> {
    resolve_config(
        current_platform(),
        variable("XDG_CONFIG_HOME"),
        variable("HOME"),
    )
}

pub(crate) fn cache_path() -> Option<PathBuf> {
    resolve_cache(
        current_platform(),
        variable("XDG_CACHE_HOME"),
        variable("HOME"),
    )
}

pub(crate) fn usage_path() -> Option<PathBuf> {
    resolve_usage(
        current_platform(),
        variable("XDG_CACHE_HOME"),
        variable("HOME"),
    )
}

fn resolve_config(
    platform: Platform,
    xdg: Option<String>,
    home: Option<String>,
) -> Option<PathBuf> {
    match platform {
        Platform::Macos => absolute(home)
            .map(|home| home.join("Library/Application Support/thinkthen/config.json")),
        Platform::Linux => absolute(xdg)
            .map(|home| home.join("thinkthen/config.json"))
            .or_else(|| absolute(home).map(|home| home.join(".config/thinkthen/config.json"))),
    }
}

fn resolve_cache(platform: Platform, xdg: Option<String>, home: Option<String>) -> Option<PathBuf> {
    match platform {
        Platform::Macos => absolute(home).map(|home| home.join("Library/Caches/thinkthen")),
        Platform::Linux => absolute(xdg)
            .map(|home| home.join("thinkthen"))
            .or_else(|| absolute(home).map(|home| home.join(".cache/thinkthen"))),
    }
}

fn resolve_usage(platform: Platform, xdg: Option<String>, home: Option<String>) -> Option<PathBuf> {
    match platform {
        Platform::Macos => absolute(home).map(|home| home.join("Library/Caches/thinkthen-usage")),
        Platform::Linux => absolute(xdg)
            .map(|home| home.join("thinkthen-usage"))
            .or_else(|| absolute(home).map(|home| home.join(".cache/thinkthen-usage"))),
    }
}

fn absolute(value: Option<String>) -> Option<PathBuf> {
    value.map(PathBuf::from).filter(|path| path.is_absolute())
}

const fn current_platform() -> Platform {
    if cfg!(target_os = "macos") {
        Platform::Macos
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
mod tests {
    use std::path::PathBuf;

    use super::{Config, Platform, resolve_cache, resolve_config, resolve_usage};

    #[test]
    fn each_refusal_names_its_field_and_never_its_value() {
        assert!(Config::parse(br#"{"schema":"thinkthen.config/1"}"#).is_ok());
        let not_closed = "the configuration file is not valid closed JSON";
        let schema = "configuration field `schema` must be `thinkthen.config/1`";
        let bytes = "configuration field `cache_bytes` must be a whole number greater than zero";
        for (text, sentence) in [
            (r#"{"schema":"thinkthen.config/1""#, not_closed),
            (r#"["schema"]"#, not_closed),
            (
                r#"{"schema":"thinkthen.config/1","cache":true,"cache":false}"#,
                not_closed,
            ),
            (r#"{}"#, schema),
            (r#"{"schema":"wrong"}"#, schema),
            (r#"{"schema":1}"#, schema),
            (
                r#"{"schema":"thinkthen.config/1","extra":true}"#,
                "the configuration file holds a field other than `schema`, `url`, `model`, `cache`, and `cache_bytes`",
            ),
            (
                r#"{"schema":"thinkthen.config/1","cache":"/folder"}"#,
                "configuration field `cache` must be true or false",
            ),
            (
                r#"{"schema":"thinkthen.config/1","url":7}"#,
                "configuration field `url` must be a string",
            ),
            (
                r#"{"schema":"thinkthen.config/1","model":["m"]}"#,
                "configuration field `model` must be a string",
            ),
            (r#"{"schema":"thinkthen.config/1","cache_bytes":0}"#, bytes),
            (r#"{"schema":"thinkthen.config/1","cache_bytes":-1}"#, bytes),
            (
                r#"{"schema":"thinkthen.config/1","model":"  "}"#,
                "configuration field `model` must not be blank",
            ),
            (
                r#"{"schema":"thinkthen.config/1","url":"http://example.com"}"#,
                "configuration field `url` must be a safe backend base",
            ),
        ] {
            let refused = Config::parse(text.as_bytes()).expect_err(text);
            assert_eq!(refused.message, sentence, "{text}");
            assert!(!refused.unreadable, "{text}");
        }
    }

    #[test]
    fn linux_and_macos_resolve_config_and_cache_independently() {
        let cases = [
            (
                Platform::Linux,
                Some("/config"),
                Some("/cache"),
                None,
                "/config/thinkthen/config.json",
                "/cache/thinkthen",
            ),
            (
                Platform::Linux,
                Some("relative"),
                Some("relative"),
                Some("/home/person"),
                "/home/person/.config/thinkthen/config.json",
                "/home/person/.cache/thinkthen",
            ),
            (
                Platform::Macos,
                Some("/ignored"),
                Some("/ignored"),
                Some("/Users/person"),
                "/Users/person/Library/Application Support/thinkthen/config.json",
                "/Users/person/Library/Caches/thinkthen",
            ),
        ];
        for (platform, config, cache, home, expected_config, expected_cache) in cases {
            assert_eq!(
                resolve_config(platform, config.map(str::to_owned), home.map(str::to_owned)),
                Some(PathBuf::from(expected_config))
            );
            assert_eq!(
                resolve_cache(platform, cache.map(str::to_owned), home.map(str::to_owned)),
                Some(PathBuf::from(expected_cache))
            );
        }
        assert_eq!(
            resolve_config(Platform::Linux, Some("/config".to_owned()), None),
            Some(PathBuf::from("/config/thinkthen/config.json"))
        );
        assert_eq!(resolve_cache(Platform::Linux, None, None), None);
        assert_eq!(
            resolve_usage(Platform::Linux, Some("/cache".to_owned()), None),
            Some(PathBuf::from("/cache/thinkthen-usage"))
        );
        assert_eq!(
            resolve_usage(Platform::Macos, None, Some("/Users/person".to_owned())),
            Some(PathBuf::from(
                "/Users/person/Library/Caches/thinkthen-usage"
            ))
        );
        for platform in [Platform::Linux, Platform::Macos] {
            for unusable in ["", "relative"] {
                assert_eq!(
                    resolve_config(
                        platform,
                        Some(unusable.to_owned()),
                        Some(unusable.to_owned())
                    ),
                    None
                );
                assert_eq!(
                    resolve_cache(
                        platform,
                        Some(unusable.to_owned()),
                        Some(unusable.to_owned())
                    ),
                    None
                );
            }
        }
    }
}
