//! A monotonic ThinkThen budget owned by one SQLite connection.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, PoisonError, Weak};
use std::time::{Duration, Instant};

use rusqlite::Connection;
use rusqlite::functions::{Context, FunctionFlags};
use thinkthen::ErrorKind;

use crate::{Failure, ffi, guard};

#[derive(Debug, Default)]
struct Budget {
    due: Mutex<Option<Instant>>,
}

type Registry = Mutex<HashMap<usize, Weak<Budget>>>;
static REGISTRY: OnceLock<Registry> = OnceLock::new();

fn registry() -> &'static Registry {
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

fn expired() -> Failure {
    Failure::of(
        ErrorKind::Deadline,
        "the connection's ThinkThen budget passed",
    )
}

/// The remaining whole milliseconds and expiry for a call. A stale handle
/// has no live owner and cannot lend a budget to a new connection.
pub(crate) fn remaining(
    db: *mut rusqlite::ffi::sqlite3,
) -> Result<Option<(i64, Instant)>, Failure> {
    let held = registry().lock().unwrap_or_else(PoisonError::into_inner);
    let owner = held.get(&(db as usize)).and_then(Weak::upgrade);
    drop(held);
    let Some(owner) = owner else { return Ok(None) };
    let due = *owner.due.lock().unwrap_or_else(PoisonError::into_inner);
    let Some(due) = due else { return Ok(None) };
    let left = due
        .checked_duration_since(Instant::now())
        .ok_or_else(expired)?;
    let millis = i64::try_from(left.as_millis()).unwrap_or(i64::MAX);
    if millis == 0 {
        return Err(expired());
    }
    Ok(Some((millis, due)))
}

/// Register the setting with connection-owned closure state. The caller sets
/// it in a separate statement before the work it should bound.
pub(crate) fn register(connection: &Connection) -> rusqlite::Result<()> {
    let owner = Arc::new(Budget::default());
    let flags = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY;
    connection.create_scalar_function("thinkthen_budget_ms", 1, flags, move |context: &Context<'_>| {
        Ok(guard("thinkthen_budget_ms", || {
            let value = match context.get_raw(0) {
                rusqlite::types::ValueRef::Integer(value) if value >= -1 => value,
                _ => return Err(Failure::usage("thinkthen_budget_ms takes -1, 0, or a positive whole number of representable milliseconds")),
            };
            let due = match value {
                -1 => None,
                0 => Some(Instant::now()),
                millis => {
                    thinkthen::CallOptions::new().deadline_millis(millis)?;
                    Some(Instant::now().checked_add(Duration::from_millis(millis as u64))
                        .ok_or_else(|| Failure::usage("the ThinkThen budget is not representable"))?)
                }
            };
            *owner.due.lock().unwrap_or_else(PoisonError::into_inner) = due;
            let db = ffi::handle_of(context) as usize;
            let mut held = registry().lock().unwrap_or_else(PoisonError::into_inner);
            held.retain(|_, weak| weak.strong_count() > 0);
            held.insert(db, Arc::downgrade(&owner));
            Ok(value)
        })?)
    })
}
