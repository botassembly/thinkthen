//! Scalar SQL judgments with one defaulted, named PostgreSQL call shape.

use pgrx::datum::Array;
use pgrx::prelude::*;
use thinkthen::{For, Judgment};

use crate::call::{self, Refusal};
use crate::ffi::RawJson;
use crate::forms::{self, Named};
use crate::{answer_value, details};

fn judged(
    verb: For,
    question: Option<&str>,
    input: Option<&str>,
    members: Option<Array<'_, &str>>,
    settings: Option<RawJson>,
    named: Named<'_>,
) -> Option<thinkthen::Details> {
    let input = input?.to_owned();
    let (settings, call) = forms::controls(settings.as_ref(), named);
    let question = forms::question(question, members, verb, &settings);
    Some(call::run(call, move |engine, options| {
        details(engine, &question, &input, options)
    }))
}

#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
#[pg_extern(name = "thinkthen_decide", parallel_restricted)]
fn decide(
    question: Option<&str>,
    input: Option<&str>,
    settings: default!(Option<RawJson>, "NULL"),
    threshold: default!(Option<&str>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    model: default!(Option<&str>, "NULL"),
    batch: default!(Option<&str>, "NULL"),
    deadline_ms: default!(Option<i64>, "NULL"),
) -> Option<bool> {
    let named = Named {
        threshold,
        context,
        model,
        batch,
        deadline_ms,
    };
    let held = judged(For::Decide, question, input, None, settings, named)?;
    match held.value() {
        Judgment::Decision(value) => answer_value(*value),
        _ => call::raise(Refusal::usage("thinkthen_decide takes a decide question")),
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
#[pg_extern(name = "thinkthen_choose", parallel_restricted)]
fn choose(
    question: Option<&str>,
    input: Option<&str>,
    members: default!(Option<Array<'_, &str>>, "NULL"),
    settings: default!(Option<RawJson>, "NULL"),
    threshold: default!(Option<&str>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    model: default!(Option<&str>, "NULL"),
    batch: default!(Option<&str>, "NULL"),
    deadline_ms: default!(Option<i64>, "NULL"),
) -> Option<String> {
    let named = Named {
        threshold,
        context,
        model,
        batch,
        deadline_ms,
    };
    let held = judged(For::Choose, question, input, members, settings, named)?;
    match held.value() {
        Judgment::Choice(value) => value.clone(),
        _ => call::raise(Refusal::usage("thinkthen_choose takes a choose question")),
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
#[pg_extern(name = "thinkthen_score", parallel_restricted)]
fn score(
    question: Option<&str>,
    input: Option<&str>,
    members: default!(Option<Array<'_, &str>>, "NULL"),
    settings: default!(Option<RawJson>, "NULL"),
    threshold: default!(Option<&str>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    model: default!(Option<&str>, "NULL"),
    batch: default!(Option<&str>, "NULL"),
    deadline_ms: default!(Option<i64>, "NULL"),
) -> Option<f64> {
    let named = Named {
        threshold,
        context,
        model,
        batch,
        deadline_ms,
    };
    let held = judged(For::Score, question, input, members, settings, named)?;
    match held.value() {
        Judgment::Score(value) => Some(*value),
        _ => call::raise(Refusal::usage("thinkthen_score takes a score question")),
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
#[pg_extern(name = "thinkthen_tag", parallel_restricted)]
fn tag(
    question: Option<&str>,
    input: Option<&str>,
    members: default!(Option<Array<'_, &str>>, "NULL"),
    settings: default!(Option<RawJson>, "NULL"),
    threshold: default!(Option<&str>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    model: default!(Option<&str>, "NULL"),
    batch: default!(Option<&str>, "NULL"),
    deadline_ms: default!(Option<i64>, "NULL"),
) -> Option<Vec<String>> {
    let named = Named {
        threshold,
        context,
        model,
        batch,
        deadline_ms,
    };
    let held = judged(For::Tag, question, input, members, settings, named)?;
    match held.value() {
        Judgment::Tags(value) => Some(value.clone()),
        _ => call::raise(Refusal::usage("thinkthen_tag takes a tag question")),
    }
}

#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
#[pg_extern(name = "thinkthen_details", parallel_restricted)]
fn details_sql(
    question: Option<&str>,
    input: Option<&str>,
    settings: default!(Option<RawJson>, "NULL"),
    threshold: default!(Option<&str>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    model: default!(Option<&str>, "NULL"),
    batch: default!(Option<&str>, "NULL"),
    deadline_ms: default!(Option<i64>, "NULL"),
) -> Option<pgrx::datum::JsonB> {
    let named = Named {
        threshold,
        context,
        model,
        batch,
        deadline_ms,
    };
    let (settings, call) = forms::controls(settings.as_ref(), named);
    let verb = forms::plan_verb(question?, &settings);
    let asked = forms::question(question, None, verb, &settings);
    let input = input?.to_owned();
    let held = call::run(call, move |engine, options| {
        details(engine, &asked, &input, options)
    });
    Some(crate::jsonb(&held.to_json()))
}

/// Recoverable row failure without losing the ordinary details envelope.
#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
#[pg_extern(name = "thinkthen_try_details", parallel_restricted)]
fn try_details(
    question: Option<&str>,
    input: Option<&str>,
    settings: default!(Option<RawJson>, "NULL"),
    threshold: default!(Option<&str>, "NULL"),
    context: default!(Option<&str>, "NULL"),
    model: default!(Option<&str>, "NULL"),
    batch: default!(Option<&str>, "NULL"),
    deadline_ms: default!(Option<i64>, "NULL"),
) -> Option<pgrx::datum::JsonB> {
    let (Some(question), Some(input)) = (question, input) else {
        return None;
    };
    let named = Named {
        threshold,
        context,
        model,
        batch,
        deadline_ms,
    };
    let result = (|| {
        let (settings, call) = forms::controls_result(settings.as_ref(), named)?;
        let verb = forms::plan_verb_result(question, &settings)?;
        let question = forms::question_result(Some(question), None, verb, &settings)?;
        let input = input.to_owned();
        call::run_result(call, move |engine, options| {
            details(engine, &question, &input, options)
        })
    })();
    let value = match result {
        Ok(answer) => {
            serde_json::json!({"status":"answered","details":serde_json::from_str::<serde_json::Value>(&answer.to_json())
            .unwrap_or_else(|_| call::raise(Refusal::of(thinkthen::ErrorKind::Defect, "a result is not JSON")))})
        }
        Err(error) => error.value().unwrap_or_else(|| call::raise(error)),
    };
    Some(pgrx::datum::JsonB(value))
}
