//! The R shim over the contract engine.
//!
//! This crate is the R surface of ADR 0017: it converts R arguments into
//! contract values, calls one engine function, converts the answer back,
//! and raises the engine's failures as R errors. No rule, no retry, and no
//! sending lives here. The stand-in implements the contract today; the
//! real engine implements it after the merge, and the move changes the two
//! dependency lines in this crate's manifest.
//!
//! R's own disciplines shape the threading. `R_CheckUserInterrupt` must
//! run on R's main thread, and it may jump out of the call frame, so every
//! engine call runs on a fresh plain worker thread while the main thread
//! waits on a channel and checks for Ctrl-C every 100 ms; no thread is
//! held between calls. R objects may only be touched on the main thread,
//! so the worker returns plain Rust data and the main thread builds the
//! R values from it. When the check jumps, R's `on.exit` cleanup
//! (`.tt_cleanup`, which calls `tt_cancel_active`) sets the call's cancel
//! token, and the engine keeps its promise: no new request starts, and the
//! requests already sent finish.
//!
//! Errors cross as one string, `kind`, a marker, `retryable`, a marker,
//! and the message, so the R half can raise a condition carrying all
//! three.

use extendr_api::prelude::*;
use std::path::Path;
use std::result::Result as StdResult;
use std::sync::{LazyLock, Mutex, mpsc};
use std::time::Duration;

use thinkthen_contract::{
    Annotated, Answer, Cancel, Engine, Options, Question, QuestionSet, Recognize, Recognized,
    Relate,
};
use thinkthen_standin::BlockingEngine;

/// How long the main thread waits between interrupt checks. The engine's
/// own wait tick is the same length, so a stop gesture lands within one
/// tick on either side.
const POLL_MS: u64 = 100;

/// The separator that carries an error's kind and retry signal to R.
const ERROR_SEP: &str = "\u{1f}";

// R's own interrupt check. It must run on the main thread, and it may jump
// out of this frame when the user pressed Ctrl-C.
extern "C" {
    fn R_CheckUserInterrupt();
}

/// The one engine value, built from the environment at the first call.
static ENGINE: LazyLock<BlockingEngine> = LazyLock::new(BlockingEngine::from_env);

/// The cancel token of the call in flight, so R's `on.exit` cleanup can
/// stop the batch after an interrupt jumped out of the frame. R runs one
/// call at a time, so one slot serves.
static ACTIVE: Mutex<Option<Cancel>> = Mutex::new(None);

/// An engine error as the one string the R half parses.
fn carry(error: thinkthen_contract::Error) -> String {
    format!("{}{ERROR_SEP}{}{ERROR_SEP}{}", error.kind, error.retryable, error.message)
}

/// One engine call on a fresh worker thread, R's interrupt check on the
/// main thread while it runs, a fresh cancel token registered for the
/// cleanup path, and plain data back for the main thread to convert.
fn call<T>(work: impl FnOnce(&BlockingEngine) -> StdResult<T, thinkthen_contract::Error> + Send + 'static) -> StdResult<T, String>
where
    T: Send + 'static,
{
    let token = Cancel::new();
    {
        let mut held = ACTIVE.lock().expect("the cancel slot locks");
        *held = Some(token);
    }
    let (sender, receiver) = mpsc::channel::<StdResult<T, thinkthen_contract::Error>>();
    let worker = std::thread::Builder::new()
        .name("tt-r-call".to_owned())
        .spawn(move || {
            let _ = sender.send(work(&ENGINE));
        })
        .expect("the call's worker thread starts");
    let answer = loop {
        match receiver.recv_timeout(Duration::from_millis(POLL_MS)) {
            Ok(answer) => break answer,
            Err(mpsc::RecvTimeoutError::Timeout) => unsafe { R_CheckUserInterrupt() },
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                let _ = worker.join();
                break Err(thinkthen_contract::Error::defect(
                    "the call's worker ended without an answer",
                ));
            }
        }
    };
    // The call is done, so its token is spent; clear the slot so a later
    // cleanup cancels nothing. After an interrupt jump this line never
    // runs, and the registered token is exactly what the cleanup wants.
    {
        let mut held = ACTIVE.lock().expect("the cancel slot locks");
        *held = None;
    }
    answer.map_err(carry)
}

/// The answer enum as plain data: 1 yes, 0 no, -1 unsure.
fn answer_code(answer: Answer) -> i32 {
    match answer {
        Answer::Yes => 1,
        Answer::No => 0,
        Answer::Unsure => -1,
    }
}

/// The code as R sees it: 1L, 0L, or NA.
fn code_robj(code: i32) -> Robj {
    if code < 0 {
        Robj::from(Nullable::<i32>::Null)
    } else {
        Robj::from(code)
    }
}

/// Take the question out of its R pointer, so a worker thread may hold it.
fn take(question: ExternalPtr<Question>) -> Question {
    (*question).clone()
}

/// One field of an annotate answer, as plain data.
#[derive(Clone)]
enum Field {
    Decision(i32),
    Choice(Option<String>),
    Score(f64, String),
    Tags(Vec<String>),
}

/// Build a question through the one file grammar, any verb, any threshold.
#[extendr]
fn tt_question_grammared(body: String) -> StdResult<ExternalPtr<Question>, String> {
    let question = Question::from_json(&body).map_err(carry)?;
    Ok(ExternalPtr::new(question))
}

/// `decide` over a column: one crossing, every judgment in input order.
#[extendr]
fn tt_decide_column(
    question: ExternalPtr<Question>,
    records: Vec<String>,
) -> StdResult<List, String> {
    let question = take(question);
    let (codes, probabilities) = call(move |engine| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.decide_many_opts(&question, &slices, Options::new(), None).map(|judgments| {
            let codes: Vec<i32> =
                judgments.iter().map(|held| answer_code(held.answer)).collect();
            let probabilities: Vec<f64> =
                judgments.iter().map(|held| held.probability).collect();
            (codes, probabilities)
        })
    })?;
    let answers: Vec<Robj> = codes.iter().map(|code| code_robj(*code)).collect();
    Ok(list!(ans = answers, prob = probabilities))
}

/// One `decide` of one evidence.
#[extendr]
fn tt_decide_one(question: ExternalPtr<Question>, evidence: String) -> StdResult<List, String> {
    let question = take(question);
    let code = call(move |engine| {
        engine.decide_opts(&question, &evidence, Options::new()).map(answer_code)
    })?;
    Ok(list!(ans = code_robj(code)))
}

/// One `choose`: the winning option, or NULL when unsure.
#[extendr]
fn tt_choose_one(
    question: ExternalPtr<Question>,
    evidence: String,
) -> StdResult<Nullable<String>, String> {
    let question = take(question);
    let pick = call(move |engine| {
        engine.choose_opts(&question, &evidence, Options::new())
    })?;
    Ok(match pick {
        Some(named) => Nullable::NotNull(named),
        None => Nullable::Null,
    })
}

/// One `score`: the weighted position and the nearest level.
#[extendr]
fn tt_score_one(question: ExternalPtr<Question>, evidence: String) -> StdResult<List, String> {
    let question = take(question);
    let (value, nearest) = call(move |engine| {
        engine
            .score_opts(&question, &evidence, Options::new())
            .map(|scored| (scored.value, scored.nearest))
    })?;
    Ok(list!(value = value, nearest = nearest))
}

/// One `tag`: the labels that held, in the question's order.
#[extendr]
fn tt_tag_one(question: ExternalPtr<Question>, evidence: String) -> StdResult<Vec<String>, String> {
    let question = take(question);
    call(move |engine| engine.tag_opts(&question, &evidence, Options::new()))
}

/// `filter` over records: the places, one-based, whose evidence held.
#[extendr]
fn tt_filter_places(
    question: ExternalPtr<Question>,
    records: Vec<String>,
) -> StdResult<Vec<i32>, String> {
    let question = take(question);
    call(move |engine| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine
            .filter_opts(&question, &slices, Options::new(), None)
            .map(|places| places.iter().map(|place| *place as i32 + 1).collect())
    })
}

/// `rank` over records: places in rank order with their probabilities.
#[extendr]
fn tt_rank_all(question: ExternalPtr<Question>, records: Vec<String>) -> StdResult<List, String> {
    let question = take(question);
    let (places, probabilities) = call(move |engine| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.rank_opts(&question, &slices, Options::new(), None).map(|ranked| {
            let places: Vec<i32> = ranked.iter().map(|held| held.index as i32 + 1).collect();
            let probabilities: Vec<f64> =
                ranked.iter().map(|held| held.probability).collect();
            (places, probabilities)
        })
    })?;
    Ok(list!(place = places, prob = probabilities))
}

/// `find` over units: the winner's place, one-based, and its probability.
#[extendr]
fn tt_find_one(question: ExternalPtr<Question>, units: Vec<String>) -> StdResult<List, String> {
    let question = take(question);
    let (place, probability) = call(move |engine| {
        let slices: Vec<&str> = units.iter().map(String::as_str).collect();
        engine
            .find_opts(&question, &slices, Options::new())
            .map(|found| (found.index, found.probability))
    })?;
    let held: Robj = match place {
        Some(index) => Robj::from(index as f64 + 1.0),
        None => Robj::from(Nullable::<f64>::Null),
    };
    Ok(list!(place = held, prob = probability))
}

/// `annotate` over records from a question set file: each row a named list
/// in the set's own order, each field typed by its question's verb.
#[extendr]
fn tt_annotate_file(path: String, records: Vec<String>) -> StdResult<List, String> {
    let rows: Vec<Vec<(String, Field)>> = call(move |engine| {
        let set = QuestionSet::from_file(Path::new(&path))?;
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.annotate_opts(&set, &slices, Options::new(), None).map(|rows| {
            rows.iter()
                .map(|fields| {
                    fields
                        .iter()
                        .map(|(name, field)| {
                            let held = match field {
                                Annotated::Decision(answer) => Field::Decision(answer_code(*answer)),
                                Annotated::Choice(pick) => Field::Choice(pick.clone()),
                                Annotated::Score(scored) => {
                                    Field::Score(scored.value, scored.nearest.clone())
                                }
                                Annotated::Tags(labels) => Field::Tags(labels.clone()),
                            };
                            (name.clone(), held)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
    })?;
    let built: Vec<List> = rows
        .iter()
        .map(|fields| {
            let names: Vec<&str> = fields.iter().map(|(name, _)| name.as_str()).collect();
            let values: Vec<Robj> = fields
                .iter()
                .map(|(_, field)| match field {
                    Field::Decision(code) => code_robj(*code),
                    Field::Choice(pick) => match pick {
                        Some(named) => Robj::from(named.as_str()),
                        None => Robj::from(Nullable::<String>::Null),
                    },
                    Field::Score(value, nearest) => list!(value = value, nearest = nearest).into(),
                    Field::Tags(labels) => {
                        Robj::from(labels.iter().map(String::as_str).collect::<Vec<_>>())
                    }
                })
                .collect();
            List::from_names_and_values(names, values)
                .expect("one annotate row as a list")
        })
        .collect();
    Ok(List::from_values(built))
}

/// The audit view of one judgment.
#[extendr]
fn tt_details_one(question: ExternalPtr<Question>, evidence: String) -> StdResult<List, String> {
    let question = take(question);
    let (probability, code, model, digest, sends) = call(move |engine| {
        engine
            .details_opts(&question, &evidence, Options::new())
            .map(|details| {
                (
                    details.probability,
                    answer_code(details.answer),
                    details.model,
                    details.digest,
                    details.sends,
                )
            })
    })?;
    Ok(list!(
        probability = probability,
        answer = code_robj(code),
        model = model,
        digest = digest,
        sends = sends
    ))
}

/// The process counters.
#[extendr]
fn tt_usage_counters() -> List {
    let usage = ENGINE.usage();
    list!(
        requests = usage.requests,
        cache_answers = usage.cache_answers,
        tokens = usage.tokens
    )
}

/// Stop the call in flight, when an interrupt jumped out of its frame.
#[extendr]
fn tt_cancel_active() -> bool {
    let mut held = ACTIVE.lock().expect("the cancel slot locks");
    match held.take() {
        Some(token) => {
            token.cancel();
            true
        }
        None => false,
    }
}

/// The digest of a question value, for the audit pages.
#[extendr]
fn tt_question_digest(question: ExternalPtr<Question>) -> String {
    question.digest()
}

/// The kind and members of a question value, for printing.
#[extendr]
fn tt_question_parts(question: ExternalPtr<Question>) -> List {
    list!(
        kind = question.kind().to_string(),
        text = question.text(),
        members = question.members(),
        threshold_named = question.threshold_named()
    )
}

/// Build a recognize ask through the question file's own section grammar.
#[extendr]
fn tt_recognize_grammared(spec: String) -> StdResult<ExternalPtr<Recognize>, String> {
    let ask = Recognize::from_json(&spec).map_err(carry)?;
    Ok(ExternalPtr::new(ask))
}

/// Build a relate ask through the question file's own section grammar.
#[extendr]
fn tt_relate_grammared(spec: String) -> StdResult<ExternalPtr<Relate>, String> {
    let ask = Relate::from_json(&spec).map_err(carry)?;
    Ok(ExternalPtr::new(ask))
}

/// `recognize` over a column: one crossing, every record in input order.
///
/// Each record becomes the plain data for a data frame of names — text,
/// kind, and the offsets in R's own string indexing, so `substr(text,
/// start, end)` is the name — plus the relations when any were found.
#[extendr]
fn tt_recognize_column(
    ask: ExternalPtr<Recognize>,
    texts: Vec<String>,
) -> StdResult<List, String> {
    let ask = (*ask).clone();
    let answers: Vec<Recognized> = call(move |engine| {
        texts
            .iter()
            .map(|text| engine.recognize(&ask, text))
            .collect::<StdResult<Vec<_>, thinkthen_contract::Error>>()
    })?;
    let entries: Vec<Robj> = answers
        .iter()
        .map(|found| {
            let text: Vec<&str> = found.entities.iter().map(|entity| entity.text.as_str()).collect();
            let kind: Vec<&str> = found.entities.iter().map(|entity| entity.kind.as_str()).collect();
            // R's own string indexing: substr counts code points from one,
            // so start is one-based and end is inclusive.
            let start: Vec<i32> = found.entities.iter().map(|entity| entity.start as i32 + 1).collect();
            let end: Vec<i32> = found.entities.iter().map(|entity| entity.end as i32).collect();
            let strength: Vec<f64> = found.entities.iter().map(|entity| entity.strength).collect();
            let relations: Robj = if found.relations.is_empty() {
                Robj::from(Nullable::<i32>::Null)
            } else {
                let name: Vec<&str> = found.relations.iter().map(|held| held.name.as_str()).collect();
                let source: Vec<i32> = found.relations.iter().map(|held| held.source as i32).collect();
                let target: Vec<i32> = found.relations.iter().map(|held| held.target as i32).collect();
                let probability: Vec<f64> =
                    found.relations.iter().map(|held| held.probability).collect();
                list!(
                    name = Robj::from(name),
                    source = source,
                    target = target,
                    probability = probability
                )
                .into()
            };
            list!(
                text = Robj::from(text),
                kind = Robj::from(kind),
                start = start,
                end = end,
                strength = strength,
                relations = relations
            )
            .into()
        })
        .collect();
    Ok(List::from_values(entries))
}

/// `relate` over records: every record crosses at once; the answer is the
/// plain data for a data frame of edges, with the kind fields when a rule
/// named kinds.
#[extendr]
fn tt_relate_records(
    ask: ExternalPtr<Relate>,
    records: Vec<String>,
) -> StdResult<List, String> {
    let ask = (*ask).clone();
    let edges = call(move |engine| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.relate(&ask, &slices)
    })?;
    let name: Vec<&str> = edges.iter().map(|edge| edge.name.as_str()).collect();
    let source: Vec<i32> = edges.iter().map(|edge| edge.source as i32).collect();
    let target: Vec<i32> = edges.iter().map(|edge| edge.target as i32).collect();
    let probability: Vec<f64> = edges.iter().map(|edge| edge.probability).collect();
    let named_kinds = edges
        .iter()
        .any(|edge| edge.source_kind.is_some() || edge.target_kind.is_some());
    if named_kinds {
        let source_kind: Vec<&str> = edges
            .iter()
            .map(|edge| edge.source_kind.as_deref().unwrap_or(""))
            .collect();
        let target_kind: Vec<&str> = edges
            .iter()
            .map(|edge| edge.target_kind.as_deref().unwrap_or(""))
            .collect();
        Ok(list!(
            name = Robj::from(name),
            source = source,
            target = target,
            probability = probability,
            source_kind = Robj::from(source_kind),
            target_kind = Robj::from(target_kind)
        ))
    } else {
        Ok(list!(
            name = Robj::from(name),
            source = source,
            target = target,
            probability = probability
        ))
    }
}

// Macro to generate exports.
extendr_module! {
    mod thinkthen;
    fn tt_question_grammared;
    fn tt_recognize_grammared;
    fn tt_relate_grammared;
    fn tt_recognize_column;
    fn tt_relate_records;
    fn tt_decide_column;
    fn tt_decide_one;
    fn tt_choose_one;
    fn tt_score_one;
    fn tt_tag_one;
    fn tt_find_one;
    fn tt_filter_places;
    fn tt_rank_all;
    fn tt_annotate_file;
    fn tt_details_one;
    fn tt_usage_counters;
    fn tt_cancel_active;
    fn tt_question_digest;
    fn tt_question_parts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defect_kind_crosses_as_its_own_kind() {
        let carried = carry(thinkthen_contract::Error::defect(
            "the engine broke its own contract",
        ));
        let parts: Vec<&str> = carried.split(ERROR_SEP).collect();
        assert_eq!(parts, ["defect", "false", "the engine broke its own contract"]);
    }
}
