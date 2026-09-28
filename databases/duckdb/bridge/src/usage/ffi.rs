//! A private fixed-width snapshot of the DuckDB process counters.
#![allow(
    unsafe_code,
    reason = "the fixed C ABI export is owned by this FFI module"
)]

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
