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
