//! The process engine and the four settings SQL gives it before it is
//! built: `thinkthen_throttle`, `thinkthen_max_requests`, `thinkthen_cache`,
//! and the process request total `thinkthen_max_requests_total` (decision 17).

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{Engine, EngineBuilder};

use crate::question::shown;
use crate::{Failure, guard};

/// The settings SQL stored, each applied over the environment at the build.
#[derive(Debug, Default)]
struct Stored {
    throttle: Option<u8>,
    max_requests: Option<Option<usize>>,
    cache: Option<Option<PathBuf>>,
    total: Option<u64>,
}

impl Stored {
    fn apply(&self, mut builder: EngineBuilder) -> Result<EngineBuilder, thinkthen::Error> {
        if let Some(value) = self.throttle {
            builder = builder.throttle(value)?;
        }
        if let Some(value) = self.max_requests {
            builder = builder.max_requests(value)?;
        }
        match &self.cache {
            Some(Some(folder)) => builder = builder.cache_at(folder)?,
            Some(None) => builder = builder.no_cache(),
            None => {}
        }
        Ok(builder)
    }
}

static STORED: Mutex<Stored> = Mutex::new(Stored {
    throttle: None,
    max_requests: None,
    cache: None,
    total: None,
});

static ENGINE: OnceLock<Engine> = OnceLock::new();

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

/// What remains of the process request total (decision 17), or `None` with
/// no total. A spent total refuses before any send.
pub(crate) fn remaining() -> Result<Option<usize>, Failure> {
    let Some(total) = stored().total else {
        return Ok(None);
    };
    let sent = built().map_or(0, |engine| engine.usage().requests_sent());
    match total.saturating_sub(sent) {
        0 => Err(spent(total)),
        left => Ok(Some(usize::try_from(left).unwrap_or(usize::MAX))),
    }
}

/// The refusal once the process request total is spent.
fn spent(total: u64) -> Failure {
    Failure::usage(format!(
        "this process has sent its total of {total} requests (thinkthen_max_requests_total)"
    ))
}

/// Check one setting against a fresh builder, then store it.
fn set(check: impl Fn(&mut Stored)) -> Result<(), Failure> {
    let mut held = stored();
    if ENGINE.get().is_some() {
        return Err(Failure::usage(
            "settings apply before the first call; this process already built its engine",
        ));
    }
    let mut next = Stored::default();
    check(&mut next);
    next.apply(EngineBuilder::from_env()?)?;
    check(&mut held);
    Ok(())
}

/// A whole-number argument, or `usage` naming what SQL gave.
fn whole(context: &Context<'_>, name: &str) -> Result<Option<i64>, Failure> {
    match context.get_raw(0) {
        ValueRef::Null => Ok(None),
        ValueRef::Integer(value) => Ok(Some(value)),
        other => Err(Failure::usage(format!(
            "{name} takes a whole number, not {}",
            shown(other)
        ))),
    }
}

/// `thinkthen_throttle(n)`: the most requests in flight, 1 through 32.
pub(crate) fn throttle(context: &Context<'_>) -> rusqlite::Result<i64> {
    Ok(guard("thinkthen_throttle", || {
        let value = whole(context, "thinkthen_throttle")?
            .ok_or_else(|| Failure::usage("thinkthen_throttle takes a whole number, not NULL"))?;
        // Out of `u8` reads as 0, so the engine's own sentence refuses it.
        let throttle = u8::try_from(value).unwrap_or(0);
        set(|held| held.throttle = Some(throttle))?;
        Ok(value)
    })?)
}

/// `thinkthen_max_requests(n)`: the most records one call takes; NULL is no limit.
pub(crate) fn max_requests(context: &Context<'_>) -> rusqlite::Result<Option<i64>> {
    Ok(guard("thinkthen_max_requests", || {
        let value = whole(context, "thinkthen_max_requests")?;
        let most = value.map(|value| usize::try_from(value).unwrap_or(0));
        set(|held| held.max_requests = Some(most))?;
        Ok(value)
    })?)
}

/// `thinkthen_cache(dir)`: cache answers in this folder; NULL turns the cache off.
pub(crate) fn cache(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_cache", || {
        let folder = match context.get_raw(0) {
            ValueRef::Null => None,
            ValueRef::Text(bytes) => Some(
                String::from_utf8(bytes.to_vec())
                    .map_err(|_| Failure::usage("thinkthen_cache takes UTF-8 text"))?,
            ),
            other => {
                return Err(Failure::usage(format!(
                    "thinkthen_cache takes a folder or NULL, not {}",
                    shown(other)
                )));
            }
        };
        set(|held| held.cache = Some(folder.clone().map(PathBuf::from)))?;
        Ok(folder)
    })?)
}

/// `thinkthen_max_requests_total(n)`: the most requests this process may
/// send; NULL is no total.
pub(crate) fn max_requests_total(context: &Context<'_>) -> rusqlite::Result<Option<i64>> {
    Ok(guard("thinkthen_max_requests_total", || {
        let value = whole(context, "thinkthen_max_requests_total")?;
        let total = value
            .map(|value| {
                u64::try_from(value)
                    .ok()
                    .filter(|total| *total > 0)
                    .ok_or_else(|| Failure::usage("a request total is a whole number of 1 or more"))
            })
            .transpose()?;
        set(|held| held.total = total)?;
        Ok(value)
    })?)
}
