//! The process engine and SQL settings fixed before its first build.

use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{Engine, EngineBuilder};

use crate::{Failure, guard};

const SETTING_NAMES: &[&str] = &[
    "base_url",
    "backend",
    "model",
    "throttle",
    "batch",
    "max_requests",
    "max_requests_total",
    "max_request_bytes",
    "cache",
    "refresh_cache",
    "timeout",
    "max_retries",
    "profile",
    "record",
    "replay",
    "usd_per_million_input",
    "usd_per_million_output",
];

type SettingPairs = Vec<(String, String)>;

/// The refusal for a folder on a single-thread host. The store opens its
/// connection on the pipeline's thread (ADR 0111's 2026-09-30 amendment).
const SINGLE_THREAD: &str = "a cache, record or replay folder needs a thread-safe SQLite, and this host's SQLite is single-threaded; name no folder, or load thinkthen into a thread-safe SQLite";

fn threads_ready(settings: &SettingPairs) -> Result<(), Failure> {
    let folder = |name| {
        settings
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, raw)| raw)
    };
    let named = folder("record").is_some()
        || folder("replay").is_some()
        || match folder("cache") {
            Some(raw) => raw != "false",
            None => std::env::var("THINKTHEN_CACHE").is_ok_and(|v| !v.trim().is_empty()),
        };
    if named && !crate::ffi::host_threads() {
        return Err(Failure::usage(SINGLE_THREAD));
    }
    Ok(())
}

fn apply(
    settings: &SettingPairs,
    mut builder: EngineBuilder,
) -> Result<EngineBuilder, thinkthen::Error> {
    for (name, raw) in settings {
        builder = builder.setting(name, raw)?;
    }
    Ok(builder)
}

static STORED: Mutex<SettingPairs> = Mutex::new(Vec::new());

static ENGINE: OnceLock<Engine> = OnceLock::new();

/// The selected cap for a safe SQLite refusal sentence.
pub(crate) fn total() -> Option<u64> {
    stored()
        .iter()
        .find(|(name, _)| name == "max_requests_total")
        .and_then(|(_, raw)| serde_json::from_str(raw).ok())
}

fn stored() -> MutexGuard<'static, SettingPairs> {
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
    threads_ready(&held)?;
    let built = apply(&held, EngineBuilder::from_env()?.shared_host())?.build()?;
    crate::ffi::flush_usage_at_exit();
    Ok(ENGINE.get_or_init(|| built))
}

/// Flush the process engine's usage totals. Reading the `OnceLock` takes no
/// lock, and the engine skips counters another process built.
pub(crate) fn finish_usage() {
    if let Some(engine) = ENGINE.get() {
        let _status = engine.finish_usage_status();
    }
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
        for name in fields.keys() {
            if !SETTING_NAMES.contains(&name.as_str()) {
                return Err(Failure::usage(format!(
                    "settings JSON has unknown key {name}"
                )));
            }
        }
        // SQLite admits positive process caps; a refused replacement keeps the prior settings.
        if fields
            .get("max_requests_total")
            .and_then(serde_json::Value::as_u64)
            == Some(0)
        {
            return Err(Failure::usage(
                "a request total is a whole number of 1 or more",
            ));
        }
        let next = fields
            .iter()
            .map(|(name, value)| (name.clone(), value.to_string()))
            .collect();
        let mut held = stored();
        if ENGINE.get().is_some() {
            return Err(Failure::usage(
                "settings apply before the first call; this process already built its engine",
            ));
        }
        threads_ready(&next)?;
        apply(&next, EngineBuilder::from_env()?)?;
        *held = next;
        Ok(source.to_owned())
    })?)
}
