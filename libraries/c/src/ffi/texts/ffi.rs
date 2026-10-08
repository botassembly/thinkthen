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
/// Read at most 1 MiB of named UTF-8 single-question JSON without sending.
/// Success owns NUL-terminated original JSON in out and its byte length;
/// free it once with thinkthen_free_string. NULL/bad UTF-8 path or output is
/// EUSAGE; unreadable/overlarge/non-UTF-8/malformed files are ELOCAL,
/// non-retryable. Failure leaves both outputs unchanged and discloses neither
/// path nor contents. Grammar and usage examples: DESIGN.md.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_file(
    engine: *const Door,
    path: *const c_char,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
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
/// Preview a closed thinkthen.plan-input/1 object without key/cache reads
/// or sends. Grammar and plan fields: DESIGN.md. Success owns NUL-terminated
/// plan JSON in out and its byte length; free once with thinkthen_free_string.
/// Unknown/repeated fields, bad verb/input/settings, conflicting question
/// settings or NULL pointers return EUSAGE with both outputs unchanged.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_plan_json(
    engine: *const Door,
    plan_json: *const c_char,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
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
