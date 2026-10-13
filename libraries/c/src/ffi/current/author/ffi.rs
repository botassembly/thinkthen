//! Native question parsing and owner-borrowed author snapshots.
#![allow(
    unsafe_code,
    reason = "counted descriptors and outputs obey the installed C header"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use super::read;
use crate::current::{self, QuestionHandle};
use crate::ffi::carriers::{QuestionAuthorV1, StringV1};

// SAFETY: AuthorOwner is constructed only by AuthorOwner::new. Its closed
// builder stores cloned native metadata and boxed immutable bytes/property
// arrays. All view pointers address those allocations; no caller pointer,
// interior mutation, or mutable accessor survives construction. Moving the
// owner preserves allocation addresses, and shared reads cannot modify them.
unsafe impl Send for crate::current::author::AuthorOwner {}
// SAFETY: the same closed, immutable backing construction permits shared reads.
unsafe impl Sync for crate::current::author::AuthorOwner {}

pub(super) fn author_view(
    owner: &QuestionHandle,
) -> Result<QuestionAuthorV1, thinkthen::ErrorKind> {
    Ok(owner.author.view)
}
pub(super) unsafe fn parse_question(
    role: u32,
    json: StringV1,
) -> Result<*mut QuestionHandle, crate::failures::Failure> {
    // SAFETY: the caller retains the counted saved-question grammar throughout admission.
    let value = current::question::parse_role(role, unsafe { read::string(json) }?)?;
    Ok(Box::into_raw(Box::new(value)))
}

pub(super) fn question_value(
    owner: &QuestionHandle,
) -> Result<*const crate::session::views::thinkthen_complete_json_v1, thinkthen::ErrorKind> {
    super::super::request_preview::value_view(&owner.definition)
}
