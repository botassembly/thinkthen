//! Every exported symbol of `include/thinkthen.h`, and the only `unsafe` in
//! this crate. Each symbol reads the host's pointers under the header's
//! argument rules, writes the out parameters only on success, and runs
//! behind [`guard`], so no panic unwinds into the host.
#![allow(
    unsafe_code,
    reason = "the C interface reads and writes the host's pointers (ADR 0047 item 3)"
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

#[path = "ffi/request_preview/ffi.rs"]
mod request_preview;
#[path = "ffi/session/ffi.rs"]
pub mod session;

/// Counted typed request construction and feed operations.
#[path = "ffi/native_request/ffi.rs"]
pub mod native_request;

use crate::EngineHandle;
use crate::failures::{self, DEFECT, Failure, Held, NO_MESSAGE, USAGE, guard};

/// The engine behind a host's pointer, or `None` for null.
///
/// # Safety
///
/// `engine` is null or a live engine that outlives `'a`.
unsafe fn held<'a>(engine: *const EngineHandle) -> Option<&'a Held> {
    // SAFETY: the caller passes null or a live engine.
    unsafe { engine.as_ref() }.map(|interface| &interface.0)
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

/// The shape every typed `_opts` interface shares: a null engine is the usage
/// code, the body runs behind the guard, a failure is recorded for the
/// calling thread, and only success reaches `write`.
///
/// # Safety
///
/// `engine` is null or a live engine.
unsafe fn run_typed<T>(
    engine: *const EngineHandle,
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
pub extern "C" fn thinkthen_engine_new() -> *mut EngineHandle {
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
pub unsafe extern "C" fn thinkthen_engine_new_with(
    settings_json: *const c_char,
) -> *mut EngineHandle {
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
            Box::into_raw(Box::new(EngineHandle(Held::new(engine))))
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
pub unsafe extern "C" fn thinkthen_engine_free(engine: *mut EngineHandle) {
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
pub unsafe extern "C" fn thinkthen_error_message(engine: *const EngineHandle) -> *const c_char {
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
pub unsafe extern "C" fn thinkthen_error_facts_json(engine: *const EngineHandle) -> *const c_char {
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
pub unsafe extern "C" fn thinkthen_error_retryable(engine: *const EngineHandle) -> std::ffi::c_int {
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
pub unsafe extern "C" fn thinkthen_error_code(engine: *const EngineHandle) -> std::ffi::c_int {
    // SAFETY: the caller passes null or a live engine.
    let held = unsafe { held(engine) };
    guard(held, DEFECT, || failures::code(held))
}

/// Free a string the interface returned; null is ignored.
///
/// # Safety
///
/// `text` is null or a string the interface returned and has not freed.
/// Free the owned JSON returned by thinkthen_request_plan_json. NULL is accepted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_free_string(text: *mut c_char) {
    guard(None, (), || {
        if !text.is_null() {
            // SAFETY: the interface made it with `CString::into_raw`, and it is freed once.
            drop(unsafe { CString::from_raw(text) });
        }
    });
}
