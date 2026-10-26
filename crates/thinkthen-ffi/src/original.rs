//! Filter input ownership follows each native row without changing its public prefix.
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
/// The original filter input. Bytes retain text/JSON encoding even for failed rows.
/// Missing and explicit conversion-failure inputs have empty bytes. Invalid UTF-8
/// text or JSON still has its original kind and bytes, so this is not a text view.
#[repr(C)]
#[derive(Debug)]
pub struct RecordView {
    /// Generated record-kind discriminator.
    pub kind: u32,
    /// Borrowed bytes owned by the row or typed value snapshot.
    pub data: *const u8,
    /// Count without a terminating NUL.
    pub len: usize,
}

/// Borrow the original filter input after call/client destruction, until row cleanup.
/// # Safety
/// Row is an engine-produced supported prefix with a live source allocation;
/// out/error are distinct empty writable slots. Immutable readers may run concurrently.
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
            if (*row).function != FunctionName::Filter.abi() {
                return Err(native::abi_error());
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
    let kind = RecordKind::from_abi(kind).ok_or_else(native::abi_error)?;
    /* SAFETY: text and JSON inputs borrow readable counted bytes until return. */
    let data = if matches!(kind, RecordKind::Text | RecordKind::Json) {
        unsafe { bytes(data, len) }
    } else {
        &[]
    };
    Ok(Original::new(kind, data))
}
