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
//! run on R's main thread, and it may jump out of the call frame; a jump
//! across Rust frames is undefined behavior, so the jump is never allowed
//! to cross one: the check runs inside `R_ToplevelExec`, which catches it
//! and reports the pending interrupt instead. Every engine call runs on a
//! fresh plain worker thread while the main thread waits on a channel and
//! runs that guarded check every 100 ms; no thread is held between calls.
//! R objects may only be touched on the main thread, so the worker returns
//! plain Rust data and the main thread builds the R values from it.
//!
//! A pending interrupt stops the call where it stands: the token is
//! cancelled (no new request starts, and the requests already sent
//! finish), the call returns the interrupt marker, and the R half raises
//! R's own interrupt condition from it. The checks before the call and
//! after it catch an interrupt that landed while R was busy elsewhere, and
//! R's `on.exit` cleanup (`.tt_cleanup`, which calls `tt_cancel_active`)
//! still stops any call whose token is registered.
//!
//! Errors cross as one string, `kind`, a marker, `retryable`, a marker,
//! and the message, so the R half can raise a condition carrying all
//! three.

use extendr_api::prelude::*;
use std::ffi::c_void;
use std::path::Path;
use std::result::Result as StdResult;
use std::sync::{Arc, LazyLock, Mutex, MutexGuard, mpsc};
use std::time::Duration;

use thinkthen_contract::{
    Annotated, Answer, Cancel, Connector, Engine, EngineConfig, Options, Question, QuestionSet,
    Recognize, Recognized, Relate, deadline_from_seconds, relate_checked,
};
use thinkthen_standin::StandinConnector;

/// How long the main thread waits between interrupt checks. The engine's
/// own wait tick is the same length, so a stop gesture lands within one
/// tick on either side.
const POLL_MS: u64 = 100;

/// The separator that carries an error's kind and retry signal to R.
const ERROR_SEP: &str = "\u{1f}";

// R's own interrupt check, and the top-level context that catches its
// jump. The check must run on the main thread.
extern "C" {
    fn R_CheckUserInterrupt();
    fn R_ToplevelExec(check: extern "C" fn(*mut c_void), data: *mut c_void) -> i32;
}

/// The one engine value, built through the contract's connector door at the
/// first call. The stand-in is the connector named today; pointing the
/// surface at the real engine changes this line alone.
static ENGINE: LazyLock<StdResult<Arc<dyn Engine>, String>> = LazyLock::new(|| {
    StandinConnector.connect(&EngineConfig::from_env()).map_err(carry)
});

/// The engine value, or the connector's own failure, built once.
fn engine() -> StdResult<Arc<dyn Engine>, String> {
    ENGINE.clone()
}

/// The cancel token of the call in flight, so a stop gesture reaches it
/// from the call's own thread. R runs one call at a time, so one slot
/// serves. A poisoned lock is recovered rather than refused: the value
/// inside is a token, and a token read after a panic is still a token.
static ACTIVE: Mutex<Option<Cancel>> = Mutex::new(None);

/// The slot, recovered from a panic that poisoned it.
fn active_slot() -> MutexGuard<'static, Option<Cancel>> {
    ACTIVE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Forget the call in flight, once its token is spent or cancelled.
fn clear_active() {
    *active_slot() = None;
}

/// An engine error as the one string the R half parses.
fn carry(error: thinkthen_contract::Error) -> String {
    format!("{}{ERROR_SEP}{}{ERROR_SEP}{}", error.kind, error.retryable, error.message)
}

/// The marker the R half reads as R's own interrupt condition, packed like
/// every other error: the kind word, the retry signal, and the message.
fn interrupt_carried() -> String {
    format!("interrupt{ERROR_SEP}false{ERROR_SEP}the call was interrupted")
}

/// Whether R holds a pending interrupt, checked without letting R's jump
/// cross a Rust frame: `R_CheckUserInterrupt` runs inside `R_ToplevelExec`,
/// which catches the jump and answers FALSE, so this frame stays whole and
/// the interrupt is reported instead of unwound.
fn interrupt_pending() -> bool {
    extern "C" fn check(_: *mut c_void) {
        unsafe { R_CheckUserInterrupt() }
    }
    unsafe { R_ToplevelExec(check, std::ptr::null_mut()) == 0 }
}

/// One engine call on a fresh worker thread, R's guarded interrupt check on
/// the main thread while it runs, a fresh cancel token registered for the
/// stop path, and plain data back for the main thread to convert.
fn call<T>(
    deadline: Option<f64>,
    work: impl for<'a> FnOnce(&dyn Engine, Options<'a>) -> StdResult<T, thinkthen_contract::Error>
        + Send
        + 'static,
) -> StdResult<T, String>
where
    T: Send + 'static,
{
    // A pending interrupt stops the call before anything is sent.
    if interrupt_pending() {
        return Err(interrupt_carried());
    }
    // The contract owns the one checked conversion of a host's deadline: a
    // NaN, a negative other than the sentinel, or an oversized budget comes
    // back as the usage kind before any thread exists. The sentinel and
    // `None` both mean no deadline, and zero stays the spent deadline.
    let budget = deadline
        .map(deadline_from_seconds)
        .transpose()
        .map_err(carry)?
        .flatten();
    let engine = engine()?;
    let token = Cancel::new();
    *active_slot() = Some(token.clone());
    let worker_token = token.clone();
    let (sender, receiver) = mpsc::channel::<StdResult<T, thinkthen_contract::Error>>();
    let worker = match std::thread::Builder::new().name("tt-r-call".to_owned()).spawn(move || {
        // The token this call registered rides every engine call, so a
        // stop gesture reaches the batch, and the deadline (when the
        // caller named one) is handed on to return the deadline kind
        // naming the budget.
        let mut options = Options::new().cancel(&worker_token);
        if let Some(budget) = budget {
            options = options.deadline_in(budget);
        }
        let _ = sender.send(work(&*engine, options));
    }) {
        Ok(worker) => worker,
        Err(error) => {
            clear_active();
            return Err(carry(thinkthen_contract::Error::local(format!(
                "the call's worker thread did not start: {error}"
            ))));
        }
    };
    let answer = loop {
        match receiver.recv_timeout(Duration::from_millis(POLL_MS)) {
            Ok(answer) => break answer,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if interrupt_pending() {
                    // Stop the call where it stands: no new request starts,
                    // and the requests already sent finish. The worker
                    // holds its own token clone, so the slot is cleared and
                    // the interrupt is reported without waiting for it.
                    token.cancel();
                    clear_active();
                    return Err(interrupt_carried());
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                let _ = worker.join();
                break Err(thinkthen_contract::Error::defect(
                    "the call's worker ended without an answer",
                ));
            }
        }
    };
    // The call is done, so its token is spent; clear the slot so a later
    // cleanup cancels nothing.
    clear_active();
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
    /// The ruled failed-question marker (0054): the kind and the closed
    /// cause, carried as this host's named list.
    Failed(String, String),
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
    deadline: Option<f64>,
) -> StdResult<List, String> {
    let question = take(question);
    let (codes, probabilities) = call(deadline, move |engine, options| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.decide_many_opts(&question, &slices, options, None).map(|judgments| {
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
fn tt_decide_one(question: ExternalPtr<Question>, evidence: String, deadline: Option<f64>) -> StdResult<List, String> {
    let question = take(question);
    let code = call(deadline, move |engine, options| {
        engine.decide_opts(&question, &evidence, options).map(answer_code)
    })?;
    Ok(list!(ans = code_robj(code)))
}

/// One `choose`: the winning option, or NULL when unsure.
#[extendr]
fn tt_choose_one(
    question: ExternalPtr<Question>,
    evidence: String,
    deadline: Option<f64>,
) -> StdResult<Nullable<String>, String> {
    let question = take(question);
    let pick = call(deadline, move |engine, options| {
        engine.choose_opts(&question, &evidence, options)
    })?;
    Ok(match pick {
        Some(named) => Nullable::NotNull(named),
        None => Nullable::Null,
    })
}

/// One `score`: the weighted position and the nearest level.
#[extendr]
fn tt_score_one(question: ExternalPtr<Question>, evidence: String, deadline: Option<f64>) -> StdResult<List, String> {
    let question = take(question);
    let (value, nearest) = call(deadline, move |engine, options| {
        engine
            .score_opts(&question, &evidence, options)
            .map(|scored| (scored.value, scored.nearest))
    })?;
    Ok(list!(value = value, nearest = nearest))
}

/// One `tag`: the labels that held, in the question's order.
#[extendr]
fn tt_tag_one(question: ExternalPtr<Question>, evidence: String, deadline: Option<f64>) -> StdResult<Vec<String>, String> {
    let question = take(question);
    call(deadline, move |engine, options| engine.tag_opts(&question, &evidence, options))
}

/// `filter` over records: the places, one-based, whose evidence held.
#[extendr]
fn tt_filter_places(
    question: ExternalPtr<Question>,
    records: Vec<String>,
    deadline: Option<f64>,
) -> StdResult<Vec<i32>, String> {
    let question = take(question);
    call(deadline, move |engine, options| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine
            .filter_opts(&question, &slices, options, None)
            .map(|places| places.iter().map(|place| *place as i32 + 1).collect())
    })
}

/// `rank` over records: places in rank order with their probabilities.
#[extendr]
fn tt_rank_all(question: ExternalPtr<Question>, records: Vec<String>, deadline: Option<f64>) -> StdResult<List, String> {
    let question = take(question);
    let (places, probabilities) = call(deadline, move |engine, options| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.rank_opts(&question, &slices, options, None).map(|ranked| {
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
fn tt_find_one(question: ExternalPtr<Question>, units: Vec<String>, deadline: Option<f64>) -> StdResult<List, String> {
    let question = take(question);
    let (place, probability) = call(deadline, move |engine, options| {
        let slices: Vec<&str> = units.iter().map(String::as_str).collect();
        engine
            .find_opts(&question, &slices, options)
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
fn tt_annotate_file(path: String, records: Vec<String>, deadline: Option<f64>) -> StdResult<List, String> {
    let rows: Vec<Vec<(String, Field)>> = call(deadline, move |engine, options| {
        let set = QuestionSet::from_file(Path::new(&path))?;
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        engine.annotate_opts(&set, &slices, options, None).map(|rows| {
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
                                Annotated::Failed(failed) => {
                                    Field::Failed(kind_word(failed.kind), cause_word(failed.cause))
                                }
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
                    // The marker as this host's own named list:
                    // `list(failed = list(kind = ..., cause = ...))`.
                    Field::Failed(kind, cause) => {
                        list!(failed = list!(kind = kind, cause = cause)).into()
                    }
                })
                .collect();
            List::from_names_and_values(names, values)
                .expect("one annotate row as a list")
        })
        .collect();
    Ok(List::from_values(built))
}

/// The question set's names and kinds, in the set's own name order, so the
/// R half types each answer column by its question rather than by the first
/// answer it happens to see.
#[extendr]
fn tt_annotate_kinds(path: String) -> StdResult<List, String> {
    let set = QuestionSet::from_file(Path::new(&path)).map_err(carry)?;
    let kinds: Vec<String> = set
        .questions()
        .iter()
        .map(|question| question.kind().to_string())
        .collect();
    Ok(list!(names = set.names().to_vec(), kinds = kinds))
}

/// The failure kind's own word; `backend` today.
fn kind_word(kind: thinkthen_contract::FailureKind) -> String {
    use thinkthen_contract::FailureKind;
    match kind {
        FailureKind::Backend => "backend".to_owned(),
    }
}

/// The closed cause list's own words, spelled once.
fn cause_word(cause: thinkthen_contract::Cause) -> String {
    use thinkthen_contract::Cause;
    match cause {
        Cause::MissingAnswer => "missing_answer".to_owned(),
        Cause::WrongKind => "wrong_kind".to_owned(),
        Cause::MissingProbability => "missing_probability".to_owned(),
        Cause::InvalidProbability => "invalid_probability".to_owned(),
        Cause::InvalidDistribution => "invalid_distribution".to_owned(),
        Cause::UnexpectedProbability => "unexpected_probability".to_owned(),
    }
}

/// The audit view of one judgment, with the logical requests' digests
/// (0053) and the failed-question count (0054).
#[extendr]
fn tt_details_one(question: ExternalPtr<Question>, evidence: String, deadline: Option<f64>) -> StdResult<List, String> {
    let question = take(question);
    let (probability, code, model, digest, sends, requests, failed_questions, nearest) =
        call(deadline, move |engine, options| {
            engine
                .details_opts(&question, &evidence, options)
                .map(|details| {
                    (
                        details.probability,
                        answer_code(details.answer),
                        details.model,
                        details.digest,
                        details.sends,
                        details.requests,
                        details.failed_questions,
                        details.nearest,
                    )
                })
        })?;
    let held: Robj = match nearest {
        Some(level) => Robj::from(level),
        None => Robj::from(Nullable::<&str>::Null),
    };
    Ok(list!(
        probability = probability,
        answer = code_robj(code),
        model = model,
        digest = digest,
        sends = sends,
        requests = requests,
        failed_questions = failed_questions,
        nearest = held
    ))
}

/// The process counters.
#[extendr]
fn tt_usage_counters() -> StdResult<List, String> {
    let usage = engine()?.usage();
    Ok(list!(
        requests = usage.requests,
        cache_answers = usage.cache_answers,
        tokens = usage.tokens
    ))
}

/// Whether R holds a pending interrupt, for the R half's own checks before
/// and after a call.
#[extendr]
fn tt_interrupt_pending() -> bool {
    interrupt_pending()
}

/// Stop the call in flight, when an interrupt jumped out of its frame.
#[extendr]
fn tt_cancel_active() -> bool {
    match active_slot().take() {
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
    deadline: Option<f64>,
) -> StdResult<List, String> {
    let ask = (*ask).clone();
    let answers: Vec<Recognized> = call(deadline, move |engine, options| {
        texts
            .iter()
            .map(|text| engine.recognize_opts(&ask, text, options))
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
    deadline: Option<f64>,
) -> StdResult<List, String> {
    let ask = (*ask).clone();
    let edges = call(deadline, move |engine, options| {
        let slices: Vec<&str> = records.iter().map(String::as_str).collect();
        relate_checked(engine, &ask, &slices, options)
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
    fn tt_annotate_kinds;
    fn tt_details_one;
    fn tt_usage_counters;
    fn tt_interrupt_pending;
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

    #[test]
    fn the_interrupt_marker_packs_as_an_interrupt() {
        let carried = interrupt_carried();
        let parts: Vec<&str> = carried.split(ERROR_SEP).collect();
        assert_eq!(parts, ["interrupt", "false", "the call was interrupted"]);
    }
}
