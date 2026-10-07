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
use std::time::Duration;

use std::num::NonZeroUsize;
use thinkthen::{BatchSetting, CallOptions, CancelToken, Counters, Engine, EngineBuilder, Error};

use crate::errors::{RowError, usage};

/// The most engines one process keeps.
const MOST: usize = 16;

/// The engine settings as the caller's session holds them; `None` is unset.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Asked {
    pub(crate) base_url: Option<String>,
    pub(crate) prices: Option<(String, String)>,
    pub(crate) refresh_cache: bool,
    pub(crate) backend: Option<String>,
    pub(crate) throttle: Option<i64>,
    pub(crate) max_requests: Option<i64>,
    pub(crate) max_request_bytes: Option<i64>,
    pub(crate) cache: Option<String>,
    pub(crate) max_requests_total: Option<i64>,
    pub(crate) model: Option<String>,
    pub(crate) timeout: Option<i64>,
    pub(crate) max_retries: Option<i64>,
    pub(crate) profile: Option<String>,
    pub(crate) record: Option<String>,
    pub(crate) replay: Option<String>,
}

/// What the caller's own file system says about a folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Probe {
    Allowed,
    Refused,
}

/// One engine's key: the settings that change what an engine is.
type Key = (
    Option<String>,
    Option<u8>,
    Option<usize>,
    Option<usize>,
    Option<String>,
    Option<String>,
    Option<u64>,
    Option<u32>,
    Option<String>,
    Option<String>,
    Option<String>,
    (Option<String>, Option<(String, String)>, bool),
);

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
    kept: Vec<Entry>,
}

static ENGINES: Mutex<Registry> = Mutex::new(Registry {
    pid: 0,
    clock: 0,
    historical: Counters::ZERO,
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
        engine.finish_usage();
    }
}

fn registry() -> std::sync::MutexGuard<'static, Registry> {
    let mut held = ENGINES.lock().unwrap_or_else(PoisonError::into_inner);
    let pid = std::process::id();
    if held.pid != pid {
        held.pid = pid;
        held.clock = 0;
        held.historical = Counters::ZERO;
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
        (&asked.cache, "cache"),
        (&asked.record, "recording"),
        (&asked.replay, "recording"),
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

/// Every value converted and run through its setter on a fresh builder.
fn request_bytes(value: Option<i64>) -> Result<Option<usize>, RowError> {
    value
        .map(|value| {
            usize::try_from(value)
                .map_err(|_| RowError::usage("max_request_bytes is a whole number of at least 1"))
        })
        .transpose()
}

fn checked(asked: &Asked) -> Result<(Key, EngineBuilder), RowError> {
    let refused = RowError::from;
    let mut builder = complete_builder(
        asked,
        EngineBuilder::from_env().map_err(refused)?.shared_host(),
    )?;
    if let Some(backend) = &asked.backend {
        builder = builder.backend(backend).map_err(refused)?;
    }
    let throttle = asked
        .throttle
        .map(|value| {
            u8::try_from(value)
                .map_err(|_| RowError::usage("a throttle is a whole number from 1 through 32"))
        })
        .transpose()?;
    if let Some(value) = throttle {
        builder = builder.throttle(value).map_err(refused)?;
    }
    let most = asked
        .max_requests
        .map(|value| {
            usize::try_from(value)
                .map_err(|_| RowError::usage("a request limit is a whole number of 1 or more"))
        })
        .transpose()?;
    builder = builder.max_requests(most).map_err(refused)?;
    let bytes = request_bytes(asked.max_request_bytes)?;
    if let Some(bytes) = bytes {
        builder = builder.max_request_bytes(bytes).map_err(refused)?;
    }
    if let Some(folder) = &asked.cache {
        builder = if folder == "off" {
            builder.no_cache()
        } else {
            builder.cache_at(folder).map_err(refused)?
        };
    }
    if let Some(model) = &asked.model {
        builder = builder.model(model).map_err(refused)?;
    }
    let timeout = asked
        .timeout
        .map(|value| {
            u64::try_from(value)
                .ok()
                .filter(|value| *value > 0)
                .ok_or_else(|| RowError::usage("a timeout is a whole number of seconds above zero"))
        })
        .transpose()?;
    if let Some(seconds) = timeout {
        builder = builder
            .timeout(Duration::from_secs(seconds))
            .map_err(refused)?;
    }
    let retries = asked
        .max_retries
        .map(|value| {
            u32::try_from(value)
                .map_err(|_| RowError::usage("a retry count is a whole number of 0 or more"))
        })
        .transpose()?;
    if let Some(retries) = retries {
        builder = builder.max_retries(retries);
    }
    if let Some(total) = asked.max_requests_total {
        total_of(total)
            .map_err(|_| RowError::usage("a request total is a whole number of 0 or more"))?;
    }
    Ok((
        (
            asked.backend.clone(),
            throttle,
            most,
            bytes,
            asked.cache.clone(),
            asked.model.clone(),
            timeout,
            retries,
            asked.profile.clone(),
            asked.record.clone(),
            asked.replay.clone(),
            (
                asked.base_url.clone(),
                asked.prices.clone(),
                asked.refresh_cache,
            ),
        ),
        builder,
    ))
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

fn complete_builder(asked: &Asked, mut builder: EngineBuilder) -> Result<EngineBuilder, RowError> {
    let refused = RowError::from;
    builder = builder.refresh_cache(asked.refresh_cache);
    if let Some(url) = &asked.base_url {
        builder = builder.base_url(url).map_err(refused)?;
    }
    if let Some((input, output)) = &asked.prices {
        builder = builder
            .prices_usd_per_million(input, output)
            .map_err(refused)?;
    }
    if let Some(profile) = &asked.profile {
        builder = builder.profile_json(profile).map_err(refused)?;
    }
    if let Some(folder) = &asked.record {
        builder = builder.record(folder).map_err(refused)?;
    }
    if let Some(folder) = &asked.replay {
        builder = builder.replay(folder).map_err(refused)?;
    }
    Ok(builder)
}
