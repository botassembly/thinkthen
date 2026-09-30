//! Environment capture for the public engine builder.

use std::path::PathBuf;

use super::{EngineBuilder, Secret, Seeded, variable};
use crate::config::{self, Config};
use crate::core::KEY_VAR;
use crate::public::error::Error;

impl EngineBuilder {
    /// Capture what the command reads: `THINKTHEN_BASE_URL`,
    /// `THINKTHEN_API_KEY`, `THINKTHEN_CACHE`, `THINKTHEN_CA_BUNDLE`,
    /// `THINKTHEN_BATCH`, `THINKTHEN_MAX_REQUEST_BYTES`,
    /// `THINKTHEN_REQUESTS_PER_MINUTE`, the XDG cache home, and the
    /// XDG configuration file. The setters and `build` read no environment.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming a malformed variable or configuration
    /// field, and [`Error::Local`] when the configuration file exists but
    /// cannot be read.
    pub fn from_env() -> Result<Self, Error> {
        let config = Config::read(config::path().as_deref()).map_err(|refused| {
            if refused.unreadable {
                Error::local(refused.message)
            } else {
                Error::usage(refused.message)
            }
        })?;
        let named = variable("THINKTHEN_CACHE")?.map(PathBuf::from);
        let mut builder = Self::new();
        builder.prices = config.prices();
        builder.seeded = Some(Seeded {
            platform: named.is_none(),
            folder: named.or_else(config::cache_path),
            enabled: config.cache_enabled(),
        });
        if let Some(base) =
            variable("THINKTHEN_BASE_URL")?.or_else(|| config.url().map(str::to_owned))
        {
            builder = builder
                .base_url(&base)
                .map_err(|error| Error::usage(format!("THINKTHEN_BASE_URL: {error}")))?;
        }
        if let Some(key) = variable(KEY_VAR)? {
            builder.key = Some(Secret(key.into()));
        }
        // A later explicit setter outranks this path, including an invalid one.
        // Validate only the path selected when the engine is built.
        builder.ca_bundle = variable("THINKTHEN_CA_BUNDLE")?.map(PathBuf::from);
        builder.env_batch = variable("THINKTHEN_BATCH")?;
        builder.per_minute = crate::engine::backoff::per_minute(
            variable("THINKTHEN_REQUESTS_PER_MINUTE")?.as_deref(),
        )
        .map_err(Error::usage)?;
        if let Some(model) = config.model() {
            builder = builder.model(model)?;
        }
        if let Some(size) = variable("THINKTHEN_MAX_REQUEST_BYTES")? {
            let value = size
                .parse::<usize>()
                .ok()
                .filter(|value| *value > 0 && size.bytes().all(|byte| byte.is_ascii_digit()))
                .ok_or_else(|| {
                    Error::usage("THINKTHEN_MAX_REQUEST_BYTES takes a whole number of at least 1")
                })?;
            builder = builder.max_request_bytes(value)?;
        }
        Ok(builder)
    }
}
