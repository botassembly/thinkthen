//! Thin native admission of the shared response-store configuration.
#![allow(
    unsafe_code,
    reason = "borrow counted configuration and optional scalar pointers"
)]
use crate::{
    HostError, SettingsHandle, TextView,
    exports::{THINKTHEN_OK, operation},
    memory::{live_mut, text},
};
use thinkthen::{Code, ResponseMode, ResponseStore, native};

/// Reuse eligible answers; save valid misses within configured bounds.
pub const THINKTHEN_RESPONSE_CACHE: u32 = 0;
/// Append immutable live exchanges without evicting recordings.
pub const THINKTHEN_RESPONSE_RECORD: u32 = 1;
/// Read a fixed recording snapshot without credentials or network.
pub const THINKTHEN_RESPONSE_REPLAY: u32 = 2;

/// Borrowed typed store configuration. Set struct_size to sizeof(thinkthen_response_store_config).
/// Optional numeric pointers are NULL for the shared defaults; pointed zero is invalid.
#[repr(C)]
#[derive(Debug)]
pub struct HostResponseStoreConfig {
    /// The initialized size of this configuration.
    pub struct_size: u32,
    /// Private local directory, copied before return.
    pub path: TextView,
    /// One THINKTHEN_RESPONSE_* mode.
    pub mode: u32,
    /// Caller-owned nonsecret namespace, copied before return.
    pub namespace_name: TextView,
    /// Explicit nonsecret model-generation label, copied before return.
    pub model_generation: TextView,
    /// Optional logical retained payload bound in bytes.
    pub max_bytes: *const u64,
    /// Optional distinct eligible ask bound.
    pub max_entries: *const u64,
    /// Optional live-cache age in whole seconds; recordings do not expire.
    pub max_age: *const u64,
    /// Optional SQLite busy timeout in whole milliseconds.
    pub busy_timeout_ms: *const u64,
}
#[derive(serde::Serialize)]
struct Wire<'a> {
    path: &'a str,
    mode: ResponseMode,
    namespace: &'a str,
    model_generation: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_entries: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    busy_timeout_ms: Option<u64>,
}
/// Copy an opt-in response-store configuration into settings. NULL disables storage.
/// No store is opened and no credential is read until client construction.
/// # Safety
/// settings is live and exclusively borrowed. config is NULL or initialized with
/// its full struct_size; text and optional scalar pointers remain readable until
/// return. error is NULL or initialized. Callers may release all inputs on return.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_settings_set_response_store(
    settings: *mut SettingsHandle,
    config: *const HostResponseStoreConfig,
    error: *mut HostError,
) -> i32 {
    /* SAFETY: inputs remain readable and the settings builder is exclusive. */
    unsafe {
        operation(error, || {
            let settings = live_mut(settings)?;
            let store = if config.is_null() {
                None
            } else {
                if (*config).struct_size < std::mem::size_of::<HostResponseStoreConfig>() as u32 {
                    return Err(native::argument_error(Code::UsageAbi));
                }
                let config = &*config;
                let mode = match config.mode {
                    THINKTHEN_RESPONSE_CACHE => ResponseMode::Cache,
                    THINKTHEN_RESPONSE_RECORD => ResponseMode::Record,
                    THINKTHEN_RESPONSE_REPLAY => ResponseMode::Replay,
                    _ => return Err(native::settings_conversion_error()),
                };
                let wire = Wire {
                    path: text(config.path.data, config.path.len)?,
                    mode,
                    namespace: text(config.namespace_name.data, config.namespace_name.len)?,
                    model_generation: text(
                        config.model_generation.data,
                        config.model_generation.len,
                    )?,
                    max_bytes: config.max_bytes.as_ref().copied(),
                    max_entries: config.max_entries.as_ref().copied(),
                    max_age: config.max_age.as_ref().copied(),
                    busy_timeout_ms: config.busy_timeout_ms.as_ref().copied(),
                };
                let json = serde_json::to_vec(&wire).map_err(|_| native::internal_error())?;
                Some(
                    serde_json::from_slice::<ResponseStore>(&json)
                        .map_err(|_| native::settings_conversion_error())?,
                )
            };
            settings.0 = settings.0.clone().response_store(store);
            Ok(THINKTHEN_OK)
        })
    }
}
