//! The exports that send nothing and hand back owned JSON text: a
//! validated question file and a plan preview.
#![allow(unsafe_code, reason = "the C door reads and writes host pointers")]

use std::ffi::c_char;

use crate::failures::Failure;
use crate::{Door, door};

use super::{hand_over, string, typed};

/// Read one named question into an owned validated JSON string.
///
/// # Safety
///
/// Every pointer follows the header's argument and lifetime rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_file(
    engine: *const Door,
    path: *const c_char,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    // SAFETY: checked pointers are read or written only under the header rules.
    unsafe {
        typed(
            engine,
            |_| {
                door::outs(out, out_len)?;
                let path = string(path, "question file path")?;
                if path.is_empty() {
                    return Err(Failure::usage("an empty question file path"));
                }
                door::question_file(path)
            },
            |held, json| hand_over(held, json, out, out_len),
        )
    }
}

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
