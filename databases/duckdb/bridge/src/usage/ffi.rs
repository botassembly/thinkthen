//! A private fixed-width snapshot of the DuckDB process counters, and the
//! exit hook that flushes the usage totals.
#![allow(
    unsafe_code,
    reason = "the fixed C ABI export and the exit hook are owned by this FFI module"
)]

use std::sync::atomic::{AtomicBool, Ordering};

use crate::engines;
use crate::ffi::{Reply, reply_boundary};

/// Copy the four retained counters in their documented order.
#[unsafe(no_mangle)]
pub(crate) extern "C" fn thinkthen_cpp_usage() -> Reply {
    reply_boundary(|| {
        Ok(engines::usage_totals()
            .into_iter()
            .flat_map(|(_, count)| i64::try_from(count).unwrap_or(i64::MAX).to_ne_bytes())
            .collect())
    })
}

/// Serialize only the live persistence observation and fixed safe advice.
#[unsafe(no_mangle)]
pub(crate) extern "C" fn thinkthen_cpp_usage_status() -> Reply {
    reply_boundary(|| {
        let state = engines::usage_status();
        let value = match state.advice() {
            Some(advice) => serde_json::json!({"state": state, "advice": advice}),
            None => serde_json::json!({"state": state}),
        };
        serde_json::to_vec(&value)
            .map_err(|_| crate::errors::usage("cannot serialize usage status"))
    })
}

/// Flush the kept engines' usage totals at exit (ADR 0113), registered once
/// when the first engine is kept.
pub(crate) fn flush_usage_at_exit() {
    static REGISTERED: AtomicBool = AtomicBool::new(false);
    if REGISTERED.swap(true, Ordering::AcqRel) {
        return;
    }
    // SAFETY: the hook is a plain function of this library, which the C
    // library runs at exit or when it unloads the library.
    let _registered = unsafe { libc::atexit(flush_usage) };
}

/// The exit hook. A panic stays inside `thinkthen::contained`.
extern "C" fn flush_usage() {
    let _flushed = thinkthen::contained(engines::finish_usage);
}
