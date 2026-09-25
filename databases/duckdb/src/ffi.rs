//! The shared `unsafe` lines of the DuckDB C API: types, chunk reads and
//! writes, setting registration, and LOAD. Each module keeps its own
//! callbacks in its own `ffi.rs`, and each callback runs its body through
//! [`guarded`], so a panic becomes that callback's error and never unwinds
//! into DuckDB.
#![allow(unsafe_code, reason = "the DuckDB extension API is a C API")]

use std::ffi::{CString, c_char};

use libduckdb_sys as sys;

use crate::errors::{defect, guarded};
use crate::scalars::{SCALARS, register_scalar};
use crate::signal;
use crate::tables::{register_usage, register_warm};

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

/// A logical type, destroyed once when dropped. Every logical type this
/// extension creates is made here (R4-16).
pub(crate) struct Logical(pub(crate) sys::duckdb_logical_type);

impl Logical {
    pub(crate) fn new(of: &Type) -> Self {
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
pub(crate) fn read_column(vector: sys::duckdb_vector, of: &Type, rows: usize) -> Column {
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
pub(crate) fn write(vector: sys::duckdb_vector, of: &Type, row: usize, value: &Value) {
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

pub(crate) fn message(text: &str) -> CString {
    CString::new(text.replace('\0', " ")).unwrap_or_default()
}

pub(crate) fn register_setting(
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
