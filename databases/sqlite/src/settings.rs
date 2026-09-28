//! The process engine and SQL settings fixed before its first build.

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::Duration;

use rusqlite::functions::Context;
use rusqlite::types::ValueRef;
use thinkthen::{Engine, EngineBuilder, SendBudget};

use crate::question::shown;
use crate::{Failure, guard};

/// The settings SQL stored, each applied over the environment at the build.
#[derive(Debug, Default)]
struct Stored {
    throttle: Option<u8>,
    max_requests: Option<Option<usize>>,
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
        if let Some(value) = self.max_requests {
            builder = builder.max_requests(value)?;
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
    max_requests: None,
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
static SEND_BUDGET: OnceLock<SendBudget> = OnceLock::new();

/// Keep one count across every engine and every call in this process.
pub(crate) fn send_budget() -> (&'static SendBudget, Option<u64>) {
    (SEND_BUDGET.get_or_init(SendBudget::new), stored().total)
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
pub(crate) fn spent(total: u64) -> Failure {
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

fn text(context: &Context<'_>, name: &str) -> Result<Option<String>, Failure> {
    match context.get_raw(0) {
        ValueRef::Null => Ok(None),
        ValueRef::Text(bytes) => String::from_utf8(bytes.to_vec())
            .map(Some)
            .map_err(|_| Failure::usage(format!("{name} takes UTF-8 text"))),
        other => Err(Failure::usage(format!(
            "{name} takes text or NULL, not {}",
            shown(other)
        ))),
    }
}

pub(crate) fn model(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_model", || {
        let value = text(context, "thinkthen_model")?;
        set(|held| held.model = value.clone())?;
        Ok(value)
    })?)
}

pub(crate) fn timeout(context: &Context<'_>) -> rusqlite::Result<Option<i64>> {
    Ok(guard("thinkthen_timeout", || {
        let value = whole(context, "thinkthen_timeout")?;
        let seconds = value
            .map(|value| {
                u64::try_from(value)
                    .ok()
                    .filter(|value| *value > 0)
                    .map(Duration::from_secs)
                    .ok_or_else(|| Failure::usage("a timeout is a whole number of seconds above zero"))
            })
            .transpose()?;
        set(|held| held.timeout = seconds)?;
        Ok(value)
    })?)
}

pub(crate) fn max_retries(context: &Context<'_>) -> rusqlite::Result<Option<i64>> {
    Ok(guard("thinkthen_max_retries", || {
        let value = whole(context, "thinkthen_max_retries")?;
        let retries = value
            .map(|value| {
                u32::try_from(value)
                    .map_err(|_| Failure::usage("a retry count is a whole number from 0 through 4294967295"))
            })
            .transpose()?;
        set(|held| held.max_retries = retries)?;
        Ok(value)
    })?)
}

pub(crate) fn profile(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_profile", || {
        let value = text(context, "thinkthen_profile")?;
        set(|held| held.profile = value.clone())?;
        Ok(value)
    })?)
}

pub(crate) fn record(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_record", || {
        let value = text(context, "thinkthen_record")?;
        set(|held| held.record = value.clone().map(PathBuf::from))?;
        Ok(value)
    })?)
}

pub(crate) fn replay(context: &Context<'_>) -> rusqlite::Result<Option<String>> {
    Ok(guard("thinkthen_replay", || {
        let value = text(context, "thinkthen_replay")?;
        set(|held| held.replay = value.clone().map(PathBuf::from))?;
        Ok(value)
    })?)
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
