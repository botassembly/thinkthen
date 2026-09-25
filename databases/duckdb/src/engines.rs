//! The engines this process keeps, one per distinct set of session settings
//! (ticket 0110 decision 5).
//!
//! `SET thinkthen_throttle`, `thinkthen_max_requests`, `thinkthen_cache`, and
//! `thinkthen_cache_bytes` reach the engine through `EngineBuilder`'s own
//! setters on `EngineBuilder::from_env()`. Each call checks every value it
//! was given before it reads the map, so a map hit never skips a check. The
//! map is keyed by the throttle, the request limit, and the cache folder. It
//! keeps no throttle of its own: main's `build` refuses a second, different
//! throttle, and a failed build is never stored.
//!
//! `SET thinkthen_max_requests_total` caps the requests this process sends
//! (Ian's ruling of 2026-09-25). Before each engine call, [`for_call`] sums
//! `requests_sent` over every engine here. A spent total refuses with zero
//! sends. Otherwise the call runs on its own engine whose request limit is
//! the smaller of the remaining total and the session's own limit. Main's
//! counters start from zero in a forked child, and so does the total.

use std::sync::{Arc, Mutex, PoisonError};

use thinkthen::{Counters, Engine, EngineBuilder};

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
    let mut engines = ENGINES.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some((_, engine)) = engines.iter().find(|(held, _)| *held == key) {
        return Ok(Arc::clone(engine));
    }
    if engines.len() >= MOST {
        return Err(usage(
            "this process already keeps 16 engines, one per throttle, request limit, and cache folder; reuse settings already in use",
        ));
    }
    let engine = Arc::new(builder.build().map_err(|error| failure(&error))?);
    engines.push((key, Arc::clone(&engine)));
    Ok(engine)
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
    Ok(((throttle, most, asked.cache.clone()), builder))
}

/// The engines built for one capped call each, and the counters of those
/// already dropped, tagged with the process that counted them.
#[derive(Debug, Default)]
struct Capped {
    pid: u32,
    live: Vec<Arc<Engine>>,
    folded: [u64; 4],
}

static CAPPED: Mutex<Option<Capped>> = Mutex::new(None);

/// The engine one call runs on. With no total set, it is the caller's kept
/// engine. With a total set, it is a new engine limited to what is left.
pub(crate) fn for_call(kept: &Arc<Engine>, asked: &Asked) -> Result<Arc<Engine>, String> {
    let Some(total) = asked.max_requests_total else {
        return Ok(Arc::clone(kept));
    };
    let total = u64::try_from(total)
        .map_err(|_| usage("a request total is a whole number of 0 or more"))?;
    let [(_, spent), ..] = usage_totals();
    let left = total.saturating_sub(spent);
    if left == 0 {
        return Err(usage(&format!(
            "this process has spent its request total of {total}; raise SET thinkthen_max_requests_total or RESET it"
        )));
    }
    let limit = usize::try_from(left).unwrap_or(usize::MAX);
    let own = asked
        .max_requests
        .and_then(|most| usize::try_from(most).ok())
        .map_or(limit, |most| most.min(limit));
    let (_, builder) = checked(&Asked {
        max_requests: i64::try_from(own).ok(),
        ..asked.clone()
    })?;
    let engine = Arc::new(builder.build().map_err(|error| failure(&error))?);
    capped(|held| held.live.push(Arc::clone(&engine)));
    Ok(engine)
}

/// Run `work` on this process's capped engines, after folding the counters
/// of each engine no call holds any longer.
fn capped<T>(work: impl FnOnce(&mut Capped) -> T) -> T {
    let mut held = CAPPED.lock().unwrap_or_else(PoisonError::into_inner);
    let pid = std::process::id();
    let current = held.get_or_insert_with(Capped::default);
    if current.pid != pid {
        *current = Capped {
            pid,
            ..Capped::default()
        };
    }
    let (done, live): (Vec<_>, Vec<_>) = std::mem::take(&mut current.live)
        .into_iter()
        .partition(|engine| Arc::strong_count(engine) == 1);
    current.live = live;
    for engine in done {
        add(&mut current.folded, &engine.usage());
    }
    work(current)
}

fn add(totals: &mut [u64; 4], counts: &Counters) {
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

/// The counters of every engine this process keeps or kept, summed.
pub(crate) fn usage_totals() -> [(&'static str, u64); 4] {
    let mut totals = capped(|held| {
        let mut totals = held.folded;
        for engine in &held.live {
            add(&mut totals, &engine.usage());
        }
        totals
    });
    for (_, engine) in ENGINES
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .iter()
    {
        add(&mut totals, &engine.usage());
    }
    let [requests, cache, input, output] = totals;
    [
        ("requests_sent", requests),
        ("cache_answers", cache),
        ("input_tokens", input),
        ("output_tokens", output),
    ]
}
