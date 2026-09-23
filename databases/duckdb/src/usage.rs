//! `thinkthen_usage()` as the one table function, through the raw C API
//! because `duckdb-rs` wires no table function. Three rows: the wire
//! sends since the process began (retried sends included), the answers a
//! cache gave with no send at all, and the tokens the replies reported.

use std::ffi::{CStr, CString, c_void};

use duckdb::ffi;


use crate::{guard, with_engine};

/// Drop a boxed emitted flag.
unsafe extern "C" fn drop_flag(payload: *mut c_void) {
    guard::contained_quiet("usage state destruction", || unsafe {
        if !payload.is_null() {
            drop(Box::from_raw(payload as *mut bool));
        }
    });
}

/// Bind: declare the two result columns, then hand back a bind payload
/// DuckDB can destroy. The columns live in bind because the table
/// function itself carries no result-column call on this API. The
/// callback is contained: a panic becomes the bind error.
unsafe extern "C" fn bind(info: ffi::duckdb_bind_info) {
    let outcome = guard::contained("usage bind", || unsafe { plan(info) }).err();
    if let Some(message) = outcome
        && let Ok(text) = CString::new(message)
    {
        unsafe { ffi::duckdb_bind_set_error(info, text.as_ptr()) };
    }
}

unsafe fn plan(info: ffi::duckdb_bind_info) -> Result<(), String> {
    unsafe {
        let varchar = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        ffi::duckdb_bind_add_result_column(info, c"metric".as_ptr(), varchar);
        let bigint = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_BIGINT);
        ffi::duckdb_bind_add_result_column(info, c"value".as_ptr(), bigint);
        ffi::duckdb_bind_set_bind_data(
            info,
            Box::into_raw(Box::new(false)) as *mut c_void,
            Some(drop_flag),
        );
        Ok(())
    }
}

/// Init: one emitted flag per scan, contained like its siblings.
unsafe extern "C" fn init(info: ffi::duckdb_init_info) {
    let outcome = guard::contained("usage init", || unsafe { init_scan(info) }).err();
    if let Some(message) = outcome
        && let Ok(text) = CString::new(message)
    {
        unsafe { ffi::duckdb_init_set_error(info, text.as_ptr()) };
    }
}

unsafe fn init_scan(info: ffi::duckdb_init_info) -> Result<(), String> {
    unsafe {
        ffi::duckdb_init_set_init_data(
            info,
            Box::into_raw(Box::new(false)) as *mut c_void,
            Some(drop_flag),
        );
        Ok(())
    }
}

/// Emit the three rows once per scan, then empty chunks. The callback is
/// contained: a panic becomes the scan's error.
unsafe extern "C" fn function(info: ffi::duckdb_function_info, output: ffi::duckdb_data_chunk) {
    if let Err(message) = guard::contained("usage scan", || unsafe { scan(info, output) }) {
        if let Ok(text) = CString::new(message) {
            unsafe { ffi::duckdb_function_set_error(info, text.as_ptr()) };
        }
        unsafe { ffi::duckdb_data_chunk_set_size(output, 0) };
    }
}

unsafe fn scan(info: ffi::duckdb_function_info, output: ffi::duckdb_data_chunk) -> Result<(), String> {
    unsafe {
        let emitted = ffi::duckdb_function_get_init_data(info) as *mut bool;
        if emitted.is_null() {
            ffi::duckdb_data_chunk_set_size(output, 0);
            return Ok(());
        }
        if *emitted {
            ffi::duckdb_data_chunk_set_size(output, 0);
            return Ok(());
        }
        *emitted = true;
        let usage = with_engine(|engine| engine.usage())?;
        let rows = [
            ("requests", usage.requests as i64),
            ("cache_answers", usage.cache_answers as i64),
            ("tokens", usage.tokens as i64),
        ];
        let metric = ffi::duckdb_data_chunk_get_vector(output, 0);
        let value =
            ffi::duckdb_vector_get_data(ffi::duckdb_data_chunk_get_vector(output, 1)) as *mut i64;
        for (i, (name, count)) in rows.iter().enumerate() {
            if let Ok(text) = CString::new(*name) {
                ffi::duckdb_vector_assign_string_element_len(
                    metric,
                    i as u64,
                    text.as_ptr(),
                    CStr::from_ptr(text.as_ptr()).to_bytes().len() as u64,
                );
            }
            *value.add(i) = *count;
        }
        ffi::duckdb_data_chunk_set_size(output, rows.len() as u64);
        Ok(())
    }
}

/// Register the table function on a raw connection.
pub unsafe fn register(con: ffi::duckdb_connection) -> Result<(), String> {
    unsafe {
        let table = ffi::duckdb_create_table_function();
        let name = CString::new("thinkthen_usage").expect("the name holds no NUL byte");
        ffi::duckdb_table_function_set_name(table, name.as_ptr());
        ffi::duckdb_table_function_set_bind(table, Some(bind));
        ffi::duckdb_table_function_set_init(table, Some(init));
        ffi::duckdb_table_function_set_function(table, Some(function));
        if ffi::duckdb_register_table_function(con, table) != ffi::DuckDBSuccess {
            let mut table = table;
            ffi::duckdb_destroy_table_function(&mut table);
            return Err("thinkthen_usage did not register".into());
        }
        let mut table = table;
        ffi::duckdb_destroy_table_function(&mut table);
        Ok(())
    }
}
