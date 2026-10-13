//! Native question parsing and owner-borrowed author snapshots.
#![allow(
    unsafe_code,
    reason = "counted descriptors and outputs obey the installed C header"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use super::read;
use crate::ffi::carriers::{QuestionAuthorV1, StringV1};
use crate::{
    EngineHandle,
    current::{self, QuestionHandle},
    failures::{DEFECT, OK, USAGE, guard},
};

/// Borrow the native author snapshot owned by this question.
/// # Safety
/// Question and output obey the header's lifetime/storage contract.
/// Borrow metadata owned by this immutable question until question_free.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_author(
    owner: *const QuestionHandle,
    out: *mut QuestionAuthorV1,
) -> std::ffi::c_int {
    guard(None, DEFECT, || {
        if out.is_null() {
            return USAGE;
        }
        // SAFETY: owner is NULL or live and immutable throughout this access.
        let Some(owner) = (unsafe { owner.as_ref() }) else {
            return USAGE;
        };
        // SAFETY: writable initialized output is published only on success.
        unsafe {
            *out = owner.author.view;
        }
        OK
    })
}
// SAFETY: AuthorOwner is constructed only by AuthorOwner::new. Its closed
// builder stores cloned native metadata and boxed immutable bytes/property
// arrays. All view pointers address those allocations; no caller pointer,
// interior mutation, or mutable accessor survives construction. Moving the
// owner preserves allocation addresses, and shared reads cannot modify them.
unsafe impl Send for crate::current::author::AuthorOwner {}
// SAFETY: the same closed, immutable backing construction permits shared reads.
unsafe impl Sync for crate::current::author::AuthorOwner {}

/// Import counted saved-question grammar through an explicitly selected native parser.
/// # Safety
/// Counted bytes and output obey the header's storage contract.
/// Import saved question-file JSON through an explicit native grammar role.
/// This imports a question only; judgment calls and result fields remain typed.
/// Inline grammar failures return EUSAGE; no role guessing or parser fallback.
/// The immutable handle owns every byte independently of json's lifetime.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_question_parse(
    engine: *const EngineHandle,
    role: u32,
    json: StringV1,
    out: *mut *mut QuestionHandle,
) -> std::ffi::c_int {
    // SAFETY: common edge validates live engine/output and counted UTF-8 storage.
    unsafe {
        crate::ffi::run_typed(
            engine,
            |_| {
                read::required(out)?;
                current::question::parse_role(role, read::string(json)?)
            },
            |_, value| {
                *out = Box::into_raw(Box::new(value));
                OK
            },
        )
    }
}
