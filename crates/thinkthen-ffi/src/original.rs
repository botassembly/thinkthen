//! Original many-record input follows each native row without changing its public prefix.
#![allow(
    unsafe_code,
    reason = "owned source allocations retain original record bytes"
)]
use crate::{HostError, HostRow, values::project};
use thinkthen::native;

pub(crate) fn view(record: &thinkthen::records::Record) -> RecordView {
    let (kind, data) = record.original();
    RecordView {
        kind: kind.abi(),
        data: data.as_ptr(),
        len: data.len(),
    }
}
/// A many-record result's input, as thinkthen_row_original writes it, with its kind and bytes as
/// sent, failed records included. Missing and invalid records have no bytes. The bytes may
/// not be UTF-8, so this is not a thinkthen_text.
#[repr(C)]
#[derive(Debug)]
pub struct RecordView {
    /// How the record was sent, a thinkthen_record_kind.
    pub kind: u32,
    /// The record's bytes, borrowed from the row or value.
    pub data: *const u8,
    /// The byte length; not NUL-terminated.
    pub len: usize,
}

/// Write the original input of a many-record row to *out.
///
/// *out borrows from the row and lasts until thinkthen_row_free(row), even after the call and
/// client are freed; never free it. Returns THINKTHEN_OK with *out written, or a negative code
/// with *out unchanged: -THINKTHEN_CODE_USAGE_ABI when the row has no retained input or
/// did not come from this library.
/// # Safety
/// Any thread may call this while no thread changes or frees row. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_original(
    row: *const HostRow,
    out: *mut RecordView,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends an owned row prefix and exclusive output slots. */
    unsafe {
        project(out, error, || {
            let crate::values::Owned::Row(row) = &crate::row::data(row)?.value.0 else {
                return Err(native::abi_error());
            };
            row.original_record()
                .map(view)
                .ok_or_else(native::abi_error)
        })
    }
}
