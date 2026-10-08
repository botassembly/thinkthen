//! Additive recognition task inputs and owner-borrowed views (ADR 0124).
#![allow(
    unsafe_code,
    reason = "counted C buffers obey the installed header contract"
)]
#![deny(unsafe_op_in_unsafe_fn)]
use super::{author, read};
use crate::ffi::carriers::{
    OptionalStringV1, QuestionAuthorV1, QuestionSpecV1, RecognitionTaskV1, StringV1,
};
use crate::{
    Door,
    current::{QuestionHandle, question::Native},
    failures::{OK, USAGE, guard},
};

/// Construct a recognition question with copied task wording.
/// # Safety
/// All active counted buffers, optional author and output obey the header contract.
#[unsafe(no_mangle)]
pub(super) unsafe extern "C" fn thinkthen_question_new_recognition_v1(
    engine: *const Door,
    spec: *const QuestionSpecV1,
    metadata: *const QuestionAuthorV1,
    task: *const RecognitionTaskV1,
    out: *mut *mut QuestionHandle,
) -> i32 {
    // SAFETY: validate required task within the engine's ordinary guarded failure path.
    unsafe {
        crate::ffi::typed(
            engine,
            |_| read::reference(task).copied(),
            |_, task| author::new_with_task(engine, spec, metadata, Some(task), out),
        )
    }
}
fn borrowed(value: Option<&str>) -> OptionalStringV1 {
    value.map_or_else(OptionalStringV1::default, |s| OptionalStringV1 {
        present: 1,
        value: StringV1 {
            data: s.as_ptr().cast(),
            len: s.len(),
        },
    })
}
/// Borrow authored task wording from an immutable live recognition question.
/// # Safety
/// Owner and writable output remain live with no concurrent destruction.
#[unsafe(no_mangle)]
pub(super) unsafe extern "C" fn thinkthen_question_recognition_task_v1(
    owner: *const QuestionHandle,
    out: *mut RecognitionTaskV1,
) -> i32 {
    guard(None, USAGE, || {
        if out.is_null() {
            return USAGE;
        }
        // SAFETY: a nonnull caller handle is live under the header contract.
        let Some(owner) = (unsafe { owner.as_ref() }) else {
            return USAGE;
        };
        let Native::Recognize(q) = &owner.native else {
            return USAGE;
        };
        let q = q.reading();
        let task = RecognitionTaskV1 {
            instructions: borrowed(q.instructions()),
            entity_definition: borrowed(q.entity_definition()),
        };
        // SAFETY: publish initialized full-sized output only after successful admission.
        unsafe {
            *out = task;
        }
        OK
    })
}
