//! Generated native operation transport. Do not edit.
#![allow(
    unsafe_code,
    reason = "foreign slots follow the operation ownership contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use crate::failures;
use thinkthen::ErrorKind;
/// Free the question owner after every borrower finishes; NULL is ignored.
/// # Safety
/// owner is NULL or a live unfreed question no concurrent call uses.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_free(owner: *mut crate::current::QuestionHandle) {
    super::guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: the owner came from this allocator and all borrowers have finished.
            drop(unsafe { Box::from_raw(owner) });
        }
    })
}

/// Borrow immutable metadata until question_free; publish only on success.
/// # Safety
/// owner is live and out is aligned writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_author(
    owner: *const crate::current::QuestionHandle,
    out: *mut crate::ffi::carriers::QuestionAuthorV1,
) -> std::ffi::c_int {
    failures::guard(None, failures::DEFECT, || {
        let outcome = (|| {
            if out.is_null() {
                return Err(ErrorKind::Usage);
            }
            // SAFETY: the caller retains this live owner throughout the operation.
            let owner = unsafe { owner.as_ref() }.ok_or(ErrorKind::Usage)?;
            let value_out = super::author::author_view(owner)?;
            // SAFETY: every slot was validated before the native operation; success publishes once.
            unsafe {
                *out = value_out;
            }
            Ok(())
        })();
        match outcome {
            Ok(()) => failures::OK,
            Err(_error) => failures::USAGE,
        }
    })
}

/// Import counted saved-question grammar through the explicitly chosen native role.
/// Grammar failures return EUSAGE; no guessing or parser fallback. The immutable
/// owner copies every byte and does not judge. Engine-thread failures stay independent
/// from immediate session and constructor slots.
/// # Safety
/// engine is live; json has readable counted UTF-8; out is aligned writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_parse(
    engine: *const crate::EngineHandle,
    role: u32,
    json: crate::ffi::carriers::StringV1,
    out: *mut *mut crate::current::QuestionHandle,
) -> std::ffi::c_int {
    // SAFETY: the caller retains its nullable engine throughout admission.
    let Some(held) = (unsafe { crate::ffi::held(engine) }) else {
        return failures::USAGE;
    };
    failures::guard(Some(held), failures::DEFECT, || {
        let outcome = (|| {
            if out.is_null() {
                return Err(failures::Failure::usage("a null required pointer"));
            }
            // SAFETY: the caller retains counted input trees until the native helper returns.
            let value_out = unsafe { super::author::parse_question(role, json) }?;
            // SAFETY: every slot was validated before the native operation; success publishes once.
            unsafe {
                *out = value_out;
            }
            Ok(())
        })();
        match held.settle(outcome) {
            Ok(()) => failures::OK,
            Err(code) => code,
        }
    })
}
