//! Ownership and counted foreign views; no execution rules live here.
#![allow(
    unsafe_code,
    reason = "this module owns counted foreign allocations and handles"
)]

use std::cell::UnsafeCell;
use std::ptr;
use thinkthen::{Cancel, Client, Error, ErrorKind, PullCall, PullRow};

/// An owned buffer whose allocator is reached only through the C cleanup call.
#[derive(Debug)]
#[repr(C)]
pub struct Buffer {
    /// Counted bytes, without a terminating NUL.
    pub data: *mut u8,
    /// Length of the allocation.
    pub len: usize,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            data: ptr::null_mut(),
            len: 0,
        }
    }
}

impl Buffer {
    pub(crate) fn new(text: String) -> Self {
        let bytes = text.into_bytes().into_boxed_slice();
        Self {
            len: bytes.len(),
            data: Box::into_raw(bytes).cast(),
        }
    }

    pub(crate) unsafe fn release(&mut self) {
        if !self.data.is_null() {
            let bytes = ptr::slice_from_raw_parts_mut(self.data, self.len);
            // SAFETY: the caller transfers the unique allocation made by new.
            drop(unsafe { Box::from_raw(bytes) });
            *self = Self::default();
        }
    }
}

/// One independently owned host row, with detail left unparsed.
#[derive(Debug, Default)]
#[repr(C)]
pub struct HostRow {
    /// Contract row JSON without top-level detail.
    pub fields: Buffer,
    /// Detail JSON, or an empty buffer when absent.
    pub detail: Buffer,
    /// Original input position, starting at one.
    pub position: usize,
    /// The shared engine's selection result as zero or one.
    pub kept: u8,
}

impl From<PullRow> for HostRow {
    fn from(row: PullRow) -> Self {
        Self {
            fields: Buffer::new(row.json),
            detail: row.detail.map(Buffer::new).unwrap_or_default(),
            position: row.position,
            kept: u8::from(row.kept),
        }
    }
}

/// Opaque owner of an admitted shared client.
#[derive(Debug)]
pub struct ClientHandle(pub(crate) Client);

/// Opaque owner with a separately shared cancellation token.
#[derive(Debug)]
pub struct CallHandle {
    pub(crate) call: UnsafeCell<PullCall>,
    pub(crate) cancel: Cancel,
}

pub(crate) unsafe fn text<'a>(data: *const u8, len: usize) -> Result<&'a str, Error> {
    // SAFETY: the header requires readable bytes for len, or NULL only at zero.
    let bytes = if len == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(data, len) }
    };
    std::str::from_utf8(bytes)
        .map_err(|_| Error::new(ErrorKind::Record, "the input is not valid UTF-8"))
}

#[expect(
    clippy::expect_used,
    reason = "the library Error serializer contains only infallible UTF-8 string fields"
)]
pub(crate) fn error_buffer(error: &Error) -> Buffer {
    Buffer::new(serde_json::to_string(error).expect("Error serializes as UTF-8 strings"))
}
