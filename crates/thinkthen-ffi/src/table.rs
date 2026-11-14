//! Table hosts describe their generated columns; the engine owns collision admission.
#![allow(
    unsafe_code,
    reason = "table admission borrows counted foreign text arrays"
)]
use crate::{
    HostError, QuestionHandle, TextView,
    exports::{THINKTHEN_OK, operation},
    memory::{live, text},
};
use thinkthen::{Error, native};

unsafe fn names<'a>(values: *const TextView, count: usize) -> Result<Vec<&'a str>, Error> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if values.is_null() {
        return Err(native::abi_error());
    }
    /* SAFETY: the caller lends count initialized text views until return. */
    unsafe { std::slice::from_raw_parts(values, count) }
        .iter()
        .map(|value| {
            /* SAFETY: each view lends its counted UTF-8 bytes until return. */
            unsafe { text(value.data, value.len) }
        })
        .collect()
}

/// Validate annotation names against a table host's fixed columns and per-question suffixes.
/// No request is sent. Returns THINKTHEN_OK, or a negative shared error code naming a clash.
/// # Safety
/// question is live and not concurrently changed. Each array points to count readable
/// thinkthen_text values, or is NULL when count is zero. Their bytes remain readable until
/// return. error is NULL or initialized with THINKTHEN_ERROR_INIT and exclusively writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_validate_table_columns(
    question: *const QuestionHandle,
    columns: *const TextView,
    column_count: usize,
    suffixes: *const TextView,
    suffix_count: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: all foreign inputs are borrowed only until this operation returns. */
    unsafe {
        operation(error, || {
            live(question)?.builder.validate_table_columns(
                &names(columns, column_count)?,
                &names(suffixes, suffix_count)?,
            )?;
            Ok(THINKTHEN_OK)
        })
    }
}
