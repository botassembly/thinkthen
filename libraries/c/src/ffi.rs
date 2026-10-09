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

/// Canonical counted descriptors and borrowed carrier layouts.
pub mod carriers;

/// Canonical values emitted into the C header.
#[path = "ffi/values.rs"]
pub mod values;

/// Counted constructors; native complete execution integration stays private.
#[path = "ffi/current/ffi.rs"]
pub mod current;
#[path = "ffi/texts/ffi.rs"]
mod texts;
#[path = "ffi/typed_facts/ffi.rs"]
mod typed_facts;

#[path = "ffi/request_preview/ffi.rs"]
mod request_preview;
#[path = "ffi/session/ffi.rs"]
pub mod session;

use thinkthen::CancelToken;

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
const NO_DEADLINE: i64 = values::THINKTHEN_NO_DEADLINE;

fn extent(count: usize, element: usize) -> Result<(), Failure> {
    if count
        .checked_mul(element)
        .is_none_or(|size| size > isize::MAX as usize)
    {
        Err(Failure::usage("an array is too large"))
    } else {
        Ok(())
    }
}

fn text_extent(len: usize) -> Result<(), Failure> {
    if len > isize::MAX as usize {
        Err(Failure::usage("a text is too large"))
    } else {
        Ok(())
    }
}

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
    text_extent(len)?;
    // SAFETY: nonnull with addressable length; the caller promises `len` bytes.
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
    extent(count, std::mem::size_of::<*const c_char>())?;
    extent(count, std::mem::size_of::<usize>())?;
    // SAFETY: nonnull with addressable extents; the caller promises `count` entries.
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

/// Build an engine from the environment; null when it cannot be built, and
/// the error functions then answer with a null engine for the failure.
/// Build from the command's environment, including address/key/cache and
/// XDG configuration/cache defaults. Building sends nothing. Invalid settings
/// return NULL/EUSAGE; unreadable cache/configuration returns NULL/ELOCAL.
/// The NULL-engine error accessors retain the calling thread's failed build.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_engine_new() -> *mut Door {
    // SAFETY: the constructor accepts null as the empty settings object.
    unsafe { thinkthen_engine_new_with(std::ptr::null()) }
}

/// Build with a UTF-8 JSON settings object; null uses the environment.
///
/// # Safety
///
/// `settings_json` is null or a live NUL-terminated string.
/// Build from the environment plus a closed UTF-8 settings object; NULL/{}
/// uses environment alone. Keys/types and token accounting: DESIGN.md.
/// No key is accepted. Unknown/repeated keys or bad types return NULL/EUSAGE
/// in the calling thread's null-engine slot. A named backend captures its key;
/// an explicit base_url receives that key. Building sends nothing.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_new_with(settings_json: *const c_char) -> *mut Door {
    guard(None, std::ptr::null_mut(), || {
        let build = || {
            let text = if settings_json.is_null() {
                "{}"
            } else {
                // SAFETY: the host promises a live NUL-terminated string.
                unsafe { string(settings_json, "settings JSON") }?
            };
            crate::settings::build(text)
        };
        failures::built(build).map_or(std::ptr::null_mut(), |engine| {
            Box::into_raw(Box::new(Door(Held::new(engine))))
        })
    })
}

/// Free an engine; null is ignored.
///
/// # Safety
///
/// `engine` is null or a live engine no other call is using.
/// Free an engine. NULL is accepted and ignored. Free it only after every
/// call on it has returned.
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
/// The message for the last failure the calling thread recorded on this
/// engine. Its borrowed pointer stays valid until that thread records its
/// next failure on this engine, the engine is freed, or the thread exits.
/// Another thread's calls never replace it. Never free the pointer.
/// A deadline's message names the limit and its value. Never NULL: before
/// any failure it names that nothing failed yet. With a null engine it is
/// the calling thread's last failed build's message in a distinct slot, valid
/// until that thread's next thinkthen_engine_new or thinkthen_engine_new_with
/// call, or its exit. Otherwise it names that no engine came.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_message(engine: *const Door) -> *const c_char {
    // SAFETY: the caller passes null or a live engine.
    let held = unsafe { held(engine) };
    guard(held, NO_MESSAGE.as_ptr(), || failures::message(held))
}

/// Borrow the last failure's call facts on this thread and engine, or null.
///
/// # Safety
///
/// `engine` is null or a live engine.
/// Borrow the calling thread's last failed call's facts on this engine, or NULL
/// when no failure with started-call facts exists. The pointer has the same
/// lifetime as thinkthen_error_message and must not be freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_facts_json(engine: *const Door) -> *const c_char {
    // SAFETY: the caller passes null or a live engine.
    let held = unsafe { held(engine) };
    guard(held, std::ptr::null(), || failures::facts(held))
}

/// 1 when the calling thread's last failure here could pass later.
///
/// # Safety
///
/// `engine` is null or a live engine.
/// Whether the same call could pass later: 1 for a backend status the
/// engine retries, such as busy or failing; 0 for a transport failure,
/// which may already have reached the backend, for a refused key, and for
/// every kind but the backend kind. Zero when nothing failed. With a null
/// engine it follows the calling thread's last failed build, else zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_retryable(engine: *const Door) -> std::ffi::c_int {
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
/// The code of the calling thread's last failure on this engine: the value
/// the failing call returned, THINKTHEN_OK when nothing failed yet. Success
/// does not clear it, so read it when a call fails. With a null engine it
/// is the calling thread's last failed build's code, else THINKTHEN_EUSAGE,
/// because no engine holds a failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_code(engine: *const Door) -> std::ffi::c_int {
    // SAFETY: the caller passes null or a live engine.
    let held = unsafe { held(engine) };
    guard(held, DEFECT, || failures::code(held))
}

/// Create a cancel token.
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
/// Fire a token: the calls carrying it start no new request or retry, let
/// the requests they sent finish, and return THINKTHEN_ECANCELLED with no
/// results, even when a sent request's reply arrives after the fire. A
/// token is one-shot: a fire leaves it fired, a second fire is ignored,
/// and no call re-arms it. Thread-safe from any thread, and it allocates
/// nothing; a null token is accepted and ignored.
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
/// Free a token. NULL is accepted and ignored.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel_token_free(token: *mut CancelToken) {
    guard(None, (), || {
        if !token.is_null() {
            // SAFETY: `thinkthen_cancel_token_new` boxed it, and no call carries it.
            drop(unsafe { Box::from_raw(token) });
        }
    });
}

/// Call `thinkthen_decide_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
/// Ask one yes-or-no question of one text: exactly `thinkthen_decide_opts`
/// with THINKTHEN_NO_DEADLINE and a null token. `question_json` is one
/// decide question in the question-file grammar, or the bare text of a
/// decide question at the cut of one half; `text` and `text_len` are the
/// evidence. The judgment lands in `out` on THINKTHEN_OK.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide(
    engine: *const Door,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    out: *mut Judgment,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_decide_opts(
            engine,
            question_json,
            text,
            text_len,
            NO_DEADLINE,
            std::ptr::null_mut(),
            out,
        )
    }
}

/// One yes-or-no question over one text; the judgment lands in `out`.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
/// The same call with the options beside it: `deadline_ms` is the budget
/// and `cancel` is the token, documented on THINKTHEN_NO_DEADLINE.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_opts(
    engine: *const Door,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    deadline_ms: i64,
    cancel: *mut CancelToken,
    out: *mut Judgment,
) -> std::ffi::c_int {
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

/// Call `thinkthen_decide_many_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
/// Bulk decide keeps judgments in input order at the engine throttle.
/// Equivalent to decide_many_opts with THINKTHEN_NO_DEADLINE and a NULL token.
/// texts and lengths have count pointers/byte lengths; out has count answers.
/// Inputs are borrowed for the call, and failure changes no output.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_many(
    engine: *const Door,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut Judgment,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_decide_many_opts(
            engine,
            question_json,
            texts,
            lengths,
            count,
            NO_DEADLINE,
            std::ptr::null_mut(),
            out,
        )
    }
}

/// One question over every text, in input order; all rows or none.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
/// The same bulk call with the options beside it.
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
) -> std::ffi::c_int {
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

/// Call `thinkthen_call_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
/// The JSON door with no budget and no token: exactly
/// `thinkthen_call_opts` with THINKTHEN_NO_DEADLINE and a null token.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call(
    engine: *const Door,
    request_json: *const c_char,
) -> *mut c_char {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe { thinkthen_call_opts(engine, request_json, NO_DEADLINE, std::ptr::null_mut()) }
}

/// The JSON door: one request, its answer as JSON text, or null.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
/// Read one NUL-terminated UTF-8 JSON request; return owned answer JSON,
/// freed once with thinkthen_free_string. Envelope/result schemas: DESIGN.md.
/// Asking success returns {"value":VALUE,"facts":FACTS}; usage returns direct
/// counters and takes no options. NULL means failure with no partial value;
/// error_code/message/retryable and error_facts_json describe that failure.
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

/// Call `thinkthen_recognize_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
/// Recognize a text using version-one recognize question JSON; follow cache.
/// Equivalent to recognize_opts with THINKTHEN_NO_DEADLINE and a NULL token.
/// Success owns {"entities": [...], "relations": [...]} JSON in out and its
/// byte length in out_len; free with thinkthen_free_string. Entity offsets
/// count code points. Failure returns its kind and leaves outputs unchanged.
/// Question/result fields and default kind: DESIGN.md.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_recognize(
    engine: *const Door,
    spec_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_recognize_opts(
            engine,
            spec_json,
            text,
            text_len,
            NO_DEADLINE,
            std::ptr::null_mut(),
            out,
            out_len,
        )
    }
}

/// Every name in one text and the relations the rules allow, as JSON.
///
/// # Safety
///
/// Every pointer follows the header's argument rules.
/// The same call with the options beside it.
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
) -> std::ffi::c_int {
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
            |held, (json, _facts)| hand_over(held, json, out, out_len),
        )
    }
}

/// Call `thinkthen_relate_opts` without a deadline or cancellation token.
/// # Safety
/// All pointers obey the corresponding options form's contract.
/// Relate count JSON records with counted byte lengths; count>255 is EUSAGE
/// before any pointer read. Equivalent to relate_opts with THINKTHEN_NO_DEADLINE/NULL.
/// Version-one relate JSON supplies rules; records carry name/kind. Other
/// fields pointers are EUSAGE. Success owns {"edges": [...]} JSON in out
/// with byte length in out_len; free with thinkthen_free_string. Failure
/// returns its kind and leaves both outputs unchanged. Schema: DESIGN.md.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_relate(
    engine: *const Door,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
    // SAFETY: caller pointers pass unchanged to the checked options form.
    unsafe {
        thinkthen_relate_opts(
            engine,
            spec_json,
            texts,
            lengths,
            count,
            NO_DEADLINE,
            std::ptr::null_mut(),
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
/// The same call with the options beside it.
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
) -> std::ffi::c_int {
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
            |held, (json, _facts)| hand_over(held, json, out, out_len),
        )
    }
}

/// Free a string the door returned; null is ignored.
///
/// # Safety
///
/// `text` is null or a string the door returned and has not freed.
/// Free a string `thinkthen_call`, `thinkthen_recognize`, or
/// `thinkthen_relate` returned, or their `_opts` twins. NULL is accepted
/// and ignored.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_free_string(text: *mut c_char) {
    guard(None, (), || {
        if !text.is_null() {
            // SAFETY: the door made it with `CString::into_raw`, and it is freed once.
            drop(unsafe { CString::from_raw(text) });
        }
    });
}

#[path = "ffi/complete/ffi.rs"]
mod complete;

#[path = "ffi/batch/ffi.rs"]
mod batch;
