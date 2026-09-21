//! `thinkthen_relate(query, rules)`: the edges as rows, because `relate`
//! needs every record at once.
//!
//! The first argument is a query whose first column is each record's id
//! and whose second is its text: `SELECT * FROM
//! thinkthen_relate('SELECT id, body FROM alerts', ['caused_by'])`. The
//! deck drew a raw subquery in that position
//! (`thinkthen_relate((SELECT id, body FROM alerts), ...)`); the stable C
//! API cannot register a table function that takes a subquery — only a
//! table-in-out function accepts a subquery parameter, and the C API
//! cannot register those — so the query crosses as a string, the shape
//! PostgreSQL's row in the design page already takes. The finding is
//! pinned in NOTES.md.
//!
//! The second argument is a `LIST` of rule names, or a single `'@file'`
//! naming a question file's `relate` section. More than 255 records is a
//! usage error before anything is asked; the contract's own
//! `relate_checked` carries that guard, so every surface inherits it.

use std::ffi::{CStr, CString, c_void};
use std::sync::Mutex;

use duckdb::ffi;
use thinkthen_contract::{Error as EngineError, Kind, Relate, relate_checked};

use crate::{engine, failure, options};

/// The live connection `thinkthen_relate` runs its query on, opened at
/// load time and kept for the process lifetime. The database pointer
/// `get_database` hands out belongs to the load state and can dangle
/// after init, so the scan must not connect from it; a kept connection
/// is the C API's own handle and stays valid while the database lives.
/// The mutex serializes scans that share it.
static CONNECTION: Mutex<Option<ConnectionHandle>> = Mutex::new(None);

/// A raw `duckdb_connection` a static can hold; every use sits under the
/// mutex, which is the synchronization the handle needs.
struct ConnectionHandle(ffi::duckdb_connection);

unsafe impl Send for ConnectionHandle {}

/// Remember the connection for `thinkthen_relate`; the extension keeps it
/// open instead of disconnecting after registration.
pub(crate) fn remember_connection(connection: ffi::duckdb_connection) {
    *CONNECTION.lock().expect("the relate connection") = Some(ConnectionHandle(connection));
}

/// Bind data: the query and the rules, read once per query plan.
struct Bind {
    query: String,
    rules: Vec<String>,
}

/// Scan data: the rows built once per scan, and the cursor over them.
struct Scan {
    rows: Vec<(String, String, String, f64)>,
    at: usize,
    built: bool,
}

unsafe extern "C" fn drop_bind(payload: *mut c_void) {
    if !payload.is_null() {
        drop(unsafe { Box::from_raw(payload as *mut Bind) });
    }
}

unsafe extern "C" fn drop_scan(payload: *mut c_void) {
    if !payload.is_null() {
        drop(unsafe { Box::from_raw(payload as *mut Scan) });
    }
}

/// A string out of a `duckdb_value`, copied and the C string freed.
unsafe fn value_string(value: ffi::duckdb_value) -> Option<String> {
    if value.is_null() {
        return None;
    }
    let raw = unsafe { ffi::duckdb_get_varchar(value) };
    if raw.is_null() {
        return None;
    }
    let text = unsafe { CStr::from_ptr(raw) }.to_string_lossy().into_owned();
    unsafe { ffi::duckdb_free(raw as *mut c_void) };
    Some(text)
}

/// A `LIST(VARCHAR)` parameter as owned strings; NULL anywhere makes the
/// whole list unreadable, and the caller names that.
unsafe fn value_list(value: ffi::duckdb_value) -> Option<Vec<String>> {
    if value.is_null() {
        return None;
    }
    let count = unsafe { ffi::duckdb_get_list_size(value) };
    let mut items = Vec::with_capacity(count as usize);
    for i in 0..count {
        let mut child = unsafe { ffi::duckdb_get_list_child(value, i) };
        let text = unsafe { value_string(child) };
        unsafe { ffi::duckdb_destroy_value(&mut child) };
        items.push(text?);
    }
    Some(items)
}

/// A bind error the user sees, then the bind data on the way through.
unsafe fn bind_error(info: ffi::duckdb_bind_info, message: &str) {
    if let Ok(text) = CString::new(message) {
        unsafe { ffi::duckdb_bind_set_error(info, text.as_ptr()) };
    }
}

unsafe extern "C" fn bind(info: ffi::duckdb_bind_info) {
    unsafe {
        for (name, kind) in [
            (c"name", ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR),
            (c"source", ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR),
            (c"target", ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR),
            (c"probability", ffi::DUCKDB_TYPE_DUCKDB_TYPE_DOUBLE),
        ] {
            let mut logical = ffi::duckdb_create_logical_type(kind);
            ffi::duckdb_bind_add_result_column(info, name.as_ptr(), logical);
            ffi::duckdb_destroy_logical_type(&mut logical);
        }
        let mut query_value = ffi::duckdb_bind_get_parameter(info, 0);
        let mut rules_value = ffi::duckdb_bind_get_parameter(info, 1);
        let query = value_string(query_value);
        let rules = value_list(rules_value);
        ffi::duckdb_destroy_value(&mut query_value);
        ffi::duckdb_destroy_value(&mut rules_value);
        let Some(query) = query else {
            bind_error(info, "thinkthen usage: the relate query is NULL");
            return;
        };
        if query.trim().is_empty() {
            bind_error(info, "thinkthen usage: the relate query is blank");
            return;
        }
        let Some(rules) = rules else {
            bind_error(
                info,
                "thinkthen usage: the relate rules are NULL or hold a NULL entry",
            );
            return;
        };
        if rules.is_empty() {
            bind_error(
                info,
                "thinkthen usage: relate needs at least one rule, as a name or a question file",
            );
            return;
        }
        ffi::duckdb_bind_set_bind_data(
            info,
            Box::into_raw(Box::new(Bind { query, rules })) as *mut c_void,
            Some(drop_bind),
        );
    }
}

unsafe extern "C" fn init(info: ffi::duckdb_init_info) {
    unsafe {
        ffi::duckdb_init_set_init_data(
            info,
            Box::into_raw(Box::new(Scan {
                rows: Vec::new(),
                at: 0,
                built: false,
            })) as *mut c_void,
            Some(drop_scan),
        );
    }
}

/// Emit the edges, up to one vector per call; the first call builds them.
unsafe extern "C" fn function(info: ffi::duckdb_function_info, output: ffi::duckdb_data_chunk) {
    unsafe {
        let scan = ffi::duckdb_function_get_init_data(info) as *mut Scan;
        if scan.is_null() {
            ffi::duckdb_data_chunk_set_size(output, 0);
            return;
        }
        let scan = &mut *scan;
        if !scan.built {
            scan.built = true;
            let bind = ffi::duckdb_function_get_bind_data(info) as *const Bind;
            if bind.is_null() {
                if let Ok(text) = CString::new("thinkthen defect: the relate plan lost its bind data")
                {
                    ffi::duckdb_function_set_error(info, text.as_ptr());
                }
                ffi::duckdb_data_chunk_set_size(output, 0);
                return;
            }
            match build_rows(&*bind) {
                Ok(rows) => scan.rows = rows,
                Err(message) => {
                    let text = CString::new(message)
                        .unwrap_or_else(|_| c"thinkthen: the relate call failed".to_owned());
                    ffi::duckdb_function_set_error(info, text.as_ptr());
                    ffi::duckdb_data_chunk_set_size(output, 0);
                    return;
                }
            }
        }
        let remaining = scan.rows.len().saturating_sub(scan.at);
        let count = remaining.min(2048);
        for i in 0..count {
            let (name, source, target, probability) = &scan.rows[scan.at + i];
            assign_string(output, 0, i, name);
            assign_string(output, 1, i, source);
            assign_string(output, 2, i, target);
            let vector = ffi::duckdb_data_chunk_get_vector(output, 3);
            let data = ffi::duckdb_vector_get_data(vector) as *mut f64;
            *data.add(i) = *probability;
        }
        scan.at += count;
        ffi::duckdb_data_chunk_set_size(output, count as u64);
    }
}

/// One string cell of the output chunk.
unsafe fn assign_string(output: ffi::duckdb_data_chunk, column: u64, row: usize, value: &str) {
    let vector = unsafe { ffi::duckdb_data_chunk_get_vector(output, column) };
    unsafe {
        ffi::duckdb_vector_assign_string_element_len(
            vector,
            row as u64,
            value.as_ptr() as *const std::os::raw::c_char,
            value.len() as u64,
        );
    }
}

/// The edges, built once per scan: run the query, ask the engine, map the
/// edge ordinals back to the query's own ids.
fn build_rows(bind: &Bind) -> Result<Vec<(String, String, String, f64)>, String> {
    let records = {
        let guard = CONNECTION.lock().map_err(|_| {
            "thinkthen defect: the relate connection's lock is poisoned".to_owned()
        })?;
        let Some(handle) = guard.as_ref() else {
            return Err("thinkthen defect: the extension did not keep its relate connection".into());
        };
        unsafe { run_query(handle.0, &bind.query)? }
    };
    if records.is_empty() {
        // No records, no pairs, nothing to ask: zero edges is arithmetic,
        // not a judgment, so the recordings are not consulted.
        return Ok(Vec::new());
    }
    let ask = build_ask(&bind.rules)?;
    let bodies: Vec<&str> = records.iter().map(|(_, body)| body.as_str()).collect();
    let edges = relate_checked(engine(), &ask, &bodies, options()).map_err(failure)?;
    let mut rows = Vec::with_capacity(edges.len());
    for edge in edges {
        let id = |record: u64| -> Result<String, String> {
            records
                .get((record as usize).saturating_sub(1))
                .map(|(id, _)| id.clone())
                .ok_or_else(|| {
                    failure(EngineError::defect(format!(
                        "the engine returned an edge over record {record}, past the {} records the query returned",
                        records.len()
                    )))
                })
        };
        rows.push((edge.name, id(edge.source)?, id(edge.target)?, edge.probability));
    }
    Ok(rows)
}

/// The rules parameter as a `Relate` ask: bare names mean any-to-any, and
/// a single `'@file'` or `'{...}'` entry is the question file's `relate`
/// section or the spec itself.
fn build_ask(rules: &[String]) -> Result<Relate, String> {
    if rules.len() == 1 {
        let only = rules[0].trim();
        if let Some(path) = only.strip_prefix('@') {
            let text = std::fs::read_to_string(path).map_err(|error| {
                format!("thinkthen local: the question file {path} did not read: {error}")
            })?;
            let value: serde_json::Value = serde_json::from_str(&text)
                .map_err(|error| format!("thinkthen usage: the question file {path} is not JSON: {error}"))?;
            let section = value.get("relate").cloned().unwrap_or(value);
            return Relate::from_json(&section.to_string()).map_err(failure);
        }
        if only.starts_with('{') {
            return Relate::from_json(only).map_err(failure);
        }
    }
    let mut ask = Relate::new();
    for name in rules {
        ask = ask.relation(name, Kind::Any, Kind::Any).map_err(failure)?;
    }
    Ok(ask)
}

/// Run the query on the kept connection and read `(id, text)` per row;
/// any value renders as its text, so an integer id comes back as `1`.
unsafe fn run_query(
    connection: ffi::duckdb_connection,
    sql: &str,
) -> Result<Vec<(String, String)>, String> {
    let sql_c = match CString::new(sql) {
        Ok(sql_c) => sql_c,
        Err(_) => {
            return Err("thinkthen usage: the relate query holds a NUL byte".into());
        }
    };
    let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
    let state = unsafe { ffi::duckdb_query(connection, sql_c.as_ptr(), &mut result) };
    if state != ffi::DuckDBSuccess {
        let message = unsafe { ffi::duckdb_result_error(&mut result) };
        let text = if message.is_null() {
            "the query failed".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(format!("thinkthen usage: the relate query failed: {text}"));
    }
    let columns = unsafe { ffi::duckdb_column_count(&mut result) };
    if columns < 2 {
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(
            "thinkthen usage: the relate query must return the id and the text as its first two columns"
                .into(),
        );
    }
    let rows = unsafe { ffi::duckdb_row_count(&mut result) };
    let mut records = Vec::with_capacity(rows as usize);
    for row in 0..rows {
        let id_null = unsafe { ffi::duckdb_value_is_null(&mut result, 0, row) };
        let text_null = unsafe { ffi::duckdb_value_is_null(&mut result, 1, row) };
        if id_null || text_null {
            unsafe { ffi::duckdb_destroy_result(&mut result) };
            return Err(format!(
                "thinkthen usage: record {} carries a NULL id or text",
                row + 1
            ));
        }
        let id_ptr = unsafe { ffi::duckdb_value_varchar(&mut result, 0, row) };
        let text_ptr = unsafe { ffi::duckdb_value_varchar(&mut result, 1, row) };
        let id = unsafe { CStr::from_ptr(id_ptr) }.to_string_lossy().into_owned();
        let text = unsafe { CStr::from_ptr(text_ptr) }.to_string_lossy().into_owned();
        unsafe { ffi::duckdb_free(id_ptr as *mut c_void) };
        unsafe { ffi::duckdb_free(text_ptr as *mut c_void) };
        records.push((id, text));
    }
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok(records)
}

/// Register `thinkthen_relate` on a raw connection.
pub unsafe fn register(connection: ffi::duckdb_connection) -> Result<(), String> {
    unsafe {
        let table = ffi::duckdb_create_table_function();
        let name = CString::new("thinkthen_relate").expect("the name holds no NUL byte");
        ffi::duckdb_table_function_set_name(table, name.as_ptr());
        let varchar = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        ffi::duckdb_table_function_add_parameter(table, varchar);
        let list = ffi::duckdb_create_list_type(varchar);
        ffi::duckdb_table_function_add_parameter(table, list);
        let mut varchar = varchar;
        ffi::duckdb_destroy_logical_type(&mut varchar);
        let mut list = list;
        ffi::duckdb_destroy_logical_type(&mut list);
        ffi::duckdb_table_function_set_bind(table, Some(bind));
        ffi::duckdb_table_function_set_init(table, Some(init));
        ffi::duckdb_table_function_set_function(table, Some(function));
        if ffi::duckdb_register_table_function(connection, table) != ffi::DuckDBSuccess {
            let mut table = table;
            ffi::duckdb_destroy_table_function(&mut table);
            return Err("thinkthen_relate did not register".into());
        }
        let mut table = table;
        ffi::duckdb_destroy_table_function(&mut table);
        Ok(())
    }
}
