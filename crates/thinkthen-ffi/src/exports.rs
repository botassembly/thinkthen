//! Version, ownership and canonical call operations.
#![allow(
    unsafe_code,
    reason = "exports borrow counted foreign inputs and own supported output prefixes"
)]
use crate::memory::{error_size, live, row_size, text};
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
/// Success: the function did what it says, or wrote the next row.
pub const THINKTHEN_OK: i32 = 0;
/// A call has no more rows; failed records do not change this.
pub const THINKTHEN_END: i32 = 1;
/// A streaming call waits for the next record or thinkthen_call_end_input.
pub const THINKTHEN_NEEDS_INPUT: i32 = 2;
/// Not sure; no answer was written. This is not an engine error code.
pub const THINKTHEN_UNSURE: i32 = 4;
/// Kept for compatibility. Failures return the negative of a thinkthen_code, never this value.
pub const THINKTHEN_ERROR: i32 = -1;
pub(crate) fn guarded<T>(run: impl FnOnce() -> T) -> std::thread::Result<T> {
    catch_unwind(AssertUnwindSafe(run))
}

pub(crate) unsafe fn operation(
    error: *mut HostError,
    run: impl FnOnce() -> Result<i32, Error>,
) -> i32 {
    /* SAFETY: only the first size field is read until its supported prefix is validated. */
    let size = unsafe { error_size(error) };
    if !error.is_null() && size.is_none() {
        return -Code::UsageAbi.abi();
    }
    if let Some(size) = size {
        /* SAFETY: an initialized error owns its message, which an earlier call may
         * have left; freeing it here lets the caller reuse the variable. */
        unsafe {
            HostError::clear(error, size);
        }
    }
    match guarded(run).unwrap_or_else(|_| Err(native::internal_error())) {
        Ok(status) => status,
        Err(failure) => {
            if let Some(size) = size {
                /* SAFETY: the supported prefix is writable and exclusively owned. */
                unsafe {
                    HostError::from_error(size, &failure).write(error);
                }
            }
            -failure.code().abi()
        }
    }
}
/// Fill error with the shared usage.settings error, for a binding that cannot convert its own
/// settings.
///
/// Returns -THINKTHEN_CODE_USAGE_SETTINGS.
/// # Safety
/// Any thread may call this. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_conversion_error(error: *mut HostError) -> i32 {
    /* SAFETY: the caller lends the sized error slot until return. */
    unsafe { operation(error, || Err(native::settings_conversion_error())) }
}
/// Fill error with the shared usage.question error, for a binding that cannot convert its own
/// question.
///
/// Returns -THINKTHEN_CODE_USAGE_QUESTION.
/// # Safety
/// Any thread may call this. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_question_conversion_error(error: *mut HostError) -> i32 {
    /* SAFETY: the caller lends the sized error slot until return. */
    unsafe { operation(error, || Err(native::question_conversion_error())) }
}
/// Fill error with the shared local.cancelled error, for a binding that stops a call itself.
///
/// Returns -THINKTHEN_CODE_LOCAL_CANCELLED.
/// # Safety
/// Any thread may call this. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancelled_error(error: *mut HostError) -> i32 {
    /* SAFETY: the caller lends the sized error slot until return. */
    unsafe { operation(error, || Err(native::cancelled_error())) }
}
/// Return the loaded library's binary interface version.
///
/// THINKTHEN_ABI_MAJOR is in the high 16 bits and THINKTHEN_ABI_MINOR in the low 16. Pass it
/// with THINKTHEN_ABI_VERSION to thinkthen_abi_compatible. Any thread may call this.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_abi_version() -> u32 {
    thinkthen::ABI_VERSION
}
/// Return the loaded library's version, such as "0.2.0".
///
/// The string is NUL-terminated and lasts for the whole process; never free it. Compare it with
/// THINKTHEN_VERSION. Any thread may call this.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}
/// Return a code's dotted name, such as "record.empty", or NULL for an unknown code.
///
/// value may be positive or negative. The string is NUL-terminated and lasts for the whole
/// process; never free it. Any thread may call this.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_code_name(value: i32) -> *const c_char {
    code_metadata(value).map_or(ptr::null(), |(_, name, _)| name.as_ptr())
}
/// Return a code's family, such as "record", or NULL for an unknown code or 0.
///
/// The string is NUL-terminated and lasts for the whole process; never free it. Any thread may
/// call this.
#[unsafe(no_mangle)]
pub extern "C" fn thinkthen_code_family(value: i32) -> *const c_char {
    code_metadata(value).map_or(ptr::null(), |(_, _, family)| family.as_ptr())
}
fn code_metadata(value: i32) -> Option<&'static (i32, std::ffi::CString, std::ffi::CString)> {
    let value = value.checked_abs()?;
    static CODES: OnceLock<Vec<(i32, std::ffi::CString, std::ffi::CString)>> = OnceLock::new();
    CODES
        .get_or_init(|| {
            Code::ALL
                .iter()
                .filter_map(|code| {
                    Some((
                        code.abi(),
                        std::ffi::CString::new(code.as_str()).ok()?,
                        std::ffi::CString::new(code.family().as_str()).ok()?,
                    ))
                })
                .collect()
        })
        .iter()
        .find(|(code, _, _)| *code == value)
}
/// Open a client from settings, or from the environment when settings is NULL.
///
/// With NULL settings, THINKTHEN_BACKEND picks the backend and the backend's own variables give
/// the key and URL. THINKTHEN_MAX_REQUESTS supplies the client allowance only on this
/// explicitly from-environment path. The client copies the settings, so they may be freed at once. Returns
/// THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
/// -THINKTHEN_CODE_USAGE_BACKEND for an unknown THINKTHEN_BACKEND,
/// -THINKTHEN_CODE_USAGE_KEY_MISSING when the backend needs a key and none is set, or another
/// usage code for settings that are not valid. The caller owns *out and frees it with
/// thinkthen_client_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. settings is NULL or live, and no thread changes it during the
/// call. out is writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative
/// return it holds the code and message; passing it again frees what it held, so a reused error
/// leaks nothing. Free it with thinkthen_error_free after its last use.
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
            let stop = settings.as_ref().and_then(|settings| settings.1.clone());
            let settings = match settings.as_ref() {
                Some(settings) => settings.0.clone(),
                None => native::environment_settings(|name| std::env::var(name).ok())?,
            };
            let engine = crate::engine(settings)?;
            let engine = match stop {
                Some(stop) => engine.cancel(stop),
                None => engine,
            };
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Open a client on one backend with every other setting at its default.
///
/// This is the shortcut for thinkthen_settings_new, thinkthen_settings_set_backend,
/// thinkthen_client_open and thinkthen_settings_free. The key comes from the backend's own
/// environment variable, and the stub backend needs none. Returns THINKTHEN_OK with *out set,
/// or a negative code with *out set to NULL: -THINKTHEN_CODE_USAGE_BACKEND for an unknown
/// backend, or -THINKTHEN_CODE_USAGE_KEY_MISSING when the backend needs a key and none is set.
/// The caller owns *out and frees it with thinkthen_client_free; an old handle in *out is not
/// freed.
/// # Safety
/// Any thread may call this. out is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_open_backend(
    backend: u32,
    out: *mut *mut ClientHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends an exclusive output slot. */
    unsafe {
        operation(error, || {
            out.write(ptr::null_mut());
            let backend = thinkthen::BackendName::from_abi(backend)
                .ok_or_else(|| native::argument_error(Code::UsageBackend))?;
            let engine = crate::engine(Settings::default().with_backend(backend))?;
            out.write(Box::into_raw(Box::new(ClientHandle(engine))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Open a client from settings JSON.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
/// for settings that are not valid, -THINKTHEN_CODE_USAGE_KEY_MISSING when the backend needs a
/// key and none is set, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller
/// owns *out and frees it with thinkthen_client_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. settings points to len readable bytes, or to a NUL-terminated
/// string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable.
/// error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
/// code and message; passing it again frees what it held, so a reused error leaks nothing. Free
/// it with thinkthen_error_free after its last use.
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
            Ok(THINKTHEN_OK)
        })
    }
}
/// Copy cumulative client usage into an independently owned JSON buffer.
///
/// The snapshot remains valid after the client is freed. Active attempts are
/// unresolved; this is client-wide accounting, not SQL statement accounting.
/// Free the returned bytes with thinkthen_buffer_free.
/// # Safety
/// client is live, out is writable and empty, and error is NULL or initialized.
/// No thread may free client until this function returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_usage(
    client: *const ClientHandle,
    out: *mut Buffer,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends a live client and exclusive output slots. */
    unsafe {
        operation(error, || {
            out.write(Buffer::default());
            let client = client
                .as_ref()
                .ok_or_else(|| native::argument_error(Code::UsageAbi))?;
            let text =
                serde_json::to_string(&client.0.usage()).map_err(|_| native::internal_error())?;
            out.write(Buffer::new(text));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Wait for owned calls to settle and copy final cumulative client usage.
///
/// First stop admitting calls and cancel/free every owned unfinished call.
/// Unlike thinkthen_client_usage, this waits for their engine workers to end.
/// The returned JSON has the same independently owned format.
/// # Safety
/// client is live; no thread opens new calls or frees it during this function.
/// out is writable and empty, and error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_usage_settled(
    client: *const ClientHandle,
    out: *mut Buffer,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends a live client and exclusive output slots. */
    unsafe {
        operation(error, || {
            out.write(Buffer::default());
            let client = client
                .as_ref()
                .ok_or_else(|| native::argument_error(Code::UsageAbi))?;
            let text = serde_json::to_string(&client.0.settled_usage())
                .map_err(|_| native::internal_error())?;
            out.write(Buffer::new(text));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Free a client.
///
/// Free a client. NULL is ignored. Calls opened from it keep running on their own.
/// # Safety
/// Call this once, after every other use of client has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_client_free(client: *mut ClientHandle) {
    if !client.is_null() {
        /* SAFETY: the caller transfers its unique allocation. */
        drop(unsafe { Box::from_raw(client) });
    }
}
/// The request that asks question, under name when it is not NULL.
pub(crate) unsafe fn named_request(
    question: *const QuestionHandle,
    name: *const TextView,
) -> Result<Request, Error> {
    /* SAFETY: the caller lends a live question and a NULL or readable name until return. */
    unsafe {
        let request = Request::new(live(question)?.build()?);
        match name.as_ref() {
            Some(name) => request.named(text(name.data, name.len)?),
            None => Ok(request),
        }
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
    let offsets = OffsetUnit::from_abi(offsets)
        .ok_or_else(|| native::argument_error(thinkthen::Code::UsageOffsets))?;
    /* SAFETY: the client remains live while the engine snapshot is retained. */
    let call = native::Call::open(&unsafe { live(client) }?.0, request, offsets)?;
    /* SAFETY: an independent owned call is published into the empty slot. */
    unsafe {
        out.write(Box::into_raw(Box::new(CallHandle::new(call, request))));
    }
    Ok(THINKTHEN_OK)
}
/// Open a streaming call that asks question about each record you send.
///
/// offsets is a thinkthen_offset; C uses THINKTHEN_OFFSET_UTF8. name, when not NULL, names the
/// run in each row's source; a present empty name is refused. Read the call with
/// thinkthen_call_next. Returns THINKTHEN_OK with *out set, or a negative code with *out set to
/// NULL: -THINKTHEN_CODE_USAGE_QUESTION for an incomplete question, -THINKTHEN_CODE_USAGE_NAME
/// for a bad name, -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit, or
/// -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it with
/// thinkthen_call_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes question. client and question are live;
/// both may be freed after return. name is NULL or points to a readable thinkthen_text. out is
/// writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
            open(client, &named_request(question, name)?, offsets, out)
        })
    }
}
/// Open a call from request JSON, for a host that speaks JSON; question files are never read.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL: a usage code
/// for a request that is not valid, -THINKTHEN_CODE_USAGE_OFFSETS for an unknown offsets unit,
/// or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns *out and frees it
/// with thinkthen_call_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. client is live. request points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is
/// writable. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it
/// holds the code and message; passing it again frees what it held, so a reused error leaks
/// nothing. Free it with thinkthen_error_free after its last use.
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
/// Send one text record to a call.
///
/// Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
/// -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. Text that is not UTF-8 is
/// sent as a record that fails as record.invalid, and blank text fails as record.empty; neither
/// reaches the backend.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. data points
/// to len readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED;
/// it is copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
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
            let driver = driven(call)?;
            driver.send(crate::finite::admit_record(
                data,
                len,
                thinkthen::records::RecordKind::Text.abi(),
            )?)?;

            Ok(THINKTHEN_OK)
        })
    }
}
/// Send one JSON record to a call.
///
/// Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
/// -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. JSON that is not valid is
/// sent as a record that fails as record.json.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. data points
/// to len readable bytes, or to a NUL-terminated string when len is THINKTHEN_NUL_TERMINATED;
/// it is copied before return. error is NULL or initialized with THINKTHEN_ERROR_INIT. On a
/// negative return it holds the code and message; passing it again frees what it held, so a
/// reused error leaks nothing. Free it with thinkthen_error_free after its last use.
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
            let record =
                crate::finite::admit_record(data, len, thinkthen::records::RecordKind::Json.abi())?;
            let driver = driven(call)?;
            driver.send(record)?;

            Ok(THINKTHEN_OK)
        })
    }
}
/// Send one UTF-16 text record, in the machine's byte order, to a call.
///
/// Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
/// -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. Text with an unpaired
/// surrogate is sent as a record that fails as record.invalid.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. data points
/// to len readable, aligned uint16_t units; it may be NULL when len is 0. error is NULL or
/// initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
/// passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            let driver = driven(call)?;
            driver.send(Record::from_utf16(units))?;

            Ok(THINKTHEN_OK)
        })
    }
}
/// Send a missing value, such as a SQL NULL, to a call.
///
/// Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
/// -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. It fails as record.missing
/// without a backend request.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. error is NULL
/// or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_send_missing(
    call: *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: only the driver accesses this call cell and output slot. */
    unsafe {
        operation(error, || {
            let driver = driven(call)?;
            driver.send(Record::missing())?;

            Ok(THINKTHEN_OK)
        })
    }
}
/// Send a value the host could not convert to a call.
///
/// Send only after thinkthen_call_next returns THINKTHEN_NEEDS_INPUT. Returns THINKTHEN_OK, or
/// -THINKTHEN_CODE_USAGE_ABI when the call is not waiting for input. It fails as record.invalid
/// without a backend request.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. error is NULL
/// or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_send_invalid(
    call: *mut CallHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: only the driver accesses this call cell and output slot. */
    unsafe {
        operation(error, || {
            let driver = driven(call)?;
            driver.send(Record::invalid())?;

            Ok(THINKTHEN_OK)
        })
    }
}
/// Say no more records are coming.
///
/// Rows already sent still arrive; thinkthen_call_next returns THINKTHEN_END after the last. It
/// returns at once.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. call is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_end_input(call: *mut CallHandle) {
    /* SAFETY: only the driver accesses the call cell. */
    if let Ok(driver) = unsafe { driven(call) } {
        driver.end_input();
    }
}
/// Read the call's next event: a row, a request for input, or the end.
///
/// Returns THINKTHEN_OK with the next row in *out; THINKTHEN_NEEDS_INPUT when a streaming call
/// waits for a send or thinkthen_call_end_input; THINKTHEN_END when every row is read; or a
/// negative code for the call as a whole, such as -THINKTHEN_CODE_LOCAL_CANCELLED. A record
/// that fails is still THINKTHEN_OK: its row has status THINKTHEN_STATUS_FAILED and its own
/// code, and reading goes on. *out is empty unless the return is THINKTHEN_OK. A row's
/// input comes from thinkthen_row_original. After a call-level error, every later read returns
/// that same error until the call is freed; a failed call never returns THINKTHEN_END.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. out is
/// initialized with THINKTHEN_ROW_INIT. Passing it again frees the row it held, so a reused row
/// leaks nothing; free it with thinkthen_row_free after its last use. error is NULL or
/// initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
/// passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
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
            HostRow::clear(out);
            if live(call)?.filter && live(call)?.c_surface {
                return Err(native::argument_error(Code::UsageFunction));
            }
            match next(call)? {
                Next::Status(status) => Ok(status),
                Next::Row(row) => {
                    publish(*row, size, out, live(call)?.domain.clone())?;
                    Ok(THINKTHEN_OK)
                }
            }
        })
    }
}
enum Next {
    Status(i32),
    Row(Box<thinkthen::AnyRow>),
}
// Only the sole driver borrows the call cell; cancellation owns a separate token.
unsafe fn driven<'a>(call: *const CallHandle) -> Result<&'a mut native::Call, Error> {
    /* SAFETY: only the sole driver accesses this cell of a live call. */
    unsafe { Ok(&mut *live(call)?.call.get()) }
}
unsafe fn next(call: *mut CallHandle) -> Result<Next, Error> {
    /* SAFETY: the sole driver owns the mutable call borrow until the event is copied. */
    unsafe {
        Ok(match driven(call)?.next_event()? {
            Event::NeedsInput => Next::Status(THINKTHEN_NEEDS_INPUT),
            Event::Finished => Next::Status(THINKTHEN_END),
            Event::Row(row) => Next::Row(Box::new(row)),
            _ => return Err(native::abi_error()),
        })
    }
}
unsafe fn publish(
    row: thinkthen::AnyRow,
    size: u32,
    out: *mut HostRow,
    domain: std::sync::Arc<crate::row::Domain>,
) -> Result<(), Error> {
    let result = HostRow::from_value(size, crate::values::ValueHandle::from_any(row), domain)?;
    /* SAFETY: out is a validated exclusive sized output slot. */
    unsafe {
        result.write(out);
    }
    Ok(())
}
/// Read the next kept record of a filter call, skipping the records it drops.
///
/// A single cut keeps answered-yes records and skips answered-no records.
/// thinkthen_call_next stays the internal adapter view of every row. Returns THINKTHEN_OK with *row
/// holding a kept row (thinkthen_row_original gives its input); THINKTHEN_NEEDS_INPUT when a
/// streaming call waits for a send; THINKTHEN_END when every record is judged; or a negative
/// code. Every kept record is returned before one terminal error summarizing all failed
/// records. Read their owned rows with thinkthen_call_next_failed. The terminal error leaves
/// *row empty; further kept reads return THINKTHEN_END.
/// -THINKTHEN_CODE_USAGE_FUNCTION means the call is not a filter call.
/// # Safety
/// One thread at a time drives call; thinkthen_call_cancel may run on any thread. row is
/// initialized with THINKTHEN_ROW_INIT. Passing it again frees the row it held, so a reused row
/// leaks nothing; free it with thinkthen_row_free after its last use. error is NULL or
/// initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and message;
/// passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_next_kept(
    call: *mut CallHandle,
    row: *mut HostRow,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: size is validated before writing any supported output field. */
    unsafe {
        operation(error, || {
            let size = row_size(row)?;
            HostRow::clear(row);
            if !live(call)?.filter {
                return Err(native::argument_error(Code::UsageFunction));
            }
            if (*live(call)?.kept.get()).finished {
                return Ok(THINKTHEN_END);
            }
            loop {
                match next(call)? {
                    Next::Status(THINKTHEN_END) => {
                        let state = &mut *live(call)?.kept.get();
                        state.finished = true;
                        return match &state.first {
                            Some((first, position)) => Err(native::records_failed(
                                first,
                                state.failures,
                                state.total,
                                *position,
                            )),
                            None => Ok(THINKTHEN_END),
                        };
                    }
                    Next::Status(status) => return Ok(status),
                    Next::Row(item) => {
                        let state = &mut *live(call)?.kept.get();
                        state.total = item.position();
                        if let Some(failure) = item.failure() {
                            state.failures = state.failures.saturating_add(1);
                            if state.first.is_none() {
                                state.first = Some((failure.clone(), item.position()));
                            }
                            state.failed.push_back(*item);
                            continue;
                        }
                        if matches!(&*item, thinkthen::AnyRow::Filter(item)
                            if matches!(item.outcome(), thinkthen::Outcome::Answered { answer: true, .. }))
                        {
                            publish(*item, size, row, live(call)?.domain.clone())?;
                            return Ok(THINKTHEN_OK);
                        }
                    }
                }
            }
        })
    }
}
/// Read the next failed result retained by a filter's kept-record operation.
///
/// Returns THINKTHEN_OK with *row written, or THINKTHEN_END when no retained failures remain.
/// Reading failures performs no backend work. A retained row owns its original record and
/// remains valid after the call is freed. -THINKTHEN_CODE_USAGE_FUNCTION refuses non-filter calls.
/// # Safety
/// One thread at a time drives call. row is initialized with THINKTHEN_ROW_INIT; passing it
/// again frees its previous contents. error is NULL or initialized with THINKTHEN_ERROR_INIT.
/// Free row and error after their last use, with thinkthen_row_free and thinkthen_error_free.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_next_failed(
    call: *mut CallHandle,
    row: *mut HostRow,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the sole driver lends a live call and exclusive sized output slots. */
    unsafe {
        operation(error, || {
            let size = row_size(row)?;
            HostRow::clear(row);
            let handle = live(call)?;
            if !handle.filter {
                return Err(native::argument_error(Code::UsageFunction));
            }
            match (&mut *handle.kept.get()).failed.pop_front() {
                Some(item) => {
                    publish(item, size, row, live(call)?.domain.clone())?;
                    Ok(THINKTHEN_OK)
                }
                None => Ok(THINKTHEN_END),
            }
        })
    }
}
/// Ask a call to stop.
///
/// It returns at once. Once the call stops, thinkthen_call_next returns
/// -THINKTHEN_CODE_LOCAL_CANCELLED, and a wait on the call's events wakes.
/// # Safety
/// Any thread may call this, also while another thread drives call. call is live until every
/// such call returns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_cancel(call: *mut CallHandle) {
    /* SAFETY: cancel accesses only its independently shared thread-safe token. */
    if let Some(call) = unsafe { call.as_ref() } {
        call.cancel.cancel();
    }
}
/// Cancel and free a call.
///
/// Free a call. NULL is ignored. It does not wait for a backend request in flight. Rows already
/// read stay valid.
/// # Safety
/// Call this once, after every other use of call, including thinkthen_call_cancel, has
/// returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_free(call: *mut CallHandle) {
    if !call.is_null() {
        /* SAFETY: the host transfers its unique call allocation. */
        drop(unsafe { Box::from_raw(call) });
    }
}
/// Free a buffer's contents and leave it empty.
///
/// NULL and an empty buffer are ignored, so a freed buffer may be freed again.
/// # Safety
/// Call this after the last read of buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_buffer_free(buffer: *mut Buffer) {
    if !buffer.is_null() {
        /* SAFETY: cleanup owns the allocation and writable slot. */
        unsafe {
            (*buffer).release();
        }
    }
}
/// Free an error's message and leave it empty.
///
/// NULL, an empty error and an error not initialized with THINKTHEN_ERROR_INIT are ignored, so
/// a freed error may be freed again or passed again.
/// # Safety
/// Call this after the last read of error.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_free(error: *mut HostError) {
    /* SAFETY: only its first field is read until a supported prefix is known. */
    if let Some(size) = unsafe { error_size(error) } {
        /* SAFETY: the supported owned message and prefix are exclusively writable. */
        unsafe {
            HostError::clear(error, size);
        }
    }
}
/// Free a row's contents and leave it empty.
///
/// NULL, an empty row and a row not initialized with THINKTHEN_ROW_INIT are ignored, so a freed
/// row may be freed again or passed again. Views borrowed from the row end here.
/// # Safety
/// Call this after the last read of row and its views.
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
