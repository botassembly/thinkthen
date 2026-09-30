//! The plan door's export: a no-send preview as owned JSON text.
#![allow(unsafe_code, reason = "the C door reads and writes host pointers")]

use std::ffi::c_char;

use crate::{Door, door};

use super::{hand_over, string, typed};

/// Preview one `thinkthen.plan-input/1` object as `plan` JSON text.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_plan_json(
    engine: *const Door,
    plan_json: *const c_char,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    // SAFETY: checked pointers are read or written only under the header rules.
    unsafe {
        typed(
            engine,
            |held| {
                door::outs(out, out_len)?;
                crate::plan::plan(&held.engine, string(plan_json, "plan input")?)
            },
            |held, json| hand_over(held, json, out, out_len),
        )
    }
}
