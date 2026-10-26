//! Settings mutation never resolves credentials.
#![allow(
    unsafe_code,
    reason = "typed exports borrow foreign builders and counted fields"
)]
use crate::{
    HostError, SettingsHandle,
    exports::{OK, operation},
    memory::text,
};
use std::time::Duration;
use thinkthen::{BackendName, Batch, Retry, native};
/// Allocate default typed settings without reading keys or environment.
/// # Safety
/// Out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_new(
    out: *mut *mut SettingsHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: outputs are exclusive writable slots. */
    unsafe {
        operation(error, || {
            out.write(Box::into_raw(Box::new(SettingsHandle::default())));
            Ok(OK)
        })
    }
}
/// Select a typed backend before opening a client.
/// # Safety
/// Settings is live and exclusively driven; error is empty writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_backend(
    settings: *mut SettingsHandle,
    backend: u32,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver owns the live builder and error slot. */
    unsafe {
        operation(error, || {
            let backend = BackendName::from_abi(backend).ok_or_else(native::abi_error)?;
            (*settings).0 = (*settings).0.clone().with_backend(backend);
            Ok(OK)
        })
    }
}
/// Copy a validated direct API key; debug output never reveals it.
/// # Safety
/// Builder and counted bytes are live; error is exclusive and empty.
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
            let key = text(data, len)?.parse()?;
            (*settings).0 = (*settings).0.clone().api_key(key);
            Ok(OK)
        })
    }
}
/// Copy a model name without resolving the backend or credentials.
/// # Safety
/// Builder and counted bytes are live; error is exclusive and empty.
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
            let value = text(data, len)?;
            (*settings).0 = (*settings).0.clone().model(value);
            Ok(OK)
        })
    }
}
/// Copy an admitted URL without reading credentials.
/// # Safety
/// Builder and counted bytes are live; error is exclusive and empty.
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
            let value = text(data, len)?;
            native::validate_url(value)?;
            (*settings).0 = (*settings).0.clone().url(value);
            Ok(OK)
        })
    }
}
/// Select a validated positive batch count.
/// # Safety
/// Builder is live and exclusive; error is empty writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_batch(
    settings: *mut SettingsHandle,
    batch: u64,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the driver owns the builder and error slot. */
    unsafe {
        operation(error, || {
            let batch = Batch::try_from(batch)?;
            (*settings).0 = (*settings).0.clone().batch(batch);
            Ok(OK)
        })
    }
}
/// Copy a validated retry policy expressed in whole milliseconds.
/// # Safety
/// Builder is live and exclusive; error is empty writable storage.
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
            let retry = Retry::new(
                max_retries,
                Duration::from_millis(initial_backoff_ms),
                Duration::from_millis(attempt_timeout_ms),
            )?;
            (*settings).0 = (*settings).0.clone().retry(retry);
            Ok(OK)
        })
    }
}
/// Release a settings builder; already opened clients own their snapshots.
/// # Safety
/// NULL or the unique live builder after all readers finish.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_free(settings: *mut SettingsHandle) {
    if !settings.is_null() {
        /* SAFETY: the caller transfers this unique allocation. */
        drop(unsafe { Box::from_raw(settings) });
    }
}

/// Copy an independent settings snapshot without resolving environment variables.
/// # Safety
/// Source is live; out and error are distinct empty writable slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_clone(
    source: *const SettingsHandle,
    out: *mut *mut SettingsHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: source remains borrowed; output slots are exclusively writable. */
    unsafe {
        operation(error, || {
            out.write(Box::into_raw(Box::new(SettingsHandle((*source).0.clone()))));
            Ok(OK)
        })
    }
}
/// Replace only the explicit key from an opaque settings snapshot.
/// # Safety
/// Both handles are live; destination is exclusive; error is empty writable storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_copy_api_key(
    destination: *mut SettingsHandle,
    source: *const SettingsHandle,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: the immutable key copy finishes before destination is replaced. */
    unsafe {
        operation(error, || {
            let snapshot = (*destination).0.clone().copy_api_key(&(*source).0);
            (*destination).0 = snapshot;
            Ok(OK)
        })
    }
}

/// Admit canonical settings with omitted fields resolved by the engine defaults.
/// # Safety
/// Counted bytes are live; out and error are distinct empty writable slots.
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
            out.write(Box::into_raw(Box::new(SettingsHandle(settings))));
            Ok(OK)
        })
    }
}
