//! Counted question admission and owned author metadata.
#![allow(
    unsafe_code,
    reason = "the reviewed C lifetime contract permits foreign pointer access"
)]
#![deny(unsafe_op_in_unsafe_fn)]
#[path = "author/ffi.rs"]
pub(super) mod author;
#[path = "read/ffi.rs"]
pub(crate) mod read;
use crate::current::QuestionHandle;
use crate::failures::guard;

/// Free the owned handle; NULL is harmless.
/// # Safety
/// A nonnull handle is live, freed once, with no concurrent borrowers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_free(owner: *mut QuestionHandle) {
    guard(None, (), || {
        if !owner.is_null() {
            // SAFETY: only the allocating library frees this live owner, once.
            drop(unsafe { Box::from_raw(owner) });
        }
    });
}
