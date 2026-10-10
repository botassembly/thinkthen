//! Canonical Request preview forwards to native admission and returns owned JSON.
#![allow(
    unsafe_code,
    reason = "the counted C door reads and writes caller storage"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::{Door, session::errors};
use std::ffi::{CString, c_char};
use thinkthen::{ErrorKind, Request, RequestEnvironment};

/// Preview canonical Request JSON without reading a key or cache or sending.
/// Preview supported fixed atomic questions under shared Request admission; unsupported functions and dynamic questions refuse.
/// Success owns NUL-terminated plan JSON in out and its byte length in out_len;
/// free once with thinkthen_free_string. Refusal leaves both outputs unchanged
/// and records the safe calling-thread thinkthen_session_error_message.
/// # Safety
/// engine is live; request_json points to request_len readable bytes (NULL
/// requires zero); out and out_len are nonnull writable, nonoverlapping slots.
/// Extents fit Rust slices. The engine remains live until this call returns.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_request_plan_json(
    engine: *const Door,
    request_json: *const c_char,
    request_len: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
    errors::call(|| {
        if out.is_null() || out_len.is_null() || out.cast::<()>() == out_len.cast::<()>() {
            return Err(ErrorKind::Usage);
        }
        // SAFETY: caller keeps its engine live throughout this operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: caller supplies the documented readable extent.
        let text =
            unsafe { super::text(request_json, request_len) }.map_err(|_| ErrorKind::Usage)?;
        let admitted = Request::from_json(text)
            .and_then(Request::admit)
            .map_err(|error| error.kind())?;
        let estimate = engine
            .0
            .engine
            .plan_request(&admitted, RequestEnvironment::default())
            .map_err(|error| error.kind())?;
        let json = serde_json::to_string(&estimate).map_err(|_| ErrorKind::Defect)?;
        let json = CString::new(json).map_err(|_| ErrorKind::Defect)?;
        let len = json.as_bytes().len();
        // SAFETY: validated slots are writable and distinct under the contract.
        unsafe {
            *out = json.into_raw();
            *out_len = len;
        }
        Ok(())
    })
}
