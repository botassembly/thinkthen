//! Generated native operation transport. Do not edit.
#![allow(
    unsafe_code,
    reason = "foreign slots follow the operation ownership contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures;
use thinkthen::ErrorKind;
/// Borrow the immutable last engine/thread failure, or constructor failure for NULL.
/// NULL means that slot holds no failure. Getters and successful engine calls preserve
/// the view, including code, safe message, retryability and typed optional facts.
/// Root and every nested value remain stable until this thread next fails on this
/// engine, engine_free or thread exit. A NULL-engine view ends at the next constructor.
/// Other threads cannot replace or move this thread slot.
/// # Safety
/// engine is NULL or live for the call and every borrowed engine view.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_view(
    engine: *const crate::EngineHandle,
) -> *const crate::session::views::thinkthen_error_view_v1 {
    // SAFETY: the caller retains its nullable engine owner.
    let held = unsafe { super::held(engine) };
    super::guard(held, std::ptr::null(), || failures::view(held))
}

/// Borrow the immutable canonical value root and nested backing until value_free.
/// Repeated and concurrent getters preserve identical pointers and prior diagnostics.
/// NULL owner or output returns EUSAGE without publishing; outputs remain untouched.
/// # Safety
/// owner remains live for every borrower; out is aligned writable pointer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_view(
    owner: *const crate::session::views::ValueHandle,
    out: *mut *const crate::session::views::thinkthen_complete_json_v1,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let owner = unsafe { owner.as_ref() }.ok_or(ErrorKind::Usage)?;
        let value_out = super::request_preview::value_view(owner)?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Parse one arbitrary JSON value through the existing Rust parser.
/// Numbers retain their original decimal bytes, including large integers and exponents;
/// object keys, null and false are preserved without request-shape admission.
/// Success transfers an immutable independent owner; free once with value_free.
/// Input bytes may be freed on return. Malformed JSON, UTF-8 or extents return EUSAGE,
/// preserve out and occupy only the calling thread immediate session diagnostic.
/// # Safety
/// json has len readable UTF-8 bytes; NULL requires zero, which is invalid JSON.
/// Extents fit Rust slices; out is nonnull aligned writable pointer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_parse(
    json: *const std::ffi::c_char,
    len: usize,
    out: *mut *mut crate::session::views::ValueHandle,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_out = unsafe { super::request_preview::parse_value(json, len) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Preview a full canonical Request through existing Rust admission and Plan serialization.
/// Sends nothing and reads no key or cache. Unknown fields and unsupported previews refuse.
/// Success transfers an immutable independent owner; free once with value_free.
/// Source bytes and engine may be freed on return. Failures preserve out and occupy
/// only the calling thread immediate session diagnostic, independently of engine/constructor slots.
/// # Safety
/// engine is live; request_json has request_len readable UTF-8 bytes. NULL requires
/// zero, which is invalid Request JSON. Extents fit Rust slices; out is nonnull
/// aligned writable pointer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_request_plan_value(
    engine: *const crate::EngineHandle,
    request_json: *const std::ffi::c_char,
    request_len: usize,
    out: *mut *mut crate::session::views::ValueHandle,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_out = unsafe {
            super::request_preview::json_plan(
                engine,
                request_json,
                request_len,
                super::request_preview::value_output,
            )
        }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Free an independent canonical value owner; NULL is ignored.
/// # Safety
/// owner is NULL or unfreed, after every operation and borrowed view finishes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_value_free(owner: *mut crate::session::views::ValueHandle) {
    super::guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: the owner came from this allocator and all borrowers have finished.
            drop(unsafe { Box::from_raw(owner) });
        }
    })
}

/// Preview named canonical components through existing Rust admission, without
/// reading a key/cache, constructing a session or sending. Fixed atomic questions
/// match the existing Plan serializer; unsupported functions/questions refuse.
/// Success transfers an independent owned canonical value; free with value_free.
/// Source bytes and engine may be freed on return. Failure leaves out untouched
/// and records only the calling thread immediate session diagnostic.
/// # Safety
/// engine is live; request is an aligned live record with readable counted UTF-8.
/// out is nonnull aligned writable pointer storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_request_plan_named_value(
    engine: *const crate::EngineHandle,
    request: *const crate::ffi::session::NamedRequestV1,
    out: *mut *mut crate::session::views::ValueHandle,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_out = unsafe { super::request_preview::named_plan(engine, request) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Build from environment plus closed UTF-8 JSON settings. NULL uses the environment.
/// Construction sends nothing; failures return NULL and occupy the constructor slot.
/// # Safety
/// settings_json is NULL or live NUL-terminated UTF-8 for the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_new_with(
    settings_json: *const std::ffi::c_char,
) -> *mut crate::EngineHandle {
    failures::guard(None, std::ptr::null_mut(), || {
        // SAFETY: the caller retains all constructor input until admission returns.
        failures::built(|| unsafe { super::settings_json(settings_json) })
            .map_or(std::ptr::null_mut(), |engine| {
                Box::into_raw(Box::new(crate::EngineHandle(failures::Held::new(engine))))
            })
    })
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
pub unsafe extern "C" fn thinkthen_error_message(
    engine: *const crate::EngineHandle,
) -> *const std::ffi::c_char {
    // SAFETY: the caller retains its nullable engine owner.
    let held = unsafe { super::held(engine) };
    super::guard(held, super::NO_MESSAGE.as_ptr(), || failures::message(held))
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
pub unsafe extern "C" fn thinkthen_error_facts_json(
    engine: *const crate::EngineHandle,
) -> *const std::ffi::c_char {
    // SAFETY: the caller retains its nullable engine owner.
    let held = unsafe { super::held(engine) };
    super::guard(held, std::ptr::null(), || failures::facts(held))
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
pub unsafe extern "C" fn thinkthen_error_retryable(
    engine: *const crate::EngineHandle,
) -> std::ffi::c_int {
    // SAFETY: the caller retains its nullable engine owner.
    let held = unsafe { super::held(engine) };
    super::guard(held, 0, || failures::retryable(held))
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
pub unsafe extern "C" fn thinkthen_error_code(
    engine: *const crate::EngineHandle,
) -> std::ffi::c_int {
    // SAFETY: the caller retains its nullable engine owner.
    let held = unsafe { super::held(engine) };
    super::guard(held, super::DEFECT, || failures::code(held))
}

/// Preview canonical Request JSON without reading a key or cache or sending.
/// Only fixed atomic questions support previews; dynamic questions and other functions refuse.
/// Success owns NUL-terminated JSON; free once with thinkthen_free_string.
/// Errors preserve output slots and use the calling thread session diagnostic.
/// # Safety
/// Engine is live; request_json has request_len readable bytes; NULL requires zero.
/// Output slots are nonnull writable and nonoverlapping. Extents fit Rust slices.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_request_plan_json(
    engine: *const crate::EngineHandle,
    request_json: *const std::ffi::c_char,
    request_len: usize,
    out: *mut *mut std::ffi::c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out_len.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out.cast::<()>() == out_len.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let (value_out, value_out_len) = unsafe {
            super::request_preview::json_plan(
                engine,
                request_json,
                request_len,
                super::request_preview::json_output,
            )
        }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
            *out_len = value_out_len;
        }
        Ok(())
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
pub unsafe extern "C" fn thinkthen_engine_free(engine: *mut crate::EngineHandle) {
    super::guard(None, (), || {
        if !engine.is_null() {
            // SAFETY: the owner came from this allocator and all borrowers have finished.
            drop(unsafe { Box::from_raw(engine) });
        }
    })
}

/// Free a string the interface returned; null is ignored.
///
/// # Safety
///
/// `text` is null or a string the interface returned and has not freed.
/// Free the owned JSON returned by thinkthen_request_plan_json. NULL is accepted.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_free_string(text: *mut std::ffi::c_char) {
    super::guard(None, (), || {
        if !text.is_null() {
            // SAFETY: the owner came from this allocator and all borrowers have finished.
            drop(unsafe { std::ffi::CString::from_raw(text) });
        }
    })
}
