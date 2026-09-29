//! The process engine and SQL settings fixed before its first build.

use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::Duration;

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{BatchSetting, Engine, EngineBuilder};

use crate::{Failure, guard};

/// The settings SQL stored, each applied over the environment at the build.
#[derive(Debug, Default)]
struct Stored {
    throttle: Option<u8>,
    batch: Option<BatchSetting>,
    max_requests: Option<Option<usize>>,
    max_request_bytes: Option<usize>,
    cache: Option<Option<PathBuf>>,
    total: Option<u64>,
    model: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    profile: Option<String>,
    record: Option<PathBuf>,
    replay: Option<PathBuf>,
}

impl Stored {
    fn apply(&self, mut builder: EngineBuilder) -> Result<EngineBuilder, thinkthen::Error> {
        if let Some(value) = self.throttle {
            builder = builder.throttle(value)?;
        }
        if let Some(value) = self.batch {
            builder = builder.batch(value);
        }
        if let Some(value) = self.max_requests {
            builder = builder.max_requests(value)?;
        }
        if let Some(value) = self.max_request_bytes {
            builder = builder.max_request_bytes(value)?;
        }
        if let Some(value) = self.total {
            builder = builder.max_requests_total(Some(value));
        }
        match &self.cache {
            Some(Some(folder)) => builder = builder.cache_at(folder)?,
            Some(None) => builder = builder.no_cache(),
            None => {}
        }
        if let Some(value) = &self.model {
            builder = builder.model(value)?;
        }
        if let Some(value) = self.timeout {
            builder = builder.timeout(value)?;
        }
        if let Some(value) = self.max_retries {
            builder = builder.max_retries(value);
        }
        if let Some(value) = &self.profile {
            builder = builder.profile_json(value)?;
        }
        if let Some(value) = &self.record {
            builder = builder.record(value)?;
        }
        if let Some(value) = &self.replay {
            builder = builder.replay(value)?;
        }
        Ok(builder)
    }
}

static STORED: Mutex<Stored> = Mutex::new(Stored {
    throttle: None,
    batch: None,
    max_requests: None,
    max_request_bytes: None,
    cache: None,
    total: None,
    model: None,
    timeout: None,
    max_retries: None,
    profile: None,
    record: None,
    replay: None,
});

static ENGINE: OnceLock<Engine> = OnceLock::new();

fn batch(value: &serde_json::Value) -> Result<BatchSetting, Failure> {
    if value.as_str() == Some("max") {
        return Ok(BatchSetting::Max);
    }
    let count = value
        .as_u64()
        .and_then(|count| usize::try_from(count).ok())
        .and_then(NonZeroUsize::new)
        .ok_or_else(|| Failure::usage("settings batch has an invalid value"))?;
    Ok(BatchSetting::Records(count))
}

fn request_total(value: &serde_json::Value) -> Result<Option<u64>, Failure> {
    let total = value.as_u64();
    if total == Some(0) {
        return Err(Failure::usage(
            "a request total is a whole number of 1 or more",
        ));
    }
    Ok(total)
}

/// The selected cap for a safe SQLite refusal sentence.
pub(crate) fn total() -> Option<u64> {
    stored().total
}

fn stored() -> MutexGuard<'static, Stored> {
    STORED.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The process engine, built on the first call that can send from the
/// environment and the stored settings. A failed build is tried again.
pub(crate) fn engine() -> Result<&'static Engine, Failure> {
    if let Some(engine) = ENGINE.get() {
        return Ok(engine);
    }
    let held = stored();
    if let Some(engine) = ENGINE.get() {
        return Ok(engine);
    }
    let built = held.apply(EngineBuilder::from_env()?)?.build()?;
    Ok(ENGINE.get_or_init(|| built))
}

/// The engine when one is built, for `thinkthen_usage`, which builds none.
pub(crate) fn built() -> Option<&'static Engine> {
    ENGINE.get()
}

/// The refusal once the process request total is spent.
pub(crate) fn spent(total: u64) -> Failure {
    Failure::usage(format!(
        "this process has sent its total of {total} requests (thinkthen_configure)"
    ))
}

/// Replace all engine settings atomically, before the engine is built.
pub(crate) fn configure(context: &Context<'_>) -> rusqlite::Result<String> {
    Ok(guard("thinkthen_configure", || {
        let source = match context.get_raw(0) {
            ValueRef::Text(bytes) => std::str::from_utf8(bytes)
                .map_err(|_| Failure::usage("settings JSON is UTF-8 text"))?,
            _ => return Err(Failure::usage("settings JSON is one object")),
        };
        // The shared reader detects duplicate names before serde_json flattens them.
        EngineBuilder::validate_settings_json(source)?;
        let value: serde_json::Value = serde_json::from_str(source)
            .map_err(|_| Failure::usage("settings JSON is one object"))?;
        let fields = value
            .as_object()
            .ok_or_else(|| Failure::usage("settings JSON is one object"))?;
        let mut next = Stored::default();
        for (key, value) in fields {
            match key.as_str() {
                "base_url" => return Err(Failure::usage("settings JSON has unknown key base_url")),
                "model" => next.model = value.as_str().map(str::to_owned),
                "throttle" => next.throttle = value.as_u64().and_then(|n| u8::try_from(n).ok()),
                "batch" => next.batch = Some(batch(value)?),
                "max_requests" => {
                    next.max_requests = Some(
                        value
                            .as_u64()
                            .map(|n| usize::try_from(n).unwrap_or(usize::MAX)),
                    )
                }
                "max_requests_total" => next.total = request_total(value)?,
                "max_request_bytes" => {
                    next.max_request_bytes = value
                        .as_u64()
                        .map(|n| usize::try_from(n).unwrap_or(usize::MAX))
                }
                "cache" => next.cache = Some(value.as_str().map(PathBuf::from)),
                "timeout" => next.timeout = value.as_u64().map(Duration::from_secs),
                "max_retries" => {
                    next.max_retries = value.as_u64().and_then(|n| u32::try_from(n).ok())
                }
                "profile" => next.profile = value.as_str().map(str::to_owned),
                "record" => next.record = value.as_str().map(PathBuf::from),
                "replay" => next.replay = value.as_str().map(PathBuf::from),
                _ => {
                    return Err(Failure::defect(
                        "the shared settings reader accepted an unknown field",
                    ));
                }
            }
        }
        let mut held = stored();
        if ENGINE.get().is_some() {
            return Err(Failure::usage(
                "settings apply before the first call; this process already built its engine",
            ));
        }
        next.apply(EngineBuilder::from_env()?)?;
        *held = next;
        Ok(source.to_owned())
    })?)
}
