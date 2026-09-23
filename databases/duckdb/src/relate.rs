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
//!
//! The scan runs its query on the kept connection of the CALLER'S
//! database, resolved at bind time through the caller's own identity
//! token (`connections.rs`), not on one process-global connection: two
//! databases in one process each load this extension and share its
//! statics, and the global used to serve whichever loaded last. The
//! stable C API cannot run a query on the caller's own connection, so a
//! temporary table on that connection stays invisible; when the query
//! misses one, `boundary_note` says so in words instead of leaving
//! "table does not exist" to mislead.
//!
//! The query reads records and nothing else, and the scan enforces that:
//! exactly one statement, run in a read-only transaction under the
//! caller's own search path, so no statement it holds can change data
//! and no work it does can commit beside the caller's transaction. A
//! relate inside a relate is refused with a usage error rather than
//! waiting forever on the connection the outer query already holds.
//! The rules file's read is checked against the caller's own
//! `enable_external_access`, `allowed_paths`, `allowed_directories`, and
//! `disabled_filesystems` at bind and again at execution, because a
//! prepared statement executes after its settings were read.

use std::ffi::{CStr, CString, c_void};

use duckdb::ffi;
use thinkthen_contract::{Error as EngineError, Kind, Relate, relate_checked};

use crate::{connections, engine_call, failure, guard, options};

/// Bind data: the query and the rules, read once per query plan, plus
/// the caller's own context (it carries the caller's database identity,
/// the temp catalog the boundary check probes, and the settings every
/// access check reads live) and a counted guard on the kept connection
/// of the caller's database. The guard is the use-after-free fix
/// (review 3, finding 1): the reaper cannot disconnect a connection a
/// prepared statement still holds, so a prepare → close-the-database →
/// execute sequence that used to segfault in
/// `duckdb_extract_statements` now finds its connection alive.
struct Bind {
    query: String,
    rules: Vec<String>,
    caller: ffi::duckdb_client_context,
    connection: connections::KeptGuard,
}

/// Scan data: the rows built once per scan, and the cursor over them.
struct Scan {
    rows: Vec<(String, String, String, f64)>,
    at: usize,
    built: bool,
}

unsafe extern "C" fn drop_bind(payload: *mut c_void) {
    guard::contained_quiet("relate bind-state destruction", || unsafe {
        if !payload.is_null() {
            let bind = Box::from_raw(payload as *mut Bind);
            let mut caller = bind.caller;
            if !caller.is_null() {
                ffi::duckdb_destroy_client_context(&mut caller);
            }
            drop(bind);
        }
    });
}

unsafe extern "C" fn drop_scan(payload: *mut c_void) {
    guard::contained_quiet("relate scan-state destruction", || unsafe {
        if !payload.is_null() {
            drop(Box::from_raw(payload as *mut Scan));
        }
    });
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

/// The bind callback, contained: a panic becomes the bind error.
unsafe extern "C" fn bind(info: ffi::duckdb_bind_info) {
    if let Err(message) = guard::contained("relate bind", || unsafe { plan(info) }) {
        unsafe { bind_error(info, &message) };
    }
}

/// Plan one relate: declare the columns, read the parameters, and
/// resolve the caller's database, its file-access setting, and the kept
/// connection the scan will use.
unsafe fn plan(info: ffi::duckdb_bind_info) -> Result<(), String> {
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
            return Err("thinkthen usage: the relate query is NULL".into());
        };
        if query.trim().is_empty() {
            return Err("thinkthen usage: the relate query is blank".into());
        }
        let Some(rules) = rules else {
            return Err(
                "thinkthen usage: the relate rules are NULL or hold a NULL entry".into(),
            );
        };
        if rules.is_empty() {
            return Err(
                "thinkthen usage: relate needs at least one rule, as a name or a question file"
                    .into(),
            );
        }
        // The caller's own context: its catalogs name the caller's
        // database among the loaded ones, and its temp catalog answers
        // the boundary check when the query misses a temporary table.
        let mut caller: ffi::duckdb_client_context = std::ptr::null_mut();
        ffi::duckdb_table_function_get_client_context(info, &mut caller);
        if caller.is_null() {
            return Err("thinkthen defect: the calling connection has no client context".into());
        }
        let connection = match connections::for_caller(caller) {
            Ok(connection) => connection,
            Err(message) => {
                let mut caller = caller;
                ffi::duckdb_destroy_client_context(&mut caller);
                return Err(message);
            }
        };        // A rules file the caller's database forbids fails the bind now,
        // and the same check runs again at execution.
        if let Some(refusal) = rules_file_refusal(&rules, caller) {
            let mut caller = caller;
            ffi::duckdb_destroy_client_context(&mut caller);
            return Err(refusal);
        }
        ffi::duckdb_bind_set_bind_data(
            info,
            Box::into_raw(Box::new(Bind {
                query,
                rules,
                caller,
                connection,
            })) as *mut c_void,
            Some(drop_bind),
        );
        Ok(())
    }
}

unsafe extern "C" fn init(info: ffi::duckdb_init_info) {
    if let Err(message) = guard::contained("relate init", || unsafe { init_scan(info) }) {
        if let Ok(text) = CString::new(message) {
            unsafe { ffi::duckdb_init_set_error(info, text.as_ptr()) };
        }
    }
}

/// One scan state per scan.
unsafe fn init_scan(info: ffi::duckdb_init_info) -> Result<(), String> {
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
        Ok(())
    }
}

/// Emit the edges, up to one vector per call; the first call builds them.
/// The callback is contained: a panic becomes the scan's error.
unsafe extern "C" fn function(info: ffi::duckdb_function_info, output: ffi::duckdb_data_chunk) {
    if let Err(message) = guard::contained("relate scan", || unsafe { scan(info, output) }) {
        if let Ok(text) = CString::new(message) {
            unsafe { ffi::duckdb_function_set_error(info, text.as_ptr()) };
        }
        unsafe { ffi::duckdb_data_chunk_set_size(output, 0) };
    }
}

unsafe fn scan(info: ffi::duckdb_function_info, output: ffi::duckdb_data_chunk) -> Result<(), String> {
    unsafe {
        let scan = ffi::duckdb_function_get_init_data(info) as *mut Scan;
        if scan.is_null() {
            ffi::duckdb_data_chunk_set_size(output, 0);
            return Ok(());
        }
        let scan = &mut *scan;
        if !scan.built {
            scan.built = true;
            let bind = ffi::duckdb_function_get_bind_data(info) as *const Bind;
            if bind.is_null() {
                return Err("thinkthen defect: the relate plan lost its bind data".into());
            }
            scan.rows = build_rows(&*bind)?;
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
        Ok(())
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

/// The edges, built once per scan: run the query on the caller's
/// database's kept connection, ask the engine, map the edge ordinals
/// back to the query's own ids. The whole build runs under the query
/// gate, so the reaper cannot close the connection mid-scan and a nested
/// relate is refused before it waits.
fn build_rows(bind: &Bind) -> Result<Vec<(String, String, String, f64)>, String> {
    // A callback carrying the kept connection's own id runs inside a query
    // that connection is executing: a relate inside a relate. It must
    // refuse here, before it waits on the gate its own outer query holds.
    if connections::caller_is_kept(bind.caller, bind.connection.connection()) {
        return Err(
            "thinkthen usage: the relate query calls thinkthen_relate while its own query is running; nested relate cannot run, because the outer query waits on the connection the inner one needs"
                .into(),
        );
    }
    // The caller's database's own gate, not a process-global one
    // (review 3, finding 12): one database's long relate no longer
    // blocks another database's.
    let _gate = bind
        .connection
        .gate()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let records = unsafe { run_query(bind) }.map_err(|message| boundary_note(bind, message))?;
    if records.is_empty() {
        // No records, no pairs, nothing to ask: zero edges is arithmetic,
        // not a judgment, so the recordings are not consulted.
        return Ok(Vec::new());
    }
    let ask = build_ask(&bind.rules, bind.caller)?;
    let bodies: Vec<&str> = records.iter().map(|(_, body)| body.as_str()).collect();
    let edges = engine_call(|engine| relate_checked(engine, &ask, &bodies, options()))?;
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

/// The refusal a rules argument naming a file earns when the calling
/// database forbids the read; bare rule names read nothing.
fn rules_file_refusal(rules: &[String], caller: ffi::duckdb_client_context) -> Option<String> {
    if rules.len() != 1 {
        return None;
    }
    let path = rules[0].trim().strip_prefix('@')?;
    connections::read_refusal(caller, path)
}

/// The rules parameter as a `Relate` ask: bare names mean any-to-any, and
/// a single `'@file'` or `'{...}'` entry is the question file's `relate`
/// section or the spec itself. A file read is checked against the calling
/// database's own settings here, at execution, however long ago the
/// statement was prepared.
fn build_ask(rules: &[String], caller: ffi::duckdb_client_context) -> Result<Relate, String> {
    if rules.len() == 1 {
        let only = rules[0].trim();
        if let Some(path) = only.strip_prefix('@') {
            if let Some(refusal) = connections::read_refusal(caller, path) {
                return Err(refusal);
            }
            let text = connections::read_question_file(path, Some(caller))?;
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

/// The raw run failure, translated when the missing table is the
/// caller's own temporary one. The stable C API cannot run a query on
/// the calling connection, so relate can never see that table; saying
/// which boundary was hit beats "table does not exist".
fn boundary_note(bind: &Bind, message: String) -> String {
    let Some(name) = missing_table(&message) else {
        return message;
    };
    if !temp_table_exists(bind.caller, &name) {
        return message;
    }
    format!(
        "thinkthen local: the relate query names the temporary table {name}, and the stable C API cannot run a query on the calling connection, so relate cannot see temporary tables; materialize it (CREATE TABLE ... AS SELECT) or run the query directly"
    )
}

/// The missing name inside DuckDB's catalog error, when it names one.
fn missing_table(message: &str) -> Option<String> {
    let rest = message.split("Table with name ").nth(1)?;
    let name = rest.split(" does not exist").next()?.trim().trim_matches('"');
    if name.is_empty() {
        None
    } else {
        Some(name.to_owned())
    }
}

/// Whether the caller's context holds a table with this name in its
/// temporary catalog.
fn temp_table_exists(caller: ffi::duckdb_client_context, name: &str) -> bool {
    let Ok(name) = CString::new(name) else {
        return false;
    };
    let mut catalog =
        unsafe { ffi::duckdb_client_context_get_catalog(caller, c"temp".as_ptr()) };
    if catalog.is_null() {
        return false;
    }
    let mut entry = unsafe {
        ffi::duckdb_catalog_get_entry(
            catalog,
            caller,
            ffi::duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_TABLE,
            c"main".as_ptr(),
            name.as_ptr(),
        )
    };
    let exists = !entry.is_null();
    if exists {
        unsafe { ffi::duckdb_destroy_catalog_entry(&mut entry) };
    }
    unsafe { ffi::duckdb_destroy_catalog(&mut catalog) };
    exists
}

/// Run the query on the caller's database's kept connection and read
/// `(id, text)` per row: exactly one statement, of the `SELECT` kind,
/// inside a read-only transaction that always rolls back, with the
/// record cap checked before any row is collected. The statement kind is
/// checked before anything runs (review 3, finding 7: `COPY ... TO`,
/// `EXPORT DATABASE`, `ATTACH`, `SET GLOBAL`, `LOAD`, and `SET VARIABLE`
/// all passed the single-statement check because each is one
/// statement), and the count runs first (review 3, finding 12: eight
/// million rows materialized 2.35 GB before the cap refused).
unsafe fn run_query(bind: &Bind) -> Result<Vec<(String, String)>, String> {
    let connection = bind.connection.connection();
    unsafe { ensure_single_statement(connection, &bind.query)? };
    unsafe { ensure_select_statement(connection, &bind.query)? };
    let search = connections::search_path_of(bind.caller);
    unsafe { begin_read_only(connection)? };
    let count_sql = format!("SELECT count(*) FROM ({}) AS thinkthen_count", bind.query);
    let outcome = unsafe {
        under_search_path(connection, search.as_deref(), || {
            let count = count_records(connection, &count_sql)?;
            if count > RECORD_CAP {
                return Err(format!(
                    "thinkthen usage: the relate query returned {count} records and relate asks about at most {RECORD_CAP}; add a LIMIT or a WHERE"
                ));
            }
            run_statement(connection, &bind.query)
        })
    };
    // The read-only transaction ends either way; rollback and an unset
    // search path leave the kept connection as the next query expects it.
    let _ = unsafe { execute(connection, c"ROLLBACK") };
    let _ = unsafe { execute(connection, c"RESET search_path") };
    outcome
}

/// The most records one relate may ask about; the contract's own
/// `relate_checked` carries the same guard.
const RECORD_CAP: u64 = 255;

/// The record count of the wrapped query, read before any row is
/// collected. The wrapper is the caller's own query, so the count is the
/// count its rows will have; a query whose rows change between the count
/// and the read still cannot materialize past the belt cap inside
/// `run_statement`.
unsafe fn count_records(connection: ffi::duckdb_connection, count_sql: &str) -> Result<u64, String> {
    let Ok(sql_c) = CString::new(count_sql) else {
        return Err("thinkthen usage: the relate query holds a NUL byte".into());
    };
    let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
    if unsafe { ffi::duckdb_query(connection, sql_c.as_ptr(), &mut result) } != ffi::DuckDBSuccess {
        let message = unsafe { ffi::duckdb_result_error(&mut result) };
        let text = if message.is_null() {
            "the count failed".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(format!("thinkthen usage: the relate query failed: {text}"));
    }
    let raw = unsafe { ffi::duckdb_value_int64(&mut result, 0, 0) };
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok(raw.max(0) as u64)
}

/// Prepare the query and require the `SELECT` statement kind, so relate
/// reads records and never writes files, attaches databases, changes
/// settings, loads extensions, or leaves variables behind for the next
/// caller.
unsafe fn ensure_select_statement(
    connection: ffi::duckdb_connection,
    sql: &str,
) -> Result<(), String> {
    let Ok(sql_c) = CString::new(sql) else {
        return Err("thinkthen usage: the relate query holds a NUL byte".into());
    };
    let mut prepared: ffi::duckdb_prepared_statement = std::ptr::null_mut();
    if unsafe { ffi::duckdb_prepare(connection, sql_c.as_ptr(), &mut prepared) } != ffi::DuckDBSuccess
        || prepared.is_null()
    {
        // The prepare's own words carry through — a missing table is
        // named by the engine, and the temporary-table boundary below
        // translates it rather than hiding it behind "did not prepare".
        let message = unsafe { ffi::duckdb_prepare_error(prepared) };
        let text = if message.is_null() {
            "the prepare refused".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        let mut prepared = prepared;
        unsafe { ffi::duckdb_destroy_prepare(&mut prepared) };
        return Err(format!("thinkthen usage: the relate query failed: {text}"));
    }
    let kind = unsafe { ffi::duckdb_prepared_statement_type(prepared) };
    let mut prepared = prepared;
    unsafe { ffi::duckdb_destroy_prepare(&mut prepared) };
    if kind != ffi::duckdb_statement_type_DUCKDB_STATEMENT_TYPE_SELECT {
        return Err(format!(
            "thinkthen usage: the relate query must be a SELECT; this one is a {} statement, and relate reads records, it does not write files, attach databases, change settings, or load extensions",
            statement_kind_name(kind)
        ));
    }
    Ok(())
}

/// The statement kind's own name, for the refusal a non-SELECT earns.
fn statement_kind_name(kind: ffi::duckdb_statement_type) -> &'static str {
    use ffi::*;
    match kind {
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_INSERT => "INSERT",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_UPDATE => "UPDATE",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_DELETE => "DELETE",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_EXPLAIN => "EXPLAIN",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_CREATE => "CREATE",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_ALTER => "ALTER",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_TRANSACTION => "TRANSACTION",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_COPY => "COPY",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_PREPARE => "PREPARE",
        duckdb_statement_type_DUCKDB_STATEMENT_TYPE_EXECUTE => "EXECUTE",
        _ => "non-SELECT",
    }
}

/// Refuse a query that holds more than one statement, so relate reads
/// records and never runs a script; a query that does not parse reports
/// the parser's own words.
unsafe fn ensure_single_statement(connection: ffi::duckdb_connection, sql: &str) -> Result<(), String> {
    let Ok(sql_c) = CString::new(sql) else {
        return Err("thinkthen usage: the relate query holds a NUL byte".into());
    };
    let mut extracted: ffi::duckdb_extracted_statements = std::ptr::null_mut();
    let count = unsafe { ffi::duckdb_extract_statements(connection, sql_c.as_ptr(), &mut extracted) };
    let outcome = if count == 0 {
        let message = if extracted.is_null() {
            std::ptr::null()
        } else {
            unsafe { ffi::duckdb_extract_statements_error(extracted) }
        };
        let text = if message.is_null() {
            "the parser reported no words".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        Err(format!("thinkthen usage: the relate query did not parse: {text}"))
    } else if count > 1 {
        Err(format!(
            "thinkthen usage: the relate query is one SQL statement and this one holds {count}; relate reads records, it does not run scripts"
        ))
    } else {
        Ok(())
    };
    unsafe { ffi::duckdb_destroy_extracted(&mut extracted) };
    outcome
}

/// Start the read-only transaction the records are read in.
unsafe fn begin_read_only(connection: ffi::duckdb_connection) -> Result<(), String> {
    unsafe { execute(connection, c"BEGIN TRANSACTION READ ONLY") }.map_err(|text| {
        format!("thinkthen usage: the relate query could not start its read-only transaction: {text}")
    })
}

/// Run `body` under the calling session's own search path when it set
/// one, so `USE other` reaches the query relate runs; the kept
/// connection's default path returns afterwards.
unsafe fn under_search_path<T>(
    connection: ffi::duckdb_connection,
    path: Option<&str>,
    body: impl FnOnce() -> Result<T, String>,
) -> Result<T, String> {
    let Some(text) = path.map(str::trim).filter(|text| !text.is_empty()) else {
        return body();
    };
    let quoted = text.replace('\'', "''");
    let Ok(sql) = CString::new(format!("SET search_path = '{quoted}'")) else {
        return Err("thinkthen usage: the calling session's search path holds a NUL byte".into());
    };
    unsafe { execute(connection, &sql) }.map_err(|reason| {
        format!("thinkthen usage: the relate query could not run under the calling session's search path {text}: {reason}")
    })?;
    body()
}

/// Run one statement and discard its rows; the failure text is the
/// engine's own.
unsafe fn execute(connection: ffi::duckdb_connection, sql: &std::ffi::CStr) -> Result<(), String> {
    let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
    let state = unsafe { ffi::duckdb_query(connection, sql.as_ptr(), &mut result) };
    if state != ffi::DuckDBSuccess {
        let message = unsafe { ffi::duckdb_result_error(&mut result) };
        let text = if message.is_null() {
            "the statement failed".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(text);
    }
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok(())
}

/// One statement's rows as `(id, text)` pairs; any value renders as its
/// text, so an integer id comes back as `1`.
unsafe fn run_statement(
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
    if rows > RECORD_CAP + 1 {
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(format!(
            "thinkthen usage: the relate query returned {rows} records and relate asks about at most {RECORD_CAP}; add a LIMIT or a WHERE"
        ));
    }
    let mut records = Vec::with_capacity(rows.min(RECORD_CAP + 1) as usize);
    for row in 0..rows.min(RECORD_CAP + 1) {
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
