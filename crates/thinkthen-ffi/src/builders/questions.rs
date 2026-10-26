//! Typed question setters retain their previous state on rejection.
#![allow(
    unsafe_code,
    reason = "typed exports borrow foreign builders and counted fields"
)]
use crate::{
    HostError, QuestionHandle,
    exports::{OK, operation},
    memory::text,
};
use std::ptr;
use thinkthen::{FunctionName, native, questions};
/// Counted borrowed UTF-8. Setters copy input; projections borrow their value owner.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextView {
    /// Readable bytes, NULL only when length is zero.
    pub data: *const u8,
    /// Byte length without terminating NUL.
    pub len: usize,
}
/// A typed borrowed relation rule. A NULL reads pointer means omitted wording.
#[repr(C)]
#[derive(Debug)]
pub struct RuleInput {
    /// Relation name.
    pub relation: TextView,
    /// Source kind.
    pub source: TextView,
    /// Target kind.
    pub target: TextView,
    /// Optional explicit wording.
    pub reads: TextView,
    /// Zero for directed, one for unordered.
    pub either: u8,
}
/// Start one typed question; domain-less functions require empty text.
/// # Safety
/// Counted text is readable; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_new(
    function: u32,
    data: *const u8,
    len: usize,
    out: *mut *mut QuestionHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: borrowed inputs and exclusive outputs are live until return. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let function = FunctionName::from_abi(function).ok_or_else(native::abi_error)?;
            let question = native::QuestionBuilder::new(function, text(data, len)?)?;
            out.write(Box::into_raw(Box::new(QuestionHandle(question))));
            Ok(OK)
        })
    }
}
/// Add a copied choose choice, leaving the builder unchanged on rejection.
/// # Safety
/// One driver owns the builder and empty error slot; text is counted readable UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_add_choice(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder is exclusive and borrowed bytes are live until return. */
    unsafe {
        operation(error, || {
            (*question)
                .0
                .add_item(FunctionName::Choose, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add a copied tag label.
/// # Safety
/// One driver owns the builder and error; text is counted readable UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_add_label(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder is exclusive and counted bytes remain readable. */
    unsafe {
        operation(error, || {
            (*question)
                .0
                .add_item(FunctionName::Tag, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add a copied score or rank level.
/// # Safety
/// One driver owns the builder and error; text is counted readable UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_add_level(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder is exclusive and counted bytes remain readable. */
    unsafe {
        operation(error, || {
            (*question)
                .0
                .add_item(FunctionName::Score, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add a copied recognition kind.
/// # Safety
/// One driver owns the builder and error; text is counted readable UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_add_kind(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder is exclusive and counted bytes remain readable. */
    unsafe {
        operation(error, || {
            (*question)
                .0
                .add_item(FunctionName::Recognize, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add one fully admitted relation rule copied from borrowed fields.
/// # Safety
/// Builder and rule fields are live; error is empty exclusive storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_add_rule(
    question: *mut QuestionHandle,
    rule: *const RuleInput,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends the complete rule and its counted fields until return. */
    unsafe {
        operation(error, || {
            let rule = &*rule;
            if rule.either > 1 {
                return Err(native::abi_error());
            }
            let mut value = questions::RelationRule::new(
                text(rule.relation.data, rule.relation.len)?,
                text(rule.source.data, rule.source.len)?,
                text(rule.target.data, rule.target.len)?,
            )?
            .either(rule.either != 0);
            if !rule.reads.data.is_null() {
                value = value.reads(text(rule.reads.data, rule.reads.len)?)?;
            }
            (*question).0.add_rule(value)?;
            Ok(OK)
        })
    }
}
/// Set a validated numeric cut for a threshold-taking function.
/// # Safety
/// A sole driver owns the builder and empty error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_cut(
    question: *mut QuestionHandle,
    cut: f64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder and output are exclusively owned. */
    unsafe {
        operation(error, || {
            (*question).0.set_cut(cut)?;
            Ok(OK)
        })
    }
}
/// Set a band for decide, filter or choose; numeric-only functions reject it.
/// # Safety
/// A sole driver owns the builder and empty error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_band(
    question: *mut QuestionHandle,
    low: f64,
    high: f64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder and output are exclusively owned. */
    unsafe {
        operation(error, || {
            (*question).0.set_band(low, high)?;
            Ok(OK)
        })
    }
}
/// Copy an admitted evidence pointer.
/// # Safety
/// A sole driver owns the builder and empty error; text is counted readable UTF-8.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_evidence(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver owns the builder and lends counted bytes until return. */
    unsafe {
        operation(error, || {
            (*question).0.set_evidence(text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Bound successful rank output, retaining failed rows.
/// # Safety
/// A sole driver owns the builder and empty error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_top(
    question: *mut QuestionHandle,
    top: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder and output are exclusively owned. */
    unsafe {
        operation(error, || {
            (*question).0.set_top(top)?;
            Ok(OK)
        })
    }
}
/// Snapshot one eligible child into annotation; rank and annotate are rejected.
/// # Safety
/// Both builders and counted name are live; error is exclusive and empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_add_entry(
    question: *mut QuestionHandle,
    name: *const u8,
    len: usize,
    child: *const QuestionHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: only the parent is mutated; no borrowed child or bytes are retained. */
    unsafe {
        operation(error, || {
            (*question)
                .0
                .add_entry(text(name, len)?, (*child).0.build()?)?;
            Ok(OK)
        })
    }
}
/// Release a question; calls, toolboxes and annotations retain owned snapshots.
/// # Safety
/// NULL or the unique allocation after all readers finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_free(question: *mut QuestionHandle) {
    if !question.is_null() {
        /* SAFETY: the caller transfers the unique allocation. */
        drop(unsafe { Box::from_raw(question) });
    }
}

/// Copy a complete admitted question into an independent mutable builder.
/// # Safety
/// Source is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_clone(
    source: *const QuestionHandle,
    out: *mut *mut QuestionHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends a live source and exclusive outputs. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let question = (*source).0.build()?;
            out.write(Box::into_raw(Box::new(QuestionHandle(
                native::QuestionBuilder::snapshot(&question),
            ))));
            Ok(OK)
        })
    }
}

/// Admit one canonical question through the shared grammar without executing it.
/// # Safety
/// Counted UTF-8 and distinct empty output slots remain live until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_from_json(
    data: *const u8,
    len: usize,
    out: *mut *mut QuestionHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends counted bytes and exclusive outputs. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let question = thinkthen::Question::from_json(text(data, len)?)?;
            out.write(Box::into_raw(Box::new(QuestionHandle(
                native::QuestionBuilder::snapshot(&question),
            ))));
            Ok(OK)
        })
    }
}
