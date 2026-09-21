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

use std::collections::HashMap;
use std::ffi::{c_char, c_int, CStr, CString};
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use rusqlite::functions::{Aggregate, Context, FunctionFlags};
use rusqlite::vtab::{
    Context as VtabContext, Filters, IndexConstraintOp, IndexInfo, Module, VTab,
    VTabConfig, VTabConnection, VTabCursor,
};
use rusqlite::{Connection, Error, ffi};
use thinkthen_contract::{
    Annotated, Cancel, Edge, Engine, Entity, ErrorKind, Kind, MAX_RELATE_RECORDS,
    Options, Question, QuestionSet, Recognize, Relate,
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
        db.config(VTabConfig::Innocuous)?;
        Ok((
            std::borrow::Cow::Borrowed(
                c"CREATE TABLE x(text,kind,start,end,strength,body hidden,kinds hidden)",
            ),
            RecognizeTab { base: ffi::sqlite3_vtab::default() },
        ))
    }

    fn best_index(&self, info: &mut IndexInfo) -> rusqlite::Result<bool> {
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
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        self.row += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.row >= self.rows.len()
    }

    fn column(&self, ctx: &mut VtabContext, i: c_int) -> rusqlite::Result<()> {
        let entity = &self.rows[self.row];
        match i {
            RECOGNIZE_TEXT => ctx.set_result(&entity.text),
            RECOGNIZE_KIND => ctx.set_result(&entity.kind),
            RECOGNIZE_START => ctx.set_result(&(entity.start as i64)),
            RECOGNIZE_END => ctx.set_result(&(entity.end as i64)),
            RECOGNIZE_STRENGTH => ctx.set_result(&entity.strength),
            _ => Ok(()),
        }
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
        db.config(VTabConfig::Innocuous)?;
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
        let mut ask = Relate::new();
        for column in RELATE_R1..=RELATE_R4 {
            if let Some(name) = get(column) {
                if !name.is_empty() {
                    ask = ask.relation(name, Kind::Any, Kind::Any).map_err(failure)?;
                }
            }
        }
        let (ids, texts) = read_records(self.db, table, id_col, body_col).map_err(failure)?;
        let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
        let edges: Vec<Edge> = engine()
            .relate_opts(&ask, &refs, Options::new())
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
    }

    fn next(&mut self) -> rusqlite::Result<()> {
        self.row += 1;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.row >= self.rows.len()
    }

    fn column(&self, ctx: &mut VtabContext, i: c_int) -> rusqlite::Result<()> {
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
    }

    fn rowid(&self) -> rusqlite::Result<i64> {
        Ok(self.row as i64 + 1)
    }
}

/// The two table-valued module registrations.
const RECOGNIZE_MODULE: Module<'static, RecognizeTab> =
    Module::eponymous_only_module();
const RELATE_MODULE: Module<'static, RelateTab> = Module::eponymous_only_module();

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
    connection.create_aggregate_function("thinkthen_warm", 2, plain, Warm)?;
    connection.create_module(c"thinkthen_recognize", &RECOGNIZE_MODULE, None)?;
    connection.create_module(c"thinkthen_relate", &RELATE_MODULE, None)?;
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
