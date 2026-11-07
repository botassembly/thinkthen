//! Finite foreign records use the same admission and worker as pulled input.
#![allow(
    unsafe_code,
    reason = "the caller lends counted descriptors and bytes until admission completes"
)]
use crate::{
    CallHandle, ClientHandle, HostError,
    exports::{THINKTHEN_OK, named_request, operation},
    memory::{bytes, live, text},
};
use std::ptr;
use thinkthen::{
    Error, OffsetUnit, Request, native,
    records::{Record, RecordKind},
};

/// One record for thinkthen_call_open_records_*; its bytes are copied before the call opens.
#[repr(C)]
#[derive(Debug)]
pub struct RecordInput {
    /// sizeof(thinkthen_record_input), at most record_stride.
    pub struct_size: u32,
    /// How to read data, a thinkthen_record_kind; missing and invalid records need no data.
    pub kind: u32,
    /// The record's bytes for text or JSON; NULL only when len is 0.
    pub data: *const u8,
    /// The byte length, or THINKTHEN_NUL_TERMINATED.
    pub len: usize,
}

pub(crate) unsafe fn admit_record(data: *const u8, len: usize, kind: u32) -> Result<Record, Error> {
    /* SAFETY: text and JSON borrow the caller's counted readable bytes. */
    unsafe {
        let kind = RecordKind::from_abi(kind)
            .ok_or_else(|| thinkthen::native::argument_error(thinkthen::Code::UsageRecordKind))?;
        let data = if matches!(kind, RecordKind::Text | RecordKind::Json) {
            bytes(data, len)
        } else {
            &[]
        };
        let record = match kind {
            RecordKind::Text => Record::from_utf8(data),
            RecordKind::Json => match std::str::from_utf8(data) {
                Ok(json) => Record::from_json(json),
                Err(_) => Record::invalid(),
            },
            RecordKind::Missing => Record::missing(),
            RecordKind::Invalid => Record::invalid(),
            _ => return Err(native::abi_error()),
        };
        Ok(record.with_original_encoding(kind, data))
    }
}

/// Open a call over count records with a question from request JSON.
///
/// The request must not carry its own records. Read the call with thinkthen_call_next; it never
/// returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK with *out set, or a negative code with
/// *out set to NULL: a usage code for a request that is not valid or carries records,
/// -THINKTHEN_CODE_USAGE_RECORD_KIND for an unknown record kind, -THINKTHEN_CODE_USAGE_ABI for
/// a bad record size or stride, -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it with
/// thinkthen_call_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. client is live. request points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. records
/// points to count readable thinkthen_record values, each record_stride bytes apart and each
/// with struct_size set, whose bytes are copied before return. out is writable. error is NULL
/// or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            open(
                client,
                &request,
                records,
                count,
                record_stride,
                offsets,
                out,
            )
        })
    }
}

// Admit the records, open the call over them and publish it into the empty slot.
unsafe fn open(
    client: *const ClientHandle,
    request: &Request,
    records: *const RecordInput,
    count: usize,
    record_stride: usize,
    offsets: u32,
    out: *mut *mut CallHandle,
) -> Result<i32, Error> {
    let offsets = OffsetUnit::from_abi(offsets)
        .ok_or_else(|| native::argument_error(thinkthen::Code::UsageOffsets))?;
    /* SAFETY: the caller lends counted inputs, a live client and an exclusive output. */
    unsafe {
        let (initial, originals) = admit_records(
            records,
            count,
            record_stride,
            request.question().function() == thinkthen::FunctionName::Filter,
        )?;
        let call = native::Call::open_records(&live(client)?.0, request, initial, offsets)?;
        out.write(Box::into_raw(Box::new(CallHandle::new(
            call, request, originals,
        ))));
    }
    Ok(THINKTHEN_OK)
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

/// Open a call that asks question about count records.
///
/// name works as in thinkthen_call_open_question. Read the call with thinkthen_call_next; it
/// never returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK with *out set, or a negative code
/// with *out set to NULL: -THINKTHEN_CODE_USAGE_QUESTION for an incomplete question,
/// -THINKTHEN_CODE_USAGE_NAME for a bad name, -THINKTHEN_CODE_USAGE_RECORD_KIND for an unknown
/// record kind, -THINKTHEN_CODE_USAGE_ABI for a bad record size or stride, or
/// -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit. The caller owns *out and frees it
/// with thinkthen_call_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes question. client and question are live.
/// records points to count readable thinkthen_record values, each record_stride bytes apart and
/// each with struct_size set, whose bytes are copied before return. name is NULL or points to a
/// readable thinkthen_text. out is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
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
            let request = named_request(question, name)?;
            open(
                client,
                &request,
                records,
                count,
                record_stride,
                offsets,
                out,
            )
        })
    }
}
