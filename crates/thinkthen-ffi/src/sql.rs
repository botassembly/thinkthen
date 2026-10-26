//! Counted foreign views over the single canonical SQL driver.
#![allow(
    unsafe_code,
    reason = "SQL borrows counted views and publishes owned sized outputs"
)]
use crate::{ClientHandle, HostError, HostRow, QuestionHandle, builders::TextView};
use crate::{
    exports::{END, NEEDS_INPUT, OK, operation},
    memory::{bytes, row_size, text},
};
use std::{ffi::c_void, ptr, time::Duration};
use thinkthen::{Request, native};
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord};
/// No event arrived during this bounded SQL wait.
pub const THINKTHEN_PENDING: i32 = 3;
/// One statement's driver; no database handle enters the engine.
#[derive(Debug)]
pub struct SqlHandle {
    call: SqlCall,
    error_reported: bool,
}
/// Counted input copied before a database cursor advances.
#[repr(C)]
#[derive(Debug)]
pub struct SqlInput {
    /// Generated record kind; missing and invalid need no byte view.
    pub kind: u32,
    /// Readable bytes, or NULL at zero length.
    pub data: *const u8,
    /// Byte length without a terminator.
    pub len: usize,
}
/// Read native interrupt state without throwing, unwinding or reentering.
pub type InterruptProbe = Option<unsafe extern "C" fn(*mut c_void) -> u8>;
/// Admit SQL settings and optional administrator policy before resolving keys.
/// # Safety
/// Counted inputs are readable; out/error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_client_open_with_policy(
    settings: *const u8,
    len: usize,
    allowed: *const u8,
    allowed_len: usize,
    out: *mut *mut ClientHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: caller lends counted inputs and exclusive supported outputs.
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let policy = if allowed.is_null() && allowed_len == 0 {
                None
            } else {
                Some(text(allowed, allowed_len)?)
            };
            let engine = thinkthen_sql::client_with_policy(text(settings, len)?, policy, |name| {
                std::env::var(name).ok()
            })?;
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(OK)
        })
    }
}
/// Admit SQL settings without an administrator backend/model restriction.
/// # Safety
/// Counted settings are readable; out/error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_client_open(
    settings: *const u8,
    len: usize,
    out: *mut *mut ClientHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: caller lends counted inputs and exclusive supported outputs.
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let engine =
                thinkthen_sql::client(text(settings, len)?, |name| std::env::var(name).ok())?;
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(OK)
        })
    }
}
/// Open a SQL call from a copied typed question snapshot.
/// # Safety
/// Handles are live; output/error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_open_question(
    client: *const ClientHandle,
    question: *const QuestionHandle,
    name: *const TextView,
    out: *mut *mut SqlHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: caller retains all handles and grants exclusive outputs.
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let mut request = Request::new((*question).0.build()?);
            if !name.is_null() {
                request = request.named(text((*name).data, (*name).len)?)?;
            }
            out.write(Box::into_raw(Box::new(SqlHandle {
                call: SqlCall::open(&(*client).0, &request)?,
                error_reported: false,
            })));
            Ok(OK)
        })
    }
}
/// Admit a structured SQL question with the shared per-function contract.
/// This consumes a question value, never a settings envelope or file reference.
/// # Safety
/// Counted input is readable; output/error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_question_from_json(
    data: *const u8,
    len: usize,
    out: *mut *mut QuestionHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: caller lends the complete input and exclusive outputs until return.
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let question = native::sql_question(text(data, len)?)?;
            out.write(Box::into_raw(Box::new(QuestionHandle(
                native::QuestionBuilder::snapshot(&question),
            ))));
            Ok(OK)
        })
    }
}
/// Admit SQL's optional cut or paired band without overriding conflicting inputs.
/// # Safety
/// Builder/error are exclusive; each non-NULL number pointer is readable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_question_set_threshold(
    question: *mut QuestionHandle,
    cut: *const f64,
    low: *const f64,
    high: *const f64,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller grants a live builder and readable optional number views.
    unsafe {
        operation(error, || {
            native::sql_threshold(
                &mut (*question).0,
                cut.as_ref().copied(),
                low.as_ref().copied(),
                high.as_ref().copied(),
            )?;
            Ok(OK)
        })
    }
}
/// Open an explicit canonical JSON request without an argument translation layer.
/// # Safety
/// Client/input remain live; output/error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_open_json(
    client: *const ClientHandle,
    data: *const u8,
    len: usize,
    out: *mut *mut SqlHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: caller lends counted input and exclusive outputs.
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let request = Request::from_json(text(data, len)?)?;
            out.write(Box::into_raw(Box::new(SqlHandle {
                call: SqlCall::open(&(*client).0, &request)?,
                error_reported: false,
            })));
            Ok(OK)
        })
    }
}
/// Copy one current chunk after NEEDS_INPUT. Bad host text becomes a failed row.
/// # Safety
/// The sole driver owns call/error. The array lends count readable records.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_send_chunk(
    call: *mut SqlHandle,
    records: *const SqlInput,
    count: usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the driver retains every borrowed view for the shared send operation.
    unsafe { send_chunk(call, records, count, error, false) }
}
/// Send PostgreSQL input with shared zero-byte JSON representation admission.
/// # Safety
/// The sole driver owns the call/error and borrows count readable input records.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_send_chunk_postgres(
    call: *mut SqlHandle,
    records: *const SqlInput,
    count: usize,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the driver retains every borrowed view for the shared send operation.
    unsafe { send_chunk(call, records, count, error, true) }
}
unsafe fn send_chunk(
    call: *mut SqlHandle,
    records: *const SqlInput,
    count: usize,
    error: *mut HostError,
    postgres: bool,
) -> i32 {
    // SAFETY: caller lends every counted view through this operation.
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
                    use thinkthen::records::RecordKind;
                    match RecordKind::from_abi(record.kind) {
                        Some(RecordKind::Missing) => Ok(SqlRecord::Missing),
                        Some(RecordKind::Invalid) => Ok(SqlRecord::Invalid),
                        Some(RecordKind::Text) => {
                            Ok(match std::str::from_utf8(bytes(record.data, record.len)) {
                                Ok(text) => SqlRecord::Text(text.to_owned()),
                                Err(_) => SqlRecord::Invalid,
                            })
                        }
                        Some(RecordKind::Json) => {
                            Ok(match std::str::from_utf8(bytes(record.data, record.len)) {
                                Ok(text) if postgres => SqlRecord::PostgresJson(text.to_owned()),
                                Ok(text) => SqlRecord::Json(text.to_owned()),
                                Err(_) => SqlRecord::Invalid,
                            })
                        }
                        _ => Err(native::abi_error()),
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            (*call).call.send_chunk(chunk)?;
            Ok(OK)
        })
    }
}
/// End the source and flush its final partial batch.
/// # Safety
/// The sole driver owns a live call with no concurrent free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_end_input(call: *mut SqlHandle) {
    // SAFETY: caller grants exclusive access.
    unsafe { (*call).call.end_input() };
}
/// Poll on the database thread and probe interruption around the wait.
/// # Safety
/// The sole driver owns call, sized row/error and optional callback context.
/// The probe cannot throw, unwind, longjmp or reenter.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_poll(
    call: *mut SqlHandle,
    timeout_ms: u32,
    interrupted: InterruptProbe,
    context: *mut c_void,
    out: *mut HostRow,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller grants the same ownership as the shared polling operation.
    unsafe { poll(call, timeout_ms, interrupted, context, out, error, false) }
}
/// Poll a PostgreSQL row after shared zero-byte representation admission.
/// # Safety
/// The sole driver owns call and supported row/error outputs and borrows the probe.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_poll_postgres(
    call: *mut SqlHandle,
    timeout_ms: u32,
    interrupted: InterruptProbe,
    context: *mut c_void,
    out: *mut HostRow,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller grants the same ownership as the shared polling operation.
    unsafe { poll(call, timeout_ms, interrupted, context, out, error, true) }
}
unsafe fn poll(
    call: *mut SqlHandle,
    timeout_ms: u32,
    interrupted: InterruptProbe,
    context: *mut c_void,
    out: *mut HostRow,
    error: *mut HostError,
    postgres: bool,
) -> i32 {
    // SAFETY: supported sizes precede any writes or driver work.
    unsafe {
        operation(error, || {
            let size = row_size(out)?;
            out.write(HostRow::empty(size));
            let handle = &mut *call;
            let call = &mut handle.call;
            match call.poll(Duration::from_millis(u64::from(timeout_ms)), || {
                interrupted.is_some_and(|probe| probe(context) != 0)
            })? {
                Some(SqlEvent::NeedsChunk) => Ok(NEEDS_INPUT),
                Some(SqlEvent::Row(row)) => {
                    let row = if postgres {
                        native::sql_postgres_row(row)?
                    } else {
                        row
                    };
                    out.write(HostRow::from_row(size, row)?);
                    Ok(OK)
                }
                Some(SqlEvent::Finished) => {
                    if handle.error_reported {
                        return Ok(END);
                    }
                    handle.error_reported = true;
                    call.final_error()
                        .map_or(Ok(END), |error| Err(error.clone()))
                }
                None => Ok(THINKTHEN_PENDING),
            }
        })
    }
}
/// Copy a selected PostgreSQL id, or replace an unrepresentable id with a failed row.
/// A NULL view preserves the engine id; a present JSON null remains an explicit id.
/// # Safety
/// Row/error are live exclusive supported outputs; an optional id view is readable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_row_postgres_id(
    row: *mut HostRow,
    id: *const TextView,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller owns the row and lends its optional selected JSON value.
    unsafe {
        operation(error, || {
            row_size(row)?;
            if id.is_null() {
                return Ok(OK);
            }
            let value = text((*id).data, (*id).len)?;
            let failure = native::sql_postgres_id_error(value)?;
            let row = &mut *row;
            row.id.release();
            if let Some(failure) = failure {
                row.answer.release();
                row.detail.release();
                row.error.release();
                row.status = thinkthen::results::Status::Failed.abi();
                row.error_code = failure.code().abi();
                row.probability = f64::NAN;
                row.answer_bool = 0;
                row.rank = 0;
                row.number = 0.0;
                row.error = crate::Buffer::new(
                    serde_json::to_string(&failure).map_err(|_| native::internal_error())?,
                );
            } else {
                row.id = crate::Buffer::new(value.to_owned());
            }
            Ok(OK)
        })
    }
}
/// Cancel and free without joining the backend worker.
/// # Safety
/// NULL or a unique live owner after driver operations finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_free(call: *mut SqlHandle) {
    if !call.is_null() {
        // SAFETY: caller transfers this unique allocation.
        drop(unsafe { Box::from_raw(call) });
    }
}

/// Request cancellation on the statement's owning database thread.
/// # Safety
/// The sole driver owns the live call with no concurrent operation or free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_cancel(call: *mut SqlHandle) {
    // SAFETY: caller retains the live statement handle.
    unsafe {
        (*call).call.cancel();
    }
}
