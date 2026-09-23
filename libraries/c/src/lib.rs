//! The C door to the thinkthen engine, version 0.0.1.
//!
//! This crate is the whole C surface: it builds `libthinkthen.so` and
//! `libthinkthen.a` against [`contract/include/thinkthen.h`] and owns every
//! exported symbol. The engine beneath exports none. The engine is built
//! through the contract's connector; the stand-in implements it today, and
//! the one line naming the connector is what the real engine changes.
//!
//! The door follows the header's lifetime rules exactly: an engine lives
//! until `thinkthen_engine_free`, a string from `thinkthen_call`,
//! `thinkthen_recognize`, or `thinkthen_relate` lives until
//! `thinkthen_free_string`, and the message from
//! `thinkthen_error_message` belongs to the calling thread: it lives until
//! that thread records its next failure, and no other thread's calls
//! replace it. Each engine carries its own failure table, so a failure on
//! one engine never hides another's, and a freed engine's failures die
//! with it.
//!
//! No panic crosses into the host. Every exported symbol runs behind the
//! contract's shared boundary (`catch_panic`), records the defect kind in
//! the engine's failure table for the calling thread, and returns a code
//! or a null the host can handle, so a bug in the door or the engine never
//! aborts the host process. The boundary holds no thread-local storage, so
//! a call after thread-local teardown — `thinkthen_engine_free` from an
//! `atexit` handler — exits clean instead of aborting. Rust's own hook
//! still prints the panic to stderr.
//!
//! Two findings from the slide, filed in `NOTES.md`, shape the code: the
//! slide passes a bare question string where the header's doc promised the
//! question-file grammar, so [`thinkthen_decide`] accepts either; and a
//! host hears its interrupts by firing a token from another thread, so the
//! door carries the token and the budget the engine already takes. The
//! design behind the option arguments, the null matrix, and the thread
//! promise is decided in `DESIGN.md`.
//!
//! Every plain spelling is exactly its `_opts` twin called with
//! `THINKTHEN_NO_DEADLINE` and a null token, and the two share one body, so
//! the equivalence cannot drift. A null engine is refused with the usage
//! code and no message, because no engine holds one.
//!
//! [`contract/include/thinkthen.h`]: ../../contract/include/thinkthen.h

use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;

use thinkthen_contract::{
    Annotated, Answer, Cancel, Connector as _, Details, Engine, EngineConfig, Error, ErrorKind,
    Options, Question, QuestionSet, Ranked, Recognize, Relate, Scored, deadline_from_millis,
    edges_json, relate_checked,
};
use thinkthen_standin::StandinConnector;

/// The header's success code.
const THINKTHEN_OK: i32 = 0;

/// The header's usage code, which a null engine also returns.
const THINKTHEN_EUSAGE: i32 = 1;

/// The header's defect code, the guard's fallback when a panic crossed
/// the door.
const THINKTHEN_EDEFECT: i32 = 6;

/// The header's `THINKTHEN_NO_DEADLINE`: -1, the one budget value that
/// sets no deadline. Every other negative value is refused with the usage
/// kind.
const NO_DEADLINE: i64 = -1;

/// The judgment the typed doors return: the outcome code and the
/// probability behind it, matching `thinkthen_answer` in the header.
/// It lives at the door, not the contract, so the contract stays free of
/// C shapes.
#[repr(C)]
#[allow(non_camel_case_types)] // the header's own name for the judgment
pub struct thinkthen_answer {
    /// `THINKTHEN_YES`, `THINKTHEN_NO`, or `THINKTHEN_UNSURE`.
    pub outcome: i32,
    /// The probability the backend gave the yes side.
    pub probability: f64,
}

/// The opaque cancel token the header names. It wraps the contract's
/// token: one flag, set from any thread and read between requests and on
/// every tick of a wait.
#[allow(non_camel_case_types)] // the header's own name for the token
pub struct thinkthen_cancel_token {
    /// The contract's token, the one every engine call reads.
    cancel: Cancel,
}

/// The opaque engine value: the engine the door calls and its own failure
/// table, one entry per thread that recorded a failure on it. A failure
/// on another engine never appears here, and the table dies with the
/// value, so a new engine at a reused address starts clean.
#[allow(non_camel_case_types)] // the header's own name for the value
pub struct thinkthen_engine {
    /// The engine the door calls, built through the contract's connector.
    engine: Arc<dyn Engine>,
    /// The last failure each thread recorded on this engine.
    failures: Mutex<HashMap<ThreadId, LastError>>,
}

/// The stored failure behind `thinkthen_error_message`.
struct LastError {
    /// The code the failing call returned.
    code: i32,
    /// Whether a second try could help.
    retryable: bool,
    /// The message, kept alive until the recording thread's next failure.
    message: CString,
}

/// The code for a kind, matching the header's six defines.
const fn code_of(kind: &ErrorKind) -> i32 {
    match kind {
        ErrorKind::Usage => 1,
        ErrorKind::Backend => 2,
        ErrorKind::Deadline => 3,
        ErrorKind::Local => 4,
        ErrorKind::Cancelled => 5,
        ErrorKind::Defect => 6,
    }
}

/// The outcome code for an answer, matching the header's three defines.
const fn outcome_of(answer: &Answer) -> i32 {
    match answer {
        Answer::Yes => 1,
        Answer::No => 0,
        Answer::Unsure => 2,
    }
}

/// A yes-or-no-or-unsure answer as JSON: `true`, `false`, or `null`.
fn json_of(answer: &Answer) -> serde_json::Value {
    match answer {
        Answer::Yes => serde_json::Value::Bool(true),
        Answer::No => serde_json::Value::Bool(false),
        Answer::Unsure => serde_json::Value::Null,
    }
}

impl thinkthen_engine {
    /// Record a failure for the calling thread in this engine's own table
    /// and return its code. The message replaces this thread's previous
    /// failure on this engine and stays alive until this thread records
    /// its next one (or the engine is freed).
    fn fail(&self, error: Error) -> i32 {
        let code = code_of(&error.kind);
        let message = CString::new(error.message).unwrap_or_else(|_| {
            CString::new("defect: the message held a NUL").unwrap_or_default()
        });
        let mut held = self
            .failures
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        held.insert(
            std::thread::current().id(),
            LastError {
                code,
                retryable: error.retryable,
                message,
            },
        );
        code
    }

    /// Run `read` over this engine's failure for the calling thread, when
    /// there is one. The lock is held only for the read: the entry belongs
    /// to this thread alone until this thread's next failure on this
    /// engine, so a value read out of it outlives the borrow.
    fn with_failure<T>(&self, read: impl FnOnce(&LastError) -> T) -> Option<T> {
        let held = self
            .failures
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        held.get(&std::thread::current().id()).map(read)
    }

    /// The code the calling thread recorded on this engine,
    /// `THINKTHEN_OK` when it recorded none.
    fn thread_error_code(&self) -> i32 {
        self.with_failure(|last| last.code).unwrap_or(THINKTHEN_OK)
    }

    /// Whether a second try could help the calling thread's last failure
    /// on this engine; zero when this thread recorded none.
    fn thread_error_retryable(&self) -> i32 {
        self.with_failure(|last| i32::from(last.retryable)).unwrap_or(0)
    }

    /// The calling thread's message for this engine, when it recorded a
    /// failure. The pointer stays valid until this thread records its next
    /// failure on this engine, exactly as the header promises.
    fn thread_error_message(&self) -> Option<*const c_char> {
        self.with_failure(|last| last.message.as_ptr())
    }

    /// One judgment over one text, in one request, carrying both the
    /// answer and the probability behind it.
    fn one_judgment(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<(Answer, f64), Error> {
        let mut judgments = self
            .engine
            .decide_many_opts(question, &[evidence], options, None)?;
        let judgment = judgments.pop().ok_or_else(|| {
            Error::defect("the engine returned no judgment for one text")
        })?;
        Ok((judgment.answer, judgment.probability))
    }
}

/// Run one door body behind the contract's shared panic boundary.
///
/// A panic from anywhere beneath the body — the engine, the contract, or
/// the door itself — is caught by [`thinkthen_contract::catch_panic`],
/// recorded as the defect kind in the engine's failure table for the
/// calling thread when an engine is at hand, and reported through
/// `fallback`, so it never unwinds into the host process. The error path
/// touches no thread-local storage, so it also works after thread-local
/// teardown (a free from an `atexit` handler). Rust's default hook still
/// prints the panic to stderr; what the guard removes is the abort that
/// would otherwise kill the host.
fn guard<T>(
    engine: *const thinkthen_engine,
    fallback: impl FnOnce() -> T,
    body: impl FnOnce() -> T,
) -> T {
    match thinkthen_contract::catch_panic("the C door", || Ok(body())) {
        Ok(value) => value,
        Err(error) => {
            if let Some(engine) = unsafe { engine.as_ref() } {
                engine.fail(error);
            }
            fallback()
        }
    }
}

/// Build a question from what the caller passed: the question-file grammar
/// when the string is a JSON object — and a parse failure there is a usage
/// error, never silently a bare question — or the bare text of a decide
/// question at the default cut otherwise. The slide passes a bare string;
/// the grammar is the ruled shape. Both work.
fn question_from(text: &str) -> Result<Question, Error> {
    let trimmed = text.trim();
    if trimmed.starts_with('{') {
        return Question::from_json(trimmed);
    }
    Question::decide(trimmed)?.cut(0.5)
}

// The door. Every function matches its declaration in
// contract/include/thinkthen.h; unsafe is the boundary, each body checks
// what it can before trusting the host, and every body runs behind the
// panic guard so no panic crosses into the host process.

/// Build an engine from the environment through the contract's connector.
/// Returns null when the connector refuses the environment or the process
/// cannot hold an engine at all.
///
/// # Panics
///
/// None that crosses the boundary: the guard catches every panic; an
/// allocation failure aborts the process by Rust's own rule.
#[unsafe(no_mangle)]
/// # Safety
///
/// The returned pointer is owned by the caller and must be freed with
/// `thinkthen_engine_free`; a null return names a refusal, never a live
/// engine.
pub unsafe extern "C" fn thinkthen_engine_new() -> *mut thinkthen_engine {
    guard(std::ptr::null(), std::ptr::null_mut, || {
        let Ok(engine) = StandinConnector.connect(&EngineConfig::from_env()) else {
            return std::ptr::null_mut();
        };
        Box::into_raw(Box::new(thinkthen_engine {
            engine,
            failures: Mutex::new(HashMap::new()),
        }))
    })
}

/// Free an engine. Null is accepted and ignored. The engine's failure
/// table dies with it, so a new engine at a reused address starts clean.
/// The error path records nothing: the value it would name is going away,
/// so the guard passes no engine.
///
/// # Safety
///
/// `engine` is null or a value this door returned and no other call is
/// using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_free(engine: *mut thinkthen_engine) {
    guard(std::ptr::null(), || {}, || {
        if !engine.is_null() {
            drop(unsafe { Box::from_raw(engine) });
        }
    })
}

/// The message for the last failure the calling thread recorded on this
/// engine, valid until that thread records its next failure; no other
/// thread's call replaces it. Never null: before any failure it names
/// that nothing failed yet, and a null engine names that no engine came.
///
/// # Safety
///
/// `engine` is null or a value this door returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_message(
    engine: *const thinkthen_engine,
) -> *const c_char {
    guard(
        engine,
        || c"the door panicked, so no message came".as_ptr(),
        || {
            let Some(held) = (unsafe { engine.as_ref() }) else {
                return c"no engine came, so no failure is named".as_ptr();
            };
            match held.thread_error_message() {
                Some(message) => message,
                None => c"no failure yet".as_ptr(),
            }
        },
    )
}

/// Whether a second try could help the calling thread's last failure on
/// this engine: 1 when it could, 0 when it could not or that thread
/// recorded none. Zero with a null engine.
///
/// # Safety
///
/// `engine` is null or a value this door returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_retryable(engine: *const thinkthen_engine) -> i32 {
    guard(engine, || 0, || {
        let Some(held) = (unsafe { engine.as_ref() }) else {
            return 0;
        };
        held.thread_error_retryable()
    })
}

/// The code of the last failure the calling thread recorded on this
/// engine: the value the failing call returned, `THINKTHEN_OK` when that
/// thread recorded none. Success does not clear it. A null engine is the
/// usage code, because no engine holds a failure.
///
/// # Safety
///
/// `engine` is null or a value this door returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_code(engine: *const thinkthen_engine) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || {
        let Some(held) = (unsafe { engine.as_ref() }) else {
            return THINKTHEN_EUSAGE;
        };
        held.thread_error_code()
    })
}

/// Create a cancel token. Free it with `thinkthen_cancel_token_free` after
/// every call that carried it has returned.
///
/// # Safety
///
/// Takes no input. The returned pointer owns one flag and borrows nothing,
/// and the host frees it exactly once.
#[unsafe(no_mangle)]
/// # Safety
///
/// The returned pointer is owned by the caller and must be freed with
/// `thinkthen_cancel_token_free`; a null return names an allocation
/// failure.
pub unsafe extern "C" fn thinkthen_cancel_token_new() -> *mut thinkthen_cancel_token {
    guard(std::ptr::null(), std::ptr::null_mut, || {
        Box::into_raw(Box::new(thinkthen_cancel_token {
            cancel: Cancel::new(),
        }))
    })
}

/// Fire a token: every call carrying it stops starting new requests and
/// returns the cancelled kind with no results. One-shot: a fire leaves the
/// token fired, a second fire is ignored, and no call re-arms it. One
/// atomic store, so any thread may call it; a null token is accepted and
/// ignored.
///
/// # Safety
///
/// `token` is null or a value this door returned and has not freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel(token: *mut thinkthen_cancel_token) {
    guard(std::ptr::null(), || {}, || {
        if let Some(token) = unsafe { token.as_ref() } {
            token.cancel.cancel();
        }
    })
}

/// Free a token. Null is accepted and ignored. Free it only after every
/// call that carried it has returned.
///
/// # Safety
///
/// `token` is null or a value this door returned and no other call is
/// using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_cancel_token_free(token: *mut thinkthen_cancel_token) {
    guard(std::ptr::null(), || {}, || {
        if !token.is_null() {
            drop(unsafe { Box::from_raw(token) });
        }
    })
}

/// The options one call carries beside its arguments: the caller's budget
/// and the caller's token. `THINKTHEN_NO_DEADLINE` (-1) sets no deadline;
/// zero is a spent budget by the engine's own rule, so it refuses before
/// anything is sent; any other negative value is refused here with the
/// usage kind; a positive value is a count of milliseconds from now,
/// converted by the contract's checked door so an oversized budget comes
/// back as the usage kind instead of a panic in the host. A null token is
/// no token.
fn control<'a>(
    deadline_ms: i64,
    token: *const thinkthen_cancel_token,
) -> Result<Options<'a>, Error> {
    let token = unsafe { token.as_ref() };
    let options = Options::new().maybe_cancel(token.map(|held| &held.cancel));
    match deadline_from_millis(deadline_ms as f64) {
        Ok(Some(budget)) => Ok(options.deadline_in(budget)),
        Ok(None) => Ok(options),
        Err(error) => Err(error),
    }
}

/// Ask one yes-or-no question of one text, with no budget and no token:
/// exactly [`thinkthen_decide_opts`] with `THINKTHEN_NO_DEADLINE` and a
/// null token.
///
/// # Safety
///
/// The same rules as [`thinkthen_decide_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide(
    engine: *const thinkthen_engine,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    out: *mut thinkthen_answer,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || unsafe {
        thinkthen_decide_opts(
            engine,
            question_json,
            text,
            text_len,
            NO_DEADLINE,
            std::ptr::null(),
            out,
        )
    })
}

/// Ask one yes-or-no question of one text, with the options beside it.
/// `question_json` is one question in the question-file grammar or the
/// bare text of a decide question; `text_len` is the evidence's byte
/// length. The judgment lands in `out` on zero; any nonzero code left
/// `out` untouched.
///
/// # Safety
///
/// `engine` is null or a value this door returned, `question_json` and
/// `text` are readable for their lengths, `cancel` is null or a live
/// token, and `out` is writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_opts(
    engine: *const thinkthen_engine,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    deadline_ms: i64,
    cancel: *const thinkthen_cancel_token,
    out: *mut thinkthen_answer,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || {
        let Some(engine) = (unsafe { engine.as_ref() }) else {
            return THINKTHEN_EUSAGE;
        };
        if out.is_null() {
            return engine.fail(Error::usage("a null out pointer"));
        }
        let options = match control(deadline_ms, cancel) {
            Ok(options) => options,
            Err(error) => return engine.fail(error),
        };
        let (answer, probability) =
            match run_decide(engine, question_json, text, text_len, options) {
                Ok(pair) => pair,
                Err(error) => return engine.fail(error),
            };
        unsafe {
            *out = thinkthen_answer {
                outcome: outcome_of(&answer),
                probability,
            };
        }
        THINKTHEN_OK
    })
}

/// The shared body of the two decide doors, safe on the Rust side.
fn run_decide(
    engine: &thinkthen_engine,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    options: Options<'_>,
) -> Result<(Answer, f64), Error> {
    let question = question_from(str_required(question_json, "question")?)?;
    let evidence = str_from(text, text_len)?;
    engine.one_judgment(&question, evidence, options)
}

/// One borrowed `&str` from a null-terminated pointer the caller passed,
/// `what` naming it in the refusal. A null pointer is a usage failure: the
/// door never guesses what a missing string meant.
///
/// # Errors
///
/// Returns the usage kind for a null pointer or bytes that are not UTF-8.
fn str_required<'a>(text: *const c_char, what: &str) -> Result<&'a str, Error> {
    if text.is_null() {
        return Err(Error::usage(format!("a null {what}")));
    }
    unsafe { CStr::from_ptr(text) }
        .to_str()
        .map_err(|_| Error::usage(format!("the {what} is not UTF-8")))
}

/// Ask the same question of every text at once, with no budget and no
/// token: exactly [`thinkthen_decide_many_opts`] with
/// `THINKTHEN_NO_DEADLINE` and a null token.
///
/// # Safety
///
/// The same rules as [`thinkthen_decide_many_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_many(
    engine: *const thinkthen_engine,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut thinkthen_answer,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || unsafe {
        thinkthen_decide_many_opts(
            engine,
            question_json,
            texts,
            lengths,
            count,
            NO_DEADLINE,
            std::ptr::null(),
            out,
        )
    })
}

/// Ask the same question of every text at once, at the engine's width,
/// keeping every judgment in input order, with the options beside it.
/// `texts` holds `count` pointers and `lengths` their byte lengths; `out`
/// holds room for `count` answers. The arrays are read, and the answers
/// written, for `count` entries only; a count of zero reads and writes
/// nothing.
///
/// # Safety
///
/// `engine` is null or a value this door returned, `question_json` is a
/// readable null-terminated string or null, the arrays are readable for
/// `count` entries, `cancel` is null or a live token, and `out` is
/// writable for `count` answers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_many_opts(
    engine: *const thinkthen_engine,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    deadline_ms: i64,
    cancel: *const thinkthen_cancel_token,
    out: *mut thinkthen_answer,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || {
        let Some(engine) = (unsafe { engine.as_ref() }) else {
            return THINKTHEN_EUSAGE;
        };
        if count > 0 {
            if out.is_null() {
                return engine.fail(Error::usage("a null out array with a nonzero count"));
            }
            if texts.is_null() {
                return engine.fail(Error::usage("a null texts array with a nonzero count"));
            }
            if lengths.is_null() {
                return engine.fail(Error::usage("a null lengths array with a nonzero count"));
            }
        }
        let options = match control(deadline_ms, cancel) {
            Ok(options) => options,
            Err(error) => return engine.fail(error),
        };
        let question = match str_required(question_json, "question").and_then(question_from) {
            Ok(question) => question,
            Err(error) => return engine.fail(error),
        };
        let mut records = Vec::with_capacity(count);
        for index in 0..count {
            let pointer = unsafe { *texts.add(index) };
            let length = unsafe { *lengths.add(index) };
            match str_from(pointer, length) {
                Ok(text) => records.push(text),
                Err(error) => return engine.fail(error),
            }
        }
        match engine
            .engine
            .decide_many_opts(&question, &records, options, None)
        {
            Ok(judgments) if judgments.len() == count => {
                for (index, judgment) in judgments.iter().enumerate() {
                    unsafe {
                        *out.add(index) = thinkthen_answer {
                            outcome: outcome_of(&judgment.answer),
                            probability: judgment.probability,
                        };
                    }
                }
                THINKTHEN_OK
            }
            Ok(judgments) => engine.fail(Error::defect(format!(
                "the engine returned {} judgments for {count} records",
                judgments.len()
            ))),
            Err(error) => engine.fail(error),
        }
    })
}

/// One borrowed `&str` from a pointer and a byte length, refusing text
/// that is not UTF-8 as a usage failure. A null pointer with a zero length
/// is the empty text; a null pointer with a nonzero length is refused.
///
/// # Errors
///
/// Returns the usage kind when the bytes are not a UTF-8 string, and for
/// a null pointer with a nonzero length.
fn str_from<'a>(text: *const c_char, len: usize) -> Result<&'a str, Error> {
    if text.is_null() {
        return if len == 0 {
            Ok("")
        } else {
            Err(Error::usage("a null text with a nonzero length"))
        };
    }
    let bytes = unsafe { std::slice::from_raw_parts(text.cast::<u8>(), len) };
    std::str::from_utf8(bytes).map_err(|_| Error::usage("a text is not UTF-8"))
}

/// The JSON door with no budget and no token: exactly
/// [`thinkthen_call_opts`] with `THINKTHEN_NO_DEADLINE` and a null token.
///
/// # Safety
///
/// The same rules as [`thinkthen_call_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call(
    engine: *const thinkthen_engine,
    request_json: *const c_char,
) -> *mut c_char {
    guard(engine, std::ptr::null_mut, || unsafe {
        thinkthen_call_opts(engine, request_json, NO_DEADLINE, std::ptr::null())
    })
}

/// The JSON door: any request of the eight verbs, with the answer as JSON
/// text the caller frees with [`thinkthen_free_string`], and with the
/// options beside it. The request is the question file's own shape with
/// the evidence beside it, and the door's reply shapes are listed in this
/// crate's `NOTES.md`. Returns null on failure, with the code on the
/// engine.
///
/// The request carries one verb:
///
/// - `{"decide": "...", "evidence": "..."}` and the same for `choose`,
///   `score`, and `tag`, with the question's own keys beside the verb's
/// - `{"decide": "...", "records": ["...", ...]}` for `filter`, with
///   `"rank": true` beside it for `rank`
/// - `{"find": "...", "units": ["...", ...]}`
/// - `{"annotate": <the question set>, "records": ["...", ...]}`
/// - `{"decide": "...", "evidence": "...", "details": true}` for the
///   audit view; any other `details` value, `false` included, is not the
///   view and the request answers as its plain verb
/// - `{"usage": true}` for the counters, which take no options
///
/// # Safety
///
/// `engine` is null or a value this door returned, `request_json` is a
/// readable null-terminated string or null, and `cancel` is null or a
/// live token.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call_opts(
    engine: *const thinkthen_engine,
    request_json: *const c_char,
    deadline_ms: i64,
    cancel: *const thinkthen_cancel_token,
) -> *mut c_char {
    guard(engine, std::ptr::null_mut, || {
        let Some(engine) = (unsafe { engine.as_ref() }) else {
            return std::ptr::null_mut();
        };
        let options = match control(deadline_ms, cancel) {
            Ok(options) => options,
            Err(error) => {
                engine.fail(error);
                return std::ptr::null_mut();
            }
        };
        let request = match str_required(request_json, "request").and_then(|text| {
            serde_json::from_str::<serde_json::Value>(text)
                .map_err(|error| Error::usage(format!("the request is not JSON: {error}")))
        }) {
            Ok(request) => request,
            Err(error) => {
                engine.fail(error);
                return std::ptr::null_mut();
            }
        };
        match call_verb(engine, &request, options) {
            Ok(reply) => {
                let Ok(text) = CString::new(reply.to_string()) else {
                    engine.fail(Error::defect("the reply held a NUL"));
                    return std::ptr::null_mut();
                };
                text.into_raw()
            }
            Err(error) => {
                engine.fail(error);
                std::ptr::null_mut()
            }
        }
    })
}

/// Route one parsed request to its verb. Every reply is a JSON value the
/// caller reads; every error is the door's six kinds. The options ride to
/// every verb that waits; the counters wait for nothing, so they ignore
/// them.
fn call_verb(
    engine: &thinkthen_engine,
    request: &serde_json::Value,
    options: Options<'_>,
) -> Result<serde_json::Value, Error> {
    let object = request
        .as_object()
        .ok_or_else(|| Error::usage("the request is not a JSON object"))?;
    if object.contains_key("usage") {
        let usage = engine.engine.usage();
        return Ok(serde_json::json!({
            "requests": usage.requests,
            "cache_answers": usage.cache_answers,
            "tokens": usage.tokens,
        }));
    }
    let evidence = |key: &str| -> Result<String, Error> {
        object
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| Error::usage(format!("the request has no {key} string")))
    };
    let records = || -> Result<Vec<String>, Error> {
        object
            .get("records")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| Error::usage("the request has no records array"))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| Error::usage("a record is not a string"))
            })
            .collect()
    };
    // The question object is the request minus the door's own keys, so the
    // caller writes the question file's shape at the top level.
    let door_keys = [
        "evidence", "records", "units", "details", "usage", "rank",
    ];
    let question_object: serde_json::Map<String, serde_json::Value> = object
        .iter()
        .filter(|(key, _)| {
            !door_keys.contains(&key.as_str()) && key.as_str() != "annotate"
        })
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();

    let ask = |evidence: &str| -> Result<(Answer, f64), Error> {
        let question_text = serde_json::to_string(&serde_json::Value::Object(
            question_object.clone(),
        ))
        .map_err(|error| Error::defect(format!("the question did not serialize: {error}")))?;
        let question = question_from(&question_text)?;
        engine.one_judgment(&question, evidence, options)
    };

    if object.get("details").and_then(serde_json::Value::as_bool) == Some(true) {
        let evidence = evidence("evidence")?;
        let question_text = serde_json::to_string(&serde_json::Value::Object(question_object))
            .map_err(|error| Error::defect(format!("the question did not serialize: {error}")))?;
        let question = question_from(&question_text)?;
        return details_json(engine, &question, &evidence, options);
    }
    if let Some(annotate) = object.get("annotate") {
        let set_text = serde_json::to_string(annotate)
            .map_err(|error| Error::defect(format!("the set did not serialize: {error}")))?;
        let set = QuestionSet::from_json(&set_text)?;
        let records = records()?;
        let references: Vec<&str> = records.iter().map(String::as_str).collect();
        let answers = engine
            .engine
            .annotate_opts(&set, &references, options, None)?;
        return Ok(serde_json::json!({ "answer": annotated_json(&answers) }));
    }
    if let Some(find) = object.get("find").and_then(serde_json::Value::as_str) {
        let units = object
            .get("units")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| Error::usage("the request has no units array"))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| Error::usage("a unit is not a string"))
            })
            .collect::<Result<Vec<&str>, Error>>()?;
        let question = Question::decide(find)?.cut(0.5)?;
        let found =
            engine.engine.find_opts(&question, &units, options)?;
        return Ok(serde_json::json!({
            "answer": found.index,
            "probability": found.probability,
        }));
    }
    if question_object.contains_key("decide") {
        if object.get("records").is_some() {
            let question_text = serde_json::to_string(&serde_json::Value::Object(question_object))
                .map_err(|error| {
                    Error::defect(format!("the question did not serialize: {error}"))
                })?;
            let question = question_from(&question_text)?;
            let records = records()?;
            let references: Vec<&str> = records.iter().map(String::as_str).collect();
            if object.contains_key("rank") {
                let ranked = engine
                    .engine
                    .rank_opts(&question, &references, options, None)?;
                return Ok(serde_json::json!({ "answer": ranked_json(&ranked) }));
            }
            let kept = engine
                .engine
                .filter_opts(&question, &references, options, None)?;
            return Ok(serde_json::json!({ "indexes": kept }));
        }
        let evidence = evidence("evidence")?;
        let (answer, probability) = ask(&evidence)?;
        return Ok(serde_json::json!({
            "answer": json_of(&answer),
            "probability": probability,
        }));
    }
    if question_object.contains_key("choose") {
        let evidence = evidence("evidence")?;
        let question = object_question(&question_object)?;
        let chosen = engine
            .engine
            .choose_opts(&question, &evidence, options)?;
        return Ok(serde_json::json!({
            "answer": chosen,
        }));
    }
    if question_object.contains_key("score") {
        let evidence = evidence("evidence")?;
        let question = object_question(&question_object)?;
        let Scored { value, nearest } =
            engine.engine.score_opts(&question, &evidence, options)?;
        return Ok(serde_json::json!({
            "answer": value,
            "nearest": nearest,
        }));
    }
    if question_object.contains_key("tag") {
        let evidence = evidence("evidence")?;
        let question = object_question(&question_object)?;
        let labels = engine.engine.tag_opts(&question, &evidence, options)?;
        return Ok(serde_json::json!({ "answer": labels }));
    }
    Err(Error::usage(
        "the request carries no verb the door knows: decide, choose, score, tag, filter, rank, find, annotate, details, usage",
    ))
}

/// Build the question from the door's collected question object.
fn object_question(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<Question, Error> {
    let text = serde_json::to_string(&serde_json::Value::Object(object.clone()))
        .map_err(|error| Error::defect(format!("the question did not serialize: {error}")))?;
    Question::from_json(&text)
}

/// The audit view as the door returns it, with the logical requests'
/// digests (0053) and the failed-question count (0054).
fn details_json(
    engine: &thinkthen_engine,
    question: &Question,
    evidence: &str,
    options: Options<'_>,
) -> Result<serde_json::Value, Error> {
    let Details {
        probability,
        answer,
        model,
        digest,
        sends,
        requests,
        failed_questions,
        nearest,
    } = engine.engine.details_opts(question, evidence, options)?;
    Ok(serde_json::json!({
        "probability": probability,
        "answer": json_of(&answer),
        "model": model,
        "digest": digest,
        "sends": sends,
        "requests": requests,
        "failed_questions": failed_questions,
        // The nearest level's name on a score question; null on every
        // other verb (ADR 0017 pick 6, settled 2026-09-21).
        "nearest": nearest,
    }))
}

/// One `rank` reply row.
fn ranked_json(ranked: &[Ranked]) -> serde_json::Value {
    serde_json::Value::Array(
        ranked
            .iter()
            .map(|row| serde_json::json!({ "index": row.index, "probability": row.probability }))
            .collect(),
    )
}

/// One `annotate` record: the set's names in order, each with its verb's
/// natural answer.
fn annotated_json(records: &[Vec<(String, Annotated)>]) -> serde_json::Value {
    serde_json::Value::Array(
        records
            .iter()
            .map(|record| {
                serde_json::Value::Object(
                    record
                        .iter()
                        .map(|(name, answer)| {
                            let value = match answer {
                                Annotated::Decision(answer) => json_of(answer),
                                Annotated::Choice(choice) => match choice {
                                    Some(label) => serde_json::Value::String(label.clone()),
                                    None => serde_json::Value::Null,
                                },
                                Annotated::Score(Scored { value, nearest }) => serde_json::json!({
                                    "answer": value,
                                    "nearest": nearest,
                                }),
                                Annotated::Tags(labels) => serde_json::json!(labels),
                                Annotated::Failed(failed) => {
                                    serde_json::json!({ "failed": failed })
                                }
                            };
                            (name.clone(), value)
                        })
                        .collect(),
                )
            })
            .collect(),
    )
}

/// Find every name in one text, and the relations the rules allow, as one
/// JSON string the caller frees with [`thinkthen_free_string`], with no
/// budget and no token: exactly [`thinkthen_recognize_opts`] with
/// `THINKTHEN_NO_DEADLINE` and a null token.
///
/// # Safety
///
/// The same rules as [`thinkthen_recognize_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_recognize(
    engine: *const thinkthen_engine,
    spec_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || unsafe {
        thinkthen_recognize_opts(
            engine,
            spec_json,
            text,
            text_len,
            NO_DEADLINE,
            std::ptr::null(),
            out,
            out_len,
        )
    })
}

/// Find every name in one text, and the relations the rules allow, as one
/// JSON string the caller frees with [`thinkthen_free_string`], with the
/// options beside it.
/// `spec_json` is the recognize section of the question file; `text` and
/// `text_len` are its bytes. In the answer, `start` and `end` count code
/// points of `text`, so a C host converts once to byte offsets before it
/// slices. The return is zero on success and the kind code on a failure,
/// with the out parameters left alone.
///
/// # Safety
///
/// `engine` is null or a value this door returned, `spec_json` is a
/// readable null-terminated string or null, `text` is readable for
/// `text_len` bytes or null with a zero length, `cancel` is null or a live
/// token, and `out` and `out_len` are writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_recognize_opts(
    engine: *const thinkthen_engine,
    spec_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    deadline_ms: i64,
    cancel: *const thinkthen_cancel_token,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || {
        let Some(engine) = (unsafe { engine.as_ref() }) else {
            return THINKTHEN_EUSAGE;
        };
        if out.is_null() {
            return engine.fail(Error::usage("a null out pointer"));
        }
        if out_len.is_null() {
            return engine.fail(Error::usage("a null out_len pointer"));
        }
        let options = match control(deadline_ms, cancel) {
            Ok(options) => options,
            Err(error) => return engine.fail(error),
        };
        match run_recognize(engine, spec_json, text, text_len, options) {
            Ok(json) => unsafe { hand_over(engine, json, out, out_len) },
            Err(error) => engine.fail(error),
        }
    })
}

/// The shared body of `thinkthen_recognize`, safe on the Rust side.
fn run_recognize(
    engine: &thinkthen_engine,
    spec_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    options: Options<'_>,
) -> Result<String, Error> {
    let spec = str_required(spec_json, "spec")?;
    let ask = Recognize::from_json(spec)?;
    let text = str_from(text, text_len)?;
    Ok(engine.engine.recognize_opts(&ask, text, options)?.to_json())
}

/// Say how every record relates to the others, as one JSON object the
/// caller frees with [`thinkthen_free_string`], with no budget and no
/// token: exactly [`thinkthen_relate_opts`] with `THINKTHEN_NO_DEADLINE`
/// and a null token.
///
/// # Safety
///
/// The same rules as [`thinkthen_relate_opts`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_relate(
    engine: *const thinkthen_engine,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || unsafe {
        thinkthen_relate_opts(
            engine,
            spec_json,
            texts,
            lengths,
            count,
            NO_DEADLINE,
            std::ptr::null(),
            out,
            out_len,
        )
    })
}

/// Say how every record relates to the others, as one JSON object the
/// caller frees with [`thinkthen_free_string`]: `{"edges": [...]}`, with
/// the options beside it.
/// `texts` holds `count` pointers and `lengths` their byte lengths; more
/// than 255 records is refused with the usage kind before anything else.
/// The return is zero on success and the kind code on a failure, with the
/// out parameters left alone.
///
/// # Safety
///
/// `engine` is null or a value this door returned, `spec_json` is a
/// readable null-terminated string or null, the arrays are readable for
/// `count` entries, `cancel` is null or a live token, and `out` and
/// `out_len` are writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_relate_opts(
    engine: *const thinkthen_engine,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    deadline_ms: i64,
    cancel: *const thinkthen_cancel_token,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    guard(engine, || THINKTHEN_EDEFECT, || {
        let Some(engine) = (unsafe { engine.as_ref() }) else {
            return THINKTHEN_EUSAGE;
        };
        if out.is_null() {
            return engine.fail(Error::usage("a null out pointer"));
        }
        if out_len.is_null() {
            return engine.fail(Error::usage("a null out_len pointer"));
        }
        let options = match control(deadline_ms, cancel) {
            Ok(options) => options,
            Err(error) => return engine.fail(error),
        };
        match run_relate(engine, spec_json, texts, lengths, count, options) {
            Ok(json) => unsafe { hand_over(engine, json, out, out_len) },
            Err(error) => engine.fail(error),
        }
    })
}

/// The shared body of `thinkthen_relate`, safe on the Rust side.
fn run_relate(
    engine: &thinkthen_engine,
    spec_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    options: Options<'_>,
) -> Result<String, Error> {
    let spec = str_required(spec_json, "spec")?;
    let ask = Relate::from_json(spec)?;
    if count > 0 {
        if texts.is_null() {
            return Err(Error::usage("a null texts array with a nonzero count"));
        }
        if lengths.is_null() {
            return Err(Error::usage("a null lengths array with a nonzero count"));
        }
    }
    let mut records = Vec::with_capacity(count);
    for index in 0..count {
        let pointer = unsafe { *texts.add(index) };
        let length = unsafe { *lengths.add(index) };
        records.push(str_from(pointer, length)?);
    }
    let edges = relate_checked(engine.engine.as_ref(), &ask, &records, options)?;
    Ok(edges_json(&edges))
}

/// Hand one JSON answer to the caller: the string and its byte length land
/// in the out parameters on zero, and nothing is written on a failure.
///
/// # Safety
///
/// `out` and `out_len` are writable.
unsafe fn hand_over(
    engine: &thinkthen_engine,
    json: String,
    out: *mut *mut c_char,
    out_len: *mut usize,
) -> i32 {
    match CString::new(json) {
        Ok(body) => {
            let length = body.as_bytes().len();
            unsafe {
                *out = body.into_raw();
                *out_len = length;
            }
            0
        }
        Err(_) => engine.fail(Error::defect("the answer held a NUL")),
    }
}

/// Free a string [`thinkthen_call`], [`thinkthen_recognize`], or
/// [`thinkthen_relate`] returned. Null is accepted and ignored.
///
/// # Safety
///
/// `text` is null or a pointer this door returned and has not freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_free_string(text: *mut c_char) {
    guard(std::ptr::null(), || {}, || {
        if !text.is_null() {
            drop(unsafe { CString::from_raw(text) });
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use thinkthen_contract::Error as ContractError;

    /// The defect kind is its own code, and its message and retry signal
    /// ride the engine's failure table for the calling thread. The
    /// construction stands in for the injected fault main's engine-only
    /// case uses; no public fault hook exists.
    #[test]
    fn the_defect_kind_carries_code_six() {
        assert_eq!(code_of(&ErrorKind::Defect), 6);
        let engine = unsafe { thinkthen_engine_new() };
        assert!(!engine.is_null(), "the engine builds without a wire");
        let held = unsafe { &*engine };
        let code = held.fail(ContractError::defect("the engine broke its own contract"));
        assert_eq!(code, 6);
        assert_eq!(
            unsafe { thinkthen_error_code(engine) },
            6,
            "the failing code rides the engine's failure table"
        );
        let message = unsafe { CStr::from_ptr(thinkthen_error_message(engine)) };
        assert!(message
            .to_str()
            .expect("the message is UTF-8")
            .contains("the engine broke its own contract"));
        assert_eq!(unsafe { thinkthen_error_retryable(engine) }, 0);
        unsafe { thinkthen_engine_free(engine) };
    }

    /// A panic beneath the door is caught by the contract's shared
    /// boundary, recorded as the defect kind for the calling thread, and
    /// returned as a code instead of unwinding into the host. Every
    /// exported symbol runs behind this same guard.
    #[test]
    fn a_panic_behind_the_door_comes_back_as_the_defect_kind() {
        let engine = unsafe { thinkthen_engine_new() };
        assert!(!engine.is_null(), "the engine builds without a wire");
        let code = guard(engine, || THINKTHEN_EDEFECT, || panic!("the probe panic"));
        assert_eq!(code, THINKTHEN_EDEFECT, "a panic is the defect code");
        assert_eq!(unsafe { thinkthen_error_code(engine) }, THINKTHEN_EDEFECT);
        let message = unsafe { CStr::from_ptr(thinkthen_error_message(engine)) }
            .to_string_lossy()
            .into_owned();
        assert!(
            message.contains("the probe panic"),
            "the panic's own text is recorded: {message}"
        );
        assert!(
            message.contains("a panic crossed the C door"),
            "the shared boundary's own spelling is recorded: {message}"
        );
        assert_eq!(
            unsafe { thinkthen_error_retryable(engine) },
            0,
            "a defect is not retryable"
        );
        unsafe { thinkthen_engine_free(engine) };
    }

    /// A failure is visible only on the engine and thread that recorded
    /// it: the table belongs to the engine and is keyed by the recording
    /// thread, so a failure on one engine never hides another engine's
    /// failure, and neither engine's message replaces the other's.
    #[test]
    fn a_failure_on_one_engine_does_not_hide_another() {
        let first = unsafe { thinkthen_engine_new() };
        let second = unsafe { thinkthen_engine_new() };
        assert!(!first.is_null() && !second.is_null(), "both engines build");
        let held_first = unsafe { &*first };
        let held_second = unsafe { &*second };
        held_first.fail(ContractError::usage("the first engine's failure"));
        held_second.fail(ContractError::local("the second engine's failure"));

        assert_eq!(
            unsafe { thinkthen_error_code(first) },
            THINKTHEN_EUSAGE,
            "the first engine still reads its own failure"
        );
        assert_eq!(
            unsafe { thinkthen_error_code(second) },
            4,
            "the second engine reads its own failure"
        );
        let first_message = unsafe { CStr::from_ptr(thinkthen_error_message(first)) }
            .to_string_lossy()
            .into_owned();
        let second_message = unsafe { CStr::from_ptr(thinkthen_error_message(second)) }
            .to_string_lossy()
            .into_owned();
        assert!(first_message.contains("the first engine's failure"), "{first_message}");
        assert!(second_message.contains("the second engine's failure"), "{second_message}");
        unsafe {
            thinkthen_engine_free(first);
            thinkthen_engine_free(second);
        }
    }

    /// A failure is visible only to the thread that recorded it: the table
    /// is keyed by the recording thread, so a second thread's `fail` does
    /// not replace the first thread's message and a fresh thread reads no
    /// failure at all.
    #[test]
    fn a_failure_belongs_to_the_thread_that_recorded_it() {
        let engine = unsafe { thinkthen_engine_new() };
        assert!(!engine.is_null(), "the engine builds without a wire");
        let held = unsafe { &*engine };
        held.fail(ContractError::usage("the main thread's failure"));
        assert_eq!(unsafe { thinkthen_error_code(engine) }, THINKTHEN_EUSAGE);

        // A raw pointer is not `Send`; the thread gets its address and
        // rebuilds it, the same shape `tests/concurrency.rs` uses.
        let key = engine as usize;
        std::thread::scope(|scope| {
            scope.spawn(move || {
                let engine = key as *mut thinkthen_engine;
                // A fresh thread reads no failure of its own, even though
                // another thread already recorded one on the same engine.
                assert_eq!(unsafe { thinkthen_error_code(engine) }, THINKTHEN_OK);
                assert_eq!(unsafe { thinkthen_error_retryable(engine) }, 0);
                let message = unsafe { CStr::from_ptr(thinkthen_error_message(engine)) }
                    .to_string_lossy()
                    .into_owned();
                assert_eq!(message, "no failure yet");
            });
        });

        // The recorder's own message survived the other thread untouched.
        let message = unsafe { CStr::from_ptr(thinkthen_error_message(engine)) }
            .to_string_lossy()
            .into_owned();
        assert!(message.contains("the main thread's failure"), "{message}");
        unsafe { thinkthen_engine_free(engine) };
    }

    /// Only the exact sentinel means "no deadline": every other negative
    /// value is refused with the usage kind, zero is the spent budget the
    /// engine itself refuses, and a positive value converts. Before this,
    /// any negative value read as no deadline.
    #[test]
    fn only_the_sentinel_means_no_deadline() {
        assert!(
            control(NO_DEADLINE, std::ptr::null()).is_ok(),
            "the sentinel means no deadline"
        );
        assert!(
            control(0, std::ptr::null()).is_ok(),
            "zero is passed on; the engine refuses the spent budget"
        );
        assert!(
            control(1_000, std::ptr::null()).is_ok(),
            "a positive budget converts"
        );
        let refused = control(-2, std::ptr::null())
            .err()
            .expect("every other negative refuses");
        assert_eq!(refused.kind, ErrorKind::Usage);
        assert!(
            refused.message.contains("negative"),
            "the message names why: {}",
            refused.message
        );
        let smallest = control(i64::MIN, std::ptr::null())
            .err()
            .expect("the smallest negative refuses");
        assert_eq!(smallest.kind, ErrorKind::Usage);
    }
}
