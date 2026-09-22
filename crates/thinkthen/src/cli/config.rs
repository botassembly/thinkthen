//! Read-only process configuration and independent platform paths.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::core::{Backend, DEFAULT_MODEL};
use crate::failure::Failure;

pub(crate) const DEFAULT_CACHE_BYTES: u64 = 100_000_000;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    schema: String,
    url: Option<String>,
    model: Option<String>,
    cache: Option<bool>,
    cache_bytes: Option<u64>,
}

impl Config {
    pub(crate) fn read(path: Option<&Path>) -> Result<Self, Failure> {
        let Some(path) = path else {
            return Ok(Self::default());
        };
        let bytes = match fs::read(path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(_) => {
                return Err(Failure::Configuration(
                    "the configuration file could not be read",
                ));
            }
        };
        let parsed: Self = serde_json::from_slice(&bytes).map_err(|_| {
            Failure::Configuration("the configuration file is not valid closed JSON")
        })?;
        parsed.validate()?;
        Ok(parsed)
    }

    fn validate(&self) -> Result<(), Failure> {
        if self.schema != "thinkthen.config/1" {
            return Err(Failure::Configuration(
                "configuration field `schema` must be `thinkthen.config/1`",
            ));
        }
        if self
            .model
            .as_ref()
            .is_some_and(|model| model.trim().is_empty())
        {
            return Err(Failure::Configuration(
                "configuration field `model` must not be blank",
            ));
        }
        if self.cache_bytes == Some(0) {
            return Err(Failure::Configuration(
                "configuration field `cache_bytes` must be greater than zero",
            ));
        }
        if let Some(url) = self.url.as_deref()
            && Backend::resolve(Some(url), None, DEFAULT_MODEL).is_err()
        {
            return Err(Failure::Configuration(
                "configuration field `url` must be a safe backend base",
            ));
        }
        Ok(())
    }

    pub(crate) fn url(&self) -> Option<&str> {
        self.url.as_deref()
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

    use super::{Config, Platform, resolve_cache, resolve_config};

    #[test]
    fn the_closed_shape_requires_its_schema_and_positive_limit() {
        let valid: Config = serde_json::from_str(r#"{"schema":"thinkthen.config/1"}"#)
            .expect("closed config parses");
        assert!(valid.validate().is_ok());
        for text in [
            r#"{}"#,
            r#"{"schema":"wrong"}"#,
            r#"{"schema":"thinkthen.config/1","extra":true}"#,
            r#"{"schema":"thinkthen.config/1","cache_bytes":0}"#,
            r#"{"schema":"thinkthen.config/1","model":"  "}"#,
            r#"{"schema":"thinkthen.config/1","url":"http://example.com"}"#,
            r#"{"schema":"thinkthen.config/1","cache":"yes"}"#,
            r#"{"schema":"thinkthen.config/1","cache_bytes":-1}"#,
            r#"{"schema":"thinkthen.config/1""#,
        ] {
            let refused = serde_json::from_str::<Config>(text)
                .map_err(|_| ())
                .and_then(|config| config.validate().map_err(|_| ()));
            assert!(refused.is_err(), "{text}");
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
