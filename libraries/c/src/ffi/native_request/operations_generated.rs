//! Generated native operation transport. Do not edit.
#![allow(
    unsafe_code,
    reason = "foreign slots follow the operation ownership contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures;
use thinkthen::ErrorKind;
/// Push a counted typed feed descriptor. Rust owns framing and admission.
/// No input pointer survives return; status is published only on success.
/// # Safety
/// session is live, status is writable and nonnull, and descriptor satisfies
/// the finite aligned carrier-tree obligations of the typed constructor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_try_push_typed(
    session: *const crate::session::SessionHandle,
    descriptor: *const super::carriers::thinkthen_request_session_descriptor_v1,
    status: *mut u32,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if status.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_status = unsafe { super::push_typed(session, descriptor, false) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *status = value_status;
        }
        Ok(())
    })
}

/// Wait for input room and transfer a typed descriptor without consuming output.
/// Closed intake retains nothing and never inspects the descriptor tree.
/// No input pointer survives return; status is published only on success.
/// # Safety
/// session is live, status is writable and nonnull, and descriptor satisfies
/// the finite aligned carrier-tree obligations of the typed constructor.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_push_typed(
    session: *const crate::session::SessionHandle,
    descriptor: *const super::carriers::thinkthen_request_session_descriptor_v1,
    status: *mut u32,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if status.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let session = unsafe { session.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_status = unsafe { super::push_typed(session, descriptor, true) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *status = value_status;
        }
        Ok(())
    })
}

/// Construct a named request from counted typed argument records through Rust.
/// The carrier tree copies strings, arrays, maps and explicit null alternatives;
/// no pointer survives return. Rust alone admits questions, controls and evidence.
/// COBOL retains its 8192-byte capacity per scalar, including image/base64 text;
/// overflow returns THINKTHEN_COBOL_OVERFLOW before constructing a session.
/// All output, error-slot and engine lifetime rules match the named constructor.
/// # Safety
/// engine is live and out is writable, nonnull. function and surface are counted
/// UTF-8 views. arguments and its entire finite carrier tree remain aligned and
/// readable throughout the call. Each pointer/length covers its declared element
/// type and fits Rust slices. Required pointers are nonnull; optional NULL means
/// missing. A tagged null arm means explicit null. Inputs may be freed on return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_session_new_typed_with_surface(
    engine: *const crate::EngineHandle,
    function: crate::session::views::thinkthen_complete_utf8_v1,
    arguments: *const super::carriers::thinkthen_request_arguments_v1,
    surface_name: crate::session::views::thinkthen_complete_utf8_v1,
    out: *mut *mut crate::session::SessionHandle,
) -> std::ffi::c_int {
    failures::session_native_call(|| {
        if out.is_null() {
            return Err(ErrorKind::Usage.into());
        }
        // SAFETY: the caller retains this live owner throughout the operation.
        let engine = unsafe { engine.as_ref() }.ok_or(ErrorKind::Usage)?;
        // SAFETY: the caller retains counted input trees until the native helper returns.
        let value_out = unsafe { super::typed_session(engine, function, arguments, surface_name) }?;
        // SAFETY: every slot was validated before the native operation; success publishes once.
        unsafe {
            *out = value_out;
        }
        Ok(())
    })
}
