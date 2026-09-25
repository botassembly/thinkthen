//! Every `unsafe` line of the DuckDB C API: types, chunk reads and writes,
//! the caller's file system and settings, registration, and each callback.
//!
//! Each callback runs its body through [`guarded`], so a panic becomes that
//! callback's error and never unwinds into DuckDB. The wrappers that fetch a
//! client context or a file system return `Option`, so a missing handle
//! fails with a pinned `defect` and nothing opens a file another way.
#![allow(unsafe_code, reason = "the DuckDB extension API is a C API")]

use std::ffi::{CString, c_char, c_void};

use libduckdb_sys as sys;

use crate::engines::{self, Asked, Probe};
use crate::errors::{defect, guarded};
use crate::questions::Caller;
use crate::scalars::{SCALARS, Scalar};
use crate::signal::{self, Invoke};
use crate::tables::{Warm, usage_rows};

/// A SQL type this extension reads or returns.
#[derive(Debug)]
pub(crate) enum Type {
    Bool,
    Double,
    BigInt,
    Text,
    List(&'static Type),
    Struct(&'static [(&'static str, Type)]),
}

/// One value to write into a result row.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    Double(f64),
    Int(i64),
    Text(String),
    List(Vec<Value>),
    Struct(Vec<Value>),
}

/// One input column of a chunk; `None` is a NULL row.
#[derive(Debug)]
pub(crate) enum Column {
    Texts(Vec<Option<String>>),
    Lists(Vec<Option<Vec<String>>>),
    Ints(Vec<Option<i64>>),
}

impl Column {
    pub(crate) fn present(&self, row: usize) -> bool {
        match self {
            Self::Texts(rows) => rows.get(row).is_some_and(Option::is_some),
            Self::Lists(rows) => rows.get(row).is_some_and(Option::is_some),
            Self::Ints(rows) => rows.get(row).is_some_and(Option::is_some),
        }
    }

    pub(crate) fn text(&self, row: usize) -> Option<&str> {
        match self {
            Self::Texts(rows) => rows.get(row)?.as_deref(),
            _ => None,
        }
    }

    pub(crate) fn list(&self, row: usize) -> Option<&[String]> {
        match self {
            Self::Lists(rows) => rows.get(row)?.as_deref(),
            _ => None,
        }
    }

    pub(crate) fn int(&self, row: usize) -> Option<i64> {
        match self {
            Self::Ints(rows) => *rows.get(row)?,
            _ => None,
        }
    }
}

/// What opening a question file through the caller's file system found.
#[derive(Debug)]
pub(crate) enum Opened {
    Text(String),
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
    fn of(info: sys::duckdb_init_info) -> Result<Self, String> {
        let context = client_context(info)
            .ok_or_else(|| defect("the scalar init got no client context from DuckDB"))?;
        let Some(system) = file_system(context) else {
            let mut context = context;
            // SAFETY: the context came from DuckDB and is destroyed once.
            unsafe { sys::duckdb_destroy_client_context(&raw mut context) };
            return Err(defect("the scalar init got no file system from DuckDB"));
        };
        Ok(Self { context, system })
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
            cache_bytes: self
                .setting(c"thinkthen_cache_bytes")
                .and_then(|value| value.number),
            max_requests_total: self
                .setting(c"thinkthen_max_requests_total")
                .and_then(|value| value.number),
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

/// Copy and free a string DuckDB allocated.
fn owned_text(raw: *mut c_char) -> Option<String> {
    if raw.is_null() {
        return None;
    }
    // SAFETY: DuckDB returns a NUL-terminated string the caller frees.
    unsafe {
        let text = std::ffi::CStr::from_ptr(raw).to_string_lossy().into_owned();
        sys::duckdb_free(raw.cast());
        Some(text)
    }
}

/// A logical type, destroyed once when dropped. Every logical type this
/// extension creates is made here (R4-16).
struct Logical(sys::duckdb_logical_type);

impl Logical {
    fn new(of: &Type) -> Self {
        // SAFETY: each parent copies its child types, and the children drop
        // after it is made.
        Self(unsafe {
            match of {
                Type::Bool => sys::duckdb_create_logical_type(sys::DUCKDB_TYPE_DUCKDB_TYPE_BOOLEAN),
                Type::Double => {
                    sys::duckdb_create_logical_type(sys::DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE)
                }
                Type::BigInt => {
                    sys::duckdb_create_logical_type(sys::DUCKDB_TYPE_DUCKDB_TYPE_BIGINT)
                }
                Type::Text => sys::duckdb_create_logical_type(sys::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR),
                Type::List(child) => sys::duckdb_create_list_type(Self::new(child).0),
                Type::Struct(fields) => {
                    let members: Vec<Self> =
                        fields.iter().map(|(_, field)| Self::new(field)).collect();
                    let mut raw: Vec<sys::duckdb_logical_type> =
                        members.iter().map(|member| member.0).collect();
                    let names: Vec<CString> =
                        fields.iter().map(|(name, _)| message(name)).collect();
                    let mut pointers: Vec<*const c_char> =
                        names.iter().map(|name| name.as_ptr()).collect();
                    sys::duckdb_create_struct_type(
                        raw.as_mut_ptr(),
                        pointers.as_mut_ptr(),
                        raw.len() as u64,
                    )
                }
            }
        })
    }
}

impl Drop for Logical {
    fn drop(&mut self) {
        // SAFETY: made once in `new`, destroyed once here.
        unsafe { sys::duckdb_destroy_logical_type(&raw mut self.0) };
    }
}

fn valid(vector: sys::duckdb_vector, row: usize) -> bool {
    // SAFETY: a null validity mask means every row is valid.
    unsafe {
        let mask = sys::duckdb_vector_get_validity(vector);
        mask.is_null() || sys::duckdb_validity_row_is_valid(mask, row as u64)
    }
}

fn text_at(vector: sys::duckdb_vector, row: usize) -> String {
    // SAFETY: a VARCHAR vector holds a `duckdb_string_t` per row.
    unsafe {
        let data = sys::duckdb_vector_get_data(vector)
            .cast::<sys::duckdb_string_t>()
            .add(row);
        let length = sys::duckdb_string_t_length(*data) as usize;
        let start = sys::duckdb_string_t_data(data).cast::<u8>();
        String::from_utf8_lossy(std::slice::from_raw_parts(start, length)).into_owned()
    }
}

/// Read one input column of `rows` rows as the registered type says.
fn read_column(vector: sys::duckdb_vector, of: &Type, rows: usize) -> Column {
    match of {
        Type::BigInt => Column::Ints(
            (0..rows)
                .map(|row| {
                    // SAFETY: a BIGINT vector holds an i64 per row.
                    valid(vector, row).then(|| unsafe {
                        *sys::duckdb_vector_get_data(vector).cast::<i64>().add(row)
                    })
                })
                .collect(),
        ),
        Type::List(_) => {
            // SAFETY: a LIST vector holds a list entry per row over one child.
            let (entries, child) = unsafe {
                (
                    sys::duckdb_vector_get_data(vector).cast::<sys::duckdb_list_entry>(),
                    sys::duckdb_list_vector_get_child(vector),
                )
            };
            Column::Lists(
                (0..rows)
                    .map(|row| {
                        if !valid(vector, row) {
                            return None;
                        }
                        // SAFETY: as above.
                        let entry = unsafe { *entries.add(row) };
                        // A NULL member makes the row NULL.
                        (entry.offset..entry.offset + entry.length)
                            .map(|at| {
                                valid(child, at as usize).then(|| text_at(child, at as usize))
                            })
                            .collect()
                    })
                    .collect(),
            )
        }
        _ => Column::Texts(
            (0..rows)
                .map(|row| valid(vector, row).then(|| text_at(vector, row)))
                .collect(),
        ),
    }
}

/// Write one value at `row` of an output vector of type `of`.
fn write(vector: sys::duckdb_vector, of: &Type, row: usize, value: &Value) {
    // SAFETY: each vector has room for `row`, and a list child is reserved
    // before it is written.
    unsafe {
        match (of, value) {
            (Type::Bool, Value::Bool(held)) => {
                *sys::duckdb_vector_get_data(vector).cast::<bool>().add(row) = *held
            }
            (Type::Double, Value::Double(held)) => {
                *sys::duckdb_vector_get_data(vector).cast::<f64>().add(row) = *held
            }
            (Type::BigInt, Value::Int(held)) => {
                *sys::duckdb_vector_get_data(vector).cast::<i64>().add(row) = *held
            }
            (Type::Text, Value::Text(held)) => {
                sys::duckdb_vector_assign_string_element_len(
                    vector,
                    row as u64,
                    held.as_ptr().cast(),
                    held.len() as u64,
                );
            }
            (Type::List(inner), Value::List(items)) => {
                let child = sys::duckdb_list_vector_get_child(vector);
                let offset = sys::duckdb_list_vector_get_size(vector);
                let length = items.len() as u64;
                sys::duckdb_list_vector_reserve(vector, offset + length);
                for (at, item) in (offset..).zip(items) {
                    write(child, inner, at as usize, item);
                }
                sys::duckdb_list_vector_set_size(vector, offset + length);
                *sys::duckdb_vector_get_data(vector)
                    .cast::<sys::duckdb_list_entry>()
                    .add(row) = sys::duckdb_list_entry { offset, length };
            }
            (Type::Struct(fields), Value::Struct(members)) => {
                for (index, ((_, field), member)) in fields.iter().zip(members).enumerate() {
                    write(
                        sys::duckdb_struct_vector_get_child(vector, index as u64),
                        field,
                        row,
                        member,
                    );
                }
            }
            _ => {
                sys::duckdb_vector_ensure_validity_writable(vector);
                sys::duckdb_validity_set_row_invalid(
                    sys::duckdb_vector_get_validity(vector),
                    row as u64,
                );
                if let Type::Struct(fields) = of {
                    for (index, (_, field)) in fields.iter().enumerate() {
                        write(
                            sys::duckdb_struct_vector_get_child(vector, index as u64),
                            field,
                            row,
                            &Value::Null,
                        );
                    }
                }
            }
        }
    }
}

fn message(text: &str) -> CString {
    CString::new(text.replace('\0', " ")).unwrap_or_default()
}

// ---- scalars ----

/// The per-thread init: the caller's handles, its settings, the cache
/// folder's check, and the engine, before any row runs.
unsafe extern "C" fn scalar_init(info: sys::duckdb_init_info) {
    let made = guarded("scalar init", || {
        let files = Files::of(info)?;
        let asked = files.settings();
        let engine = engines::engine_for(&asked, |path| files.probe(path))?;
        Ok(Box::new(Caller::new(engine, asked, files)))
    });
    // SAFETY: DuckDB owns the state and calls `drop_caller` once.
    unsafe {
        match made {
            Ok(caller) => sys::duckdb_scalar_function_init_set_state(
                info,
                Box::into_raw(caller).cast(),
                Some(drop_caller),
            ),
            Err(text) => sys::duckdb_scalar_function_init_set_error(info, message(&text).as_ptr()),
        }
    }
}

unsafe extern "C" fn drop_caller(state: *mut c_void) {
    let _ = guarded("scalar state destruction", || {
        if !state.is_null() {
            // SAFETY: the state is the box `scalar_init` leaked.
            drop(unsafe { Box::from_raw(state.cast::<Caller>()) });
        }
        Ok(())
    });
}

unsafe extern "C" fn scalar_invoke(
    info: sys::duckdb_function_info,
    input: sys::duckdb_data_chunk,
    output: sys::duckdb_vector,
) {
    if let Err(text) = guarded("scalar", || answer(info, input, output)) {
        // SAFETY: a valid function info.
        unsafe { sys::duckdb_scalar_function_set_error(info, message(&text).as_ptr()) };
    }
}

fn answer(
    info: sys::duckdb_function_info,
    input: sys::duckdb_data_chunk,
    output: sys::duckdb_vector,
) -> Result<(), String> {
    // SAFETY: extra info is a `&'static Scalar`; the state is this thread's
    // own `Caller`, or null when DuckDB ran no init.
    let (scalar, caller, rows, count) = unsafe {
        (
            sys::duckdb_scalar_function_get_extra_info(info)
                .cast::<Scalar>()
                .as_ref(),
            sys::duckdb_scalar_function_get_state(info)
                .cast::<Caller>()
                .as_mut(),
            sys::duckdb_data_chunk_get_size(input) as usize,
            sys::duckdb_data_chunk_get_column_count(input),
        )
    };
    let scalar = scalar.ok_or_else(|| defect("the scalar ran with no function record"))?;
    let caller = caller.ok_or_else(|| defect("the scalar ran with no init state"))?;
    let columns: Vec<Column> = (0..count)
        .map(|index| {
            let of = scalar
                .arguments
                .get(index as usize)
                .unwrap_or(&Type::BigInt);
            // SAFETY: the index is below the chunk's column count.
            read_column(
                unsafe { sys::duckdb_data_chunk_get_vector(input, index) },
                of,
                rows,
            )
        })
        .collect();
    let invoke = Invoke::begin();
    let values = crate::scalars::run(scalar.verb, caller, &invoke, &columns, rows)?;
    for (row, value) in values.iter().enumerate() {
        write(output, &scalar.result, row, value);
    }
    Ok(())
}

fn register_scalar(
    connection: sys::duckdb_connection,
    scalar: &'static Scalar,
) -> Result<(), String> {
    let name = message(scalar.name);
    // SAFETY: each created object is copied on add or register, then
    // destroyed once.
    unsafe {
        let mut set = sys::duckdb_create_scalar_function_set(name.as_ptr());
        for deadline in [false, true]
            .into_iter()
            .take(if scalar.deadline { 2 } else { 1 })
        {
            let mut function = sys::duckdb_create_scalar_function();
            sys::duckdb_scalar_function_set_name(function, name.as_ptr());
            let deadline_type = deadline.then_some(&Type::BigInt);
            for argument in scalar.arguments.iter().chain(deadline_type) {
                let made = Logical::new(argument);
                sys::duckdb_scalar_function_add_parameter(function, made.0);
            }
            let result = Logical::new(&scalar.result);
            sys::duckdb_scalar_function_set_return_type(function, result.0);
            sys::duckdb_scalar_function_set_function(function, Some(scalar_invoke));
            sys::duckdb_scalar_function_set_init(function, Some(scalar_init));
            sys::duckdb_scalar_function_set_volatile(function);
            sys::duckdb_scalar_function_set_extra_info(
                function,
                std::ptr::from_ref(scalar).cast_mut().cast(),
                None,
            );
            sys::duckdb_add_scalar_function_to_set(set, function);
            sys::duckdb_destroy_scalar_function(&raw mut function);
        }
        let state = sys::duckdb_register_scalar_function_set(connection, set);
        sys::duckdb_destroy_scalar_function_set(&raw mut set);
        if state == sys::duckdb_state_DuckDBSuccess {
            Ok(())
        } else {
            Err(format!("{} did not register", scalar.name))
        }
    }
}

fn register_setting(
    connection: sys::duckdb_connection,
    name: &std::ffi::CStr,
    of: &Type,
) -> Result<(), String> {
    // SAFETY: as in `register_scalar`.
    unsafe {
        let mut option = sys::duckdb_create_config_option();
        sys::duckdb_config_option_set_name(option, name.as_ptr());
        let made = Logical::new(of);
        sys::duckdb_config_option_set_type(option, made.0);
        sys::duckdb_config_option_set_default_scope(
            option,
            sys::duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_SESSION,
        );
        sys::duckdb_config_option_set_description(
            option,
            c"a thinkthen engine setting (ADR 0017 section 5)".as_ptr(),
        );
        let state = sys::duckdb_register_config_option(connection, option);
        sys::duckdb_destroy_config_option(&raw mut option);
        if state == sys::duckdb_state_DuckDBSuccess {
            Ok(())
        } else {
            Err(format!("{name:?} did not register"))
        }
    }
}

// ---- thinkthen_usage() ----

unsafe extern "C" fn drop_flag(flag: *mut c_void) {
    let _ = guarded("usage state destruction", || {
        if !flag.is_null() {
            // SAFETY: the flag is a box this file leaked.
            drop(unsafe { Box::from_raw(flag.cast::<bool>()) });
        }
        Ok(())
    });
}

unsafe extern "C" fn usage_bind(info: sys::duckdb_bind_info) {
    let bound = guarded("usage bind", || {
        // SAFETY: each type is copied by the bind, then destroyed.
        unsafe {
            let text = Logical::new(&Type::Text);
            sys::duckdb_bind_add_result_column(info, c"metric".as_ptr(), text.0);
            let number = Logical::new(&Type::BigInt);
            sys::duckdb_bind_add_result_column(info, c"value".as_ptr(), number.0);
        }
        Ok(())
    });
    if let Err(text) = bound {
        // SAFETY: a valid bind info.
        unsafe { sys::duckdb_bind_set_error(info, message(&text).as_ptr()) };
    }
}

unsafe extern "C" fn usage_init(info: sys::duckdb_init_info) {
    let made = guarded("usage init", || Ok(Box::new(false)));
    // SAFETY: DuckDB owns the flag and calls `drop_flag` once.
    unsafe {
        match made {
            Ok(flag) => {
                sys::duckdb_init_set_init_data(info, Box::into_raw(flag).cast(), Some(drop_flag))
            }
            Err(text) => sys::duckdb_init_set_error(info, message(&text).as_ptr()),
        }
    }
}

unsafe extern "C" fn usage_scan(info: sys::duckdb_function_info, output: sys::duckdb_data_chunk) {
    let scanned = guarded("usage scan", || {
        // SAFETY: the init data is this scan's flag; the output has two
        // columns with room for four rows.
        unsafe {
            let done = sys::duckdb_function_get_init_data(info)
                .cast::<bool>()
                .as_mut()
                .ok_or_else(|| defect("the usage scan ran with no init state"))?;
            if *done {
                sys::duckdb_data_chunk_set_size(output, 0);
                return Ok(());
            }
            *done = true;
            let rows = usage_rows();
            let (names, values) = (
                sys::duckdb_data_chunk_get_vector(output, 0),
                sys::duckdb_data_chunk_get_vector(output, 1),
            );
            for (row, (name, count)) in rows.iter().enumerate() {
                write(names, &Type::Text, row, &Value::Text((*name).to_owned()));
                write(values, &Type::BigInt, row, &Value::Int(*count));
            }
            sys::duckdb_data_chunk_set_size(output, rows.len() as u64);
        }
        Ok(())
    });
    if let Err(text) = scanned {
        // SAFETY: a valid function info and chunk.
        unsafe {
            sys::duckdb_function_set_error(info, message(&text).as_ptr());
            sys::duckdb_data_chunk_set_size(output, 0);
        }
    }
}

fn register_usage(connection: sys::duckdb_connection) -> Result<(), String> {
    // SAFETY: as in `register_scalar`.
    unsafe {
        let mut table = sys::duckdb_create_table_function();
        sys::duckdb_table_function_set_name(table, c"thinkthen_usage".as_ptr());
        sys::duckdb_table_function_set_bind(table, Some(usage_bind));
        sys::duckdb_table_function_set_init(table, Some(usage_init));
        sys::duckdb_table_function_set_function(table, Some(usage_scan));
        let state = sys::duckdb_register_table_function(connection, table);
        sys::duckdb_destroy_table_function(&raw mut table);
        if state == sys::duckdb_state_DuckDBSuccess {
            Ok(())
        } else {
            Err("thinkthen_usage did not register".to_owned())
        }
    }
}

// ---- thinkthen_warm(question, text) ----

/// A warm state block holds one pointer to a boxed [`Warm`]. DuckDB moves
/// state blocks by byte copy, which keeps the pointer.
fn warm_at<'a>(state: sys::duckdb_aggregate_state) -> Result<&'a mut Warm, String> {
    // SAFETY: `warm_init` wrote a leaked box into every block.
    unsafe {
        state
            .cast::<*mut Warm>()
            .as_ref()
            .and_then(|boxed| boxed.as_mut())
    }
    .ok_or_else(|| defect("the warm aggregate ran with no state"))
}

fn warm_error(info: sys::duckdb_function_info, outcome: Result<(), String>) {
    if let Err(text) = outcome {
        // SAFETY: a valid function info.
        unsafe { sys::duckdb_aggregate_function_set_error(info, message(&text).as_ptr()) };
    }
}

unsafe extern "C" fn warm_size(_: sys::duckdb_function_info) -> u64 {
    size_of::<*mut Warm>() as u64
}

unsafe extern "C" fn warm_init(
    info: sys::duckdb_function_info,
    state: sys::duckdb_aggregate_state,
) {
    warm_error(
        info,
        guarded("warm init", || {
            // SAFETY: DuckDB allocated `warm_size` bytes for this block.
            unsafe { *state.cast::<*mut Warm>() = Box::into_raw(Box::default()) };
            Ok(())
        }),
    );
}

unsafe extern "C" fn warm_destroy(states: *mut sys::duckdb_aggregate_state, count: u64) {
    let _ = guarded("warm destruction", || {
        for index in 0..count as usize {
            // SAFETY: each block holds a box `warm_init` leaked, freed once.
            unsafe {
                let boxed = *(*states.add(index)).cast::<*mut Warm>();
                if !boxed.is_null() {
                    drop(Box::from_raw(boxed));
                }
            }
        }
        Ok(())
    });
}

unsafe extern "C" fn warm_update(
    info: sys::duckdb_function_info,
    input: sys::duckdb_data_chunk,
    states: *mut sys::duckdb_aggregate_state,
) {
    warm_error(
        info,
        guarded("warm update", || {
            // SAFETY: two VARCHAR columns and one state per row.
            let rows = unsafe { sys::duckdb_data_chunk_get_size(input) } as usize;
            let read = |index| {
                read_column(
                    unsafe { sys::duckdb_data_chunk_get_vector(input, index) },
                    &Type::Text,
                    rows,
                )
            };
            let (questions, texts) = (read(0), read(1));
            for row in 0..rows {
                if let (Some(question), Some(text)) = (questions.text(row), texts.text(row)) {
                    // SAFETY: as above.
                    warm_at(unsafe { *states.add(row) })?.add(question, text)?;
                }
            }
            Ok(())
        }),
    );
}

unsafe extern "C" fn warm_combine(
    info: sys::duckdb_function_info,
    source: *mut sys::duckdb_aggregate_state,
    target: *mut sys::duckdb_aggregate_state,
    count: u64,
) {
    warm_error(
        info,
        guarded("warm combine", || {
            for index in 0..count as usize {
                // SAFETY: both arrays hold `count` states.
                let (from, to) = unsafe { (*source.add(index), *target.add(index)) };
                warm_at(to)?.merge(warm_at(from)?)?;
            }
            Ok(())
        }),
    );
}

unsafe extern "C" fn warm_finalize(
    info: sys::duckdb_function_info,
    source: *mut sys::duckdb_aggregate_state,
    result: sys::duckdb_vector,
    count: u64,
    offset: u64,
) {
    warm_error(
        info,
        guarded("warm finalize", || {
            for index in 0..count as usize {
                // SAFETY: the array holds `count` states.
                let asked = warm_at(unsafe { *source.add(index) })?.finish()?;
                write(
                    result,
                    &Type::BigInt,
                    offset as usize + index,
                    &Value::Int(asked),
                );
            }
            Ok(())
        }),
    );
}

fn register_warm(connection: sys::duckdb_connection) -> Result<(), String> {
    // SAFETY: as in `register_scalar`.
    unsafe {
        let mut aggregate = sys::duckdb_create_aggregate_function();
        sys::duckdb_aggregate_function_set_name(aggregate, c"thinkthen_warm".as_ptr());
        for of in [&Type::Text, &Type::Text] {
            let made = Logical::new(of);
            sys::duckdb_aggregate_function_add_parameter(aggregate, made.0);
        }
        let result = Logical::new(&Type::BigInt);
        sys::duckdb_aggregate_function_set_return_type(aggregate, result.0);
        sys::duckdb_aggregate_function_set_functions(
            aggregate,
            Some(warm_size),
            Some(warm_init),
            Some(warm_update),
            Some(warm_combine),
            Some(warm_finalize),
        );
        sys::duckdb_aggregate_function_set_destructor(aggregate, Some(warm_destroy));
        let state = sys::duckdb_register_aggregate_function(connection, aggregate);
        sys::duckdb_destroy_aggregate_function(&raw mut aggregate);
        if state == sys::duckdb_state_DuckDBSuccess {
            Ok(())
        } else {
            Err("thinkthen_warm did not register".to_owned())
        }
    }
}

// ---- LOAD ----

/// The extension's entry point. LOAD registers the settings, the scalars,
/// the usage table, and the warm aggregate, then takes SIGINT. It sends
/// nothing.
///
/// # Safety
///
/// DuckDB calls this once per LOAD with its own extension info and access.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_init_c_api(
    info: sys::duckdb_extension_info,
    access: *const sys::duckdb_extension_access,
) -> bool {
    let loaded = guarded("load", || {
        // SAFETY: DuckDB hands a valid info and access table.
        unsafe {
            if !sys::duckdb_rs_extension_api_init(info, access, "v1.5.5").map_err(defect)? {
                return Ok(());
            }
            let database =
                (*access)
                    .get_database
                    .ok_or_else(|| defect("DuckDB offered no database"))?(info);
            let mut connection: sys::duckdb_connection = std::ptr::null_mut();
            if database.is_null()
                || sys::duckdb_connect(*database, &raw mut connection)
                    != sys::duckdb_state_DuckDBSuccess
            {
                return Err(defect("the extension could not connect to its database"));
            }
            let registered = register_all(connection);
            sys::duckdb_disconnect(&raw mut connection);
            registered?;
        }
        signal::install();
        Ok(())
    });
    match loaded {
        Ok(()) => true,
        Err(text) => {
            // SAFETY: as above.
            unsafe {
                if let Some(set_error) = access.as_ref().and_then(|table| table.set_error) {
                    set_error(info, message(&text).as_ptr());
                }
            }
            false
        }
    }
}

fn register_all(connection: sys::duckdb_connection) -> Result<(), String> {
    register_setting(connection, c"thinkthen_throttle", &Type::BigInt)?;
    register_setting(connection, c"thinkthen_max_requests", &Type::BigInt)?;
    register_setting(connection, c"thinkthen_cache", &Type::Text)?;
    register_setting(connection, c"thinkthen_cache_bytes", &Type::BigInt)?;
    register_setting(connection, c"thinkthen_max_requests_total", &Type::BigInt)?;
    for scalar in &SCALARS {
        register_scalar(connection, scalar)?;
    }
    register_usage(connection)?;
    register_warm(connection)
}
