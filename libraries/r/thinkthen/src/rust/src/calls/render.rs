//! Convert completed Rust-owned accounts on R's main thread only.

use extendr_api::prelude::*;
use thinkthen::{Answer, FailureCause, Judgment, Probabilities};

use super::account::{Completed, Counts, Detail};
use super::receipt::State;

fn null() -> Robj {
    Nullable::<i32>::Null.into()
}

fn named(pairs: Vec<(&str, Robj)>) -> List {
    List::from_pairs(pairs)
}

fn facts(value: &Counts) -> List {
    let mut fields = vec![
        ("records", (value.records as f64).into()),
        ("requests_sent", (value.requests_sent as f64).into()),
        ("cache_answers", (value.cache_answers as f64).into()),
        ("seconds", value.seconds.into()),
    ];
    if let Some(tokens) = value.input_tokens {
        fields.push(("input_tokens", (tokens as f64).into()));
    }
    if let Some(tokens) = value.output_tokens {
        fields.push(("output_tokens", (tokens as f64).into()));
    }
    if let Some(model) = &value.model {
        fields.push(("model", model.as_str().into()));
    }
    named(fields)
}

fn answer(value: &Judgment) -> Robj {
    match value {
        Judgment::Decision(Answer::Yes) => true.into(),
        Judgment::Decision(Answer::No) => false.into(),
        Judgment::Decision(Answer::Unsure) | Judgment::Choice(None) => null(),
        Judgment::Choice(Some(pick)) => pick.as_str().into(),
        Judgment::Score(score) => (*score).into(),
        Judgment::Tags(tags) => tags.clone().into(),
    }
}

fn cause_word(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "missing_answer",
        FailureCause::WrongKind => "wrong_kind",
        FailureCause::MissingProbability => "missing_probability",
        FailureCause::InvalidProbability => "invalid_probability",
        FailureCause::InvalidDistribution => "invalid_distribution",
        FailureCause::UnexpectedProbability => "unexpected_probability",
    }
}

fn detail(value: &Detail) -> List {
    let mut fields = vec![
        ("index", (value.index as f64).into()),
        ("position", (value.position as f64).into()),
        ("question_sha256", value.question_sha256.as_str().into()),
        ("model", value.model.as_str().into()),
        ("url", value.url.as_str().into()),
        ("requests", value.requests.clone().into()),
        ("requests_sent", (value.requests_sent as f64).into()),
        ("cached", value.cached.into()),
        ("failed_questions", (value.failed_questions as f64).into()),
    ];
    if let Some(member) = &value.member {
        fields.push(("member", member.as_str().into()));
    }
    if let Some(stage) = value.stage {
        fields.push(("stage", stage.into()));
    }
    if let Some(judged) = &value.answer {
        fields.push(("answer", answer(judged)));
    }
    if let Some(cause) = value.failed {
        fields.push((
            "failed",
            list!(kind = "backend", cause = cause_word(cause)).into(),
        ));
    }
    if let Some(probabilities) = &value.probabilities {
        let probabilities: Robj = match probabilities {
            Probabilities::YesNo { yes } => (*yes).into(),
            Probabilities::Named(rows) => named(
                rows.iter()
                    .map(|row| (row.name(), row.probability().into()))
                    .collect(),
            )
            .into(),
        };
        fields.push(("probabilities", probabilities));
    }
    if let Some(confidence) = value.confidence {
        fields.push(("confidence", confidence.into()));
    }
    if let Some(usage) = value.usage {
        fields.push((
            "usage",
            list!(
                input_tokens = usage.input_tokens() as f64,
                output_tokens = usage.output_tokens() as f64
            )
            .into(),
        ));
    }
    named(fields)
}

pub(crate) fn envelope<T>(completed: Completed<T>, convert: impl FnOnce(T) -> Robj) -> List {
    let mut fields = vec![("tt_envelope", true.into())];
    match completed.result {
        Ok(value) => fields.push(("value", convert(value))),
        Err(error) => fields.push(("error", error.into())),
    }
    fields.push((
        "facts",
        completed
            .snapshot
            .facts
            .as_ref()
            .map_or_else(null, |one| facts(one).into()),
    ));
    fields.push((
        "details",
        List::from_values(completed.snapshot.details.iter().map(detail)).into(),
    ));
    named(fields)
}

pub(crate) fn receipt(state: State) -> List {
    match state {
        State::Unused => list!(state = "unused"),
        State::Claimed => list!(state = "claimed"),
        State::Running => list!(state = "running"),
        State::Terminal {
            kind,
            snapshot: value,
        } => named(vec![
            ("state", "terminal".into()),
            ("kind", kind.into()),
            (
                "facts",
                value
                    .facts
                    .as_ref()
                    .map_or_else(null, |one| facts(one).into()),
            ),
            (
                "details",
                List::from_values(value.details.iter().map(detail)).into(),
            ),
        ]),
    }
}
