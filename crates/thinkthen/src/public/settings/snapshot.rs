//! Explicit nonsecret views of settings already captured by the builder.
use super::{Cache, EngineBuilder, Secret};
use crate::{Error, config};
use std::fmt;
use std::path::PathBuf;

#[derive(Default)]
pub(super) struct CapturedMetadata {
    pub(super) path: Option<PathBuf>,
    pub(super) present: bool,
    pub(super) shared: bool,
    pub(super) cache_configured: bool,
    pub(super) cache_bytes: Option<u64>,
    pub(super) cache_bytes_configured: bool,
}

/// Route-independent cache settings for explicit maintenance and status commands.
/// This view contains no backend, credential or answer. Debug withholds paths.
#[derive(Clone)]
pub struct CacheSettings {
    /// Whether the captured configuration may be changed by another user.
    pub configuration_shared: bool,
    /// Whether answer caching is enabled for this builder.
    pub cache_enabled: bool,
    /// The existing tier selecting enablement.
    pub cache_enabled_source: &'static str,
    /// The captured cache path, including a disabled platform default.
    pub cache_path: Option<PathBuf>,
    /// The existing path tier: platform, environment or command_line.
    pub cache_path_source: &'static str,
    /// Whether the path is the private platform default.
    pub cache_is_platform_default: bool,
    /// The captured allocated-byte target for cache pruning.
    pub cache_target_bytes: u64,
    /// The existing pruning target tier.
    pub cache_target_source: &'static str,
}
impl fmt::Debug for CacheSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("CacheSettings")
            .field("configuration_shared", &self.configuration_shared)
            .field("cache_enabled", &self.cache_enabled)
            .field("cache_target_bytes", &self.cache_target_bytes)
            .finish_non_exhaustive()
    }
}

/// Resolved settings for an explicit caller-owned status or plan presentation.
/// No credential value is exposed. The builder retains every captured key.
/// Debug withholds address, model and local paths as the builder does.
#[derive(Clone)]
pub struct ResolvedSettings {
    /// The captured configuration path, when the platform supplied one.
    pub configuration_path: Option<PathBuf>,
    /// Whether that configuration file existed at capture.
    pub configuration_present: bool,
    /// Whether another user may change the captured configuration.
    pub configuration_shared: bool,
    /// The selected named backend, if any.
    pub backend_name: Option<String>,
    /// The validated posting address selected for this builder.
    pub url: String,
    /// The tier selecting the address: built_in, backend, environment,
    /// configuration or command_line.
    pub url_source: &'static str,
    /// The selected model name.
    pub model: String,
    /// The tier selecting the model, using the same names as `url_source`.
    pub model_source: &'static str,
    /// The selected backend's first credential variable name.
    pub key_variable: String,
    /// Approved credential variable names in their native fallback order.
    /// The caller may capture only this selected route's variables after selection
    /// and host guards. This list contains names, never credential values.
    pub key_variables: Vec<String>,
    /// Whether a usable credential was captured, without exposing it.
    pub api_key_set: bool,
    /// Whether the builder selected answer caching.
    pub cache_enabled: bool,
    /// The tier selecting cache enablement.
    pub cache_enabled_source: &'static str,
    /// The captured cache path, including a disabled default cache's path.
    /// A bare builder has no captured platform path.
    pub cache_path: Option<PathBuf>,
    /// The cache path's source: platform, environment or command_line.
    pub cache_path_source: &'static str,
    /// Whether the cache path is the private platform default.
    pub cache_is_platform_default: bool,
    /// The captured target allocated bytes used by cache pruning.
    pub cache_target_bytes: u64,
    /// The tier selecting the pruning target.
    pub cache_target_source: &'static str,
    /// The captured count-only usage directory, if any.
    pub usage_path: Option<PathBuf>,
}

impl fmt::Debug for ResolvedSettings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ResolvedSettings")
            .field("configuration_present", &self.configuration_present)
            .field("configuration_shared", &self.configuration_shared)
            .field("api_key_set", &self.api_key_set)
            .field("cache_enabled", &self.cache_enabled)
            .finish_non_exhaustive()
    }
}

impl EngineBuilder {
    /// Inspect captured cache settings without selecting a backend or address.
    /// Maintenance commands use this after [`Self::from_config`] so irrelevant
    /// route settings cannot refuse an explicit cache operation. This reads no
    /// environment, file, credential or answer and sends nothing. The caller
    /// owns environment cache overrides, command precedence and warning timing.
    pub fn cache_settings(&self) -> CacheSettings {
        let metadata = &self.captured.metadata;
        let platform = self.seeded.as_ref().is_none_or(|seeded| seeded.platform);
        let cache_enabled = match &self.cache {
            Cache::Off => false,
            Cache::At(_) => true,
            Cache::Default => self.seeded.as_ref().is_none_or(|seeded| {
                !seeded.platform || (seeded.enabled && !self.server)
            }) && (self.seeded.is_some() || !self.server),
        };
        let cache_enabled_source = match self.cache {
            Cache::Off | Cache::At(_) => "command_line",
            Cache::Default if !platform => "environment",
            Cache::Default if metadata.cache_configured => "configuration",
            Cache::Default => "built_in",
        };
        let (cache_path, cache_path_source, cache_is_platform_default) = match &self.cache {
            Cache::At(path) => (Some(path.clone()), "command_line", false),
            _ => (
                self.seeded.as_ref().and_then(|seeded| seeded.folder.clone()),
                if platform { "platform" } else { "environment" },
                platform,
            ),
        };
        CacheSettings {
            configuration_shared: metadata.shared,
            cache_enabled,
            cache_enabled_source,
            cache_path,
            cache_path_source,
            cache_is_platform_default,
            cache_target_bytes: metadata.cache_bytes.unwrap_or(config::DEFAULT_CACHE_BYTES),
            cache_target_source: if metadata.cache_bytes_configured {
                "configuration"
            } else {
                "built_in"
            },
        }
    }

    /// Inspect nonsecret resolved settings without building an engine.
    /// This operation reads no environment, file, answer or credential source;
    /// it uses only values already captured by this builder. It sends nothing.
    /// Status renders this view; the caller owns its text and output format.
    ///
    /// # Errors
    /// Returns the same backend selection and address-secret refusals as build.
    pub fn resolved_settings(&self) -> Result<ResolvedSettings, Error> {
        let selected = self.selected()?;
        if selected.backend.address_contains_key(
            selected.key.as_ref().map(|Secret(value)| value.as_ref()),
        ) {
            return Err(Error::usage(crate::core::KEY_IN_ADDRESS));
        }
        let metadata = &self.captured.metadata;
        let cache = self.cache_settings();
        Ok(ResolvedSettings {
            configuration_path: metadata.path.clone(),
            configuration_present: metadata.present,
            configuration_shared: metadata.shared,
            backend_name: selected.name,
            url: selected.backend.url().as_str().to_owned(),
            url_source: selected.url_source,
            model: selected.backend.model().as_str().to_owned(),
            model_source: selected.model_source,
            key_variable: selected.variable,
            key_variables: selected.variables,
            api_key_set: selected.key.is_some(),
            cache_enabled: cache.cache_enabled,
            cache_enabled_source: cache.cache_enabled_source,
            cache_path: cache.cache_path,
            cache_path_source: cache.cache_path_source,
            cache_is_platform_default: cache.cache_is_platform_default,
            cache_target_bytes: cache.cache_target_bytes,
            cache_target_source: cache.cache_target_source,
            usage_path: self.seeded.as_ref().and_then(|seeded| seeded.usage.clone()),
        })
    }
}
