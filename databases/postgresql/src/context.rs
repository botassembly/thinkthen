//! Final literal context overloads for SQL judgments.

use pgrx::datum::{Array, JsonB};
use pgrx::prelude::*;
use thinkthen::{Judgment, LoadedQuestion, Probabilities};

use crate::call::{self, OrRaise as _, Refusal};
use crate::{
    answer_value, array, context, context_result, decide, details, jsonb, judged, question,
    question_result,
};

#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide_context(
    question_text: Option<&str>,
    evidence: Option<&str>,
    shared: Option<&str>,
) -> Option<bool> {
    let question = question(question_text, "", None);
    let evidence = evidence?.to_owned();
    let shared = context(shared);
    let answer = call::run(call::read(), move |engine, options| {
        let Some(text) = shared.as_deref() else {
            return decide(engine, &question, &evidence, options).map(Some);
        };
        let options = options.context(text);
        let rows = match &question {
            LoadedQuestion::Question(held) => engine
                .decide_many_with(held, [evidence.as_str()], options)
                .collect::<Result<Vec<_>, _>>(),
            LoadedQuestion::Banded(held) => engine
                .decide_many_with(held, [evidence.as_str()], options)
                .collect::<Result<Vec<_>, _>>(),
        }?;
        Ok(rows.first().map(|row| *row.value()))
    })
    .unwrap_or_else(|| {
        call::raise(Refusal::of(
            thinkthen::ErrorKind::Defect,
            "one record yielded no answer",
        ))
    });
    answer_value(answer)
}

#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide_array_context(
    question_text: Option<&str>,
    evidences: Option<Array<'_, &str>>,
    shared: Option<&str>,
) -> TableIterator<'static, (name!(i, i32), name!(decided, Option<bool>))> {
    array::decide_array(question_text, evidences, shared)
}

#[pg_extern(name = "thinkthen_probability", parallel_restricted)]
fn probability_context(
    question_text: Option<&str>,
    evidence: Option<&str>,
    shared: Option<&str>,
) -> Option<f64> {
    let question = question(question_text, "", None);
    let evidence = evidence?;
    match judged(question, evidence, context(shared)).probabilities() {
        Probabilities::YesNo { yes } => Some(*yes),
        Probabilities::Named(_) => call::raise(Refusal::usage(
            "thinkthen_probability takes a decide question",
        )),
    }
}

#[pg_extern(name = "thinkthen_choose", parallel_restricted)]
fn choose_context(
    question_text: Option<&str>,
    evidence: Option<&str>,
    options: Option<Array<'_, &str>>,
    shared: Option<&str>,
) -> Option<String> {
    let question = question(question_text, "options", options);
    let evidence = evidence?;
    match judged(question, evidence, context(shared)).value() {
        Judgment::Choice(pick) => pick.clone(),
        _ => call::raise(Refusal::usage("thinkthen_choose takes a choose question")),
    }
}

#[pg_extern(name = "thinkthen_score", parallel_restricted)]
fn score_context(
    question_text: Option<&str>,
    evidence: Option<&str>,
    levels: Option<Array<'_, &str>>,
    shared: Option<&str>,
) -> Option<f64> {
    let question = question(question_text, "levels", levels);
    let evidence = evidence?;
    match judged(question, evidence, context(shared)).value() {
        Judgment::Score(position) => Some(*position),
        _ => call::raise(Refusal::usage("thinkthen_score takes a score question")),
    }
}

#[pg_extern(name = "thinkthen_tag", parallel_restricted)]
fn tag_context(
    question_text: Option<&str>,
    evidence: Option<&str>,
    labels: Option<Array<'_, &str>>,
    shared: Option<&str>,
) -> Option<Vec<String>> {
    let question = question(question_text, "labels", labels);
    let evidence = evidence?;
    match judged(question, evidence, context(shared)).value() {
        Judgment::Tags(held) => Some(held.clone()),
        _ => call::raise(Refusal::usage("thinkthen_tag takes a tag question")),
    }
}

#[pg_extern(name = "thinkthen_details", parallel_restricted)]
fn details_context(
    question_text: Option<&str>,
    evidence: Option<&str>,
    shared: Option<&str>,
) -> Option<JsonB> {
    let question = question(question_text, "", None);
    let evidence = evidence?;
    let shared = context(shared);
    let answer = judged(question, evidence, shared.clone());
    let text = if shared.is_some() {
        answer.to_scalar_json()
    } else {
        answer.to_json()
    };
    Some(jsonb(&text))
}

#[pg_extern(name = "thinkthen_try_details", parallel_restricted)]
fn try_details_context(
    question_text: Option<&str>,
    evidence: Option<&str>,
    shared: Option<&str>,
) -> Option<JsonB> {
    let (Some(question_text), Some(evidence)) = (question_text, evidence) else {
        return None;
    };
    let result = (|| {
        let question = question_result(Some(question_text))?;
        let call = call::read_result()?;
        let evidence = evidence.to_owned();
        let shared = context_result(shared)?;
        call::run_result(call, move |engine, options| {
            let Some(text) = shared.as_deref() else {
                return details(engine, &question, &evidence, options)
                    .map(|answer| Some(answer.to_json()));
            };
            let options = options.context(text);
            let rows = match &question {
                LoadedQuestion::Question(held) => engine
                    .details_many_with(held, [evidence.as_str()], options)
                    .collect::<Result<Vec<_>, _>>(),
                LoadedQuestion::Banded(held) => engine
                    .details_many_with(held, [evidence.as_str()], options)
                    .collect::<Result<Vec<_>, _>>(),
            }?;
            Ok(rows.first().map(|row| row.value().to_scalar_json()))
        })
    })();
    let value = match result {
        Ok(Some(text)) => {
            let details: serde_json::Value = serde_json::from_str(&text)
                .map_err(|_| Refusal::of(thinkthen::ErrorKind::Defect, "a result is not JSON"))
                .or_raise();
            serde_json::json!({"status":"answered","details":details})
        }
        Ok(None) => call::raise(Refusal::of(
            thinkthen::ErrorKind::Defect,
            "one record yielded no detail",
        )),
        Err(error) => error.value().unwrap_or_else(|| call::raise(error)),
    };
    Some(JsonB(value))
}
