//! Every session export guards panics and publishes outputs only on success.
#![allow(
    unsafe_code,
    reason = "the C ownership contract permits foreign pointer access"
)]
#![deny(unsafe_op_in_unsafe_fn)]
#[path = "operations_generated.rs"]
mod operations;
use crate::EngineHandle;
use crate::failures;
use crate::session::{SessionHandle, SessionResultHandle};
pub use operations::*;
use std::ffi::c_char;
use thinkthen::{ErrorKind, Request, RequestReaderFailure, RequestSessionRead, Surface};

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
    engine: *const EngineHandle,
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
    engine: *const EngineHandle,
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

/// Existing canonical components for one named request. Every view is counted
/// UTF-8, without a trailing NUL. No bytes or foreign pointers survive return.
#[repr(C)]
pub struct NamedRequestV1 {
    /// Exact lower-case public function name.
    pub function: crate::session::views::thinkthen_complete_utf8_v1,
    /// Existing canonical question selector JSON.
    pub question_json: crate::session::views::thinkthen_complete_utf8_v1,
    /// Existing canonical input selector JSON.
    pub input_json: crate::session::views::thinkthen_complete_utf8_v1,
    /// Existing canonical options JSON; pass {} for omitted controls.
    pub options_json: crate::session::views::thinkthen_complete_utf8_v1,
}

impl std::fmt::Debug for NamedRequestV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NamedRequestV1(<withheld>)")
    }
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
    engine: *const EngineHandle,
    request: *const NamedRequestV1,
    surface: crate::session::views::thinkthen_complete_utf8_v1,
    out: *mut *mut SessionHandle,
) -> std::ffi::c_int {
    // SAFETY: every component and the output obey the caller's counted contract.
    unsafe {
        start_session(
            engine,
            out,
            || named_text(surface)?.parse().map_err(|_| ErrorKind::Usage),
            || {
                let request = request.as_ref().ok_or(ErrorKind::Usage)?;
                Request::named_from_json(
                    named_text(request.function)?,
                    named_text(request.question_json)?,
                    named_text(request.input_json)?,
                    named_text(request.options_json)?,
                )
                .map_err(decode_failure)
            },
        )
    }
}

unsafe fn named_text<'a>(
    view: crate::session::views::thinkthen_complete_utf8_v1,
) -> Result<&'a str, ErrorKind> {
    // SAFETY: the caller retains the readable extent for this shared borrow.
    unsafe { super::text(view.data.cast(), view.len) }.map_err(|_| ErrorKind::Usage)
}

pub(super) fn decode_failure(error: thinkthen::Error) -> failures::Failure {
    if error.request_decode_cause() == Some(thinkthen::RequestDecodeCause::Malformed) {
        ErrorKind::Usage.into()
    } else {
        error.into()
    }
}

unsafe fn new_session(
    engine: *const EngineHandle,
    request_json: *const c_char,
    request_len: usize,
    out: *mut *mut SessionHandle,
    surface: impl FnOnce() -> Result<Surface, ErrorKind>,
) -> std::ffi::c_int {
    // SAFETY: forward the existing export's input and output obligations.
    unsafe {
        start_session(engine, out, surface, || {
            let text = super::text(request_json, request_len).map_err(|_| ErrorKind::Usage)?;
            Request::from_json(text).map_err(decode_failure)
        })
    }
}

pub(super) unsafe fn start_session(
    engine: *const EngineHandle,
    out: *mut *mut SessionHandle,
    surface: impl FnOnce() -> Result<Surface, ErrorKind>,
    request: impl FnOnce() -> Result<Request, failures::Failure>,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        required(out)?;
        let surface = surface()?;
        // SAFETY: the caller keeps its engine live until this call returns.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        let session = engine
            .0
            .engine
            .request_session_with_surface(request()?, surface)
            .map_err(failures::Failure::from)?;
        let owner = Box::into_raw(Box::new(SessionHandle(session, surface)));
        // SAFETY: required checked nonnull and the caller promises writable storage.
        unsafe {
            *out = owner;
        }
        Ok(())
    })
}

fn read_packet(
    session: &SessionHandle,
    interest: Option<u32>,
) -> Result<(u32, *mut SessionResultHandle), ErrorKind> {
    let packet = match interest {
        None => session.0.try_read(),
        Some(THINKTHEN_SESSION_WAIT_RESULT_V1) => session
            .0
            .next_packet(thinkthen::RequestSessionWaitInterest::Result)
            .map_err(|error| error.kind())?,
        Some(THINKTHEN_SESSION_WAIT_SETTLEMENT_V1) => session
            .0
            .next_packet(thinkthen::RequestSessionWaitInterest::Settlement)
            .map_err(|error| error.kind())?,
        _ => return Err(ErrorKind::Usage),
    };
    let (state, owner) = match packet {
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
    Ok((state, owner))
}

/// Signal cancellation without waiting for a provider or final facts. NULL is ignored.
/// # Safety
/// session is NULL or live throughout the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_cancel(session: *mut SessionHandle) {
    failures::session_guard((), || {
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
    failures::session_guard((), || {
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
pub extern "C" fn thinkthen_session_error_facts_json() -> *const c_char {
    failures::session_guard(std::ptr::null(), failures::session_facts)
}

/// Release an independent packet owner. NULL is ignored.
/// # Safety
/// result is NULL or an unfreed owner no operation or borrowed view is using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_result_free(result: *mut SessionResultHandle) {
    failures::session_guard((), || {
        if !result.is_null() {
            // SAFETY: try_read boxed this owner and all borrowers finished.
            drop(unsafe { Box::from_raw(result) });
        }
    });
}

fn usage_status(
    engine: &EngineHandle,
    observe: fn(&thinkthen::Engine) -> thinkthen::UsagePersistence,
) -> Result<
    (
        crate::session::views_generated::thinkthen_complete_usage_persistence_v1,
        crate::session::views::thinkthen_complete_utf8_v1,
    ),
    ErrorKind,
> {
    use crate::session::views_generated as view;
    use thinkthen::UsagePersistence;
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
    Ok((
        view::thinkthen_complete_usage_persistence_v1 { kind },
        advice,
    ))
}

#[cfg(test)]
#[path = "tests/ffi.rs"]
mod tests;

fn wait_ready(session: &SessionHandle, interest: u32, timeout_ms: u32) -> Result<u32, ErrorKind> {
    use thinkthen::RequestSessionWaitInterest as Interest;
    let interest = match interest {
        THINKTHEN_SESSION_WAIT_RESULT_V1 => Interest::Result,
        THINKTHEN_SESSION_WAIT_INPUT_V1 => Interest::Input,
        THINKTHEN_SESSION_WAIT_SETTLEMENT_V1 => Interest::Settlement,
        _ => return Err(ErrorKind::Usage),
    };
    let ready = session.0.wait(
        interest,
        std::time::Duration::from_millis(u64::from(timeout_ms)),
    );
    let ready = ready_status(ready);
    Ok(ready)
}
fn packet_json(result: &SessionResultHandle) -> Result<(*const c_char, usize), ErrorKind> {
    result.json()
}
fn packet_view(
    result: &SessionResultHandle,
) -> Result<*const crate::session::views_generated::thinkthen_complete_session_packet_v1, ErrorKind>
{
    result.view()
}

unsafe fn try_push_json(
    session: &SessionHandle,
    descriptor_json: *const c_char,
    descriptor_len: usize,
) -> Result<u32, ErrorKind> {
    // SAFETY: the caller promises the counted readable extent.
    let text =
        unsafe { super::text(descriptor_json, descriptor_len) }.map_err(|_| ErrorKind::Usage)?;
    let state = push_status(
        session
            .0
            .try_push_json(text)
            .map_err(|error| error.kind())?,
    );
    Ok(state)
}

unsafe fn push_json(
    session: &SessionHandle,
    descriptor_json: *const c_char,
    descriptor_len: usize,
) -> Result<u32, failures::Failure> {
    let state = push_status(session.0.push_with::<failures::Failure>(|| {
        // SAFETY: the caller retains its counted payload until transfer returns.
        let text = unsafe { super::text(descriptor_json, descriptor_len) }
            .map_err(|_| ErrorKind::Usage)?;
        thinkthen::RequestSessionDescriptor::from_json(text).map_err(failures::Failure::from)
    })?);
    Ok(state)
}

unsafe fn finish_json(
    session: &SessionHandle,
    failure_json: *const c_char,
    failure_len: usize,
) -> Result<(), ErrorKind> {
    let failure = if failure_json.is_null() && failure_len == 0 {
        None
    } else {
        // SAFETY: the caller promises the counted readable extent.
        let text =
            unsafe { super::text(failure_json, failure_len) }.map_err(|_| ErrorKind::Usage)?;
        Some(RequestReaderFailure::from_json(text).map_err(|error| error.kind())?)
    };
    session.0.finish(failure).map_err(|error| error.kind())
}
