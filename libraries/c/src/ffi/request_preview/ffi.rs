//! Canonical Request preview forwards to native admission and returns owned JSON.
#![allow(
    unsafe_code,
    reason = "the counted C interface reads and writes caller storage"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::EngineHandle;
use std::ffi::{CString, c_char};
use thinkthen::{ErrorKind, Request, RequestEnvironment};

pub(super) fn plan(
    engine: &EngineHandle,
    request: Request,
) -> Result<(*mut c_char, usize), ErrorKind> {
    let admitted = request.admit().map_err(|error| error.kind())?;
    let estimate = engine
        .0
        .engine
        .plan_request(&admitted, RequestEnvironment::default())
        .map_err(|error| error.kind())?;
    let json = serde_json::to_string(&estimate).map_err(|_| ErrorKind::Defect)?;
    let json = CString::new(json).map_err(|_| ErrorKind::Defect)?;
    let len = json.as_bytes().len();
    Ok((json.into_raw(), len))
}

pub(super) unsafe fn json_plan(
    engine: &EngineHandle,
    request_json: *const c_char,
    request_len: usize,
) -> Result<(*mut c_char, usize), ErrorKind> {
    // SAFETY: caller supplies the documented live readable UTF-8 extent.
    let text = unsafe { super::text(request_json, request_len) }.map_err(|_| ErrorKind::Usage)?;
    let request = Request::from_json(text).map_err(|error| error.kind())?;
    plan(engine, request)
}
