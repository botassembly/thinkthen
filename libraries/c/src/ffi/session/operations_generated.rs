//! Generated native operation transport. Do not edit.
#![allow(
    unsafe_code,
    reason = "foreign slots follow the operation ownership contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures;
use thinkthen::ErrorKind;
/// Push transferred one descriptor to the session.
pub const THINKTHEN_SESSION_ACCEPTED_V1: u32 = 0;
/// Push retained nothing; retry the same descriptor.
pub const THINKTHEN_SESSION_FULL_V1: u32 = 1;
/// Intake closed; stop advancing the producer.
pub const THINKTHEN_SESSION_CLOSED_V1: u32 = 2;
/// Read transferred one independent packet owner.
pub const THINKTHEN_SESSION_RESULT_V1: u32 = 0;
/// Work has not settled and no packet is ready.
pub const THINKTHEN_SESSION_PENDING_V1: u32 = 1;
/// Terminal was read and no more output will arrive.
pub const THINKTHEN_SESSION_END_V1: u32 = 2;
/// Wait only for results, end or cancel; input room cannot wake this wait.
pub const THINKTHEN_SESSION_WAIT_RESULT_V1: u32 = 0;
/// Wait for result, input room/closure, end or cancel.
pub const THINKTHEN_SESSION_WAIT_INPUT_V1: u32 = 1;
/// Drain output after cancel without waking repeatedly for cancellation.
pub const THINKTHEN_SESSION_WAIT_SETTLEMENT_V1: u32 = 2;
/// A result or terminal packet is ready; no packet has been consumed.
pub const THINKTHEN_SESSION_READY_RESULT_V1: u32 = 0;
/// Caller-supplied input has room.
pub const THINKTHEN_SESSION_READY_INPUT_V1: u32 = 1;
/// Caller-supplied input has closed.
pub const THINKTHEN_SESSION_READY_CLOSED_V1: u32 = 2;
/// Every output packet has been consumed.
pub const THINKTHEN_SESSION_READY_END_V1: u32 = 3;
/// Intake was cancelled; settlement can still be pending.
pub const THINKTHEN_SESSION_READY_CANCELLED_V1: u32 = 4;
/// The caller-given timeout passed.
pub const THINKTHEN_SESSION_READY_TIMEOUT_V1: u32 = 5;
pub(crate) fn push_status(value: thinkthen::RequestSessionPushStatus) -> u32 {
    match value {
        thinkthen::RequestSessionPushStatus::Accepted => THINKTHEN_SESSION_ACCEPTED_V1,
        thinkthen::RequestSessionPushStatus::Full => THINKTHEN_SESSION_FULL_V1,
        thinkthen::RequestSessionPushStatus::Closed => THINKTHEN_SESSION_CLOSED_V1,
    }
}
pub(crate) fn ready_status(value: thinkthen::RequestSessionReady) -> u32 {
    match value {
        thinkthen::RequestSessionReady::Result => THINKTHEN_SESSION_READY_RESULT_V1,
        thinkthen::RequestSessionReady::Input => THINKTHEN_SESSION_READY_INPUT_V1,
        thinkthen::RequestSessionReady::Closed => THINKTHEN_SESSION_READY_CLOSED_V1,
        thinkthen::RequestSessionReady::End => THINKTHEN_SESSION_READY_END_V1,
        thinkthen::RequestSessionReady::Cancelled => THINKTHEN_SESSION_READY_CANCELLED_V1,
        thinkthen::RequestSessionReady::Timeout => THINKTHEN_SESSION_READY_TIMEOUT_V1,
    }
}
pub(crate) fn failure_code(value: thinkthen::ErrorKind) -> i32 {
    match value {
        thinkthen::ErrorKind::Usage => crate::ffi::values::THINKTHEN_EUSAGE,
        thinkthen::ErrorKind::Backend => crate::ffi::values::THINKTHEN_EBACKEND,
        thinkthen::ErrorKind::Deadline => crate::ffi::values::THINKTHEN_EDEADLINE,
        thinkthen::ErrorKind::Local => crate::ffi::values::THINKTHEN_ELOCAL,
        thinkthen::ErrorKind::Cancelled => crate::ffi::values::THINKTHEN_ECANCELLED,
        thinkthen::ErrorKind::Defect => crate::ffi::values::THINKTHEN_EDEFECT,
    }
}
pub(crate) fn failure_message(value: thinkthen::ErrorKind) -> &'static str {
    match value {
        thinkthen::ErrorKind::Usage => "invalid session arguments or input",
        thinkthen::ErrorKind::Backend => "session backend refused the operation",
        thinkthen::ErrorKind::Deadline => "session deadline expired",
        thinkthen::ErrorKind::Local => "session local operation failed",
        thinkthen::ErrorKind::Cancelled => "session operation cancelled",
        thinkthen::ErrorKind::Defect => "defect: a session operation failed",
    }
}
/// Borrow the calling thread immediate session failure, or NULL before failure.
/// Success, getters, engine calls and terminal packets preserve this independent slot.
/// The root, safe message and every typed optional fact remain valid until the next
/// immediate session failure on this thread or thread exit.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_session_error_view()
-> *const crate::session::views::thinkthen_error_view_v1 {
    failures::session_guard(std::ptr::null(), failures::session_view)
}

/// Block for requested readiness without consuming a packet or joining provider
/// work. timeout_ms is the maximum condition wait in milliseconds; zero checks
/// readiness immediately. RESULT ignores input room; INPUT also watches intake;
/// SETTLEMENT ignores cancel to drain final packets after cancellation. Output
/// readiness takes priority over cancel. Success preserves thread-local errors;
/// failures preserve out_ready and set only the calling thread's diagnostic.
/// # Safety
/// session is live for the whole wait, including concurrent cancel/read/push.
/// Shared operations are synchronized internally. out_ready is writable and
/// nonnull. Free only after every concurrent operation, including wait, returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_wait(
    session: *const crate::session::SessionHandle,
    interest: u32,
    timeout_ms: u32,
    out_ready: *mut u32,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out_ready.is_null() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        let value_out_ready = super::wait_ready(session, interest, timeout_ms)?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out_ready = value_out_ready;
        }
        Ok(())
    })
}

/// Transfer one owned packet without waiting for native work.
/// RESULT transfers an owner; PENDING and END write NULL to out. Validate both
/// output slots before popping. A result survives session_free and engine_free.
/// # Safety
/// session is live. status and out are writable, nonnull, distinct addresses.
/// Alignment, readable/writable ranges and partial overlaps are caller duties.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_try_read(
    session: *mut crate::session::SessionHandle,
    status: *mut u32,
    out: *mut *mut crate::session::SessionResultHandle,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if status.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out.is_null() {
            return Err(ErrorKind::Usage);
        }
        if status.cast::<()>() == out.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        let (value_status, value_out) = super::read_packet(session, None)?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *status = value_status;
            *out = value_out;
        }
        Ok(())
    })
}

/// Block and transfer one packet without polling or joining provider work.
/// interest is WAIT_RESULT or WAIT_SETTLEMENT. RESULT wakes promptly on cancel
/// with ECANCELLED; output already queued wins. SETTLEMENT drains later terminal
/// facts after cancel. RESULT and END match try_read, including packet ownership.
/// Errors preserve both output slots and affect only this thread's session error.
/// # Safety
/// The try_read slot obligations apply. Keep session live throughout the wait;
/// concurrent cancel/push/finish are synchronized. Free after every call returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_next_packet(
    session: *mut crate::session::SessionHandle,
    interest: u32,
    status: *mut u32,
    out: *mut *mut crate::session::SessionResultHandle,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if status.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out.is_null() {
            return Err(ErrorKind::Usage);
        }
        if status.cast::<()>() == out.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        let (value_status, value_out) = super::read_packet(session, Some(interest))?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *status = value_status;
            *out = value_out;
        }
        Ok(())
    })
}

/// Borrow canonical immutable packet JSON, including an owned trailing NUL.
/// The length excludes that terminator. Bytes stay valid until result_free,
/// even after session_free and engine_free. Concurrent access is synchronized.
/// Immediate failure preserves both outputs and records the session diagnostic.
/// # Safety
/// result is live throughout the call. out and out_len are nonnull, writable,
/// distinct addresses; alignment and partial overlaps are caller duties.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_result_json(
    result: *const crate::session::SessionResultHandle,
    out: *mut *const std::ffi::c_char,
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
        let result = unsafe { result.as_ref() }.ok_or(ErrorKind::Usage)?;
        let (value_out, value_out_len) = super::packet_json(result)?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
            *out_len = value_out_len;
        }
        Ok(())
    })
}

/// Borrow the complete immutable typed packet graph. Nested views retain every
/// known field and unknown extension, and survive engine/session destruction.
/// Repeated and concurrent access returns the same view. Only result_free ends
/// all view lifetimes. Failure leaves out unchanged.
/// # Safety
/// result stays live during access and while using any borrowed view; out is
/// nonnull, aligned writable pointer storage. Free only after all borrowers finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_result_view(
    result: *const crate::session::SessionResultHandle,
    out: *mut *const crate::session::views_generated::thinkthen_complete_session_packet_v1,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let result = unsafe { result.as_ref() }.ok_or(ErrorKind::Usage)?;
        let value_out = super::packet_view(result)?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Observe this engine's live usage persistence without waiting for a writer.
/// On success assign both outputs; Failed is an observation, not an operation
/// error. Advice is a static counted UTF-8 view, or NULL with zero length.
/// Success preserves the calling-thread session diagnostic.
/// # Safety
/// engine is live for the call. Both outputs are nonnull, aligned, writable,
/// nonoverlapping slots. Destruction waits for concurrent callers to return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_usage_persistence_v1(
    engine: *const crate::EngineHandle,
    out_state: *mut crate::session::views_generated::thinkthen_complete_usage_persistence_v1,
    out_advice: *mut crate::session::views::thinkthen_complete_utf8_v1,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out_state.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out_advice.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out_state.cast::<()>() == out_advice.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        let (value_out_state, value_out_advice) =
            super::usage_status(engine, thinkthen::Engine::usage_persistence)?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out_state = value_out_state;
            *out_advice = value_out_advice;
        }
        Ok(())
    })
}

/// Finish this engine's current usage deltas and observe their persistence.
/// Only usage-lock acquisition has a deadline; other filesystem work can take
/// longer. Written covers current deltas only, not future calls or other engines.
/// Failed carries static native safe advice readable after engine destruction
/// while the library remains loaded. Every nonzero return preserves outputs.
/// # Safety
/// The engine and output obligations of usage_persistence_v1 apply.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_finish_usage_status_v1(
    engine: *const crate::EngineHandle,
    out_state: *mut crate::session::views_generated::thinkthen_complete_usage_persistence_v1,
    out_advice: *mut crate::session::views::thinkthen_complete_utf8_v1,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if out_state.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out_advice.is_null() {
            return Err(ErrorKind::Usage);
        }
        if out_state.cast::<()>() == out_advice.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        let (value_out_state, value_out_advice) =
            super::usage_status(engine, thinkthen::Engine::finish_usage_status)?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out_state = value_out_state;
            *out_advice = value_out_advice;
        }
        Ok(())
    })
}

/// Admit one owned descriptor through the shared native decoder without waiting.
/// ACCEPTED retains decoded data; FULL and CLOSED retain nothing. Check native
/// capacity before decoding or allocating retained content. A concurrent closure
/// returns CLOSED without publishing. On OK status always receives a value.
/// # Safety
/// session is live, descriptor_json has descriptor_len readable UTF-8 bytes
/// (NULL requires zero), and status is nonnull and writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_try_push(
    session: *mut crate::session::SessionHandle,
    descriptor_json: *const std::ffi::c_char,
    descriptor_len: usize,
    status: *mut u32,
) -> std::ffi::c_int {
    failures::session_call(|| {
        if status.is_null() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_status =
            unsafe { super::try_push_json(session, descriptor_json, descriptor_len) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *status = value_status;
        }
        Ok(())
    })
}

/// Wait for input room and transfer one descriptor without consuming output.
/// Pending output cannot wake this producer. Closure or cancellation returns
/// CLOSED without decoding the payload. Failures preserve status and publish
/// only the calling thread diagnostic. No input bytes survive return.
/// # Safety
/// session stays live throughout the wait. descriptor_json has descriptor_len
/// readable bytes (NULL requires zero); status is nonnull writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_push(
    session: *mut crate::session::SessionHandle,
    descriptor_json: *const std::ffi::c_char,
    descriptor_len: usize,
    status: *mut u32,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if status.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_status = unsafe { super::push_json(session, descriptor_json, descriptor_len) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *status = value_status;
        }
        Ok(())
    })
}

/// Fix input EOF or a canonical closed reader failure. NULL with zero means EOF;
/// other inputs decode kind io, utf8, invalid_input or producer and optional location.
/// No input bytes survive return. A changed finish refuses without changing intake.
/// # Safety
/// session is live and failure_json has failure_len readable UTF-8 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_finish(
    session: *mut crate::session::SessionHandle,
    failure_json: *const std::ffi::c_char,
    failure_len: usize,
) -> std::ffi::c_int {
    failures::session_call(|| {
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        unsafe { super::finish_json(session, failure_json, failure_len) }
    })
}

/// Admit length-delimited UTF-8 request JSON and create an owned native session.
/// The engine identifies this invocation as the C surface.
/// Inputs may be freed or overwritten after return. The engine may be freed
/// after construction; the worker owns its engine. Immediate errors leave out
/// unchanged and record only the calling-thread session error slot.
/// # Safety
/// engine is live, request_json points at request_len readable bytes (NULL
/// requires zero), and out is writable. Extents must fit Rust slices.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_new(
    engine: *const crate::EngineHandle,
    request_json: *const std::ffi::c_char,
    request_len: usize,
    out: *mut *mut crate::session::SessionHandle,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_out = unsafe { super::json_session(engine, request_json, request_len) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Create an owned session attributed to its outer language wrapper.
/// surface names an exact native Surface token, without a trailing NUL.
/// Invalid tokens return Usage without retaining or printing their spelling.
/// All ownership, diagnostics and output rules match thinkthen_session_new.
/// # Safety
/// The thinkthen_session_new obligations apply. surface points at surface_len
/// readable UTF-8 bytes; NULL requires zero. Extents must fit Rust slices.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_new_with_surface(
    engine: *const crate::EngineHandle,
    request_json: *const std::ffi::c_char,
    request_len: usize,
    surface: *const std::ffi::c_char,
    surface_len: usize,
    out: *mut *mut crate::session::SessionHandle,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_out = unsafe {
            super::json_surface_session(engine, request_json, request_len, surface, surface_len)
        }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Construct one named request through Rust and create its owned native session.
/// Components use the existing canonical question, input and options grammars.
/// Materialized Records become find Units or relate Entities in shared Rust;
/// every materialized descriptor is decoded before session creation. Sources
/// and caller feeds retain their selectors. Unknown names say "unknown function".
/// Ownership, diagnostics and output rules match thinkthen_session_new.
/// # Safety
/// engine is live; request points to a readable aligned component structure.
/// Every view points to its counted readable UTF-8 bytes (NULL requires zero);
/// extents fit Rust slices. out is nonnull writable pointer storage. All inputs
/// may be freed or overwritten after return; the engine may also be freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_new_named_with_surface(
    engine: *const crate::EngineHandle,
    request: *const crate::ffi::session::NamedRequestV1,
    surface: crate::session::views::thinkthen_complete_utf8_v1,
    out: *mut *mut crate::session::SessionHandle,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_out = unsafe { super::named_session(engine, request, surface) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}

/// Signal cancellation without waiting for a provider or final facts. NULL is ignored.
/// # Safety
/// session is NULL or live throughout the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_cancel(session: *mut crate::session::SessionHandle) {
    failures::session_guard((), || {
        // SAFETY: the caller retains the nullable owner throughout the helper.
        unsafe { super::cancel_session(session) }
    })
}

/// Close the output receiver and release this owner without joining native work.
/// NULL is ignored. Free once after all concurrent operations return.
/// # Safety
/// session is NULL or an unfreed owner no operation is using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_free(session: *mut crate::session::SessionHandle) {
    failures::session_guard((), || {
        if !session.is_null() {
            // SAFETY: the owner came from this allocator and all borrowers have finished.
            drop(unsafe { Box::from_raw(session) });
        }
    })
}

/// Release an independent packet owner. NULL is ignored.
/// # Safety
/// result is NULL or an unfreed owner no operation or borrowed view is using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_result_free(
    result: *mut crate::session::SessionResultHandle,
) {
    failures::session_guard((), || {
        if !result.is_null() {
            // SAFETY: the owner came from this allocator and all borrowers have finished.
            drop(unsafe { Box::from_raw(result) });
        }
    })
}

/// Borrow a safe UTF-8 immediate diagnostic, never NULL. Success preserves
/// it; the next immediate session failure or thread exit ends its validity.
/// Execution failures arrive as terminal packets and never change this slot.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_session_error_message() -> *const std::ffi::c_char {
    failures::session_guard(
        c"defect: a session operation failed".as_ptr(),
        failures::session_message,
    )
}

/// Read the retry signal of the calling thread's last immediate session failure.
/// Returns zero before a failure. Success and engine construction preserve it.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_session_error_retryable() -> std::ffi::c_int {
    failures::session_guard(0, failures::session_retryable)
}

/// Borrow the native facts JSON of the last immediate session failure, or NULL.
/// The calling thread owns these bytes until its next immediate session failure
/// or thread exit. Success and engine construction preserve the same pointer.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_session_error_facts_json() -> *const std::ffi::c_char {
    failures::session_guard(std::ptr::null(), failures::session_facts)
}
