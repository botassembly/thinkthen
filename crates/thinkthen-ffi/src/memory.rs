//! Native ownership, sized output prefixes and counted borrowed inputs.
#![allow(
    unsafe_code,
    reason = "the ABI owns foreign pointers and allocated output prefixes"
)]
use std::{cell::UnsafeCell, mem::size_of, ptr};
use thinkthen::{
    Cancel, Engine, Error, Settings,
    native::{self, QuestionBuilder, ToolboxBuilder},
};
/// An engine allocation, released only by its matching cleanup operation.
#[repr(C)]
#[derive(Debug)]
pub struct Buffer {
    /// Counted UTF-8 bytes without a terminating NUL.
    pub data: *mut u8,
    /// Byte length.
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
/// A sized owned error; code zero means no error and HTTP status zero means absent.
#[repr(C)]
#[derive(Debug)]
pub struct HostError {
    /// The caller's allocation size, preserved by cleanup.
    pub struct_size: u32,
    /// A stable generated code, or zero.
    pub code: i32,
    /// Optional HTTP status; zero means absent.
    pub http_status: u16,
    /// Owned plain UTF-8 message, independent of the call.
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
    pub(crate) fn from_error(size: u32, error: &Error) -> Self {
        Self {
            struct_size: size,
            code: error.code().abi(),
            http_status: error.http_status().unwrap_or(0),
            message: Buffer::new(error.message().to_owned()),
        }
    }
}
/// One independently owned typed row. Scalar fields are selected by function/status.
#[repr(C)]
#[derive(Debug)]
pub struct HostRow {
    /// The caller's allocation size, preserved by cleanup.
    pub struct_size: u32,
    /// Generated function discriminator.
    pub function: u32,
    /// Generated status discriminator.
    pub status: u32,
    /// Original one-based input position.
    pub position: u64,
    /// Full precision aggregate probability, or NaN when absent.
    pub probability: f64,
    /// Failed-record code, or zero.
    pub error_code: i32,
    /// Answered decide/filter value as zero or one.
    pub answer_bool: u8,
    /// Answered rank, independent of position.
    pub rank: u64,
    /// Answered score.
    pub number: f64,
    /// Optional original id as exact contract JSON.
    pub id: Buffer,
    /// Typed answer as contract JSON, absent for unsure/failed.
    pub answer: Buffer,
    /// Optional typed function detail as contract JSON.
    pub detail: Buffer,
    /// Typed attempted-route source as contract JSON.
    pub source: Buffer,
    /// Failed record's typed contract error as JSON.
    pub error: Buffer,
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
    pub(crate) unsafe fn release(&mut self) {
        /* SAFETY: a valid prefix owns each distinct buffer independently. */
        unsafe {
            self.id.release();
            self.answer.release();
            self.detail.release();
            self.source.release();
            self.error.release();
        }
        let size = self.struct_size;
        *self = Self::empty(size);
    }
}
/// An independently retained admitted engine.
#[derive(Debug)]
pub struct ClientHandle(pub(crate) Engine);
/// One driving thread owns the cell; cancel accesses only its separate token.
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
/// Mutable settings copied when a client is opened.
#[derive(Debug, Default)]
pub struct SettingsHandle(pub(crate) Settings);
/// Mutable typed question with no retained caller-owned pointers.
#[derive(Debug)]
pub struct QuestionHandle(pub(crate) QuestionBuilder);
/// Named typed snapshots that survive their inserted children.
#[derive(Debug, Default)]
pub struct ToolboxHandle(pub(crate) ToolboxBuilder);
/// Borrow counted input bytes only until the export returns.
pub(crate) unsafe fn bytes<'a>(data: *const u8, len: usize) -> &'a [u8] {
    if len == 0 {
        &[]
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
    /* SAFETY: a non-NULL output permits reading its first size field. */
    if out.is_null() || unsafe { (*out).struct_size } < (size_of::<HostRow>() as u32) {
        Err(native::abi_error())
    } else {
        /* SAFETY: size was validated without reading past its prefix. */
        Ok(unsafe { (*out).struct_size })
    }
}
pub(crate) unsafe fn error_size(out: *mut HostError) -> Option<u32> {
    /* SAFETY: a non-NULL output permits reading its first size field. */
    if out.is_null() || unsafe { (*out).struct_size } < (size_of::<HostError>() as u32) {
        None
    } else {
        /* SAFETY: size is readable and covers the supported prefix. */
        Some(unsafe { (*out).struct_size })
    }
}
