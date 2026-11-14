//! Single-record execution uses the same reserved-capacity native driver.
#![allow(
    unsafe_code,
    reason = "the single-record export borrows counted bytes and sized native output slots"
)]
use crate::{
    ClientHandle, HostError, HostRow, QuestionHandle,
    exports::{THINKTHEN_OK, operation},
    memory::{live, row_size},
};
use thinkthen::{
    Engine, Error, OffsetUnit, Question, Request,
    native::{self, Event},
    records::Record,
    results::{AnyRow, Status},
};

/// Run one admitted record, promoting only its own failure at the call.
pub(crate) fn evaluate(
    client: &Engine,
    question: Question,
    record: Record,
) -> Result<AnyRow, Error> {
    let request = Request::new(question);
    let mut call = native::Call::open(client, &request, OffsetUnit::Utf8)?;
    if !matches!(call.next_event()?, Event::NeedsInput) {
        return Err(native::internal_error());
    }
    call.send(record)?;
    call.end_input();
    let Event::Row(row) = call.next_event()? else {
        return Err(native::internal_error());
    };
    if !matches!(call.next_event()?, Event::Finished) {
        return Err(native::internal_error());
    }
    if let Some(error) = row.error() {
        return Err(error.clone());
    }
    Ok(row)
}

/// The status a single-record call returns for a row: OK, UNSURE or the failure.
pub(crate) fn status(row: &AnyRow) -> Result<i32, Error> {
    match (row.status(), row.error()) {
        (_, Some(error)) => Err(error.clone()),
        (Status::Unsure, None) => Ok(crate::THINKTHEN_UNSURE),
        _ => Ok(THINKTHEN_OK),
    }
}

/// Answer one question about one record and write its whole row to *out.
///
/// Returns THINKTHEN_OK with an answered or unsure row. A failed parent returns its
/// negative code with the output empty; answered forms keep failed entries. Read an
/// unsure row with thinkthen_row_answer_* to get THINKTHEN_UNSURE. Filter and rank
/// require a many-record call and are refused with usage.function.
/// # Safety
/// Any thread may call this, and threads may share the client and the question while no
/// thread changes the question. data points to len readable bytes (or a NUL-terminated
/// string with THINKTHEN_NUL_TERMINATED). out is initialized with THINKTHEN_ROW_INIT or
/// freed after its last use; error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_run_one(
    client: *const ClientHandle,
    question: *const QuestionHandle,
    data: *const u8,
    len: usize,
    kind: u32,
    out: *mut HostRow,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: inputs remain live through their copy and outputs are exclusive sized slots. */
    unsafe {
        operation(error, || {
            let size = row_size(out)?;
            HostRow::clear(out);
            let record = crate::finite::admit_record(data, len, kind)?;
            let question = live(question)?.build()?;
            if matches!(
                question.function(),
                thinkthen::FunctionName::Filter | thinkthen::FunctionName::Rank
            ) {
                return Err(native::argument_error(thinkthen::Code::UsageFunction));
            }
            let domain = std::sync::Arc::new(crate::row::Domain::from_question(&question));
            let row = evaluate(&live(client)?.0, question, record)?;
            let result =
                HostRow::from_value(size, crate::values::ValueHandle::from_any(row), domain)?;
            result.write(out);
            Ok(THINKTHEN_OK)
        })
    }
}
