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
/// UTF-8 text as a pointer and a byte length, not NUL-terminated. Text passed in is copied;
/// text written out borrows from the value it came from.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextView {
    /// The bytes; NULL only when len is 0.
    pub data: *const u8,
    /// The byte length, or THINKTHEN_NUL_TERMINATED for text passed in.
    pub len: usize,
}
/// One relation rule for thinkthen_question_add_rule or thinkthen_relate; copied before return.
#[repr(C)]
#[derive(Debug)]
pub struct RuleInput {
    /// The relation's name, such as "works_for".
    pub relation: TextView,
    /// The kind of name the relation starts from.
    pub source: TextView,
    /// The kind of name it points to.
    pub target: TextView,
    /// How the model should read the relation; data NULL to leave it out.
    pub reads: TextView,
    /// 1 when the relation runs both ways, 0 when it runs from source to target.
    pub either: u8,
}
/// Start a question that asks function about text.
///
/// function is a thinkthen_function. recognize, relate and annotate take empty text; every
/// other function needs a question text. Returns THINKTHEN_OK with *out set to a new question,
/// or a negative code with *out set to NULL: -THINKTHEN_CODE_USAGE_FUNCTION for an unknown
/// function, -THINKTHEN_CODE_USAGE_QUESTION for blank text, or text given to a function that
/// takes none, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out
/// and frees it with thinkthen_question_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
/// when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            let function = FunctionName::from_abi(function)
                .ok_or_else(|| thinkthen::native::argument_error(thinkthen::Code::UsageFunction))?;
            let question = native::QuestionBuilder::new(function, text(data, len)?)?;
            out.write(Box::into_raw(Box::new(QuestionHandle::new(question))));
            Ok(OK)
        })
    }
}
/// Add one choice to a choose question.
///
/// Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is not choose, or the choice is blank or
/// repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
                .builder
                .add_item(FunctionName::Choose, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add one label to a tag question.
///
/// Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is not tag, or the label is blank or
/// repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
                .builder
                .add_item(FunctionName::Tag, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add one level, lowest first to a score or rank question.
///
/// Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is not score or rank, or the level is blank
/// or repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
                .builder
                .add_item(FunctionName::Score, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add one kind of name to recognize to a recognize question.
///
/// Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is not recognize, or the kind is blank or
/// repeated, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
                .builder
                .add_item(FunctionName::Recognize, text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Add one relation rule to a relate question.
///
/// rule->relation names the relation, rule->source and rule->target the kinds it joins,
/// rule->reads (data NULL to leave it out) how to read it, and rule->either 1 when it runs both
/// ways. Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is not relate or the rule is blank or
/// repeated, -THINKTHEN_CODE_USAGE_ARGUMENT when either is not 0 or 1, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. rule points to a readable rule whose text fields are
/// readable; they are copied. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_add_rule(
    question: *mut QuestionHandle,
    rule: *const RuleInput,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends the complete rule and its counted fields until return. */
    unsafe {
        operation(error, || {
            (*question).builder.add_rule(relation_rule(&*rule)?)?;
            Ok(OK)
        })
    }
}
/// Copy one borrowed rule through the engine's rule admission.
pub(crate) unsafe fn relation_rule(
    rule: &RuleInput,
) -> Result<questions::RelationRule, thinkthen::Error> {
    if rule.either > 1 {
        return Err(native::argument_error(thinkthen::Code::UsageArgument));
    }
    /* SAFETY: the caller lends each counted rule field until return. */
    unsafe {
        let mut value = questions::RelationRule::new(
            text(rule.relation.data, rule.relation.len)?,
            text(rule.source.data, rule.source.len)?,
            text(rule.target.data, rule.target.len)?,
        )?
        .either(rule.either != 0);
        if !rule.reads.data.is_null() {
            value = value.reads(text(rule.reads.data, rule.reads.len)?)?;
        }
        Ok(value)
    }
}
/// Set the cut a probability must reach for a yes or a choice.
///
/// Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_THRESHOLD when cut is outside 0 to 1 or the function takes no cut.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_cut(
    question: *mut QuestionHandle,
    cut: f64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder and output are exclusively owned. */
    unsafe {
        operation(error, || {
            (*question).builder.set_cut(cut)?;
            Ok(OK)
        })
    }
}
/// Set a band of probabilities, from low to high, that gives THINKTHEN_UNSURE.
///
/// Only decide, filter and choose take a band. Returns THINKTHEN_OK, or a negative code with
/// the question unchanged: -THINKTHEN_CODE_USAGE_THRESHOLD when the band is out of order,
/// outside 0 to 1, or the function takes none.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
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
            (*question).builder.set_band(low, high)?;
            Ok(OK)
        })
    }
}
/// Point the question at the part of a JSON record to read, as a JSON Pointer.
///
/// Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_POINTER when data is not a JSON Pointer, or -THINKTHEN_CODE_USAGE_ABI
/// when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_field(
    question: *mut QuestionHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver owns the builder and lends counted bytes until return. */
    unsafe {
        operation(error, || {
            (*question).builder.set_field(text(data, len)?)?;
            Ok(OK)
        })
    }
}
/// Keep only the top answered rows of a rank question; failed rows are always kept.
///
/// Returns THINKTHEN_OK, or a negative code with the question unchanged:
/// -THINKTHEN_CODE_USAGE_QUESTION when the question is not rank or top is 0.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_top(
    question: *mut QuestionHandle,
    top: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder and output are exclusively owned. */
    unsafe {
        operation(error, || {
            (*question).builder.set_top(top)?;
            Ok(OK)
        })
    }
}
/// Add a copy of child to an annotate question under name.
///
/// Changing or freeing child afterwards does not change the annotation. Returns THINKTHEN_OK,
/// or a negative code with the question unchanged: -THINKTHEN_CODE_USAGE_QUESTION when the
/// question is not annotate, child is rank or annotate, or child is incomplete;
/// -THINKTHEN_CODE_USAGE_NAME when name is blank or repeated; or -THINKTHEN_CODE_USAGE_ABI when
/// the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. child is a live question no thread changes during the
/// call. name points to len readable bytes, or to a NUL-terminated string when len is
/// THINKTHEN_NUL_TERMINATED; it is copied before return. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
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
                .builder
                .add_entry(text(name, len)?, (*child).build()?)?;
            Ok(OK)
        })
    }
}
/// Free a question.
///
/// Free a question made by any thinkthen_question_* function. NULL is ignored. Calls, toolboxes
/// and annotations made from it keep their own copies.
/// # Safety
/// Call this once, after every other use of question has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_free(question: *mut QuestionHandle) {
    if !question.is_null() {
        /* SAFETY: the caller transfers the unique allocation. */
        drop(unsafe { Box::from_raw(question) });
    }
}

/// Copy a complete question into a new question that changes on its own.
///
/// Returns THINKTHEN_OK with *out set to a new question, or a negative code with *out set to
/// NULL: -THINKTHEN_CODE_USAGE_QUESTION when source is incomplete. The caller owns *out and
/// frees it with thinkthen_question_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes source. out is writable. error is NULL or
/// initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
/// passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            let question = (*source).build()?;
            out.write(Box::into_raw(Box::new(QuestionHandle::new(
                native::QuestionBuilder::snapshot(&question),
            ))));
            Ok(OK)
        })
    }
}

/// Read one question from its JSON definition.
///
/// Returns THINKTHEN_OK with *out set to a new question, or a negative code with *out set to
/// NULL: -THINKTHEN_CODE_USAGE_QUESTION when the JSON is not a valid question, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it with
/// thinkthen_question_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
/// when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            out.write(Box::into_raw(Box::new(QuestionHandle::new(
                native::QuestionBuilder::snapshot(&question),
            ))));
            Ok(OK)
        })
    }
}
