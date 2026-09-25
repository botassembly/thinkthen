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

use crate::errors::{failure, usage};

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

/// Each kept engine beside its key.
type Kept = Vec<(Key, Arc<Engine>)>;

static ENGINES: Mutex<Kept> = Mutex::new(Vec::new());

/// The engine for these settings, built on first use. `probe` opens a path
/// through the caller's own file system.
pub(crate) fn engine_for(
    asked: &Asked,
    probe: impl FnOnce(&str) -> Probe,
) -> Result<Arc<Engine>, String> {
    let (key, builder) = checked(asked)?;
    if let Some(folder) = &key.2 {
        if !folder.starts_with('/') || folder.contains("://") {
            return Err(usage(
                "a cache folder set from SQL is an absolute local path with no scheme",
            ));
        }
        if probe(&format!("{}/.probe", folder.trim_end_matches('/'))) == Probe::Refused {
            return Err(usage(
                "the cache folder is outside what this database's file settings allow",
            ));
        }
    }
    let full = || {
        usage(
            "this process already keeps 16 engines, one per throttle, request limit, and cache folder; reuse settings already in use",
        )
    };
    {
        let engines = ENGINES.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((_, engine)) = engines.iter().find(|(held, _)| *held == key) {
            return Ok(Arc::clone(engine));
        }
        if engines.len() >= MOST {
            return Err(full());
        }
    }
    // The build runs outside the lock, so a fork during a build never
    // leaves the child's map locked. A racing build of the same key loses
    // to the one stored first.
    let built = Arc::new(builder.build().map_err(|error| failure(&error))?);
    let mut engines = ENGINES.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some((_, engine)) = engines.iter().find(|(held, _)| *held == key) {
        return Ok(Arc::clone(engine));
    }
    if engines.len() >= MOST {
        return Err(full());
    }
    engines.push((key, Arc::clone(&built)));
    Ok(built)
}

/// The engine the environment alone describes, for the calls that cannot
/// read a session: the warm aggregate.
pub(crate) fn from_env() -> Result<Arc<Engine>, String> {
    engine_for(&Asked::default(), |_| Probe::Allowed)
}

/// Every value converted and run through its setter on a fresh builder.
fn checked(asked: &Asked) -> Result<(Key, EngineBuilder), String> {
    let refused = |error: thinkthen::Error| failure(&error);
    let mut builder = EngineBuilder::from_env().map_err(refused)?;
    let throttle = asked
        .throttle
        .map(|value| {
            u8::try_from(value).map_err(|_| usage("a throttle is a whole number from 1 through 32"))
        })
        .transpose()?;
    if let Some(value) = throttle {
        builder = builder.throttle(value).map_err(refused)?;
    }
    let most = asked
        .max_requests
        .map(|value| {
            usize::try_from(value)
                .map_err(|_| usage("a request limit is a whole number of 1 or more"))
        })
        .transpose()?;
    builder = builder.max_requests(most).map_err(refused)?;
    if let Some(folder) = &asked.cache {
        builder = builder.cache_at(folder).map_err(refused)?;
    }
    if let Some(cap) = asked.cache_bytes {
        let cap = u64::try_from(cap)
            .map_err(|_| usage("a cache cap is a whole number of bytes above zero"))?;
        builder = builder.cache_bytes(cap).map_err(refused)?;
    }
    if let Some(total) = asked.max_requests_total {
        total_of(total)?;
    }
    Ok(((throttle, most, asked.cache.clone()), builder))
}

fn total_of(total: i64) -> Result<u64, String> {
    u64::try_from(total).map_err(|_| usage("a request total is a whole number of 0 or more"))
}

/// The texts a call may send, and the refusal it raises after them.
type Allowed = (Vec<String>, Option<String>);

/// The texts one call may send under the process's request total, and the
/// refusal the call raises after it sends them when the total cut it short.
/// A spent total refuses before anything is sent.
pub(crate) fn within_total(asked: &Asked, mut texts: Vec<String>) -> Result<Allowed, String> {
    let Some(total) = asked.max_requests_total else {
        return Ok((texts, None));
    };
    let total = total_of(total)?;
    let [(_, spent), ..] = usage_totals();
    let spent_out = || {
        usage(&format!(
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

/// The counters of every engine this process keeps, summed.
pub(crate) fn usage_totals() -> [(&'static str, u64); 4] {
    let mut totals = [0_u64; 4];
    for (_, engine) in ENGINES
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .iter()
    {
        let counts = engine.usage();
        let each = [
            counts.requests_sent(),
            counts.cache_answers(),
            counts.input_tokens(),
            counts.output_tokens(),
        ];
        for (total, count) in totals.iter_mut().zip(each) {
            *total = total.saturating_add(count);
        }
    }
    let [requests, cache, input, output] = totals;
    [
        ("requests_sent", requests),
        ("cache_answers", cache),
        ("input_tokens", input),
        ("output_tokens", output),
    ]
}
