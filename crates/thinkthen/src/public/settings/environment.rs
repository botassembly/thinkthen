//! Environment capture for the public engine builder.

use std::path::PathBuf;

use std::collections::BTreeMap;

use super::{EngineBuilder, Secret, variable};
use crate::config::{self, Config};
use crate::core::{Backend, DEFAULT_MODEL, KEY_VAR, ModelName, named};
use crate::public::error::Error;

impl EngineBuilder {
    /// Whether another user may change the configuration captured by this builder.
    /// This observation reads nothing and does not resolve a backend or credential.
    #[must_use]
    pub fn configuration_shared(&self) -> bool {
        self.captured.metadata.shared
    }

    /// Capture what the command reads: `THINKTHEN_BASE_URL`,
    /// `THINKTHEN_BACKEND`, `THINKTHEN_API_KEY`, each key variable a built-in
    /// backend or a configuration entry names, `THINKTHEN_CACHE`, `THINKTHEN_CA_BUNDLE`,
    /// `THINKTHEN_BATCH`, `THINKTHEN_MAX_REQUEST_BYTES`,
    /// `THINKTHEN_REQUESTS_PER_MINUTE`,
    /// `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL`, the XDG cache and state
    /// homes, and the XDG configuration file. The setters and `build` read no
    /// environment.
    ///
    /// The engine adds its requests, retries, live tokens and cache answers to
    /// the command's count-only usage totals, which `thinkthen status` reads.
    /// When the usage folder exists and cannot be read, the engine's first send
    /// returns [`Error::Local`] naming the file and the fix, and sends nothing.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming a malformed variable or configuration
    /// field, and [`Error::Local`] when the configuration file exists but
    /// cannot be read.
    pub fn from_env() -> Result<Self, Error> {
        let config_path = config::path();
        let config = Config::read(config_path.as_deref()).map_err(|refused| {
            if refused.unreadable {
                Error::local(refused.message.into_owned())
            } else {
                Error::usage(refused.message.into_owned())
            }
        })?;
        let named = variable("THINKTHEN_CACHE")?.map(PathBuf::from);
        let platform = named.is_none();
        let folder = named.or_else(config::cache_path);
        let mut builder = Self::configured(&config, config_path, folder, platform);
        let base_url = variable("THINKTHEN_BASE_URL")?;
        if let Some(base) = &base_url {
            Backend::resolve(Some(base), None, DEFAULT_MODEL)
                .map_err(|error| Error::usage(format!("THINKTHEN_BASE_URL: {error}")))?;
        }
        builder.captured.base_url = base_url;
        builder.captured.backend = variable("THINKTHEN_BACKEND")?;
        builder.captured.keys = keys(&config)?;
        // A later explicit setter outranks this path, including an invalid one.
        // Validate only the path selected when the engine is built.
        builder.ca_bundle = variable("THINKTHEN_CA_BUNDLE")?.map(PathBuf::from);
        builder.env_batch = variable("THINKTHEN_BATCH")?;
        builder.per_minute = crate::engine::backoff::per_minute(
            variable("THINKTHEN_REQUESTS_PER_MINUTE")?.as_deref(),
        )
        .map_err(Error::usage)?;
        builder.max_estimated_input_tokens_total =
            crate::engine::estimated_total(variable("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL")?)
                .map_err(Error::usage)?;
        if let Some(model) = config.model() {
            ModelName::new(model).map_err(Error::refused)?;
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

/// Capture `THINKTHEN_API_KEY` and each nonblank variable a built-in or a
/// configuration entry names. `THINKTHEN_API_KEY` keeps today's UTF-8 refusal.
/// Another variable that is not UTF-8 counts as unset, as the command reads it.
fn keys(config: &Config) -> Result<BTreeMap<String, Secret>, Error> {
    let mut keys = BTreeMap::new();
    if let Some(key) = variable(KEY_VAR)? {
        keys.insert(KEY_VAR.to_owned(), Secret(key.into()));
    }
    let named = named::built_in_keys().map(str::to_owned).chain(
        config
            .named()
            .iter()
            .flat_map(|entry| entry.keys().to_vec()),
    );
    for name in named {
        if keys.contains_key(&name) {
            continue;
        }
        if let Ok(Some(key)) = variable(&name) {
            keys.insert(name, Secret(key.into()));
        }
    }
    Ok(keys)
}
