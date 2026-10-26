//! Finite foreign records use the same admission and worker as pulled input.
#![allow(
    unsafe_code,
    reason = "the caller lends counted descriptors and bytes until admission completes"
)]
use crate::{
    CallHandle, ClientHandle, HostError,
    exports::{OK, operation},
    memory::{bytes, text},
};
use std::ptr;
use thinkthen::{
    Error, OffsetUnit, Request, native,
    records::{Record, RecordKind},
};

/// One borrowed finite input; its byte view is copied before open returns.
#[repr(C)]
#[derive(Debug)]
pub struct RecordInput {
    /// Allocated descriptor size, at least the current prefix and at most record_stride.
    pub struct_size: u32,
    /// Generated record kind; missing and invalid need no byte view.
    pub kind: u32,
    /// Readable bytes for text or JSON, or NULL at zero length.
    pub data: *const u8,
    /// Byte length without a terminator.
    pub len: usize,
}

pub(crate) unsafe fn admit_record(data: *const u8, len: usize, kind: u32) -> Result<Record, Error> {
    /* SAFETY: text and JSON borrow the caller's counted readable bytes. */
    unsafe {
        match RecordKind::from_abi(kind).ok_or_else(native::abi_error)? {
            RecordKind::Text => Ok(Record::from_utf8(bytes(data, len))),
            RecordKind::Json => Ok(match std::str::from_utf8(bytes(data, len)) {
                Ok(json) => Record::from_json(json),
                Err(_) => Record::invalid(),
            }),
            RecordKind::Missing => Ok(Record::missing()),
            RecordKind::Invalid => Ok(Record::invalid()),
            _ => Err(native::abi_error()),
        }
    }
}

/// Admit a question and copy all finite descriptors before starting the shared worker.
/// Initial records in request JSON are rejected as competing collections.
/// # Safety
/// Client and counted request bytes are live; records points to count readable slots
/// of record_stride bytes each, with readable byte views for text and JSON kinds.
/// Each slot has a readable size field; out/error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_open_records_json(
    client: *const ClientHandle,
    request: *const u8,
    len: usize,
    records: *const RecordInput,
    count: usize,
    record_stride: usize,
    offsets: u32,
    out: *mut *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends the documented counted inputs and exclusive outputs. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let request = Request::from_json(text(request, len)?)?;
            if !request.records().is_empty() {
                return Err(native::abi_error());
            }
            let offsets = OffsetUnit::from_abi(offsets).ok_or_else(native::abi_error)?;
            let (initial, originals) = admit_records(
                records,
                count,
                record_stride,
                request.question().function() == thinkthen::FunctionName::Filter,
            )?;
            let call = native::Call::open_records(&(*client).0, &request, initial, offsets)?;
            out.write(Box::into_raw(Box::new(CallHandle::new(
                call, &request, originals,
            ))));
            Ok(OK)
        })
    }
}

unsafe fn admit_records(
    records: *const RecordInput,
    count: usize,
    record_stride: usize,
    retain: bool,
) -> Result<(Vec<Record>, Vec<crate::original::Original>), Error> {
    /* SAFETY: callers lend count readable descriptors and their byte views. */
    unsafe {
        if count > 0
            && (records.is_null()
                || record_stride < size_of::<RecordInput>()
                || count > (isize::MAX as usize) / record_stride)
        {
            return Err(native::abi_error());
        }
        let mut initial = Vec::with_capacity(count);
        let mut originals = Vec::new();
        for index in 0..count {
            let input = records.cast::<u8>().add(index * record_stride);
            let size = ptr::read_unaligned(input.cast::<u32>()) as usize;
            if size < size_of::<RecordInput>() || size > record_stride {
                return Err(native::abi_error());
            }
            let input = ptr::read_unaligned(input.cast::<RecordInput>());
            initial.push(admit_record(input.data, input.len, input.kind)?);
            if retain {
                originals.push(crate::original::input(input.data, input.len, input.kind)?);
            }
        }
        Ok((initial, originals))
    }
}

/// Open a typed named question over a finite collection copied before return.
/// The exact count preserves finite batching and model-bound admission.
/// # Safety
/// Client, question, optional name and counted descriptors are live until return.
/// Each descriptor has record_stride readable bytes and a supported size prefix.
/// Out and error are distinct empty writable slots; one thread drives the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_open_records_question(
    client: *const ClientHandle,
    question: *const crate::QuestionHandle,
    name: *const crate::builders::TextView,
    records: *const RecordInput,
    count: usize,
    record_stride: usize,
    offsets: u32,
    out: *mut *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends counted inputs and exclusive outputs. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let mut request = Request::new((*question).0.build()?);
            if !name.is_null() {
                request = request.named(text((*name).data, (*name).len)?)?;
            }
            let offsets = OffsetUnit::from_abi(offsets).ok_or_else(native::abi_error)?;
            let (initial, originals) = admit_records(
                records,
                count,
                record_stride,
                request.question().function() == thinkthen::FunctionName::Filter,
            )?;
            let call = native::Call::open_records(&(*client).0, &request, initial, offsets)?;
            out.write(Box::into_raw(Box::new(CallHandle::new(
                call, &request, originals,
            ))));
            Ok(OK)
        })
    }
}
