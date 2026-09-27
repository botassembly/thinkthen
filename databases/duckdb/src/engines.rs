//! The engines this process keeps, one per distinct set of session settings
//! (ticket 0110 decision 5).
//!
//! `SET thinkthen_throttle`, `thinkthen_max_requests`, `thinkthen_cache`, and
//! `thinkthen_cache_bytes` reach the engine through `EngineBuilder`'s own
//! setters on `EngineBuilder::from_env()`. Each call checks every value it
//! was given, the request total among them, before it reads the map, so a map hit never skips a check. The
//! map is keyed by the throttle, the request limit, and the cache folder. It
//! keeps no throttle of its own: main's `build` refuses a second, different
//! throttle, and a failed build is never stored.
//!
//! `SET thinkthen_max_requests_total` caps the requests this process sends
//! (Ian's ruling of 2026-09-25). Before each engine call, [`within_total`]
//! sums `requests_sent` over every engine here. A spent total refuses with
//! zero sends. A call with more texts than remain sends the first ones that
//! fit, then refuses with the same sentence. Main's counters start from zero
//! in a forked child, and so does the total.

use std::sync::{Arc, Mutex, PoisonError};

use thinkthen::{Engine, EngineBuilder};

use crate::errors::{RowError, usage};

/// The most engines one process keeps.
const MOST: usize = 16;

/// The four settings as the caller's session holds them; `None` is unset.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Asked {
    pub(crate) throttle: Option<i64>,
    pub(crate) max_requests: Option<i64>,
    pub(crate) cache: Option<String>,
    pub(crate) cache_bytes: Option<i64>,
    pub(crate) max_requests_total: Option<i64>,
}

/// What the caller's own file system says about a folder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Probe {
    Allowed,
    Refused,
}

/// One engine's key: the settings that change what an engine is.
type Key = (Option<u8>, Option<usize>, Option<String>);

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
    historical: [u64; 4],
    kept: Vec<Entry>,
}

static ENGINES: Mutex<Registry> = Mutex::new(Registry {
    pid: 0,
    clock: 0,
    historical: [0; 4],
    kept: Vec::new(),
});

fn registry() -> std::sync::MutexGuard<'static, Registry> {
    let mut held = ENGINES.lock().unwrap_or_else(PoisonError::into_inner);
    let pid = std::process::id();
    if held.pid != pid {
        held.pid = pid;
        held.clock = 0;
        held.historical = [0; 4];
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
        add_counts(&mut self.historical, &old.engine);
        Ok(())
    }
}

fn add_counts(total: &mut [u64; 4], engine: &Engine) {
    let counts = engine.usage();
    let each = [
        counts.requests_sent(),
        counts.cache_answers(),
        counts.input_tokens(),
        counts.output_tokens(),
    ];
    for (sum, value) in total.iter_mut().zip(each) {
        *sum = sum.saturating_add(value);
    }
}

/// The engine for these settings, built on first use. `probe` opens a path
/// through the caller's own file system.
pub(crate) fn engine_for(
    asked: &Asked,
    probe: impl FnOnce(&str) -> Probe,
) -> Result<Arc<Engine>, String> {
    engine_for_typed(asked, probe).map_err(|error| error.text)
}

pub(crate) fn engine_for_typed(
    asked: &Asked,
    probe: impl FnOnce(&str) -> Probe,
) -> Result<Arc<Engine>, RowError> {
    let (key, builder) = checked(asked)?;
    if let Some(folder) = &key.2 {
        if !folder.starts_with('/') || folder.contains("://") {
            return Err(RowError::usage(
                "a cache folder set from SQL is an absolute local path with no scheme",
            ));
        }
        if probe(&format!("{}/.probe", folder.trim_end_matches('/'))) == Probe::Refused {
            return Err(RowError::usage(
                "the cache folder is outside what this database's file settings allow",
            ));
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
    Ok(built)
}

/// The engine the environment alone describes, for the calls that cannot
/// read a session: the warm aggregate.
pub(crate) fn from_env() -> Result<Arc<Engine>, String> {
    engine_for(&Asked::default(), |_| Probe::Allowed)
}

/// Every value converted and run through its setter on a fresh builder.
fn checked(asked: &Asked) -> Result<(Key, EngineBuilder), RowError> {
    let refused = RowError::from;
    let mut builder = EngineBuilder::from_env().map_err(refused)?;
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
    if let Some(folder) = &asked.cache {
        builder = builder.cache_at(folder).map_err(refused)?;
    }
    if let Some(cap) = asked.cache_bytes {
        let cap = u64::try_from(cap)
            .map_err(|_| RowError::usage("a cache cap is a whole number of bytes above zero"))?;
        builder = builder.cache_bytes(cap).map_err(refused)?;
    }
    if let Some(total) = asked.max_requests_total {
        total_of(total)
            .map_err(|_| RowError::usage("a request total is a whole number of 0 or more"))?;
    }
    Ok(((throttle, most, asked.cache.clone()), builder))
}

fn total_of(total: i64) -> Result<u64, String> {
    u64::try_from(total).map_err(|_| usage("a request total is a whole number of 0 or more"))
}

/// The texts a call may send, and the refusal it raises after them.
type Allowed = (Vec<String>, Option<String>);
type TypedAllowed = (Vec<String>, Option<RowError>);

/// The texts one call may send under the process's request total, and the
/// refusal the call raises after it sends them when the total cut it short.
/// A spent total refuses before anything is sent.
pub(crate) fn within_total(asked: &Asked, texts: Vec<String>) -> Result<Allowed, String> {
    within_total_typed(asked, texts)
        .map(|(texts, cut)| (texts, cut.map(|error| error.text)))
        .map_err(|error| error.text)
}

pub(crate) fn within_total_typed(
    asked: &Asked,
    mut texts: Vec<String>,
) -> Result<TypedAllowed, RowError> {
    let Some(total) = asked.max_requests_total else {
        return Ok((texts, None));
    };
    let total = total_of(total)
        .map_err(|_| RowError::usage("a request total is a whole number of 0 or more"))?;
    let [(_, spent), ..] = usage_totals();
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
    let mut totals = engines.historical;
    for entry in &engines.kept {
        add_counts(&mut totals, &entry.engine);
    }
    let [requests, cache, input, output] = totals;
    [
        ("requests_sent", requests),
        ("cache_answers", cache),
        ("input_tokens", input),
        ("output_tokens", output),
    ]
}
