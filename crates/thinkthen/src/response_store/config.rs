//! Explicit admission of caller-owned response retention.
use crate::{Code, Error};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// How an explicitly selected local store serves this client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ResponseMode {
    /// Reuse unexpired answers and retain misses within bounded eviction limits.
    Cache,
    /// Append immutable live exchanges; never evict recordings to make room.
    Record,
    /// Read a fixed committed recording snapshot, without credentials or network.
    Replay,
}

/// Opt-in local retention of model inputs and responses, without encryption at rest.
/// The directory must be private and on a local filesystem. SQLite allocation
/// can exceed the logical payload limit; its separate allowance is twice that
/// limit plus 16 MiB. No environment variable enables this setting.
/// Network filesystems are unsupported. Cache mode intentionally reuses one
/// sampled answer; change `model_generation` when a provider alias changes.
/// Replay selects the latest recorded answer committed before client creation,
/// ignores cache expiry, and never falls back to live execution.
///
/// ```no_run
/// use thinkthen::{ResponseMode, ResponseStore, Settings};
/// let store = ResponseStore::new("./responses", ResponseMode::Replay, "evaluation", "v1")?;
/// let client = Settings::default().response_store(Some(store)).engine(|_| None)?;
/// # Ok::<(), thinkthen::Error>(())
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(try_from = "Input", into = "Input")]
#[schemars(with = "Input")]
pub struct ResponseStore {
    pub(super) path: PathBuf,
    pub(super) mode: ResponseMode,
    pub(super) namespace: String,
    pub(super) generation: String,
    pub(super) max_bytes: u64,
    pub(super) max_entries: u64,
    pub(super) max_age: u64,
    pub(super) busy_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Input {
    path: PathBuf,
    mode: ResponseMode,
    namespace: String,
    model_generation: String,
    #[serde(default = "default_bytes")]
    #[schemars(range(min = 1))]
    max_bytes: u64,
    #[serde(default = "default_entries")]
    #[schemars(range(min = 1))]
    max_entries: u64,
    #[serde(default = "default_age")]
    #[schemars(range(min = 1))]
    max_age: u64,
    #[serde(default = "default_busy")]
    #[schemars(range(min = 1))]
    busy_timeout_ms: u64,
}
const fn default_bytes() -> u64 {
    64 * 1024 * 1024
}
const fn default_entries() -> u64 {
    4096
}
const fn default_age() -> u64 {
    86400
}
const fn default_busy() -> u64 {
    5000
}

impl ResponseStore {
    /// Select a private directory, mode and explicit nonsecret account/model labels.
    /// Labels are not credentials and must not be derived from credentials.
    /// # Errors
    /// Returns `usage.settings` for an empty path or blank label.
    pub fn new(
        path: impl Into<PathBuf>,
        mode: ResponseMode,
        namespace: impl Into<String>,
        model_generation: impl Into<String>,
    ) -> Result<Self, Error> {
        Input {
            path: path.into(),
            mode,
            namespace: namespace.into(),
            model_generation: model_generation.into(),
            max_bytes: default_bytes(),
            max_entries: default_entries(),
            max_age: default_age(),
            busy_timeout_ms: default_busy(),
        }
        .try_into()
    }
    /// Override logical retained bytes, eligible asks and cache age in seconds.
    /// Recordings never expire; record mode refuses its byte quota instead of evicting.
    /// # Errors
    /// Returns `usage.settings` for zero or unrepresentable bounds.
    pub fn limits(
        self,
        max_bytes: u64,
        max_entries: u64,
        max_age: Duration,
    ) -> Result<Self, Error> {
        if max_age.subsec_nanos() != 0 {
            return Err(settings());
        }
        let mut input: Input = self.into();
        input.max_bytes = max_bytes;
        input.max_entries = max_entries;
        input.max_age = max_age.as_secs();
        input.try_into()
    }
    /// Bound waiting for a competing SQLite writer, in whole milliseconds.
    /// # Errors
    /// Returns `usage.settings` for zero, fractional milliseconds or an excessive wait.
    pub fn busy_timeout(self, duration: Duration) -> Result<Self, Error> {
        if !duration.subsec_nanos().is_multiple_of(1_000_000) {
            return Err(settings());
        }
        let mut input: Input = self.into();
        input.busy_timeout_ms = u64::try_from(duration.as_millis()).map_err(|_| settings())?;
        input.try_into()
    }
    /// The admitted local directory.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
    /// The selected retention or replay policy.
    #[must_use]
    pub const fn mode(&self) -> ResponseMode {
        self.mode
    }
    pub(super) fn allocation(&self) -> Result<u64, Error> {
        self.max_bytes
            .checked_mul(2)
            .and_then(|bytes| bytes.checked_add(16 * 1024 * 1024))
            .filter(|bytes| *bytes <= (u64::from(u32::MAX) - 1) * 4096)
            .ok_or_else(settings)
    }
}
impl TryFrom<Input> for ResponseStore {
    type Error = Error;
    fn try_from(value: Input) -> Result<Self, Error> {
        if value.path.as_os_str().is_empty()
            || value.namespace.trim().is_empty()
            || value.model_generation.trim().is_empty()
            || value.max_bytes == 0
            || value.max_entries == 0
            || value.max_age == 0
            || value.busy_timeout_ms == 0
            || value.max_entries > i64::MAX as u64
            || value.max_age > i64::MAX as u64
            || value.busy_timeout_ms > i32::MAX as u64
        {
            return Err(settings());
        }
        let store = Self {
            path: value.path,
            mode: value.mode,
            namespace: value.namespace,
            generation: value.model_generation,
            max_bytes: value.max_bytes,
            max_entries: value.max_entries,
            max_age: value.max_age,
            busy_ms: value.busy_timeout_ms,
        };
        store.allocation()?;
        Ok(store)
    }
}
impl From<ResponseStore> for Input {
    fn from(value: ResponseStore) -> Self {
        Self {
            path: value.path,
            mode: value.mode,
            namespace: value.namespace,
            model_generation: value.generation,
            max_bytes: value.max_bytes,
            max_entries: value.max_entries,
            max_age: value.max_age,
            busy_timeout_ms: value.busy_ms,
        }
    }
}
fn settings() -> Error {
    Error::from_contract(
        Code::UsageSettings,
        "response_store requires a path, nonblank namespace and model_generation, and positive representable limits",
    )
}
