//! Settings mutation never resolves credentials.
#![allow(
    unsafe_code,
    reason = "typed exports borrow foreign builders and counted fields"
)]
use crate::{
    HostError, SettingsHandle,
    exports::{THINKTHEN_OK, operation},
    memory::{live, live_mut, text},
};
use std::time::Duration;
use thinkthen::{BackendName, Batch, Retry, native};
/// Make settings with every default, without reading keys or the environment.
///
/// Returns THINKTHEN_OK with *out set. The caller owns *out and frees it with
/// thinkthen_settings_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. out is writable. error is NULL or initialized with
/// THINKTHEN_ERROR_INIT. On a negative return it holds the code and message; passing it again
/// frees what it held, so a reused error leaks nothing. Free it with thinkthen_error_free after
/// its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_new(
    out: *mut *mut SettingsHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: outputs are exclusive writable slots. */
    unsafe {
        operation(error, || {
            out.write(Box::into_raw(Box::new(SettingsHandle::default())));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Choose the backend, a thinkthen_backend.
///
/// Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_BACKEND with the settings unchanged for an
/// unknown backend.
/// # Safety
/// One thread at a time may call this on a settings object; other settings objects may be used
/// on other threads. settings is live. error is NULL or initialized with THINKTHEN_ERROR_INIT.
/// On a negative return it holds the code and message; passing it again frees what it held, so
/// a reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_backend(
    settings: *mut SettingsHandle,
    backend: u32,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver owns the live builder and error slot. */
    unsafe {
        operation(error, || {
            let handle = live_mut(settings)?;
            let backend = BackendName::from_abi(backend)
                .ok_or_else(|| thinkthen::native::argument_error(thinkthen::Code::UsageBackend))?;
            handle.0 = handle.0.clone().with_backend(backend);
            Ok(THINKTHEN_OK)
        })
    }
}
/// Set the API key; the library never prints or logs it.
///
/// Without one, the client reads the backend's key variable. Returns THINKTHEN_OK, or a
/// negative code with the settings unchanged: -THINKTHEN_CODE_USAGE_KEY_MISSING for a blank
/// key, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a settings object; other settings objects may be used
/// on other threads. settings is live. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_api_key(
    settings: *mut SettingsHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder is exclusive and the bytes remain readable until return. */
    unsafe {
        operation(error, || {
            let handle = live_mut(settings)?;
            let key = text(data, len)?.parse()?;
            handle.0 = handle.0.clone().api_key(key);
            Ok(THINKTHEN_OK)
        })
    }
}
/// Set the model name the backend uses.
///
/// Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8, with the
/// settings unchanged.
/// # Safety
/// One thread at a time may call this on a settings object; other settings objects may be used
/// on other threads. settings is live. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_model(
    settings: *mut SettingsHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver owns the builder and lends readable counted text. */
    unsafe {
        operation(error, || {
            let handle = live_mut(settings)?;
            let value = text(data, len)?;
            handle.0 = handle.0.clone().model(value);
            Ok(THINKTHEN_OK)
        })
    }
}
/// Set the backend's base URL.
///
/// Returns THINKTHEN_OK, or a negative code with the settings unchanged:
/// -THINKTHEN_CODE_USAGE_URL for a URL that is not valid, or -THINKTHEN_CODE_USAGE_ABI when the
/// text is not UTF-8.
/// # Safety
/// One thread at a time may call this on a settings object; other settings objects may be used
/// on other threads. settings is live. data points to len readable bytes, or to a NUL-
/// terminated string when len is THINKTHEN_NUL_TERMINATED; it is copied before return. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_url(
    settings: *mut SettingsHandle,
    data: *const u8,
    len: usize,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the builder and borrowed counted bytes are live through return. */
    unsafe {
        operation(error, || {
            let handle = live_mut(settings)?;
            let value = text(data, len)?;
            native::validate_url(value)?;
            handle.0 = handle.0.clone().url(value);
            Ok(THINKTHEN_OK)
        })
    }
}
/// Set how many records go to the backend in one request.
///
/// Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_BATCH with the settings unchanged when batch
/// is 0 or too large.
/// # Safety
/// One thread at a time may call this on a settings object; other settings objects may be used
/// on other threads. settings is live. error is NULL or initialized with THINKTHEN_ERROR_INIT.
/// On a negative return it holds the code and message; passing it again frees what it held, so
/// a reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_batch(
    settings: *mut SettingsHandle,
    batch: u64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver owns the builder and error slot. */
    unsafe {
        operation(error, || {
            let handle = live_mut(settings)?;
            let batch = Batch::try_from(batch)?;
            handle.0 = handle.0.clone().batch(batch);
            Ok(THINKTHEN_OK)
        })
    }
}
/// Set how many times a failed request is retried, the first wait, and each attempt's limit, in
/// milliseconds.
///
/// Returns THINKTHEN_OK, or -THINKTHEN_CODE_USAGE_SETTINGS with the settings unchanged for
/// values out of range.
/// # Safety
/// One thread at a time may call this on a settings object; other settings objects may be used
/// on other threads. settings is live. error is NULL or initialized with THINKTHEN_ERROR_INIT.
/// On a negative return it holds the code and message; passing it again frees what it held, so
/// a reused error leaks nothing. Free it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_retry(
    settings: *mut SettingsHandle,
    max_retries: u32,
    initial_backoff_ms: u64,
    attempt_timeout_ms: u64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver exclusively owns builder and error slot. */
    unsafe {
        operation(error, || {
            let handle = live_mut(settings)?;
            let retry = Retry::new(
                max_retries,
                Duration::from_millis(initial_backoff_ms),
                Duration::from_millis(attempt_timeout_ms),
            )?;
            handle.0 = handle.0.clone().retry(retry);
            Ok(THINKTHEN_OK)
        })
    }
}
/// Free settings.
///
/// Free settings. NULL is ignored. Clients opened from them keep their own copy.
/// # Safety
/// Call this once, after every other use of settings has returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_free(settings: *mut SettingsHandle) {
    if !settings.is_null() {
        /* SAFETY: the caller transfers this unique allocation. */
        drop(unsafe { Box::from_raw(settings) });
    }
}

/// Copy settings into a new settings object, without reading the environment.
///
/// Returns THINKTHEN_OK with *out set. The caller owns *out and frees it with
/// thinkthen_settings_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this while no thread changes or frees source. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_clone(
    source: *const SettingsHandle,
    out: *mut *mut SettingsHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: source remains borrowed; output slots are exclusively writable. */
    unsafe {
        operation(error, || {
            out.write(Box::into_raw(Box::new(SettingsHandle(
                live(source)?.0.clone(),
                live(source)?.1.clone(),
            ))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Copy only the API key from source into destination.
///
/// Returns THINKTHEN_OK.
/// # Safety
/// One thread at a time may change destination, and no thread changes source during the call.
/// error is NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the
/// code and message; passing it again frees what it held, so a reused error leaks nothing. Free
/// it with thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_copy_api_key(
    destination: *mut SettingsHandle,
    source: *const SettingsHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the immutable key copy finishes before destination is replaced. */
    unsafe {
        operation(error, || {
            let snapshot = live_mut(destination)?
                .0
                .clone()
                .copy_api_key(&live(source)?.0);
            live_mut(destination)?.0 = snapshot;
            Ok(THINKTHEN_OK)
        })
    }
}

/// Read settings from JSON; fields left out take their defaults.
///
/// Returns THINKTHEN_OK with *out set, or a negative code with *out set to NULL:
/// -THINKTHEN_CODE_USAGE_SETTINGS, or another usage code naming the field, for JSON that is not
/// valid settings, or -THINKTHEN_CODE_USAGE_ABI when the text is not UTF-8. The caller owns
/// *out and frees it with thinkthen_settings_free; an old handle in *out is not freed.
/// # Safety
/// Any thread may call this. data points to len readable bytes, or to a NUL-terminated string
/// when len is THINKTHEN_NUL_TERMINATED; it is copied before return. out is writable. error is
/// NULL or initialized with THINKTHEN_ERROR_INIT. On a negative return it holds the code and
/// message; passing it again frees what it held, so a reused error leaks nothing. Free it with
/// thinkthen_error_free after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_from_json(
    data: *const u8,
    len: usize,
    out: *mut *mut SettingsHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: input bytes remain borrowed; outputs are exclusively writable. */
    unsafe {
        operation(error, || {
            out.write(std::ptr::null_mut());
            let settings = thinkthen::Settings::from_json(text(data, len)?)?;
            out.write(Box::into_raw(Box::new(SettingsHandle(settings, None))));
            Ok(THINKTHEN_OK)
        })
    }
}

/// Set the lifetime HTTP-attempt allowance. Zero prevents all remote sends.
/// Existing clients are unaffected; clones of a newly opened client share its allowance.
/// # Safety
/// settings is live and exclusively borrowed. error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_max_requests(
    settings: *mut SettingsHandle,
    max_requests: u64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the caller lends a live exclusive settings builder. */
    unsafe {
        operation(error, || {
            let handle = live_mut(settings)?;
            handle.0 = handle.0.clone().max_requests(Some(max_requests));
            Ok(THINKTHEN_OK)
        })
    }
}

/// A cancellation token shared with the settings and clients that copy it.
#[derive(Debug)]
pub struct CancelHandle(thinkthen::Cancel);
/// Make a cancellation token.
/// # Safety
/// out is writable; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel_new(
    out: *mut *mut CancelHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: the output slots are exclusive until return.
    unsafe {
        operation(error, || {
            out.write(Box::into_raw(Box::new(CancelHandle(
                thinkthen::Cancel::default(),
            ))));
            Ok(THINKTHEN_OK)
        })
    }
}
/// Request cancellation from any thread.
/// # Safety
/// stop is NULL or a live token not freed concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel_request(stop: *const CancelHandle) {
    // SAFETY: the caller lends a live shared token.
    if let Ok(stop) = unsafe { live(stop) } {
        stop.0.cancel();
    }
}
/// Free the caller's token; copies held by settings and clients stay live.
/// # Safety
/// stop is NULL or uniquely transferred after its last use.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel_free(stop: *mut CancelHandle) {
    if !stop.is_null() {
        // SAFETY: caller transfers the unique handle allocation.
        drop(unsafe { Box::from_raw(stop) });
    }
}
/// Copy a cancellation token into settings.
/// # Safety
/// settings is exclusive and live; stop is live; error is NULL or initialized.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_cancel(
    settings: *mut SettingsHandle,
    stop: *const CancelHandle,
    error: *mut HostError,
) -> i32 {
    // SAFETY: settings is exclusively borrowed while stop is copied.
    unsafe {
        operation(error, || {
            let token = live(stop)?.0.clone();
            let settings = live_mut(settings)?;
            settings.1 = Some(token);
            Ok(THINKTHEN_OK)
        })
    }
}
