//! The connections this surface keeps: one per database that loads the
//! extension, and the two facts a callback asks of them — which one
//! belongs to the caller's database, and whether the database allows
//! file reads.
//!
//! The stable C API hands a table function's bind callback the caller's
//! client context but never the caller's connection, so the relate query
//! cannot run on that connection; it runs on a connection opened at load
//! time from the database the extension was loaded into. The caller's
//! context names the database among the loaded ones, which is why the
//! registry is keyed by each database's own `current_database()` name and
//! resolved through `duckdb_client_context_get_catalog` at bind time.
//! Two databases in one process share this extension's statics, so one
//! global connection used to serve whichever database loaded last — the
//! review's wrong-database bug.
//!
//! The client-context door is unstable C API, and the extension declares
//! itself `C_STRUCT_UNSTABLE` (`USE_UNSTABLE_C_API=1` in the Makefile),
//! so DuckDB fills it in.

use std::ffi::{CStr, CString};
use std::sync::Mutex;

use duckdb::ffi;

/// One database's kept connection.
struct Kept {
    /// The database's own `current_database()` name.
    name: String,
    /// The raw connection, opened at load time and never disconnected.
    connection: ffi::duckdb_connection,
}

// Every use sits under the registry's mutex or inside the serialized
// query gate, which is the synchronization the handle needs.
unsafe impl Send for Kept {}

/// The databases that loaded the extension, in load order. Two databases
/// in one process share this static, so every entry is a candidate the
/// caller's context can name.
static KEPT: Mutex<Vec<Kept>> = Mutex::new(Vec::new());

/// The gate that serializes every query run on a kept connection: two
/// caller connections can scan relate at once.
static RUN: Mutex<()> = Mutex::new(());

/// Open the database's kept connection, learn its name, and remember it.
///
/// # Errors
///
/// The defect kind when the database refuses a connection or its name
/// does not read.
pub(crate) fn register(database: ffi::duckdb_database) -> Result<ffi::duckdb_connection, String> {
    let mut raw: ffi::duckdb_connection = std::ptr::null_mut();
    if unsafe { ffi::duckdb_connect(database, &mut raw) } != ffi::DuckDBSuccess {
        return Err("thinkthen defect: the database refused a kept connection".into());
    }
    let name = database_name(raw)?;
    lock(&KEPT).push(Kept { name, connection: raw });
    Ok(raw)
}

/// Run `body` with the query gate held, so kept connections are used by
/// one scan at a time.
pub(crate) fn run_serialized<T>(body: impl FnOnce() -> T) -> T {
    let _gate = lock(&RUN);
    body()
}

/// The kept connection belonging to the caller's database.
///
/// One loaded database answers directly. With several, the caller's
/// context resolves each kept database's name as a catalog; exactly one
/// match is the caller's. None means the caller's database did not load
/// this extension, and several means two databases share a name and
/// cannot be told apart — both are defects the caller sees, not guesses.
///
/// # Errors
///
/// The defect kind when no kept database or more than one matches.
pub(crate) fn for_caller(caller: ffi::duckdb_client_context) -> Result<ffi::duckdb_connection, String> {
    let kept = lock(&KEPT);
    if kept.len() == 1 {
        return Ok(kept[0].connection);
    }
    let matched: Vec<&Kept> = kept
        .iter()
        .filter(|entry| catalog_exists(caller, &entry.name))
        .collect();
    match matched.as_slice() {
        [only] => Ok(only.connection),
        [] => Err(
            "thinkthen defect: no loaded database matches the calling connection; the extension is loaded per database"
                .into(),
        ),
        _ => Err(
            "thinkthen defect: two loaded databases share a name, so the calling database cannot be told apart"
                .into(),
        ),
    }
}

/// Whether every loaded database allows external file access.
///
/// The scalar doors cannot name the calling database, so any database
/// that forbids file reads refuses them all: never read a file a calling
/// database's `enable_external_access` would refuse.
pub(crate) fn files_allowed() -> bool {
    let kept = lock(&KEPT);
    kept.iter().all(|entry| {
        let mut context: ffi::duckdb_client_context = std::ptr::null_mut();
        unsafe { ffi::duckdb_connection_get_client_context(entry.connection, &mut context) };
        if context.is_null() {
            return true;
        }
        let allowed = external_access_enabled(context);
        let mut context = context;
        unsafe { ffi::duckdb_destroy_client_context(&mut context) };
        allowed
    })
}

/// Whether this context's database allows external file access, read
/// through the DuckDB API so the database's own setting answers.
pub(crate) fn external_access_enabled(context: ffi::duckdb_client_context) -> bool {
    let name = c"enable_external_access";
    let mut scope = ffi::duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_INVALID;
    let mut value =
        unsafe { ffi::duckdb_client_context_get_config_option(context, name.as_ptr(), &mut scope) };
    if value.is_null() {
        // An option this DuckDB does not know cannot forbid anything.
        return true;
    }
    let enabled = unsafe { ffi::duckdb_get_bool(value) };
    unsafe { ffi::duckdb_destroy_value(&mut value) };
    enabled
}

/// Whether the caller's context resolves a catalog with this name.
fn catalog_exists(caller: ffi::duckdb_client_context, name: &str) -> bool {
    let Ok(name) = CString::new(name) else {
        return false;
    };
    let mut catalog =
        unsafe { ffi::duckdb_client_context_get_catalog(caller, name.as_ptr()) };
    let exists = !catalog.is_null();
    if exists {
        unsafe { ffi::duckdb_destroy_catalog(&mut catalog) };
    }
    exists
}

/// The database's own name, through `current_database()` on its kept
/// connection.
fn database_name(connection: ffi::duckdb_connection) -> Result<String, String> {
    let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
    let sql = c"SELECT current_database()";
    if unsafe { ffi::duckdb_query(connection, sql.as_ptr(), &mut result) } != ffi::DuckDBSuccess {
        let message = unsafe { ffi::duckdb_result_error(&mut result) };
        let text = if message.is_null() {
            "the query failed".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(format!("thinkthen defect: the database's name did not read: {text}"));
    }
    let raw = unsafe { ffi::duckdb_value_varchar(&mut result, 0, 0) };
    if raw.is_null() {
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err("thinkthen defect: current_database() returned nothing".into());
    }
    let name = unsafe { CStr::from_ptr(raw) }.to_string_lossy().into_owned();
    unsafe { ffi::duckdb_free(raw as *mut std::os::raw::c_void) };
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok(name)
}

/// A mutex's guard, surviving a poisoned lock the way a contained panic
/// leaves it: the data is still the registry or the gate.
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
