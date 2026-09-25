//! The worker, the wait, and each verb's plain-data answer as R values.
//!
//! Every call runs on a fresh worker thread that owns its inputs, its
//! engine clone, and a clone of the call's cancel token. The main thread
//! waits on a channel in 100 ms ticks and runs R's interrupt check at each
//! one. A pending interrupt cancels the token and returns the marker at
//! once, so a request already on the wire never holds the caller. The
//! detached worker then sends nothing new and finishes what it sent.

use std::any::Any;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread;
use std::time::Duration;

use extendr_api::prelude::*;
use std::result::Result;
use thinkthen::{
    Annotated, Answer, CallOptions, CancelToken, DecisionQuestion, Engine, Error, Evidence,
    FailureCause, LoadedQuestion, Question, QuestionSet,
};

use crate::{carry, defect, engine, interrupted, usage};

/// How long the main thread waits between R's interrupt checks.
const TICK: Duration = Duration::from_millis(100);

/// R's guarded interrupt check, run on the main thread.
pub(crate) type Pending<'a> = &'a dyn Fn() -> bool;

/// A shim result: R's value, or the packed failure.
pub(crate) type Crossed<T> = Result<T, String>;

/// Cancels the call's token on every way out of the wait, so a detached
/// worker starts no new request after its caller has left.
struct Stop(CancelToken);

impl Drop for Stop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

/// One engine call on a fresh worker, with the checks ADR 0042 places
/// before the spawn and at each tick. A refused deadline sends nothing,
/// because it is checked here before any thread starts.
pub(crate) fn call<T: Send + 'static>(
    deadline: Option<f64>,
    pending: Pending<'_>,
    work: impl FnOnce(&Engine, CallOptions<'_>) -> Result<T, Error> + Send + 'static,
) -> Crossed<T> {
    if pending() {
        return Err(interrupted());
    }
    options(&CancelToken::new(), deadline).map_err(|error| carry(&error))?;
    let engine = engine()?;
    let token = CancelToken::new();
    let held = token.clone();
    on_worker(token, pending, move || {
        let options = options(&held, deadline).map_err(|error| carry(&error))?;
        work(&engine, options).map_err(|error| carry(&error))
    })
}

/// The call's controls: its token, and its deadline in seconds.
fn options(token: &CancelToken, deadline: Option<f64>) -> Result<CallOptions<'_>, Error> {
    let options = CallOptions::new().cancel(token);
    deadline.map_or(Ok(options), |seconds| options.deadline_seconds(seconds))
}

/// Run `body` on a new thread and wait for it in ticks.
fn on_worker<T: Send + 'static>(
    token: CancelToken,
    pending: Pending<'_>,
    body: impl FnOnce() -> Crossed<T> + Send + 'static,
) -> Crossed<T> {
    let (sender, receiver) = channel();
    thread::Builder::new()
        .name("thinkthen-r".to_owned())
        .spawn(move || {
            let answer = catch_unwind(AssertUnwindSafe(body)).unwrap_or_else(|panic| {
                Err(defect(&format!("the call panicked: {}", words(&*panic))))
            });
            // The caller left after an interrupt when this send fails.
            let _ignored = sender.send(answer);
        })
        .map_err(|error| defect(&format!("the call's worker did not start: {error}")))?;
    wait(&receiver, &Stop(token), pending)
}

/// The main thread's wait. The stop guard cancels on every return.
fn wait<T>(receiver: &Receiver<Crossed<T>>, _stop: &Stop, pending: Pending<'_>) -> Crossed<T> {
    loop {
        match receiver.recv_timeout(TICK) {
            Ok(answer) => return answer,
            Err(RecvTimeoutError::Timeout) if pending() => return Err(interrupted()),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return Err(defect("the call's worker ended without an answer"));
            }
        }
    }
}

/// A panic's own words.
fn words(panic: &(dyn Any + Send)) -> String {
    panic
        .downcast_ref::<&str>()
        .map(|held| (*held).to_owned())
        .or_else(|| panic.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "no message".to_owned())
}

/// One text and its one-based place in the caller's vector.
#[derive(Debug)]
struct Placed {
    place: i32,
    text: String,
}

impl Evidence for Placed {
    fn evidence(&self) -> &str {
        &self.text
    }
}

fn placed(texts: Vec<String>) -> Vec<Placed> {
    (1..)
        .zip(texts)
        .map(|(place, text)| Placed { place, text })
        .collect()
}

/// A question file's question, checked on the main thread.
pub(crate) fn question(json: &str) -> Crossed<LoadedQuestion> {
    Question::from_json(json).map_err(|error| carry(&error))
}

/// 1 yes, 0 no, and -1 unsure, which the R half reads as `NA`.
const fn code(answer: Answer) -> i32 {
    match answer {
        Answer::Yes => 1,
        Answer::No => 0,
        Answer::Unsure => -1,
    }
}

/// Each record's answer code and yes probability.
type Judged = (Vec<i32>, Vec<f64>);

fn judged<Q: DecisionQuestion>(
    engine: &Engine,
    question: &Q,
    texts: &[String],
    options: CallOptions<'_>,
) -> Result<Judged, Error> {
    let mut answers = (Vec::new(), Vec::new());
    for row in engine.decide_many_with(question, texts.iter().map(String::as_str), options) {
        let row = row?;
        answers.0.push(code(*row.value()));
        answers.1.push(row.probability());
    }
    Ok(answers)
}

/// `decide` over a column, in input order.
pub(crate) fn decide(
    json: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let asked = question(json)?;
    let (codes, probabilities) = call(deadline, pending, move |engine, options| match &asked {
        LoadedQuestion::Question(held) => judged(engine, held, &texts, options),
        LoadedQuestion::Banded(held) => judged(engine, held, &texts, options),
    })?;
    Ok(list!(answer = codes, probability = probabilities))
}

/// `filter` over records: the one-based places whose evidence held.
pub(crate) fn filter(
    json: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<Vec<i32>> {
    let LoadedQuestion::Question(asked) = question(json)? else {
        return Err(usage("filter does not take a banded question"));
    };
    call(deadline, pending, move |engine, options| {
        engine
            .filter_with(&asked, placed(texts), options)
            .map(|kept| kept.map(|held| held.place))
            .collect()
    })
}

/// `rank` over records: places in rank order with their probabilities.
pub(crate) fn rank(
    text: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let asked = Question::rank(text).map_err(|error| carry(&error))?;
    let ranked = call(deadline, pending, move |engine, options| {
        engine.rank_with(&asked, placed(texts), options)
    })?;
    let places: Vec<i32> = ranked.iter().map(|row| row.input().place).collect();
    let probabilities: Vec<f64> = ranked.iter().map(thinkthen::Ranked::probability).collect();
    Ok(list!(place = places, probability = probabilities))
}

/// `find` over units: the selected place and its probability, or `NULL`s.
pub(crate) fn find(
    text: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let asked = Question::find(text).map_err(|error| carry(&error))?;
    let found = call(deadline, pending, move |engine, options| {
        engine.find_with(&asked, placed(texts), options)
    })?;
    let selected = found.candidates().iter().find(|held| {
        held.input()
            .zip(found.selected())
            .is_some_and(|(one, two)| one.place == two.place)
    });
    Ok(match selected {
        Some(held) => list!(
            place = held.input().map_or(0, |one| one.place),
            probability = held.probability()
        ),
        None => list!(
            place = Nullable::<i32>::Null,
            probability = Nullable::<f64>::Null
        ),
    })
}

/// Every record's values, in set order.
fn annotated(
    set: QuestionSet,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<Vec<Vec<Annotated>>> {
    call(deadline, pending, move |engine, options| {
        engine
            .annotate_with(&set, texts.iter().map(String::as_str), options)
            .map(|record| {
                record.map(|held| {
                    held.values()
                        .iter()
                        .map(|one| one.value().clone())
                        .collect()
                })
            })
            .collect()
    })
}

/// A failure cause as the shared cases spell it.
const fn cause_word(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "missing_answer",
        FailureCause::WrongKind => "wrong_kind",
        FailureCause::MissingProbability => "missing_probability",
        FailureCause::InvalidProbability => "invalid_probability",
        FailureCause::InvalidDistribution => "invalid_distribution",
        FailureCause::UnexpectedProbability => "unexpected_probability",
    }
}

/// One value as R's own: a decision code, a choice or `NULL`, a position,
/// labels, or the ruled `list(failed = list(kind, cause))` marker.
fn cell(value: &Annotated) -> Robj {
    match value {
        Annotated::Decision(answer) => code(*answer).into(),
        Annotated::Choice(pick) => Nullable::from(pick.clone()).into(),
        Annotated::Score(position) => (*position).into(),
        Annotated::Tags(labels) => labels.clone().into(),
        Annotated::Failed(failed) => list!(
            failed = list!(
                kind = failed.kind().name(),
                cause = cause_word(failed.cause())
            )
        )
        .into(),
    }
}

/// `annotate` over a column from a question set: the members' names and
/// kinds, and one list of cells a member.
pub(crate) fn annotate(
    set: QuestionSet,
    texts: Vec<String>,
    taken: &[String],
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let members: Vec<(String, String)> = set
        .members()
        .map(|(name, kind)| (name.to_owned(), format!("{kind:?}").to_lowercase()))
        .collect();
    if let Some((name, _)) = members.iter().find(|(name, _)| taken.contains(name)) {
        return Err(usage(&format!(
            "annotate cannot add a question named '{name}': the input already has a column by that name; rename one"
        )));
    }
    let rows = annotated(set, texts, deadline, pending)?;
    let columns = (0..members.len())
        .map(|at| {
            let cells = rows.iter().map(|row| {
                row.get(at)
                    .map(cell)
                    .ok_or_else(|| defect("an annotate record lacked a member"))
            });
            cells.collect::<Crossed<Vec<Robj>>>().map(List::from_values)
        })
        .collect::<Crossed<Vec<List>>>()?;
    let names: Vec<&str> = members.iter().map(|(name, _)| name.as_str()).collect();
    let kinds: Vec<&str> = members.iter().map(|(_, kind)| kind.as_str()).collect();
    Ok(list!(
        names = names,
        kinds = kinds,
        columns = List::from_values(columns)
    ))
}

/// `choose`, `score`, or `tag` over a column as one `annotate` of a
/// one-question set (ticket 0095). A failed cell raises `backend`.
pub(crate) fn column(
    json: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<List> {
    let LoadedQuestion::Question(asked) = question(json)? else {
        return Err(usage("only a decide question takes a band"));
    };
    let set = QuestionSet::builder()
        .question("value", asked)
        .and_then(thinkthen::QuestionSetBuilder::build)
        .map_err(|error| carry(&error))?;
    let rows = annotated(set, texts, deadline, pending)?;
    let cells = rows
        .iter()
        .zip(1..)
        .map(|(row, place): (&Vec<Annotated>, i32)| match row.first() {
            Some(Annotated::Failed(failed)) => Err(crate::packed(
                "backend",
                false,
                &format!(
                    "the backend failed row {place}: {}",
                    cause_word(failed.cause())
                ),
            )),
            Some(value) => Ok(cell(value)),
            None => Err(defect("an annotate record held no value")),
        });
    Ok(List::from_values(cells.collect::<Crossed<Vec<Robj>>>()?))
}

/// The audit view of one judgment: the command's `--details` document.
pub(crate) fn details(
    json: &str,
    text: String,
    deadline: Option<f64>,
    pending: Pending<'_>,
) -> Crossed<String> {
    let asked = question(json)?;
    call(deadline, pending, move |engine, options| {
        match &asked {
            LoadedQuestion::Question(held) => engine.details_with(held, &text, options),
            LoadedQuestion::Banded(held) => engine.details_with(held, &text, options),
        }
        .map(|held| held.to_json())
    })
}

/// The counters of the engine in use, as doubles.
pub(crate) fn counters() -> Crossed<List> {
    let counted = engine()?.usage();
    // R has no 64-bit integer, and counts stay far below 2^53.
    let as_double = |value: u64| value as f64;
    Ok(list!(
        requests_sent = as_double(counted.requests_sent()),
        cache_answers = as_double(counted.cache_answers()),
        input_tokens = as_double(counted.input_tokens()),
        output_tokens = as_double(counted.output_tokens())
    ))
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static PANICS: AtomicUsize = AtomicUsize::new(0);

    #[test]
    fn each_worker_edge_answers_without_a_stray_panic() {
        std::panic::set_hook(Box::new(|_| {
            PANICS.fetch_add(1, Ordering::SeqCst);
        }));
        let ticks = AtomicUsize::new(0);
        let left = on_worker(
            CancelToken::new(),
            &|| ticks.fetch_add(1, Ordering::SeqCst) > 0,
            || {
                thread::sleep(Duration::from_millis(400));
                Ok(1)
            },
        );
        assert_eq!(left, Err(interrupted()));
        thread::sleep(Duration::from_millis(500));
        assert_eq!(
            PANICS.load(Ordering::SeqCst),
            0,
            "the worker's failed send panicked"
        );

        let closed = channel::<Crossed<i32>>();
        drop(closed.0);
        let answer = wait(&closed.1, &Stop(CancelToken::new()), &|| false);
        assert_eq!(
            answer,
            Err(defect("the call's worker ended without an answer"))
        );

        let boom: Crossed<i32> = on_worker(CancelToken::new(), &|| false, || {
            std::panic::panic_any("boom")
        });
        assert_eq!(boom, Err(defect("the call panicked: boom")));
        assert_eq!(on_worker(CancelToken::new(), &|| false, || Ok(7)), Ok(7));
        let _ = std::panic::take_hook();
    }

    #[test]
    fn the_stop_guard_cancels_the_token_on_an_interrupt() {
        let token = CancelToken::new();
        let seen = token.clone();
        let left = on_worker(token, &|| true, || {
            thread::sleep(Duration::from_millis(300));
            Ok(())
        });
        assert_eq!(left, Err(interrupted()));
        assert!(seen.is_cancelled());
    }
}
