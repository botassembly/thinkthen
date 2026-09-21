//! The SQLite surface of thinkthen: eight SQL functions over one engine.
//!
//! The ruled names carry the `thinkthen_` prefix, `NULL` is "not sure", and
//! a question is a plain string, a JSON question, or a question file named
//! with the command's spelling `'@refund.json'`. `thinkthen_warm` is the
//! bulk spine: an aggregate that judges a table in one pass at the process
//! width and saves every answer, so the queries that follow read the saved
//! answers row by row at no further cost. In SQL, `filter` is the
//! `WHERE thinkthen_decide` pattern the slide draws, and ordering answers
//! ride in `thinkthen_details`; the container verbs' aggregate forms are
//! the database ADR's to rule.
//!
//! Nothing here sends, retries, or schedules: the engine behind the
//! [`Engine`] trait owns all of that. Load-time init registers the
//! functions and touches no wire; the engine is built lazily on the first
//! call, and a fork is repaired by the engine's process check.

use std::collections::HashMap;
use std::ffi::{c_char, c_int};
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use rusqlite::functions::{Aggregate, Context, FunctionFlags};
use rusqlite::{Connection, Error, ffi};
use thinkthen_contract::{
    Annotated, Cancel, Engine, ErrorKind, Options, Question, QuestionSet,
};
use thinkthen_standin::BlockingEngine;

/// The connection that loaded the extension, held for the interrupt poll.
/// A process loads the extension onto one connection at a time; the CLI
/// shape this surface ships for is one connection.
static DB: AtomicUsize = AtomicUsize::new(0);

/// The engine every call binds, built once, lazily, never at load time.
fn engine() -> &'static BlockingEngine {
    static ENGINE: OnceLock<BlockingEngine> = OnceLock::new();
    ENGINE.get_or_init(BlockingEngine::from_env)
}

/// One saved answer, keyed by the question's digest and the evidence.
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

/// Answers the map served without a send.
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

/// Read a file named by the `'@name'` spelling, relative to the process
/// working directory. Where a database may read question files from is the
/// database ADR's to rule; the process directory is this surface's pick,
/// named here so a ruling can move it in one place.
fn named_file(argument: &str) -> Result<String, String> {
    let path = Path::new(&argument[1..]);
    std::fs::read_to_string(path)
        .map_err(|failure| format!("cannot read question file {path:?}: {failure}"))
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

/// One tick of the wait, on the calling thread: hear the host's interrupt
/// and cancel the token. A set token ends the batch with the cancelled
/// kind; requests already sent finish.
fn hear_interrupts(token: &Cancel) {
    let db = DB.load(Ordering::Relaxed) as *mut ffi::sqlite3;
    if !db.is_null() && unsafe { sqlite3_is_interrupted(db) } != 0 {
        token.cancel();
    }
}

/// `thinkthen_decide(question, text)`: 1, 0, or `NULL` when unsure.
fn decide(context: &Context<'_>) -> Result<Option<i64>, Error> {
    let question = question(context.get_raw(0).as_str()?)?;
    let evidence = context.get_raw(1).as_str()?;
    let key = (question.digest(), evidence.to_string());
    if let Some(Saved::Decision(answer)) = answers().lock().unwrap().get(&key) {
        cache_hits().fetch_add(1, Ordering::Relaxed);
        return Ok(answer.value().map(|held| if held { 1 } else { 0 }));
    }
    let answer = engine()
        .decide_opts(&question, evidence, Options::new())
        .map_err(failure)?;
    answers().lock().unwrap().insert(key, Saved::Decision(answer));
    Ok(answer.value().map(|held| if held { 1 } else { 0 }))
}

/// `thinkthen_probe(text)`: the text back unchanged, for the
/// maintainability test.
fn probe(context: &Context<'_>) -> Result<Option<String>, Error> {
    Ok(Some(context.get_raw(0).as_str()?.to_owned()))
}

/// `thinkthen_choose(question, text)`: the winning option's text, `NULL`
/// when unresolved.
fn choose(context: &Context<'_>) -> Result<Option<String>, Error> {
    let question = question(context.get_raw(0).as_str()?)?;
    let evidence = context.get_raw(1).as_str()?;
    let key = (question.digest(), evidence.to_string());
    if let Some(Saved::Choice(choice)) = answers().lock().unwrap().get(&key) {
        cache_hits().fetch_add(1, Ordering::Relaxed);
        return Ok(choice.clone());
    }
    let choice = engine()
        .choose_opts(&question, evidence, Options::new())
        .map_err(failure)?;
    answers()
        .lock()
        .unwrap()
        .insert(key, Saved::Choice(choice.clone()));
    Ok(choice)
}

/// `thinkthen_score(question, text)`: the specification's number from 0 to
/// K−1. The nearest level's name rides in `thinkthen_details`.
fn score(context: &Context<'_>) -> Result<Option<f64>, Error> {
    let question = question(context.get_raw(0).as_str()?)?;
    let evidence = context.get_raw(1).as_str()?;
    let key = (question.digest(), evidence.to_string());
    if let Some(Saved::Score(scored)) = answers().lock().unwrap().get(&key) {
        cache_hits().fetch_add(1, Ordering::Relaxed);
        return Ok(Some(scored.value));
    }
    let scored = engine()
        .score_opts(&question, evidence, Options::new())
        .map_err(failure)?;
    answers()
        .lock()
        .unwrap()
        .insert(key, Saved::Score(scored.clone()));
    Ok(Some(scored.value))
}

/// `thinkthen_tag(question, text)`: the labels that held, as a JSON array
/// in the question's order.
fn tag(context: &Context<'_>) -> Result<Option<String>, Error> {
    let question = question(context.get_raw(0).as_str()?)?;
    let evidence = context.get_raw(1).as_str()?;
    let key = (question.digest(), evidence.to_string());
    if let Some(Saved::Tags(labels)) = answers().lock().unwrap().get(&key) {
        cache_hits().fetch_add(1, Ordering::Relaxed);
        return Ok(Some(serde_json::to_string(labels).unwrap()));
    }
    let labels = engine()
        .tag_opts(&question, evidence, Options::new())
        .map_err(failure)?;
    let held = serde_json::to_string(&labels).unwrap();
    answers()
        .lock()
        .unwrap()
        .insert(key, Saved::Tags(labels));
    Ok(Some(held))
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
    let set = set(context.get_raw(0).as_str()?)?;
    let evidence = context.get_raw(1).as_str()?;
    let key = (set_digest(&set), evidence.to_string());
    if let Some(Saved::Annotate(object)) = answers().lock().unwrap().get(&key)
    {
        cache_hits().fetch_add(1, Ordering::Relaxed);
        return Ok(Some(object.clone()));
    }
    let records = engine()
        .annotate_opts(&set, &[evidence], Options::new(), None)
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
}

/// `thinkthen_details(question, text)`: the judgment and its audit trail,
/// as one JSON object. The `sends` field is what the bill sees.
fn details(context: &Context<'_>) -> Result<Option<String>, Error> {
    let question = question(context.get_raw(0).as_str()?)?;
    let evidence = context.get_raw(1).as_str()?;
    let audit = engine()
        .details_opts(&question, evidence, Options::new())
        .map_err(failure)?;
    let object = serde_json::json!({
        "probability": audit.probability,
        "answer": audit.answer.value(),
        "model": audit.model,
        "digest": audit.digest,
        "sends": audit.sends,
    });
    Ok(Some(
        serde_json::to_string(&object)
            .map_err(|failure| local_failure(failure.to_string()))?,
    ))
}

/// `thinkthen_usage()`: the process counters as one JSON object — sends,
/// answers the session map served with no send, and the tokens the
/// replies reported. `thinkthen_usage('reset')` zeroes them, clears the
/// saved answers, and returns the zeros.
fn usage(context: &Context<'_>) -> Result<String, Error> {
    if context.len() == 1 && context.get_raw(0).as_str()? == "reset" {
        thinkthen_standin::reset_usage();
        cache_hits().store(0, Ordering::Relaxed);
        answers().lock().unwrap().clear();
    }
    let held = engine().usage();
    let object = serde_json::json!({
        "requests": held.requests,
        "cache_answers": cache_hits().load(Ordering::Relaxed),
        "tokens": held.tokens,
    });
    Ok(serde_json::to_string(&object)
        .map_err(|failure| local_failure(failure.to_string()))?)
}

/// The `thinkthen_warm` accumulator: the texts one flush holds.
struct WarmState {
    question: Option<Arc<Question>>,
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
        Ok(WarmState { question: None, pending: Vec::new(), judged: 0 })
    }

    fn step(
        &self,
        context: &mut Context<'_>,
        state: &mut WarmState,
    ) -> Result<(), Error> {
        let question = question(context.get_raw(0).as_str()?)?;
        let text = context.get_raw(1).as_str()?.to_string();
        if answers().lock().unwrap().contains_key(&(question.digest(), text.clone())) {
            cache_hits().fetch_add(1, Ordering::Relaxed);
            return Ok(());
        }
        if state.question.is_none() {
            state.question = Some(question.clone());
        }
        state.pending.push(text);
        if state.pending.len() >= CHUNK {
            flush(state)?;
        }
        Ok(())
    }

    fn finalize(
        &self,
        _: &mut Context<'_>,
        mut state: Option<WarmState>,
    ) -> Result<Option<i64>, Error> {
        if let Some(state) = state.as_mut() {
            if !state.pending.is_empty() {
                flush(state)?;
            }
            return Ok(Some(state.judged as i64));
        }
        Ok(None)
    }
}

/// Judge the pending texts at once, saving every judgment. A set
/// interrupt cancels the token through the poll; no new request starts,
/// the requests sent finish, and the statement ends with the cancelled
/// kind.
fn flush(state: &mut WarmState) -> Result<(), Error> {
    let Some(question) = state.question.clone() else {
        return Ok(());
    };
    let token = Cancel::new();
    let records: Vec<&str> = state.pending.iter().map(String::as_str).collect();
    let mut poll = || hear_interrupts(&token);
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

// rusqlite's loadable headers stop at SQLite 3.34, so this 3.41 call is
// declared against the host's own library and linked directly. The host
// process has libsqlite3 loaded, so the symbol resolves; a host that
// keeps its symbols hidden would break this, and that trap is on record
// from the 207 experiment.
#[link(name = "sqlite3")]
unsafe extern "C" {
    fn sqlite3_is_interrupted(db: *mut ffi::sqlite3) -> c_int;
}

/// Register the eight-function surface. Returns false: not loaded
/// permanently.
fn init(connection: Connection) -> Result<bool, Error> {
    let volatile = FunctionFlags::SQLITE_UTF8;
    let plain = FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC;
    connection.create_scalar_function("thinkthen_decide", 2, volatile, decide)?;
    connection.create_scalar_function("thinkthen_choose", 2, plain, choose)?;
    connection.create_scalar_function("thinkthen_score", 2, plain, score)?;
    connection.create_scalar_function("thinkthen_tag", 2, plain, tag)?;
    connection.create_scalar_function("thinkthen_annotate", 2, volatile, annotate)?;
    connection.create_scalar_function("thinkthen_details", 2, volatile, details)?;
    connection.create_scalar_function("thinkthen_usage", -1, volatile, usage)?;
    connection.create_scalar_function("thinkthen_probe", 1, plain, probe)?;
    connection.create_aggregate_function("thinkthen_warm", 2, plain, Warm)?;
    Ok(false)
}

/// The extension entry SQLite derives from the artifact's basename. It
/// registers the functions and touches no wire; the engine is built
/// lazily on the first call, and a fork is repaired by the engine's
/// process check.
#[unsafe(no_mangle)]
pub extern "C" fn sqlite3_thinkthen_init(
    db: *mut ffi::sqlite3,
    message: *mut *mut c_char,
    api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    DB.store(db as usize, Ordering::Relaxed);
    // SAFETY: this is the contract of extension_init2; init only registers.
    unsafe { Connection::extension_init2(db, message, api, init) }
}
