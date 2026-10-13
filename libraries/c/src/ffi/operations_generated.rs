//! Generated native operation transport. Do not edit.
#![allow(
    unsafe_code,
    reason = "foreign slots follow the operation ownership contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures;
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
