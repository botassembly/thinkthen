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
    // Every engine allocation has the same private pointer prefix. Only source
    // allocations attach an Original. Cleanup never probes foreign allocations.
    const PREFIX: usize = size_of::<*mut crate::original::Original>();
    pub(crate) fn new(text: String) -> Self {
        #[cfg(test)]
        LIVE.with(|live| live.set(live.get() + 1));
        let mut allocation = vec![0_u8; Self::PREFIX];
        allocation.extend_from_slice(text.as_bytes());
        let len = text.len();
        let base = Box::into_raw(allocation.into_boxed_slice()).cast::<u8>();
        /* SAFETY: PREFIX leaves room for one unaligned optional owner pointer. */
        let data = unsafe { base.add(Self::PREFIX) };
        Self { data, len }
    }
    pub(crate) unsafe fn attach_original(&mut self, original: crate::original::Original) {
        /* SAFETY: only an exclusive source allocation created above reaches this path. */
        unsafe {
            let slot = self
                .data
                .sub(Self::PREFIX)
                .cast::<*mut crate::original::Original>();
            debug_assert!(ptr::read_unaligned(slot).is_null());
            ptr::write_unaligned(slot, Box::into_raw(Box::new(original)));
        }
    }
    pub(crate) unsafe fn original(&self) -> Option<&crate::original::Original> {
        if self.data.is_null() {
            return None;
        }
        /* SAFETY: a live engine source allocation has this private pointer prefix. */
        unsafe {
            let owner = ptr::read_unaligned(
                self.data
                    .sub(Self::PREFIX)
                    .cast::<*mut crate::original::Original>(),
            );
            owner.as_ref()
        }
    }
    pub(crate) unsafe fn release(&mut self) {
        if !self.data.is_null() {
            #[cfg(test)]
            LIVE.with(|live| live.set(live.get() - 1));
            /* SAFETY: cleanup accepts only unique allocations from new. The full
             * boxed slice length is the visible length plus the fixed prefix. */
            unsafe {
                let base = self.data.sub(Self::PREFIX);
                let owner = ptr::read_unaligned(base.cast::<*mut crate::original::Original>());
                if !owner.is_null() {
                    drop(Box::from_raw(owner));
                }
                drop(Box::from_raw(ptr::slice_from_raw_parts_mut(
                    base,
                    self.len + Self::PREFIX,
                )));
            }
            *self = Self::default();
        }
    }
}
#[cfg(test)]
thread_local! {
    /// Engine buffers this thread made and has not freed, so a test can see a leak.
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
        Self {
            struct_size: size,
            code: error.code().abi(),
            http_status: error.http_status().unwrap_or(0),
            message: Buffer::new(error.message().to_owned()),
        }
    }
}
/// One record's result. Initialize it with THINKTHEN_ROW_INIT and free it with
/// thinkthen_row_free; passing it again frees what it held. Read the answer fields only when
/// status is THINKTHEN_STATUS_ANSWERED, or use thinkthen_row_values for typed access.
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
    /// A decide or filter answer: 1 for yes, 0 for no. 0 also when not answered.
    pub answer_bool: u8,
    /// A rank answer, 1 for the first.
    pub rank: u64,
    /// A score answer, from 0 to 1.
    pub number: f64,
    /// The record's id as JSON, or empty.
    pub id: Buffer,
    /// The answer as JSON; empty when not sure or failed.
    pub answer: Buffer,
    /// The answer's detail as JSON, or empty.
    pub detail: Buffer,
    /// Which question, backend and model gave the row, as JSON.
    pub source: Buffer,
    /// A failed record's error as JSON, or empty.
    pub error: Buffer,
}
impl HostRow {
    pub(crate) unsafe fn read(out: *const Self) -> Self {
        let mut value = Self::empty(Self::PREFIX_SIZE);
        /* SAFETY: callers validate a readable 0.2 prefix; copied buffers stay borrowed. */
        unsafe {
            ptr::copy_nonoverlapping(
                out.cast::<u8>(),
                ptr::from_mut(&mut value).cast::<u8>(),
                Self::PREFIX_SIZE as usize,
            );
        }
        value
    }
    pub(crate) fn empty(size: u32) -> Self {
        Self {
            struct_size: size,
            function: 0,
            status: 0,
            position: 0,
            probability: f64::NAN,
            error_code: 0,
            answer_bool: 0,
            rank: 0,
            number: 0.0,
            id: Buffer::default(),
            answer: Buffer::default(),
            detail: Buffer::default(),
            source: Buffer::default(),
            error: Buffer::default(),
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
            (*ptr::addr_of_mut!((*out).id)).release();
            (*ptr::addr_of_mut!((*out).answer)).release();
            (*ptr::addr_of_mut!((*out).detail)).release();
            (*ptr::addr_of_mut!((*out).source)).release();
            (*ptr::addr_of_mut!((*out).error)).release();
            Self::empty((*out).struct_size).write(out);
        }
    }
}
/// An open client. Any thread may use it, and calls opened from it outlive it. Free it with
/// thinkthen_client_free.
#[derive(Debug)]
pub struct ClientHandle(pub(crate) Engine);
/// An open call. One thread drives it at a time; thinkthen_call_cancel may run on any thread.
/// Free it with thinkthen_call_free.
#[derive(Debug)]
pub struct CallHandle {
    pub(crate) call: UnsafeCell<native::Call>,
    pub(crate) cancel: Cancel,
    pub(crate) originals: UnsafeCell<Option<std::collections::VecDeque<crate::original::Original>>>,
}
impl CallHandle {
    pub(crate) fn new(
        call: native::Call,
        request: &thinkthen::Request,
        originals: Vec<crate::original::Original>,
    ) -> Self {
        let cancel = call.cancellation();
        Self {
            call: UnsafeCell::new(call),
            cancel,
            originals: UnsafeCell::new(
                (request.question().function() == thinkthen::FunctionName::Filter)
                    .then(|| originals.into()),
            ),
        }
    }
}
/// Settings for a client, copied when it opens. Free them with thinkthen_settings_free.
#[derive(Debug, Default)]
pub struct SettingsHandle(pub(crate) Settings);
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

// The 0.2 fields end here even when a later engine appends fields. Neither the
// minimum accepted size nor output writes follow the engine's full struct size.
macro_rules! prefix_output {
    ($output:ty, $last:ident) => {
        impl $output {
            pub(crate) const PREFIX_SIZE: u32 =
                (offset_of!(Self, $last) + size_of::<Buffer>()) as u32;
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
prefix_output!(HostRow, error);
prefix_output!(HostError, message);
