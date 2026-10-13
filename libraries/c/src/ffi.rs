//! Every exported symbol of `include/thinkthen.h`, and the only `unsafe` in
//! this crate. Each symbol reads the host's pointers under the header's
//! argument rules, writes the out parameters only on success, and runs
//! behind [`guard`], so no panic unwinds into the host.
#![allow(
    unsafe_code,
    reason = "the C interface reads and writes the host's pointers (ADR 0047 item 3)"
)]

#[path = "ffi/operations_generated.rs"]
mod operations;
pub use operations::*;
use std::ffi::{CStr, c_char};

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
