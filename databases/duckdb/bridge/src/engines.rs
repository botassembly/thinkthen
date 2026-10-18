//! The engines this process keeps, one per distinct set of session settings
//! (ticket 0110 decision 5).
//!
//! Every caller-session value reaches an `EngineBuilder::from_env()` setter.
//! The complete engine key includes every value that changes a built engine.
//! Each call validates and probes paths before it reads the map, so a map hit
//! never skips a check. The builder refuses a conflicting process throttle;
//! a failed build is never stored.
//!
//! `SET thinkthen_max_requests_total` caps sends in this process. The
//! engine's one process total reserves each actual send, including a retry;
//! `within_total` only cuts ordered rows before a call, from the same count.
//! Both reset in a forked child.

use std::sync::{Arc, Mutex, PoisonError};

use std::num::NonZeroUsize;
use thinkthen::{BatchSetting, CallOptions, CancelToken, Counters, Engine, EngineBuilder, Error};

use crate::errors::{RowError, usage};

/// The most engines one process keeps.
const MOST: usize = 16;

/// The engine settings as the caller's session holds them; `None` is unset.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Asked {
    pub(crate) settings: std::collections::BTreeMap<String, String>,
    pub(crate) max_requests_total: Option<i64>,
}
impl Asked {
    pub(crate) fn with_model(&mut self, model: String) {
        self.settings.insert(
            "model".to_owned(),
            serde_json::Value::String(model).to_string(),
        );
    }
    fn folder(&self, name: &str) -> Option<String> {
        self.settings
            .get(name)
            .and_then(|raw| serde_json::from_str(raw).ok())
    }
}

/// What the caller's own file system says about a folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Probe {
    Allowed,
    Refused,
}

/// One engine's key: the settings that change what an engine is.
type Key = std::collections::BTreeMap<String, String>;

#[allow(dead_code, reason = "used by the C++ bridge across its verb families")]
pub(crate) fn options<'a>(
    deadline_ms: i64,
    token: &'a CancelToken,
    total: Option<i64>,
) -> Result<CallOptions<'a>, String> {
    let limit = total.map(total_of).transpose()?;
    let options = if deadline_ms == -1 {
        CallOptions::new()
    } else {
        CallOptions::new()
            .deadline_millis(deadline_ms)
            .map_err(|error| RowError::from(error).text)?
    };
    Ok(options.cancel(token).max_requests_total(limit))
}

pub(crate) fn batch_setting(batch: &str) -> Result<BatchSetting, String> {
    let setting = if batch == "max" {
        BatchSetting::Max
    } else if !batch.is_empty() && batch.bytes().all(|byte| byte.is_ascii_digit()) {
        let count = batch
            .parse::<usize>()
            .ok()
            .and_then(NonZeroUsize::new)
            .ok_or_else(|| usage("batch takes max or a whole number of at least 1"))?;
        BatchSetting::Records(count)
    } else {
        return Err(usage("batch takes max or a whole number of at least 1"));
    };
    Ok(setting)
}

/// C++ scalar and warm calls read this session's batch cap and literal context.
pub(crate) fn options_for<'a>(
    deadline_ms: i64,
    token: &'a CancelToken,
    total: Option<i64>,
    batch: Option<&str>,
    context: Option<&'a str>,
) -> Result<CallOptions<'a>, String> {
    let mut options = options(deadline_ms, token, total)?;
    if let Some(batch) = batch {
        let setting = batch_setting(batch)?;
        options = options.batch(setting);
    }
    if let Some(context) = context {
        options = options.context(context);
    }
    Ok(options)
}

#[allow(dead_code, reason = "used by the C++ bridge across its verb families")]
pub(crate) fn call_error(error: Error, total: Option<i64>) -> RowError {
    if error.send_budget_denial().is_some() {
        let total = total.unwrap_or_default();
        return RowError::usage(&format!(
            "this process has spent its request total of {total}; raise SET thinkthen_max_requests_total or RESET it"
        ));
    }
    RowError::from(error)
}

#[derive(Debug)]
struct Entry {
    key: Key,
    engine: Arc<Engine>,
    used: u64,
}

#[derive(Debug)]
struct Registry {
    pid: u32,
    clock: u64,
    historical: Counters,
    persistence_failed: bool,
    kept: Vec<Entry>,
}

static ENGINES: Mutex<Registry> = Mutex::new(Registry {
    pid: 0,
    clock: 0,
    historical: Counters::ZERO,
    persistence_failed: false,
    kept: Vec::new(),
});

/// Flush every engine this process kept (ADR 0113). A busy registry skips
/// the flush rather than waiting on a lock, and a registry another process
/// built, as in a forked child, is left alone.
pub(crate) fn finish_usage() {
    let kept: Vec<Arc<Engine>> = match ENGINES.try_lock() {
        Ok(held) if held.pid == std::process::id() => held
            .kept
            .iter()
            .map(|entry| Arc::clone(&entry.engine))
            .collect(),
        _ => return,
    };
    for engine in kept {
        let _status = engine.finish_usage_status();
    }
}

fn registry() -> std::sync::MutexGuard<'static, Registry> {
    let mut held = ENGINES.lock().unwrap_or_else(PoisonError::into_inner);
    let pid = std::process::id();
    if held.pid != pid {
        held.pid = pid;
        held.clock = 0;
        held.historical = Counters::ZERO;
        held.persistence_failed = false;
        held.kept.clear();
    }
    held
}

impl Registry {
    fn found(&mut self, key: &Key) -> Option<Arc<Engine>> {
        let entry = self.kept.iter_mut().find(|entry| &entry.key == key)?;
        self.clock = self.clock.saturating_add(1);
        entry.used = self.clock;
        Some(Arc::clone(&entry.engine))
    }

    fn room(&mut self) -> Result<(), String> {
        if self.kept.len() < MOST {
            return Ok(());
        }
        let at = self.kept.iter().enumerate()
            .filter(|(_, entry)| Arc::strong_count(&entry.engine) == 1)
            .min_by_key(|(_, entry)| entry.used)
            .map(|(at, _)| at)
            .ok_or_else(|| usage("16 ThinkThen engine settings plans are in use; finish a holding query, reuse current settings, or start a new process"))?;
        let old = self.kept.remove(at);
        // Retirement already waits for the writer in Engine's drop. Observe
        // that same finalization before losing its failure latch.
        self.persistence_failed |=
            old.engine.finish_usage_status() == thinkthen::UsagePersistence::Failed;
        self.historical = self.historical + old.engine.usage();
        Ok(())
    }
}

/// The engine for these settings, built on first use. `probe` opens a path
/// through the caller's own file system.
pub(crate) fn engine_for(
    asked: &Asked,
    probe: impl FnMut(&str) -> Probe,
) -> Result<Arc<Engine>, String> {
    engine_for_typed(asked, probe).map_err(|error| error.text)
}

pub(crate) fn engine_for_typed(
    asked: &Asked,
    mut probe: impl FnMut(&str) -> Probe,
) -> Result<Arc<Engine>, RowError> {
    let (key, builder) = checked(asked)?;
    for (folder, kind) in [
        (asked.folder("cache"), "cache"),
        (asked.folder("record"), "recording"),
        (asked.folder("replay"), "recording"),
    ] {
        let Some(folder) = folder.as_ref() else {
            continue;
        };
        if kind == "cache" && folder == "off" {
            continue;
        }
        if !folder.starts_with('/') || folder.contains("://") {
            return Err(RowError::usage(if kind == "cache" {
                "a cache folder set from SQL is an absolute local path with no scheme"
            } else {
                "a SQL folder is an absolute local path with no scheme"
            }));
        }
        if probe(&format!("{}/.probe", folder.trim_end_matches('/'))) == Probe::Refused {
            return Err(RowError::usage(&format!(
                "the {kind} folder is outside what this database's file settings allow"
            )));
        }
    }
    {
        let mut engines = registry();
        if let Some(engine) = engines.found(&key) {
            return Ok(engine);
        }
        if engines.kept.len() >= MOST
            && !engines
                .kept
                .iter()
                .any(|entry| Arc::strong_count(&entry.engine) == 1)
        {
            return Err(RowError::usage(
                "16 ThinkThen engine settings plans are in use; finish a holding query, reuse current settings, or start a new process",
            ));
        }
    }
    // The build runs outside the lock, so a fork during a build never
    // leaves the child's map locked. A racing build of the same key loses
    // to the one stored first.
    let built = Arc::new(builder.build().map_err(RowError::from)?);
    let mut engines = registry();
    if let Some(engine) = engines.found(&key) {
        return Ok(engine);
    }
    engines.room().map_err(|_| RowError::usage("16 ThinkThen engine settings plans are in use; finish a holding query, reuse current settings, or start a new process"))?;
    engines.clock = engines.clock.saturating_add(1);
    let used = engines.clock;
    engines.kept.push(Entry {
        key,
        engine: Arc::clone(&built),
        used,
    });
    crate::usage_ffi::flush_usage_at_exit();
    Ok(built)
}

/// Validate every raw setting before reusing an engine from the registry.
fn checked(asked: &Asked) -> Result<(Key, EngineBuilder), RowError> {
    let mut builder = EngineBuilder::from_env()
        .map_err(RowError::from)?
        .shared_host();
    for (name, raw) in &asked.settings {
        builder = builder.setting(name, raw).map_err(RowError::from)?;
    }
    Ok((asked.settings.clone(), builder))
}

fn total_of(total: i64) -> Result<u64, String> {
    u64::try_from(total).map_err(|_| usage("a request total is a whole number of 0 or more"))
}

/// The texts a call may send, and the refusal it raises after them.
type TypedAllowed = (Vec<String>, Option<RowError>);

/// The texts one call may send under the process's request total, and the
/// refusal the call raises after it sends them when the total cut it short.
/// A spent total refuses before anything is sent.
pub(crate) fn within_total_typed(
    asked: &Asked,
    mut texts: Vec<String>,
) -> Result<TypedAllowed, RowError> {
    let Some(total) = asked.max_requests_total else {
        return Ok((texts, None));
    };
    let total = total_of(total)
        .map_err(|_| RowError::usage("a request total is a whole number of 0 or more"))?;
    let spent = thinkthen::process_requests_sent();
    let spent_out = || {
        RowError::usage(&format!(
            "this process has spent its request total of {total}; raise SET thinkthen_max_requests_total or RESET it"
        ))
    };
    let left = usize::try_from(total.saturating_sub(spent)).unwrap_or(usize::MAX);
    if left == 0 {
        return Err(spent_out());
    }
    if texts.len() <= left {
        return Ok((texts, None));
    }
    texts.truncate(left);
    Ok((texts, Some(spent_out())))
}

/// Historical counters and the counters of every resident engine, summed.
pub(crate) fn usage_totals() -> [(&'static str, u64); 4] {
    let engines = registry();
    let totals = engines.historical
        + engines
            .kept
            .iter()
            .map(|entry| entry.engine.usage())
            .sum::<Counters>();
    [
        ("requests_sent", totals.requests_sent()),
        ("cache_answers", totals.cache_answers()),
        ("input_tokens", totals.input_tokens()),
        ("output_tokens", totals.output_tokens()),
    ]
}

/// Observe resident engines and failures retained at retirement without waiting.
pub(crate) fn usage_status() -> thinkthen::UsagePersistence {
    use thinkthen::UsagePersistence;
    match ENGINES.try_lock() {
        Ok(held) if held.pid == std::process::id() => {
            let historical = held.persistence_failed.then_some(UsagePersistence::Failed);
            UsagePersistence::aggregate(
                historical.into_iter().chain(
                    held.kept
                        .iter()
                        .map(|entry| entry.engine.usage_persistence()),
                ),
            )
        }
        Ok(_) => UsagePersistence::Disabled,
        Err(_) => UsagePersistence::Pending,
    }
}
