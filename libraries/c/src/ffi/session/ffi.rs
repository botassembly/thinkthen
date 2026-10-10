//! Every session export guards panics and publishes outputs only on success.
#![allow(
    unsafe_code,
    reason = "the C ownership contract permits foreign pointer access"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::Door;
use crate::session::{SessionHandle, SessionResultHandle, errors};
use std::ffi::c_char;
use thinkthen::{
    ErrorKind, Request, RequestReaderFailure, RequestSessionPushStatus, RequestSessionRead, Surface,
};

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

fn required<T>(slot: *mut T) -> Result<(), ErrorKind> {
    if slot.is_null() {
        Err(ErrorKind::Usage)
    } else {
        Ok(())
    }
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
    engine: *const Door,
    request_json: *const c_char,
    request_len: usize,
    out: *mut *mut SessionHandle,
) -> std::ffi::c_int {
    // SAFETY: this export forwards the caller's documented pointer obligations.
    unsafe { new_session(engine, request_json, request_len, out, || Ok(Surface::C)) }
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
    engine: *const Door,
    request_json: *const c_char,
    request_len: usize,
    surface: *const c_char,
    surface_len: usize,
    out: *mut *mut SessionHandle,
) -> std::ffi::c_int {
    // SAFETY: both counted inputs obey the caller's readable extent contract.
    unsafe {
        new_session(engine, request_json, request_len, out, || {
            super::text(surface, surface_len)
                .map_err(|_| ErrorKind::Usage)?
                .parse::<Surface>()
                .map_err(|_| ErrorKind::Usage)
        })
    }
}

unsafe fn new_session(
    engine: *const Door,
    request_json: *const c_char,
    request_len: usize,
    out: *mut *mut SessionHandle,
    surface: impl FnOnce() -> Result<Surface, ErrorKind>,
) -> std::ffi::c_int {
    errors::native_call(|| {
        required(out)?;
        let surface = surface()?;
        // SAFETY: the caller keeps its engine live until this call returns.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller supplies the documented readable extent.
        let text =
            unsafe { super::text(request_json, request_len) }.map_err(|_| ErrorKind::Usage)?;
        let request = Request::from_json(text).map_err(|error| {
            if error.to_string() == "invalid canonical request" {
                errors::Failure::Kind(ErrorKind::Usage)
            } else {
                errors::Failure::Native(error)
            }
        })?;
        let session = engine
            .0
            .engine
            .request_session_with_surface(request, surface)
            .map_err(errors::Failure::Native)?;
        let owner = Box::into_raw(Box::new(SessionHandle(session)));
        // SAFETY: required checked nonnull and the caller promises writable storage.
        unsafe {
            *out = owner;
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
    session: *mut SessionHandle,
    status: *mut u32,
    out: *mut *mut SessionResultHandle,
) -> std::ffi::c_int {
    errors::call(|| {
        required(status)?;
        required(out)?;
        if status.cast::<()>() == out.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller holds the session live during the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        let (state, owner) = match session.0.try_read() {
            RequestSessionRead::Result(packet) => (
                THINKTHEN_SESSION_RESULT_V1,
                Box::into_raw(Box::new(SessionResultHandle {
                    packet,
                    json: std::sync::Mutex::new(None),
                    view: std::sync::Mutex::new(None),
                })),
            ),
            RequestSessionRead::Pending => (THINKTHEN_SESSION_PENDING_V1, std::ptr::null_mut()),
            RequestSessionRead::End => (THINKTHEN_SESSION_END_V1, std::ptr::null_mut()),
        };
        // SAFETY: required outputs are distinct, writable slots under the contract.
        unsafe {
            *status = state;
            *out = owner;
        }
        Ok(())
    })
}

/// Signal cancellation without waiting for a provider or final facts. NULL is ignored.
/// # Safety
/// session is NULL or live throughout the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_cancel(session: *mut SessionHandle) {
    errors::guard((), || {
        // SAFETY: the caller provides NULL or a live owner.
        if let Some(session) = unsafe { session.as_ref() } {
            session.0.cancel();
        }
    });
}

/// Close the output receiver and release this owner without joining native work.
/// NULL is ignored. Free once after all concurrent operations return.
/// # Safety
/// session is NULL or an unfreed owner no operation is using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_free(session: *mut SessionHandle) {
    errors::guard((), || {
        if !session.is_null() {
            // SAFETY: the constructor boxed this owner and all operations returned.
            drop(unsafe { Box::from_raw(session) });
        }
    });
}

/// Borrow a safe UTF-8 immediate diagnostic, never NULL. Success preserves
/// it; the next immediate session failure or thread exit ends its validity.
/// Execution failures arrive as terminal packets and never change this slot.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_session_error_message() -> *const c_char {
    errors::guard(
        c"defect: a session operation failed".as_ptr(),
        errors::message,
    )
}

/// Release an independent packet owner. NULL is ignored.
/// # Safety
/// result is NULL or an unfreed owner no operation or borrowed view is using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_result_free(result: *mut SessionResultHandle) {
    errors::guard((), || {
        if !result.is_null() {
            // SAFETY: try_read boxed this owner and all borrowers finished.
            drop(unsafe { Box::from_raw(result) });
        }
    });
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
    session: *mut SessionHandle,
    descriptor_json: *const c_char,
    descriptor_len: usize,
    status: *mut u32,
) -> std::ffi::c_int {
    errors::call(|| {
        required(status)?;
        // SAFETY: the caller keeps its owner live and supplies readable bytes.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller promises the counted readable extent.
        let text = unsafe { super::text(descriptor_json, descriptor_len) }
            .map_err(|_| ErrorKind::Usage)?;
        let state = match session
            .0
            .try_push_json(text)
            .map_err(|error| error.kind())?
        {
            RequestSessionPushStatus::Accepted => THINKTHEN_SESSION_ACCEPTED_V1,
            RequestSessionPushStatus::Full => THINKTHEN_SESSION_FULL_V1,
            RequestSessionPushStatus::Closed => THINKTHEN_SESSION_CLOSED_V1,
        };
        // SAFETY: required checked nonnull and the caller promises writable storage.
        unsafe {
            *status = state;
        }
        Ok(())
    })
}

/// Fix input EOF or a canonical closed reader failure. NULL with zero means EOF;
/// other inputs decode kind io, utf8 or invalid_input and optional location.
/// No input bytes survive return. A changed finish refuses without changing intake.
/// # Safety
/// session is live and failure_json has failure_len readable UTF-8 bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_finish(
    session: *mut SessionHandle,
    failure_json: *const c_char,
    failure_len: usize,
) -> std::ffi::c_int {
    errors::call(|| {
        // SAFETY: the caller keeps the owner live during the call.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        let failure = if failure_json.is_null() && failure_len == 0 {
            None
        } else {
            // SAFETY: the caller promises the counted readable extent.
            let text =
                unsafe { super::text(failure_json, failure_len) }.map_err(|_| ErrorKind::Usage)?;
            Some(RequestReaderFailure::from_json(text).map_err(|error| error.kind())?)
        };
        session.0.finish(failure).map_err(|error| error.kind())
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
    result: *const SessionResultHandle,
    out: *mut *const c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
    errors::call(|| {
        required(out)?;
        required(out_len)?;
        if out.cast::<()>() == out_len.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: the caller keeps this independent owner live during access.
        let result = unsafe { result.as_ref() }.ok_or(ErrorKind::Usage)?;
        let (bytes, len) = result.json()?;
        // SAFETY: both outputs were checked before any fallible operation.
        unsafe {
            *out = bytes;
            *out_len = len;
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
    result: *const SessionResultHandle,
    out: *mut *const crate::session::views_generated::thinkthen_complete_session_packet_v1,
) -> std::ffi::c_int {
    errors::call(|| {
        required(out)?;
        // SAFETY: the caller keeps the independent result owner live.
        let result = unsafe { result.as_ref() }.ok_or(ErrorKind::Usage)?;
        let view = result.view()?;
        // SAFETY: required checked the caller's documented writable slot.
        unsafe {
            *out = view;
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
    engine: *const Door,
    out_state: *mut crate::session::views_generated::thinkthen_complete_usage_persistence_v1,
    out_advice: *mut crate::session::views::thinkthen_complete_utf8_v1,
) -> std::ffi::c_int {
    // SAFETY: forward the caller's documented engine and output obligations.
    unsafe {
        usage_status(
            engine,
            out_state,
            out_advice,
            thinkthen::Engine::usage_persistence,
        )
    }
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
    engine: *const Door,
    out_state: *mut crate::session::views_generated::thinkthen_complete_usage_persistence_v1,
    out_advice: *mut crate::session::views::thinkthen_complete_utf8_v1,
) -> std::ffi::c_int {
    // SAFETY: forward the caller's documented engine and output obligations.
    unsafe {
        usage_status(
            engine,
            out_state,
            out_advice,
            thinkthen::Engine::finish_usage_status,
        )
    }
}

unsafe fn usage_status(
    engine: *const Door,
    out_state: *mut crate::session::views_generated::thinkthen_complete_usage_persistence_v1,
    out_advice: *mut crate::session::views::thinkthen_complete_utf8_v1,
    observe: fn(&thinkthen::Engine) -> thinkthen::UsagePersistence,
) -> std::ffi::c_int {
    use crate::session::views_generated as view;
    use thinkthen::UsagePersistence;
    errors::call(|| {
        required(out_state)?;
        required(out_advice)?;
        if out_state.cast::<()>() == out_advice.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: outputs are checked before observing the caller's live engine.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        let observed = observe(&engine.0.engine);
        let kind = match observed {
            UsagePersistence::Disabled => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1,
            UsagePersistence::Pending => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1,
            UsagePersistence::Written => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1,
            UsagePersistence::Failed => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1,
        };
        let advice = observed.advice().map_or(
            crate::session::views::thinkthen_complete_utf8_v1 {
                data: std::ptr::null(),
                len: 0,
            },
            |text| crate::session::views::thinkthen_complete_utf8_v1 {
                data: text.as_ptr().cast(),
                len: text.len(),
            },
        );
        // SAFETY: both outputs are writable, distinct slots; nothing fallible
        // remains after the observation and static projection have succeeded.
        unsafe {
            *out_state = view::thinkthen_complete_usage_persistence_v1 { kind };
            *out_advice = advice;
        }
        Ok(())
    })
}
