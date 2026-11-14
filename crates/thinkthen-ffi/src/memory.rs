//! Native ownership, sized output prefixes and counted borrowed inputs.
#![allow(
    unsafe_code,
    reason = "the ABI owns foreign pointers and allocated output prefixes"
)]
use std::{
    cell::UnsafeCell,
    mem::{ManuallyDrop, offset_of, size_of},
    ptr,
};
use thinkthen::{
    Cancel, Engine, Error, Settings,
    native::{self, QuestionBuilder, ToolboxBuilder},
};
/// Bytes the library allocated for the caller. Free them with thinkthen_buffer_free, or with
/// the free function of the error or row that holds the buffer.
#[repr(C)]
#[derive(Debug)]
pub struct Buffer {
    /// UTF-8 bytes, not NUL-terminated; NULL when empty.
    pub data: *mut u8,
    /// The byte length.
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
    const PREFIX: usize = size_of::<usize>();
    pub(crate) fn new(text: String) -> Self {
        #[cfg(test)]
        LIVE.with(|live| live.set(live.get() + 1));
        let len = text.len();
        let mut allocation = Vec::from(len.to_ne_bytes());
        allocation.extend_from_slice(text.as_bytes());
        allocation.push(0);
        let base = Box::into_raw(allocation.into_boxed_slice()).cast::<u8>();
        /* SAFETY: PREFIX leaves room for one unaligned optional owner pointer. */
        let data = unsafe { base.add(Self::PREFIX) };
        Self { data, len }
    }
    pub(crate) unsafe fn release(&mut self) {
        if !self.data.is_null() {
            #[cfg(test)]
            LIVE.with(|live| live.set(live.get() - 1));
            /* SAFETY: cleanup accepts only unique allocations from new. The full
             * boxed slice length is the stored length plus the fixed prefix. */
            unsafe {
                let base = self.data.sub(Self::PREFIX);
                let len = ptr::read_unaligned(base.cast::<usize>());
                drop(Box::from_raw(ptr::slice_from_raw_parts_mut(
                    base,
                    len + Self::PREFIX + 1,
                )));
            }
            *self = Self::default();
        }
    }
}
#[cfg(test)]
thread_local! {
    /// Engine buffers and row payloads this thread made and has not freed.
    pub(crate) static LIVE: std::cell::Cell<isize> = const { std::cell::Cell::new(0) };
}
/// Why a function failed. Initialize it with THINKTHEN_ERROR_INIT and free it with
/// thinkthen_error_free; passing it again frees what it held.
#[repr(C)]
#[cfg_attr(test, repr(align(256)))]
#[derive(Debug)]
pub struct HostError {
    /// The caller's allocation size, preserved by cleanup.
    pub struct_size: u32,
    /// The thinkthen_code, or 0 when there is no error.
    pub code: i32,
    /// The backend's HTTP status, or 0 when there is none.
    pub http_status: u16,
    /// The message, owned by the error; it never holds a key.
    pub message: Buffer,
}
impl HostError {
    pub(crate) fn empty(size: u32) -> Self {
        Self {
            struct_size: size,
            code: 0,
            http_status: 0,
            message: Buffer::default(),
        }
    }
    /// Free the message an initialized error still holds, then empty it.
    pub(crate) unsafe fn clear(out: *mut Self, size: u32) {
        /* SAFETY: callers validated a supported prefix that owns its message. */
        unsafe {
            (*ptr::addr_of_mut!((*out).message)).release();
            Self::empty(size).write(out);
        }
    }
    pub(crate) fn from_error(size: u32, error: &Error) -> Self {
        // One allocation owns both the exact counted message and its C display string.
        let mut text = error.message().to_owned();
        let message_len = text.len();
        text.push('\0');
        text.push_str(
            &format!("{}: {}", error.code().as_str(), error.message()).replace('\0', "\\0"),
        );
        let mut message = Buffer::new(text);
        message.len = message_len;
        Self {
            struct_size: size,
            code: error.code().abi(),
            http_status: error.http_status().unwrap_or(0),
            message,
        }
    }
}
/// One record's result. Initialize it with THINKTHEN_ROW_INIT and free it with
/// thinkthen_row_free; passing it again frees what it held. Read answers with thinkthen_row_answer_*.
#[repr(C)]
#[cfg_attr(test, repr(align(256)))]
#[derive(Debug)]
pub struct HostRow {
    /// The caller's allocation size, preserved by cleanup.
    pub struct_size: u32,
    /// The question's function.
    pub function: u32,
    /// Whether the record was answered, not sure, or failed.
    pub status: u32,
    /// The record's input position, 1 for the first.
    pub position: u64,
    /// The answer's probability, or NaN when its function has none.
    pub probability: f64,
    /// A failed record's thinkthen_code, or 0.
    pub error_code: i32,
    /// Private owned result storage. Never read, change or free this pointer directly.
    pub private_data: *mut crate::row::RowData,
}
impl HostRow {
    pub(crate) fn empty(size: u32) -> Self {
        Self {
            struct_size: size,
            function: 0,
            status: 0,
            position: 0,
            probability: f64::NAN,
            error_code: 0,
            private_data: ptr::null_mut(),
        }
    }
    /// Free what an output row still holds from an earlier result, then empty it,
    /// so reusing one row variable never leaks.
    pub(crate) unsafe fn clear(out: *mut Self) {
        /* SAFETY: callers validated the size; an initialized row owns its buffers. */
        unsafe { Self::release(out) }
    }
    pub(crate) unsafe fn release(out: *mut Self) {
        /* SAFETY: the validated 0.2 prefix owns each distinct buffer independently. */
        unsafe {
            if !(*out).private_data.is_null() {
                drop(Box::from_raw((*out).private_data));
            }
            Self::empty((*out).struct_size).write(out);
        }
    }
}
/// An open client. Any thread may use it, and calls opened from it outlive it. Free it with
/// thinkthen_client_free.
#[derive(Debug)]
pub struct ClientHandle(pub(crate) Engine);
#[derive(Debug, Default)]
pub(crate) struct KeptState {
    pub(crate) failed: std::collections::VecDeque<thinkthen::AnyRow>,
    pub(crate) first: Option<(Error, u64)>,
    pub(crate) failures: usize,
    pub(crate) total: u64,
    pub(crate) finished: bool,
}
/// An open call. One thread drives it at a time; thinkthen_call_cancel may run on any thread.
/// Free it with thinkthen_call_free.
#[derive(Debug)]
pub struct CallHandle {
    pub(crate) call: UnsafeCell<native::Call>,
    pub(crate) cancel: Cancel,
    pub(crate) filter: bool,
    pub(crate) c_surface: bool,
    pub(crate) domain: std::sync::Arc<crate::row::Domain>,
    pub(crate) kept: UnsafeCell<KeptState>,
}
impl CallHandle {
    pub(crate) fn new(call: native::Call, request: &thinkthen::Request) -> Self {
        let cancel = call.cancellation();
        Self {
            call: UnsafeCell::new(call),
            cancel,
            filter: request.question().function() == thinkthen::FunctionName::Filter,
            kept: UnsafeCell::new(KeptState::default()),
            c_surface: false,
            domain: std::sync::Arc::new(crate::row::Domain::from_question(request.question())),
        }
    }
}
/// Settings for a client, copied when it opens. Free them with thinkthen_settings_free.
#[derive(Debug, Default)]
pub struct SettingsHandle(pub(crate) Settings, pub(crate) Option<Cancel>);
/// A question. One thread at a time changes it; once complete, many threads may use it.
/// Free it with thinkthen_question_free.
#[derive(Debug)]
pub struct QuestionHandle {
    pub(crate) builder: QuestionBuilder,
    pub(crate) meanings: crate::builders::meanings::Meanings,
}
impl QuestionHandle {
    pub(crate) fn new(builder: QuestionBuilder) -> Self {
        Self {
            builder,
            meanings: crate::builders::meanings::Meanings::default(),
        }
    }
    /// Admit the complete question, with its answer meanings and descriptions.
    pub(crate) fn build(&self) -> Result<thinkthen::Question, Error> {
        self.meanings.apply(self.builder.build()?)
    }
}
/// A set of named questions, each copied in. Free it with thinkthen_toolbox_free.
#[derive(Debug, Default)]
pub struct ToolboxHandle(pub(crate) ToolboxBuilder);
/// Borrow a caller's handle, or fail with the ABI error when it is NULL.
pub(crate) unsafe fn live<'a, T>(handle: *const T) -> Result<&'a T, Error> {
    /* SAFETY: the caller lends a live handle or NULL until the export returns. */
    unsafe { handle.as_ref() }.ok_or_else(native::abi_error)
}
/// Borrow a handle the caller's one driving thread lends exclusively, as `live` does.
pub(crate) unsafe fn live_mut<'a, T>(handle: *mut T) -> Result<&'a mut T, Error> {
    /* SAFETY: the caller lends a live handle or NULL until the export returns. */
    unsafe { handle.as_mut() }.ok_or_else(native::abi_error)
}
/// Borrow counted input bytes only until the export returns.
/// THINKTHEN_NUL_TERMINATED (SIZE_MAX) as the length reads up to the first NUL.
pub(crate) unsafe fn bytes<'a>(data: *const u8, len: usize) -> &'a [u8] {
    if len == 0 || data.is_null() {
        &[]
    } else if len == usize::MAX {
        /* SAFETY: the caller lends a NUL-terminated string for this length. */
        unsafe { std::ffi::CStr::from_ptr(data.cast()).to_bytes() }
    } else {
        /* SAFETY: the caller lends len readable bytes. */
        unsafe { std::slice::from_raw_parts(data, len) }
    }
}
pub(crate) unsafe fn text<'a>(data: *const u8, len: usize) -> Result<&'a str, Error> {
    /* SAFETY: this call borrows the same counted input. */
    std::str::from_utf8(unsafe { bytes(data, len) }).map_err(|_| native::abi_error())
}
pub(crate) unsafe fn row_size(out: *mut HostRow) -> Result<u32, Error> {
    if out.is_null() {
        return Err(native::abi_error());
    }
    /* SAFETY: a non-NULL output permits reading its first size field. */
    let size = unsafe { out.cast::<u32>().read_unaligned() };
    if size == 0 {
        Err(native::uninitialized_row_error())
    } else if size < HostRow::PREFIX_SIZE {
        Err(native::abi_error())
    } else {
        Ok(size)
    }
}
pub(crate) unsafe fn error_size(out: *mut HostError) -> Option<u32> {
    /* SAFETY: a non-NULL output permits reading its first size field. */
    if out.is_null() || unsafe { out.cast::<u32>().read_unaligned() } < HostError::PREFIX_SIZE {
        None
    } else {
        /* SAFETY: size is readable and covers the supported prefix. */
        Some(unsafe { out.cast::<u32>().read_unaligned() })
    }
}

macro_rules! size_of_val_type {
    (HostRow, private_data) => {
        size_of::<*mut crate::row::RowData>()
    };
    (HostError, message) => {
        size_of::<Buffer>()
    };
}
// The 0.2 fields end here even when a later engine appends fields. Neither the
// minimum accepted size nor output writes follow the engine's full struct size.
macro_rules! prefix_output {
    ($output:ident, $last:ident) => {
        impl $output {
            pub(crate) const PREFIX_SIZE: u32 =
                (offset_of!(Self, $last) + size_of_val_type!($output, $last)) as u32;
            pub(crate) unsafe fn write(self, out: *mut Self) {
                let value = ManuallyDrop::new(self);
                /* SAFETY: callers validate a writable 0.2 prefix; ownership moves to out. */
                unsafe {
                    ptr::copy_nonoverlapping(
                        ptr::from_ref(&*value).cast::<u8>(),
                        out.cast::<u8>(),
                        Self::PREFIX_SIZE as usize,
                    );
                }
            }
        }
    };
}
prefix_output!(HostRow, private_data);
prefix_output!(HostError, message);
