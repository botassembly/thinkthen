//! The calls a worker runs: owned inputs in, plain Rust values out.

use std::sync::{Mutex, PoisonError};

use thinkthen::{
    Answer, Batch, CallOptions, CancelToken, Details, Engine, Entity, Evidence, Facts, Judgment,
    LoadedQuestion, Question, QuestionSet, Recognize, Relate,
};

use crate::result::{Completed, Detail};
use crate::{Controls, Fault};

/// One text and its place in the caller's input.
#[derive(Debug)]
struct Text(usize, String);

impl Evidence for Text {
    fn evidence(&self) -> &str {
        &self.1
    }
}

fn texts(records: Vec<String>) -> impl Iterator<Item = Text> {
    records
        .into_iter()
        .enumerate()
        .map(|(place, text)| Text(place, text))
}

/// What one call asks, with every input owned, so the worker holds no
/// Ruby value.
#[derive(Debug)]
pub(crate) enum Ask {
    Decide(LoadedQuestion, String),
    Details(LoadedQuestion, String),
    Score(LoadedQuestion, String),
    DecideMany(LoadedQuestion, Vec<String>),
    Many(LoadedQuestion, Vec<String>),
    Filter(LoadedQuestion, Vec<String>),
    Rank(String, Vec<String>),
    /// The question text, whether it offers none, and the units.
    Find(String, bool, Vec<String>),
    Annotate(QuestionSet, Vec<String>),
    Recognize(Recognize, String),
    Relate(Relate, Vec<Entity>),
}

/// One judgment's value in the shape Ruby reads.
#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Decision(Option<bool>),
    Choice(Option<String>),
    Score(f64),
    Tags(Vec<String>),
}

/// What one call answers.
#[derive(Debug, PartialEq)]
pub(crate) enum Output {
    Answer(Option<bool>),
    Score(f64),
    Details(String, Value, Option<String>),
    Rows(Vec<(Option<bool>, f64)>),
    Many(Vec<Value>),
    Places(Vec<usize>),
    Ranked(Vec<(usize, f64)>),
    Found(Option<(usize, f64)>),
    Json(String),
    JsonRows(Vec<String>),
}

const fn answer(value: Answer) -> Option<bool> {
    match value {
        Answer::Yes => Some(true),
        Answer::No => Some(false),
        Answer::Unsure => None,
    }
}

fn value_of(found: &Judgment) -> Value {
    match found {
        Judgment::Decision(held) => Value::Decision(answer(*held)),
        Judgment::Choice(pick) => Value::Choice(pick.clone()),
        Judgment::Score(position) => Value::Score(*position),
        Judgment::Tags(labels) => Value::Tags(labels.clone()),
    }
}

fn details(found: &Details) -> Output {
    Output::Details(
        found.to_json(),
        value_of(found.value()),
        found.nearest().map(str::to_owned),
    )
}

/// A question that reads one cut, for the calls that refuse a band.
fn unbanded(question: LoadedQuestion, call: &str) -> Result<Question, Fault> {
    match question {
        LoadedQuestion::Question(question) => Ok(question),
        LoadedQuestion::Banded(_) => Err(Fault::usage(format!(
            "{call} does not take a banded question"
        ))),
    }
}

/// Run one call to its end on the calling thread, the worker.
fn collected<T, V>(
    mut batch: Batch<'_, T>,
    map: impl Fn(T) -> V,
) -> Result<(Vec<V>, Facts), Fault> {
    let mut values = Vec::new();
    for row in batch.by_ref() {
        values.push(map(row?));
    }
    let facts = batch.facts().cloned().ok_or_else(|| {
        Fault::of(
            thinkthen::ErrorKind::Defect,
            "a completed batch has no facts",
        )
    })?;
    Ok((values, facts))
}

pub(crate) fn run(
    engine: &Engine,
    ask: Ask,
    own: &CancelToken,
    controls: Controls,
) -> Result<Completed, Fault> {
    let observed = Mutex::new(Vec::new());
    let observe = |event: thinkthen::RecordObservation<'_>| {
        if let Some(detail) = Detail::copy(event) {
            observed
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(detail);
        }
    };
    let mut options = CallOptions::new().cancel(own);
    if let Some(seconds) = controls.deadline {
        options = options.deadline_seconds(seconds)?;
    }
    if let Some(setting) = controls.batch {
        options = options.batch(setting);
    }
    if let Some(text) = controls.context.as_deref() {
        options = options.context(text);
    }
    options = options.observe(&observe);
    let result = run_inner(engine, ask, options);
    let details = observed
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner);
    match result {
        Ok((value, facts)) => Ok(Completed {
            value,
            facts,
            details,
        }),
        Err(mut fault) => {
            fault.details = details;
            Err(fault)
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one dispatch matches the owned Ask variants without parallel tables"
)]
fn run_inner(
    engine: &Engine,
    ask: Ask,
    options: CallOptions<'_>,
) -> Result<(Output, Facts), Fault> {
    Ok(match ask {
        Ask::Decide(LoadedQuestion::Question(question), text) => {
            let call = engine.decide_with(&question, &text, options)?;
            let facts = call.facts().clone();
            (Output::Answer(answer(call.into_value())), facts)
        }
        Ask::Decide(LoadedQuestion::Banded(question), text) => {
            let call = engine.decide_with(&question, &text, options)?;
            let facts = call.facts().clone();
            (Output::Answer(answer(call.into_value())), facts)
        }
        Ask::Details(LoadedQuestion::Question(question), text) => {
            let call = engine.details_with(&question, &text, options)?;
            (details(call.value()), call.facts().clone())
        }
        Ask::Details(LoadedQuestion::Banded(question), text) => {
            let call = engine.details_with(&question, &text, options)?;
            (details(call.value()), call.facts().clone())
        }
        Ask::Score(question, text) => {
            let call = engine.score_with(&unbanded(question, "score")?, &text, options)?;
            let facts = call.facts().clone();
            (Output::Score(call.into_value()), facts)
        }
        Ask::DecideMany(LoadedQuestion::Question(question), records) => {
            let (rows, facts) = collected(
                engine.decide_many_with(&question, records, options),
                |row| (answer(*row.value()), row.probability()),
            )?;
            (Output::Rows(rows), facts)
        }
        Ask::DecideMany(LoadedQuestion::Banded(question), records) => {
            let (rows, facts) = collected(
                engine.decide_many_with(&question, records, options),
                |row| (answer(*row.value()), row.probability()),
            )?;
            (Output::Rows(rows), facts)
        }
        Ask::Many(LoadedQuestion::Question(question), records) => {
            let (rows, facts) = collected(
                engine.details_many_with(&question, records, options),
                |row| value_of(row.value().value()),
            )?;
            (Output::Many(rows), facts)
        }
        Ask::Many(LoadedQuestion::Banded(question), records) => {
            let (rows, facts) = collected(
                engine.details_many_with(&question, records, options),
                |row| value_of(row.value().value()),
            )?;
            (Output::Many(rows), facts)
        }
        Ask::Filter(question, records) => {
            let question = unbanded(question, "filter")?;
            let (places, facts) = collected(
                engine.filter_with(&question, texts(records), options),
                |Text(place, _)| place,
            )?;
            (Output::Places(places), facts)
        }
        Ask::Rank(text, records) => {
            let call = engine.rank_with(&Question::rank(&text)?, texts(records), options)?;
            let facts = call.facts().clone();
            (
                Output::Ranked(
                    call.into_value()
                        .into_iter()
                        .map(|ranked| (ranked.input().0, ranked.probability()))
                        .collect(),
                ),
                facts,
            )
        }
        Ask::Find(text, none, units) => {
            let asked = Question::find(&text)?;
            let asked = if none { asked.offering_none()? } else { asked };
            let call = engine.find_with(&asked, texts(units), options)?;
            let facts = call.facts().clone();
            let found = call.into_value();
            (
                Output::Found(found.selected().and_then(|Text(place, _)| {
                    let candidate = found.candidates().get(*place)?;
                    Some((*place, candidate.probability()))
                })),
                facts,
            )
        }
        Ask::Annotate(set, records) => {
            let (rows, facts) = collected(engine.annotate_with(&set, records, options), |row| {
                row.value_json()
            })?;
            (Output::JsonRows(rows), facts)
        }
        Ask::Recognize(ask, text) => {
            let call = engine.recognize_with(&ask, &text, options)?;
            let facts = call.facts().clone();
            (Output::Json(call.into_value().to_json()), facts)
        }
        Ask::Relate(ask, entities) => {
            let call = engine.relate_with(&ask, entities, options)?;
            let facts = call.facts().clone();
            (
                Output::JsonRows(
                    call.into_value()
                        .iter()
                        .map(thinkthen::Edge::to_json)
                        .collect(),
                ),
                facts,
            )
        }
    })
}
