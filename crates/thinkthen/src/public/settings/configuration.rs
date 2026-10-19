//! Capture platform configuration without admitting unrelated request settings.
use super::{EngineBuilder, Seeded, backend::Captured, snapshot::CapturedMetadata};
use crate::{Error, config::{self, Config}};
use std::path::PathBuf;

impl EngineBuilder {
    /// Read the platform configuration and capture its default cache and usage paths.
    /// This reads no request-setting or credential environment variables. A host
    /// can select its effective route through existing setters before capturing
    /// only the credential variables named by [`Self::resolved_settings`].
    /// Route and address guards run during selection; the final key-in-address
    /// guard runs after an explicit key is supplied, during selection or build.
    ///
    /// Unlike [`Self::from_env`], this does not admit unrelated environment
    /// controls. The host owns their capture, relevance and override precedence.
    /// # Errors
    /// Returns the existing safe local configuration refusal, or Usage for an
    /// invalid configured price pair. Configuration is never modified.
    pub fn from_config() -> Result<Self, Error> {
        let path = config::path();
        let config = Config::read(path.as_deref()).map_err(|error| {
            let public = if let Some(message) = error.price {
                Error::usage(message)
            } else {
                Error::local(error.message.to_string())
            };
            public.with_diagnostic(crate::public::error::diagnostic::Diagnostic::Configuration(error))
        })?;
        Ok(Self::configured(&config, path, config::cache_path(), true))
    }

    pub(super) fn configured(
        config: &Config,
        path: Option<PathBuf>,
        folder: Option<PathBuf>,
        platform: bool,
    ) -> Self {
        let mut builder = Self::new();
        builder.seeded = Some(Seeded {
            platform,
            folder,
            enabled: config.cache_enabled(),
            usage: config::usage_path(),
        });
        builder.captured = Captured {
            config_url: config.url().map(str::to_owned),
            config_backend: config.backend().map(str::to_owned),
            config_model: config.model().map(str::to_owned),
            prices: config.prices(),
            configured: config.named().to_vec(),
            metadata: CapturedMetadata {
                path,
                present: config.present(),
                shared: config.shared(),
                cache_configured: config.has_cache(),
                cache_bytes: Some(config.cache_bytes()),
                cache_bytes_configured: config.has_cache_bytes(),
            },
            ..Captured::default()
        };
        builder
    }
}
