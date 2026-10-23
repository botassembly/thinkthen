//! Counted foreign views over the single shared SQL host.
#![allow(
    unsafe_code,
    reason = "SQL exports borrow counted C inputs and publish owned handles"
)]

use std::ffi::c_void;
use std::time::Duration;

use crate::exports::operation;
use crate::memory::text;
use crate::{Buffer, ClientHandle, HostRow};
use thinkthen::{Error, ErrorKind};
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord};

/// Statement-owned SQL host; no database pointer is retained.
#[derive(Debug)]
pub struct SqlHandle(SqlCall);

/// Borrowed UTF-8 record bytes copied by chunk submission.
#[derive(Debug)]
#[repr(C)]
pub struct SqlInput {
    /// Zero for raw text, one for structured JSON.
    pub kind: u32,
    /// Readable bytes for `len`, or NULL at zero length.
    pub data: *const u8,
    /// Byte length, without a terminating NUL.
    pub len: usize,
}

/// Read a native interrupt flag without throwing or calling the driver.
pub type InterruptProbe = Option<unsafe extern "C" fn(*mut c_void) -> u8>;

/// Admit SQL settings before resolving process-owned backend configuration.
/// # Safety
/// Settings are counted readable bytes; out and error are distinct writable
/// empty slots. Input is admitted before return, and no foreign view is retained.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_client_open(
    settings: *const u8,
    len: usize,
    out: *mut *mut ClientHandle,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: required output is exclusive and owns no client.
    unsafe { out.write(std::ptr::null_mut()) };
    // SAFETY: counted settings and output slots remain valid through return.
    unsafe {
        operation(error, || {
            let client =
                thinkthen_sql::client(text(settings, len)?, |name| std::env::var(name).ok())?;
            out.write(Box::into_raw(Box::new(ClientHandle(client))));
            Ok(0)
        })
    }
}

/// Admit SQL settings under an administrator's backend/model policy.
/// # Safety
/// Counted inputs are readable; out and error are distinct writable empty slots.
/// A NULL policy with zero length selects the default-only policy.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_client_open_with_policy(
    settings: *const u8,
    len: usize,
    allowed: *const u8,
    allowed_len: usize,
    out: *mut *mut ClientHandle,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: required output is exclusive and owns no client.
    unsafe { out.write(std::ptr::null_mut()) };
    // SAFETY: counted inputs and output slots remain valid through return.
    unsafe {
        operation(error, || {
            let allowed = if allowed.is_null() && allowed_len == 0 {
                None
            } else {
                Some(text(allowed, allowed_len)?)
            };
            let client =
                thinkthen_sql::client_with_policy(text(settings, len)?, allowed, |name| {
                    std::env::var(name).ok()
                })?;
            out.write(Box::into_raw(Box::new(ClientHandle(client))));
            Ok(0)
        })
    }
}

/// Admit native SQL arguments using the shared shorthand and request grammar.
/// # Safety
/// Inputs are readable counted bytes; client is live; output slots are distinct,
/// writable and empty. All bytes are copied or admitted before return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_open(
    client: *const ClientHandle,
    function: *const u8,
    function_len: usize,
    question: *const u8,
    question_len: usize,
    options: *const u8,
    options_len: usize,
    out: *mut *mut SqlHandle,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: the required output slot is exclusive and owns no call.
    unsafe { out.write(std::ptr::null_mut()) };
    // SAFETY: the host retains the client and all counted inputs through return.
    unsafe {
        operation(error, || {
            let options = if options.is_null() && options_len == 0 {
                None
            } else {
                Some(text(options, options_len)?)
            };
            let request = thinkthen_sql::request(
                text(function, function_len)?,
                text(question, question_len)?,
                options,
            )?;
            let call = SqlCall::open(&(*client).0, &request)?;
            out.write(Box::into_raw(Box::new(SqlHandle(call))));
            Ok(0)
        })
    }
}

/// Copy one current chunk after the host requested input.
/// # Safety
/// The sole driver owns the live call and error slot. The array and its counted
/// byte views are readable for this operation; NULL is allowed for zero count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_send_chunk(
    call: *mut SqlHandle,
    records: *const SqlInput,
    count: usize,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: all foreign extents live through this synchronous operation.
    unsafe {
        operation(error, || {
            let records = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(records, count)
            };
            let chunk = records
                .iter()
                .map(|record| {
                    let value = text(record.data, record.len)?.to_owned();
                    match record.kind {
                        0 => Ok(SqlRecord::Text(value)),
                        1 => Ok(SqlRecord::Json(value)),
                        _ => Err(Error::new(ErrorKind::Record, "invalid SQL record kind")),
                    }
                })
                .collect::<Result<Vec<_>, Error>>()?;
            (*call).0.send_chunk(chunk)?;
            Ok(0)
        })
    }
}

/// Close the source; only this operation ends input and flushes its final batch.
/// # Safety
/// The sole driver owns the live call, with no operation or free concurrent.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_end_input(call: *mut SqlHandle) {
    // SAFETY: the host grants the driver exclusive call access.
    unsafe { (*call).0.end_input() };
}

/// Poll one SQL host event and probe interruption on this execution thread.
/// # Safety
/// The sole driver owns call and distinct empty outputs. The optional probe and
/// context live through return and cannot throw, unwind, longjmp or reenter.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_poll(
    call: *mut SqlHandle,
    timeout_ms: u32,
    interrupted: InterruptProbe,
    context: *mut c_void,
    out: *mut HostRow,
    error: *mut Buffer,
) -> i32 {
    // SAFETY: the row slot is exclusive writable storage with no live row.
    unsafe { out.write(HostRow::default()) };
    // SAFETY: only this driver accesses the call and invokes the caller's probe.
    unsafe {
        operation(error, || {
            let call = &mut (*call).0;
            let event = call.poll(Duration::from_millis(u64::from(timeout_ms)), || {
                interrupted.is_some_and(|probe| probe(context) != 0)
            })?;
            match event {
                Some(SqlEvent::NeedsChunk) => Ok(2),
                Some(SqlEvent::Row(row)) => {
                    out.write(row.into());
                    Ok(0)
                }
                Some(SqlEvent::Finished) => {
                    call.final_error().map_or(Ok(1), |error| Err(error.clone()))
                }
                None => Ok(3),
            }
        })
    }
}

/// Cancel and release without joining an unfinished engine worker.
/// # Safety
/// NULL or a unique live owner after all driver operations have returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_free(call: *mut SqlHandle) {
    if !call.is_null() {
        // SAFETY: the host transfers its unique allocation after active readers.
        drop(unsafe { Box::from_raw(call) });
    }
}
