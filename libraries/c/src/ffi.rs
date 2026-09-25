//! Every exported symbol of `include/thinkthen.h`, and the only `unsafe` in
//! this crate. Each symbol reads the host's pointers under the header's
//! argument rules, hands borrowed Rust values to [`crate::door`] or
//! [`crate::call`], writes the out parameters only on success, and runs
//! behind [`guard`], so no panic unwinds into the host.
#![allow(
    unsafe_code,
    reason = "the C door reads and writes the host's pointers (ADR 0047 item 3)"
)]

use std::ffi::{CStr, CString, c_char};

use thinkthen::{CancelToken, Engine};

use crate::failures::{self, DEFECT, Failure, Held, NO_MESSAGE, OK, USAGE, guard};
use crate::{Door, Judgment, door};

/// The engine behind a host's pointer, or `None` for null.
///
/// # Safety
///
/// `engine` is null or a live engine that outlives `'a`.
unsafe fn held<'a>(engine: *const Door) -> Option<&'a Held> {
    // SAFETY: the caller passes null or a live engine.
    unsafe { engine.as_ref() }.map(|door| &door.0)
}

/// The header's `THINKTHEN_NO_DEADLINE`.
const NO_DEADLINE: i64 = -1;

/// A NUL-terminated string the header names, refused when null or not UTF-8.
///
/// # Safety
///
/// `text` is null or a NUL-terminated string that outlives `'a`.
unsafe fn string<'a>(text: *const c_char, what: &str) -> Result<&'a str, Failure> {
    if text.is_null() {
        return Err(Failure::usage(format!("a null {what}")));
    }
    // SAFETY: `text` is not null, and the caller promises the terminator.
    unsafe { CStr::from_ptr(text) }
        .to_str()
        .map_err(|_| Failure::usage(format!("the {what} is not UTF-8")))
}

/// Exactly `len` bytes of text. Null with zero is the empty text.
///
/// # Safety
///
/// `text` is null or points at `len` readable bytes that outlive `'a`.
unsafe fn text<'a>(text: *const c_char, len: usize) -> Result<&'a str, Failure> {
    if text.is_null() {
        return if len == 0 {
            Ok("")
        } else {
            Err(Failure::usage("a null text with a nonzero length"))
        };
    }
    // SAFETY: `text` is not null, and the caller promises `len` bytes.
    let bytes = unsafe { std::slice::from_raw_parts(text.cast::<u8>(), len) };
    std::str::from_utf8(bytes).map_err(|_| Failure::usage("a text is not UTF-8"))
}

/// `count` texts from the host's two arrays.
///
/// # Safety
///
/// Each array is null or holds `count` entries, and each text follows
/// [`text`]'s rule with its length.
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
    // SAFETY: both arrays are not null, and the caller promises `count` entries.
    let (texts, lengths) = unsafe {
        (
            std::slice::from_raw_parts(texts, count),
            std::slice::from_raw_parts(lengths, count),
        )
    };
    texts
        .iter()
        .zip(lengths)
        // SAFETY: each entry follows `text`'s rule, as the caller promises.
        .map(|(one, len)| unsafe { text(*one, *len) })
        .collect()
}

/// Hand one JSON string to the host through `out` and `out_len`.
///
/// # Safety
///
/// `out` and `out_len` are writable, as [`outs`] checked for null.
unsafe fn hand_over(held: &Held, json: String, out: *mut *mut c_char, out_len: *mut usize) -> i32 {
    match CString::new(json) {
        Ok(json) => {
            // SAFETY: the caller promises both pointers are writable.
            unsafe {
                *out_len = json.as_bytes().len();
                *out = json.into_raw();
            }
            OK
        }
        Err(_) => held.fail(Failure::defect("the answer held a NUL")),
    }
}

/// The shape every typed `_opts` door shares: a null engine is the usage
/// code, the body runs behind the guard, a failure is recorded for the
/// calling thread, and only success reaches `write`.
///
/// # Safety
///
/// `engine` is null or a live engine.
unsafe fn typed<T>(
    engine: *const Door,
    ask: impl FnOnce(&Held) -> Result<T, Failure>,
    write: impl FnOnce(&Held, T) -> i32,
) -> i32 {
    // SAFETY: the caller passes null or a live engine.
    let Some(held) = (unsafe { held(engine) }) else {
        return USAGE;
    };
    guard(Some(held), DEFECT, || match held.settle(ask(held)) {
        Ok(value) => write(held, value),
        Err(code) => code,
    })
}

/// A plain spelling: its `_opts` twin with no budget and no token. The
/// arguments before `;` come before the budget, and the rest after the token.
macro_rules! plain {
    ($name:ident => $opts:ident($($arg:ident: $ty:ty),*; $($out:ident: $out_ty:ty),*) -> $ret:ty) => {
        #[doc = concat!("[`", stringify!($opts), "`] with no budget and no token.")]
        ///
        /// # Safety
        ///
        #[doc = concat!("As [`", stringify!($opts), "`].")]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name($($arg: $ty,)* $($out: $out_ty),*) -> $ret {
            // SAFETY: the caller's pointers pass through unchanged.
            unsafe { $opts($($arg,)* NO_DEADLINE, std::ptr::null_mut(), $($out),*) }
        }
    };
}

/// Build an engine from the environment; null when it cannot be built, and
/// the error functions then answer with a null engine for the failure.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_engine_new() -> *mut Door {
    guard(None, std::ptr::null_mut(), || {
        failures::built(Engine::from_env()).map_or(std::ptr::null_mut(), |engine| {
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
            // SAFETY: `thinkthen_engine_new` boxed it, and no call uses it.
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
    // SAFETY: the caller passes null or a live engine.
    let held = unsafe { held(engine) };
    guard(held, NO_MESSAGE.as_ptr(), || failures::message(held))
}

/// 1 when the calling thread's last failure here could pass later.
///
/// # Safety
///
/// `engine` is null or a live engine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_retryable(engine: *const Door) -> i32 {
    // SAFETY: the caller passes null or a live engine.
    let held = unsafe { held(engine) };
    guard(held, 0, || failures::retryable(held))
}

/// The calling thread's last code here; with a null engine, its last failed
/// build's code, else the usage code.
///
/// # Safety
///
/// `engine` is null or a live engine.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_code(engine: *const Door) -> i32 {
    // SAFETY: the caller passes null or a live engine.
    let held = unsafe { held(engine) };
    guard(held, DEFECT, || failures::code(held))
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
        // SAFETY: the caller passes null or a live token.
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
            // SAFETY: `thinkthen_cancel_token_new` boxed it, and no call carries it.
            drop(unsafe { Box::from_raw(token) });
        }
    });
}

plain!(thinkthen_decide => thinkthen_decide_opts(
    engine: *const Door, question_json: *const c_char, text: *const c_char, text_len: usize;
    out: *mut Judgment) -> i32);

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
    // SAFETY: the header's argument rules make each pointer null or valid
    // for what it names, and `out` is written only after its null check.
    unsafe {
        typed(
            engine,
            |held| {
                if out.is_null() {
                    return Err(Failure::usage("a null out pointer"));
                }
                let options = door::options(deadline_ms, cancel.as_ref())?;
                let question = door::question(string(question_json, "question")?)?;
                let evidence = self::text(text, text_len)?;
                door::decide(&held.engine, &question, evidence, options)
            },
            |_, judgment| {
                *out = judgment;
                OK
            },
        )
    }
}

plain!(thinkthen_decide_many => thinkthen_decide_many_opts(
    engine: *const Door, question_json: *const c_char, texts: *const *const c_char,
    lengths: *const usize, count: usize; out: *mut Judgment) -> i32);

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
    // SAFETY: the header's argument rules make each pointer null or valid
    // for what it names. `out` holds `count` rows, and a nonzero count
    // passed its null check; `decide_many` returns exactly `count` rows.
    unsafe {
        typed(
            engine,
            |held| {
                if count > 0 && out.is_null() {
                    return Err(Failure::usage("a null out array with a nonzero count"));
                }
                let records = self::texts(texts, lengths, count)?;
                let options = door::options(deadline_ms, cancel.as_ref())?;
                let question = door::question(string(question_json, "question")?)?;
                door::decide_many(&held.engine, &question, &records, options)
            },
            |_, rows| {
                for (place, row) in rows.into_iter().enumerate() {
                    *out.add(place) = row;
                }
                OK
            },
        )
    }
}

plain!(thinkthen_call => thinkthen_call_opts(
    engine: *const Door, request_json: *const c_char;) -> *mut c_char);

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
    // SAFETY: the caller passes null or a live engine.
    let Some(held) = (unsafe { held(engine) }) else {
        return std::ptr::null_mut();
    };
    guard(Some(held), std::ptr::null_mut(), || {
        // SAFETY: the request is null or NUL-terminated, and the token is
        // null or live, as the header's argument rules say.
        let asked = unsafe { string(request_json, "request") }.and_then(|request| {
            crate::call::call(&held.engine, request, deadline_ms, unsafe {
                cancel.as_ref()
            })
        });
        match held.settle(asked.and_then(|json| {
            CString::new(json).map_err(|_| Failure::defect("the answer held a NUL"))
        })) {
            Ok(json) => json.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    })
}

plain!(thinkthen_recognize => thinkthen_recognize_opts(
    engine: *const Door, spec_json: *const c_char, text: *const c_char, text_len: usize;
    out: *mut *mut c_char, out_len: *mut usize) -> i32);

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
    // SAFETY: the header's argument rules make each pointer null or valid
    // for what it names, and the outs are written only after their null check.
    unsafe {
        typed(
            engine,
            |held| {
                door::outs(out, out_len)?;
                let options = door::options(deadline_ms, cancel.as_ref())?;
                let spec = string(spec_json, "spec")?;
                let evidence = self::text(text, text_len)?;
                door::recognize(&held.engine, spec, evidence, options)
            },
            |held, json| hand_over(held, json, out, out_len),
        )
    }
}

plain!(thinkthen_relate => thinkthen_relate_opts(
    engine: *const Door, spec_json: *const c_char, texts: *const *const c_char,
    lengths: *const usize, count: usize; out: *mut *mut c_char, out_len: *mut usize) -> i32);

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
    // SAFETY: the header's argument rules make each pointer null or valid
    // for what it names, and the outs are written only after their null check.
    unsafe {
        typed(
            engine,
            |held| {
                door::capped(count)?;
                door::outs(out, out_len)?;
                let options = door::options(deadline_ms, cancel.as_ref())?;
                let spec = string(spec_json, "spec")?;
                let entities = door::entities(&self::texts(texts, lengths, count)?)?;
                door::relate(&held.engine, spec, entities, options)
            },
            |held, json| hand_over(held, json, out, out_len),
        )
    }
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
            // SAFETY: the door made it with `CString::into_raw`, and it is freed once.
            drop(unsafe { CString::from_raw(text) });
        }
    });
}
