//! Owned JSON snapshots of one Rust call's facts and ordered questions.

use std::sync::Mutex;

use serde_json::{Value, json};
use thinkthen::{Batch, Call, Facts, Judgment, RecordObservation};

use super::{Call as NativeCall, Failure};
use thinkthen::{
    Answer, CallOptions, DecisionQuestion, DetailQuestion, Engine, Entity, Evidence,
    LoadedQuestion, Question, QuestionKind, QuestionSet, Recognize, Relate,
};

/// A record and its place, so `filter` and `rank` report indexes.
struct Indexed(usize, String);

impl Evidence for Indexed {
    fn evidence(&self) -> &str {
        &self.1
    }
}

pub(super) struct Finished {
    pub(super) value: String,
    pub(super) facts: Facts,
}

pub(super) fn eager<T>(call: Call<T>, value: impl FnOnce(&T) -> String) -> Finished {
    Finished {
        value: value(call.value()),
        facts: call.facts().clone(),
    }
}

pub(super) fn batch<T>(
    mut rows: Batch<'_, T>,
    value: impl Fn(&T) -> String,
) -> Result<Finished, Failure> {
    let mut values = Vec::new();
    for row in &mut rows {
        values.push(value(&row?).to_owned());
    }
    let facts = rows
        .facts()
        .cloned()
        .ok_or_else(|| Failure::defect("a completed batch held no final facts"))?;
    Ok(Finished {
        value: format!("[{}]", values.join(",")),
        facts,
    })
}

/// Each question event as the crate serializes it. A row event has no JSON
/// form, so it is skipped; a question event always serializes.
#[derive(Default)]
pub(super) struct Observed(Mutex<Vec<Value>>);

impl Observed {
    pub(super) fn capture(&self, event: RecordObservation<'_>) {
        if matches!(event, RecordObservation::Row { .. }) {
            return;
        }
        if let (Ok(held), Ok(mut rows)) = (serde_json::to_value(&event), self.0.lock()) {
            rows.push(held);
        }
    }

    pub(super) fn snapshot(&self) -> Value {
        self.0
            .lock()
            .map_or_else(|_| json!([]), |rows| Value::Array(rows.clone()))
    }
}

pub(super) fn judgment(value: &Judgment) -> Value {
    match value {
        Judgment::Decision(answer) => match answer {
            thinkthen::Answer::Yes => json!(true),
            thinkthen::Answer::No => json!(false),
            thinkthen::Answer::Unsure => Value::Null,
        },
        Judgment::Choice(pick) => json!(pick),
        Judgment::Score(score) => json!(score),
        Judgment::Tags(labels) => json!(labels),
    }
}

pub(super) fn run(
    engine: &Engine,
    call: &NativeCall,
    options: CallOptions<'_>,
) -> Result<Finished, Failure> {
    let spec = call.spec.as_deref().unwrap_or_default();
    let text = call.payload.as_str();
    match call.op.as_str() {
        "decide" => Ok(eager(
            engine.decide_with(decision(&question(spec)?), text, options)?,
            |answer| word(*answer),
        )),
        "decide_many" => {
            let asked = question(spec)?;
            let rows = engine.decide_many_with(decision(&asked), records(text)?, options);
            batch(rows, |row| word(*row.value()))
        }
        "choose" | "tag" => {
            let asked = kind_of(&call.op, question(spec)?)?;
            Ok(eager(
                engine.details_with(detail(&asked), text, options)?,
                |details| judgment(details.value()).to_string(),
            ))
        }
        "choose_many" | "score_many" | "tag_many" => {
            let kind = call.op.strip_suffix("_many").unwrap_or_default();
            let asked = kind_of(kind, question(spec)?)?;
            batch(
                engine.details_many_with(detail(&asked), records(text)?, options),
                |row| judgment(row.value().value()).to_string(),
            )
        }
        "score" => match kind_of("score", question(spec)?)? {
            LoadedQuestion::Question(asked) => {
                Ok(eager(engine.score_with(&asked, text, options)?, |score| {
                    json!(score).to_string()
                }))
            }
            LoadedQuestion::Banded(_) => {
                Err(Failure::usage("score does not take a banded question"))
            }
        },
        "filter" => match question(spec)? {
            LoadedQuestion::Question(asked) => batch(
                engine.filter_with(&asked, indexed(text)?, options),
                |kept| json!(kept.0).to_string(),
            ),
            LoadedQuestion::Banded(_) => {
                Err(Failure::usage("filter does not take a banded question"))
            }
        },
        "rank" => Ok(eager(
            engine.rank_with(&Question::rank(spec)?, indexed(text)?, options)?,
            |ranked| {
                let rows: Vec<Value> = ranked
                    .iter()
                    .map(|row| json!({ "index": row.input().0, "probability": row.probability() }))
                    .collect();
                Value::from(rows).to_string()
            },
        )),
        "find" => found(engine, Question::find(spec)?, text, options),
        "find_none" => found(
            engine,
            Question::find(spec)?.offering_none()?,
            text,
            options,
        ),
        "annotate" => annotated(engine, spec, text, options),
        "details" => Ok(eager(
            engine.details_with(detail(&question(spec)?), text, options)?,
            thinkthen::Details::to_json,
        )),
        "recognize" => Ok(eager(
            engine.recognize_with(&Recognize::from_json(spec)?, text, options)?,
            thinkthen::Recognized::to_json,
        )),
        "relate" => related(engine, spec, text, options),
        other => Err(Failure::usage(format!("the door knows no op {other}"))),
    }
}

/// The selected unit's index and probability, or `null` when none was selected.
fn found(
    engine: &Engine,
    asked: Question,
    text: &str,
    options: CallOptions<'_>,
) -> Result<Finished, Failure> {
    Ok(eager(
        engine.find_with(&asked, indexed(text)?, options)?,
        |found| {
            found
                .candidates()
                .iter()
                .find_map(|candidate| {
                    let unit = candidate.input()?;
                    (found.selected().map(|held| held.0) == Some(unit.0))
                        .then(|| json!({ "index": unit.0, "probability": candidate.probability() }))
                })
                .unwrap_or(Value::Null)
                .to_string()
        },
    ))
}

/// Each record's bare `annotate` value. The set is a path or the set's JSON.
fn annotated(
    engine: &Engine,
    spec: &str,
    text: &str,
    options: CallOptions<'_>,
) -> Result<Finished, Failure> {
    let set = if spec.trim_start().starts_with('{') {
        QuestionSet::from_json(spec)?
    } else {
        QuestionSet::load(spec)?
    };
    batch(engine.annotate_with(&set, records(text)?, options), |row| {
        row.value_json()
    })
}

/// Each edge as the command's bare `relate` line.
fn related(
    engine: &Engine,
    spec: &str,
    text: &str,
    options: CallOptions<'_>,
) -> Result<Finished, Failure> {
    let pairs: Vec<(String, String)> = serde_json::from_str(text)
        .map_err(|_| Failure::usage("relate takes a list of [name, kind] pairs"))?;
    let entities = pairs
        .iter()
        .map(|(name, kind)| Entity::new(name, kind))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(eager(
        engine.relate_with(&Relate::from_json(spec)?, entities, options)?,
        |edges| {
            let lines: Vec<String> = edges.iter().map(thinkthen::Edge::to_json).collect();
            format!("[{}]", lines.join(","))
        },
    ))
}

fn question(spec: &str) -> Result<LoadedQuestion, Failure> {
    Ok(Question::from_json(spec)?)
}

/// Refuse a question of another kind before any send.
fn kind_of(op: &str, asked: LoadedQuestion) -> Result<LoadedQuestion, Failure> {
    let (kind, wanted) = match (&asked, op) {
        (LoadedQuestion::Banded(_), _) => (QuestionKind::Decide, false),
        (LoadedQuestion::Question(held), "choose") => {
            (held.kind(), held.kind() == QuestionKind::Choose)
        }
        (LoadedQuestion::Question(held), "tag") => (held.kind(), held.kind() == QuestionKind::Tag),
        (LoadedQuestion::Question(held), _) => (held.kind(), held.kind() == QuestionKind::Score),
    };
    if wanted {
        Ok(asked)
    } else {
        Err(Failure::usage(format!(
            "{op} does not take a {} question",
            kind_word(kind)
        )))
    }
}

/// The word a refusal uses for a question kind.
const fn kind_word(kind: QuestionKind) -> &'static str {
    match kind {
        QuestionKind::Decide => "decide",
        QuestionKind::Choose => "choose",
        QuestionKind::Tag => "tag",
        QuestionKind::Score => "score",
        QuestionKind::Rank => "rank",
        QuestionKind::Find => "find",
    }
}

fn decision(asked: &LoadedQuestion) -> &dyn DecisionQuestion {
    match asked {
        LoadedQuestion::Question(held) => held,
        LoadedQuestion::Banded(held) => held,
    }
}

fn detail(asked: &LoadedQuestion) -> &dyn DetailQuestion {
    match asked {
        LoadedQuestion::Question(held) => held,
        LoadedQuestion::Banded(held) => held,
    }
}

fn word(answer: Answer) -> String {
    match answer {
        Answer::Yes => "true",
        Answer::No => "false",
        Answer::Unsure => "null",
    }
    .to_owned()
}

pub(super) fn records(payload: &str) -> Result<Vec<String>, Failure> {
    serde_json::from_str(payload).map_err(|_| Failure::usage("records is an array of strings"))
}

fn indexed(payload: &str) -> Result<Vec<Indexed>, Failure> {
    Ok(records(payload)?
        .into_iter()
        .enumerate()
        .map(|(at, text)| Indexed(at, text))
        .collect())
}
