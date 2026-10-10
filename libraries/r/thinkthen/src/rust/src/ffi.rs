//! The one module that touches R: the `extern` block, the interrupt check,
//! the text and deadline crossings, and the functions extendr exports.
//!
//! Each exported function reads R's arguments into safe Rust values here
//! and hands them to `calls` or `relate`, which hold no `unsafe`.
//!
//! R's interrupt check must run on R's main thread, and it may jump out of
//! the frame that calls it. A jump across a Rust frame is undefined
//! behavior, so the check runs inside `R_ToplevelExec`, which catches the
//! jump and reports the pending interrupt instead.
#![allow(
    unsafe_code,
    reason = "the R API is C: the extern block, the checked text and deadline reads, and extendr's generated wrappers"
)]
#![allow(
    missing_docs,
    reason = "extendr's generated wrappers and module functions carry no documentation"
)]

use std::ffi::{CStr, c_char, c_void};
use std::sync::Arc;

use extendr_api::SEXP;
use extendr_api::prelude::*;

use crate::calls::receipt::Receipt;
use crate::calls::{self, Crossed};
use crate::usage;

mod engine;
mod request;
mod values;
use values::{batch_of, completion_of, required_completion, whole_of};

#[allow(
    improper_ctypes,
    reason = "SEXP is R's own opaque pointer, as R's headers declare it"
)]
unsafe extern "C" {
    fn R_CheckUserInterrupt();
    fn R_ToplevelExec(check: extern "C" fn(*mut c_void), data: *mut c_void) -> i32;
    fn Rf_getCharCE(x: SEXP) -> i32;
    fn Rf_translateCharUTF8(x: SEXP) -> *const c_char;
    fn STRING_ELT(x: SEXP, i: isize) -> SEXP;
    fn R_CHAR(x: SEXP) -> *const c_char;
    fn REAL_ELT(x: SEXP, i: isize) -> f64;
    fn INTEGER_ELT(x: SEXP, i: isize) -> i32;
    fn R_IsNA(x: f64) -> i32;
    static R_NaString: SEXP;
    static R_NaInt: i32;
}

/// Whether R holds a pending interrupt, checked without letting R's jump
/// cross a Rust frame.
fn interrupt_pending() -> bool {
    extern "C" fn check(_: *mut c_void) {
        // SAFETY: R_ToplevelExec runs this on the main thread and catches its jump.
        unsafe { R_CheckUserInterrupt() }
    }
    // SAFETY: called on R's main thread, with a callback that takes no data.
    unsafe { R_ToplevelExec(check, std::ptr::null_mut()) == 0 }
}

// R's encoding marks (Rinternals.h `cetype_t`) that this surface translates.
const CE_NATIVE: i32 = 0;
const CE_UTF8: i32 = 1;
const CE_LATIN1: i32 = 2;

/// One CHARSXP as UTF-8 text, or a usage refusal naming the conversion.
///
/// Native-marked bytes that are valid UTF-8 cross as themselves under any
/// locale, and invalid ones are refused. Latin1 and UTF-8 marks translate
/// through R. A bytes mark and invalid bytes are refused by name.
fn charsxp_text(charsxp: SEXP, what: &str) -> Crossed<String> {
    // SAFETY: charsxp is a CHARSXP R handed this call on the main thread.
    let (na, mark) = unsafe { (charsxp == R_NaString, Rf_getCharCE(charsxp)) };
    if na {
        return Err(usage(&format!(
            "{what} carries NA, which this call does not take"
        )));
    }
    let refused = |how: &str| {
        usage(&format!(
            "{what} carries {how}; convert it with enc2utf8() or iconv() first"
        ))
    };
    let bytes = match mark {
        // SAFETY: R_CHAR points at the CHARSXP's own NUL-terminated bytes.
        CE_NATIVE => unsafe { CStr::from_ptr(R_CHAR(charsxp)) }.to_bytes(),
        // SAFETY: R's translator returns NUL-terminated UTF-8 it owns.
        CE_UTF8 | CE_LATIN1 => unsafe { CStr::from_ptr(Rf_translateCharUTF8(charsxp)) }.to_bytes(),
        _ => return Err(refused("a string R cannot translate to UTF-8")),
    };
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|_| refused("bytes that are not valid UTF-8"))
}

pub(crate) fn text_of(value: &Robj, what: &str) -> Crossed<String> {
    match value.rtype() {
        // SAFETY: a length-one character vector's first element is a CHARSXP.
        Rtype::Strings if value.len() == 1 => {
            charsxp_text(unsafe { STRING_ELT(value.get(), 0) }, what)
        }
        _ => Err(usage(&format!("{what} must be one string"))),
    }
}

/// One R character vector as UTF-8 texts.
fn texts_of(value: &Robj, what: &str) -> Crossed<Vec<String>> {
    if value.rtype() != Rtype::Strings {
        return Err(usage(&format!("{what} must be a character vector")));
    }
    (0..value.len())
        .map(|at| {
            let at = isize::try_from(at).map_err(|_| usage(&format!("{what} is too long")))?;
            // SAFETY: at is inside the character vector R handed this call.
            charsxp_text(unsafe { STRING_ELT(value.get(), at) }, what)
        })
        .collect()
}

/// Convert host character values with the one native encoding crossing.
#[extendr]
fn tt_text_utf8(value: Robj) -> Crossed<Vec<String>> {
    texts_of(&value, "a text")
}

/// A number of length one with no class but `AsIs`, or `None` for `NULL`.
fn number_of(value: &Robj, what: &str) -> Crossed<Option<f64>> {
    if value.is_null() {
        return Ok(None);
    }
    let plain = value.len() == 1
        && value
            .class()
            .is_none_or(|mut names| names.next() == Some("AsIs") && names.next().is_none());
    let refused = || usage(&format!("{what} is a whole number or NULL"));
    // SAFETY: the value is a length-one double or integer vector R handed this call.
    let (held, na) = match value.rtype() {
        Rtype::Doubles if plain => unsafe {
            let held = REAL_ELT(value.get(), 0);
            (held, R_IsNA(held) != 0)
        },
        Rtype::Integers if plain => unsafe {
            let held = INTEGER_ELT(value.get(), 0);
            (f64::from(held), held == R_NaInt)
        },
        _ => return Err(refused()),
    };
    if na {
        return Err(usage(&format!("{what} is NA: pass a whole number or NULL")));
    }
    Ok(Some(held))
}

/// The deadline argument is whole milliseconds, `-1`, or `NULL`.
pub(crate) fn deadline_of(value: &Robj) -> Crossed<Option<f64>> {
    number_of(value, "deadline_ms")
}

#[extendr]
fn tt_completion_new() -> ExternalPtr<Arc<Receipt>> {
    ExternalPtr::new(Arc::new(Receipt::new()))
}

#[extendr]
fn tt_completion_claim(value: Robj) -> Crossed<()> {
    if required_completion(&value)?.claim() {
        Ok(())
    } else {
        Err(usage("completion belongs to an earlier call"))
    }
}

#[extendr]
fn tt_completion_read_native(value: Robj) -> Crossed<String> {
    calls::render::receipt(&required_completion(&value)?.read())
}

#[extendr]
fn tt_completion_settle_early(value: Robj, kind: Robj) -> Crossed<()> {
    let kind = text_of(&kind, "completion kind")?;
    if ![
        "usage",
        "backend",
        "local",
        "cancelled",
        "deadline",
        "defect",
        "interrupt",
    ]
    .contains(&kind.as_str())
    {
        return Err(usage("completion kind is invalid"));
    }
    required_completion(&value)?.settle_early(kind);
    Ok(())
}

#[extendr]
fn tt_complete_error_native(error: Robj) -> Crossed<Robj> {
    crate::native_results::failure(&text_of(&error, "complete error")?)
}

#[extendr]
fn tt_usage_counters() -> Crossed<String> {
    calls::counters()
}

#[extendr]
#[expect(
    clippy::too_many_arguments,
    reason = "R's engine constructor passes its public settings through this binding"
)]
fn tt_engine_set(
    base_url: Robj,
    model: Robj,
    throttle: Robj,
    max_requests: Robj,
    max_requests_total: Robj,
    max_request_bytes: Robj,
    cache: Robj,
    timeout: Robj,
    max_retries: Robj,
    record: Robj,
    replay: Robj,
    profile: Robj,
    batch: Robj,
    backend: Robj,
    refresh_cache: Robj,
) -> Crossed<()> {
    engine::configure([
        base_url,
        model,
        throttle,
        max_requests,
        max_requests_total,
        max_request_bytes,
        cache,
        timeout,
        max_retries,
        record,
        replay,
        profile,
        batch,
        backend,
        refresh_cache,
    ])
}

#[extendr]
fn tt_usage_persistence_native() -> Crossed<Robj> {
    engine::status(false)
}

#[extendr]
fn tt_finish_usage_status_native() -> Crossed<Robj> {
    engine::status(true)
}

#[extendr]
fn tt_interrupt_pending() -> bool {
    interrupt_pending()
}

extendr_module! {
    mod thinkthen;
    use request;
    fn tt_text_utf8;
    fn tt_engine_set;
    fn tt_complete_error_native;
    fn tt_usage_counters;
    fn tt_usage_persistence_native;
    fn tt_finish_usage_status_native;
    fn tt_interrupt_pending;
    fn tt_completion_new;
    fn tt_completion_claim;
    fn tt_completion_read_native;
    fn tt_completion_settle_early;
}
