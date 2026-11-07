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

/// Run one admitted record through the shared driver and return its row.
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
/// Returns THINKTHEN_OK when the record is answered, THINKTHEN_UNSURE when the model is not
/// sure, or a negative thinkthen_code: the record's own code when it fails (for example
/// -THINKTHEN_CODE_RECORD_EMPTY for blank text, which is never sent), a backend code when
/// the backend fails, or a usage code for a bad argument. Read row.answer_bool, row.number
/// or row.rank only after THINKTHEN_OK. The row is written in every case except a usage
/// error. Passing the same row again frees what it held, so a reused row leaks nothing;
/// free it with thinkthen_row_free after its last use.
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
            let filter = question.function() == thinkthen::FunctionName::Filter;
            let row = evaluate(&live(client)?.0, question, record)?;
            let status = status(&row);
            let mut result = HostRow::from_row(size, row)?;
            if filter {
                result
                    .source
                    .attach_original(crate::original::input(data, len, kind)?);
            }
            result.write(out);
            status
        })
    }
}
