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

#[path = "ffi/runtime_version/ffi.rs"]
mod runtime_version;
pub use runtime_version::thinkthen_version;

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
use crate::failures::{DEFECT, Failure, Held, NO_MESSAGE, guard};

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

/// Build from the environment without sending. Failure returns NULL and an
/// independent owned error; success writes NULL to error. NULL error discards
/// diagnostics. Free returned errors once with thinkthen_call_error_free.
/// No thread-local diagnostic slot changes.
/// # Safety
/// error is NULL or aligned writable pointer storage for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_new(
    error: *mut *mut run_call::CallErrorHandle,
) -> *mut EngineHandle {
    // SAFETY: the constructor accepts null settings and the same error storage.
    unsafe { thinkthen_engine_new_with(std::ptr::null(), error) }
}

/// Publish an engine or the existing independent failure owner without TLS.
/// # Safety
/// error is NULL or aligned writable pointer storage separate from build inputs.
unsafe fn owned_constructor(
    error: *mut *mut run_call::CallErrorHandle,
    build: impl FnOnce() -> Result<thinkthen::Engine, Failure>,
) -> *mut EngineHandle {
    let outcome = crate::failures::owned_call(|| {
        build().map(|engine| Box::into_raw(Box::new(EngineHandle(Held::new(engine)))))
    });
    let (engine, failure) = match outcome {
        Ok(engine) => (engine, None),
        Err(failure) => (std::ptr::null_mut(), Some(failure)),
    };
    if !error.is_null() {
        // SAFETY: the caller retains writable storage; each error transfers once.
        unsafe {
            *error = failure.map_or(std::ptr::null_mut(), |failure| {
                Box::into_raw(Box::new(run_call::CallErrorHandle::from(failure)))
            });
        }
    }
    engine
}

unsafe fn settings_json(settings_json: *const c_char) -> Result<thinkthen::Engine, Failure> {
    let text = if settings_json.is_null() {
        "{}"
    } else {
        // SAFETY: the host retains its NUL-terminated settings throughout admission.
        unsafe { string(settings_json, "settings JSON") }?
    };
    crate::settings::build(text)
}

// SAFETY: Last::of owns immutable native backing, CString bytes and a boxed root.
// It stores only copied carrier records; no caller pointer or thread-affine value survives.
unsafe impl Send for crate::failures::Last {}

/// Blocking named calls with independent owned results and failures.
#[path = "ffi/run_call/ffi.rs"]
pub mod run_call;
