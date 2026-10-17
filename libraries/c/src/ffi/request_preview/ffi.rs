//! Canonical Request preview forwards to native admission and returns owned JSON.
#![allow(
    unsafe_code,
    reason = "the counted C interface reads and writes caller storage"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::EngineHandle;
use std::ffi::{CString, c_char};
use thinkthen::{ErrorKind, Request, RequestEnvironment};

pub(super) fn plan<T>(
    engine: &EngineHandle,
    request: Request,
    output: impl FnOnce(&str) -> Result<T, ErrorKind>,
) -> Result<T, ErrorKind> {
    let admitted = request.admit().map_err(|error| error.kind())?;
    let estimate = engine
        .0
        .engine
        .plan_request(&admitted, RequestEnvironment::default())
        .map_err(|error| error.kind())?;
    let json = serde_json::to_string(&estimate).map_err(|_| ErrorKind::Defect)?;
    output(&json)
}
pub(super) fn json_output(json: &str) -> Result<(*mut c_char, usize), ErrorKind> {
    let json = CString::new(json).map_err(|_| ErrorKind::Defect)?;
    let len = json.as_bytes().len();
    Ok((json.into_raw(), len))
}

pub(super) unsafe fn json_plan<T>(
    engine: &EngineHandle,
    request_json: *const c_char,
    request_len: usize,
    output: impl FnOnce(&str) -> Result<T, ErrorKind>,
) -> Result<T, ErrorKind> {
    // SAFETY: caller supplies the documented live readable UTF-8 extent.
    let text = unsafe { super::text(request_json, request_len) }.map_err(|_| ErrorKind::Usage)?;
    let request = Request::from_json(text).map_err(|error| error.kind())?;
    plan(engine, request, output)
}

// SAFETY: the closed ValueHandle constructor owns only read_json's immutable
// boxed roots, nested values and copied bytes. No caller pointers or mutable
// accessors survive; moving owners preserves backing and concurrent reads.
unsafe impl Send for crate::session::views::ValueHandle {}
// SAFETY: the same immutable closed construction permits shared reads.
unsafe impl Sync for crate::session::views::ValueHandle {}
pub(super) fn value_output(
    json: &str,
) -> Result<*mut crate::session::views::ValueHandle, ErrorKind> {
    Ok(Box::into_raw(Box::new(
        crate::session::views::ValueHandle::new(json)?,
    )))
}
pub(super) fn value_view(
    owner: &crate::session::views::ValueHandle,
) -> Result<*const crate::session::views::thinkthen_complete_json_v1, ErrorKind> {
    Ok(owner.0.root)
}
pub(super) unsafe fn named_plan(
    engine: &EngineHandle,
    request: *const super::session::NamedRequestV1,
) -> Result<*mut crate::session::views::ValueHandle, crate::failures::Failure> {
    // SAFETY: all named components remain live until this operation returns.
    plan(
        engine,
        unsafe { super::session::named_request(request) }?,
        value_output,
    )
    .map_err(Into::into)
}

pub(super) unsafe fn parse_value(
    json: *const c_char,
    len: usize,
) -> Result<*mut crate::session::views::ValueHandle, ErrorKind> {
    // SAFETY: caller supplies readable counted UTF-8 until return.
    let text = unsafe { super::text(json, len) }.map_err(|_| ErrorKind::Usage)?;
    value_output(text).map_err(|_| ErrorKind::Usage)
}
