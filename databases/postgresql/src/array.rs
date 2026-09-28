//! The SQL array adapter: first-occurrence packing and original SQL slots.

use std::collections::HashMap;

use pgrx::datum::Array;
use pgrx::prelude::*;
use thinkthen::LoadedQuestion;

use crate::{answer_value, call};

pub(crate) fn decide_distinct(
    question: LoadedQuestion,
    distinct: Vec<String>,
    context: Option<String>,
) -> Vec<Option<bool>> {
    let call = call::read();
    call.within(distinct.len());
    call::run(call, move |engine, options| {
        let options = context
            .as_deref()
            .map_or(options, |text| options.context(text));
        let rows = match &question {
            LoadedQuestion::Question(held) => engine
                .decide_many_with(held, distinct.iter().map(String::as_str), options)
                .collect::<Result<Vec<_>, _>>(),
            LoadedQuestion::Banded(held) => engine
                .decide_many_with(held, distinct.iter().map(String::as_str), options)
                .collect::<Result<Vec<_>, _>>(),
        };
        rows.map(|rows| {
            rows.into_iter()
                .map(|row| answer_value(*row.value()))
                .collect()
        })
    })
}

pub(crate) fn decide_array(
    question: Option<&str>,
    evidences: Option<Array<'_, &str>>,
    context: Option<&str>,
) -> TableIterator<'static, (name!(i, i32), name!(decided, Option<bool>))> {
    let question = crate::question(question, "", None);
    let context = crate::context(context);
    let rows: Vec<Option<String>> = evidences
        .iter()
        .flat_map(|held| held.iter().map(|text| text.map(str::to_owned)))
        .collect();
    let mut distinct = Vec::new();
    let mut found = HashMap::new();
    let at: Vec<Option<usize>> = rows
        .iter()
        .map(|text| {
            text.as_ref().map(|text| {
                *found.entry(text.clone()).or_insert_with(|| {
                    let place = distinct.len();
                    distinct.push(text.clone());
                    place
                })
            })
        })
        .collect();
    let decided = decide_distinct(question, distinct, context);
    let out: Vec<_> = at
        .into_iter()
        .enumerate()
        .map(|(place, at)| {
            let value = at.and_then(|at| decided.get(at).copied().flatten());
            (i32::try_from(place).unwrap_or(i32::MAX), value)
        })
        .collect();
    TableIterator::new(out)
}
