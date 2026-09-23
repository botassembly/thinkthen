//! The SQLite surface of thinkthen: ten SQL functions over one engine.
//!
//! The ruled names carry the `thinkthen_` prefix, `NULL` is "not sure", and
//! a question is a plain string, a JSON question, or a question file named
//! with the command's spelling `'@refund.json'`. `thinkthen_warm` is the
//! bulk spine: an aggregate that judges a table in one pass at the process
//! width and saves every answer, so the queries that follow read the saved
//! answers row by row at no further cost. In SQL, `filter` is the
//! `WHERE thinkthen_decide` pattern the slide draws, and ordering answers
//! ride in `thinkthen_details`; the container verbs' aggregate forms are
//! the database ADR's to rule. `thinkthen_recognize` and
//! `thinkthen_relate` are table-valued functions: `recognize` answers one
//! text with one row per name, and `relate` reads a whole table at once
//! and answers one row per edge, because it needs every record together.
//!
//! Nothing here sends, retries, or schedules: the engine behind the
//! [`Engine`] trait owns all of that. Load-time init registers the
//! functions and touches no wire; the engine is built lazily on the first
//! call, and a fork is repaired by the engine's process check.
//!
//! Every function is volatile and direct-only: no paid call is legal from
//! a view, a trigger, a default, an index expression, or a CHECK
//! constraint, so the schema of a database the host has not vouched for
//! cannot spend money or read files. That promise needs SQLite 3.50.0 or
//! newer: below 3.50.0 a CHECK constraint in an untrusted database reaches
//! the functions (SQLite marks a call node as from-DDL only in the
//! deterministic branch of its resolver, so a volatile function skips the
//! mark and `SQLITE_DIRECTONLY` is not enforced; 3.50.0 moved the mark into
//! the DIRECT/UNSAFE branch). The load-time floor check refuses an older
//! host by name, and `tests/schema_refusal.py` proves every schema object
//! refuses at the floor. The interrupt poll reads the calling connection's
//! own handle from SQLite's own context, never a process-wide one, so two
//! connections can open, close, and interrupt independently.

use std::collections::HashMap;
use std::ffi::{c_char, c_int, CStr, CString};
use std::path::Path;
use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, Once, OnceLock};

use rusqlite::functions::{Aggregate, Context, FunctionFlags};
use rusqlite::vtab::{
    Context as VtabContext, Filters, IndexConstraintOp, IndexInfo, Module, VTab,
    VTabConfig, VTabConnection, VTabCursor,
};
use rusqlite::{Connection, Error, ffi};
use thinkthen_contract::{
    Annotated, Cancel, Connector as _, Edge, Engine, EngineConfig, Entity,
    ErrorKind, MAX_RELATE_RECORDS, Options, Question, QuestionSet, Recognize,
    Relate,
};
use thinkthen_standin::StandinConnector;

/// The host's `sqlite3_api_routines` table, extended past the 3.34.1
/// bindings the routed headers stop at, to the `is_interrupted` field
/// SQLite 3.41 added (the floor sits at 3.50.0, so the field is always
/// there).
///
/// The tail lists every field the host header adds after `txn_state` and
/// before `is_interrupted`, in the header's order (sqlite3ext.h, the 3.35
/// through 3.41 groups). Every one of them is a function pointer, so an
/// opaque pointer keeps each offset exact; only `is_interrupted` is ever
/// called, and only after the floor check names a host that carries it.
#[repr(C)]
struct ApiRoutines {
    base: ffi::sqlite3_api_routines,
    changes64: *const (),
    total_changes64: *const (),
    autovacuum_pages: *const (),
    error_offset: *const (),
    vtab_rhs_value: *const (),
    vtab_distinct: *const (),
    vtab_in: *const (),
    vtab_in_first: *const (),
    vtab_in_next: *const (),
    deserialize: *const (),
    serialize: *const (),
    db_name: *const (),
    value_encoding: *const (),
    is_interrupted: Option<unsafe extern "C" fn(*mut ffi::sqlite3) -> c_int>,
}

/// The host's own `is_interrupted`, resolved from its API table at load
/// time. The host's copy is the only correct one: the connection handle
/// belongs to the host's SQLite, and a second copy would read foreign
/// memory. Null until a load passes the floor check.
static IS_INTERRUPTED: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

/// The host's API table pointer, held between the entry and `init` so the
/// extended tail is read only after the floor check passes.
static API_TABLE: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

/// The engine every call binds, built once, lazily, never at load time.
///
/// The stand-in's connector builds it; pointing this line at the real
/// engine's connector is the swap the merge makes.
fn engine() -> &'static Arc<dyn Engine> {
    static ENGINE: OnceLock<Arc<dyn Engine>> = OnceLock::new();
    ENGINE.get_or_init(|| {
        StandinConnector
            .connect(&EngineConfig::from_env())
            .expect("the stand-in connector has no failure path")
    })
}

/// One saved answer, keyed by the question's digest and the evidence.
///
/// Temporary, the stand-in's: the engine's own cache replaces this map at
/// the engine swap, and the map and its hit counter are deleted together
/// then (ADR 0017 puts the cache and the counters in the engine).
///
/// The map is the session memory a database holds when no cache folder is
/// named: `thinkthen_warm` fills it, and every scalar call reads it before
/// the wire. Equal pairs of question and text are judged once.
#[derive(Clone)]
enum Saved {
    /// A `decide` answer: yes, no, or unsure.
    Decision(thinkthen_contract::Answer),
    /// A `choose` winner, `None` when unresolved.
    Choice(Option<String>),
    /// A `score` answer: the position and the nearest level.
    Score(thinkthen_contract::Scored),
    /// The labels a `tag` held, in the question's order.
    Tags(Vec<String>),
    /// An `annotate` record's answer object, already serialized.
    Annotate(String),
}

/// The saved answers of this session.
fn answers() -> &'static Mutex<HashMap<(String, String), Saved>> {
    static ANSWERS: OnceLock<Mutex<HashMap<(String, String), Saved>>> =
        OnceLock::new();
    ANSWERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Answers the map served without a send. Temporary, deleted with the
/// map at the engine swap.
fn cache_hits() -> &'static AtomicU64 {
    static HITS: AtomicU64 = AtomicU64::new(0);
    &HITS
}

/// Parsed questions by their argument text, so a row-by-row query parses
/// once.
fn questions() -> &'static Mutex<HashMap<String, Arc<Question>>> {
    static QUESTIONS: OnceLock<Mutex<HashMap<String, Arc<Question>>>> =
        OnceLock::new();
    QUESTIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Parsed question sets by their argument text.
fn sets() -> &'static Mutex<HashMap<String, Arc<QuestionSet>>> {
    static SETS: OnceLock<Mutex<HashMap<String, Arc<QuestionSet>>>> =
        OnceLock::new();
    SETS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// A failure reading a named local file.
fn local_failure(text: String) -> Error {
    Error::SqliteFailure(
        ffi::Error::new(ffi::SQLITE_CANTOPEN),
        Some(format!("thinkthen local: {text}")),
    )
}

/// Map an engine failure to SQLite's own error reporting. The message
/// carries the kind and, where it matters, whether a second try could
/// help, because SQLite shows message text and nothing else.
fn failure(error: thinkthen_contract::Error) -> Error {
    let text = error.message;
    let message = match error.kind {
        ErrorKind::Cancelled => format!("thinkthen cancelled: {text}"),
        _ => format!(
            "thinkthen {}{}: {text}",
            match error.kind {
                ErrorKind::Usage => "usage",
                ErrorKind::Backend => "backend",
                ErrorKind::Local => "local",
                ErrorKind::Cancelled => "cancelled",
                ErrorKind::Deadline => "deadline",
                ErrorKind::Defect => "defect",
            },
            if error.retryable { " (retryable)" } else { "" }
        ),
    };
    let code = match error.kind {
        ErrorKind::Cancelled => ffi::SQLITE_INTERRUPT,
        ErrorKind::Usage => ffi::SQLITE_CONSTRAINT,
        ErrorKind::Local => ffi::SQLITE_CANTOPEN,
        _ => ffi::SQLITE_ERROR,
    };
    Error::SqliteFailure(ffi::Error::new(code), Some(message))
}

/// The largest file a named question file may hold, in bytes (review 3,
/// item 2): a question file is kilobytes, and the cap refuses an
/// endless read — a fifo, `/dev/zero`, anything that is not a bounded
/// regular file — before memory grows.
const FILE_CAP: u64 = 1024 * 1024;

/// Read a file named by the `'@name'` spelling, through the same rule
/// every surface reads by (review 3, item 2): a regular file of at most
/// [`FILE_CAP`] bytes, read no further than one byte past the cap, with
/// one message for every unreadable cause so a caller learns nothing
/// about the filesystem. The path is opened exactly once with
/// `O_NOFOLLOW` and `O_NONBLOCK` and the checks read the opened
/// descriptor, so a path swapped between check and open cannot smuggle
/// another file in or park the process on a fifo (review 4, item 7).
/// Where a database may read question files from is the database ADR's
/// to rule; the process directory is this surface's pick, named here so
/// a ruling can move it in one place.
fn named_file(argument: &str) -> Result<String, String> {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    let path = Path::new(&argument[1..]);
    let refused = || format!("the question file '{argument}' did not read: it must be a regular file at most {FILE_CAP} bytes");
    // Open once, never following the final component, never blocking.
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| refused())?;
    // The metadata of what was opened, not of what the path names now.
    let meta = file.metadata().map_err(|_| refused())?;
    if !meta.is_file() {
        return Err(refused());
    }
    if meta.len() > FILE_CAP {
        return Err(format!("the question file '{argument}' is over the {FILE_CAP} byte cap"));
    }
    let mut text = String::new();
    // One byte past the cap tells a file that grew after the check from
    // one that sits at the cap.
    file.take(FILE_CAP + 1)
        .read_to_string(&mut text)
        .map_err(|_| refused())?;
    if text.len() as u64 > FILE_CAP {
        return Err(format!("the question file '{argument}' is over the {FILE_CAP} byte cap"));
    }
    Ok(text)
}

/// The question one argument names: a file with the `'@'` spelling, a JSON
/// question, or plain text with the default cut.
fn question(argument: &str) -> Result<Arc<Question>, Error> {
    if let Some(held) = questions().lock().unwrap().get(argument) {
        return Ok(held.clone());
    }
    let parsed = if argument.starts_with('@') {
        let text = named_file(argument).map_err(local_failure)?;
        Question::from_json(&text).map_err(failure)?
    } else if argument.starts_with('{') {
        Question::from_json(argument).map_err(failure)?
    } else {
        Question::decide(argument).map_err(failure)?.cut(0.5).map_err(failure)?
    };
    let held = Arc::new(parsed);
    questions()
        .lock()
        .unwrap()
        .insert(argument.to_string(), held.clone());
    Ok(held)
}

/// The question set one argument names, for `annotate`.
fn set(argument: &str) -> Result<Arc<QuestionSet>, Error> {
    if let Some(held) = sets().lock().unwrap().get(argument) {
        return Ok(held.clone());
    }
    let text = if argument.starts_with('@') {
        named_file(argument).map_err(local_failure)?
    } else {
        argument.to_string()
    };
    let held = Arc::new(QuestionSet::from_json(&text).map_err(failure)?);
    sets().lock()
        .unwrap()
        .insert(argument.to_string(), held.clone());
    Ok(held)
}

/// The connection a callback is running on, read from the host's own
/// context. Per call, never a global: a closed connection can never be
/// read, and two connections cannot confuse each other.
fn connection_of(context: &Context<'_>) -> *mut ffi::sqlite3 {
    // SAFETY: the context belongs to the live call, and the handle is
    // borrowed for one read of the interrupt flag.
    match unsafe { context.get_connection() } {
        Ok(connection) => unsafe { connection.handle() },
        Err(_) => std::ptr::null_mut(),
    }
}

/// One tick of the wait, on the calling thread: hear the host's interrupt
/// on the connection this call runs on, and cancel the token. A set token
/// ends the batch with the cancelled kind; requests already sent finish.
fn hear_interrupts(db: *mut ffi::sqlite3, token: &Cancel) {
    if db.is_null() {
        return;
    }
    let check = IS_INTERRUPTED.load(Ordering::Relaxed);
    if check.is_null() {
        return;
    }
    // SAFETY: the pointer came from the host's own API table at load time
    // (the floor check guarantees the field), and `db` is the live handle
    // of the call being served.
    let check: unsafe extern "C" fn(*mut ffi::sqlite3) -> c_int =
        unsafe { std::mem::transmute(check) };
    if unsafe { check(db) } != 0 {
        token.cancel();
    }
}

/// Run one callback body — a SQL function or a virtual-table callback —
/// through the contract's shared panic boundary, turning a panic into the
/// surface's defect error instead of letting it unwind across SQLite's own
/// C frames. The boundary name and the message shape are the contract's,
/// so every surface's door spells them the same.
fn guarded<T>(what: &str, body: impl FnOnce() -> rusqlite::Result<T>) -> rusqlite::Result<T> {
    match thinkthen_contract::catch_panic(what, || Ok(body())) {
        Ok(result) => result,
        Err(error) => Err(failure(error)),
    }
}

/// How often the shared watcher reads each watched connection's
/// interrupt flag.
const WATCH_TICK: std::time::Duration = std::time::Duration::from_millis(5);

/// One live watch: the connection's address, the host's interrupt check
/// (resolved at load time), and the token the check arms.
struct Watch {
    db: usize,
    check: usize,
    token: Cancel,
    id: u64,
}

/// The process-wide watcher: one thread, started on first use, that ticks
/// every registered watch and parks while none are registered. All reads
/// of a watched connection happen under the registry lock, and a call's
/// removal takes the same lock, so the thread can never touch a
/// connection whose call already returned. Before, every uncached call
/// spawned its own thread, and a stop landing in the tick window paid the
/// full 5 ms because the loop never read the stop flag before waiting
/// (review 3, item 25).
static WATCHES: Mutex<Vec<Watch>> = Mutex::new(Vec::new());
static WAKE: Condvar = Condvar::new();
static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
static WATCH_THREAD: Once = Once::new();

fn start_watch_thread() {
    WATCH_THREAD.call_once(|| {
        std::thread::Builder::new()
            .name("thinkthen-watch".to_owned())
            .spawn(hub_thread)
            .expect("the watcher thread starts");
    });
}

/// The watcher's whole body: tick the registry under the lock, cancel the
/// tokens whose connections show an interrupt, drop those watches, then
/// wait one tick — or park entirely while the registry is empty, woken by
/// the next registration.
fn hub_thread() {
    let mut held = WATCHES.lock().unwrap();
    loop {
        if held.is_empty() {
            held = WAKE.wait(held).unwrap();
            continue;
        }
        // SAFETY: the addresses and the function pointer came from the
        // live connection and the host's API table at registration, and
        // the registry lock is held for the whole tick, so no watched
        // connection is freed mid-read.
        let mut keep = Vec::with_capacity(held.len());
        for watch in held.drain(..) {
            let check: unsafe extern "C" fn(*mut ffi::sqlite3) -> c_int =
                unsafe { std::mem::transmute(watch.check) };
            if unsafe { check(watch.db as *mut ffi::sqlite3) } != 0 {
                watch.token.cancel();
            } else {
                keep.push(watch);
            }
        }
        *held = keep;
        held = WAKE.wait_timeout(held, WATCH_TICK).unwrap().0;
    }
}

/// The host's interrupt watched while one blocking single-row call runs
/// on the calling thread.
///
/// A batch hands the engine a poll that runs on the calling thread's
/// ticks; a scalar call has no such hand, and the calling thread sits
/// inside the engine, so nothing on it can read the host flag while a send
/// or a backoff wait runs. The shared watcher reads the same flag
/// (`sqlite3_is_interrupted` is an atomic read, safe from any thread) and
/// arms the call's token; the engine's own waits then stop between
/// requests, the ruled promise: no new request starts, sent ones finish.
/// Dropping the handle removes the watch under the registry lock, so the
/// watcher never reads a connection whose call has returned.
struct InterruptWatch {
    id: Option<u64>,
}

impl InterruptWatch {
    /// Watch `db` for `token` for as long as this value lives, reading
    /// the host's own `is_interrupted` (resolved at load time). Watches
    /// nothing when the host's check or the handle is missing.
    fn start(db: *mut ffi::sqlite3, token: Cancel) -> Self {
        Self::start_with(db, token, IS_INTERRUPTED.load(Ordering::Relaxed))
    }

    /// The body of [`InterruptWatch::start`] with the host check passed
    /// in, so a unit test can drive the watcher with its own flag reader
    /// without touching the loader's one.
    fn start_with(db: *mut ffi::sqlite3, token: Cancel, check: *mut ()) -> Self {
        if db.is_null() || check.is_null() {
            return Self { id: None };
        }
        start_watch_thread();
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let mut held = WATCHES.lock().unwrap();
        held.push(Watch { db: db as usize, check: check as usize, token, id });
        drop(held);
        WAKE.notify_all();
        Self { id: Some(id) }
    }

    /// How many watches the shared watcher holds — for the tests.
    fn watched_for_test() -> usize {
        WATCHES.lock().unwrap().len()
    }
}

impl Drop for InterruptWatch {
    fn drop(&mut self) {
        let Some(id) = self.id else { return };
        let mut held = WATCHES.lock().unwrap();
        held.retain(|watch| watch.id != id);
    }
}

/// The options one single-row call carries: the per-call deadline in the
/// ruled third argument (milliseconds: `-1` none, `0` spent, a positive
/// budget, any other negative refused), converted by the contract's one
/// checked door, and the call's cancel token, armed by the interrupt
/// watcher. The two-argument spelling the deck draws carries no deadline.
fn call_options<'a>(context: &Context<'_>, token: &'a Cancel) -> Result<Options<'a>, Error> {
    let mut options = Options::new().cancel(token);
    if context.len() >= 3 {
        let millis = match context.get_raw(2) {
            rusqlite::types::ValueRef::Integer(whole) => whole as f64,
            rusqlite::types::ValueRef::Real(fraction) => fraction,
            _ => {
                return Err(failure(thinkthen_contract::Error::usage(
                    "the deadline is not a number",
                )))
            }
        };
        options = options.with_deadline_millis(Some(millis)).map_err(failure)?;
        // The engine's own guard fires a spent budget at entry; this
        // shim's session map sits in front of the engine, so the check
        // is repeated here: a spent budget refuses rather than answering
        // from the map, matching every other door.
        if options.passed() {
            return Err(failure(thinkthen_contract::Error::deadline(options.seconds())));
        }
    }
    Ok(options)
}

/// `thinkthen_decide(question, text)`: 1, 0, or `NULL` when unsure.
fn decide(context: &Context<'_>) -> Result<Option<i64>, Error> {
    guarded("thinkthen_decide", || {
        let token = Cancel::new();
        let options = call_options(context, &token)?;
        let question = question(context.get_raw(0).as_str()?)?;
        let evidence = context.get_raw(1).as_str()?;
        let key = (question.digest(), evidence.to_string());
        if let Some(Saved::Decision(answer)) = answers().lock().unwrap().get(&key) {
            cache_hits().fetch_add(1, Ordering::Relaxed);
            return Ok(answer.value().map(|held| if held { 1 } else { 0 }));
        }
        let _watch = InterruptWatch::start(connection_of(context), token.clone());
        let answer = engine()
            .decide_opts(&question, evidence, options)
            .map_err(failure)?;
        answers().lock().unwrap().insert(key, Saved::Decision(answer));
        Ok(answer.value().map(|held| if held { 1 } else { 0 }))
    })
}

/// `thinkthen_choose(question, text)`: the winning option's text, `NULL`
/// when unresolved.
fn choose(context: &Context<'_>) -> Result<Option<String>, Error> {
    guarded("thinkthen_choose", || {
        let token = Cancel::new();
        let options = call_options(context, &token)?;
        let question = question(context.get_raw(0).as_str()?)?;
        let evidence = context.get_raw(1).as_str()?;
        let key = (question.digest(), evidence.to_string());
        if let Some(Saved::Choice(choice)) = answers().lock().unwrap().get(&key) {
            cache_hits().fetch_add(1, Ordering::Relaxed);
            return Ok(choice.clone());
        }
        let _watch = InterruptWatch::start(connection_of(context), token.clone());
        let choice = engine()
            .choose_opts(&question, evidence, options)
            .map_err(failure)?;
        answers()
            .lock()
            .unwrap()
            .insert(key, Saved::Choice(choice.clone()));
        Ok(choice)
    })
}

/// `thinkthen_score(question, text)`: the specification's number from 0 to
/// K−1. The nearest level's name rides in `thinkthen_details`.
fn score(context: &Context<'_>) -> Result<Option<f64>, Error> {
    guarded("thinkthen_score", || {
        let token = Cancel::new();
        let options = call_options(context, &token)?;
        let question = question(context.get_raw(0).as_str()?)?;
        let evidence = context.get_raw(1).as_str()?;
        let key = (question.digest(), evidence.to_string());
        if let Some(Saved::Score(scored)) = answers().lock().unwrap().get(&key) {
            cache_hits().fetch_add(1, Ordering::Relaxed);
            return Ok(Some(scored.value));
        }
        let _watch = InterruptWatch::start(connection_of(context), token.clone());
        let scored = engine()
            .score_opts(&question, evidence, options)
            .map_err(failure)?;
        answers()
            .lock()
            .unwrap()
            .insert(key, Saved::Score(scored.clone()));
        Ok(Some(scored.value))
    })
}

/// `thinkthen_tag(question, text)`: the labels that held, as a JSON array
/// in the question's order.
fn tag(context: &Context<'_>) -> Result<Option<String>, Error> {
    guarded("thinkthen_tag", || {
        let token = Cancel::new();
        let options = call_options(context, &token)?;
        let question = question(context.get_raw(0).as_str()?)?;
        let evidence = context.get_raw(1).as_str()?;
        let key = (question.digest(), evidence.to_string());
        if let Some(Saved::Tags(labels)) = answers().lock().unwrap().get(&key) {
            cache_hits().fetch_add(1, Ordering::Relaxed);
            return Ok(Some(serde_json::to_string(labels).unwrap()));
        }
        let _watch = InterruptWatch::start(connection_of(context), token.clone());
        let labels = engine()
            .tag_opts(&question, evidence, options)
            .map_err(failure)?;
        let held = serde_json::to_string(&labels).unwrap();
        answers()
            .lock()
            .unwrap()
            .insert(key, Saved::Tags(labels));
        Ok(Some(held))
    })
}

/// One field of an `annotate` answer, in the conformance file's shape.
fn field_of(field: &Annotated) -> serde_json::Value {
    match field {
        Annotated::Decision(answer) => {
            serde_json::json!({ "answer": answer.value() })
        }
        Annotated::Choice(choice) => serde_json::json!({ "answer": choice }),
        Annotated::Score(scored) => serde_json::json!({
            "answer": scored.value, "nearest": scored.nearest,
        }),
        Annotated::Tags(labels) => serde_json::json!({ "answer": labels }),
        // The ruled failed marker (0054), in the same place a value
        // would sit: `{"failed":{"kind":"backend","cause":CAUSE}}`.
        Annotated::Failed(failed) => serde_json::json!({ "failed": failed }),
    }
}

/// The digest of a question set over its members, so `annotate` saves
/// under its own key.
fn set_digest(set: &QuestionSet) -> String {
    set.questions()
        .iter()
        .map(Question::digest)
        .collect::<Vec<_>>()
        .join("+")
}

/// `thinkthen_annotate(set, text)`: one object, one field per question in
/// the set's name order, each field the judgment in the conformance
/// shape.
fn annotate(context: &Context<'_>) -> Result<Option<String>, Error> {
    guarded("thinkthen_annotate", || {
        let token = Cancel::new();
        let options = call_options(context, &token)?;
        let set = set(context.get_raw(0).as_str()?)?;
        let evidence = context.get_raw(1).as_str()?;
        let key = (set_digest(&set), evidence.to_string());
        if let Some(Saved::Annotate(object)) =
            answers().lock().unwrap().get(&key)
        {
            cache_hits().fetch_add(1, Ordering::Relaxed);
            return Ok(Some(object.clone()));
        }
        let _watch = InterruptWatch::start(connection_of(context), token.clone());
        let records = engine()
            .annotate_opts(&set, &[evidence], options, None)
            .map_err(failure)?;
        let Some(record) = records.first() else {
            return Ok(None);
        };
        let object = record
            .iter()
            .map(|(name, field)| {
                format!(
                    "{}:{}",
                    serde_json::to_string(name).unwrap(),
                    serde_json::to_string(&field_of(field)).unwrap()
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let object = format!("{{{object}}}");
        answers()
            .lock()
            .unwrap()
            .insert(key, Saved::Annotate(object.clone()));
        Ok(Some(object))
    })
}

/// `thinkthen_details(question, text)`: the judgment and its audit trail,
/// as one JSON object. The `sends` field is what the bill sees.
fn details(context: &Context<'_>) -> Result<Option<String>, Error> {
    guarded("thinkthen_details", || {
        let token = Cancel::new();
        let options = call_options(context, &token)?;
        let question = question(context.get_raw(0).as_str()?)?;
        let evidence = context.get_raw(1).as_str()?;
        let _watch = InterruptWatch::start(connection_of(context), token.clone());
        let audit = engine()
            .details_opts(&question, evidence, options)
            .map_err(failure)?;
        let object = serde_json::json!({
            "probability": audit.probability,
            "answer": audit.answer.value(),
            "nearest": audit.nearest,
            "model": audit.model,
            "digest": audit.digest,
            "sends": audit.sends,
            "requests": audit.requests,
            "failed_questions": audit.failed_questions,
        });
        Ok(Some(
            serde_json::to_string(&object)
                .map_err(|failure| local_failure(failure.to_string()))?,
        ))
    })
}

/// `thinkthen_usage()`: the process counters as one JSON object — sends,
/// answers the session map served with no send, and the tokens the
/// replies reported. The counters are cumulative and are never reset;
/// a caller takes two snapshots and subtracts them. The removed reset
/// spelling refuses and names that substitution.
fn usage(context: &Context<'_>) -> Result<String, Error> {
    guarded("thinkthen_usage", || {
        if context.len() != 0 {
            let argument = context.get_raw(0).as_str()?;
            let message = if argument == "reset" {
                "the reset spelling is removed; the counters are cumulative, so take two snapshots and subtract them"
            } else {
                "thinkthen_usage takes no arguments; the counters are cumulative, so subtract two snapshots"
            };
            return Err(failure(thinkthen_contract::Error::usage(message)));
        }
        let held = engine().usage();
        let object = serde_json::json!({
            "requests": held.requests,
            "cache_answers": cache_hits().load(Ordering::Relaxed),
            "tokens": held.tokens,
        });
        Ok(serde_json::to_string(&object)
            .map_err(|failure| local_failure(failure.to_string()))?)
    })
}

/// The `thinkthen_warm` accumulator: the texts one flush holds, under the
/// question that was live when they arrived. A different question closes
/// the group it held — before, the first question judged every later
/// row's text, and a pair absent from the cache was served from it
/// (review 3, item 17).
struct WarmState {
    question: Option<Arc<Question>>,
    digest: Option<String>,
    pending: Vec<String>,
    judged: u64,
}

/// The `thinkthen_warm` aggregate: judge every row in one pass at the
/// process width, saving each answer. Flushes every 256 rows, because the
/// chunk bounds memory and the width sets the speed. The wait hears the
/// host's interrupt on every tick, so a stopped query ends between
/// records, one in-flight round deep.
#[derive(Default)]
struct Warm;

/// The warm flush bound, from the 256-row ruling.
const CHUNK: usize = 256;

impl Aggregate<WarmState, Option<i64>> for Warm {
    fn init(&self, _: &mut Context<'_>) -> Result<WarmState, Error> {
        guarded("thinkthen_warm", || {
            Ok(WarmState { question: None, digest: None, pending: Vec::new(), judged: 0 })
        })
    }

    fn step(
        &self,
        context: &mut Context<'_>,
        state: &mut WarmState,
    ) -> Result<(), Error> {
        guarded("thinkthen_warm", || {
            let question = question(context.get_raw(0).as_str()?)?;
            let text = context.get_raw(1).as_str()?.to_string();
            if answers().lock().unwrap().contains_key(&(question.digest(), text.clone())) {
                cache_hits().fetch_add(1, Ordering::Relaxed);
                return Ok(());
            }
            if state.digest.as_deref() != Some(&question.digest()) {
                // A new question closes the group the old one held, so
                // every text is judged under the question that named it
                // (review 3, item 17).
                if !state.pending.is_empty() {
                    flush(state, connection_of(context))?;
                }
                state.digest = Some(question.digest());
                state.question = Some(question.clone());
            }
            state.pending.push(text);
            if state.pending.len() >= CHUNK {
                flush(state, connection_of(context))?;
            }
            Ok(())
        })
    }

    fn finalize(
        &self,
        context: &mut Context<'_>,
        mut state: Option<WarmState>,
    ) -> Result<Option<i64>, Error> {
        guarded("thinkthen_warm", || {
            if let Some(state) = state.as_mut() {
                if !state.pending.is_empty() {
                    flush(state, connection_of(context))?;
                }
                return Ok(Some(state.judged as i64));
            }
            Ok(None)
        })
    }
}

/// Judge the pending texts at once, saving every judgment. A set
/// interrupt cancels the token through the poll, which reads the calling
/// connection's own handle; no new request starts, the requests sent
/// finish, and the statement ends with the cancelled kind.
fn flush(state: &mut WarmState, db: *mut ffi::sqlite3) -> Result<(), Error> {
    let Some(question) = state.question.clone() else {
        return Ok(());
    };
    let token = Cancel::new();
    let records: Vec<&str> = state.pending.iter().map(String::as_str).collect();
    let mut poll = || hear_interrupts(db, &token);
    let judgments = engine()
        .decide_many_opts(&question, &records, Options::new().cancel(&token), Some(&mut poll))
        .map_err(failure)?;
    let digest = question.digest();
    let mut saved = answers().lock().unwrap();
    for (text, judgment) in state.pending.drain(..).zip(judgments) {
        saved.insert((digest.clone(), text), Saved::Decision(judgment.answer));
        state.judged += 1;
    }
    Ok(())
}

// ---------------------------------------------------------------------
// `thinkthen_recognize` and `thinkthen_relate`: the two table-valued
// functions. Each is an eponymous-only virtual table — usable straight
// from a FROM clause — with its arguments declared as hidden columns and
// bound through xBestIndex, the shape SQLite rules for a table-valued
// function. `recognize` answers one text with one row per name;
// `relate` reads a whole table (or query) and answers one row per edge,
// because it needs every record at once.
// ---------------------------------------------------------------------

/// One record's identity, as the named id column held it.
#[derive(Clone)]
enum RecordId {
    /// An INTEGER id.
    Int(i64),
    /// A TEXT id.
    Text(String),
}

/// The message SQLite holds for the connection, for an error that names
/// what the SQL did.
fn sqlite_message(db: *mut ffi::sqlite3) -> String {
    // SAFETY: errmsg returns a valid C string for a live connection.
    unsafe { CStr::from_ptr(ffi::sqlite3_errmsg(db)) }
        .to_string_lossy()
        .into_owned()
}

/// Read the text of column `i` on the current row.
fn column_text(stmt: *mut ffi::sqlite3_stmt, i: c_int) -> Option<String> {
    // SAFETY: the row is live and the column holds TEXT.
    let text = unsafe { ffi::sqlite3_column_text(stmt, i) };
    if text.is_null() {
        return None;
    }
    let len = unsafe { ffi::sqlite3_column_bytes(stmt, i) } as usize;
    // SAFETY: text points at len bytes owned by the live statement.
    let bytes = unsafe { std::slice::from_raw_parts(text, len) };
    String::from_utf8(bytes.to_vec()).ok()
}

/// Read a whole table's `(id, body)` rows for `thinkthen_relate`.
///
/// The read is a nested read-only `SELECT id, body FROM table` on the
/// same connection, materialized before any row is served because the
/// engine needs every record at once. It stops one row past the ruled
/// record limit, so a refusal costs a bounded read. Input order is the
/// order the SELECT reads the table; for a rowid table that is rowid
/// order.
fn read_records(
    db: *mut ffi::sqlite3,
    table: &str,
    id_col: &str,
    body_col: &str,
) -> Result<(Vec<RecordId>, Vec<String>), thinkthen_contract::Error> {
    use thinkthen_contract::Error as ContractError;
    if db.is_null() {
        return Err(ContractError::defect("relate ran with no database handle"));
    }
    let quote = |name: &str| format!("\"{}\"", name.replace('"', "\"\""));
    let sql = format!(
        "SELECT {}, {} FROM {}",
        quote(id_col),
        quote(body_col),
        quote(table)
    );
    let Ok(sql) = CString::new(sql) else {
        return Err(ContractError::usage(
            "the table or column name holds a NUL byte",
        ));
    };
    let mut stmt: *mut ffi::sqlite3_stmt = std::ptr::null_mut();
    // SAFETY: db is live, sql is NUL-terminated, and stmt receives the
    // new statement or stays null.
    let rc = unsafe {
        ffi::sqlite3_prepare_v2(db, sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut())
    };
    if rc != ffi::SQLITE_OK {
        return Err(ContractError::usage(format!(
            "cannot read the table or query given: {}",
            sqlite_message(db)
        )));
    }
    let mut ids: Vec<RecordId> = Vec::new();
    let mut texts: Vec<String> = Vec::new();
    let mut outcome: Result<(), thinkthen_contract::Error> = Ok(());
    loop {
        // SAFETY: stmt is live and not finalized.
        let rc = unsafe { ffi::sqlite3_step(stmt) };
        if rc == ffi::SQLITE_ROW {
            // SAFETY: the row is live; the type checks guard the reads.
            let id_type = unsafe { ffi::sqlite3_column_type(stmt, 0) };
            let id = if id_type == ffi::SQLITE_INTEGER {
                // SAFETY: the column holds INTEGER.
                RecordId::Int(unsafe { ffi::sqlite3_column_int64(stmt, 0) })
            } else if id_type == ffi::SQLITE_TEXT {
                match column_text(stmt, 0) {
                    Some(id) => RecordId::Text(id),
                    None => {
                        outcome = Err(ContractError::usage(
                            "the id column holds text that is not UTF-8",
                        ));
                        RecordId::Int(0)
                    }
                }
            } else {
                outcome = Err(ContractError::usage(
                    "the id column must hold integers or text",
                ));
                RecordId::Int(0)
            };
            // SAFETY: the row is live; the type checks guard the reads.
            let body_type = unsafe { ffi::sqlite3_column_type(stmt, 1) };
            let body = if body_type == ffi::SQLITE_TEXT {
                column_text(stmt, 1).unwrap_or_default()
            } else {
                outcome = Err(ContractError::usage(format!(
                    "record {} has no text; every record needs one string",
                    ids.len() + 1
                )));
                String::new()
            };
            ids.push(id);
            texts.push(body);
            if outcome.is_err() || ids.len() > MAX_RELATE_RECORDS {
                break;
            }
        } else if rc == ffi::SQLITE_DONE {
            break;
        } else {
            outcome = Err(ContractError::usage(format!(
                "reading the table or query failed: {}",
                sqlite_message(db)
            )));
            break;
        }
    }
    // SAFETY: stmt was prepared here and is finalized exactly once.
    unsafe { ffi::sqlite3_finalize(stmt) };
    outcome?;
    Ok((ids, texts))
}

/// `thinkthen_recognize` column numbers.
const RECOGNIZE_TEXT: c_int = 0;
const RECOGNIZE_KIND: c_int = 1;
const RECOGNIZE_START: c_int = 2;
const RECOGNIZE_END: c_int = 3;
const RECOGNIZE_STRENGTH: c_int = 4;
const RECOGNIZE_BODY: usize = 5;
const RECOGNIZE_KINDS: usize = 6;
const RECOGNIZE_COLUMNS: usize = 7;

/// The `thinkthen_recognize` virtual table: one text in, one row per name
/// out, with the five ruled columns.
#[repr(C)]
struct RecognizeTab {
    /// Base class. Must be first.
    base: ffi::sqlite3_vtab,
}

// SAFETY: the connect body registers a well-formed module; the cursor
// answers from owned rows.
unsafe impl<'vtab> VTab<'vtab> for RecognizeTab {
    type Aux = ();
    type Cursor = RecognizeCursor;

    fn connect(
        db: &mut VTabConnection,
        _aux: Option<&()>,
        _module_name: &[u8],
        _database_name: &[u8],
        _table_name: &[u8],
        _args: &[&[u8]],
    ) -> rusqlite::Result<(std::borrow::Cow<'static, CStr>, Self)> {
        // Direct-only, like the eight functions: a view or trigger inside
        // an untrusted schema cannot reach the paid call.
        db.config(VTabConfig::DirectOnly)?;
        Ok((
            std::borrow::Cow::Borrowed(
                c"CREATE TABLE x(text,kind,start,end,strength,body hidden,kinds hidden)",
            ),
            RecognizeTab { base: ffi::sqlite3_vtab::default() },
        ))
    }

    fn best_index(&self, info: &mut IndexInfo) -> rusqlite::Result<bool> {
        guarded("thinkthen_recognize", || {
            let mut present = [false; RECOGNIZE_COLUMNS];
            let mut constraint = [usize::MAX; RECOGNIZE_COLUMNS];
            for (i, held) in info.constraints().enumerate() {
                let column = held.column() as usize;
                if column < RECOGNIZE_BODY || column >= RECOGNIZE_COLUMNS {
                    continue;
                }
                if !held.is_usable()
                    || held.operator() != IndexConstraintOp::SQLITE_INDEX_CONSTRAINT_EQ
                {
                    return Ok(false);
                }
                present[column] = true;
                constraint[column] = i;
            }
            if !present[RECOGNIZE_BODY] {
                return Ok(false);
            }
            let mut mask: c_int = 0;
            let mut argv: c_int = 0;
            for column in RECOGNIZE_BODY..RECOGNIZE_COLUMNS {
                if present[column] {
                    argv += 1;
                    mask |= 1 << column;
                    let mut usage = info.constraint_usage(constraint[column]);
                    usage.set_argv_index(argv);
                    usage.set_omit(true);
                }
            }
            info.set_idx_num(mask);
            info.set_estimated_cost(50.0);
            info.set_estimated_rows(4);
            Ok(true)
        })
    }

    fn open(&'vtab mut self) -> rusqlite::Result<Self::Cursor> {
        Ok(RecognizeCursor {
            base: ffi::sqlite3_vtab_cursor::default(),
            rows: Vec::new(),
            row: 0,
        })
    }
}

/// The cursor for `thinkthen_recognize`: the entities of one text.
#[repr(C)]
struct RecognizeCursor {
    /// Base class. Must be first.
    base: ffi::sqlite3_vtab_cursor,
    rows: Vec<Entity>,
    row: usize,
}

// SAFETY: the cursor serves owned rows and does no unsafe work of its
// own beyond the module plumbing.
unsafe impl VTabCursor for RecognizeCursor {
    fn filter(
        &mut self,
        idx_num: c_int,
        _idx_str: Option<&str>,
        args: &Filters<'_>,
    ) -> rusqlite::Result<()> {
        guarded("thinkthen_recognize", || {
            let mut arg = 0;
            let body = if idx_num & (1 << RECOGNIZE_BODY) != 0 {
                let held: Option<String> = args.get(arg)?;
                arg += 1;
                held
            } else {
                None
            };
            let kinds = if idx_num & (1 << RECOGNIZE_KINDS) != 0 {
                let held: Option<String> = args.get(arg)?;
                held
            } else {
                None
            };
            let body = body.ok_or_else(|| {
                failure(thinkthen_contract::Error::usage(
                    "thinkthen_recognize: the text argument is required",
                ))
            })?;
            let kinds: Vec<String> = match kinds.as_deref().map(str::trim) {
                None | Some("") => Vec::new(),
                Some(list) => list.split(',').map(|kind| kind.trim().to_string()).collect(),
            };
            let ask = if kinds.is_empty() {
                Recognize::new()
            } else {
                Recognize::new().kinds(kinds)
            };
            let recognized = engine()
                .recognize_opts(&ask, &body, Options::new())
                .map_err(failure)?;
            self.rows = recognized.entities;
            self.row = 0;
            Ok(())
        })
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        self.row += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.row >= self.rows.len()
    }

    fn column(&self, ctx: &mut VtabContext, i: c_int) -> rusqlite::Result<()> {
        guarded("thinkthen_recognize", || {
            let entity = &self.rows[self.row];
            match i {
                RECOGNIZE_TEXT => ctx.set_result(&entity.text),
                RECOGNIZE_KIND => ctx.set_result(&entity.kind),
                RECOGNIZE_START => ctx.set_result(&(entity.start as i64)),
                RECOGNIZE_END => ctx.set_result(&(entity.end as i64)),
                RECOGNIZE_STRENGTH => ctx.set_result(&entity.strength),
                _ => Ok(()),
            }
        })
    }

    fn rowid(&self) -> rusqlite::Result<i64> {
        Ok(self.row as i64 + 1)
    }
}

/// `thinkthen_relate` column numbers.
const RELATE_NAME: c_int = 0;
const RELATE_SOURCE: c_int = 1;
const RELATE_TARGET: c_int = 2;
const RELATE_PROBABILITY: c_int = 3;
const RELATE_TABLE: usize = 4;
const RELATE_ID: usize = 5;
const RELATE_BODY: usize = 6;
const RELATE_R1: usize = 7;
const RELATE_R4: usize = 10;
const RELATE_COLUMNS: usize = 11;

/// The `thinkthen_relate` virtual table: a whole table (or query) in, one
/// row per edge out.
#[repr(C)]
struct RelateTab {
    /// Base class. Must be first.
    base: ffi::sqlite3_vtab,
    /// The connection the module connected to, for the nested read.
    db: *mut ffi::sqlite3,
}

// SAFETY: the connect body registers a well-formed module; the nested
// read is read-only and bounded.
unsafe impl<'vtab> VTab<'vtab> for RelateTab {
    type Aux = ();
    type Cursor = RelateCursor;

    fn connect(
        db: &mut VTabConnection,
        _aux: Option<&()>,
        _module_name: &[u8],
        _database_name: &[u8],
        _table_name: &[u8],
        _args: &[&[u8]],
    ) -> rusqlite::Result<(std::borrow::Cow<'static, CStr>, Self)> {
        // Direct-only, like the eight functions: a view or trigger inside
        // an untrusted schema cannot reach the paid call.
        db.config(VTabConfig::DirectOnly)?;
        // SAFETY: the handle belongs to this connection and outlives the
        // virtual table; the nested read is read-only.
        let handle = unsafe { db.handle() };
        Ok((
            std::borrow::Cow::Borrowed(
                c"CREATE TABLE x(name,source,target,probability,table_name hidden,id_column hidden,body_column hidden,relation_1 hidden,relation_2 hidden,relation_3 hidden,relation_4 hidden)",
            ),
            RelateTab { base: ffi::sqlite3_vtab::default(), db: handle },
        ))
    }

    fn best_index(&self, info: &mut IndexInfo) -> rusqlite::Result<bool> {
        guarded("thinkthen_relate", || {
            let mut present = [false; RELATE_COLUMNS];
            let mut constraint = [usize::MAX; RELATE_COLUMNS];
            for (i, held) in info.constraints().enumerate() {
                let column = held.column() as usize;
                if column < RELATE_TABLE || column >= RELATE_COLUMNS {
                    continue;
                }
                if !held.is_usable()
                    || held.operator() != IndexConstraintOp::SQLITE_INDEX_CONSTRAINT_EQ
                {
                    return Ok(false);
                }
                present[column] = true;
                constraint[column] = i;
            }
            if !present[RELATE_TABLE] || !present[RELATE_ID] || !present[RELATE_BODY] {
                return Ok(false);
            }
            let mut mask: c_int = 0;
            let mut argv: c_int = 0;
            for column in RELATE_TABLE..RELATE_COLUMNS {
                if present[column] {
                    argv += 1;
                    mask |= 1 << column;
                    let mut usage = info.constraint_usage(constraint[column]);
                    usage.set_argv_index(argv);
                    usage.set_omit(true);
                }
            }
            info.set_idx_num(mask);
            info.set_estimated_cost(200.0);
            info.set_estimated_rows(8);
            Ok(true)
        })
    }

    fn open(&'vtab mut self) -> rusqlite::Result<Self::Cursor> {
        Ok(RelateCursor {
            base: ffi::sqlite3_vtab_cursor::default(),
            db: self.db,
            rows: Vec::new(),
            row: 0,
        })
    }
}

/// One row of `thinkthen_relate`: an edge, with the ids the id column
/// held.
struct RelateRow {
    name: String,
    source: RecordId,
    target: RecordId,
    probability: f64,
}

/// The cursor for `thinkthen_relate`: the edges of one record set.
#[repr(C)]
struct RelateCursor {
    /// Base class. Must be first.
    base: ffi::sqlite3_vtab_cursor,
    db: *mut ffi::sqlite3,
    rows: Vec<RelateRow>,
    row: usize,
}

// SAFETY: the cursor serves owned rows; the nested read is read-only and
// bounded.
unsafe impl VTabCursor for RelateCursor {
    fn filter(
        &mut self,
        idx_num: c_int,
        _idx_str: Option<&str>,
        args: &Filters<'_>,
    ) -> rusqlite::Result<()> {
        guarded("thinkthen_relate", || {
            let mut values: Vec<Option<String>> = Vec::new();
            for column in RELATE_TABLE..RELATE_COLUMNS {
                if idx_num & (1 << column) != 0 {
                    values.push(args.get(values.len())?);
                }
            }
            let mut rank = [usize::MAX; RELATE_COLUMNS];
            let mut n = 0;
            for column in RELATE_TABLE..RELATE_COLUMNS {
                if idx_num & (1 << column) != 0 {
                    rank[column] = n;
                    n += 1;
                }
            }
            let get = |column: usize| -> Option<&str> {
                if rank[column] == usize::MAX {
                    None
                } else {
                    values[rank[column]].as_deref()
                }
            };
            let table = get(RELATE_TABLE).ok_or_else(|| {
                failure(thinkthen_contract::Error::usage(
                    "thinkthen_relate: the table name argument is required",
                ))
            })?;
            let id_col = get(RELATE_ID).ok_or_else(|| {
                failure(thinkthen_contract::Error::usage(
                    "thinkthen_relate: the id column argument is required",
                ))
            })?;
            let body_col = get(RELATE_BODY).ok_or_else(|| {
                failure(thinkthen_contract::Error::usage(
                    "thinkthen_relate: the body column argument is required",
                ))
            })?;
            let ask: Relate;
            let slots: Vec<&str> = (RELATE_R1..=RELATE_R4)
                .filter_map(|column| get(column))
                .filter(|value| !value.is_empty())
                .collect();
            if slots.len() == 1 && slots[0].starts_with('@') {
                // The file form: the question file's `relate` section, or the
                // whole file when it carries no section — the contract's own
                // parser reads it, so no grammar lives here.
                let text = named_file(slots[0]).map_err(local_failure)?;
                let value: serde_json::Value = serde_json::from_str(&text).map_err(|error| {
                    failure(thinkthen_contract::Error::usage(format!(
                        "the question file {} is not JSON: {error}",
                        &slots[0][1..]
                    )))
                })?;
                let section = value.get("relate").cloned().unwrap_or(value);
                ask = Relate::from_json(&section.to_string()).map_err(failure)?;
            } else if slots.len() == 1 && slots[0].starts_with('{') {
                ask = Relate::from_json(slots[0]).map_err(failure)?;
            } else {
                // The inline rule grammar, one slot per rule: `NAME` or
                // `NAME=SOURCE:TARGET`, with a leading `either:` for the
                // both-ways rule and `*` for any kind at an end. The slots
                // translate into the question file's own grammar and the
                // contract's parser reads that, so the spellings cannot drift.
                let mut relations: Vec<serde_json::Value> = Vec::new();
                let mut eithers: Vec<serde_json::Value> = Vec::new();
                for value in &slots {
                    let (either, rule) = match value.strip_prefix("either:") {
                        Some(rest) => (true, rest),
                        None => (false, *value),
                    };
                    match rule.split_once('=') {
                        Some((name, ends)) => {
                            let (source, target) = ends.split_once(':').ok_or_else(|| {
                                failure(thinkthen_contract::Error::usage(format!(
                                    "the relation rule {name} names one end; the ruled spelling is NAME=SOURCE:TARGET"
                                )))
                            })?;
                            relations.push(serde_json::json!({
                                "name": name,
                                "source": source,
                                "target": target,
                                "either": either,
                            }));
                        }
                        None if either => eithers.push(serde_json::json!(rule)),
                        None => relations.push(serde_json::json!(rule)),
                    }
                }
                let spec = serde_json::json!({ "relations": relations, "either": eithers });
                ask = Relate::from_json(&spec.to_string()).map_err(failure)?;
            }
            let (ids, texts) = read_records(self.db, table, id_col, body_col).map_err(failure)?;
            let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
            let edges: Vec<Edge> =
                thinkthen_contract::relate_checked(engine().as_ref(), &ask, &refs, Options::new())
                    .map_err(failure)?;
            let mut rows = Vec::with_capacity(edges.len());
            for edge in &edges {
                let source = edge
                    .source
                    .checked_sub(1)
                    .and_then(|i| ids.get(i as usize))
                    .cloned()
                    .ok_or_else(|| {
                        failure(thinkthen_contract::Error::defect(format!(
                            "the edge names record {}, and only {} records came in",
                            edge.source,
                            ids.len()
                        )))
                    })?;
                let target = edge
                    .target
                    .checked_sub(1)
                    .and_then(|i| ids.get(i as usize))
                    .cloned()
                    .ok_or_else(|| {
                        failure(thinkthen_contract::Error::defect(format!(
                            "the edge names record {}, and only {} records came in",
                            edge.target,
                            ids.len()
                        )))
                    })?;
                rows.push(RelateRow {
                    name: edge.name.clone(),
                    source,
                    target,
                    probability: edge.probability,
                });
            }
            self.rows = rows;
            self.row = 0;
            Ok(())
        })
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        self.row += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.row >= self.rows.len()
    }

    fn column(&self, ctx: &mut VtabContext, i: c_int) -> rusqlite::Result<()> {
        guarded("thinkthen_relate", || {
            let held = &self.rows[self.row];
            match i {
                RELATE_NAME => ctx.set_result(&held.name),
                RELATE_SOURCE => match &held.source {
                    RecordId::Int(id) => ctx.set_result(id),
                    RecordId::Text(id) => ctx.set_result(id),
                },
                RELATE_TARGET => match &held.target {
                    RecordId::Int(id) => ctx.set_result(id),
                    RecordId::Text(id) => ctx.set_result(id),
                },
                RELATE_PROBABILITY => ctx.set_result(&held.probability),
                _ => Ok(()),
            }
        })
    }

    fn rowid(&self) -> rusqlite::Result<i64> {
        Ok(self.row as i64 + 1)
    }
}

/// The two table-valued module registrations.
const RECOGNIZE_MODULE: Module<'static, RecognizeTab> =
    Module::eponymous_only_module();
const RELATE_MODULE: Module<'static, RelateTab> = Module::eponymous_only_module();

// The interrupt check resolves through the host's own API table (see
// `ApiRoutines`), never a directly linked symbol: a host that statically
// links SQLite would otherwise be read by a second copy of the library.
// The 207 experiment's trap — a host keeping its symbols hidden — cannot
// bite this shape.

/// The support floor, SQLite 3.50.0: below it, a CHECK constraint in an
/// untrusted database still reaches a DIRECTONLY function. SQLite marks a
/// function call node as from-DDL only in the deterministic branch of its
/// resolver (3.49.0 and earlier), so a volatile function — every function
/// here — is resolved inside a CHECK without the mark and
/// `SQLITE_DIRECTONLY` is not enforced; 3.50.0 moved the mark into the
/// DIRECT/UNSAFE branch (the fix recorded in NOTES.md, 2026-09-22, with
/// the version matrix). 3.50.0 also carries the `is_interrupted` field
/// 3.41 added, so the interrupt poll's host call is unaffected.
const FLOOR: c_int = 3_050_000;

/// The refusal for a host below the floor, or `None` when the host is
/// new enough. Pure so the message is testable without an old SQLite.
fn version_refusal(host: c_int) -> Option<String> {
    if host >= FLOOR {
        return None;
    }
    let major = host / 1_000_000;
    let minor = host / 1_000 % 1_000;
    let patch = host % 1_000;
    Some(format!(
        "thinkthen needs SQLite 3.50.0 or newer (below 3.50.0 a CHECK constraint in an \
         untrusted database reaches the functions, so a schema could spend money or read \
         files); this host is {major}.{minor}.{patch} ({host})"
    ))
}

/// Register the eight-function surface. Returns false: not loaded
/// permanently.
fn init(connection: Connection) -> Result<bool, Error> {
    // Refuse an old host by name at load time. Below the floor a CHECK
    // constraint in an untrusted database still reaches the functions, so
    // the surface does not load there; without the check the host would
    // load the artifact and the promise would be false.
    let host = unsafe { ffi::sqlite3_libversion_number() };
    if let Some(refusal) = version_refusal(host) {
        return Err(Error::SqliteFailure(
            ffi::Error::new(ffi::SQLITE_ERROR),
            Some(refusal),
        ));
    }
    // The floor passed, so the host's API table carries `is_interrupted`
    // at the offset the extended tail names. Resolve the host's own
    // function here; the poll never calls a second SQLite copy.
    let table = API_TABLE.load(Ordering::Relaxed) as *const ApiRoutines;
    if !table.is_null() {
        // SAFETY: the pointer is the host's own table, and the floor
        // check above proves the field exists in it.
        if let Some(check) = unsafe { (*table).is_interrupted } {
            IS_INTERRUPTED.store(check as *mut (), Ordering::Relaxed);
        }
    }
    // Volatile and direct-only: `SQLITE_DETERMINISTIC` stays off, so no
    // paid call is legal in an index expression, and `SQLITE_DIRECTONLY`
    // refuses views, triggers, defaults, and CHECK constraints whatever
    // the host's trusted_schema setting says (at the 3.50.0 floor).
    let volatile = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DIRECTONLY;
    // Each judgment scalar carries the two-argument spelling the deck
    // draws and a three-argument spelling whose last argument is the
    // per-call deadline in milliseconds (-1 none, 0 spent, positive a
    // budget). SQLite overloads by arity, so no drawn call changes.
    connection.create_scalar_function("thinkthen_decide", 2, volatile, decide)?;
    connection.create_scalar_function("thinkthen_decide", 3, volatile, decide)?;
    connection.create_scalar_function("thinkthen_choose", 2, volatile, choose)?;
    connection.create_scalar_function("thinkthen_choose", 3, volatile, choose)?;
    connection.create_scalar_function("thinkthen_score", 2, volatile, score)?;
    connection.create_scalar_function("thinkthen_score", 3, volatile, score)?;
    connection.create_scalar_function("thinkthen_tag", 2, volatile, tag)?;
    connection.create_scalar_function("thinkthen_tag", 3, volatile, tag)?;
    connection.create_scalar_function("thinkthen_annotate", 2, volatile, annotate)?;
    connection.create_scalar_function("thinkthen_annotate", 3, volatile, annotate)?;
    connection.create_scalar_function("thinkthen_details", 2, volatile, details)?;
    connection.create_scalar_function("thinkthen_details", 3, volatile, details)?;
    connection.create_scalar_function("thinkthen_usage", -1, volatile, usage)?;
    connection.create_aggregate_function("thinkthen_warm", 2, volatile, Warm)?;
    connection.create_module(c"thinkthen_recognize", &RECOGNIZE_MODULE, None)?;
    connection.create_module(c"thinkthen_relate", &RELATE_MODULE, None)?;
    Ok(false)
}

/// The extension entry SQLite derives from the artifact's basename. It
/// registers the functions and touches no wire; the engine is built
/// lazily on the first call, and a fork is repaired by the engine's
/// process check.
///
/// Unsafe to call from Rust: the three pointers are SQLite's own, and
/// only the host may hand them over (clippy's `not_unsafe_ptr_arg_deref`
/// names the same contract).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sqlite3_thinkthen_init(
    db: *mut ffi::sqlite3,
    message: *mut *mut c_char,
    api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    // Hold the host's own API table for `init`, which reads its extended
    // tail only after the floor check passes.
    API_TABLE.store(api as *mut (), Ordering::Relaxed);
    // SAFETY: this is the contract of extension_init2; init only registers.
    unsafe { Connection::extension_init2(db, message, api, init) }
}

#[cfg(test)]
mod mapping_tests {
    use super::*;

    /// Ruling 2 of the product rulings: the defect kind maps to SQLite's
    /// own error surface, with the kind named in the message. No public
    /// door carries a fault hook; this proves the mapping at the shim level.
    /// The load-time floor check: an old host is refused by name, the
    /// floor itself passes, and the refusal names the floor, the host,
    /// and the reason, so a user reads what to upgrade and why. The floor
    /// is 3.50.0 because below it a CHECK constraint in an untrusted
    /// database reaches the functions (see `FLOOR`).
    #[test]
    fn an_old_host_is_refused_by_name_and_a_new_one_passes() {
        let refusal = version_refusal(3_045_001).expect("3.45.1 is below the floor");
        assert!(refusal.contains("3.50.0"), "{refusal}");
        assert!(refusal.contains("3.45.1 (3045001)"), "{refusal}");
        assert!(refusal.contains("CHECK constraint"), "{refusal}");
        assert!(version_refusal(3_049_000).is_some(), "3.49 is below the floor");
        assert!(version_refusal(3_050_000).is_none(), "the floor itself passes");
        assert!(version_refusal(3_053_004).is_none(), "a newer host passes");
        // The live `sqlite3_libversion_number()` call lives in `init`,
        // where the loadable API is initialized; a unit test cannot call
        // the routed symbol. `tests/schema_refusal.py` exercises it on
        // both sides of the floor: the stock host refuses to load, the
        // floor host loads and refuses every schema object.
    }

    #[test]
    fn the_defect_kind_maps_to_the_engines_error() {
        match failure(thinkthen_contract::Error::defect("the plan lost its bind data")) {
            Error::SqliteFailure(inner, Some(message)) => {
                assert_eq!(inner.code, ffi::Error::new(ffi::SQLITE_ERROR).code);
                assert!(message.contains("thinkthen defect:"), "{message}");
                assert!(message.contains("the plan lost its bind data"), "{message}");
            }
            other => panic!("expected a SqliteFailure, got {other:?}"),
        }
    }
    #[test]
    fn the_api_tail_layout_holds() {
        // The extended tail appends thirteen function pointers between the
        // 3.34.1 bindings and `is_interrupted`; a bindings upgrade that
        // moved anything fails here instead of reading a wrong offset in
        // a host's table.
        assert_eq!(
            std::mem::offset_of!(ApiRoutines, is_interrupted),
            std::mem::size_of::<ffi::sqlite3_api_routines>()
                + 13 * std::mem::size_of::<*const ()>()
        );
        assert_eq!(
            std::mem::size_of::<ApiRoutines>(),
            std::mem::offset_of!(ApiRoutines, is_interrupted)
                + std::mem::size_of::<*const ()>()
        );
    }

    /// The flag the watcher test's fake host check reads.
    static WATCH_FLAG: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    /// A stand-in host check for the watcher test, so the loader's own
    /// resolved check is never touched by a test.
    unsafe extern "C" fn watch_flag(_: *mut ffi::sqlite3) -> c_int {
        WATCH_FLAG.load(Ordering::SeqCst) as c_int
    }

    /// The single-row watcher: quiet while the flag is clear, arms the
    /// token when the host's flag sets (this is how `sqlite3_interrupt`
    /// reaches an engine wait with the calling thread blocked), and stops
    /// promptly when the call ends — no per-call latency from the watch.
    #[test]
    fn the_watcher_arms_the_token_and_stops_with_the_call() {
        use std::time::{Duration, Instant};

        WATCH_FLAG.store(false, Ordering::SeqCst);
        let token = Cancel::new();
        let handle = 1_usize as *mut ffi::sqlite3;
        let watch = InterruptWatch::start_with(handle, token.clone(), watch_flag as *mut ());
        std::thread::sleep(Duration::from_millis(30));
        assert!(!token.is_cancelled(), "the token is armed only by the host flag");
        WATCH_FLAG.store(true, Ordering::SeqCst);
        let armed = Instant::now();
        while !token.is_cancelled() {
            assert!(
                armed.elapsed() < Duration::from_millis(500),
                "the watcher never armed the token"
            );
            std::thread::sleep(Duration::from_millis(2));
        }
        // The shared watcher (review 3, item 25): an interrupted watch is
        // consumed by the watcher itself, and a live one leaves the
        // registry the moment its handle drops — under the same lock the
        // tick runs in, with no second thread spawned for the call (the
        // one watcher thread serves every call).
        WATCH_FLAG.store(false, Ordering::SeqCst);
        let held = Cancel::new();
        let second = InterruptWatch::start_with(handle, held, watch_flag as *mut ());
        let grown = InterruptWatch::watched_for_test();
        assert!(grown >= 1, "the second watch registered");
        drop(second);
        assert!(
            InterruptWatch::watched_for_test() < grown,
            "the watch left the registry"
        );
        drop(watch);
    }

    #[test]
    fn a_panic_becomes_a_defect_not_an_unwind() {
        let held = guarded("thinkthen_probe", || -> rusqlite::Result<()> {
            panic!("the probe blew up")
        })
        .expect_err("a panic is a defect");
        match held {
            Error::SqliteFailure(inner, Some(message)) => {
                assert_eq!(inner.code, ffi::Error::new(ffi::SQLITE_ERROR).code);
                // The shared boundary's message: the contract owns the
                // spelling every surface's door uses.
                assert!(
                    message.contains("a panic crossed thinkthen_probe: the probe blew up"),
                    "{message}"
                );
            }
            other => panic!("expected a SqliteFailure, got {other:?}"),
        }
    }
}

#[cfg(test)]
mod cancel_tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering as AtomicOrdering};
    use std::thread;
    use std::time::{Duration, Instant};

    /// The flag the fake host interrupt check reads.
    static HOST_INTERRUPTED: AtomicBool = AtomicBool::new(false);

    /// A stand-in for the host's own `is_interrupted`, stored where the
    /// loader stores the real one, so the test exercises `hear_interrupts`
    /// itself rather than a copy of its shape.
    unsafe extern "C" fn fake_is_interrupted(_: *mut ffi::sqlite3) -> c_int {
        HOST_INTERRUPTED.load(AtomicOrdering::SeqCst) as c_int
    }

    /// The fast-backend interrupt proof (lane B item 5, the poll-bug shape)
    /// at the shape this surface wires: the poll calls `hear_interrupts`,
    /// which reads the host's own `is_interrupted` and sets the cancel
    /// token, and a fast backend never idles the engine's wait, so the
    /// poll must run on the busy arm. The handle argument is a sentinel:
    /// the fake host check ignores it, and production passes the calling
    /// connection's own handle (proven end to end in
    /// `tests/two_connections.py`). The call must return the cancelled
    /// kind within about a tick. With the busy-arm tick missing, the flag
    /// would never be read (the null wait never idles) and the 8M-record
    /// batch would run to completion (~50 s), so the bound cannot be met
    /// by a batch that finishes.
    ///
    /// End to end through SQLite cannot isolate this: on a fast backend
    /// SQLite's own step loop aborts between warm flushes (measured
    /// `interrupted` at the signal, 2026-09-21), and the stub-backed wire
    /// suite proves the slow-backend shape where our poll carries the stop.
    #[test]
    fn a_fast_backend_runs_the_poll_within_a_tick() {
        unsafe {
            std::env::set_var("ENGINE_NULL", "1");
            std::env::set_var("ENGINE_WIDTH", "1");
        }
        HOST_INTERRUPTED.store(false, AtomicOrdering::SeqCst);
        IS_INTERRUPTED.store(fake_is_interrupted as *mut (), AtomicOrdering::SeqCst);
        let engine = StandinConnector
            .connect(&EngineConfig::from_env())
            .expect("the stand-in connector builds");
        let question = Question::from_json(r#"{"decide":"Is this a complaint?"}"#)
            .expect("the question parses");
        let texts: Vec<String> = (0..1_000_000).map(|i| format!("record {i}")).collect();
        let records: Vec<&str> =
            (0..8_000_000).map(|i| texts[i % texts.len()].as_str()).collect();
        let token = Cancel::new();
        let polls = Arc::new(AtomicUsize::new(0));
        let fired_at: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
        let setter = thread::spawn(|| {
            thread::sleep(Duration::from_millis(150));
            HOST_INTERRUPTED.store(true, AtomicOrdering::SeqCst);
        });
        let handle = 1_usize as *mut ffi::sqlite3;
        let mut poll = {
            let token = token.clone();
            let polls = Arc::clone(&polls);
            let fired_at = Arc::clone(&fired_at);
            move || {
                polls.fetch_add(1, AtomicOrdering::SeqCst);
                let set_before = HOST_INTERRUPTED.load(AtomicOrdering::SeqCst);
                hear_interrupts(handle, &token);
                if set_before && token.is_cancelled() {
                    let mut held = fired_at.lock().expect("the fired stamp");
                    if held.is_none() {
                        *held = Some(Instant::now());
                    }
                }
            }
        };
        let outcome = engine.decide_many_opts(
            &question,
            &records,
            Options::new().cancel(&token),
            Some(&mut poll),
        );
        setter.join().expect("the setter joins");
        let fired = fired_at
            .lock()
            .expect("the fired stamp")
            .expect("the token fired");
        let heard = Instant::now().duration_since(fired);
        let error = outcome.expect_err("a cancelled batch returns the cancelled kind");
        assert_eq!(error.kind.to_string(), "cancelled", "{}", error.message);
        assert!(
            heard <= Duration::from_millis(1_500),
            "the interrupt waited {heard:?} past the token; a busy wait starved the poll"
        );
        let ran = polls.load(AtomicOrdering::SeqCst);
        assert!(
            ran >= 2,
            "the poll ran {ran} time(s); the busy arm never ticked"
        );
    }
}
