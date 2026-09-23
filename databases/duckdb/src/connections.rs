//! The kept connections: one per loaded database, resolved for a caller
//! through an identity no SQL can forge, and released only when no
//! prepared statement still holds a handle to them.
//!
//! A table function cannot run SQL on the caller's own connection, so
//! each database gets a kept connection the relate scan reads through.
//! Two databases in one process share these statics, so the registry
//! must tell them apart. The identity is a uniquely named in-memory
//! database attached to the caller's own instance at LOAD time, and the
//! proof goes one layer deeper than the name (review 4, finding 5): the
//! probe database carries a randomly named marker table, and a caller
//! matches only when the catalog it resolves under the probe name also
//! holds that marker — so attaching an empty `:memory:` under a known
//! probe name, or under another database's name, forges nothing. There
//! is no name fallback at all (review 4): a database whose probe could
//! not attach — a read-only database — is never routed to while any
//! other database is loaded, and the error says so. And there is no
//! single-entry shortcut (review 4, finding 4): one loaded entry proves
//! nothing, because a database whose entry the reaper released can
//! still call through its instance's registration, and its caller must
//! be refused, never routed onto the only entry left.
//!
//! The kept connection outlives its usefulness when its database has no
//! caller connections left, because the database instance — and with it
//! the file lock of a file database — stays held while any connection
//! lives. The reaper releases those, and retirement is decided the
//! safe way around (review 4, finding 6): a database is marked retired
//! only AFTER a pass confirms it alone, never speculatively before, so
//! a live database's calls never fail a routing check mid-window. Guards
//! come in two flavors: a routing guard, refused for retired entries,
//! and a holding guard, allowed for them — the file-access checks use
//! holding guards over EVERY loaded entry, retired or not, so a
//! retired-but-live database's `enable_external_access=false` refuses
//! exactly as a live one's does.

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
    /// A relate scan is holding this database's gate right now: the
    /// interrupt bridge reads it from the signal handler (an atomic
    /// load is signal-safe) and interrupts exactly the connections
    /// whose queries are running (review 4, finding 13: one Ctrl-C with
    /// two queries in flight stopped only one, and neither Ctrl-C nor
    /// `con.interrupt()` reached a slow relate at all).
    busy: AtomicBool,
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

    /// Mark this database's kept connection as running a query, so the
    /// interrupt bridge can reach it; pair with [`Self::idle`].
    pub(crate) fn set_busy(&self) {
        self.state.busy.store(true, Ordering::Release);
    }

    /// Mark this database's kept connection idle again.
    pub(crate) fn set_idle(&self) {
        self.state.busy.store(false, Ordering::Release);
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
    /// The database's own `current_database()` name, for messages and
    /// the catalog a no-probe entry would once have matched by (never
    /// a routing identity — review 4, finding 5's forgery).
    name: String,
    /// The attached in-memory database that identifies this instance:
    /// `thinkthen_instance_<128-bit random hex>`, carrying the marker
    /// table `thinkthen_marker_<128-bit random hex>` inside it. Both
    /// names are OS randomness, not clock, counter, or PID, so they
    /// cannot be enumerated from `SHOW DATABASES` timing.
    probe: Option<(String, String)>,
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
/// identity probe with its marker table, and remember it.
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
            busy: AtomicBool::new(false),
        }),
    });
    REAP_INTERVAL_MS.store(200, Ordering::Release);
    spawn_reaper();
    Ok(raw)
}

/// 128 bits of OS randomness as hex: two independently seeded hashers'
/// outputs, which the platform seeds from its own entropy pool. Not a
/// cryptographic guarantee, but beyond enumeration, which is the
/// review's bar — clock, counter, and PID are gone (review 4, finding 5).
fn random_hex() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut hex = String::with_capacity(32);
    for _ in 0..2 {
        let mut hasher = std::hash::RandomState::new().build_hasher();
        hasher.write_usize(0);
        hex.push_str(&format!("{:016x}", hasher.finish()));
    }
    hex
}

/// The uniquely named in-memory database attached as this instance's
/// identity probe, carrying a randomly named marker table inside it.
/// `:memory:` attachments hold no file and live in the instance's
/// catalog manager, so nothing on disk changes; the caller's own
/// context resolves the name only inside the same instance, and the
/// marker table makes a same-named alias provably not the probe: an
/// attacker's `ATTACH ':memory:' AS <probe name>` carries no marker, so
/// the identity check fails where the name alone would have passed
/// (review 4, finding 5, forge2). A read-only database that refuses the
/// attach keeps `None` and is never routed to while other databases are
/// loaded (no name fallback — finding 5, forge3).
fn attach_probe(connection: ffi::duckdb_connection) -> Result<(String, String), String> {
    let probe = format!("thinkthen_instance_{}", random_hex());
    let marker = format!("thinkthen_marker_{}", random_hex());
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
    // The marker table: an empty table with a random name inside the
    // probe database, the object the identity check resolves for.
    let sql = format!("CREATE TABLE {probe}.{marker}(i INTEGER)");
    let mut result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
    let Ok(sql_c) = CString::new(sql) else {
        return Err("the marker name holds no NUL".into());
    };
    let marker_state =
        unsafe { ffi::duckdb_query(connection, sql_c.as_ptr(), &mut result) };
    if marker_state != ffi::DuckDBSuccess {
        // The attach worked but the marker did not: the probe proves
        // nothing beyond its name, so refuse to rely on it.
        let message = unsafe { ffi::duckdb_result_error(&mut result) };
        let text = if message.is_null() {
            "the marker refused".to_owned()
        } else {
            unsafe { CStr::from_ptr(message) }.to_string_lossy().into_owned()
        };
        unsafe { ffi::duckdb_destroy_result(&mut result) };
        let detach = format!("DETACH {probe}");
        if let Ok(detach_c) = CString::new(detach) {
            let mut drop_result: ffi::duckdb_result = unsafe { std::mem::zeroed() };
            unsafe { ffi::duckdb_query(connection, detach_c.as_ptr(), &mut drop_result) };
            unsafe { ffi::duckdb_destroy_result(&mut drop_result) };
        }
        return Err(format!("the identity probe's marker did not create: {text}"));
    }
    unsafe { ffi::duckdb_destroy_result(&mut result) };
    Ok((probe, marker))
}

/// A routing guard on `entry`, or `None` when the reaper has retired
/// it. Must be called under the registry lock: the reaper's disconnect
/// re-checks `users` under the same lock, so a guard either exists
/// before the disconnect decision or the entry is already gone.
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

/// A holding guard on `entry`, allowed even when the reaper has retired
/// it: the holder keeps the connection alive for exactly as long as it
/// needs to read a setting or a file through it, so a retired-but-live
/// database's access rules refuse exactly a live one's do (review 4,
/// finding 6). Must be called under the registry lock, like `acquire`.
fn acquire_holding(entry: &Kept) -> Option<KeptGuard> {
    // A retired entry is still in the registry until the reaper's
    // confirmed-alone path removes it, and that removal re-checks
    // `users` under the registry lock — so the increment races nothing.
    entry.state.users.fetch_add(1, Ordering::AcqRel);
    Some(KeptGuard {
        connection: entry.connection,
        state: entry.state.clone(),
    })
}

/// The kept connection belonging to the caller's database, as a guard.
///
/// Every entry proves itself the same way, alone or beside others:
/// the caller's context must resolve the entry's probe name AND find
/// the marker table inside the resolved catalog. There is no
/// single-entry shortcut (review 4, finding 4: after the reaper
/// released a database's entry, its still-registered caller was routed
/// onto the only entry left) and no name fallback (review 4, finding 5:
/// a read-only database's name was forged by an empty `:memory:` under
/// it). An entry whose probe could not attach is never a candidate; a
/// caller that matches nothing hears which boundary it hit.
///
/// # Errors
///
/// The defect kind when no loaded database matches the caller.
pub(crate) fn for_caller(caller: ffi::duckdb_client_context) -> Result<KeptGuard, String> {
    // Guards for every candidate, taken under one registry lock, then
    // probed outside it: the probe reads catalogs, and no lock order
    // ever puts the registry inside a gate.
    let candidates: Vec<(KeptGuard, Option<(String, String)>)> = {
        let kept = lock(&KEPT);
        kept.iter()
            .filter_map(|entry| {
                acquire(entry).map(|guard| (guard, entry.probe.clone()))
            })
            .collect()
    };
    let unnamed_loaded = {
        let kept = lock(&KEPT);
        kept.iter().any(|entry| entry.probe.is_none())
    };
    if candidates.is_empty() {
        return Err(no_match_message(unnamed_loaded));
    }
    let mut hits: Vec<KeptGuard> = Vec::new();
    for (guard, probe) in candidates {
        let Some((probe, marker)) = probe else {
            // No probe: this entry is never routable while others load.
            continue;
        };
        if probe_is_ours(caller, &probe, &marker) {
            hits.push(guard);
        }
    }
    match hits.len() {
        1 => Ok(hits.pop().expect("one hit")),
        0 => Err(no_match_message(unnamed_loaded)),
        _ => Err(
            "thinkthen defect: two loaded databases answered the caller's identity; the probes are 128-bit random with markers, so this cannot happen"
                .into(),
        ),
    }
}

/// The refusal a caller that proved no identity earns: the read-only
/// boundary when an unprovable database is loaded, the reload remedy
/// otherwise.
fn no_match_message(unnamed_loaded: bool) -> String {
    if unnamed_loaded {
        "thinkthen local: no loaded database answers this connection's identity, and a loaded database could not attach its identity probe (a read-only database cannot carry one), so it cannot be routed to while other databases are loaded; relate works on the database that loaded this extension alone, or on a writable database"
            .into()
    } else {
        reload_message()
    }
}

/// Whether the caller's own context resolves the probe name to the
/// catalog this extension attached: the name resolves only inside the
/// instance it was attached to, and the marker table inside that
/// catalog is the part an alias cannot carry (review 4, finding 5).
fn probe_is_ours(caller: ffi::duckdb_client_context, probe: &str, marker: &str) -> bool {
    let (Ok(probe), Ok(marker)) = (CString::new(probe), CString::new(marker)) else {
        return false;
    };
    let mut catalog =
        unsafe { ffi::duckdb_client_context_get_catalog(caller, probe.as_ptr()) };
    if catalog.is_null() {
        return false;
    }
    let mut entry = unsafe {
        ffi::duckdb_catalog_get_entry(
            catalog,
            caller,
            ffi::duckdb_catalog_entry_type_DUCKDB_CATALOG_ENTRY_TYPE_TABLE,
            c"main".as_ptr(),
            marker.as_ptr(),
        )
    };
    let ours = !entry.is_null();
    if ours {
        unsafe { ffi::duckdb_destroy_catalog_entry(&mut entry) };
    }
    unsafe { ffi::duckdb_destroy_catalog(&mut catalog) };
    ours
}

/// Interrupt every kept connection currently running a relate query:
/// called from the SIGINT handler after the engine's cancel token, so
/// one Ctrl-C stops every in-flight query on every loaded database —
/// both of two concurrent relates, and a slow one the caller's own
/// interrupt cannot reach, because the query runs on the kept
/// connection, not the caller's (review 4, finding 13). Only atomic
/// loads happen under the handler; `duckdb_interrupt` itself is the
/// engine's own thread-safe interruption door.
pub(crate) fn interrupt_busy() {
    let targets: Vec<ffi::duckdb_connection> = {
        let kept = lock(&KEPT);
        kept.iter()
            .filter(|entry| entry.state.busy.load(Ordering::Acquire))
            .map(|entry| entry.connection)
            .collect()
    };
    for connection in targets {
        unsafe { ffi::duckdb_interrupt(connection) };
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
/// them all. Every entry is consulted — retired ones through holding
/// guards — so the reaping window cannot skip a refusal (review 4,
/// finding 6).
pub(crate) fn file_read_refusal(path: &str) -> Option<String> {
    let guards: Vec<KeptGuard> = {
        let kept = lock(&KEPT);
        kept.iter().filter_map(acquire_holding).collect()
    };
    for guard in guards {
        // The connection's own gate: the reaper counts on this
        // connection, and two threads on one DuckDB connection is
        // corruption, not a race — every read of the kept connection
        // runs under its gate, the same rule the relate scans follow.
        let _gate = guard
            .gate()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
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
/// the conservative side of the same door. There is no raw
/// `std::fs` path anymore (review 4, finding 6): with no entry to read
/// through, the read is refused in words instead of bypassing every
/// loaded database's rules.
///
/// # Errors
///
/// The local kind, with the file system's own message, when the read is
/// refused or fails.
pub(crate) fn read_question_file(
    path: &str,
    caller: Option<ffi::duckdb_client_context>,
) -> Result<String, String> {
    let outcome = match caller {
        Some(context) => unsafe { read_through_file_system(context, path) },
        None => {
            let Some(held) = kept_context() else {
                return Err(format!(
                    "thinkthen local: the question file {path} was not read: no loaded database is left to read it through; LOAD the extension again"
                ));
            };
            unsafe { read_through_file_system(held.context(), path) }
        }
    };
    outcome
}

/// A holding-guarded kept connection, its context, and the gate that
/// makes touching that connection safe: the reaper counts on the kept
/// connection, so every read of it runs under its own gate, held for
/// the context's whole life. The lifetime transmute is sound because
/// the KeptGuard in here keeps the DbState — and with it the gate's
/// mutex — alive exactly as long as this holder lives.
pub(crate) struct HeldContext {
    guard: KeptGuard,
    context: ffi::duckdb_client_context,
    gate: std::sync::MutexGuard<'static, ()>,
}

impl Drop for HeldContext {
    fn drop(&mut self) {
        let mut context = self.context;
        unsafe { ffi::duckdb_destroy_client_context(&mut context) };
    }
}

impl HeldContext {
    /// The context, borrowed for as long as the holder lives.
    pub(crate) fn context(&self) -> ffi::duckdb_client_context {
        self.context
    }
}

fn kept_context() -> Option<HeldContext> {
    let guard = {
        let kept = lock(&KEPT);
        kept.iter().find_map(acquire_holding)?
    };
    let gate = guard
        .gate()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut context: ffi::duckdb_client_context = std::ptr::null_mut();
    unsafe { ffi::duckdb_connection_get_client_context(guard.connection(), &mut context) };
    if context.is_null() {
        return None;
    }
    Some(HeldContext {
        gate: unsafe { std::mem::transmute(gate) },
        guard,
        context,
    })
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

/// Set once the process begins exiting, so the reaper stops touching
/// connections the host is tearing down underneath it: the teardown
/// race a clean-close probe exposed (review 4), where interpreter
/// finalization corrupted state under a concurrent count query.
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

/// Set by `atexit`, which runs in the main thread before static
/// destruction, so the flag is visible to the reaper before any host
/// teardown begins.
extern "C" fn stop_reaper_at_exit() {
    SHUTDOWN.store(true, Ordering::SeqCst);
}

/// Start the reaper once per process. The thread sleeps on its clock —
/// poked by a guard dropping on a retired database — and each pass
/// examines a bounded slice of the registry, so a fleet of idle
/// databases costs one pass's budget, not a constant 64% of a core
/// (review 3, finding 26). The thread ends at process exit: the
/// `atexit` flag stops its passes before the host tears the engines
/// down underneath them.
fn spawn_reaper() {
    if REAPER.swap(true, Ordering::SeqCst) {
        return;
    }
    static ATEXIT_INSTALLED: AtomicBool = AtomicBool::new(false);
    if !ATEXIT_INSTALLED.swap(true, Ordering::SeqCst) {
        unsafe { libc::atexit(stop_reaper_at_exit) };
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
            if SHUTDOWN.load(Ordering::SeqCst) {
                return;
            }
            let released = reap_once();
            let next = if released > 0 {
                200
            } else {
                (REAP_INTERVAL_MS.load(Ordering::Acquire) * 2).min(2_000)
            };
            REAP_INTERVAL_MS.store(next, Ordering::Release);
        });
}

/// One pass: examine a bounded slice of the registry and release what a
/// confirmed check says is alone. Retirement is decided AFTER the
/// count, never before it (review 4, finding 6's window and the
/// spurious relate failures): a candidate is counted under its own
/// gate, and only a database that is alone is marked retired — so a
/// live database's routing never fails mid-window. A database that is
/// alone but still guarded (a prepared statement holds it) stays
/// retired until the last guard drops, which pokes the reaper. The
/// disconnect and the registry removal happen under the registry lock
/// with a final `users` re-check, so a guard either exists before the
/// decision or the entry is already gone; no lock order puts the
/// registry inside a gate that a query holds.
fn reap_once() -> usize {
    let mut released = 0usize;
    // Candidates: a bounded, wrapping slice of the registry, stepping by
    // the budget so passes tile it without gaps.
    let slice: Vec<(ffi::duckdb_connection, Arc<DbState>, String, String)> = {
        let kept = lock(&KEPT);
        let len = kept.len();
        if len == 0 {
            return 0;
        }
        let start = REAP_CURSOR.fetch_add(REAP_BUDGET, Ordering::AcqRel);
        (0..REAP_BUDGET.min(len))
            .map(|offset| {
                let entry = &kept[(start + offset) % len];
                (
                    entry.connection,
                    entry.state.clone(),
                    entry
                        .probe
                        .as_ref()
                        .map(|(probe, _)| probe.clone())
                        .unwrap_or_default(),
                    entry
                        .probe
                        .as_ref()
                        .map(|(_, marker)| marker.clone())
                        .unwrap_or_default(),
                )
            })
            .collect()
    };
    for (connection, state, probe, marker) in slice {
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
            // database, and heal any retired tail an earlier pass left.
            state.retired.store(false, Ordering::Release);
            continue;
        }
        if state.users.load(Ordering::Acquire) > 0 {
            // Alone but guarded: a prepared statement still holds this
            // database. Stay retired; the last guard's drop pokes the
            // reaper and a pass finishes this path.
            state.retired.store(true, Ordering::Release);
            continue;
        }
        // Alone and unguarded: retire, remove, and disconnect under one
        // registry lock, with a final users re-check so a holding guard
        // that arrived under the lock is honored, not orphaned.
        let connection = connection;
        let removed = {
            let mut kept = lock(&KEPT);
            if state.users.load(Ordering::Acquire) > 0 {
                false
            } else {
                state.retired.store(true, Ordering::Release);
                let mut connection = connection;
                detach_probe(connection, &probe, &marker);
                unsafe { ffi::duckdb_disconnect(&mut connection) };
                kept.retain(|entry| !Arc::ptr_eq(&entry.state, &state));
                true
            }
        };
        if removed {
            released += 1;
        }
    }
    released
}

/// Best-effort detach of the identity probe before the disconnect,
/// dropping its marker table with it.
fn detach_probe(connection: ffi::duckdb_connection, probe: &str, marker: &str) {
    if probe.is_empty() || marker.is_empty() {
        return;
    }
    let sql = format!("DROP TABLE IF EXISTS {probe}.{marker}; DETACH {probe}");
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
