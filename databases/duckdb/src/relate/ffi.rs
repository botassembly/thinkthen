//! Every `unsafe` line of `thinkthen_relate`: its registration, its two
//! settings, and its bind, init, and scan callbacks. The `test-hooks` build
//! adds `thinkthen_test_hook_reap()`, the forced reaper pass for R3-1.
#![allow(
    unsafe_code,
    reason = "a table function is a DuckDB C API callback set"
)]

use std::ffi::{CStr, c_void};

use libduckdb_sys as sys;

use super::{Bound, COLUMNS, HOLDING, Row, Rules, SECONDS, bind, rows};
use crate::errors::{defect, guarded};
#[cfg(feature = "test-hooks")]
use crate::ffi::Value;
use crate::ffi::{Logical, Type, message, write};
use crate::questions::Files;

/// One scan's rows, built on its first call, and how many it has sent.
#[derive(Debug, Default)]
struct Scan {
    rows: Option<Vec<Row>>,
    at: usize,
}

/// A text parameter; NULL reads `None`.
fn text(value: sys::duckdb_value) -> Option<String> {
    // SAFETY: a live value; the string is freed once.
    unsafe {
        if value.is_null() || sys::duckdb_is_null_value(value) {
            return None;
        }
        let raw = sys::duckdb_get_varchar(value);
        if raw.is_null() {
            return None;
        }
        let owned = CStr::from_ptr(raw).to_string_lossy().into_owned();
        sys::duckdb_free(raw.cast());
        Some(owned)
    }
}

/// The rules parameter: text, or a `LIST` of texts. A NULL anywhere reads
/// `None`.
fn rules(value: sys::duckdb_value) -> Option<Rules> {
    // SAFETY: a live value; each child is destroyed once, and the value's
    // type belongs to the value.
    unsafe {
        if value.is_null() || sys::duckdb_is_null_value(value) {
            return None;
        }
        let kind = sys::duckdb_get_type_id(sys::duckdb_get_value_type(value));
        if kind != sys::DUCKDB_TYPE_DUCKDB_TYPE_LIST {
            return text(value).map(Rules::Text);
        }
        let mut listed = Vec::new();
        for index in 0..sys::duckdb_get_list_size(value) {
            let mut child = sys::duckdb_get_list_child(value, index);
            let read = text(child);
            sys::duckdb_destroy_value(&raw mut child);
            listed.push(read?);
        }
        Some(Rules::List(listed))
    }
}

fn parameter(info: sys::duckdb_bind_info, index: u64) -> sys::duckdb_value {
    // SAFETY: relate registers two parameters.
    unsafe { sys::duckdb_bind_get_parameter(info, index) }
}

unsafe extern "C" fn relate_bind(info: sys::duckdb_bind_info) {
    let bound = guarded("relate bind", || {
        for (name, of) in &COLUMNS {
            let made = Logical::new(of);
            // SAFETY: the bind copies the type.
            unsafe { sys::duckdb_bind_add_result_column(info, name.as_ptr(), made.raw()) };
        }
        let (mut query, mut listed) = (parameter(info, 0), parameter(info, 1));
        let read = (text(query), rules(listed));
        // SAFETY: each parameter value is destroyed once.
        unsafe {
            sys::duckdb_destroy_value(&raw mut query);
            sys::duckdb_destroy_value(&raw mut listed);
        }
        bind(Files::of_bind(info)?, read.0, read.1).map(Box::new)
    });
    // SAFETY: DuckDB owns the bind data and calls `drop_bound` once.
    unsafe {
        match bound {
            Ok(bound) => {
                sys::duckdb_bind_set_bind_data(info, Box::into_raw(bound).cast(), Some(drop_bound))
            }
            Err(said) => sys::duckdb_bind_set_error(info, message(&said).as_ptr()),
        }
    }
}

unsafe extern "C" fn drop_bound(data: *mut c_void) {
    let _ = guarded("relate bind destruction", || {
        if !data.is_null() {
            // SAFETY: the box `relate_bind` leaked.
            drop(unsafe { Box::from_raw(data.cast::<Bound>()) });
        }
        Ok(())
    });
}

unsafe extern "C" fn relate_init(info: sys::duckdb_init_info) {
    let made = guarded("relate init", || Ok(Box::<Scan>::default()));
    // SAFETY: DuckDB owns the scan state and calls `drop_scan` once.
    unsafe {
        match made {
            Ok(scan) => {
                sys::duckdb_init_set_init_data(info, Box::into_raw(scan).cast(), Some(drop_scan))
            }
            Err(said) => sys::duckdb_init_set_error(info, message(&said).as_ptr()),
        }
        sys::duckdb_init_set_max_threads(info, 1);
    }
}

unsafe extern "C" fn drop_scan(data: *mut c_void) {
    let _ = guarded("relate scan destruction", || {
        if !data.is_null() {
            // SAFETY: the box `relate_init` leaked.
            drop(unsafe { Box::from_raw(data.cast::<Scan>()) });
        }
        Ok(())
    });
}

unsafe extern "C" fn relate_scan(info: sys::duckdb_function_info, output: sys::duckdb_data_chunk) {
    let scanned = guarded("relate scan", || {
        // SAFETY: the bind data and the init data this file set.
        let (bound, scan) = unsafe {
            (
                sys::duckdb_function_get_bind_data(info)
                    .cast::<Bound>()
                    .as_ref(),
                sys::duckdb_function_get_init_data(info)
                    .cast::<Scan>()
                    .as_mut(),
            )
        };
        let bound = bound.ok_or_else(|| defect("the relate scan ran with no bind data"))?;
        let scan = scan.ok_or_else(|| defect("the relate scan ran with no init state"))?;
        if scan.rows.is_none() {
            scan.rows = Some(rows(bound)?);
        }
        let all = scan.rows.as_deref().unwrap_or_default();
        // SAFETY: a chunk holds `duckdb_vector_size` rows per column.
        let room = unsafe { sys::duckdb_vector_size() } as usize;
        let batch = all.get(scan.at..).unwrap_or_default();
        let batch = batch.get(..room.min(batch.len())).unwrap_or_default();
        for (row, values) in batch.iter().enumerate() {
            for (column, ((_, of), value)) in COLUMNS.iter().zip(values).enumerate() {
                // SAFETY: four output columns.
                write(
                    unsafe { sys::duckdb_data_chunk_get_vector(output, column as u64) },
                    of,
                    row,
                    value,
                );
            }
        }
        scan.at += batch.len();
        // SAFETY: a valid chunk.
        unsafe { sys::duckdb_data_chunk_set_size(output, batch.len() as u64) };
        Ok(())
    });
    if let Err(said) = scanned {
        // SAFETY: a valid function info and chunk.
        unsafe {
            sys::duckdb_function_set_error(info, message(&said).as_ptr());
            sys::duckdb_data_chunk_set_size(output, 0);
        }
    }
}

/// Register `thinkthen_relate(query VARCHAR, rules ANY)` and its settings.
pub(crate) fn register(connection: sys::duckdb_connection) -> Result<(), String> {
    table(
        connection,
        c"thinkthen_relate",
        &[Type::Text, Type::Any],
        relate_bind,
        relate_init,
        relate_scan,
    )?;
    crate::ffi::register_setting(connection, SECONDS, &Type::BigInt)?;
    crate::ffi::register_setting(connection, HOLDING, &Type::BigInt)?;
    #[cfg(feature = "test-hooks")]
    table(
        connection,
        c"thinkthen_test_hook_reap",
        &[],
        reap_bind,
        relate_init,
        reap_scan,
    )?;
    Ok(())
}

fn table(
    connection: sys::duckdb_connection,
    name: &CStr,
    parameters: &[Type],
    on_bind: unsafe extern "C" fn(sys::duckdb_bind_info),
    on_init: unsafe extern "C" fn(sys::duckdb_init_info),
    on_scan: unsafe extern "C" fn(sys::duckdb_function_info, sys::duckdb_data_chunk),
) -> Result<(), String> {
    // SAFETY: each created object is copied on register, then destroyed once.
    unsafe {
        let mut table = sys::duckdb_create_table_function();
        sys::duckdb_table_function_set_name(table, name.as_ptr());
        for of in parameters {
            let made = Logical::new(of);
            sys::duckdb_table_function_add_parameter(table, made.raw());
        }
        sys::duckdb_table_function_set_bind(table, Some(on_bind));
        sys::duckdb_table_function_set_init(table, Some(on_init));
        sys::duckdb_table_function_set_function(table, Some(on_scan));
        let state = sys::duckdb_register_table_function(connection, table);
        sys::duckdb_destroy_table_function(&raw mut table);
        if state == sys::duckdb_state_DuckDBSuccess {
            Ok(())
        } else {
            Err(format!("{name:?} did not register"))
        }
    }
}

#[cfg(feature = "test-hooks")]
unsafe extern "C" fn reap_bind(info: sys::duckdb_bind_info) {
    let made = Logical::new(&Type::BigInt);
    // SAFETY: the bind copies the type.
    unsafe { sys::duckdb_bind_add_result_column(info, c"released".as_ptr(), made.raw()) };
}

/// The forced reaper pass: one row, the count it released.
#[cfg(feature = "test-hooks")]
unsafe extern "C" fn reap_scan(info: sys::duckdb_function_info, output: sys::duckdb_data_chunk) {
    // SAFETY: the init data `relate_init` set.
    let scan = unsafe {
        sys::duckdb_function_get_init_data(info)
            .cast::<Scan>()
            .as_mut()
    };
    let first = scan.is_some_and(|scan| scan.rows.replace(Vec::new()).is_none());
    if first {
        let released = crate::connections::reap(true);
        // SAFETY: one BIGINT column.
        let column = unsafe { sys::duckdb_data_chunk_get_vector(output, 0) };
        write(
            column,
            &Type::BigInt,
            0,
            &Value::Int(i64::try_from(released).unwrap_or(i64::MAX)),
        );
    }
    // SAFETY: a valid chunk.
    unsafe { sys::duckdb_data_chunk_set_size(output, u64::from(first)) };
}
