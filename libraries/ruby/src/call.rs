//! The calls a worker runs: owned inputs in, plain Rust values out.

use thinkthen::{
    Answer, CallOptions, CancelToken, Details, Engine, Entity, Evidence, Judgment, LoadedQuestion,
    Question, QuestionSet, Recognize, Relate,
};

use crate::Fault;

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
    Filter(LoadedQuestion, Vec<String>),
    Rank(String, Vec<String>),
    Find(String, Vec<String>),
    Annotate(QuestionSet, Vec<String>),
    Recognize(Recognize, String),
    Relate(Relate, Vec<Entity>),
}

impl Ask {
    /// How many records a many-record call reads.
    pub(crate) fn records(&self) -> Option<usize> {
        match self {
            Self::DecideMany(_, records)
            | Self::Filter(_, records)
            | Self::Rank(_, records)
            | Self::Find(_, records)
            | Self::Annotate(_, records) => Some(records.len()),
            // The engine does not cap relate's entities by the record limit.
            Self::Decide(..)
            | Self::Details(..)
            | Self::Score(..)
            | Self::Recognize(..)
            | Self::Relate(..) => None,
        }
    }
}

/// Refuse a call over the engine's record limit before any request. The
/// engine's own batches refuse only once they reach the limit, and the
/// binding has read every record already.
pub(crate) fn within(most: Option<usize>, ask: &Ask) -> Result<(), Fault> {
    match (most, ask.records()) {
        (Some(most), Some(records)) if records > most => Err(Fault::usage(format!(
            "this engine answers at most {most} records in one call"
        ))),
        _ => Ok(()),
    }
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

fn details(found: &Details) -> Output {
    let value = match found.value() {
        Judgment::Decision(held) => Value::Decision(answer(*held)),
        Judgment::Choice(pick) => Value::Choice(pick.clone()),
        Judgment::Score(position) => Value::Score(*position),
        Judgment::Tags(labels) => Value::Tags(labels.clone()),
    };
    Output::Details(found.to_json(), value, found.nearest().map(str::to_owned))
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
pub(crate) fn run(
    engine: &Engine,
    ask: Ask,
    own: &CancelToken,
    deadline: Option<f64>,
) -> Result<Output, Fault> {
    let mut options = CallOptions::new().cancel(own);
    if let Some(seconds) = deadline {
        options = options.deadline_seconds(seconds)?;
    }
    Ok(match ask {
        Ask::Decide(LoadedQuestion::Question(question), text) => {
            Output::Answer(answer(engine.decide_with(&question, &text, options)?))
        }
        Ask::Decide(LoadedQuestion::Banded(question), text) => {
            Output::Answer(answer(engine.decide_with(&question, &text, options)?))
        }
        Ask::Details(LoadedQuestion::Question(question), text) => {
            details(&engine.details_with(&question, &text, options)?)
        }
        Ask::Details(LoadedQuestion::Banded(question), text) => {
            details(&engine.details_with(&question, &text, options)?)
        }
        Ask::Score(question, text) => {
            Output::Score(engine.score_with(&unbanded(question, "score")?, &text, options)?)
        }
        Ask::DecideMany(LoadedQuestion::Question(question), records) => Output::Rows(
            engine
                .decide_many_with(&question, records, options)
                .map(|row| row.map(|row| (answer(*row.value()), row.probability())))
                .collect::<Result<_, _>>()?,
        ),
        Ask::DecideMany(LoadedQuestion::Banded(question), records) => Output::Rows(
            engine
                .decide_many_with(&question, records, options)
                .map(|row| row.map(|row| (answer(*row.value()), row.probability())))
                .collect::<Result<_, _>>()?,
        ),
        Ask::Filter(question, records) => {
            let question = unbanded(question, "filter")?;
            Output::Places(
                engine
                    .filter_with(&question, texts(records), options)
                    .map(|kept| kept.map(|Text(place, _)| place))
                    .collect::<Result<_, _>>()?,
            )
        }
        Ask::Rank(text, records) => Output::Ranked(
            engine
                .rank_with(&Question::rank(&text)?, texts(records), options)?
                .into_iter()
                .map(|ranked| (ranked.input().0, ranked.probability()))
                .collect(),
        ),
        Ask::Find(text, units) => {
            let found = engine.find_with(&Question::find(&text)?, texts(units), options)?;
            Output::Found(found.selected().and_then(|Text(place, _)| {
                let candidate = found.candidates().get(*place)?;
                Some((*place, candidate.probability()))
            }))
        }
        Ask::Annotate(set, records) => Output::JsonRows(
            engine
                .annotate_with(&set, records, options)
                .map(|record| record.map(|record| record.value_json()))
                .collect::<Result<_, _>>()?,
        ),
        Ask::Recognize(ask, text) => {
            Output::Json(engine.recognize_with(&ask, &text, options)?.to_json())
        }
        Ask::Relate(ask, entities) => Output::JsonRows(
            engine
                .relate_with(&ask, entities, options)?
                .iter()
                .map(thinkthen::Edge::to_json)
                .collect(),
        ),
    })
}
