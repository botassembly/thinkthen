//! Typed client settings resolve the selected backend once and keep keys private.
use crate::{BackendTable, Batch, Engine, Error, ErrorKind, Key, RetryPolicy};
use std::num::NonZeroU64;
use std::time::Duration;

pub use crate::core::BackendName;

/// The existing retry policy expressed in integer milliseconds at JSON boundaries.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(default, deny_unknown_fields)]
pub struct Retry {
    #[serde(deserialize_with = "retry_count")]
    #[schemars(schema_with = "retry_count_schema")]
    max_retries: u32,
    #[serde(deserialize_with = "crate::core::diagnostic::initial_backoff_ms")]
    initial_backoff_ms: u64,
    #[serde(deserialize_with = "crate::core::diagnostic::attempt_timeout_ms")]
    attempt_timeout_ms: NonZeroU64,
}
impl Default for Retry {
    fn default() -> Self {
        Self {
            max_retries: crate::retry::DEFAULT_RETRIES,
            initial_backoff_ms: crate::retry::DEFAULT_BACKOFF_MS,
            attempt_timeout_ms: crate::retry::DEFAULT_TIMEOUT_MS,
        }
    }
}
impl From<RetryPolicy> for Retry {
    fn from(policy: RetryPolicy) -> Self {
        Self {
            max_retries: policy.max_retries as u32,
            initial_backoff_ms: policy.initial_backoff.as_millis() as u64,
            attempt_timeout_ms: NonZeroU64::new(policy.attempt_timeout.as_millis() as u64)
                .unwrap_or(NonZeroU64::MIN),
        }
    }
}
impl Retry {
    /// Maximum resends after a retryable response; the original send is separate.
    pub const MAX_RETRIES: u32 = 10;

    /// Admit a retry policy without rounding fractional milliseconds.
    /// # Errors
    /// Returns a settings error for fractional, zero-timeout or excessive durations.
    pub fn new(
        max_retries: u32,
        initial_backoff: Duration,
        attempt_timeout: Duration,
    ) -> Result<Self, Error> {
        fn millis(value: Duration) -> Result<u64, Error> {
            if !value.subsec_nanos().is_multiple_of(1_000_000) {
                return Err(settings_error(crate::core::diagnostic::requirement(
                    "retry",
                    "durations in whole milliseconds",
                    "a fractional millisecond duration",
                )));
            }
            u64::try_from(value.as_millis()).map_err(|_| {
                settings_error(crate::core::diagnostic::requirement(
                    "retry",
                    "durations within the supported range",
                    "an oversized duration",
                ))
            })
        }
        Ok(Self {
            max_retries: checked_retry_count(max_retries)?,
            initial_backoff_ms: millis(initial_backoff)?,
            attempt_timeout_ms: NonZeroU64::new(millis(attempt_timeout)?).ok_or_else(|| {
                settings_error(crate::core::diagnostic::requirement(
                    "retry.attempt_timeout_ms",
                    "a positive duration",
                    "zero milliseconds",
                ))
            })?,
        })
    }
    fn policy(self) -> Result<RetryPolicy, Error> {
        Ok(RetryPolicy {
            max_retries: usize::try_from(checked_retry_count(self.max_retries)?).map_err(|_| {
                settings_error(crate::core::diagnostic::requirement(
                    "retry.max_retries",
                    "a count within the platform range",
                    "an oversized count",
                ))
            })?,
            initial_backoff: Duration::from_millis(self.initial_backoff_ms),
            attempt_timeout: Duration::from_millis(self.attempt_timeout_ms.get()),
        })
    }
}

/// Immutable operational settings; serialization always omits the API key.
#[non_exhaustive]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "Input", into = "Input")]
#[schemars(with = "Input")]
pub struct Settings {
    backend: BackendName,
    model: Option<String>,
    url: Option<String>,
    api_key: Option<Key>,
    batch: Option<Batch>,
    retry: Retry,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Input {
    #[serde(default = "default_backend")]
    #[schemars(with = "BackendName")]
    #[serde(deserialize_with = "crate::core::diagnostic::backend")]
    backend: String,
    #[serde(default)]
    #[schemars(!default)]
    #[serde(deserialize_with = "crate::core::diagnostic::model")]
    model: Option<String>,
    #[serde(default)]
    #[schemars(!default)]
    #[serde(deserialize_with = "crate::core::diagnostic::url")]
    url: Option<String>,
    #[serde(skip_serializing)]
    #[serde(default)]
    #[schemars(!default)]
    #[serde(deserialize_with = "crate::core::diagnostic::api_key")]
    api_key: Option<String>,
    #[schemars(with = "Option<Batch>")]
    #[serde(default)]
    #[schemars(!default)]
    #[serde(deserialize_with = "crate::core::diagnostic::batch")]
    batch: Option<serde_json::Number>,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::retry")]
    retry: Retry,
}
impl Settings {
    /// Copy only the explicit key from another snapshot without exposing bytes.
    #[cfg(feature = "native")]
    #[must_use]
    pub fn copy_api_key(mut self, source: &Self) -> Self {
        self.api_key = source.api_key.clone();
        self
    }
    pub(crate) fn resolved(
        backend: BackendName,
        model: &str,
        url: Option<&str>,
        batch: Batch,
        retry: Retry,
    ) -> Self {
        Self {
            backend,
            model: Some(model.to_owned()),
            url: url.map(str::to_owned),
            api_key: None,
            batch: Some(batch),
            retry,
        }
    }
    /// Admit canonical settings without resolving environment variables.
    /// # Errors
    /// Returns a settings or key error.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str::<Input>(json)
            .map_err(|error| {
                settings_error(crate::core::diagnostic::context(
                    "settings",
                    &error.to_string(),
                ))
            })?
            .try_into()
    }
    /// The selected backend; defaults to System One.
    #[must_use]
    pub const fn backend(&self) -> BackendName {
        self.backend
    }
    /// Select a backend with its own model and environment defaults.
    #[must_use]
    pub fn with_backend(mut self, backend: BackendName) -> Self {
        self.backend = backend;
        self
    }
    /// Supply a direct API key; it takes precedence over the selected variable.
    #[must_use]
    pub fn api_key(mut self, api_key: Key) -> Self {
        self.api_key = Some(api_key);
        self
    }
    /// Select an explicit model.
    #[must_use]
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
    /// Override the backend address using the library's URL policy.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    /// Select a validated batch size.
    #[must_use]
    pub fn batch(mut self, batch: Batch) -> Self {
        self.batch = Some(batch);
        self
    }
    /// Select a validated retry policy.
    #[must_use]
    pub fn retry(mut self, retry: Retry) -> Self {
        self.retry = retry;
        self
    }
    #[cfg(feature = "native")]
    pub(crate) fn selection<'a>(
        &'a self,
        table: &'a BackendTable,
    ) -> Result<(&'a str, &'a str), Error> {
        table.selection(self.backend.as_str(), self.model.as_deref())
    }
    #[cfg(feature = "native")]
    pub(crate) fn sql_retry_limit(self) -> Result<Self, Error> {
        let limit = Retry::default().max_retries;
        if self.retry.max_retries > limit {
            return Err(settings_error(crate::core::diagnostic::requirement(
                "retry.max_retries",
                &format!("a SQL retry count no greater than the default {limit}"),
                "an increased retry count",
            )));
        }
        Ok(self)
    }
    /// Resolve only this backend's environment and create its reusable engine.
    /// # Errors
    /// Returns the shared backend, URL, credential, batch or policy error.
    pub fn engine(&self, variable: impl Fn(&str) -> Option<String>) -> Result<Engine, Error> {
        let backend = BackendTable::default().resolve_direct(
            self.backend.as_str(),
            self.url.as_deref(),
            self.model.as_deref(),
            self.api_key.as_ref(),
            &variable,
        )?;
        let batch = match self.batch {
            Some(batch) => batch,
            None if self.backend == BackendName::Stub => Batch::default(),
            None => Batch::resolve(None, variable)?,
        };
        Ok(Engine::new(backend)
            .batch(batch)
            .retry_policy(self.retry.policy()?))
    }
}
impl TryFrom<Input> for Settings {
    type Error = Error;
    fn try_from(value: Input) -> Result<Self, Error> {
        let api_key = value.api_key.map(|key| key.parse()).transpose()?;
        Ok(Self {
            backend: value.backend.parse()?,
            model: value.model,
            url: value.url,
            api_key,
            batch: value
                .batch
                .map(|value| {
                    value
                        .as_u64()
                        .ok_or_else(|| {
                            Error::new(
                                ErrorKind::Batch,
                                crate::core::diagnostic::requirement(
                                    "batch",
                                    "a positive integer",
                                    "a noninteger or out-of-range value (contents redacted)",
                                ),
                            )
                        })
                        .and_then(Batch::try_from)
                })
                .transpose()?,
            retry: value.retry,
        })
    }
}
impl From<Settings> for Input {
    fn from(value: Settings) -> Self {
        Self {
            backend: value.backend.as_str().to_owned(),
            model: value.model,
            url: value.url,
            api_key: None,
            batch: value
                .batch
                .map(|batch| serde_json::Number::from(batch.get().get())),
            retry: value.retry,
        }
    }
}
fn settings_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Settings, message)
}

fn default_backend() -> String {
    BackendName::default().as_str().to_owned()
}

fn checked_retry_count(value: u32) -> Result<u32, Error> {
    if value > Retry::MAX_RETRIES {
        return Err(settings_error(crate::core::diagnostic::requirement(
            "retry.max_retries",
            &format!("an integer from 0 through {}", Retry::MAX_RETRIES),
            "an oversized count",
        )));
    }
    Ok(value)
}

fn retry_count<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    let count = crate::core::diagnostic::max_retries(deserializer)?;
    checked_retry_count(count).map_err(serde::de::Error::custom)
}

fn retry_count_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({"type":"integer", "format":"uint32", "minimum":0, "maximum":Retry::MAX_RETRIES})
}
