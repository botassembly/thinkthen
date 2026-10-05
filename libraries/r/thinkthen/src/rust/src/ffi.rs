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
use crate::relate::{self};
use crate::usage;

mod engine;
mod settings;
mod values;
use values::{asked, batch_of, completion_of, context_of, required_completion, spec_of, whole_of};

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

/// One R value as one string.
fn text_of(value: &Robj, what: &str) -> Crossed<String> {
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

fn positions_of(value: &Robj, count: usize) -> Crossed<Vec<usize>> {
    if value.rtype() != Rtype::Integers || value.len() != count {
        return Err(usage("observation positions must match the live evidence"));
    }
    (0..count)
        .map(|at| {
            let at =
                isize::try_from(at).map_err(|_| usage("observation positions are too long"))?;
            // SAFETY: at is inside the integer vector R handed this call.
            let held = unsafe { INTEGER_ELT(value.get(), at) };
            usize::try_from(held).map_err(|_| usage("observation positions must be nonnegative"))
        })
        .collect()
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
fn deadline_of(value: &Robj) -> Crossed<Option<f64>> {
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

/// Check a question file and name its kind.
#[extendr]
fn tt_question_check(body: Robj) -> Crossed<String> {
    let asked = calls::question(&text_of(&body, "the question")?)?;
    Ok(match asked {
        thinkthen::LoadedQuestion::Question(held) => format!("{:?}", held.kind()).to_lowercase(),
        thinkthen::LoadedQuestion::Banded(_) => "decide".to_owned(),
    })
}

/// Read a question file under the crate's one 1 MiB cap.
#[extendr]
fn tt_question_file(path: Robj) -> Crossed<String> {
    thinkthen::read_question_file(text_of(&path, "a question file")?).map_err(|reason| {
        let message = match reason {
            thinkthen::QuestionFileError::TooLarge => "the question file is too large",
            _ => "the question file could not be read",
        };
        crate::packed(thinkthen::ErrorKind::Local.name(), false, message)
    })
}

/// Validate keyword settings with the shared core grammar before a send.
#[extendr]
fn tt_settings_check(body: Robj, kind: Robj) -> Crossed<()> {
    settings::check(&body, &kind)
}

#[extendr]
#[expect(
    clippy::too_many_arguments,
    reason = "R's column binding passes its public controls and original positions explicitly"
)]
fn tt_decide_column(
    question: Robj,
    records: Robj,
    positions: Robj,
    deadline: Robj,
    batch: Robj,
    context: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (json, texts) = asked(&question, &records, "the evidence")?;
    let positions = positions_of(&positions, texts.len())?;
    calls::decide(
        &json,
        texts,
        deadline_of(&deadline)?,
        batch_of(&batch)?,
        context_of(&context)?,
        &interrupt_pending,
        completion_of(&completion)?,
        positions,
    )
}

#[extendr]
#[expect(
    clippy::too_many_arguments,
    reason = "R's column binding passes its public controls and original positions explicitly"
)]
fn tt_column(
    question: Robj,
    records: Robj,
    positions: Robj,
    deadline: Robj,
    batch: Robj,
    context: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (json, texts) = asked(&question, &records, "the evidence")?;
    let positions = positions_of(&positions, texts.len())?;
    calls::column(
        &json,
        texts,
        deadline_of(&deadline)?,
        batch_of(&batch)?,
        context_of(&context)?,
        &interrupt_pending,
        completion_of(&completion)?,
        positions,
    )
}

/// Preview the same packed request bodies without reading a key or sending.
#[extendr]
fn tt_plan_column(question: Robj, records: Robj, batch: Robj, context: Robj) -> Crossed<List> {
    let (json, texts) = asked(&question, &records, "the evidence")?;
    crate::plan::preview(&json, texts, batch_of(&batch)?, context_of(&context)?)
}

#[extendr]
fn tt_filter_places(
    question: Robj,
    records: Robj,
    deadline: Robj,
    batch: Robj,
    context: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (json, texts) = asked(&question, &records, "the records")?;
    calls::filter(
        &json,
        texts,
        deadline_of(&deadline)?,
        batch_of(&batch)?,
        context_of(&context)?,
        &interrupt_pending,
        completion_of(&completion)?,
    )
}

#[extendr]
fn tt_rank_all(
    question: Robj,
    records: Robj,
    deadline: Robj,
    batch: Robj,
    context: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (text, texts) = asked(&question, &records, "the records")?;
    calls::rank(
        &text,
        texts,
        deadline_of(&deadline)?,
        batch_of(&batch)?,
        context_of(&context)?,
        &interrupt_pending,
        completion_of(&completion)?,
    )
}

#[extendr]
fn tt_find_one(
    question: Robj,
    units: Robj,
    none: Robj,
    deadline: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (text, texts) = asked(&question, &units, "the units")?;
    // tt_find checks `none` is TRUE or FALSE before it crosses.
    let none = none.as_bool() == Some(true);
    calls::find(
        &text,
        none,
        texts,
        deadline_of(&deadline)?,
        &interrupt_pending,
        completion_of(&completion)?,
    )
}

#[extendr]
fn tt_annotate_file(
    path: Robj,
    records: Robj,
    taken: Robj,
    deadline: Robj,
    batch: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (path, texts) = (
        text_of(&path, "the question set path")?,
        texts_of(&records, "the records")?,
    );
    let (taken, deadline) = (
        texts_of(&taken, "the input's names")?,
        deadline_of(&deadline)?,
    );
    let set = thinkthen::QuestionSet::load(path).map_err(|error| crate::carry(&error))?;
    calls::annotate(
        set,
        texts,
        &taken,
        deadline,
        batch_of(&batch)?,
        &interrupt_pending,
        completion_of(&completion)?,
    )
}

#[extendr]
fn tt_details_one(
    question: Robj,
    evidence: Robj,
    deadline: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (json, text) = (
        text_of(&question, "the question")?,
        text_of(&evidence, "the evidence")?,
    );
    calls::details(
        &json,
        text,
        deadline_of(&deadline)?,
        &interrupt_pending,
        completion_of(&completion)?,
    )
}

#[extendr]
fn tt_usage_counters() -> Crossed<String> {
    calls::counters()
}

#[extendr]
fn tt_interrupt_pending() -> bool {
    interrupt_pending()
}

#[extendr]
fn tt_recognize_column(
    spec: Robj,
    path: bool,
    texts: Robj,
    positions: Robj,
    deadline: Robj,
    completion: Robj,
) -> Crossed<List> {
    let (spec, texts) = (spec_of(&spec, path)?, texts_of(&texts, "the evidence")?);
    let positions = positions_of(&positions, texts.len())?;
    relate::recognize(
        &spec,
        texts,
        positions,
        deadline_of(&deadline)?,
        &interrupt_pending,
        completion_of(&completion)?,
    )
}

#[extendr]
fn tt_relate_frame(
    spec: Robj,
    path: bool,
    names: Robj,
    kinds: Robj,
    deadline: Robj,
    completion: Robj,
) -> Crossed<List> {
    let spec = spec_of(&spec, path)?;
    let (names, kinds) = (
        texts_of(&names, "the name column")?,
        texts_of(&kinds, "the kind column")?,
    );
    relate::relate(
        &spec,
        names.into_iter().zip(kinds).collect(),
        deadline_of(&deadline)?,
        &interrupt_pending,
        completion_of(&completion)?,
    )
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
    ])
}

extendr_module! {
    mod thinkthen;
    fn tt_question_check;
    fn tt_question_file;
    fn tt_settings_check;
    fn tt_decide_column;
    fn tt_plan_column;
    fn tt_column;
    fn tt_filter_places;
    fn tt_rank_all;
    fn tt_find_one;
    fn tt_annotate_file;
    fn tt_details_one;
    fn tt_usage_counters;
    fn tt_interrupt_pending;
    fn tt_recognize_column;
    fn tt_relate_frame;
    fn tt_engine_set;
    fn tt_completion_new;
    fn tt_completion_claim;
    fn tt_completion_read_native;
    fn tt_completion_settle_early;
}
