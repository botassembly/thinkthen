//! Ruby conversion and the one-worker completion receipt.

use super::*;
use crate::call::{Output, Value as Judged};
use crate::result::{Completed, Detail};
use magnus::RArray;
use thinkthen::{Facts, Judgment, Probabilities};

pub(super) fn facts_hash(ruby: &Ruby, facts: &Facts) -> Result<RHash, Error> {
    let held = ruby.hash_new();
    held.aset(ruby.to_symbol("records"), facts.records())?;
    held.aset(ruby.to_symbol("requests_sent"), facts.requests_sent())?;
    held.aset(ruby.to_symbol("cache_answers"), facts.cache_answers())?;
    held.aset(ruby.to_symbol("seconds"), facts.seconds())?;
    if let Some(value) = facts.input_tokens() {
        held.aset(ruby.to_symbol("input_tokens"), value)?;
    }
    if let Some(value) = facts.output_tokens() {
        held.aset(ruby.to_symbol("output_tokens"), value)?;
    }
    if let Some(value) = facts.model() {
        held.aset(ruby.to_symbol("model"), value)?;
    }
    Ok(held)
}

fn judgment(ruby: &Ruby, value: &Judgment) -> Value {
    match value {
        Judgment::Decision(answer) => ruby.into_value(match answer {
            thinkthen::Answer::Yes => Some(true),
            thinkthen::Answer::No => Some(false),
            thinkthen::Answer::Unsure => None,
        }),
        Judgment::Choice(pick) => ruby.into_value(pick.clone()),
        Judgment::Score(score) => ruby.into_value(*score),
        Judgment::Tags(tags) => ruby.into_value(tags.clone()),
    }
}

fn failure_name(cause: thinkthen::FailureCause) -> &'static str {
    match cause {
        thinkthen::FailureCause::MissingAnswer => "missing_answer",
        thinkthen::FailureCause::WrongKind => "wrong_kind",
        thinkthen::FailureCause::MissingProbability => "missing_probability",
        thinkthen::FailureCause::InvalidProbability => "invalid_probability",
        thinkthen::FailureCause::InvalidDistribution => "invalid_distribution",
        thinkthen::FailureCause::UnexpectedProbability => "unexpected_probability",
    }
}

fn detail_hash(ruby: &Ruby, detail: &Detail) -> Result<RHash, Error> {
    let held = ruby.hash_new();
    held.aset(ruby.to_symbol("index"), detail.index)?;
    held.aset(ruby.to_symbol("position"), detail.position)?;
    held.aset(
        ruby.to_symbol("question_sha256"),
        detail.question_sha256.as_str(),
    )?;
    held.aset(ruby.to_symbol("model"), detail.model.as_str())?;
    held.aset(ruby.to_symbol("url"), detail.url.as_str())?;
    held.aset(ruby.to_symbol("requests"), detail.requests.clone())?;
    held.aset(ruby.to_symbol("requests_sent"), detail.requests_sent)?;
    held.aset(ruby.to_symbol("cached"), detail.cached)?;
    held.aset(ruby.to_symbol("failed_questions"), detail.failed_questions)?;
    if let Some(value) = &detail.member {
        held.aset(ruby.to_symbol("member"), value.as_str())?;
    }
    if let Some(value) = detail.stage {
        held.aset(ruby.to_symbol("stage"), value)?;
    }
    if let Some(value) = &detail.answer {
        held.aset(ruby.to_symbol("answer"), judgment(ruby, value))?;
    }
    if let Some(value) = detail.failed {
        let failed = ruby.hash_new();
        failed.aset(ruby.to_symbol("kind"), "backend")?;
        failed.aset(ruby.to_symbol("cause"), failure_name(value))?;
        held.aset(ruby.to_symbol("failed"), failed)?;
    }
    if let Some(value) = &detail.probabilities {
        match value {
            Probabilities::YesNo { yes } => held.aset(ruby.to_symbol("probabilities"), *yes)?,
            Probabilities::Named(rows) => {
                let values: Vec<(String, f64)> = rows
                    .iter()
                    .map(|row| (row.name().to_owned(), row.probability()))
                    .collect();
                held.aset(ruby.to_symbol("probabilities"), values)?;
            }
        }
    }
    if let Some(value) = detail.confidence {
        held.aset(ruby.to_symbol("confidence"), value)?;
    }
    if let Some(value) = detail.usage {
        let usage = ruby.hash_new();
        usage.aset(ruby.to_symbol("input_tokens"), value.input_tokens())?;
        usage.aset(ruby.to_symbol("output_tokens"), value.output_tokens())?;
        held.aset(ruby.to_symbol("usage"), usage)?;
    }
    Ok(held)
}

pub(super) fn details_array(ruby: &Ruby, details: &[Detail]) -> Result<RArray, Error> {
    let rows = ruby.ary_new();
    for detail in details {
        rows.push(detail_hash(ruby, detail)?)?;
    }
    Ok(rows)
}

fn judged(ruby: &Ruby, value: &Judged) -> Value {
    match value {
        Judged::Decision(answer) => ruby.into_value(*answer),
        Judged::Choice(pick) => ruby.into_value(pick.clone()),
        Judged::Score(position) => ruby.into_value(*position),
        Judged::Tags(labels) => ruby.into_value(labels.clone()),
    }
}

pub(super) fn output(ruby: &Ruby, answer: &Output) -> Result<Value, Error> {
    Ok(match answer {
        Output::Answer(answer) => ruby.into_value(*answer),
        Output::Score(position) => ruby.into_value(*position),
        Output::Details(json, value, nearest) => {
            ruby.into_value((json.clone(), judged(ruby, value), nearest.clone()))
        }
        Output::Rows(rows) => ruby.into_value(rows.clone()),
        Output::Many(rows) => {
            let array = ruby.ary_new();
            for row in rows {
                array.push(judged(ruby, row))?;
            }
            array.as_value()
        }
        Output::Places(places) => ruby.into_value(places.clone()),
        Output::Ranked(ranked) => ruby.into_value(ranked.clone()),
        Output::Found(found) => ruby.into_value(*found),
        Output::Json(json) => ruby.into_value(json.clone()),
        Output::JsonRows(rows) => ruby.into_value(rows.clone()),
    })
}

pub(super) fn attach_completion(ruby: &Ruby, error: &Error, handoff: &Arc<Handoff>) {
    if let Some(value) = error.value() {
        let receipt = CompletionValue(Arc::clone(handoff));
        let _: Result<Value, Error> = value.funcall(
            "instance_variable_set",
            ("@thinkthen_completion", ruby.into_value(receipt)),
        );
    }
}

pub(super) fn protected_completion(
    ruby: &Ruby,
    terminal: &Result<Completed, Fault>,
) -> Result<RHash, Error> {
    let held = ruby.hash_new();
    match terminal {
        Ok(done) => {
            held.aset(ruby.to_symbol("outcome"), "succeeded")?;
            held.aset(ruby.to_symbol("facts"), facts_hash(ruby, &done.facts)?)?;
            held.aset(
                ruby.to_symbol("details"),
                details_array(ruby, &done.details)?,
            )?;
        }
        Err(fault) => {
            held.aset(
                ruby.to_symbol("outcome"),
                if fault.message == "defect: the Ruby binding panicked" {
                    "panicked"
                } else {
                    "failed"
                },
            )?;
            held.aset(
                ruby.to_symbol("facts"),
                fault
                    .facts
                    .as_ref()
                    .map(|facts| facts_hash(ruby, facts))
                    .transpose()?,
            )?;
            held.aset(
                ruby.to_symbol("details"),
                if fault.facts.is_some() {
                    Some(details_array(ruby, &fault.details)?)
                } else {
                    None
                },
            )?;
            held.aset(ruby.to_symbol("kind"), fault.kind.name())?;
            held.aset(ruby.to_symbol("message"), fault.message.as_str())?;
            held.aset(ruby.to_symbol("retryable"), fault.retryable)?;
        }
    }
    Ok(held)
}
