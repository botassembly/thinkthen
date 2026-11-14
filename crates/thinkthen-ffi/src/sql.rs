//! Counted foreign views over the single canonical SQL driver.
#![allow(
    unsafe_code,
    reason = "SQL borrows counted views and publishes owned sized outputs"
)]
use crate::{ClientHandle, HostError, HostRow, QuestionHandle, builders::TextView};
use crate::{
    exports::{THINKTHEN_END, THINKTHEN_NEEDS_INPUT, THINKTHEN_OK, named_request, operation},
    memory::{bytes, live, live_mut, row_size, text},
};
use std::{ffi::c_void, ptr, time::Duration};
use thinkthen::{Request, native};
use thinkthen_sql::{SqlCall, SqlEvent, SqlRecord};
/// thinkthen_sql_call_poll's timeout passed with nothing to report; poll again.
pub const THINKTHEN_PENDING: i32 = 3;
/// A database extension's call for one statement; free it with thinkthen_sql_call_free.
#[derive(Debug)]
pub struct SqlHandle {
    call: SqlCall,
    error_reported: bool,
    domain: std::sync::Arc<crate::row::Domain>,
}
/// One database row's input for thinkthen_sql_call_send_chunk; copied before return.
#[repr(C)]
#[derive(Debug)]
pub struct SqlInput {
    /// How to read data, a thinkthen_record_kind; missing and invalid rows need no data.
    pub kind: u32,
    /// The row's bytes; NULL only when len is 0.
    pub data: *const u8,
    /// The byte length; not NUL-terminated.
    pub len: usize,
}
/// A database's interrupt check: returns nonzero to stop the call. It must not throw,
/// unwind, long-jump or call back into this library.
pub type InterruptProbe = Option<unsafe extern "C" fn(*mut c_void) -> u8>;
/// Open a client for a database extension from settings JSON and the administrator's allowed
/// backends and models.
///
/// allowed is JSON, or empty for no limit. Returns THINKTHEN_OK with *out set, or a negative
/// code with *out set to NULL: -THINKTHEN_CODE_USAGE_SQL_POLICY for a policy that is not valid,
/// -THINKTHEN_CODE_USAGE_SQL_SELECTION when it does not allow the chosen backend and model,
/// another usage code for settings that are not valid, or -THINKTHEN_CODE_USAGE_ABI when the
/// text is not UTF-8. The caller owns *out and frees it with thinkthen_client_free; an old
/// handle in *out is not freed.
/// # Safety
/// Any thread may call this. settings points to len readable bytes, or to a NUL-terminated
/// string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. allowed points to
/// allowed_len readable bytes, or to a NUL-terminated string when allowed_len is
/// THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is NULL or
/// initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
/// passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            let engine = crate::engine(native::sql_settings_with_policy(
                text(settings, len)?,
                policy,
                |name| std::env::var(name).ok(),
            )?)?;
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Open a client for a database extension from settings JSON, with no administrator limit.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
/// for settings that are not valid, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// The caller owns *out and frees it with thinkthen_client_free; an old handle in *out is not
/// freed.
/// # Safety
/// Any thread may call this. settings points to len readable bytes, or to a NUL-terminated
/// string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable.
/// error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
/// code and message; passing it again frees what it held, so a reused error leaks nothing. Free
/// it with thinkthen_error_free after its last use.
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
            let engine = crate::engine(native::sql_settings(text(settings, len)?)?)?;
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Open a database call that asks question about each chunk of rows.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
/// -THINKTHEN_CODE_USAGE_QUESTION for an incomplete question, -THINKTHEN_CODE_USAGE_NAME for a
/// bad name, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and
/// frees it with thinkthen_sql_call_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes question. name is NULL or points to a
/// readable thinkthen_text. out is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
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
            let request = named_request(question, name)?;
            out.write(Box::into_raw(Box::new(SqlHandle {
                call: SqlCall::open(&live(client)?.0, &request)?,
                error_reported: false,
                domain: std::sync::Arc::new(crate::row::Domain::from_question(request.question())),
            })));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Read one question from a database function's JSON argument.
///
/// Returns THINKTHEN_OK with *out set to a new question, or a negative code with *out set to
/// NULL: -THINKTHEN_CODE_USAGE_QUESTION when the JSON is not a valid question, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it with
/// thinkthen_question_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
/// when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            out.write(Box::into_raw(Box::new(QuestionHandle::new(
                native::QuestionBuilder::snapshot(&question),
            ))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Set a database function's optional cut, or its band from low to high.
///
/// Each pointer may be NULL to leave that value out; low and high come together. Returns
/// THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_THRESHOLD with the question unchanged for values that
/// are out of range, conflict, or do not fit the function.
/// # Safety
/// One thread at a time may call this on a question; other questions may be used on other
/// threads. question is a live question. Each non-NULL pointer points to a readable double.
/// error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
/// code and message; passing it again frees what it held, so a reused error leaks nothing. Free
/// it with thinkthen_error_free after its last use.
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
                &mut live_mut(question)?.builder,
                cut.as_ref().copied(),
                low.as_ref().copied(),
                high.as_ref().copied(),
            )?;
            Ok(THINKTHEN_OK)
        })
    }
}
/// Open a database call from request JSON.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
/// for a request that is not valid, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// The caller owns *out and frees it with thinkthen_sql_call_free; an old handle in *out is not
/// freed.
/// # Safety
/// Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
/// when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
                call: SqlCall::open(&live(client)?.0, &request)?,
                error_reported: false,
                domain: std::sync::Arc::new(crate::row::Domain::from_question(request.question())),
            })));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Select whether completion reports failed records; filter always reports them.
/// Rows remain available until the final statement failure. enabled is 0 or 1.
/// # Safety
/// One database thread drives call. error is NULL or an initialized, exclusive output.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_set_raise_if_failed(
    call: *mut SqlHandle,
    enabled: u8,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller retains its live call and exclusive error slot. */
    unsafe {
        operation(error, || {
            if enabled > 1 {
                return Err(native::abi_error());
            }
            live_mut(call)?.call.set_raise_if_failed(enabled != 0);
            Ok(THINKTHEN_OK)
        })
    }
}

/// Send count database rows to a call.
///
/// Send only after thinkthen_sql_call_poll returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK,
/// or -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input or a record is
/// malformed. A value that is not valid text fails as its own row.
/// # Safety
/// One database thread drives call. records points to count readable records whose bytes are
/// copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
/// return it holds the code and message; passing it again frees what it held, so a reused error
/// leaks nothing. Free it with thinkthen_error_free after its last use.
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
/// Send count PostgreSQL rows to a call, refusing a zero byte in JSON as PostgreSQL does.
///
/// Send only after thinkthen_sql_call_poll returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK,
/// or -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input or a record is
/// malformed. A value that is not valid text fails as its own row.
/// # Safety
/// One database thread drives call. records points to count readable records whose bytes are
/// copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
/// return it holds the code and message; passing it again frees what it held, so a reused error
/// leaks nothing. Free it with thinkthen_error_free after its last use.
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
            live_mut(call)?.call.send_chunk(chunk)?;
            Ok(THINKTHEN_OK)
        })
    }
}
/// Say no more rows are coming, and send the last partial batch.
///
/// It returns at once.
/// # Safety
/// One database thread drives call. call is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_end_input(call: *mut SqlHandle) {
    // SAFETY: caller grants exclusive access.
    let _ = crate::exports::guarded(|| {
        if let Ok(call) = unsafe { live_mut(call) } {
            call.call.end_input();
        }
    });
}
/// Wait for the next row, request for input, or end, on the database thread.
///
/// Waits at most timeout_ms, calling interrupted(context) around the wait; when it returns
/// nonzero the call stops with -THINKTHEN_CODE_LOCAL_CANCELLED. Returns THINKTHEN_OK with the
/// next row in *out, THINKTHEN_NEEDS_INPUT, THINKTHEN_END, or a negative code for the call as a
/// whole. A failed record is THINKTHEN_OK with a failed row. *out is overwritten, not freed, so
/// free it with thinkthen_row_free before polling again.
/// # Safety
/// One database thread drives call. interrupted is NULL or a function that returns without
/// throwing, unwinding, long-jumping or calling back into this library. out is initialized with
/// THINKTHEN_ROW_INIT. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
/// return it holds the code and message; passing it again frees what it held, so a reused error
/// leaks nothing. Free it with thinkthen_error_free after its last use.
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
/// Wait for the next row as thinkthen_sql_call_poll does, refusing text PostgreSQL cannot
/// store.
///
/// Waits at most timeout_ms, calling interrupted(context) around the wait; when it returns
/// nonzero the call stops with -THINKTHEN_CODE_LOCAL_CANCELLED. Returns THINKTHEN_OK with the
/// next row in *out, THINKTHEN_NEEDS_INPUT, THINKTHEN_END, or a negative code for the call as a
/// whole. A failed record is THINKTHEN_OK with a failed row. *out is overwritten, not freed, so
/// free it with thinkthen_row_free before polling again.
/// # Safety
/// One database thread drives call. interrupted is NULL or a function that returns without
/// throwing, unwinding, long-jumping or calling back into this library. out is initialized with
/// THINKTHEN_ROW_INIT. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
/// return it holds the code and message; passing it again frees what it held, so a reused error
/// leaks nothing. Free it with thinkthen_error_free after its last use.
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
            HostRow::empty(size).write(out);
            let handle = live_mut(call)?;
            let call = &mut handle.call;
            let timeout = Duration::from_millis(u64::from(timeout_ms));
            let probe = || interrupted.is_some_and(|probe| probe(context) != 0);
            let event = if postgres {
                call.poll_postgres(timeout, probe)?
            } else {
                call.poll(timeout, probe)?
            };
            match event {
                Some(SqlEvent::NeedsChunk) => Ok(THINKTHEN_NEEDS_INPUT),
                Some(SqlEvent::Row(row)) => {
                    HostRow::from_value(
                        size,
                        crate::values::ValueHandle::from_any(row),
                        handle.domain.clone(),
                    )?
                    .write(out);
                    Ok(THINKTHEN_OK)
                }
                Some(SqlEvent::Finished) => {
                    if handle.error_reported {
                        return Ok(THINKTHEN_END);
                    }
                    handle.error_reported = true;
                    call.final_error()
                        .map_or(Ok(THINKTHEN_END), |error| Err(error.clone()))
                }
                None => Ok(THINKTHEN_PENDING),
            }
        })
    }
}
/// Give a PostgreSQL row the id the query selected.
///
/// id NULL keeps the row's id. An id PostgreSQL cannot store turns the row into a failed row.
/// Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_ABI when row did not come from this library.
/// The row is changed in place; free it with thinkthen_row_free.
/// # Safety
/// One database thread drives call. row is the most recent row this call wrote, before the
/// next row or completion. id is NULL or points to a
/// readable thinkthen_text. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_row_postgres_id(
    call: *mut SqlHandle,
    row: *mut HostRow,
    id: *const TextView,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the caller owns the row and lends its optional selected JSON value.
    unsafe {
        operation(error, || {
            row_size(row)?;
            let call = &mut live_mut(call)?.call;
            if id.is_null() {
                return Ok(THINKTHEN_OK);
            }
            let value = text((*id).data, (*id).len)?;
            let failure = native::sql_postgres_id_error(value)?;
            if let Some(failure) = &failure {
                call.observe_row_failure((*row).position, failure)?;
            }
            let data = crate::row::data(row)?;
            if let Some(failure) = failure {
                let crate::values::Owned::Row(typed) = &data.value.0 else {
                    return Err(native::abi_error());
                };
                let replacement = HostRow::from_value(
                    (*row).struct_size,
                    crate::values::ValueHandle::from_any(native::row_failed(
                        typed.clone(),
                        failure,
                    )),
                    data.domain.clone(),
                )?;
                HostRow::clear(row);
                replacement.write(row);
            } else {
                let raw = serde_json::value::RawValue::from_string(value.to_owned())
                    .map_err(|_| native::abi_error())?;
                let crate::values::Owned::Row(typed) = &data.value.0 else {
                    return Err(native::abi_error());
                };
                let replacement = HostRow::from_value(
                    (*row).struct_size,
                    crate::values::ValueHandle::from_any(native::row_selected_id(
                        typed.clone(),
                        value,
                    )),
                    data.domain.clone(),
                )?;
                HostRow::clear(row);
                replacement.write(row);
                (*(*row).private_data).id_override = Some(raw);
            }
            Ok(THINKTHEN_OK)
        })
    }
}
/// Cancel and free a database call.
///
/// Free a database call. NULL is ignored. It does not wait for a backend request in flight.
/// # Safety
/// Call this once, after every other use of call has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_free(call: *mut SqlHandle) {
    let _ = crate::exports::guarded(|| {
        if !call.is_null() {
            // SAFETY: caller transfers this unique allocation.
            drop(unsafe { Box::from_raw(call) });
        }
    });
}

/// Ask a database call to stop.
///
/// It returns at once.
/// # Safety
/// One database thread drives call. call is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_sql_call_cancel(call: *mut SqlHandle) {
    // SAFETY: caller retains the live statement handle.
    let _ = crate::exports::guarded(|| {
        if let Ok(call) = unsafe { live(call) } {
            call.call.cancel();
        }
    });
}
