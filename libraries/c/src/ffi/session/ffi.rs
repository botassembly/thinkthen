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

/// Existing canonical components for one named request. Every view is counted
/// UTF-8, without a trailing NUL. No bytes or foreign pointers survive return.
#[repr(C)]
pub struct NamedRequest {
    /// Exact lower-case public function name.
    pub function: crate::session::views::thinkthen_complete_utf8,
    /// Existing canonical question selector JSON.
    pub question_json: crate::session::views::thinkthen_complete_utf8,
    /// Existing canonical input selector JSON.
    pub input_json: crate::session::views::thinkthen_complete_utf8,
    /// Existing canonical options JSON; pass {} for omitted controls.
    pub options_json: crate::session::views::thinkthen_complete_utf8,
}

impl std::fmt::Debug for NamedRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NamedRequest(<withheld>)")
    }
}

unsafe fn named_text<'a>(
    view: crate::session::views::thinkthen_complete_utf8,
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
    engine: &EngineHandle,
    request_json: *const c_char,
    request_len: usize,
    surface: impl FnOnce() -> Result<Surface, ErrorKind>,
) -> Result<*mut SessionHandle, failures::Failure> {
    // SAFETY: forward the existing export's input and output obligations.
    unsafe {
        start_session(engine, surface, || {
            let text = super::text(request_json, request_len).map_err(|_| ErrorKind::Usage)?;
            Request::from_json(text).map_err(decode_failure)
        })
    }
}

pub(super) fn start_session(
    engine: &EngineHandle,
    surface: impl FnOnce() -> Result<Surface, ErrorKind>,
    request: impl FnOnce() -> Result<Request, failures::Failure>,
) -> Result<*mut SessionHandle, failures::Failure> {
    let surface = surface()?;
    let session = engine
        .0
        .engine
        .request_session_with_surface(request()?, surface)
        .map_err(failures::Failure::from)?;
    Ok(Box::into_raw(Box::new(SessionHandle(session, surface))))
}

fn read_packet(
    session: &SessionHandle,
    interest: Option<u32>,
) -> Result<(u32, *mut SessionResultHandle), ErrorKind> {
    let packet = match interest {
        None => session.0.try_read(),
        Some(THINKTHEN_SESSION_WAIT_RESULT) => session
            .0
            .next_packet(thinkthen::RequestSessionWaitInterest::Result)
            .map_err(|error| error.kind())?,
        Some(THINKTHEN_SESSION_WAIT_SETTLEMENT) => session
            .0
            .next_packet(thinkthen::RequestSessionWaitInterest::Settlement)
            .map_err(|error| error.kind())?,
        _ => return Err(ErrorKind::Usage),
    };
    let (state, owner) = match packet {
        RequestSessionRead::Result(packet) => (
            THINKTHEN_SESSION_RESULT,
            Box::into_raw(Box::new(SessionResultHandle {
                packet,
                json: std::sync::Mutex::new(None),
                view: std::sync::Mutex::new(None),
            })),
        ),
        RequestSessionRead::Pending => (THINKTHEN_SESSION_PENDING, std::ptr::null_mut()),
        RequestSessionRead::End => (THINKTHEN_SESSION_END, std::ptr::null_mut()),
    };
    Ok((state, owner))
}

fn usage_status(
    engine: &EngineHandle,
    observe: fn(&thinkthen::Engine) -> thinkthen::UsagePersistence,
) -> Result<
    (
        crate::session::views_generated::thinkthen_complete_usage_persistence,
        crate::session::views::thinkthen_complete_utf8,
    ),
    ErrorKind,
> {
    use crate::session::views_generated as view;
    use thinkthen::UsagePersistence;
    let observed = observe(&engine.0.engine);
    let kind = match observed {
        UsagePersistence::Disabled => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED,
        UsagePersistence::Pending => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING,
        UsagePersistence::Written => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN,
        UsagePersistence::Failed => view::THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED,
    };
    let advice = observed.advice().map_or(
        crate::session::views::thinkthen_complete_utf8 {
            data: std::ptr::null(),
            len: 0,
        },
        |text| crate::session::views::thinkthen_complete_utf8 {
            data: text.as_ptr().cast(),
            len: text.len(),
        },
    );
    Ok((
        view::thinkthen_complete_usage_persistence { kind },
        advice,
    ))
}

#[cfg(test)]
#[path = "tests/ffi.rs"]
mod tests;

fn wait_ready(session: &SessionHandle, interest: u32, timeout_ms: u32) -> Result<u32, ErrorKind> {
    use thinkthen::RequestSessionWaitInterest as Interest;
    let interest = match interest {
        THINKTHEN_SESSION_WAIT_RESULT => Interest::Result,
        THINKTHEN_SESSION_WAIT_INPUT => Interest::Input,
        THINKTHEN_SESSION_WAIT_SETTLEMENT => Interest::Settlement,
        THINKTHEN_SESSION_WAIT_SETTLEMENT_ARM if timeout_ms == 0 => Interest::SettlementArm,
        THINKTHEN_SESSION_WAIT_INTERRUPTIBLE_SETTLEMENT => Interest::InterruptibleSettlement,
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
) -> Result<*const crate::session::views_generated::thinkthen_complete_session_packet, ErrorKind>
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

unsafe fn json_session(
    engine: &EngineHandle,
    request_json: *const c_char,
    request_len: usize,
) -> Result<*mut SessionHandle, failures::Failure> {
    // SAFETY: this export forwards the caller's documented pointer obligations.
    unsafe { new_session(engine, request_json, request_len, || Ok(Surface::C)) }
}

unsafe fn json_surface_session(
    engine: &EngineHandle,
    request_json: *const c_char,
    request_len: usize,
    surface: *const c_char,
    surface_len: usize,
) -> Result<*mut SessionHandle, failures::Failure> {
    // SAFETY: both counted inputs obey the caller's readable extent contract.
    unsafe {
        new_session(engine, request_json, request_len, || {
            super::text(surface, surface_len)
                .map_err(|_| ErrorKind::Usage)?
                .parse::<Surface>()
                .map_err(|_| ErrorKind::Usage)
        })
    }
}

unsafe fn named_session(
    engine: &EngineHandle,
    request: *const NamedRequest,
    surface: crate::session::views::thinkthen_complete_utf8,
) -> Result<*mut SessionHandle, failures::Failure> {
    // SAFETY: every component and the output obey the caller's counted contract.
    unsafe {
        start_session(
            engine,
            || named_text(surface)?.parse().map_err(|_| ErrorKind::Usage),
            || named_request(request),
        )
    }
}

unsafe fn cancel_session(session: *mut SessionHandle) {
    // SAFETY: the caller supplies NULL or retains its live owner.
    if let Some(session) = unsafe { session.as_ref() } {
        session.0.cancel();
    }
}

pub(super) unsafe fn named_request(
    request: *const NamedRequest,
) -> Result<Request, failures::Failure> {
    // SAFETY: each named view inherits the caller's counted lifetime contract.
    unsafe {
        let request = request.as_ref().ok_or(ErrorKind::Usage)?;
        Request::named_from_json(
            named_text(request.function)?,
            named_text(request.question_json)?,
            named_text(request.input_json)?,
            named_text(request.options_json)?,
        )
        .map_err(decode_failure)
    }
}
