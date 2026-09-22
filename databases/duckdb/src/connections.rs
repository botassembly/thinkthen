//! The connections this surface keeps: one per database that loads the
//! extension, and the facts a callback asks of them — which one belongs
//! to the caller's database, whether a file read is allowed, and when to
//! let go.
//!
//! The stable C API hands a table function's bind callback the caller's
//! client context but never the caller's connection, so the relate query
//! cannot run on that connection; it runs on a connection opened at load
//! time from the database the extension was loaded into. Two databases in
//! one process share this extension's statics, so each database must be
//! told apart. Names cannot do it: two in-memory databases both report
//! `memory`, and the first two-database fix resolved exactly one match and
//! failed both (review finding). Identity is a token instead: at load the
//! extension registers one extension setting on that database,
//! `thinkthen_instance_token`, whose default value is unique to the load,
//! and a bind reads the caller's own token from the caller's context —
//! the same C API door DuckDB uses for every setting, so the caller's
//! database answers with its own token and no name is consulted.
//!
//! A kept connection holds the database instance alive, and with it the
//! database file's lock: after the caller closed every connection, the
//! file stayed locked and an in-process reopen hung (review finding). A
//! reaper thread watches each kept connection's own
//! `duckdb_connection_count()` and, when the kept connection is the only
//! one left, disconnects it and forgets the entry, so a closed database
//! releases its file like any database with no connections. The reaper
//! holds the query gate around its count and its disconnect, and every
//! other use of a kept connection sits under the same gate, so a
//! connection is never disconnected mid-query.
//!
//! A relate inside a relate would wait forever on the connection the
//! outer query is already using. DuckDB runs the inner query's callbacks
//! on its own thread with the kept connection's own client context, so
//! the identity that tells it apart is that context's connection id: it
//! equals the kept connection's, where every caller's callback carries
//! the caller's own id. `caller_is_kept` is that comparison, and relate
//! refuses on it before it can wait on the gate.

use std::ffi::{CStr, CString, c_void};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

use duckdb::ffi;

/// The extension setting each loaded database carries: its own instance
/// token, so a caller's context answers with its database's identity.
const TOKEN_OPTION: &str = "thinkthen_instance_token";

/// One database's kept connection.
struct Kept {
    /// The database's own token; `None` when the setting could not
    /// register, in which case resolution falls back to names.
    token: Option<String>,
    /// The database's own `current_database()` name, for messages and the
    /// name-based fallback.
    name: String,
    /// The raw connection, opened at load time.
    connection: ffi::duckdb_connection,
    /// The kept connection's own DuckDB connection id, so a callback that
    /// runs INSIDE a query this connection is executing — a nested relate
    /// — can be told from a callback on a caller's own connection.
    connection_id: u64,
}

// Every use sits under the registry's mutex or inside the serialized
// query gate, which is the synchronization the handle needs.
unsafe impl Send for Kept {}

/// The databases that loaded the extension, in load order. Two databases
/// in one process share this static, so every entry is a candidate the
/// caller's context can name.
static KEPT: Mutex<Vec<Kept>> = Mutex::new(Vec::new());

/// The gate that serializes every query run on a kept connection: two
/// caller connections can scan relate at once, and the reaper counts on
/// one.
static RUN: Mutex<()> = Mutex::new(());

/// The next token's serial number.
static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);

/// Whether the reaper has been started; one per process.
static REAPER: AtomicBool = AtomicBool::new(false);

/// How often the reaper looks; small enough that a closed database
/// releases its file promptly, rare enough to cost nothing.
const REAP_INTERVAL: Duration = Duration::from_millis(200);

/// Open the database's kept connection, learn its name, register its
/// identity token, and remember it.
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
    let token = register_token(raw, &name).ok();
    let connection_id = kept_connection_id(raw);
    lock(&KEPT).push(Kept {
        token,
        name,
        connection: raw,
        connection_id,
    });
    spawn_reaper();
    Ok(raw)
}

/// Run `body` with the query gate held, so kept connections are used by
/// one scan at a time.
pub(crate) fn run_serialized<T>(body: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    let _gate = lock(&RUN);
    body()
}

/// Whether this callback runs inside a query the given kept connection is
/// itself executing: DuckDB binds and scans a nested table function with
/// that connection's own client context, so its connection id equals the
/// kept connection's, where a caller's callback carries the caller's own
/// id. Such a call must refuse before it waits on the gate its own query
/// holds — that wait is the nested-relate hang.
pub(crate) fn caller_is_kept(
    caller: ffi::duckdb_client_context,
    connection: ffi::duckdb_connection,
) -> bool {
    let kept = lock(&KEPT);
    let Some(entry) = kept.iter().find(|entry| entry.connection == connection) else {
        return false;
    };
    (unsafe { ffi::duckdb_client_context_get_connection_id(caller) }) as u64 == entry.connection_id
}

/// The kept connection's own DuckDB connection id.
fn kept_connection_id(connection: ffi::duckdb_connection) -> u64 {
    let mut context: ffi::duckdb_client_context = std::ptr::null_mut();
    unsafe { ffi::duckdb_connection_get_client_context(connection, &mut context) };
    if context.is_null() {
        return 0;
    }
    let id = unsafe { ffi::duckdb_client_context_get_connection_id(context) } as u64;
    let mut context = context;
    unsafe { ffi::duckdb_destroy_client_context(&mut context) };
    id
}

/// The kept connection belonging to the caller's database.
///
/// The caller's own token answers first: the caller's context reads the
/// token its database registered, and exactly that entry's connection
/// comes back, so two databases with the same name never collide. When
/// the token setting could not register in a database, the name-based
/// resolution answers instead: one loaded database answers directly, and
/// with several, the caller's context resolves each kept database's name
/// as a catalog — exactly one match is the caller's.
///
/// # Errors
///
/// The defect kind when no kept database matches the caller, or when
/// several name matches leave the caller's database ambiguous.
pub(crate) fn for_caller(
    caller: ffi::duckdb_client_context,
) -> Result<ffi::duckdb_connection, String> {
    let kept = lock(&KEPT);
    if let Some(token) = token_of(caller) {
        return match kept
            .iter()
            .find(|entry| entry.token.as_deref() == Some(token.as_str()))
        {
            Some(entry) => Ok(entry.connection),
            None => Err(
                "thinkthen defect: the calling database carries no loaded database's token; the extension is loaded per database, and a released kept connection needs a fresh LOAD"
                    .into(),
            ),
        };
    }
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
            "thinkthen defect: two loaded databases share a name and carry no identity token, so the calling database cannot be told apart"
                .into(),
        ),
    }
}

/// Whether every loaded database allows reading this path: never read a
/// file a calling database would refuse. The scalar doors cannot name the
/// calling database, so any database that forbids the read refuses them
/// all.
pub(crate) fn file_read_refusal(path: &str) -> Option<String> {
    let kept = lock(&KEPT);
    for entry in kept.iter() {
        let mut context: ffi::duckdb_client_context = std::ptr::null_mut();
        unsafe { ffi::duckdb_connection_get_client_context(entry.connection, &mut context) };
        if context.is_null() {
            continue;
        }
        let refusal = read_refusal(context, path);
        let mut context = context;
        unsafe { ffi::duckdb_destroy_client_context(&mut context) };
        if refusal.is_some() {
            return refusal;
        }
    }
    None
}

/// Whether this database allows reading this path, read live through the
/// DuckDB API so the database's own settings answer. The rules mirror
/// `DBConfig::CanAccessFile`: external access on allows everything, and
/// with it off only `allowed_paths` and `allowed_directories` carve out
/// exceptions.
pub(crate) fn read_refusal(context: ffi::duckdb_client_context, path: &str) -> Option<String> {
    if local_files_disabled(context) {
        return Some(format!(
            "thinkthen local: the question file {path} was not read: the LocalFileSystem is disabled for this database"
        ));
    }
    if external_access_enabled(context) {
        return None;
    }
    if allowed_path(context, path) {
        return None;
    }
    Some(format!(
        "thinkthen local: the question file {path} was not read: enable_external_access is off for this database and the path is outside its allowed_paths and allowed_directories"
    ))
}

/// Whether this context's database allows external file access, read
/// through the DuckDB API so the database's own setting answers.
pub(crate) fn external_access_enabled(context: ffi::duckdb_client_context) -> bool {
    match option_bool(context, "enable_external_access") {
        Some(enabled) => enabled,
        // An option this DuckDB does not know cannot forbid anything.
        None => true,
    }
}

/// Whether `disabled_filesystems` names the LocalFileSystem.
fn local_files_disabled(context: ffi::duckdb_client_context) -> bool {
    option_text(context, "disabled_filesystems")
        .map(|text| {
            text.split(',')
                .any(|name| name.trim().eq_ignore_ascii_case("LocalFileSystem"))
        })
        .unwrap_or(false)
}

/// Whether the path is one `allowed_paths` or `allowed_directories` names.
/// The settings hold canonical paths with forward slashes; the path is
/// canonicalized the same way before the comparison, falling back to its
/// own text when it does not canonicalize.
fn allowed_path(context: ffi::duckdb_client_context, path: &str) -> bool {
    let canonical = std::fs::canonicalize(path)
        .map(|real| real.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.replace('\\', "/"));
    if option_texts(context, "allowed_paths")
        .iter()
        .any(|allowed| allowed.replace('\\', "/") == canonical)
    {
        return true;
    }
    option_texts(context, "allowed_directories").iter().any(|allowed| {
        let mut prefix = allowed.replace('\\', "/");
        if !prefix.ends_with('/') {
            prefix.push('/');
        }
        canonical.starts_with(&prefix)
    })
}

/// A boolean setting's value, when the setting answers.
fn option_bool(context: ffi::duckdb_client_context, name: &str) -> Option<bool> {
    let value = option_value(context, name)?;
    let answer = unsafe { ffi::duckdb_get_bool(value) };
    let mut value = value;
    unsafe { ffi::duckdb_destroy_value(&mut value) };
    Some(answer)
}

/// A text setting's value, when the setting answers.
fn option_text(context: ffi::duckdb_client_context, name: &str) -> Option<String> {
    let value = option_value(context, name)?;
    let raw = unsafe { ffi::duckdb_get_varchar(value) };
    let text = if raw.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(raw) }.to_string_lossy().into_owned())
    };
    if !raw.is_null() {
        unsafe { ffi::duckdb_free(raw as *mut c_void) };
    }
    let mut value = value;
    unsafe { ffi::duckdb_destroy_value(&mut value) };
    text.filter(|held| !held.is_empty())
}

/// A list setting's entries; a text value reads as its single entry, and
/// an unanswered setting reads as no entries.
fn option_texts(context: ffi::duckdb_client_context, name: &str) -> Vec<String> {
    let Some(value) = option_value(context, name) else {
        return Vec::new();
    };
    let count = unsafe { ffi::duckdb_get_list_size(value) };
    if count == 0 {
        let text = option_text(context, name);
        let mut value = value;
        unsafe { ffi::duckdb_destroy_value(&mut value) };
        return text.into_iter().collect();
    }
    let mut entries = Vec::with_capacity(count as usize);
    for at in 0..count {
        let mut child = unsafe { ffi::duckdb_get_list_child(value, at) };
        let raw = unsafe { ffi::duckdb_get_varchar(child) };
        if !raw.is_null() {
            let text = unsafe { CStr::from_ptr(raw) }.to_string_lossy().into_owned();
            unsafe { ffi::duckdb_free(raw as *mut c_void) };
            if !text.is_empty() {
                entries.push(text);
            }
        }
        unsafe { ffi::duckdb_destroy_value(&mut child) };
    }
    let mut value = value;
    unsafe { ffi::duckdb_destroy_value(&mut value) };
    entries
}

/// One setting's value as DuckDB answers it for this context.
fn option_value(context: ffi::duckdb_client_context, name: &str) -> Option<ffi::duckdb_value> {
    let name = CString::new(name).ok()?;
    let mut scope = ffi::duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_INVALID;
    let value = unsafe {
        ffi::duckdb_client_context_get_config_option(context, name.as_ptr(), &mut scope)
    };
    if value.is_null() {
        None
    } else {
        Some(value)
    }
}

/// Read a question file through DuckDB's own file system, so the rules
/// DuckDB applies to its own reads apply here: a disabled file system
/// refuses with DuckDB's own words, and a caller's file system carries
/// the caller's own access checks. The caller's context answers where one
/// is known; the scalar doors read through the loaded databases' own
/// file system, the conservative side of the same door.
///
/// # Errors
///
/// The local kind, with the file system's own message, when the read is
/// refused or fails.
pub(crate) fn read_question_file(
    path: &str,
    caller: Option<ffi::duckdb_client_context>,
) -> Result<String, String> {
    let borrowed = caller.is_some();
    let Some(context) = caller.or_else(kept_context) else {
        return std::fs::read_to_string(path)
            .map_err(|error| format!("thinkthen local: the question file {path} did not read: {error}"));
    };
    let outcome = unsafe { read_through_file_system(context, path) };
    if !borrowed {
        let mut context = context;
        unsafe { ffi::duckdb_destroy_client_context(&mut context) };
    }
    outcome
}

/// A client context from the first kept connection, when one exists.
fn kept_context() -> Option<ffi::duckdb_client_context> {
    let connection = lock(&KEPT).first().map(|entry| entry.connection)?;
    let mut context: ffi::duckdb_client_context = std::ptr::null_mut();
    unsafe { ffi::duckdb_connection_get_client_context(connection, &mut context) };
    if context.is_null() {
        None
    } else {
        Some(context)
    }
}

/// One read through the context's file system; the bytes are read through
/// a DuckDB file handle, so no rule this extension forgot can be skipped.
unsafe fn read_through_file_system(
    context: ffi::duckdb_client_context,
    path: &str,
) -> Result<String, String> {
    let fs = unsafe { ffi::duckdb_client_context_get_file_system(context) };
    if fs.is_null() {
        return std::fs::read_to_string(path)
            .map_err(|error| format!("thinkthen local: the question file {path} did not read: {error}"));
    }
    let outcome = unsafe { open_and_read(fs, path) };
    let mut fs = fs;
    unsafe { ffi::duckdb_destroy_file_system(&mut fs) };
    outcome
}

/// Open, read, and close one file through the file system.
unsafe fn open_and_read(fs: ffi::duckdb_file_system, path: &str) -> Result<String, String> {
    let Ok(c_path) = CString::new(path) else {
        return Err("thinkthen usage: the question file path holds a NUL byte".into());
    };
    let options = unsafe { ffi::duckdb_create_file_open_options() };
    if options.is_null() {
        return Err("thinkthen defect: the file system refused an options slot".into());
    }
    unsafe {
        ffi::duckdb_file_open_options_set_flag(
            options,
            ffi::duckdb_file_flag_DUCKDB_FILE_FLAG_READ,
            true,
        )
    };
    let mut handle: ffi::duckdb_file_handle = std::ptr::null_mut();
    let state = unsafe { ffi::duckdb_file_system_open(fs, c_path.as_ptr(), options, &mut handle) };
    let mut options = options;
    unsafe { ffi::duckdb_destroy_file_open_options(&mut options) };
    if state != ffi::DuckDBSuccess {
        return Err(format!(
            "thinkthen local: the question file {path} was not read: {}",
            unsafe { file_system_error(fs) }
        ));
    }
    let size = unsafe { ffi::duckdb_file_handle_size(handle) };
    let mut buffer = vec![0_u8; size.max(0) as usize];
    let read = unsafe {
        ffi::duckdb_file_handle_read(handle, buffer.as_mut_ptr() as *mut c_void, size)
    };
    let closed = unsafe { ffi::duckdb_file_handle_close(handle) };
    if closed != ffi::DuckDBSuccess || read < 0 {
        return Err(format!("thinkthen local: the question file {path} did not read"));
    }
    buffer.truncate(read as usize);
    String::from_utf8(buffer)
        .map_err(|_| format!("thinkthen local: the question file {path} is not UTF-8"))
}

/// The file system's own last error, as its words.
unsafe fn file_system_error(fs: ffi::duckdb_file_system) -> String {
    let data = unsafe { ffi::duckdb_file_system_error_data(fs) };
    if data.is_null() {
        return "the file system refused the open".to_owned();
    }
    let message = unsafe { ffi::duckdb_error_data_message(data) };
    if message.is_null() {
        return "the file system refused the open".to_owned();
    }
    unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
}

/// The caller context's own instance token, when the setting answers.
fn token_of(caller: ffi::duckdb_client_context) -> Option<String> {
    option_text(caller, TOKEN_OPTION)
}

/// The calling session's own search path, when it set one: the first
/// entry names the catalog and schema `USE` or `SET search_path` chose,
/// which is the path the relate query must run under to read the tables
/// the caller means.
pub(crate) fn search_path_of(caller: ffi::duckdb_client_context) -> Option<String> {
    option_text(caller, "search_path")
}

/// Whether the caller's context resolves a catalog with this name; the
/// name-based fallback.
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
    unsafe { ffi::duckdb_free(raw as *mut c_void) };
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok(name)
}

/// Register this database's identity token as an extension setting, so a
/// bind can read the caller's own token back. The token is unique to the
/// load: the process, a serial number, and the clock.
fn register_token(connection: ffi::duckdb_connection, name: &str) -> Result<String, String> {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|span| span.as_nanos())
        .unwrap_or(0);
    let token = format!(
        "thinkthen:{name}:{}:{}:{stamp}",
        std::process::id(),
        NEXT_TOKEN.fetch_add(1, Ordering::SeqCst)
    );
    let c_name = CString::new(TOKEN_OPTION).map_err(|_| "the token name holds no NUL".to_owned())?;
    let c_description = CString::new("The ThinkThen surface's identity token for this database")
        .map_err(|_| "the token description holds no NUL".to_owned())?;
    let c_token = CString::new(token.as_str()).map_err(|_| "the token holds no NUL".to_owned())?;
    unsafe {
        let option = ffi::duckdb_create_config_option();
        if option.is_null() {
            return Err("thinkthen defect: the settings slot did not create".into());
        }
        ffi::duckdb_config_option_set_name(option, c_name.as_ptr());
        ffi::duckdb_config_option_set_description(option, c_description.as_ptr());
        let mut kind = ffi::duckdb_create_logical_type(ffi::DUCKDB_TYPE_DUCKDB_TYPE_VARCHAR);
        ffi::duckdb_config_option_set_type(option, kind);
        ffi::duckdb_destroy_logical_type(&mut kind);
        let mut value = ffi::duckdb_create_varchar(c_token.as_ptr());
        ffi::duckdb_config_option_set_default_value(option, value);
        ffi::duckdb_destroy_value(&mut value);
        ffi::duckdb_config_option_set_default_scope(
            option,
            ffi::duckdb_config_option_scope_DUCKDB_CONFIG_OPTION_SCOPE_GLOBAL,
        );
        let state = ffi::duckdb_register_config_option(connection, option);
        let mut option = option;
        ffi::duckdb_destroy_config_option(&mut option);
        if state != ffi::DuckDBSuccess {
            return Err("thinkthen defect: the database refused the identity setting".into());
        }
    }
    Ok(token)
}

/// Start the reaper once per process.
fn spawn_reaper() {
    if REAPER.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = std::thread::Builder::new()
        .name("thinkthen-reaper".into())
        .spawn(|| loop {
            std::thread::sleep(REAP_INTERVAL);
            reap_once();
        });
}

/// One look: for every kept connection left alone in its database, let it
/// go, so the database instance — and its file lock — can be released the
/// way a database with no connections is.
fn reap_once() {
    let watched: Vec<ffi::duckdb_connection> =
        lock(&KEPT).iter().map(|entry| entry.connection).collect();
    for connection in watched {
        let alone = run_serialized(|| count_connections(connection))
            .map(|count| count <= 1)
            .unwrap_or(false);
        if alone {
            release(connection);
        }
    }
}

/// Close one kept connection when its database is still alone; a caller
/// connection that arrived since the count keeps the entry.
fn release(connection: ffi::duckdb_connection) {
    let removed = {
        let mut kept = lock(&KEPT);
        kept.iter()
            .position(|entry| entry.connection == connection)
            .map(|at| kept.remove(at))
    };
    let Some(mut entry) = removed else {
        return;
    };
    let still_alone = run_serialized(|| count_connections(entry.connection))
        .map(|count| count <= 1)
        .unwrap_or(false);
    if still_alone {
        unsafe { ffi::duckdb_disconnect(&mut entry.connection) };
    } else {
        lock(&KEPT).push(entry);
    }
}

/// The connection count of the connection's own database, through the
/// engine's `duckdb_connection_count()`: the kept connection itself
/// counted, so one means no caller connection remains. The function
/// answers one row holding the count, so the row is read, never wrapped
/// in a `count(*)` that would answer one for the function's one row.
fn count_connections(connection: ffi::duckdb_connection) -> Result<u64, String> {
    let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
    let sql = c"SELECT * FROM duckdb_connection_count()";
    if unsafe { ffi::duckdb_query(connection, sql.as_ptr(), &mut result) } != ffi::DuckDBSuccess {
        let message = unsafe { ffi::duckdb_result_error(&mut result) };
        let text = if message.is_null() {
            "the count failed".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(text);
    }
    let raw = unsafe { ffi::duckdb_value_varchar(&mut result, 0, 0) };
    let count = if raw.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .parse::<u64>()
            .ok()
    };
    if !raw.is_null() {
        unsafe { ffi::duckdb_free(raw as *mut c_void) };
    }
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    count.ok_or_else(|| "the connection count did not read".to_owned())
}

/// A mutex's guard, surviving a poisoned lock the way a contained panic
/// leaves it: the data is still the registry or the gate.
fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
