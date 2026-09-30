//! One jsonb object makes one packed call and one PostgreSQL tuplestore scan.

use pgrx::datum::JsonB;
use pgrx::prelude::*;
use thinkthen::{Details, For, Judgment, Probabilities};

use crate::call::{self, OrRaise as _};
use crate::ffi::RawJson;
use crate::forms::{self, Named};

fn answered(
    verb: For,
    question: Option<&str>,
    input: Option<JsonB>,
    settings: Option<RawJson>,
) -> Vec<(String, Details)> {
    let (settings, call) = forms::controls(settings.as_ref(), Named::default());
    let question = forms::question(question, None, verb, &settings);
    let records = forms::keyed(input);
    call.within(records.len());
    let (keys, texts): (Vec<_>, Vec<_>) = records.into_iter().unzip();
    let answers = call::run(call, move |engine, options| {
        let rows = engine
            .details_many_with(&question, texts, options)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows
            .into_iter()
            .map(|row| row.into_parts().1)
            .collect::<Vec<_>>())
    });
    keys.into_iter().zip(answers).collect()
}

fn probability(detail: &Details) -> Option<f64> {
    match (detail.value(), detail.probabilities()) {
        (Judgment::Decision(_), Probabilities::YesNo { yes }) => Some(*yes),
        (Judgment::Choice(Some(label)), Probabilities::Named(choices)) => choices
            .iter()
            .find(|choice| choice.name() == label)
            .map(|choice| choice.probability()),
        (Judgment::Choice(None), _) => None,
        _ => call::raise(call::usage("probability belongs to decide or choose")),
    }
}

#[pg_extern(name = "thinkthen_decide_many", parallel_restricted)]
fn decide_many(
    question: Option<&str>,
    input: Option<JsonB>,
    settings: default!(Option<RawJson>, "NULL"),
) -> TableIterator<
    'static,
    (
        name!(key, String),
        name!(value, Option<bool>),
        name!(probability, f64),
    ),
> {
    call::guarded(|| {
        let rows = answered(For::Decide, question, input, settings)
            .into_iter()
            .map(|(key, detail)| {
                let value = match detail.value() {
                    Judgment::Decision(answer) => crate::answer_value(*answer),
                    _ => call::raise(call::usage("thinkthen_decide_many takes a decide question")),
                };
                let yes = probability(&detail).unwrap_or_else(|| {
                    call::raise(call::defect("a decide row carried no probability"))
                });
                (key, value, yes)
            })
            .collect::<Vec<_>>();
        TableIterator::new(rows)
    })
}

#[allow(
    clippy::type_complexity,
    reason = "pgrx reads the named SQL columns from this inline tuple"
)]
#[pg_extern(name = "thinkthen_choose_many", parallel_restricted)]
fn choose_many(
    question: Option<&str>,
    input: Option<JsonB>,
    settings: default!(Option<RawJson>, "NULL"),
) -> TableIterator<
    'static,
    (
        name!(key, String),
        name!(value, Option<String>),
        name!(probability, Option<f64>),
    ),
> {
    call::guarded(|| {
        let rows = answered(For::Choose, question, input, settings)
            .into_iter()
            .map(|(key, detail)| {
                let value = match detail.value() {
                    Judgment::Choice(value) => value.clone(),
                    _ => call::raise(call::usage("thinkthen_choose_many takes a choose question")),
                };
                (key, value, probability(&detail))
            })
            .collect::<Vec<_>>();
        TableIterator::new(rows)
    })
}

#[pg_extern(name = "thinkthen_score_many", parallel_restricted)]
fn score_many(
    question: Option<&str>,
    input: Option<JsonB>,
    settings: default!(Option<RawJson>, "NULL"),
) -> TableIterator<'static, (name!(key, String), name!(value, f64))> {
    call::guarded(|| {
        let rows = answered(For::Score, question, input, settings)
            .into_iter()
            .map(|(key, detail)| {
                let value = match detail.value() {
                    Judgment::Score(value) => *value,
                    _ => call::raise(call::usage("thinkthen_score_many takes a score question")),
                };
                (key, value)
            })
            .collect::<Vec<_>>();
        TableIterator::new(rows)
    })
}

#[pg_extern(name = "thinkthen_tag_many", parallel_restricted)]
fn tag_many(
    question: Option<&str>,
    input: Option<JsonB>,
    settings: default!(Option<RawJson>, "NULL"),
) -> TableIterator<'static, (name!(key, String), name!(value, Vec<String>))> {
    call::guarded(|| {
        let rows = answered(For::Tag, question, input, settings)
            .into_iter()
            .map(|(key, detail)| {
                let value = match detail.value() {
                    Judgment::Tags(value) => value.clone(),
                    _ => call::raise(call::usage("thinkthen_tag_many takes a tag question")),
                };
                (key, value)
            })
            .collect::<Vec<_>>();
        TableIterator::new(rows)
    })
}

#[pg_extern(name = "thinkthen_plan", parallel_restricted)]
fn plan(
    question: Option<&str>,
    input: Option<JsonB>,
    settings: default!(Option<RawJson>, "NULL"),
) -> Option<JsonB> {
    call::guarded(|| {
        let question = question?;
        let (settings, call) = forms::controls(settings.as_ref(), Named::default());
        let verb = forms::plan_verb(question, &settings);
        let question = forms::question(Some(question), None, verb, &settings);
        let records: Vec<_> = forms::keyed(input)
            .into_iter()
            .map(|(_, value)| value)
            .collect();
        let estimate = call::run(call, move |engine, options| {
            engine.plan_with(&question, records, options)
        });
        let written = serde_json::to_value(&estimate)
            .map_err(|_| call::defect("the plan did not serialize"))
            .or_raise();
        Some(JsonB(written))
    })
}
