//! Single-record execution uses the same reserved-capacity native driver.
#![allow(
    unsafe_code,
    reason = "the single-record export borrows counted bytes and sized native output slots"
)]
use crate::{
    ClientHandle, HostError, HostRow, QuestionHandle,
    exports::{OK, operation},
    memory::row_size,
};
use thinkthen::{
    OffsetUnit, Request,
    native::{self, Event},
};
/// Evaluate one record; a failed record is an OK row, never an operation error.
/// # Safety
/// Handles and counted input are live; sized row/error outputs are distinct and empty.
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
            out.write(HostRow::empty(size));
            let record = crate::finite::admit_record(data, len, kind)?;
            let request = Request::new((*question).0.build()?);
            let mut call = native::Call::open(&(*client).0, &request, OffsetUnit::CodePoints)?;
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
            let mut result = HostRow::from_row(size, row)?;
            if request.question().function() == thinkthen::FunctionName::Filter {
                result
                    .source
                    .attach_original(crate::original::input(data, len, kind)?);
            }
            out.write(result);
            Ok(OK)
        })
    }
}
