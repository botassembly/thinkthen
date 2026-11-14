//! Native enum domains use the shared question builder's sole admission rules.
#![allow(
    unsafe_code,
    reason = "domain setters borrow counted foreign word arrays"
)]
use crate::{
    HostError, QuestionHandle,
    exports::{THINKTHEN_OK, operation},
    memory::{live, live_mut, text},
};
use thinkthen::{FunctionName, native};
unsafe fn words<'a>(
    data: *const *const u8,
    count: usize,
) -> Result<Vec<&'a str>, thinkthen::Error> {
    if count > 0 && data.is_null() {
        return Err(native::abi_error());
    }
    // SAFETY: callers lend count readable word pointers and NUL-terminated strings.
    unsafe {
        (0..count)
            .map(|i| {
                let word = data.add(i).read();
                if word.is_null() {
                    return Err(native::argument_error(thinkthen::Code::UsageQuestion));
                }
                text(word, usize::MAX)
            })
            .collect()
    }
}
unsafe fn set_words(
    question: *mut QuestionHandle,
    data: *const *const u8,
    count: usize,
    error: *mut HostError,
    function: FunctionName,
) -> i32 {
    // SAFETY: the caller lends word pointers and an exclusive builder.
    unsafe {
        operation(error, || {
            live_mut(question)?
                .builder
                .set_items(function, &words(data, count)?)?;
            Ok(THINKTHEN_OK)
        })
    }
}
/// Copy the complete ordered word array into a question.
/// # Safety
/// question is exclusive; data holds count readable NUL-terminated strings;
/// error is NULL or initialized. Inputs are copied before return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_choices(
    question: *mut QuestionHandle,
    data: *const *const u8,
    count: usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: set_words admits and copies the borrowed array.
    unsafe { set_words(question, data, count, error, FunctionName::Choose) }
}
/// Copy the complete ordered word array into a question.
/// # Safety
/// question is exclusive; data holds count readable NUL-terminated strings;
/// error is NULL or initialized. Inputs are copied before return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_labels(
    question: *mut QuestionHandle,
    data: *const *const u8,
    count: usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: set_words admits and copies the borrowed array.
    unsafe { set_words(question, data, count, error, FunctionName::Tag) }
}
/// Copy the complete ordered word array into a question.
/// # Safety
/// question is exclusive; data holds count readable NUL-terminated strings;
/// error is NULL or initialized. Inputs are copied before return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_set_levels(
    question: *mut QuestionHandle,
    data: *const *const u8,
    count: usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: set_words admits and copies the borrowed array.
    unsafe { set_words(question, data, count, error, FunctionName::Score) }
}
/// Check a saved question's complete word order against a caller's enum.
/// # Safety
/// question is live; data holds count readable NUL-terminated words; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_expect_choices(
    question: *const QuestionHandle,
    data: *const *const u8,
    count: usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the builder and strings remain borrowed until the comparison completes.
    unsafe {
        operation(error, || {
            live(question)?.builder.expect_items(&words(data, count)?)?;
            Ok(THINKTHEN_OK)
        })
    }
}
