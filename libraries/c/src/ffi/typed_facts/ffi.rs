//! Owned facts beside the typed C results. All pointer reads and writes stay
//! at this guarded FFI edge; a failure publishes neither result nor facts.
#![allow(unsafe_code, reason = "the C door reads and writes host pointers")]

use std::ffi::{CString, c_char};
use std::mem::size_of;

use thinkthen::{CancelToken, Facts};

use crate::failures::{self, DEFECT, Failure, Held, OK, USAGE};
use crate::{Door, Judgment, door};

use super::{extent, held, string, text, text_extent};

fn slots(slots: &[*mut ()]) -> Result<(), Failure> {
    if slots.iter().any(|slot| slot.is_null()) {
        return Err(Failure::usage("a null output pointer"));
    }
    for (index, slot) in slots.iter().enumerate() {
        if slots.iter().take(index).any(|prior| prior == slot) {
            return Err(Failure::usage("output pointers are equal"));
        }
    }
    Ok(())
}

fn facts_string(completed: &Option<Facts>) -> Result<CString, Failure> {
    let facts = completed
        .as_ref()
        .ok_or_else(|| Failure::defect("a completed call held no facts"))?;
    CString::new(failures::facts_json(facts)).map_err(|_| Failure::defect("the facts held a NUL"))
}

unsafe fn records<'a>(
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
) -> Result<Vec<&'a str>, Failure> {
    extent(count, size_of::<*const c_char>())?;
    extent(count, size_of::<usize>())?;
    if count == 0 {
        return Ok(Vec::new());
    }
    if texts.is_null() || lengths.is_null() {
        return Err(Failure::usage("a null input array with a nonzero count"));
    }
    // SAFETY: the caller supplies live, aligned arrays with count entries.
    let texts = unsafe { std::slice::from_raw_parts(texts, count) };
    let lengths = unsafe { std::slice::from_raw_parts(lengths, count) };
    texts
        .iter()
        .zip(lengths)
        .map(|(value, len)| {
            text_extent(*len)?;
            // SAFETY: each text follows the documented input-array contract.
            unsafe { text(*value, *len) }
        })
        .collect()
}

unsafe fn token<'a>(token: *mut CancelToken) -> Option<&'a CancelToken> {
    // SAFETY: a nonnull token stays live for the duration of the call.
    unsafe { token.as_ref() }
}

unsafe fn run<T>(
    engine: *const Door,
    ask: impl FnOnce(&Held, &mut Option<Facts>) -> Result<T, Failure>,
    publish: impl FnOnce((T, CString)) -> i32,
) -> i32 {
    // SAFETY: the caller supplies a null or live engine.
    let Some(held) = (unsafe { held(engine) }) else {
        return USAGE;
    };
    let mut completed = None;
    failures::guard_completed(held, &mut completed, DEFECT, |completed| {
        let answer = ask(held, completed).and_then(|answer| {
            let facts = facts_string(completed)?;
            Ok((answer, facts))
        });
        match held.settle(answer.map_err(|failure| failure.completed(completed.as_ref()))) {
            Ok(answer) => publish(answer),
            Err(code) => code,
        }
    })
}

/// Call `thinkthen_decide_with_facts_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
/// Compatibility typed forms: each successful call owns final facts JSON beside
/// its result. The facts object contains records, requests_sent, cache_answers,
/// seconds, and optional input_tokens, output_tokens, and model. Free each
/// returned JSON string with thinkthen_free_string. The original decide, decide_many, recognize and relate
/// forms remain ABI-compatible bare-result forms; they do not return facts.
/// A nonzero code changes no output slot. A started failure's facts remain
/// available from thinkthen_error_facts_json under its borrowed lifetime.
/// All output slots must be nonnull (except the zero-count answer array) and
/// must not share an address. Counts times pointer, size_t, and answer sizes,
/// and every text length, must fit PTRDIFF_MAX. The caller supplies live,
/// aligned, adequately sized, nonoverlapping input and output storage.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_decide_with_facts(
    engine: *const Door,
    question_json: *const c_char,
    evidence: *const c_char,
    evidence_len: usize,
    out: *mut Judgment,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_decide_with_facts_opts(
            engine,
            question_json,
            evidence,
            evidence_len,
            -1,
            std::ptr::null_mut(),
            out,
            facts_json,
            facts_len,
        )
    }
}

/// Decide and return an owned facts JSON string.
///
/// # Safety
/// Pointers and output storage obey the C header's rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_decide_with_facts_opts(
    engine: *const Door,
    question_json: *const c_char,
    evidence: *const c_char,
    evidence_len: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut Judgment,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: all raw inputs are checked before the first send.
    unsafe {
        run(
            engine,
            |held, completed| {
                slots(&[out.cast(), facts_json.cast(), facts_len.cast()])?;
                text_extent(evidence_len)?;
                let options = door::options(deadline_ms, token(cancel))?;
                let question = door::question(string(question_json, "question")?)?;
                let evidence = text(evidence, evidence_len)?;
                door::decide_with_facts(&held.engine, &question, evidence, options, Some(completed))
            },
            |(answer, facts)| {
                *out = answer;
                *facts_len = facts.as_bytes().len();
                *facts_json = facts.into_raw();
                OK
            },
        )
    }
}

/// Call `thinkthen_decide_many_with_facts_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_decide_many_with_facts(
    engine: *const Door,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut Judgment,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_decide_many_with_facts_opts(
            engine,
            question_json,
            texts,
            lengths,
            count,
            -1,
            std::ptr::null_mut(),
            out,
            facts_json,
            facts_len,
        )
    }
}

/// Decide in input order and return final owned batch facts.
///
/// # Safety
/// Pointers and output storage obey the C header's rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_decide_many_with_facts_opts(
    engine: *const Door,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut Judgment,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: all raw inputs are checked before the first send.
    unsafe {
        run(
            engine,
            |held, completed| {
                extent(count, size_of::<*const c_char>())?;
                extent(count, size_of::<usize>())?;
                extent(count, size_of::<Judgment>())?;
                if count > 0 && out.is_null() {
                    return Err(Failure::usage("a null out array with a nonzero count"));
                }
                let mut outputs = vec![facts_json.cast(), facts_len.cast()];
                if !out.is_null() {
                    outputs.push(out.cast());
                }
                slots(&outputs)?;
                let records = records(texts, lengths, count)?;
                let options = door::options(deadline_ms, token(cancel))?;
                let question = door::question(string(question_json, "question")?)?;
                door::decide_many_with_facts(
                    &held.engine,
                    &question,
                    &records,
                    options,
                    Some(completed),
                )
            },
            |(answers, facts)| {
                for (index, answer) in answers.into_iter().enumerate() {
                    *out.add(index) = answer;
                }
                *facts_len = facts.as_bytes().len();
                *facts_json = facts.into_raw();
                OK
            },
        )
    }
}

/// Call `thinkthen_recognize_with_facts_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_recognize_with_facts(
    engine: *const Door,
    spec_json: *const c_char,
    evidence: *const c_char,
    evidence_len: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_recognize_with_facts_opts(
            engine,
            spec_json,
            evidence,
            evidence_len,
            -1,
            std::ptr::null_mut(),
            out,
            out_len,
            facts_json,
            facts_len,
        )
    }
}

/// Recognize and return owned result and facts JSON strings.
///
/// # Safety
/// Pointers and output storage obey the C header's rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_recognize_with_facts_opts(
    engine: *const Door,
    spec_json: *const c_char,
    evidence: *const c_char,
    evidence_len: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut *mut c_char,
    out_len: *mut usize,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: all raw inputs are checked before the first send.
    unsafe {
        run(
            engine,
            |held, completed| {
                slots(&[
                    out.cast(),
                    out_len.cast(),
                    facts_json.cast(),
                    facts_len.cast(),
                ])?;
                text_extent(evidence_len)?;
                let options = door::options(deadline_ms, token(cancel))?;
                let spec = string(spec_json, "spec")?;
                let evidence = text(evidence, evidence_len)?;
                let json =
                    door::recognize_with_facts(&held.engine, spec, evidence, options, completed)?;
                CString::new(json).map_err(|_| Failure::defect("the answer held a NUL"))
            },
            |(json, facts)| {
                *out_len = json.as_bytes().len();
                *facts_len = facts.as_bytes().len();
                *out = json.into_raw();
                *facts_json = facts.into_raw();
                OK
            },
        )
    }
}

/// Call `thinkthen_relate_with_facts_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_relate_with_facts(
    engine: *const Door,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_relate_with_facts_opts(
            engine,
            spec_json,
            texts,
            lengths,
            count,
            -1,
            std::ptr::null_mut(),
            out,
            out_len,
            facts_json,
            facts_len,
        )
    }
}

/// Relate and return owned result and facts JSON strings.
///
/// # Safety
/// Pointers and output storage obey the C header's rules.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_relate_with_facts_opts(
    engine: *const Door,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut *mut c_char,
    out_len: *mut usize,
    facts_json: *mut *mut c_char,
    facts_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: all raw inputs are checked before the first send.
    unsafe {
        run(
            engine,
            |held, completed| {
                door::capped(count)?;
                extent(count, size_of::<*const c_char>())?;
                extent(count, size_of::<usize>())?;
                slots(&[
                    out.cast(),
                    out_len.cast(),
                    facts_json.cast(),
                    facts_len.cast(),
                ])?;
                let records = records(texts, lengths, count)?;
                let options = door::options(deadline_ms, token(cancel))?;
                let spec = string(spec_json, "spec")?;
                let entities = door::entities(&records)?;
                let json =
                    door::relate_with_facts(&held.engine, spec, entities, options, completed)?;
                CString::new(json).map_err(|_| Failure::defect("the answer held a NUL"))
            },
            |(json, facts)| {
                *out_len = json.as_bytes().len();
                *facts_len = facts.as_bytes().len();
                *out = json.into_raw();
                *facts_json = facts.into_raw();
                OK
            },
        )
    }
}
