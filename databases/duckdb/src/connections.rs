//! The kept connections: one per loaded database, resolved for a caller
//! through an identity no SQL can forge, and released only when no
//! prepared statement still holds a handle to them.
//!
//! A table function cannot run SQL on the caller's own connection, so
//! each database gets a kept connection the relate scan reads through.
//! Two databases in one process share these statics, so the registry
//! must tell them apart: the identity is a uniquely named in-memory
//! database attached to the caller's own instance at LOAD time. The
//! caller's context resolves that name if and only if it belongs to the
//! same instance, so two databases with one name — two `:memory:`
//! instances — never collide, and no `SET` of any setting can move a
//! caller onto another database's connection, because the identity is a
//! catalog object, not a setting (review 3, finding 6: the old instance
//! token was an ordinary setting, and another database's token could be
//! `SET` onto a session to read that database's tables past
//! `enable_external_access`).
//!
//! The kept connection outlives its usefulness when its database has no
//! caller connections left, because the database instance — and with it
//! the file lock of a file database — stays held while any connection
//! lives. The reaper releases those, but a raw pointer handed to a
//! prepared statement and freed by the reaper underneath it is a
//! use-after-free (review 3, finding 1), so every handle the registry
//! gives out is a [`KeptGuard`]: it counts itself under the registry
//! lock, the reaper refuses new guards for a database it wants to
//! release, and the disconnect happens only at zero guards. The reaper
//! is budgeted and backs off (review 3, finding 26: one count query per
//! database per 200 ms burned 64% of a core over 500 idle databases).

use std::ffi::{CStr, CString, c_void};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use duckdb::ffi;

/// One database's shared state: the query gate every use of its kept
/// connection holds, the count of live guards, and the reaper's
/// retirement flag.
pub(crate) struct DbState {
    /// Serializes every query run on the kept connection: relate scans
    /// and the reaper's counts. One gate per database, never global —
    /// review 3, finding 12: the one global gate let one database's
    /// eight-million-row relate block another database's for 4.25 s.
    gate: Mutex<()>,
    /// Live [`KeptGuard`]s: prepared statements holding this database.
    /// Acquired under the registry lock beside the retirement check;
    /// released anywhere, because retirement already blocks new guards.
    users: AtomicUsize,
    /// The reaper's decision: no new guards, and the disconnect waits
    /// for `users` to reach zero. A count that finds caller connections
    /// clears it again.
    retired: AtomicBool,
}

/// A counted handle to one database's kept connection. The connection
/// cannot be disconnected while one of these lives, so a prepared
/// statement can hold one across any number of reaper passes.
pub(crate) struct KeptGuard {
    connection: ffi::duckdb_connection,
    state: Arc<DbState>,
}

impl KeptGuard {
    /// The kept connection; alive as long as this guard lives.
    pub(crate) fn connection(&self) -> ffi::duckdb_connection {
        self.connection
    }

    /// This database's query gate; hold it to run SQL on the connection.
    pub(crate) fn gate(&self) -> &Mutex<()> {
        &self.state.gate
    }
}

impl Drop for KeptGuard {
    fn drop(&mut self) {
        self.state.users.fetch_sub(1, Ordering::AcqRel);
        // A guard outliving the reaper's decision holds the disconnect
        // until now; wake the reaper so it lands promptly, not on the
        // next tick.
        if self.state.retired.load(Ordering::Acquire) {
            REAP_POKE.notify_all();
        }
    }
}

/// One database's registry entry.
struct Kept {
    /// The database's own `current_database()` name, for messages, the
    /// fast single-database path, and the catalog the probe resolves in.
    name: String,
    /// The attached in-memory database that identifies this instance:
    /// `thinkthen_instance_<128-bit hex>`. Resolvable by a caller's
    /// context only when the caller belongs to this same instance.
    probe: Option<String>,
    /// The kept connection, opened at load time. Freed only by the
    /// reaper at zero guards, under the gate.
    connection: ffi::duckdb_connection,
    /// The kept connection's own DuckDB connection id, so a callback
    /// running inside a query this connection executes — a nested
    /// relate — can be told from one on a caller's connection.
    connection_id: u64,
    /// The shared state guards and the reaper coordinate through.
    state: Arc<DbState>,
}

// Every use sits under the registry's mutex or behind a guard's count,
// which is the synchronization the handle needs.
unsafe impl Send for Kept {}

/// The databases that loaded the extension, in load order. Two
/// databases in one process share this static, so every entry is a
/// candidate the caller's context can name.
static KEPT: Mutex<Vec<Kept>> = Mutex::new(Vec::new());

/// The next probe's serial number, folded into the random name.
static NEXT_PROBE: AtomicU64 = AtomicU64::new(1);

/// Whether the reaper has been started; one per process.
static REAPER: AtomicBool = AtomicBool::new(false);

/// The reaper's sleep, adapted: it grows while nothing is released and
/// resets on any release, so a fleet of idle databases costs a fraction
/// of the fixed 200 ms pass (review 3, finding 26).
static REAP_INTERVAL_MS: AtomicU64 = AtomicU64::new(200);

/// How many entries one reaper pass may examine; the rest wait for the
/// next pass, so one pass is bounded no matter how many databases are
/// loaded.
const REAP_BUDGET: usize = 8;

/// The round-robin cursor over the registry, so budgeted passes still
/// reach every database eventually.
static REAP_CURSOR: AtomicUsize = AtomicUsize::new(0);

/// The reaper's clock: poked by a guard dropping on a retired database
/// so its disconnect lands promptly instead of at the next tick.
static REAP_TICK: Mutex<()> = Mutex::new(());
static REAP_POKE: Condvar = Condvar::new();
/// Open the database's kept connection, learn its name, attach its
/// identity probe, and remember it.
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
    let probe = attach_probe(raw).ok();
    let connection_id = kept_connection_id(raw);
    lock(&KEPT).push(Kept {
        name,
        probe,
        connection: raw,
        connection_id,
        state: Arc::new(DbState {
            gate: Mutex::new(()),
            users: AtomicUsize::new(0),
            retired: AtomicBool::new(false),
        }),
    });
    REAP_INTERVAL_MS.store(200, Ordering::Release);
    spawn_reaper();
    Ok(raw)
}

/// The uniquely named in-memory database attached as this instance's
/// identity probe. `:memory:` attachments hold no file and live in the
/// instance's catalog manager, so nothing on disk changes; the caller's
/// own context resolves the name only inside the same instance. A
/// read-only database that refuses the attach keeps `None` and falls
/// back to names.
fn attach_probe(connection: ffi::duckdb_connection) -> Result<String, String> {
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|span| span.as_nanos())
        .unwrap_or(0);
    let probe = format!(
        "thinkthen_instance_{:x}_{:x}_{}",
        stamp,
        NEXT_PROBE.fetch_add(1, Ordering::SeqCst),
        std::process::id()
    );
    let sql = format!("ATTACH ':memory:' AS {probe}");
    let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
    let Ok(sql_c) = CString::new(sql) else {
        return Err("the probe name holds no NUL".into());
    };
    if unsafe { ffi::duckdb_query(connection, sql_c.as_ptr(), &mut result) } != ffi::DuckDBSuccess {
        let message = unsafe { ffi::duckdb_result_error(&mut result) };
        let text = if message.is_null() {
            "the attach refused".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        return Err(format!("the database refused the identity probe: {text}"));
    }
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok(probe)
}

/// A guard on `entry`, or `None` when the reaper has retired it.
fn acquire(entry: &Kept) -> Option<KeptGuard> {
    if entry.state.retired.load(Ordering::Acquire) {
        return None;
    }
    entry.state.users.fetch_add(1, Ordering::AcqRel);
    Some(KeptGuard {
        connection: entry.connection,
        state: entry.state.clone(),
    })
}

/// The kept connection belonging to the caller's database, as a guard.
///
/// One loaded database needs no identity at all. Otherwise the caller's
/// context resolves each entry's probe — the uniquely named in-memory
/// database attached at that entry's LOAD — and exactly the entry whose
/// probe resolves is the caller's own instance, because a context
/// resolves catalogs only inside the instance it belongs to. The
/// previous instance-token setting is gone: it was SQL-settable, so one
/// database's token could steer another's relate (review 3, finding 6).
/// A database whose probe could not attach is named by its catalog, the
/// name fallback; two same-named databases without probes are an error
/// that names the boundary rather than a guess.
///
/// # Errors
///
/// The defect kind when no loaded database matches the caller.
pub(crate) fn for_caller(caller: ffi::duckdb_client_context) -> Result<KeptGuard, String> {
    {
        let kept = lock(&KEPT);
        if kept.len() == 1 {
            return acquire(&kept[0]).ok_or_else(reload_message);
        }
    }
    // Guards for every candidate, taken under one registry lock, then
    // probed outside it: the probe reads catalogs, and no lock order
    // ever puts the registry inside a gate.
    let candidates: Vec<(KeptGuard, Option<String>, String)> = {
        let kept = lock(&KEPT);
        kept.iter()
            .filter_map(|entry| {
                acquire(entry).map(|guard| (guard, entry.probe.clone(), entry.name.clone()))
            })
            .collect()
    };
    if candidates.is_empty() {
        return Err(reload_message());
    }
    let mut hits: Vec<KeptGuard> = Vec::new();
    let mut ambiguity: Option<String> = None;
    for (guard, probe, name) in candidates {
        let mine = match &probe {
            Some(probe) => catalog_exists(caller, probe),
            None => {
                // No probe: this entry can only be named by its catalog,
                // and only when no other entry carries the same name.
                ambiguity = Some(name.clone());
                catalog_exists(caller, &name)
            }
        };
        if mine {
            hits.push(guard);
        }
    }
    match hits.len() {
        1 => Ok(hits.pop().expect("one hit")),
        0 => Err(format!(
            "thinkthen defect: no loaded database matches the calling connection; the extension is loaded per database{}",
            ambiguity.map(|_| " and a name-only database could not be matched").unwrap_or("")
        )),
        _ => Err(
            "thinkthen defect: two loaded databases answered the caller's identity; the probes must be unique, so this cannot happen"
                .into(),
        ),
    }
}

/// The reload remedy, as one message.
fn reload_message() -> String {
    "thinkthen defect: the calling database's kept connection was released with its last caller; LOAD the extension again".into()
}

/// Whether this callback runs inside a query the given kept connection
/// is itself executing: DuckDB binds and scans a nested table function
/// with that connection's own client context, so its connection id
/// equals the kept connection's, where a caller's callback carries the
/// caller's own id. Such a call must refuse before it waits on the gate
/// its own query holds — that wait is the nested-relate hang.
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

/// Whether every loaded database allows reading this path: never read a
/// file a calling database would refuse. The scalar doors cannot name
/// the calling database, so any database that forbids the read refuses
/// them all. Every context read runs behind a guard, so the reaper
/// cannot disconnect the connection it came from.
pub(crate) fn file_read_refusal(path: &str) -> Option<String> {
    let guards: Vec<KeptGuard> = {
        let kept = lock(&KEPT);
        kept.iter().filter_map(acquire).collect()
    };
    for guard in guards {
        let mut context: ffi::duckdb_client_context = std::ptr::null_mut();
        unsafe { ffi::duckdb_connection_get_client_context(guard.connection(), &mut context) };
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
/// is known; the scalar doors read through a guarded kept connection,
/// the conservative side of the same door.
///
/// # Errors
///
/// The local kind, with the file system's own message, when the read is
/// refused or fails.
pub(crate) fn read_question_file(
    path: &str,
    caller: Option<ffi::duckdb_client_context>,
) -> Result<String, String> {
    let held = caller.is_some();
    let outcome = match caller {
        Some(context) => unsafe { read_through_file_system(context, path) },
        None => {
            let Some((guard, context)) = kept_context() else {
                return std::fs::read_to_string(path).map_err(|error| {
                    format!("thinkthen local: the question file {path} did not read: {error}")
                });
            };
            let outcome = unsafe { read_through_file_system(context, path) };
            let mut context = context;
            unsafe { ffi::duckdb_destroy_client_context(&mut context) };
            drop(guard);
            outcome
        }
    };
    let _ = held;
    outcome
}

/// A guarded kept connection and a context borrowed from it. The guard
/// keeps the connection alive for the context's whole life; without it
/// the reaper could disconnect the connection between the registry read
/// and the file read (review 3, finding 1's second window).
fn kept_context() -> Option<(KeptGuard, ffi::duckdb_client_context)> {
    let guard = {
        let kept = lock(&KEPT);
        kept.iter().find_map(acquire)?
    };
    let mut context: ffi::duckdb_client_context = std::ptr::null_mut();
    unsafe { ffi::duckdb_connection_get_client_context(guard.connection(), &mut context) };
    if context.is_null() {
        None
    } else {
        Some((guard, context))
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

/// The calling session's own search path, when it set one: the first
/// entry names the catalog and schema `USE` or `SET search_path` chose,
/// which is the path the relate query must run under to read the tables
/// the caller means.
pub(crate) fn search_path_of(caller: ffi::duckdb_client_context) -> Option<String> {
    option_text(caller, "search_path")
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
    unsafe { ffi::duckdb_free(raw as *mut c_void) };
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok(name)
}

/// Start the reaper once per process. The thread sleeps on its clock —
/// poked by a guard dropping on a retired database — and each pass
/// examines a bounded slice of the registry, so a fleet of idle
/// databases costs one pass's budget, not a constant 64% of a core
/// (review 3, finding 26).
fn spawn_reaper() {
    if REAPER.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = std::thread::Builder::new()
        .name("thinkthen-reaper".into())
        .spawn(|| loop {
            let interval = Duration::from_millis(REAP_INTERVAL_MS.load(Ordering::Acquire));
            let Ok(clock) = REAP_TICK.lock() else {
                return;
            };
            let (_guard, _woke) = REAP_POKE
                .wait_timeout(clock, interval)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            drop(_guard);
            let released = reap_once();
            let next = if released > 0 {
                200
            } else {
                (REAP_INTERVAL_MS.load(Ordering::Acquire) * 2).min(2_000)
            };
            REAP_INTERVAL_MS.store(next, Ordering::Release);
        });
}

/// One pass: retire and examine a bounded slice of the registry. A
/// candidate is marked retired first — new guards stop — then counted
/// under its own gate, and only a database that is alone at zero guards
/// is disconnected. A database still holding caller connections is
/// un-retired and stays; a database whose gate a scan is using stays
/// retired (the scan holds a guard, and that guard's drop pokes the
/// reaper to finish the release). The gate is tried, never waited for,
/// so a long relate never stalls a pass.
///
/// The disconnect and its registry removal happen outside every gate and
/// after the registry lock, so no lock order puts the registry inside a
/// gate.
fn reap_once() -> usize {
    let mut released = 0usize;
    // Candidates: a bounded, wrapping slice of the registry, stepping by
    // the budget so passes tile it without gaps.
    let slice: Vec<(ffi::duckdb_connection, Arc<DbState>, String)> = {
        let kept = lock(&KEPT);
        let len = kept.len();
        if len == 0 {
            return 0;
        }
        let start = REAP_CURSOR.fetch_add(REAP_BUDGET, Ordering::AcqRel);
        (0..REAP_BUDGET.min(len))
            .map(|offset| {
                let entry = &kept[(start + offset) % len];
                entry.state.retired.store(true, Ordering::Release);
                (
                    entry.connection,
                    entry.state.clone(),
                    entry.probe.clone().unwrap_or_default(),
                )
            })
            .collect()
    };
    for (connection, state, probe) in slice {
        let alone = match state.gate.try_lock() {
            Ok(_held) => count_connections(connection)
                .map(|count| count <= 1)
                .unwrap_or(false),
            // A scan is using the connection: it holds a guard, so the
            // release waits for that guard's drop, which pokes the reaper.
            Err(_) => false,
        };
        if !alone {
            // Caller connections remain, or the count refused: keep the
            // database. A later pass looks again.
            state.retired.store(false, Ordering::Release);
            continue;
        }
        if state.users.load(Ordering::Acquire) > 0 {
            // Alone but guarded: a prepared statement still holds this
            // database. Stay retired; the last guard's drop pokes the
            // reaper and a pass finishes this path.
            continue;
        }
        detach_probe(connection, &probe);
        let mut connection = connection;
        unsafe { ffi::duckdb_disconnect(&mut connection) };
        lock(&KEPT).retain(|entry| !Arc::ptr_eq(&entry.state, &state));
        released += 1;
    }
    released
}

/// Best-effort detach of the identity probe before the disconnect.
fn detach_probe(connection: ffi::duckdb_connection, probe: &str) {
    if probe.is_empty() {
        return;
    }
    let sql = format!("DETACH {probe}");
    if let Ok(sql_c) = CString::new(sql) {
        let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
        unsafe { ffi::duckdb_query(connection, sql_c.as_ptr(), &mut result) };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
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
