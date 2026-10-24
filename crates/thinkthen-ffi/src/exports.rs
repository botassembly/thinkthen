//! Public foreign entry points, forwarding behavior to admitted Rust values.
#![allow(
    unsafe_code,
    reason = "C exports dereference counted borrowed pointers under the header contract"
)]

use crate::memory::{error_buffer, text};
use crate::{Buffer, CallHandle, ClientHandle, HostRow};
use std::cell::UnsafeCell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use thinkthen::{Cancel, Client, Error, ErrorKind, OffsetUnit, PullEvent, SdkRequest};

const OK: i32 = 0;
const END: i32 = 1;
const NEEDS_INPUT: i32 = 2;
const ERROR: i32 = -1;

pub(crate) unsafe fn operation(
    error: *mut Buffer,
    run: impl FnOnce() -> Result<i32, Error>,
) -> i32 {
    // SAFETY: the required error slot is writable and owns no live buffer.
    unsafe { error.write(Buffer::default()) };
    let result = catch_unwind(AssertUnwindSafe(run)).unwrap_or_else(|_| {
        Err(Error::new(
            ErrorKind::Io,
            "the native operation could not complete",
        ))
    });
    match result {
        Ok(status) => status,
        Err(failure) => {
            // SAFETY: the operation's exclusive error slot receives its own allocation.
            unsafe { error.write(error_buffer(&failure)) };
            ERROR
        }
    }
}

/// Admit client settings and publish an independent client owner.
/// # Safety
/// All pointers obey the readable input and writable distinct output contract.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_open(
    settings: *const u8,
    len: usize,
    out: *mut *mut ClientHandle,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: the required output slot owns no live client.
    unsafe { out.write(std::ptr::null_mut()) };
    // SAFETY: counted bytes and error slot obey the export's contract.
    unsafe {
        operation(error, || {
            let client = Client::from_json(text(settings, len)?, |name| std::env::var(name).ok())?;
            out.write(Box::into_raw(Box::new(ClientHandle(client))));
            Ok(OK)
        })
    }
}

/// Release a client; independently opened calls retain their engine.
/// # Safety
/// The pointer is NULL or this library's unfreed owner, with no active callers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_free(client: *mut ClientHandle) {
    if !client.is_null() {
        // SAFETY: the host transfers the unique client allocation after its readers.
        drop(unsafe { Box::from_raw(client) });
    }
}

/// Admit one call using a retained client and shared offset type.
/// # Safety
/// Client and input bytes are live; output slots are distinct and writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_open(
    client: *const ClientHandle,
    request: *const u8,
    len: usize,
    offsets: u32,
    out: *mut *mut CallHandle,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: the required output slot owns no live call.
    unsafe { out.write(std::ptr::null_mut()) };
    // SAFETY: the host retains inputs and gives this operation exclusive outputs.
    unsafe {
        operation(error, || {
            let offsets = match offsets {
                0 => OffsetUnit::Utf8,
                1 => OffsetUnit::Utf16,
                2 => OffsetUnit::CodePoints,
                _ => return Err(Error::new(ErrorKind::Question, "invalid offset unit")),
            };
            let request = SdkRequest::from_json(text(request, len)?)?;
            let cancel = Cancel::default();
            let call = (*client).0.open(&request, offsets, cancel.clone())?;
            out.write(Box::into_raw(Box::new(CallHandle {
                call: UnsafeCell::new(call),
                cancel,
            })));
            Ok(OK)
        })
    }
}

/// Supply a raw UTF-8 text record when the shared call requests input.
/// # Safety
/// One driver owns the live call, the input extent and exclusive error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_send_text(
    call: *mut CallHandle,
    record: *const u8,
    len: usize,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: only the driver accesses the call cell; cancel uses a separate token.
    unsafe {
        operation(error, || {
            (&mut *(*call).call.get()).send_text(text(record, len)?)?;
            Ok(OK)
        })
    }
}

/// Supply counted native-endian UTF-16 through the shared strict text decoder.
/// # Safety
/// One driver owns the live call, readable aligned units and exclusive error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_send_utf16(
    call: *mut CallHandle,
    record: *const u16,
    len: usize,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: the driver owns the cell; the host lends len aligned units or NULL/zero.
    unsafe {
        operation(error, || {
            let units = if len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(record, len)
            };
            let text = thinkthen::utf16_text(units)?;
            (&mut *(*call).call.get()).send_text(&text)?;
            Ok(OK)
        })
    }
}

/// Supply a structured JSON record, leaving evidence admission to Rust.
/// # Safety
/// One driver owns the live call, the input extent and exclusive error slot.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_send_json(
    call: *mut CallHandle,
    record: *const u8,
    len: usize,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: only the driver accesses the call cell; bytes live through return.
    unsafe {
        operation(error, || {
            (&mut *(*call).call.get()).send_json(text(record, len)?)?;
            Ok(OK)
        })
    }
}

/// End input without waiting for execution or queue capacity.
/// # Safety
/// The live call has one driver and cannot be freed during this operation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_end_input(call: *mut CallHandle) {
    // SAFETY: the driver exclusively borrows the operation cell.
    unsafe { (&mut *(*call).call.get()).end_input() };
}

/// Pull one shared row, input request or terminal outcome.
/// # Safety
/// One driver owns the live call; row and error outputs are distinct and empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_next(
    call: *mut CallHandle,
    out: *mut HostRow,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: the row slot is exclusive writable storage, with no live row.
    unsafe { out.write(HostRow::default()) };
    // SAFETY: the one driver alone accesses the cell and required output slots.
    unsafe {
        operation(error, || {
            let call = &mut *(*call).call.get();
            match call.next_event() {
                PullEvent::NeedsInput => Ok(NEEDS_INPUT),
                PullEvent::Row(row) => {
                    out.write(row.into());
                    Ok(OK)
                }
                PullEvent::Finished => call
                    .final_error()
                    .map_or(Ok(END), |error| Err(error.clone())),
            }
        })
    }
}

/// Request cancellation without accessing the driver's mutable operation.
/// # Safety
/// The call remains live until every concurrent operation has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel(call: *mut CallHandle) {
    // SAFETY: the independently shared token is live and safe to cancel concurrently.
    unsafe { (*call).cancel.cancel() };
}

/// Release the host owner without joining the backend worker.
/// # Safety
/// The pointer is NULL or a unique live owner after all host operations return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_free(call: *mut CallHandle) {
    if !call.is_null() {
        // SAFETY: the host transfers the unique call allocation with no active users.
        drop(unsafe { Box::from_raw(call) });
    }
}

/// Release one returned buffer and clear its fields.
/// # Safety
/// The pointer is NULL or writable storage owning an unread library allocation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_buffer_free(buffer: *mut Buffer) {
    if !buffer.is_null() {
        // SAFETY: the unique buffer and its writable slot are transferred for cleanup.
        unsafe { (*buffer).release() };
    }
}

/// Release both row buffers and clear all fields.
/// # Safety
/// The pointer is NULL or a uniquely owned row after every reader has finished.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_row_free(row: *mut HostRow) {
    if !row.is_null() {
        // SAFETY: both unique row buffers and their containing writable slot are owned.
        unsafe {
            (*row).fields.release();
            (*row).detail.release();
            row.write(HostRow::default());
        }
    }
}
