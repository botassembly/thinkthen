//! The ThinkThen DuckDB surface: nine SQL functions over the one engine,
//! per ADR 0017.
//!
//! Rule 1 of the database pages holds: no rule lives here. The extension
//! converts SQL values in and out, groups each chunk by its distinct
//! built questions and texts so a repeated row costs one judgment, and
//! maps the contract's six error kinds onto DuckDB's error reporting with
//! the retryable signal carried in the message. The engine owns
//! thresholds, retries, width, the cache, the counters, and answers.
//!
//! The functions are `thinkthen_decide`, `thinkthen_probability`,
//! `thinkthen_choose`, `thinkthen_score`, `thinkthen_tag`,
//! `thinkthen_annotate`, `thinkthen_details`, `thinkthen_usage`, and
//! `thinkthen_warm`. `filter`, `rank`, and `find` get no functions:
//! `WHERE`, `ORDER BY`, and `LIMIT` are those verbs. `NULL` is "not sure"
//! on every answering function, and a failure is an error that never
//! reads as `NULL`.
//!
//! A question argument is the question, as plain text under the grammar's
//! default cut, as a file named with the command's spelling
//! `'@refund.json'`, or as the file grammar's own JSON. A verb whose
//! members ride in a `LIST` argument takes its question as plain text and
//! its members from the list; the file spellings carry their own.
//!
//! The engine builds lazily on the first call and never in the
//! load-time init: a load may not touch the wire. Every engine call
//! carries the process-wide cancel token, and the SIGINT handler below
//! sets it, so the CLI's interrupt reaches the waits between requests.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex, OnceLock};

use duckdb::{
    Connection,
    core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId},
    ffi,
    types::DuckString,
    vscalar::{ScalarFunctionSignature, VScalar},
    vtab::arrow::WritableVector,
};
use thinkthen_contract::{
    Annotated, Answer, Cancel, Engine as ContractEngine, Error as EngineError, Judgment, Options,
    Question, QuestionKind, QuestionSet, Scored,
};
use thinkthen_standin::BlockingEngine;

mod recognize;
mod relate;
mod relations;
mod usage;
mod warm;

/// The engine this surface calls, built from the environment on the first
/// call. The stand-in implements the contract today; the real engine
/// replaces it with one changed dependency.
static ENGINE: OnceLock<BlockingEngine> = OnceLock::new();

/// The engine value, built once, on first use and never at load time.
fn engine() -> &'static BlockingEngine {
    ENGINE.get_or_init(BlockingEngine::from_env)
}

/// The options every call carries: the process-wide cancel token.
fn options() -> Options<'static> {
    Options::new().maybe_cancel(Some(&CANCEL))
}

/// The process-wide cancellation token. Every engine call carries it, and
/// the SIGINT handler below sets it, so the CLI's interrupt path reaches
/// the waits between requests and between batch items.
static CANCEL: LazyLock<Cancel> = LazyLock::new(Cancel::new);

/// The handler SIGINT replaced, so the CLI's own interrupt still runs.
static HANDLER_CHAIN: AtomicUsize = AtomicUsize::new(0);

/// Whether the handler is already installed, so a second LOAD does not
/// chain a handler to itself.
static HANDLER_SET: AtomicBool = AtomicBool::new(false);

/// Cancel the token, then run whatever handler was there before. SIG_DFL
/// and SIG_IGN sit at 0 and 1 and are never called as functions.
extern "C" fn on_interrupt(signal: libc::c_int) {
    CANCEL.cancel();
    let previous = HANDLER_CHAIN.load(Ordering::SeqCst);
    if previous > 1 {
        let chained: extern "C" fn(libc::c_int) = unsafe { std::mem::transmute(previous) };
        chained(signal);
    }
}

/// Take SIGINT for the token, keeping the previous handler on the chain.
unsafe fn install_interrupt_handler() {
    if HANDLER_SET.swap(true, Ordering::SeqCst) {
        return;
    }
    let previous = unsafe {
        libc::signal(
            libc::SIGINT,
            on_interrupt as *const () as libc::sighandler_t,
        )
    };
    if previous != libc::SIG_ERR && previous > 1 {
        HANDLER_CHAIN.store(previous, Ordering::SeqCst);
    }
}

/// A failed warm's poison, keyed by question argument and text. No
/// error-reporting call works from an aggregate callback on this C API
/// version, so a failed warm marks its texts here and the scalar query
/// that reads them raises the error through the path that does work.
/// The entry clears the moment it is raised, so the pair is retried on
/// the next call instead of failing forever.
static POISON: LazyLock<Mutex<HashMap<(String, String), String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Mark texts a failed warm could not judge.
fn poison(question: &str, texts: &HashSet<String>, failure: &str) {
    let mut poisoned = POISON.lock().expect("the poison");
    for text in texts {
        poisoned.insert((question.to_owned(), text.clone()), failure.to_owned());
    }
}

/// A contract failure as the SQL error a reader sees, with the kind and
/// the retryable signal both carried.
fn failure(error: EngineError) -> String {
    let retry = if error.retryable {
        " (a second try could help)"
    } else {
        ""
    };
    format!("thinkthen {}{retry}: {}", error.kind, error.message)
}

/// Resolve a question argument: `'@file.json'` names a file, a leading
/// `{` is the file grammar's own JSON, and anything else is the question
/// as plain text under the grammar's default cut. One grammar, one door.
fn resolve_question(arg: &str) -> Result<Question, String> {
    if let Some(path) = arg.strip_prefix('@') {
        Question::from_file(Path::new(path)).map_err(failure)
    } else if arg.starts_with('{') {
        Question::from_json(arg).map_err(failure)
    } else {
        let file = serde_json::json!({ "decide": arg }).to_string();
        Question::from_json(&file).map_err(failure)
    }
}

/// Build a members verb's question from plain text and the `LIST`
/// argument. The members live in the list; the file spellings belong to
/// the verbs whose members ride inside the question.
fn resolve_member_question(
    arg: &str,
    members: &[String],
    kind: QuestionKind,
) -> Result<Question, String> {
    if arg.starts_with('@') || arg.starts_with('{') {
        return Err(format!(
            "thinkthen usage: {kind} takes its question as plain text and its {} in the list",
            match kind {
                QuestionKind::Choose => "options",
                QuestionKind::Score => "levels",
                _ => "labels",
            }
        ));
    }
    let owned: Vec<&str> = members.iter().map(String::as_str).collect();
    let built = match kind {
        QuestionKind::Choose => Question::choose(arg, &owned).and_then(ChoiceBuilt::build),
        QuestionKind::Score => Question::score(arg, &owned),
        QuestionKind::Tag => Question::tag(arg, &owned),
        QuestionKind::Decide => unreachable!("decide carries no list"),
    };
    built.map_err(failure)
}

/// The builder `Question::choose` returns, named so `build` needs no
/// imported type.
use thinkthen_contract::ChoiceBuilder as ChoiceBuilt;

/// Judge one built question's texts at the engine's width. One text asks
/// once; many cross once through the batch door.
fn judged(question: &Question, texts: &[&str]) -> Result<Vec<Judgment>, String> {
    engine()
        .decide_many_opts(question, texts, options(), None)
        .map_err(failure)
}

/// The warm aggregate's judge: resolve the question argument, judge every
/// distinct text once through the batch door, and return how many were
/// asked. A failure poisons the texts it reached and still returns a
/// count, because no aggregate error channel exists on this C API version.
fn judge_warm(arg: &str, seen: HashSet<String>) -> Result<usize, String> {
    if seen.is_empty() {
        return Ok(0);
    }
    let mut texts: Vec<String> = seen.iter().cloned().collect();
    texts.sort_unstable();
    let question = resolve_question(arg)?;
    let borrowed: Vec<&str> = texts.iter().map(String::as_str).collect();
    match judged(&question, &borrowed) {
        Ok(_) => Ok(texts.len()),
        Err(failed) => {
            poison(arg, &seen, &failed);
            Ok(0)
        }
    }
}

/// The distinct questions and texts of a chunk, with each row's slot into
/// them, so a repeated (question, text) pair costs one judgment.
struct Distinct {
    /// The built questions, one per distinct digest, in first-seen order.
    questions: Vec<Question>,
    /// The distinct (digest index, text) pairs the rows hold.
    pairs: Vec<(usize, String)>,
    /// Each row's pair, or `None` where a NULL made the row NULL.
    slots: Vec<Option<usize>>,
}

/// Group a chunk by its distinct built questions and texts. Questions are
/// keyed by the core's own digest, which covers the text, the threshold,
/// and the members, so two spellings of one question group together.
fn distinct_of(
    args: &[Option<String>],
    members: &[Option<Vec<String>>],
    texts: &[Option<String>],
    kind: Option<QuestionKind>,
) -> Result<Distinct, String> {
    let mut built: HashMap<String, usize> = HashMap::new();
    let mut questions: Vec<Question> = Vec::new();
    let mut pairs: Vec<(usize, String)> = Vec::new();
    let mut pair_index: HashMap<(usize, String), usize> = HashMap::new();
    let mut slots: Vec<Option<usize>> = vec![None; texts.len()];
    for i in 0..texts.len() {
        let (Some(arg), Some(text)) = (args[i].as_deref(), texts[i].as_deref()) else {
            continue;
        };
        if let Some(failed) = POISON
            .lock()
            .expect("the poison")
            .remove(&(arg.to_owned(), text.to_owned()))
        {
            return Err(failed);
        }
        let question = match kind {
            None => resolve_question(arg)?,
            Some(kind) => {
                let Some(list) = members[i].as_deref() else {
                    continue;
                };
                resolve_member_question(arg, list, kind)?
            }
        };
        let digest = question.digest();
        let question_at = *built.entry(digest).or_insert_with(|| {
            questions.push(question);
            questions.len() - 1
        });
        let key = (question_at, text.to_owned());
        let slot = *pair_index.entry(key).or_insert_with(|| {
            pairs.push((question_at, text.to_owned()));
            pairs.len() - 1
        });
        slots[i] = Some(slot);
    }
    Ok(Distinct {
        questions,
        pairs,
        slots,
    })
}

/// Read one VARCHAR column of a `VScalar` chunk as owned strings.
fn read_strings(input: &mut DataChunkHandle, column: usize) -> Vec<Option<String>> {
    let len = input.len();
    let vector = input.flat_vector(column);
    let raw = unsafe { vector.as_slice_with_len::<duckdb::ffi::duckdb_string_t>(len) };
    (0..len)
        .map(|i| {
            if vector.row_is_null(i as u64) {
                None
            } else {
                let mut value = raw[i];
                Some(DuckString::new(&mut value).as_str().to_string())
            }
        })
        .collect()
}

/// Read one LIST(VARCHAR) column of a `VScalar` chunk as owned string
/// lists. A NULL list or a NULL member inside one makes the row NULL, the
/// same semantics a NULL question or text carries.
fn read_list_strings(input: &mut DataChunkHandle, column: usize) -> Vec<Option<Vec<String>>> {
    let len = input.len();
    let lists = input.list_vector(column);
    let child_len = lists.len();
    let child = lists.child(child_len.max(1));
    let raw = unsafe { child.as_slice_with_len::<duckdb::ffi::duckdb_string_t>(child_len.max(1)) };
    (0..len)
        .map(|i| {
            if lists.row_is_null(i as u64) {
                return None;
            }
            let (offset, length) = lists.get_entry(i);
            let mut items = Vec::with_capacity(length);
            for j in 0..length {
                let at = offset + j;
                if at >= child_len || child.row_is_null(at as u64) {
                    return None;
                }
                let mut value = raw[at];
                items.push(DuckString::new(&mut value).as_str().to_string());
            }
            Some(items)
        })
        .collect()
}

/// Read one VARCHAR column of a raw C-API chunk as owned strings. NULL in
/// the validity mask reads as `None`.
pub(crate) unsafe fn read_raw_column(
    input: ffi::duckdb_data_chunk,
    column: usize,
) -> Vec<Option<String>> {
    let len = unsafe { ffi::duckdb_data_chunk_get_size(input) } as usize;
    let vector = unsafe { ffi::duckdb_data_chunk_get_vector(input, column as u64) };
    let data = unsafe { ffi::duckdb_vector_get_data(vector) } as *mut ffi::duckdb_string_t;
    let mask = unsafe { ffi::duckdb_vector_get_validity(vector) };
    (0..len)
        .map(|i| {
            let valid =
                mask.is_null() || unsafe { ffi::duckdb_validity_row_is_valid(mask, i as u64) };
            if !valid {
                None
            } else {
                let mut raw = unsafe { *data.add(i) };
                Some(DuckString::new(&mut raw).as_str().to_string())
            }
        })
        .collect()
}

/// `thinkthen_decide(question, text)`: the database's boolean, `NULL` for
/// "not sure".
struct DecideScalar;

impl VScalar for DecideScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let distinct = distinct_of(&questions, &[], &texts, None)?;
        let answers = judged_rows(&distinct)?;
        {
            let mut out = output.flat_vector();
            let values = unsafe { out.as_mut_slice_with_len::<bool>(answers.len()) };
            for (i, answer) in answers.iter().enumerate() {
                if let Some(value) = answer.and_then(Answer::value) {
                    values[i] = value;
                }
            }
        }
        let mut out = output.flat_vector();
        for (i, answer) in answers.iter().enumerate() {
            if answer.and_then(Answer::value).is_none() {
                out.set_null(i);
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            LogicalTypeId::Boolean.into(),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// Every row's decide answer, drawn from the chunk's distinct pairs.
fn judged_rows(distinct: &Distinct) -> std::result::Result<Vec<Option<Answer>>, String> {
    let mut answers: Vec<Option<Answer>> = vec![None; distinct.pairs.len()];
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for (slot, (question_at, _)) in distinct.pairs.iter().enumerate() {
        groups.entry(*question_at).or_default().push(slot);
    }
    for (question_at, slots) in groups {
        let texts: Vec<&str> = slots
            .iter()
            .map(|slot| distinct.pairs[*slot].1.as_str())
            .collect();
        for (slot, judgment) in slots
            .iter()
            .zip(judged(&distinct.questions[question_at], &texts)?)
        {
            answers[*slot] = Some(judgment.answer);
        }
    }
    Ok(answers)
}

/// `thinkthen_probability(question, text)`: the probability the backend
/// gave the yes side, and `ORDER BY` on it ranks the rows.
struct ProbabilityScalar;

impl VScalar for ProbabilityScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let distinct = distinct_of(&questions, &[], &texts, None)?;
        let mut values: Vec<Option<f64>> = vec![None; distinct.pairs.len()];
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        for (slot, (question_at, _)) in distinct.pairs.iter().enumerate() {
            groups.entry(*question_at).or_default().push(slot);
        }
        for (question_at, slots) in groups {
            let texts: Vec<&str> = slots
                .iter()
                .map(|slot| distinct.pairs[*slot].1.as_str())
                .collect();
            for (slot, judgment) in slots
                .iter()
                .zip(judged(&distinct.questions[question_at], &texts)?)
            {
                values[*slot] = Some(judgment.probability);
            }
        }
        {
            let mut out = output.flat_vector();
            let slice = unsafe { out.as_mut_slice_with_len::<f64>(values.len()) };
            for (i, value) in values.iter().enumerate() {
                if let Some(number) = value {
                    slice[i] = *number;
                }
            }
        }
        let mut out = output.flat_vector();
        for (i, value) in values.iter().enumerate() {
            if value.is_none() {
                out.set_null(i);
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            LogicalTypeId::Double.into(),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_choose(question, text, options)`: the option that won,
/// `NULL` when nothing cleared the question's own rule.
struct ChooseScalar;

impl VScalar for ChooseScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let option_lists = read_list_strings(input, 2);
        let distinct = distinct_of(
            &questions,
            &option_lists,
            &texts,
            Some(QuestionKind::Choose),
        )?;
        let mut picked: Vec<Option<Option<String>>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let answer = engine()
                .choose_opts(&distinct.questions[*question_at], text, options())
                .map_err(failure)?;
            picked[slot] = Some(answer);
        }
        let mut out = output.flat_vector();
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| picked[slot].clone()) {
                Some(Some(choice)) => {
                    out.insert(i, choice.as_str());
                }
                _ => out.set_null(i),
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![
                LogicalTypeId::Varchar.into(),
                LogicalTypeId::Varchar.into(),
                LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
            ],
            LogicalTypeId::Varchar.into(),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_score(question, text, levels)`: the specification's
/// probability-weighted position from 0 to K-1. The nearest level's name
/// rides in `thinkthen_details`.
struct ScoreScalar;

impl VScalar for ScoreScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let levels = read_list_strings(input, 2);
        let distinct = distinct_of(&questions, &levels, &texts, Some(QuestionKind::Score))?;
        let mut scored: Vec<Option<Scored>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let answer = engine()
                .score_opts(&distinct.questions[*question_at], text, options())
                .map_err(failure)?;
            scored[slot] = Some(answer);
        }
        {
            let mut out = output.flat_vector();
            let slice = unsafe { out.as_mut_slice_with_len::<f64>(scored.len()) };
            for (i, value) in scored.iter().enumerate() {
                if let Some(score) = value {
                    slice[i] = score.value;
                }
            }
        }
        let mut out = output.flat_vector();
        for (i, value) in scored.iter().enumerate() {
            if value.is_none() {
                out.set_null(i);
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![
                LogicalTypeId::Varchar.into(),
                LogicalTypeId::Varchar.into(),
                LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
            ],
            LogicalTypeId::Double.into(),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_tag(question, text, labels)`: the labels that held, in the
/// question's own order.
struct TagScalar;

impl VScalar for TagScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let labels = read_list_strings(input, 2);
        let distinct = distinct_of(&questions, &labels, &texts, Some(QuestionKind::Tag))?;
        let mut held: Vec<Option<Vec<String>>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let answer = engine()
                .tag_opts(&distinct.questions[*question_at], text, options())
                .map_err(failure)?;
            held[slot] = Some(answer);
        }
        let len = texts.len();
        let row_held: Vec<Option<Vec<String>>> = (0..len)
            .map(|i| distinct.slots[i].and_then(|slot| held[slot].clone()))
            .collect();
        let total: usize = row_held
            .iter()
            .map(|row| row.as_ref().map_or(0, Vec::len))
            .sum();

        let mut lists = output.list_vector();
        let child = lists.child(total.max(1));
        let mut offset = 0usize;
        for (i, row) in row_held.iter().enumerate() {
            match row {
                None => lists.set_null(i),
                Some(values) => {
                    lists.set_entry(i, offset, values.len());
                    for value in values {
                        child.insert(offset, value.as_str());
                        offset += 1;
                    }
                }
            }
        }
        lists.set_len(total);
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![
                LogicalTypeId::Varchar.into(),
                LogicalTypeId::Varchar.into(),
                LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
            ],
            LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// `thinkthen_annotate(set, text)`: one request per record, every question
/// with the same evidence riding in it, the evidence billed once.
///
/// The carrier is a JSON object, one field per question in the set's name
/// order, because a scalar's return type is declared at registration and
/// a set's questions are not known there. DuckDB's own JSON functions
/// read the object, so no field is out of reach.
struct AnnotateScalar;

impl VScalar for AnnotateScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let sets = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let mut distinct: HashMap<(String, String), usize> = HashMap::new();
        let mut unique: Vec<(String, String)> = Vec::new();
        let mut slots: Vec<Option<usize>> = vec![None; texts.len()];
        for i in 0..texts.len() {
            let (Some(arg), Some(text)) = (sets[i].as_deref(), texts[i].as_deref()) else {
                continue;
            };
            let key = (arg.to_owned(), text.to_owned());
            let slot = *distinct.entry(key).or_insert_with(|| {
                unique.push((arg.to_owned(), text.to_owned()));
                unique.len() - 1
            });
            slots[i] = Some(slot);
        }
        let mut judged: Vec<Option<String>> = vec![None; unique.len()];
        for (slot, (arg, text)) in unique.iter().enumerate() {
            let set = if let Some(path) = arg.strip_prefix('@') {
                QuestionSet::from_file(Path::new(path)).map_err(failure)?
            } else if arg.starts_with('{') {
                QuestionSet::from_json(arg).map_err(failure)?
            } else {
                return Err(
                    "thinkthen usage: annotate names a question set as '@form.json' or JSON".into(),
                );
            };
            let records = engine()
                .annotate_opts(&set, &[text.as_str()], options(), None)
                .map_err(failure)?;
            judged[slot] = Some(annotate_json(&records[0]).map_err(failure)?);
        }
        let mut out = output.flat_vector();
        for (i, slot) in slots.iter().enumerate() {
            match slot.and_then(|slot| judged[slot].as_deref()) {
                Some(object) => {
                    out.insert(i, object);
                }
                None => out.set_null(i),
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            LogicalTypeId::Varchar.into(),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// One annotate record as the JSON object SQL reads.
fn annotate_json(record: &[(String, Annotated)]) -> Result<String, EngineError> {
    let mut object = serde_json::Map::new();
    for (name, answer) in record {
        let value = match answer {
            Annotated::Decision(answer) => match answer.value() {
                Some(true) => serde_json::Value::Bool(true),
                Some(false) => serde_json::Value::Bool(false),
                None => serde_json::Value::Null,
            },
            Annotated::Choice(None) => serde_json::Value::Null,
            Annotated::Choice(Some(winner)) => serde_json::Value::String(winner.clone()),
            Annotated::Score(scored) => serde_json::json!({
                "position": scored.value,
                "nearest": scored.nearest,
            }),
            Annotated::Tags(held) => serde_json::json!(held),
            Annotated::Failed(failed) => serde_json::json!({ "failed": failed }),
        };
        object.insert(name.clone(), value);
    }
    serde_json::to_string(&serde_json::Value::Object(object))
        .map_err(|error| EngineError::defect(error.to_string()))
}

/// One details row as the SQL struct reads it: every member `Option`, so
/// the members that exist only for a decide question read NULL on the
/// others instead of a fake value.
#[derive(Clone)]
struct TrailRow {
    /// The yes-probability, when the question is a decide.
    probability: Option<f64>,
    /// The answer's word, when the question is a decide.
    answer: Option<String>,
    /// The model the request named.
    model: String,
    /// The question's digest with its threshold.
    digest: String,
    /// The nearest level's name, for a score question; NULL for every
    /// other verb. This is the contract's settled `nearest` field, and
    /// the struct member carries that name.
    nearest: Option<String>,
    /// The wire sends that produced the judgment, when the audit door
    /// could count them.
    sends: Option<u64>,
    /// The recording digests of the logical requests that produced the
    /// judgment (0053): always a list, one element for a one-request
    /// result, in construction order; a retry adds no element.
    requests: Vec<String>,
    /// Failed logical questions in this result (0054); always present,
    /// zero on this surface by construction, because a failed single
    /// question is a whole-call error here.
    failed_questions: u32,
}

/// `thinkthen_details(question, text)`: the audit trail, with the sends
/// that produced the judgment and, for a score question, the nearest
/// level's name. A score, choose, or tag reply carries no
/// yes-probability, so for those the question's own fields carry the
/// audit and probability, answer, and sends read NULL.
struct DetailsScalar;

impl VScalar for DetailsScalar {
    type State = ();

    fn invoke(
        _: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let questions = read_strings(input, 0);
        let texts = read_strings(input, 1);
        let len = texts.len();
        let distinct = distinct_of(&questions, &[], &texts, None)?;
        let mut trail: Vec<Option<TrailRow>> = vec![None; distinct.pairs.len()];
        for (slot, (question_at, text)) in distinct.pairs.iter().enumerate() {
            let question = &distinct.questions[*question_at];
            let row = if question.kind() == QuestionKind::Decide {
                let details = engine()
                    .details_opts(question, text, options())
                    .map_err(failure)?;
                TrailRow {
                    probability: Some(details.probability),
                    answer: Some(details.answer.to_string()),
                    model: details.model,
                    digest: details.digest,
                    nearest: None,
                    sends: Some(u64::from(details.sends)),
                    requests: details.requests,
                    failed_questions: details.failed_questions,
                }
            } else {
                let nearest = if question.kind() == QuestionKind::Score {
                    Some(
                        engine()
                            .score_opts(question, text, options())
                            .map_err(failure)?
                            .nearest,
                    )
                } else {
                    None
                };
                // The requests list for a non-decide question: the
                // engine's own digest rule, the same one `details` uses
                // for decide. When the real engine lands, its details
                // carry the list for every kind and this call goes away.
                let requests = vec![thinkthen_standin::request_digest(
                    &question,
                    thinkthen_contract::Settings::from_env().model.as_deref(),
                    text,
                )
                .map_err(failure)?];
                TrailRow {
                    probability: None,
                    answer: None,
                    model: question.model().to_owned(),
                    digest: question.digest(),
                    nearest,
                    sends: None,
                    requests,
                    failed_questions: 0,
                }
            };
            trail[slot] = Some(row);
        }
        {
            let mut parent = output.flat_vector();
            for (i, slot) in distinct.slots.iter().enumerate() {
                if slot.and_then(|slot| trail[slot].as_ref()).is_none() {
                    parent.set_null(i);
                }
            }
        }
        let structs = output.struct_vector();
        {
            let mut child = structs.child(0, len);
            let values = unsafe { child.as_mut_slice_with_len::<f64>(len) };
            for (i, slot) in distinct.slots.iter().enumerate() {
                if let Some(row) = slot.and_then(|slot| trail[slot].as_ref()) {
                    if let Some(number) = row.probability {
                        values[i] = number;
                    }
                }
            }
        }
        let mut child = structs.child(1, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                Some(row) => {
                    if let Some(answer) = row.answer.as_deref() {
                        child.insert(i, answer);
                    } else {
                        child.set_null(i);
                    }
                }
                None => child.set_null(i),
            }
        }
        let mut child = structs.child(2, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                Some(row) => {
                    child.insert(i, row.model.as_str());
                }
                None => child.set_null(i),
            }
        }
        let mut child = structs.child(3, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                Some(row) => {
                    child.insert(i, row.digest.as_str());
                }
                None => child.set_null(i),
            }
        }
        let mut child = structs.child(4, len);
        for (i, slot) in distinct.slots.iter().enumerate() {
            match slot.and_then(|slot| trail[slot].as_ref()) {
                Some(row) => match row.nearest.as_deref() {
                    Some(nearest) => {
                        child.insert(i, nearest);
                    }
                    None => child.set_null(i),
                },
                None => child.set_null(i),
            }
        }
        {
            let mut child = structs.child(5, len);
            let values = unsafe { child.as_mut_slice_with_len::<u64>(len) };
            for (i, slot) in distinct.slots.iter().enumerate() {
                if let Some(row) = slot.and_then(|slot| trail[slot].as_ref()) {
                    if let Some(sends) = row.sends {
                        values[i] = sends;
                    }
                }
            }
        }
        {
            let rows: Vec<Option<&Vec<String>>> = distinct
                .slots
                .iter()
                .map(|slot| {
                    slot.and_then(|slot| trail[slot].as_ref())
                        .map(|row| &row.requests)
                })
                .collect();
            let total: usize = rows.iter().map(|row| row.map_or(0, Vec::len)).sum();
            let mut lists = structs.list_vector_child(6);
            {
                let child = lists.child(total.max(1));
                let mut offset = 0usize;
                for (i, row) in rows.iter().enumerate() {
                    match row {
                        None => lists.set_null(i),
                        Some(values) => {
                            lists.set_entry(i, offset, values.len());
                            for value in values.iter() {
                                child.insert(offset, value.as_str());
                                offset += 1;
                            }
                        }
                    }
                }
            }
            lists.set_len(total);
        }
        {
            let mut child = structs.child(7, len);
            let values = unsafe { child.as_mut_slice_with_len::<u64>(len) };
            for (i, slot) in distinct.slots.iter().enumerate() {
                if let Some(row) = slot.and_then(|slot| trail[slot].as_ref()) {
                    values[i] = u64::from(row.failed_questions);
                }
            }
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into(), LogicalTypeId::Varchar.into()],
            LogicalTypeHandle::struct_type(&[
                ("probability", LogicalTypeId::Double.into()),
                ("answer", LogicalTypeId::Varchar.into()),
                ("model", LogicalTypeId::Varchar.into()),
                ("digest", LogicalTypeId::Varchar.into()),
                ("nearest", LogicalTypeId::Varchar.into()),
                ("sends", LogicalTypeId::UBigint.into()),
                (
                    "requests",
                    LogicalTypeHandle::list(&LogicalTypeId::Varchar.into()),
                ),
                ("failed_questions", LogicalTypeId::UBigint.into()),
            ]),
        )]
    }

    fn volatile() -> bool {
        true
    }
}

/// The extension's entrypoint, hand-written where the macro-generated one
/// hides the raw connection the aggregate and the table function need.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thinkthen_init_c_api(
    info: ffi::duckdb_extension_info,
    access: *const ffi::duckdb_extension_access,
) -> bool {
    unsafe { init(info, access).is_ok() }
}

unsafe fn init(
    info: ffi::duckdb_extension_info,
    access: *const ffi::duckdb_extension_access,
) -> Result<(), Box<dyn Error>> {
    unsafe { install_interrupt_handler() };
    if !unsafe { ffi::duckdb_rs_extension_api_init(info, access, "v1.5.5") }? {
        return Ok(());
    }
    let get_database = unsafe { (*access) }
        .get_database
        .ok_or("the extension access names no get_database")?;
    let database = unsafe { *get_database(info) };
    let connection = unsafe { Connection::open_from_raw(database) }?;
    connection.register_scalar_function::<DecideScalar>("thinkthen_decide")?;
    connection.register_scalar_function::<ProbabilityScalar>("thinkthen_probability")?;
    connection.register_scalar_function::<ChooseScalar>("thinkthen_choose")?;
    connection.register_scalar_function::<ScoreScalar>("thinkthen_score")?;
    connection.register_scalar_function::<TagScalar>("thinkthen_tag")?;
    connection.register_scalar_function::<AnnotateScalar>("thinkthen_annotate")?;
    connection.register_scalar_function::<DetailsScalar>("thinkthen_details")?;
    connection.register_scalar_function::<recognize::RecognizeScalar>("thinkthen_recognize")?;
    connection.register_scalar_function::<relations::RelationsScalar>("thinkthen_relations")?;

    // The relate table function runs the query it is given. The database
    // pointer `get_database` hands out belongs to the load state and can
    // dangle after init, so the extension keeps this connection open for
    // the process lifetime and runs the relate query on it instead.
    let mut raw: ffi::duckdb_connection = std::ptr::null_mut();
    if unsafe { ffi::duckdb_connect(database, &mut raw) } != ffi::DuckDBSuccess {
        return Err("the raw connection refused".into());
    }
    let table = unsafe { usage::register(raw) };
    let aggregate = unsafe { warm::register(raw) };
    let relate_function = unsafe { relate::register(raw) };
    table?;
    aggregate?;
    relate_function?;
    relate::remember_connection(raw);
    Ok(())
}

#[cfg(test)]
mod mapping_tests {
    use super::*;

    /// Ruling 2 of the product rulings: the defect kind maps to this
    /// engine's own error surface, with the kind named in the message. No
    /// public door carries a fault hook; this proves the mapping at the
    /// shim level.
    #[test]
    fn the_defect_kind_maps_to_the_engines_error() {
        let text = failure(EngineError::defect("the relate plan lost its bind data"));
        assert!(text.contains("thinkthen defect:"), "{text}");
        assert!(text.contains("the relate plan lost its bind data"), "{text}");
    }
}

#[cfg(test)]
mod cancel_tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    /// The fast-backend cancel proof (lane B item 5, the poll-bug shape)
    /// at the engine this surface calls: on the null backend the wait's
    /// channel never idles, so the poll tick must run on the busy arm too
    /// and a token set mid-batch must end the call within about a tick —
    /// not after the whole batch, which is what the bug did (SIGINT one
    /// second into a three-million-record null batch surfaced 8.48 s
    /// later, at batch end). Width 1 stretches the full batch so the
    /// contrast is unambiguous: the broken path drains it all.
    ///
    /// This lives here and not in a CLI script because DuckDB's own
    /// abort ends a native query on SIGINT within a chunk, so a CLI
    /// timing test cannot isolate the engine's tick on a fast backend;
    /// the stub-backed wire suite proves the wire-side shape.
    #[test]
    fn a_fast_backend_hears_a_cancel_within_a_tick() {
        // Before any engine or config is built in this process.
        unsafe {
            std::env::set_var("ENGINE_NULL", "1");
            std::env::set_var("ENGINE_WIDTH", "1");
        }
        let engine = BlockingEngine::from_env();
        let question = Question::from_json(r#"{"decide":"Is this a complaint?"}"#)
            .expect("the question parses");
        let texts: Vec<String> = (0..1_000_000).map(|i| format!("record {i}")).collect();
        let records: Vec<&str> = (0..8_000_000).map(|i| texts[i % texts.len()].as_str()).collect();
        let token = Cancel::new();
        let fired = Arc::new(Mutex::new(None::<Instant>));
        let setter = {
            let token = token.clone();
            let fired = Arc::clone(&fired);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(150));
                token.cancel();
                *fired.lock().expect("the fired stamp") = Some(Instant::now());
            })
        };
        let outcome = engine.decide_many_opts(
            &question,
            &records,
            Options::new().cancel(&token),
            None,
        );
        setter.join().expect("the setter joins");
        let fired_at = fired
            .lock()
            .expect("the fired stamp")
            .expect("the token fired");
        let heard = Instant::now().duration_since(fired_at);
        let error = outcome.expect_err("a cancelled batch returns the cancelled kind");
        assert_eq!(error.kind.to_string(), "cancelled", "{}", error.message);
        assert!(
            heard <= Duration::from_millis(1_500),
            "the interrupt waited {heard:?} past the token; a fast backend starved the poll"
        );
    }
}
