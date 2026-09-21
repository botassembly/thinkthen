//! The C door to the thinkthen engine, version 0.0.1.
//!
//! This crate is the whole C surface: it builds `libthinkthen.so` and
//! `libthinkthen.a` against [`contract/include/thinkthen.h`] and owns every
//! exported symbol. The engine beneath exports none. The stand-in engine
//! implements the contract today; when the real engine lands, the path
//! dependency changes and nothing here does.
//!
//! The door follows the header's lifetime rules exactly: an engine lives
//! until `thinkthen_engine_free`, a string from `thinkthen_call` lives
//! until `thinkthen_free_string`, and the message from
//! `thinkthen_error_message` lives until the next call on the same engine.
//!
//! Two findings from the slide, filed in `NOTES.md`, shape the code: the
//! slide passes a bare question string where the header's doc promised the
//! question-file grammar, so [`thinkthen_decide`] accepts either; and the
//! header exposes no cancel token, deadline, or poll callback, so a C host
//! cannot reach those today — recorded as a contract finding, not worked
//! around.
//!
//! [`contract/include/thinkthen.h`]: ../../contract/include/thinkthen.h

use std::ffi::{c_char, CStr, CString};
use std::sync::Mutex;

use thinkthen_contract::{
    Annotated, Answer, Details, Engine as _, Error, ErrorKind, Options, Question,
    QuestionSet, Ranked, Scored,
};
use thinkthen_standin::BlockingEngine;

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

/// The opaque engine value. The engine plus the last failure's message,
/// which the header promises stays valid until the next call.
#[allow(non_camel_case_types)] // the header's own name for the value
pub struct thinkthen_engine {
    /// The engine the door calls. Built from the environment.
    engine: BlockingEngine,
    /// The last failure: its code, its retry signal, and a C string the
    /// caller may still hold a pointer to. Guarded because a host may
    /// call the door from many threads over one engine.
    last: Mutex<Option<LastError>>,
}

/// The stored failure behind `thinkthen_error_message`.
struct LastError {
    /// Whether a second try could help.
    retryable: bool,
    /// The message, kept alive until the next call replaces it.
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
    /// Record a failure and return its code. The message the caller reads
    /// stays alive until the next call on this engine.
    fn fail(&self, error: Error) -> i32 {
        let code = code_of(&error.kind);
        let message = CString::new(error.message)
            .unwrap_or_else(|_| CString::new("defect: the message held a NUL").expect("static"));
        *self
            .last
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(LastError {
            retryable: error.retryable,
            message,
        });
        code
    }

    /// One judgment over one text, in one request, carrying both the
    /// answer and the probability behind it.
    fn one_judgment(
        &self,
        question: &Question,
        evidence: &str,
    ) -> Result<(Answer, f64), Error> {
        let mut judgments = self
            .engine
            .decide_many_opts(question, &[evidence], Options::new(), None)?;
        let judgment = judgments.pop().ok_or_else(|| {
            Error::defect("the engine returned no judgment for one text")
        })?;
        Ok((judgment.answer, judgment.probability))
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
// contract/include/thinkthen.h; unsafe is the boundary, and each body
// checks what it can before trusting the host.

/// Build an engine from the environment. Returns null only when the
/// process cannot hold an engine at all.
///
/// # Panics
///
/// Never: allocation failure aborts the process by Rust's own rule.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_new() -> *mut thinkthen_engine {
    let engine = BlockingEngine::from_env();
    Box::into_raw(Box::new(thinkthen_engine {
        engine,
        last: Mutex::new(None),
    }))
}

/// Free an engine. Null is accepted and ignored.
///
/// # Safety
///
/// `engine` is null or a value this door returned and no other call is
/// using.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_engine_free(engine: *mut thinkthen_engine) {
    if !engine.is_null() {
        drop(unsafe { Box::from_raw(engine) });
    }
}

/// The message for the last failure on this engine, valid until the next
/// call. Never null: before any failure it names that nothing failed yet.
///
/// # Safety
///
/// `engine` is a value this door returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_message(
    engine: *const thinkthen_engine,
) -> *const c_char {
    let engine = unsafe { engine.as_ref().expect("engine is not null") };
    let guard = engine
        .last
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    match guard.as_ref() {
        Some(last) => last.message.as_ptr(),
        None => c"no failure yet".as_ptr(),
    }
}

/// Whether a second try could help the last failure: 1 when it could,
/// 0 when it could not or nothing failed.
///
/// # Safety
///
/// `engine` is a value this door returned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_error_retryable(engine: *const thinkthen_engine) -> i32 {
    let engine = unsafe { engine.as_ref().expect("engine is not null") };
    let guard = engine
        .last
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    i32::from(guard.as_ref().is_some_and(|last| last.retryable))
}

/// Ask one yes-or-no question of one text. `question_json` is one question
/// in the question-file grammar or the bare text of a decide question;
/// `text_len` is the evidence's byte length. The judgment lands in `out`
/// on zero; any nonzero code left `out` untouched.
///
/// # Safety
///
/// `engine` is a value this door returned, `question_json` and `text` are
/// readable for their lengths, and `out` is writable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide(
    engine: *const thinkthen_engine,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
    out: *mut thinkthen_answer,
) -> i32 {
    let engine = unsafe { engine.as_ref().expect("engine is not null") };
    let (answer, probability) = match run_decide(engine, question_json, text, text_len) {
        Ok(pair) => pair,
        Err(error) => return engine.fail(error),
    };
    unsafe {
        *out = thinkthen_answer {
            outcome: outcome_of(&answer),
            probability,
        };
    }
    0
}

/// The shared body of the two decide doors, safe on the Rust side.
fn run_decide(
    engine: &thinkthen_engine,
    question_json: *const c_char,
    text: *const c_char,
    text_len: usize,
) -> Result<(Answer, f64), Error> {
    let question_json = unsafe { CStr::from_ptr(question_json) }
        .to_str()
        .map_err(|_| Error::usage("the question is not UTF-8"))?;
    let question = question_from(question_json)?;
    let evidence = str_from(text, text_len)?;
    engine.one_judgment(&question, evidence)
}

/// Ask the same question of every text at once, at the engine's width,
/// keeping every judgment in input order. `texts` holds `count` pointers
/// and `lengths` their byte lengths; `out` holds room for `count`
/// answers.
///
/// # Safety
///
/// `engine` is a value this door returned, the arrays are readable for
/// `count` entries, and `out` is writable for `count` answers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_decide_many(
    engine: *const thinkthen_engine,
    question_json: *const c_char,
    texts: *const *const c_char,
    lengths: *const usize,
    count: usize,
    out: *mut thinkthen_answer,
) -> i32 {
    let engine = unsafe { engine.as_ref().expect("engine is not null") };
    let question = match unsafe { CStr::from_ptr(question_json) }
        .to_str()
        .map_err(|_| Error::usage("the question is not UTF-8"))
        .and_then(question_from)
    {
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
        .decide_many_opts(&question, &records, Options::new(), None)
    {
        Ok(judgments) => {
            for (index, judgment) in judgments.iter().enumerate() {
                unsafe {
                    *out.add(index) = thinkthen_answer {
                        outcome: outcome_of(&judgment.answer),
                        probability: judgment.probability,
                    };
                }
            }
            0
        }
        Err(error) => engine.fail(error),
    }
}

/// One borrowed `&str` from a pointer and a byte length, refusing text
/// that is not UTF-8 as a usage failure.
///
/// # Errors
///
/// Returns the usage kind when the bytes are not a UTF-8 string.
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

/// The JSON door: any request of the eight verbs, with the answer as JSON
/// text the caller frees with [`thinkthen_free_string`]. The request is
/// the question file's own shape with the evidence beside it, and the
/// door's reply shapes are listed in this crate's `NOTES.md`. Returns
/// null on failure, with the code on the engine.
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
///   audit view
/// - `{"usage": true}` for the counters
///
/// # Safety
///
/// `engine` is a value this door returned and `request_json` is a readable
/// null-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_call(
    engine: *const thinkthen_engine,
    request_json: *const c_char,
) -> *mut c_char {
    let engine = unsafe { engine.as_ref().expect("engine is not null") };
    let request = match unsafe { CStr::from_ptr(request_json) }
        .to_str()
        .map_err(|_| Error::usage("the request is not UTF-8"))
        .and_then(|text| {
            serde_json::from_str::<serde_json::Value>(text)
                .map_err(|error| Error::usage(format!("the request is not JSON: {error}")))
        }) {
        Ok(request) => request,
        Err(error) => {
            engine.fail(error);
            return std::ptr::null_mut();
        }
    };
    match call_verb(engine, &request) {
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
}

/// Route one parsed request to its verb. Every reply is a JSON value the
/// caller reads; every error is the door's six kinds.
fn call_verb(
    engine: &thinkthen_engine,
    request: &serde_json::Value,
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
        engine.one_judgment(&question, evidence)
    };

    if object.contains_key("details") {
        let evidence = evidence("evidence")?;
        let question_text = serde_json::to_string(&serde_json::Value::Object(question_object))
            .map_err(|error| Error::defect(format!("the question did not serialize: {error}")))?;
        let question = question_from(&question_text)?;
        return details_json(engine, &question, &evidence);
    }
    if let Some(annotate) = object.get("annotate") {
        let set_text = serde_json::to_string(annotate)
            .map_err(|error| Error::defect(format!("the set did not serialize: {error}")))?;
        let set = QuestionSet::from_json(&set_text)?;
        let records = records()?;
        let references: Vec<&str> = records.iter().map(String::as_str).collect();
        let answers = engine
            .engine
            .annotate_opts(&set, &references, Options::new(), None)?;
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
            engine.engine.find_opts(&question, &units, Options::new())?;
        return Ok(serde_json::json!({
            "answer": found.index,
            "probability": found.probability,
        }));
    }
    if question_object.contains_key("decide") {
        if let Some(_) = object.get("records") {
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
                    .rank_opts(&question, &references, Options::new(), None)?;
                return Ok(serde_json::json!({ "answer": ranked_json(&ranked) }));
            }
            let kept = engine
                .engine
                .filter_opts(&question, &references, Options::new(), None)?;
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
            .choose_opts(&question, &evidence, Options::new())?;
        return Ok(serde_json::json!({
            "answer": chosen,
        }));
    }
    if question_object.contains_key("score") {
        let evidence = evidence("evidence")?;
        let question = object_question(&question_object)?;
        let Scored { value, nearest } =
            engine.engine.score_opts(&question, &evidence, Options::new())?;
        return Ok(serde_json::json!({
            "answer": value,
            "nearest": nearest,
        }));
    }
    if question_object.contains_key("tag") {
        let evidence = evidence("evidence")?;
        let question = object_question(&question_object)?;
        let labels = engine.engine.tag_opts(&question, &evidence, Options::new())?;
        return Ok(serde_json::json!({ "answer": labels }));
    }
    Err(Error::usage(
        "the request carries no verb the door knows: decide, choose, score, tag, find, annotate, details, usage",
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

/// The audit view as the door returns it.
fn details_json(
    engine: &thinkthen_engine,
    question: &Question,
    evidence: &str,
) -> Result<serde_json::Value, Error> {
    let Details {
        probability,
        answer,
        model,
        digest,
        sends,
    } = engine
        .engine
        .details_opts(question, evidence, Options::new())?;
    Ok(serde_json::json!({
        "probability": probability,
        "answer": json_of(&answer),
        "model": model,
        "digest": digest,
        "sends": sends,
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
                            };
                            (name.clone(), value)
                        })
                        .collect(),
                )
            })
            .collect(),
    )
}

/// Free a string [`thinkthen_call`] returned. Null is accepted and ignored.
///
/// # Safety
///
/// `text` is null or a pointer this door returned and has not freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_free_string(text: *mut c_char) {
    if !text.is_null() {
        drop(unsafe { CString::from_raw(text) });
    }
}
