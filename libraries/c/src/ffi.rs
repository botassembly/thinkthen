//! Every exported symbol of `include/thinkthen.h`, and the only `unsafe` in
//! this crate. Each symbol reads the host's pointers under the header's
//! argument rules, hands borrowed Rust values to [`crate::door`] or
//! [`crate::call`], writes the out parameters only on success, and runs
//! behind [`guard`], so no panic unwinds into the host.
#![allow(
    unsafe_code,
    reason = "the C door reads and writes the host's pointers (ADR 0047 item 3)"
)]
#![expect(
    clippy::too_many_arguments,
    reason = "the header freezes each `_opts` signature"
)]

use std::ffi::{CStr, CString, c_char};

use thinkthen::{CancelToken, Engine};

use crate::door::{self, MOST_RELATED};
use crate::failures::{DEFECT, Failure, Held, NO_ENGINE, NO_MESSAGE, OK, USAGE, guard};

/// The header's `thinkthen_answer`: the outcome code, then the probability
/// of yes. Two fixed fields, never a third.
#[repr(C)]
#[derive(Debug)]
pub struct Judgment {
    /// `THINKTHEN_YES`, `THINKTHEN_NO`, or `THINKTHEN_UNSURE`.
    pub outcome: i32,
    /// The probability the backend gave the yes side.
    pub probability: f64,
}

/// The header's opaque `thinkthen_engine`: one engine and its failure table.
#[derive(Debug)]
pub struct Door(Held);

/// The engine behind a host's pointer, or `None` for null.
unsafe fn held<'a>(engine: *const Door) -> Option<&'a Held> {
    unsafe { engine.as_ref() }.map(|door| &door.0)
}

/// The header's `THINKTHEN_NO_DEADLINE`.
const NO_DEADLINE: i64 = -1;

/// A NUL-terminated string the header names, refused when null or not UTF-8.
unsafe fn string<'a>(text: *const c_char, what: &str) -> Result<&'a str, Failure> {
    if text.is_null() {
        return Err(Failure::usage(format!("a null {what}")));
    }
    unsafe { CStr::from_ptr(text) }
        .to_str()
        .map_err(|_| Failure::usage(format!("the {what} is not UTF-8")))
}

/// Exactly `len` bytes of text. Null with zero is the empty text.
unsafe fn text<'a>(text: *const c_char, len: usize) -> Result<&'a str, Failure> {
    if text.is_null() {
        return if len == 0 {
            Ok("")
        } else {
            Err(Failure::usage("a null text with a nonzero length"))
        };
    }
    let bytes = unsafe { std::slice::from_raw_parts(text.cast::<u8>(), len) };
    std::str::from_utf8(bytes).map_err(|_| Failure::usage("a text is not UTF-8"))
}

/// `count` texts from the host's two arrays.
unsafe fn texts<'a>(
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
) -> Result<Vec<&'a str>, Failure> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if texts.is_null() {
        return Err(Failure::usage("a null texts array with a nonzero count"));
    }
    if lengths.is_null() {
        return Err(Failure::usage("a null lengths array with a nonzero count"));
    }
    let (texts, lengths) = unsafe {
        (
            std::slice::from_raw_parts(texts, count),
            std::slice::from_raw_parts(lengths, count),
        )
    };
    texts
        .iter()
        .zip(lengths)
        .map(|(one, len)| unsafe { text(*one, *len) })
        .collect()
}

/// Hand one JSON string to the host through `out` and `out_len`.
unsafe fn hand_over(held: &Held, json: String, out: *mut *mut c_char, out_len: *mut usize) -> i32 {
    match CString::new(json) {
        Ok(json) => {
            unsafe {
                *out_len = json.as_bytes().len();
                *out = json.into_raw();
            }
            OK
        }
        Err(_) => held.fail(Failure::defect("the answer held a NUL")),
    }
}

/// Build an engine from the environment; null when the settings are invalid.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_engine_new() -> *mut Door {
    guard(None, std::ptr::null_mut(), || {
        Engine::from_env().map_or(std::ptr::null_mut(), |engine| {
            Box::into_raw(Box::new(Door(Held::new(engine))))
        })
    })
}

/// Free an engine; null is ignored.
///
/// # Safety
///
/// `engine` is null or a live engine no other call is using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_free(engine: *mut Door) {
    guard(None, (), || {
        if !engine.is_null() {
            drop(unsafe { Box::from_raw(engine) });
        }
    });
}

/// The calling thread's last message on this engine; never null.
///
/// # Safety
///
/// `engine` is null or a live engine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_message(engine: *const Door) -> *const c_char {
    let held = unsafe { held(engine) };
    guard(held, NO_MESSAGE.as_ptr(), || {
        held.map_or(NO_ENGINE.as_ptr(), Held::message)
    })
}

/// 1 when the calling thread's last failure here could pass later.
///
/// # Safety
///
/// `engine` is null or a live engine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_retryable(engine: *const Door) -> i32 {
    let held = unsafe { held(engine) };
    guard(held, 0, || held.map_or(0, Held::retryable))
}

/// The calling thread's last code here; the usage code with a null engine.
///
/// # Safety
///
/// `engine` is null or a live engine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_code(engine: *const Door) -> i32 {
    let held = unsafe { held(engine) };
    guard(held, DEFECT, || held.map_or(USAGE, Held::code))
}

/// Create a cancel token.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_cancel_token_new() -> *mut CancelToken {
    guard(None, std::ptr::null_mut(), || {
        Box::into_raw(Box::new(CancelToken::new()))
    })
}

/// Fire a token from any thread; null is ignored.
///
/// # Safety
///
/// `token` is null or a live token.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel(token: *mut CancelToken) {
    guard(None, (), || {
        if let Some(token) = unsafe { token.as_ref() } {
            token.cancel();
        }
    });
}

/// Free a token; null is ignored.
///
/// # Safety
///
/// `token` is null or a live token no call is carrying.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel_token_free(token: *mut CancelToken) {
    guard(None, (), || {
        if !token.is_null() {
            drop(unsafe { Box::from_raw(token) });
        }
    });
}

/// [`thinkthen_decide_opts`] with no budget and no token.
///
/// # Safety
///
/// As [`thinkthen_decide_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide(
    engine: *const Door,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    out: *mut Judgment,
) -> i32 {
    let null = std::ptr::null_mut();
    unsafe { thinkthen_decide_opts(engine, question_json, text, text_len, NO_DEADLINE, null, out) }
}

/// One yes-or-no question over one text; the judgment lands in `out`.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_opts(
    engine: *const Door,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut Judgment,
) -> i32 {
    let Some(held) = (unsafe { held(engine) }) else {
        return USAGE;
    };
    guard(Some(held), DEFECT, || {
        let asked = (|| {
            if out.is_null() {
                return Err(Failure::usage("a null out pointer"));
            }
            let options = door::options(deadline_ms, unsafe { cancel.as_ref() })?;
            let question = door::question(unsafe { string(question_json, "question") }?)?;
            let evidence = unsafe { self::text(text, text_len) }?;
            door::decide(&held.engine, &question, evidence, options)
        })();
        match held.settle(asked) {
            Ok((outcome, probability)) => {
                unsafe { *out = Judgment { outcome, probability } };
                OK
            }
            Err(code) => code,
        }
    })
}

/// [`thinkthen_decide_many_opts`] with no budget and no token.
///
/// # Safety
///
/// As [`thinkthen_decide_many_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_many(
    engine: *const Door,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut Judgment,
) -> i32 {
    let null = std::ptr::null_mut();
    unsafe {
        thinkthen_decide_many_opts(
            engine,
            question_json,
            texts,
            lengths,
            count,
            NO_DEADLINE,
            null,
            out,
        )
    }
}

/// One question over every text, in input order; all rows or none.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_many_opts(
    engine: *const Door,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut Judgment,
) -> i32 {
    let Some(held) = (unsafe { held(engine) }) else {
        return USAGE;
    };
    guard(Some(held), DEFECT, || {
        let asked = (|| {
            if count > 0 && out.is_null() {
                return Err(Failure::usage("a null out array with a nonzero count"));
            }
            let records = unsafe { self::texts(texts, lengths, count) }?;
            let options = door::options(deadline_ms, unsafe { cancel.as_ref() })?;
            let question = door::question(unsafe { string(question_json, "question") }?)?;
            door::decide_many(&held.engine, &question, &records, options)
        })();
        match held.settle(asked) {
            Ok(rows) => {
                for (place, (outcome, probability)) in rows.into_iter().enumerate() {
                    unsafe { *out.add(place) = Judgment { outcome, probability } };
                }
                OK
            }
            Err(code) => code,
        }
    })
}

/// [`thinkthen_call_opts`] with no budget and no token.
///
/// # Safety
///
/// As [`thinkthen_call_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call(
    engine: *const Door,
    request_json: *const c_char,
) -> *mut c_char {
    unsafe { thinkthen_call_opts(engine, request_json, NO_DEADLINE, std::ptr::null_mut()) }
}

/// The JSON door: one request, its answer as JSON text, or null.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_opts(
    engine: *const Door,
    request_json: *const c_char,
    deadline_ms: i64,
    cancel: *mut CancelToken,
) -> *mut c_char {
    let Some(held) = (unsafe { held(engine) }) else {
        return std::ptr::null_mut();
    };
    guard(Some(held), std::ptr::null_mut(), || {
        let asked = unsafe { string(request_json, "request") }.and_then(|request| {
            crate::call::call(&held.engine, request, deadline_ms, unsafe { cancel.as_ref() })
        });
        match held.settle(asked.and_then(|json| {
            CString::new(json).map_err(|_| Failure::defect("the answer held a NUL"))
        })) {
            Ok(json) => json.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    })
}

/// [`thinkthen_recognize_opts`] with no budget and no token.
///
/// # Safety
///
/// As [`thinkthen_recognize_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_recognize(
    engine: *const Door,
    spec_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    let null = std::ptr::null_mut();
    unsafe {
        thinkthen_recognize_opts(engine, spec_json, text, text_len, NO_DEADLINE, null, out, out_len)
    }
}

/// Every name in one text and the relations the rules allow, as JSON.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_recognize_opts(
    engine: *const Door,
    spec_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    let Some(held) = (unsafe { held(engine) }) else {
        return USAGE;
    };
    guard(Some(held), DEFECT, || {
        let asked = (|| {
            outs(out, out_len)?;
            let options = door::options(deadline_ms, unsafe { cancel.as_ref() })?;
            let spec = unsafe { string(spec_json, "spec") }?;
            let evidence = unsafe { self::text(text, text_len) }?;
            door::recognize(&held.engine, spec, evidence, options)
        })();
        match held.settle(asked) {
            Ok(json) => unsafe { hand_over(held, json, out, out_len) },
            Err(code) => code,
        }
    })
}

/// [`thinkthen_relate_opts`] with no budget and no token.
///
/// # Safety
///
/// As [`thinkthen_relate_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_relate(
    engine: *const Door,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    let null = std::ptr::null_mut();
    unsafe {
        thinkthen_relate_opts(
            engine,
            spec_json,
            texts,
            lengths,
            count,
            NO_DEADLINE,
            null,
            out,
            out_len,
        )
    }
}

/// How the given records relate, as `{"edges": [...]}`.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_relate_opts(
    engine: *const Door,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    let Some(held) = (unsafe { held(engine) }) else {
        return USAGE;
    };
    guard(Some(held), DEFECT, || {
        let asked = (|| {
            if count > MOST_RELATED {
                return Err(Failure::usage("relate takes at most 255 records"));
            }
            outs(out, out_len)?;
            let options = door::options(deadline_ms, unsafe { cancel.as_ref() })?;
            let spec = unsafe { string(spec_json, "spec") }?;
            let records = unsafe { self::texts(texts, lengths, count) }?;
            let entities = records.into_iter().map(door::entity).collect::<Result<_, _>>()?;
            door::relate(&held.engine, spec, entities, options)
        })();
        match held.settle(asked) {
            Ok(json) => unsafe { hand_over(held, json, out, out_len) },
            Err(code) => code,
        }
    })
}

/// Refuse a null `out` or `out_len`, which success always writes.
fn outs(out: *mut *mut c_char, out_len: *mut usize) -> Result<(), Failure> {
    if out.is_null() {
        return Err(Failure::usage("a null out pointer"));
    }
    if out_len.is_null() {
        return Err(Failure::usage("a null out_len pointer"));
    }
    Ok(())
}

/// Free a string the door returned; null is ignored.
///
/// # Safety
///
/// `text` is null or a string the door returned and has not freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_free_string(text: *mut c_char) {
    guard(None, (), || {
        if !text.is_null() {
            drop(unsafe { CString::from_raw(text) });
        }
    });
}
