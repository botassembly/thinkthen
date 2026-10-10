//! The worker, the wait, and each verb's plain-data answer as R values.
//!
//! Every call runs on a fresh worker thread that owns its inputs, its
//! engine clone, and a clone of the call's cancel token. The main thread
//! waits on a channel in 100 ms ticks and runs R's interrupt check at each
//! one. A pending interrupt cancels the token and returns the marker at
//! once, so a request already on the wire never holds the caller. The
//! detached worker then sends nothing new and finishes what it sent.

use crate::{defect, engine};

pub(crate) mod account;
#[cfg(test)]
mod diagnostics;
pub(crate) mod receipt;
pub(crate) mod render;
mod worker;

#[cfg(test)]
pub(crate) use worker::on_worker;
pub(crate) use worker::{Crossed, call_owned};

/// The counters of the engine in use, as JSON.
pub(crate) fn counters() -> Crossed<String> {
    serde_json::to_string(&engine()?.usage())
        .map_err(|_| defect("the counters could not be written as JSON"))
}
