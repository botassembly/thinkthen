//! Scalar SQL judgments with one defaulted, named PostgreSQL call shape.

use pgrx::datum::Array;
use pgrx::prelude::*;
use thinkthen::{For, Judgment};

use crate::call;
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
) -> Option<crate::request::JudgmentRow> {
    let (question, input) = (question?, input?);
    let input = input.to_owned();
    let (settings, call) = forms::controls(settings.as_ref(), named);
    let question = forms::question(Some(question), members, verb, &settings);
    Some(
        call::run(call, move |engine, options| {
            details(engine, &question, &input, options)
        })
        .unwrap_or_else(|| call::raise(call::defect("details returned no record"))),
    )
}

crate::descriptions::describe! {
["Return a boolean decision, or NULL when unsure. ", &crate::descriptions::scalar_text()].concat();
[name = "thinkthen_decide", parallel_restricted];
#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
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
    call::guarded(|| {
        let named = Named::new(threshold, context, model, batch, deadline_ms);
        let held = judged(For::Decide, question, input, None, settings, named)?;
        match held.value {
            Judgment::Decision(value) => answer_value(value),
            _ => call::raise(call::usage("thinkthen_decide takes a decide question")),
        }
    })
}
}

crate::descriptions::describe! {
["Return one chosen member, or NULL when unsure. ", &crate::descriptions::scalar_text()].concat();
[name = "thinkthen_choose", parallel_restricted];
#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
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
    call::guarded(|| {
        let named = Named::new(threshold, context, model, batch, deadline_ms);
        let held = judged(For::Choose, question, input, members, settings, named)?;
        match held.value {
            Judgment::Choice(value) => value,
            _ => call::raise(call::usage("thinkthen_choose takes a choose question")),
        }
    })
}
}

crate::descriptions::describe! {
["Return a numeric score using the authored levels. ", &crate::descriptions::scalar_text()].concat();
[name = "thinkthen_score", parallel_restricted];
#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
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
    call::guarded(|| {
        let named = Named::new(threshold, context, model, batch, deadline_ms);
        let held = judged(For::Score, question, input, members, settings, named)?;
        match held.value {
            Judgment::Score(value) => Some(value),
            _ => call::raise(call::usage("thinkthen_score takes a score question")),
        }
    })
}
}

crate::descriptions::describe! {
["Return every matching authored label as text[]. ", &crate::descriptions::scalar_text()].concat();
[name = "thinkthen_tag", parallel_restricted];
#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
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
    call::guarded(|| {
        let named = Named::new(threshold, context, model, batch, deadline_ms);
        let held = judged(For::Tag, question, input, members, settings, named)?;
        match held.value {
            Judgment::Tags(value) => Some(value),
            _ => call::raise(call::usage("thinkthen_tag takes a tag question")),
        }
    })
}
}
