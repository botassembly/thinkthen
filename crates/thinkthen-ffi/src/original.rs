//! Filter and Rank input ownership follows each native row without changing its public prefix.
#![allow(
    unsafe_code,
    reason = "owned source allocations retain original record bytes"
)]
use crate::{
    HostError, HostRow,
    memory::{bytes, row_size},
    values::project,
};
use thinkthen::{FunctionName, Request, native, records::RecordKind};

#[derive(Debug, Clone)]
pub(crate) struct Original {
    pub(crate) kind: u32,
    pub(crate) data: Vec<u8>,
}
impl Original {
    pub(crate) fn new(kind: RecordKind, data: &[u8]) -> Self {
        Self {
            kind: kind.abi(),
            data: data.to_owned(),
        }
    }
    pub(crate) fn view(&self) -> RecordView {
        RecordView {
            kind: self.kind,
            data: self.data.as_ptr(),
            len: self.data.len(),
        }
    }
    pub(crate) fn initial(request: &Request) -> Vec<Self> {
        if request.question().function() != FunctionName::Filter {
            return Vec::new();
        }
        request
            .records()
            .iter()
            .map(|value| match value {
                serde_json::Value::Null => Self::new(RecordKind::Missing, &[]),
                serde_json::Value::String(text) => Self::new(RecordKind::Text, text.as_bytes()),
                value => Self::new(RecordKind::Json, value.to_string().as_bytes()),
            })
            .collect()
    }
}
/// A filter or rank record's input, as thinkthen_row_original writes it, with its kind and bytes as
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

/// Write the input of a filter or rank row to *out.
///
/// *out borrows from the row and lasts until thinkthen_row_free(row), even after the call and
/// client are freed; never free it. Returns THINKTHEN_OK with *out written, or a negative code
/// with *out unchanged: -THINKTHEN_CODE_USAGE_FUNCTION when the row is not a filter or rank row, or
/// -THINKTHEN_CODE_USAGE_ABI when the row did not come from this library.
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
            row_size(row.cast_mut())?;
            if ![FunctionName::Filter.abi(), FunctionName::Rank.abi()].contains(&(*row).function) {
                return Err(native::argument_error(thinkthen::Code::UsageFunction));
            }
            (*row)
                .source
                .original()
                .map(Original::view)
                .ok_or_else(native::abi_error)
        })
    }
}

pub(crate) unsafe fn input(
    data: *const u8,
    len: usize,
    kind: u32,
) -> Result<Original, thinkthen::Error> {
    let kind = RecordKind::from_abi(kind)
        .ok_or_else(|| thinkthen::native::argument_error(thinkthen::Code::UsageRecordKind))?;
    /* SAFETY: text and JSON inputs borrow readable counted bytes until return. */
    let data = if matches!(kind, RecordKind::Text | RecordKind::Json) {
        unsafe { bytes(data, len) }
    } else {
        &[]
    };
    Ok(Original::new(kind, data))
}

pub(crate) fn attach_rank(row: &mut HostRow, record: &thinkthen::records::Record) {
    let (kind, data) = record.original();
    // SAFETY: from_row exclusively owns a fresh engine-produced source allocation.
    unsafe {
        row.source.attach_original(Original::new(kind, data));
    }
}
