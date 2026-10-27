//! Version, ownership and canonical call operations.
#![allow(
    unsafe_code,
    reason = "exports borrow counted foreign inputs and own supported output prefixes"
)]
use crate::memory::{bytes, error_size, row_size, text};
use crate::{
    Buffer, CallHandle, ClientHandle, HostError, HostRow, QuestionHandle, SettingsHandle, TextView,
};
use std::{
    ffi::c_char,
    panic::{AssertUnwindSafe, catch_unwind},
    ptr,
    sync::OnceLock,
};
use thinkthen::{
    Code, Error, OffsetUnit, Request, Settings,
    native::{self, Event},
    records::Record,
};
/// A row or successful operation.
pub const THINKTHEN_OK: i32 = 0;
/// Successful completion, including after record failures.
pub const THINKTHEN_END: i32 = 1;
/// One input slot is reserved before the host sends its next record.
pub const THINKTHEN_NEEDS_INPUT: i32 = 2;
/// A call-level error, separate from the row status and code registry.
pub const THINKTHEN_ERROR: i32 = -1;
pub(crate) const OK: i32 = THINKTHEN_OK;
pub(crate) const END: i32 = THINKTHEN_END;
pub(crate) const NEEDS_INPUT: i32 = THINKTHEN_NEEDS_INPUT;
pub(crate) const ERROR: i32 = THINKTHEN_ERROR;
pub(crate) unsafe fn operation(
    error: *mut HostError,
    run: impl FnOnce() -> Result<i32, Error>,
) -> i32 {
    /* SAFETY: only the first size field is read until its supported prefix is validated. */
    let Some(size) = (unsafe { error_size(error) }) else {
        return ERROR;
    };
    /* SAFETY: the validated slot owns no live allocation on entry. */
    unsafe {
        HostError::empty(size).write(error);
    }
    match catch_unwind(AssertUnwindSafe(run)).unwrap_or_else(|_| Err(native::internal_error())) {
        Ok(status) => status,
        Err(failure) => {
            /* SAFETY: the supported prefix is writable and exclusively owned. */
            unsafe {
                HostError::from_error(size, &failure).write(error);
            }
            ERROR
        }
    }
}
/// Return the shared call-level error for host settings conversion failure.
/// # Safety
/// The error is an exclusive, empty writable slot with a supported struct_size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_conversion_error(error: *mut HostError) -> i32 {
    /* SAFETY: the caller lends the sized error slot until return. */
    unsafe { operation(error, || Err(native::settings_conversion_error())) }
}
/// Return the shared call-level error for host question conversion failure.
/// # Safety
/// The error is an exclusive, empty writable slot with a supported struct_size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_conversion_error(error: *mut HostError) -> i32 {
    /* SAFETY: the caller lends the sized error slot until return. */
    unsafe { operation(error, || Err(native::question_conversion_error())) }
}
/// Return the shared cancellation error after a host interrupts a completed pull.
/// # Safety
/// The error is an exclusive, empty writable slot with a supported struct_size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancelled_error(error: *mut HostError) -> i32 {
    /* SAFETY: the caller lends the sized error slot until return. */
    unsafe { operation(error, || Err(native::cancelled_error())) }
}
/// Native ABI major/minor, independent of package version.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_abi_version() -> u32 {
    thinkthen::ABI_VERSION
}
/// Process-lifetime NUL-terminated package version; never free this pointer.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}
/// Process-lifetime dotted code name, or NULL for an unknown integer.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_code_name(value: i32) -> *const c_char {
    static NAMES: OnceLock<Vec<(i32, std::ffi::CString)>> = OnceLock::new();
    NAMES
        .get_or_init(|| {
            Code::ALL
                .iter()
                .filter_map(|code| {
                    std::ffi::CString::new(code.as_str())
                        .ok()
                        .map(|name| (code.abi(), name))
                })
                .collect()
        })
        .iter()
        .find(|(code, _)| *code == value)
        .map_or(ptr::null(), |(_, name)| name.as_ptr())
}
/// Open a client from a copied typed settings snapshot.
/// # Safety
/// Handles are live; out/error are distinct, empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_open(
    settings: *const SettingsHandle,
    out: *mut *mut ClientHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends all inputs and exclusive output slots. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let engine = crate::engine((*settings).0.clone())?;
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(OK)
        })
    }
}
/// Admit canonical client JSON for a JSON-native host before resolving keys.
/// # Safety
/// Counted bytes and distinct, empty outputs remain live until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_open_json(
    settings: *const u8,
    len: usize,
    out: *mut *mut ClientHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: inputs are counted readable bytes and outputs are exclusive slots. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let settings = Settings::from_json(text(settings, len)?)?;
            let engine = crate::engine(settings)?;
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(OK)
        })
    }
}
/// Free this client; its calls retain the engine independently.
/// # Safety
/// NULL or one live owner after all active operations finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_free(client: *mut ClientHandle) {
    if !client.is_null() {
        /* SAFETY: the caller transfers its unique allocation. */
        drop(unsafe { Box::from_raw(client) });
    }
}
pub(crate) unsafe fn open(
    client: *const ClientHandle,
    request: &Request,
    offsets: u32,
    out: *mut *mut CallHandle,
) -> Result<i32, Error> {
    /* SAFETY: the output is exclusive and initially owns no call. */
    unsafe {
        out.write(ptr::null_mut());
    }
    let offsets = OffsetUnit::from_abi(offsets).ok_or_else(native::abi_error)?;
    /* SAFETY: the client remains live while the engine snapshot is retained. */
    let call = native::Call::open(unsafe { &(*client).0 }, request, offsets)?;
    /* SAFETY: an independent owned call is published into the empty slot. */
    unsafe {
        out.write(Box::into_raw(Box::new(CallHandle::new(
            call,
            request,
            crate::original::Original::initial(request),
        ))));
    }
    Ok(OK)
}
/// Open a copied typed question with its lists, options and optional execution name.
/// NULL name is absent; a present empty name fails the shared name validation.
/// Toolbox selection never supplies this execution name.
/// # Safety
/// Handles, optional name view and distinct empty outputs remain live until return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_open_question(
    client: *const ClientHandle,
    question: *const QuestionHandle,
    name: *const TextView,
    offsets: u32,
    out: *mut *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends live inputs and exclusive writable outputs. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let mut request = Request::new((*question).0.build()?);
            if !name.is_null() {
                request = request.named(text((*name).data, (*name).len)?)?;
            }
            open(client, &request, offsets, out)
        })
    }
}
/// Admit a canonical request for a JSON-native host; never load a question file.
/// # Safety
/// Client and counted bytes are live; outputs are distinct empty slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_open_json(
    client: *const ClientHandle,
    request: *const u8,
    len: usize,
    offsets: u32,
    out: *mut *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: counted input and outputs obey their documented extents. */
    unsafe {
        operation(error, || {
            open(
                client,
                &Request::from_json(text(request, len)?)?,
                offsets,
                out,
            )
        })
    }
}
/// Send copied text after NEEDS_INPUT; invalid UTF-8 becomes one record.invalid row.
/// # Safety
/// One driver owns a live call and borrows counted readable bytes and an empty error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_send_text(
    call: *mut CallHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: one driver exclusively accesses the cell; cancel uses a separate token. */
    unsafe {
        operation(error, || {
            (&mut *(*call).call.get()).send(Record::from_utf8(bytes(data, len)))?;
            if let Some(originals) = &mut *(*call).originals.get() {
                originals.push_back(crate::original::Original::new(
                    thinkthen::records::RecordKind::Text,
                    bytes(data, len),
                ));
            }
            Ok(OK)
        })
    }
}
/// Send copied structured JSON; admission failures become failed records.
/// # Safety
/// One driver owns the call; counted input and error slots remain live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_send_json(
    call: *mut CallHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: one driver accesses the cell and borrows counted bytes until return. */
    unsafe {
        operation(error, || {
            let record = match std::str::from_utf8(bytes(data, len)) {
                Ok(json) => Record::from_json(json),
                Err(_) => Record::invalid(),
            };
            (&mut *(*call).call.get()).send(record)?;
            if let Some(originals) = &mut *(*call).originals.get() {
                originals.push_back(crate::original::Original::new(
                    thinkthen::records::RecordKind::Json,
                    bytes(data, len),
                ));
            }
            Ok(OK)
        })
    }
}
/// Send native-endian UTF-16; unpaired surrogates become one failed record.
/// # Safety
/// One driver owns the call; data lends len aligned units or NULL at zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_send_utf16(
    call: *mut CallHandle,
    data: *const u16,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends aligned counted units and an exclusive error slot. */
    unsafe {
        operation(error, || {
            let units = if len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(data, len)
            };
            (&mut *(*call).call.get()).send(Record::from_utf16(units))?;
            if let Some(originals) = &mut *(*call).originals.get() {
                originals.push_back(match String::from_utf16(units) {
                    Ok(text) => crate::original::Original::new(
                        thinkthen::records::RecordKind::Text,
                        text.as_bytes(),
                    ),
                    Err(_) => {
                        crate::original::Original::new(thinkthen::records::RecordKind::Invalid, &[])
                    }
                });
            }
            Ok(OK)
        })
    }
}
/// Send an explicit host missing value without a backend call.
/// # Safety
/// One driver owns the live call and empty exclusive error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_send_missing(
    call: *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: only the driver accesses this call cell and output slot. */
    unsafe {
        operation(error, || {
            (&mut *(*call).call.get()).send(Record::missing())?;
            if let Some(originals) = &mut *(*call).originals.get() {
                originals.push_back(crate::original::Original::new(
                    thinkthen::records::RecordKind::Missing,
                    &[],
                ));
            }
            Ok(OK)
        })
    }
}
/// Send an explicit host conversion failure without fabricating a JSON id.
/// # Safety
/// One driver owns the live call and empty exclusive error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_send_invalid(
    call: *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: only the driver accesses this call cell and output slot. */
    unsafe {
        operation(error, || {
            (&mut *(*call).call.get()).send(Record::invalid())?;
            if let Some(originals) = &mut *(*call).originals.get() {
                originals.push_back(crate::original::Original::new(
                    thinkthen::records::RecordKind::Invalid,
                    &[],
                ));
            }
            Ok(OK)
        })
    }
}
/// Close input without waiting for output capacity or backend work.
/// # Safety
/// A sole driver owns a live call; free cannot run concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_end_input(call: *mut CallHandle) {
    /* SAFETY: only the driver accesses the call cell. */
    unsafe {
        (&mut *(*call).call.get()).end_input();
    }
}
/// Pull one row, reserved input slot or completion. Failed records return OK.
/// # Safety
/// A sole driver owns the call; row/error are sized, distinct, empty output slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_next(
    call: *mut CallHandle,
    out: *mut HostRow,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: size is validated before writing any supported output field. */
    unsafe {
        operation(error, || {
            let size = row_size(out)?;
            HostRow::empty(size).write(out);
            match (&mut *(*call).call.get()).next_event()? {
                Event::NeedsInput => Ok(NEEDS_INPUT),
                Event::Finished => Ok(END),
                Event::Row(row) => {
                    let mut result = HostRow::from_row(size, row)?;
                    if let Some(originals) = &mut *(*call).originals.get() {
                        let original = originals.pop_front().ok_or_else(native::internal_error)?;
                        result.source.attach_original(original);
                    }
                    result.write(out);
                    Ok(OK)
                }
                _ => Err(native::abi_error()),
            }
        })
    }
}
/// Request cancellation concurrently without touching driver state.
/// # Safety
/// The handle remains live until every concurrent operation returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_cancel(call: *mut CallHandle) {
    /* SAFETY: cancel accesses only its independently shared thread-safe token. */
    unsafe {
        (*call).cancel.cancel();
    }
}
/// Cancel and release a call without joining a backend worker.
/// # Safety
/// NULL or the unique owner after all active operations finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_free(call: *mut CallHandle) {
    if !call.is_null() {
        /* SAFETY: the host transfers its unique call allocation. */
        drop(unsafe { Box::from_raw(call) });
    }
}
/// Free one owned counted buffer and clear it; empty/NULL is harmless.
/// # Safety
/// The slot uniquely owns an engine allocation and no reader remains.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_buffer_free(buffer: *mut Buffer) {
    if !buffer.is_null() {
        /* SAFETY: cleanup owns the allocation and writable slot. */
        unsafe {
            (*buffer).release();
        }
    }
}
/// Free a supported error prefix while preserving caller size and trailing bytes.
/// # Safety
/// NULL or a sized unique error after readers finish; unsupported prefixes are untouched.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_free(error: *mut HostError) {
    /* SAFETY: only its first field is read until a supported prefix is known. */
    if let Some(size) = unsafe { error_size(error) } {
        /* SAFETY: the supported owned message and prefix are exclusively writable. */
        unsafe {
            (*error).message.release();
            HostError::empty(size).write(error);
        }
    }
}
/// Free a supported row prefix while preserving caller size and trailing bytes.
/// # Safety
/// NULL or a sized unique row after readers finish; unsupported prefixes are untouched.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_free(row: *mut HostRow) {
    /* SAFETY: the first size field validates the remaining supported prefix. */
    if unsafe { row_size(row) }.is_ok() {
        /* SAFETY: every buffer in the valid prefix has one owner. */
        unsafe {
            HostRow::release(row);
        }
    }
}
