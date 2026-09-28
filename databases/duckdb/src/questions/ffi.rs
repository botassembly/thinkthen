//! Every `unsafe` line of the caller's file system and settings: the
//! client context and file system one init holds, opens through them, and
//! set values. A missing handle fails with a pinned `defect`, so nothing
//! opens a file another way.
#![allow(
    unsafe_code,
    reason = "the caller's file system is a DuckDB C API handle"
)]

use std::ffi::CString;

use libduckdb_sys as sys;

use crate::engines::{Asked, Probe};
use crate::errors::defect;
use crate::ffi::owned_text;

/// The most bytes one `@file` read takes: a question file is small, and a
/// device such as `/dev/zero` never ends.
pub(crate) const MOST_BYTES: usize = 1024 * 1024;

/// What opening a question file through the caller's file system found.
#[derive(Debug)]
pub(crate) enum Opened {
    Text(String),
    TooLarge,
    NotText,
    Missing,
    Refused,
}

/// The caller's client context and file system, held for one init.
#[derive(Debug)]
pub(crate) struct Files {
    context: sys::duckdb_client_context,
    system: sys::duckdb_file_system,
}

/// How one open through the caller's file system ended.
enum Opening {
    Open(sys::duckdb_file_handle),
    Io,
    Other,
}

impl Files {
    /// The caller's handles from the scalar's init. A missing one is a
    /// defect: no file opens another way.
    pub(crate) fn of(info: sys::duckdb_init_info) -> Result<Self, String> {
        Self::held(client_context(info), "scalar init")
    }

    /// The caller's handles from a table function's bind (ticket 0118
    /// decision 5), under the same rule.
    pub(crate) fn of_bind(info: sys::duckdb_bind_info) -> Result<Self, String> {
        let mut context: sys::duckdb_client_context = std::ptr::null_mut();
        // SAFETY: DuckDB writes the out pointer or leaves it null.
        unsafe { sys::duckdb_table_function_get_client_context(info, &raw mut context) };
        Self::held((!context.is_null()).then_some(context), "relate bind")
    }

    /// A kept connection's handles (ticket 0129), for the warm aggregate,
    /// which has no client context of its own.
    pub(crate) fn of_connection(connection: sys::duckdb_connection) -> Result<Self, String> {
        let mut context: sys::duckdb_client_context = std::ptr::null_mut();
        // SAFETY: a live kept connection; DuckDB writes the out pointer.
        unsafe { sys::duckdb_connection_get_client_context(connection, &raw mut context) };
        Self::held((!context.is_null()).then_some(context), "warm")
    }

    fn held(context: Option<sys::duckdb_client_context>, what: &str) -> Result<Self, String> {
        let context = context
            .ok_or_else(|| defect(&format!("the {what} got no client context from DuckDB")))?;
        let Some(system) = file_system(context) else {
            let mut context = context;
            // SAFETY: the context came from DuckDB and is destroyed once.
            unsafe { sys::duckdb_destroy_client_context(&raw mut context) };
            return Err(defect(&format!(
                "the {what} got no file system from DuckDB"
            )));
        };
        Ok(Self { context, system })
    }

    /// The caller's client context, valid while these handles live.
    pub(crate) const fn context(&self) -> sys::duckdb_client_context {
        self.context
    }

    /// One whole-number setting; unset, NULL, or `RESET` reads `None`.
    pub(crate) fn number(&self, name: &std::ffi::CStr) -> Option<i64> {
        self.setting(name).and_then(|value| value.number)
    }

    /// One text setting under the same rule.
    pub(crate) fn text(&self, name: &std::ffi::CStr) -> Option<String> {
        self.setting(name).and_then(|value| value.text)
    }

    /// Read one question file through the caller's file system. An IO error
    /// reads as missing; any other error refuses.
    pub(crate) fn read(&self, path: &str) -> Opened {
        let mut handle = match self.open(path) {
            Opening::Open(handle) => handle,
            Opening::Io => return Opened::Missing,
            Opening::Other => return Opened::Refused,
        };
        let mut bytes = Vec::new();
        let mut buffer = vec![0_u8; 64 * 1024];
        let whole = loop {
            if bytes.len() > MOST_BYTES {
                break false;
            }
            // SAFETY: the buffer holds `len` writable bytes.
            let read = unsafe {
                sys::duckdb_file_handle_read(
                    handle,
                    buffer.as_mut_ptr().cast(),
                    i64::try_from(buffer.len()).unwrap_or(0),
                )
            };
            match usize::try_from(read) {
                Ok(0) => break true,
                Ok(count) => bytes.extend_from_slice(buffer.get(..count).unwrap_or_default()),
                Err(_) => break false,
            }
        };
        // SAFETY: the handle is open and is closed and destroyed once.
        unsafe {
            sys::duckdb_file_handle_close(handle);
            sys::duckdb_destroy_file_handle(&raw mut handle);
        }
        if bytes.len() > MOST_BYTES {
            return Opened::TooLarge;
        }
        if !whole {
            return Opened::Missing;
        }
        String::from_utf8(bytes).map_or(Opened::NotText, Opened::Text)
    }

    /// Open `path` for reading only to learn whether the caller's settings
    /// allow it. Success or an IO error allows; anything else refuses.
    pub(crate) fn probe(&self, path: &str) -> Probe {
        match self.open(path) {
            Opening::Open(mut handle) => {
                // SAFETY: as in `read`.
                unsafe {
                    sys::duckdb_file_handle_close(handle);
                    sys::duckdb_destroy_file_handle(&raw mut handle);
                }
                Probe::Allowed
            }
            Opening::Io => Probe::Allowed,
            Opening::Other => Probe::Refused,
        }
    }

    fn open(&self, path: &str) -> Opening {
        let Ok(path) = CString::new(path) else {
            return Opening::Other;
        };
        let mut handle: sys::duckdb_file_handle = std::ptr::null_mut();
        // SAFETY: valid handles and a NUL-terminated path; each created
        // object is destroyed once.
        unsafe {
            let mut options = sys::duckdb_create_file_open_options();
            sys::duckdb_file_open_options_set_flag(
                options,
                sys::duckdb_file_flag_DUCKDB_FILE_FLAG_READ,
                true,
            );
            let state =
                sys::duckdb_file_system_open(self.system, path.as_ptr(), options, &raw mut handle);
            sys::duckdb_destroy_file_open_options(&raw mut options);
            if state == sys::duckdb_state_DuckDBSuccess && !handle.is_null() {
                return Opening::Open(handle);
            }
            let mut error = sys::duckdb_file_system_error_data(self.system);
            let kind = sys::duckdb_error_data_error_type(error);
            sys::duckdb_destroy_error_data(&raw mut error);
            if kind == sys::duckdb_error_type_DUCKDB_ERROR_IO {
                Opening::Io
            } else {
                Opening::Other
            }
        }
    }

    /// The four session settings; an unset or `RESET` one reads `None`.
    pub(crate) fn settings(&self) -> Asked {
        Asked {
            throttle: self
                .setting(c"thinkthen_throttle")
                .and_then(|value| value.number),
            max_requests: self
                .setting(c"thinkthen_max_requests")
                .and_then(|value| value.number),
            cache: self
                .setting(c"thinkthen_cache")
                .and_then(|value| value.text),
            max_requests_total: self
                .setting(c"thinkthen_max_requests_total")
                .and_then(|value| value.number),
            ..Asked::default()
        }
    }

    fn setting(&self, name: &std::ffi::CStr) -> Option<Setting> {
        // SAFETY: a valid context and name; the value is destroyed once.
        unsafe {
            let mut scope = sys::duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_INVALID;
            let mut value = sys::duckdb_client_context_get_config_option(
                self.context,
                name.as_ptr(),
                &raw mut scope,
            );
            if value.is_null() {
                return None;
            }
            let read = (!sys::duckdb_is_null_value(value)).then(|| Setting {
                number: Some(sys::duckdb_get_int64(value)),
                text: owned_text(sys::duckdb_get_varchar(value)),
            });
            sys::duckdb_destroy_value(&raw mut value);
            read
        }
    }
}

impl Drop for Files {
    fn drop(&mut self) {
        // SAFETY: both handles came from DuckDB and are destroyed once.
        unsafe {
            sys::duckdb_destroy_file_system(&raw mut self.system);
            sys::duckdb_destroy_client_context(&raw mut self.context);
        }
    }
}

/// One set value, read both ways; the setting's registered type decides
/// which one the caller uses.
struct Setting {
    number: Option<i64>,
    text: Option<String>,
}

fn client_context(info: sys::duckdb_init_info) -> Option<sys::duckdb_client_context> {
    let mut context: sys::duckdb_client_context = std::ptr::null_mut();
    // SAFETY: DuckDB writes the out pointer or leaves it null.
    unsafe { sys::duckdb_scalar_function_init_get_client_context(info, &raw mut context) };
    (!context.is_null()).then_some(context)
}

fn file_system(context: sys::duckdb_client_context) -> Option<sys::duckdb_file_system> {
    // SAFETY: a valid context.
    let system = unsafe { sys::duckdb_client_context_get_file_system(context) };
    (!system.is_null()).then_some(system)
}
