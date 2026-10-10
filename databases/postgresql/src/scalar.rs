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
) -> Option<thinkthen::Details> {
    let input = input?.to_owned();
    let (settings, call) = forms::controls(settings.as_ref(), named);
    let contextual = settings.context().is_some();
    let question = forms::question(question, members, verb, &settings);
    Some(
        call::run(call, move |engine, options| {
            details(engine, &question, &input, options, contextual)
        })
        .unwrap_or_else(|| call::raise(call::defect("details returned no record"))),
    )
}

crate::descriptions::describe! {
["Return a boolean decision, or NULL when unsure. ", crate::descriptions::SCALAR_TEXT].concat();
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
            _ => call::raise(call::usage("thinkthen_decide takes a decide question")),
        }
    })
}
}

crate::descriptions::describe! {
["Return one chosen member, or NULL when unsure. ", crate::descriptions::SCALAR_TEXT].concat();
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
            _ => call::raise(call::usage("thinkthen_choose takes a choose question")),
        }
    })
}
}

crate::descriptions::describe! {
["Return a numeric score using the authored levels. ", crate::descriptions::SCALAR_TEXT].concat();
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
            _ => call::raise(call::usage("thinkthen_score takes a score question")),
        }
    })
}
}

crate::descriptions::describe! {
["Return every matching authored label as text[]. ", crate::descriptions::SCALAR_TEXT].concat();
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
            _ => call::raise(call::usage("thinkthen_tag takes a tag question")),
        }
    })
}
}

crate::descriptions::describe! {
"Return detailed judgment JSON as jsonb. Judge text with a literal/JSON question or privileged/confined @file. NULL question returns NULL after controls validation; NULL input returns NULL after question validation; NULL optional settings uses defaults. Evidence files must be read by the client. Failures raise SQL errors.";
[name = "thinkthen_details", parallel_restricted];
#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
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
    call::guarded(|| {
        let named = Named {
            threshold,
            context,
            model,
            batch,
            deadline_ms,
        };
        let (settings, call) = forms::controls(settings.as_ref(), named);
        let contextual = settings.context().is_some();
        let verb = forms::plan_verb(question?, &settings);
        let asked = forms::question(question, None, verb, &settings);
        let input = input?.to_owned();
        let held = call::run(call, move |engine, options| {
            details(engine, &asked, &input, options, contextual)
        })
        .unwrap_or_else(|| call::raise(call::defect("details returned no record")));
        Some(crate::jsonb(&held.to_json()))
    })
}
}

crate::descriptions::describe! {
"Return detailed judgment JSON or a recoverable row failure as jsonb. Judge text with a literal/JSON question or privileged/confined @file. NULL question or input returns NULL before checking partners; NULL optional settings uses defaults. Evidence files must be read by the client. Recoverable failures return failure JSON; other failures raise SQL errors.";
[name = "thinkthen_try_details", parallel_restricted];
/// Recoverable row failure without losing the ordinary details envelope.
#[allow(
    clippy::too_many_arguments,
    reason = "PostgreSQL exposes these defaulted named parameters in its public SQL signature"
)]
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
    call::guarded(|| {
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
            let contextual = settings.context().is_some();
            let verb = forms::plan_verb_result(question, &settings)?;
            let question = forms::question_result(Some(question), None, verb, &settings)?;
            let input = input.to_owned();
            let answer = call::run_result(call, move |engine, options| {
                details(engine, &question, &input, options, contextual)
            })?;
            answer.ok_or_else(|| call::defect("details returned no record"))
        })();
        let value = match result {
            Ok(answer) => {
                serde_json::json!({"status":"answered","details":serde_json::from_str::<serde_json::Value>(&answer.to_json())
            .unwrap_or_else(|_| call::raise(call::defect("a result is not JSON")))})
            }
            Err(error) => call::failed_row(&error).unwrap_or_else(|| call::raise(error)),
        };
        Some(pgrx::datum::JsonB(value))
    })
}
}
