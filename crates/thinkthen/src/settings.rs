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
    backend: Option<BackendName>,
    model: Option<String>,
    url: Option<String>,
    api_key: Option<Key>,
    batch: Option<Batch>,
    max_requests: Option<u64>,
    retry: Retry,
    response_store: Option<crate::ResponseStore>,
}
#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Input {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "BackendName")]
    #[serde(deserialize_with = "explicit_backend")]
    backend: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "crate::core::diagnostic::max_requests")]
    max_requests: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    response_store: Option<crate::ResponseStore>,
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
        max_requests: Option<u64>,
        response_store: Option<crate::ResponseStore>,
    ) -> Self {
        Self {
            backend: Some(backend),
            model: Some(model.to_owned()),
            url: url.map(str::to_owned),
            api_key: None,
            batch: Some(batch),
            max_requests,
            retry,
            response_store,
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
    /// The explicit backend, or the built-in default before environment resolution.
    #[must_use]
    pub const fn backend(&self) -> BackendName {
        match self.backend {
            Some(backend) => backend,
            None => BackendName::SystemOne,
        }
    }
    /// Select a backend with its own model and environment defaults.
    #[must_use]
    pub fn with_backend(mut self, backend: BackendName) -> Self {
        self.backend = Some(backend);
        self
    }
    /// Resolve the explicit backend, then THINKTHEN_BACKEND, then the built-in default.
    /// # Errors
    /// Returns usage.backend for an unknown environment value before reading credentials.
    pub fn resolve_backend(
        &self,
        variable: impl Fn(&str) -> Option<String>,
    ) -> Result<BackendName, Error> {
        match self.backend {
            Some(backend) => Ok(backend),
            None => variable("THINKTHEN_BACKEND")
                .map(|name| name.parse())
                .transpose()
                .map(Option::unwrap_or_default),
        }
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
    /// Bound actual send attempts over the resulting client's lifetime.
    /// None is unlimited; zero prevents every remote send. Clones share the allowance.
    #[must_use]
    pub fn max_requests(mut self, max_requests: Option<u64>) -> Self {
        self.max_requests = max_requests;
        self
    }
    /// The configured client-lifetime request allowance, not a remaining balance.
    #[must_use]
    pub const fn request_limit(&self) -> Option<u64> {
        self.max_requests
    }
    /// Admit a command or explicit from-environment construction's allowance.
    pub(crate) fn environment_request_limit(mut self, text: Option<String>) -> Result<Self, Error> {
        if self.max_requests.is_none() {
            self.max_requests = text
                .map(|text| {
                    text.parse::<u64>().map_err(|_| {
                        settings_error(crate::core::diagnostic::requirement(
                            "max_requests",
                            "an unsigned integer",
                            "an invalid request allowance (contents redacted)",
                        ))
                    })
                })
                .transpose()?;
        }
        Ok(self)
    }
    /// Explicitly enable local retention, or disable it with `None`.
    #[must_use]
    pub fn response_store(mut self, store: Option<crate::ResponseStore>) -> Self {
        self.response_store = store;
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
        table.selection(self.backend().as_str(), self.model.as_deref())
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
        let selected = self.resolve_backend(&variable)?;
        let table = BackendTable::default();
        let mut backend = table.resolve_route(
            selected.as_str(),
            self.url.as_deref(),
            self.model.as_deref(),
            &variable,
        )?;
        let store = self
            .response_store
            .as_ref()
            .map(|config| {
                crate::response_store::validate_route(&backend)?;
                crate::response_store::Store::open(config.clone()).map(std::sync::Arc::new)
            })
            .transpose()?;
        if !store.as_ref().is_some_and(|store| store.replay()) {
            table.authorize(&mut backend, self.api_key.as_ref(), &variable)?;
        }
        let batch = match self.batch {
            Some(batch) => batch,
            None if selected == BackendName::Stub => Batch::default(),
            None => Batch::resolve(None, variable)?,
        };
        let mut engine = Engine::new(backend)
            .batch(batch)
            .retry_policy(self.retry.policy()?);
        engine.account = crate::usage::Account::new(self.max_requests);
        engine.store = store;
        Ok(engine)
    }
}
impl TryFrom<Input> for Settings {
    type Error = Error;
    fn try_from(value: Input) -> Result<Self, Error> {
        let api_key = value.api_key.map(|key| key.parse()).transpose()?;
        Ok(Self {
            backend: value.backend.map(|name| name.parse()).transpose()?,
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
            max_requests: value.max_requests,
            response_store: value.response_store,
        })
    }
}
impl From<Settings> for Input {
    fn from(value: Settings) -> Self {
        Self {
            backend: value.backend.map(|backend| backend.as_str().to_owned()),
            model: value.model,
            url: value.url,
            api_key: None,
            batch: value
                .batch
                .map(|batch| serde_json::Number::from(batch.get().get())),
            retry: value.retry,
            max_requests: value.max_requests,
            response_store: value.response_store,
        }
    }
}
fn settings_error(message: impl Into<String>) -> Error {
    Error::new(ErrorKind::Settings, message)
}

fn explicit_backend<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    crate::core::diagnostic::backend(deserializer).map(Some)
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
