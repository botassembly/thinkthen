//! Every `unsafe` line of the kept connections: open, close, query, and
//! interrupt one, and read a caller's connection id and catalogs.
#![allow(unsafe_code, reason = "a kept connection is a DuckDB C API handle")]

use std::ffi::{CStr, CString};

use libduckdb_sys as sys;

use crate::ffi::message;
use crate::questions::Files;

/// One connection this extension opened on a loaded database.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Conn(sys::duckdb_connection);

// SAFETY: DuckDB connections may move between threads. Every query on a kept
// connection runs under that database's gate, and `duckdb_interrupt` is
// DuckDB's own thread-safe stop.
unsafe impl Send for Conn {}
// SAFETY: as above.
unsafe impl Sync for Conn {}

/// A query's rows, every column read as text; `None` is NULL.
#[derive(Debug, Default)]
pub(crate) struct Rows {
    pub(crate) columns: usize,
    pub(crate) rows: Vec<Vec<Option<String>>>,
}

impl Conn {
    /// A new connection on the loading database.
    pub(crate) fn open(database: sys::duckdb_database) -> Result<Self, String> {
        let mut raw: sys::duckdb_connection = std::ptr::null_mut();
        // SAFETY: DuckDB hands LOAD a live database; the out pointer is valid.
        let state = unsafe { sys::duckdb_connect(database, &raw mut raw) };
        if state == sys::duckdb_state_DuckDBSuccess && !raw.is_null() {
            Ok(Self(raw))
        } else {
            Err("the database refused a kept connection".to_owned())
        }
    }

    /// Close the connection. The registry calls this once, with no guard left.
    pub(crate) fn close(self) {
        let mut raw = self.0;
        // SAFETY: opened in `open` and closed once.
        unsafe { sys::duckdb_disconnect(&raw mut raw) };
    }

    /// Stop the query running on this connection, if any.
    pub(crate) fn interrupt(self) {
        // SAFETY: a live connection; DuckDB's own thread-safe stop.
        unsafe { sys::duckdb_interrupt(self.0) };
    }

    /// DuckDB's id for this connection.
    pub(crate) fn id(self) -> u64 {
        let mut context: sys::duckdb_client_context = std::ptr::null_mut();
        // SAFETY: a live connection; the context is destroyed once.
        unsafe {
            sys::duckdb_connection_get_client_context(self.0, &raw mut context);
            if context.is_null() {
                return 0;
            }
            let id = sys::duckdb_client_context_get_connection_id(context);
            sys::duckdb_destroy_client_context(&raw mut context);
            id
        }
    }

    /// Run one statement and drop its rows; `Err` holds DuckDB's words.
    pub(crate) fn execute(self, sql: &str) -> Result<(), String> {
        self.query(sql, |_| Ok(Rows::default())).map(drop)
    }

    /// Run one query and read every column as text. A column of another
    /// type refuses, so the caller's SQL casts.
    pub(crate) fn rows(self, sql: &str) -> Result<Rows, String> {
        self.query(sql, |result| {
            // SAFETY: a successful result; each chunk is destroyed once.
            unsafe {
                let columns = sys::duckdb_column_count(result) as usize;
                for column in 0..columns {
                    if sys::duckdb_column_type(result, column as u64)
                        != sys::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR
                    {
                        return Err(format!("column {} did not come back as text", column + 1));
                    }
                }
                let mut rows = Vec::new();
                loop {
                    let mut chunk = sys::duckdb_fetch_chunk(*result);
                    if chunk.is_null() {
                        break;
                    }
                    let size = sys::duckdb_data_chunk_get_size(chunk) as usize;
                    let vectors: Vec<_> = (0..columns)
                        .map(|column| sys::duckdb_data_chunk_get_vector(chunk, column as u64))
                        .collect();
                    for row in 0..size {
                        rows.push(vectors.iter().map(|vector| text(*vector, row)).collect());
                    }
                    sys::duckdb_destroy_data_chunk(&raw mut chunk);
                }
                Ok(Rows { columns, rows })
            }
        })
    }

    fn query(
        self,
        sql: &str,
        read: impl FnOnce(&mut sys::duckdb_result) -> Result<Rows, String>,
    ) -> Result<Rows, String> {
        let sql = message(sql);
        // SAFETY: a zeroed result is what `duckdb_query` expects, and it is
        // destroyed once whatever the outcome.
        unsafe {
            let mut result: sys::duckdb_result = std::mem::zeroed();
            let state = sys::duckdb_query(self.0, sql.as_ptr(), &raw mut result);
            let read = if state == sys::duckdb_state_DuckDBSuccess {
                read(&mut result)
            } else {
                Err(words(
                    sys::duckdb_result_error(&raw mut result),
                    "the query failed",
                ))
            };
            sys::duckdb_destroy_result(&raw mut result);
            read
        }
    }

    /// How many statements `sql` holds; `Err` holds the parser's words.
    pub(crate) fn statements(self, sql: &str) -> Result<u64, String> {
        let sql = message(sql);
        // SAFETY: the extracted statements are destroyed once.
        unsafe {
            let mut extracted: sys::duckdb_extracted_statements = std::ptr::null_mut();
            let count = sys::duckdb_extract_statements(self.0, sql.as_ptr(), &raw mut extracted);
            let outcome = if count == 0 {
                let said = if extracted.is_null() {
                    std::ptr::null()
                } else {
                    sys::duckdb_extract_statements_error(extracted)
                };
                Err(words(said, "the parser gave no reason"))
            } else {
                Ok(count)
            };
            sys::duckdb_destroy_extracted(&raw mut extracted);
            outcome
        }
    }

    /// Whether `sql` prepares as a `SELECT`; `Err` holds the prepare's words.
    pub(crate) fn is_select(self, sql: &str) -> Result<bool, String> {
        let sql = message(sql);
        // SAFETY: the prepared statement is destroyed once.
        unsafe {
            let mut prepared: sys::duckdb_prepared_statement = std::ptr::null_mut();
            let state = sys::duckdb_prepare(self.0, sql.as_ptr(), &raw mut prepared);
            let outcome = if state == sys::duckdb_state_DuckDBSuccess && !prepared.is_null() {
                Ok(sys::duckdb_prepared_statement_type(prepared)
                    == sys::duckdb_statement_type_DUCKDB_STATEMENT_TYPE_SELECT)
            } else {
                Err(words(
                    sys::duckdb_prepare_error(prepared),
                    "the prepare failed",
                ))
            };
            sys::duckdb_destroy_prepare(&raw mut prepared);
            outcome
        }
    }
}

/// DuckDB's words, or `fallback` when it gave none.
fn words(said: *const std::ffi::c_char, fallback: &str) -> String {
    if said.is_null() {
        return fallback.to_owned();
    }
    // SAFETY: DuckDB returns a NUL-terminated string it owns.
    unsafe { CStr::from_ptr(said) }
        .to_string_lossy()
        .into_owned()
}

fn text(vector: sys::duckdb_vector, row: usize) -> Option<String> {
    // SAFETY: a VARCHAR vector with at least `row + 1` rows.
    unsafe {
        let mask = sys::duckdb_vector_get_validity(vector);
        if !mask.is_null() && !sys::duckdb_validity_row_is_valid(mask, row as u64) {
            return None;
        }
        let data = sys::duckdb_vector_get_data(vector)
            .cast::<sys::duckdb_string_t>()
            .add(row);
        let length = sys::duckdb_string_t_length(*data) as usize;
        let start = sys::duckdb_string_t_data(data).cast::<u8>();
        Some(String::from_utf8_lossy(std::slice::from_raw_parts(start, length)).into_owned())
    }
}

impl Files {
    /// DuckDB's id for the caller's connection.
    pub(crate) fn connection_id(&self) -> u64 {
        // SAFETY: a live client context.
        unsafe { sys::duckdb_client_context_get_connection_id(self.context()) }
    }

    /// Whether the caller's own catalog `catalog` holds `main.table`. A name
    /// resolves only inside the caller's own instance.
    pub(crate) fn has_table(&self, catalog: &str, table: &str) -> bool {
        let (Ok(catalog), Ok(table)) = (CString::new(catalog), CString::new(table)) else {
            return false;
        };
        // SAFETY: a live context inside the caller's transaction; each
        // handle is destroyed once.
        unsafe {
            let mut found =
                sys::duckdb_client_context_get_catalog(self.context(), catalog.as_ptr());
            if found.is_null() {
                return false;
            }
            let mut entry = sys::duckdb_catalog_get_entry(
                found,
                self.context(),
                sys::duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_TABLE,
                c"main".as_ptr(),
                table.as_ptr(),
            );
            let held = !entry.is_null();
            if held {
                sys::duckdb_destroy_catalog_entry(&raw mut entry);
            }
            sys::duckdb_destroy_catalog(&raw mut found);
            held
        }
    }
}

/// Run `stop` when the process exits.
pub(crate) fn at_exit(stop: extern "C" fn()) {
    // SAFETY: `stop` only stores to an atomic.
    unsafe { libc::atexit(stop) };
}
