//! The worker, the wait, and each verb's plain-data answer as R values.
//!
//! Every call runs on a fresh worker thread that owns its inputs, its
//! engine clone, and a clone of the call's cancel token. The main thread
//! waits on a channel in 100 ms ticks and runs R's interrupt check at each
//! one. A pending interrupt cancels the token and returns the marker at
//! once, so a request already on the wire never holds the caller. The
//! detached worker then sends nothing new and finishes what it sent.

use extendr_api::prelude::*;
use serde::Serialize;
use serde_json::value::RawValue;
use std::sync::Arc;
use thinkthen::{
    Answer, BatchSetting, CallOptions, DecisionQuestion, Engine, Evidence, LoadedQuestion,
    Question, QuestionSet,
};

use crate::{carry, defect, engine, usage};
use account::Account;
use receipt::Receipt;

pub(crate) mod account;
#[cfg(test)]
mod diagnostics;
pub(crate) mod receipt;
pub(crate) mod render;
mod worker;

#[cfg(test)]
pub(crate) use worker::on_worker;
pub(crate) use worker::{Crossed, Pending, call_owned};

/// One JSON text the crate wrote, carried as it is.
pub(crate) fn raw(text: String) -> Crossed<Box<RawValue>> {
    RawValue::from_string(text).map_err(|_| defect("the engine wrote a value that is not JSON"))
}

/// One text and its one-based place in the caller's vector.
#[derive(Debug)]
struct Placed {
    place: i32,
    text: String,
}

impl Evidence for Placed {
    fn evidence(&self) -> &str {
        &self.text
    }
}

fn placed(texts: Vec<String>) -> Vec<Placed> {
    (1..)
        .zip(texts)
        .map(|(place, text)| Placed { place, text })
        .collect()
}

/// A question file's question, checked on the main thread.
pub(crate) fn question(json: &str) -> Crossed<LoadedQuestion> {
    Question::from_json(json).map_err(|error| carry(&error))
}

/// `decide`'s answer: `true`, `false`, or `null` for unsure.
const fn answer(value: Answer) -> Option<bool> {
    match value {
        Answer::Yes => Some(true),
        Answer::No => Some(false),
        Answer::Unsure => None,
    }
}

/// Each record's answer and yes probability, in input order.
#[derive(Default, Serialize)]
struct Decided {
    answer: Vec<Option<bool>>,
    probability: Vec<f64>,
}

fn judged<Q: DecisionQuestion>(
    engine: &Engine,
    question: &Q,
    texts: &[String],
    options: CallOptions<'_>,
    account: &Account,
) -> Crossed<Decided> {
    let mut answers = Decided::default();
    let mut batch = engine.decide_many_with(question, texts.iter().map(String::as_str), options);
    let rows = account::collect(&mut batch, account)?;
    if texts.is_empty() {
        account.no_work();
    }
    for row in rows {
        answers.answer.push(answer(*row.value()));
        answers.probability.push(row.probability());
    }
    Ok(answers)
}

/// Apply a call's batch and context controls.
fn controlled<'a>(
    options: CallOptions<'a>,
    batch: Option<BatchSetting>,
    context: Option<&'a str>,
) -> CallOptions<'a> {
    let options = batch.map_or(options, |value| options.batch(value));
    context.map_or(options, |value| options.context(value))
}

/// `decide` over a column, in input order.
#[expect(
    clippy::too_many_arguments,
    reason = "the binding passes public call controls explicitly"
)]
pub(crate) fn decide(
    json: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    batch: Option<BatchSetting>,
    context: Option<String>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
    positions: Vec<usize>,
) -> Crossed<List> {
    let asked = question(json)?;
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        Some(positions),
        move |engine, options, account| {
            let options = controlled(options, batch, context.as_deref());
            match &asked {
                LoadedQuestion::Question(held) => judged(engine, held, &texts, options, account),
                LoadedQuestion::Banded(held) => judged(engine, held, &texts, options, account),
            }
        },
    )?;
    render::envelope(completed)
}

/// `filter` over records: the one-based places whose evidence held.
#[expect(
    clippy::too_many_arguments,
    reason = "the binding passes public call controls explicitly"
)]
pub(crate) fn filter(
    json: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    batch: Option<BatchSetting>,
    context: Option<String>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
) -> Crossed<List> {
    let LoadedQuestion::Question(asked) = question(json)? else {
        return Err(usage("filter does not take a banded question"));
    };
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        None,
        move |engine, options, account| {
            let empty = texts.is_empty();
            let options = controlled(options, batch, context.as_deref());
            let mut rows = engine.filter_with(&asked, placed(texts), options);
            let found = account::collect(&mut rows, account)?;
            if empty {
                account.no_work();
            }
            Ok(found.into_iter().map(|held| held.place).collect::<Vec<_>>())
        },
    )?;
    render::envelope(completed)
}

/// `rank`'s places in rank order, with their probabilities.
#[derive(Default, Serialize)]
struct RankedPlaces {
    place: Vec<i32>,
    probability: Vec<f64>,
}

/// `rank` over records: places in rank order with their probabilities.
#[expect(
    clippy::too_many_arguments,
    reason = "the binding passes public call controls explicitly"
)]
pub(crate) fn rank(
    text: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    batch: Option<BatchSetting>,
    context: Option<String>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
) -> Crossed<List> {
    let asked = Question::rank(text).map_err(|error| carry(&error))?;
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        None,
        move |engine, options, account| {
            let options = controlled(options, batch, context.as_deref());
            let ranked = account.run(|| engine.rank_with(&asked, placed(texts), options))?;
            let mut places = RankedPlaces::default();
            for row in &ranked {
                places.place.push(row.input().place);
                places.probability.push(row.probability());
            }
            Ok(places)
        },
    )?;
    render::envelope(completed)
}

/// `find`'s selected place and its probability, both `null` for none.
#[derive(Serialize)]
struct FoundPlace {
    place: Option<i32>,
    probability: Option<f64>,
}

/// `find` over units: the selected place and its probability, or `NULL`s.
pub(crate) fn find(
    text: &str,
    none: bool,
    texts: Vec<String>,
    deadline: Option<f64>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
) -> Crossed<List> {
    let asked = Question::find(text)
        .and_then(|asked| {
            if none {
                asked.offering_none()
            } else {
                Ok(asked)
            }
        })
        .map_err(|error| carry(&error))?;
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        None,
        move |engine, options, account| {
            let found = account.run(|| engine.find_with(&asked, placed(texts), options))?;
            let selected = found.candidates().iter().find(|held| {
                held.input()
                    .zip(found.selected())
                    .is_some_and(|(one, two)| one.place == two.place)
            });
            Ok(FoundPlace {
                place: selected.and_then(|held| held.input().map(|one| one.place)),
                probability: selected.map(thinkthen::Candidate::probability),
            })
        },
    )?;
    render::envelope(completed)
}

/// A set's member names and kinds, and each record's bare `annotate` object.
#[derive(Serialize)]
struct AnnotatedRows {
    names: Vec<String>,
    kinds: Vec<String>,
    rows: Vec<Box<RawValue>>,
}

/// `annotate` over a column from a question set.
#[expect(
    clippy::too_many_arguments,
    reason = "the binding passes public call controls explicitly"
)]
pub(crate) fn annotate(
    set: QuestionSet,
    texts: Vec<String>,
    taken: &[String],
    deadline: Option<f64>,
    batch: Option<BatchSetting>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
) -> Crossed<List> {
    let (names, kinds): (Vec<String>, Vec<String>) = set
        .members()
        .map(|(name, kind)| (name.to_owned(), format!("{kind:?}").to_lowercase()))
        .unzip();
    if let Some(name) = names.iter().find(|name| taken.contains(name)) {
        return Err(usage(&format!(
            "annotate cannot add a question named '{name}': the input already has a column by that name; rename one"
        )));
    }
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        None,
        move |engine, options, account| {
            let options = batch.map_or(options, |value| options.batch(value));
            let mut rows = engine.annotate_with(&set, texts.iter().map(String::as_str), options);
            let found = account::collect(&mut rows, account)?;
            if texts.is_empty() {
                account.no_work();
            }
            let rows = found
                .iter()
                .map(|held| raw(held.value_json()))
                .collect::<Crossed<Vec<_>>>()?;
            Ok(AnnotatedRows { names, kinds, rows })
        },
    )?;
    render::envelope(completed)
}

/// `choose`, `score`, or `tag` over a column: each record's `--details`
/// document, from which the R half reads the value and the chosen label's
/// probability.
#[expect(
    clippy::too_many_arguments,
    reason = "the binding passes public call controls explicitly"
)]
pub(crate) fn column(
    json: &str,
    texts: Vec<String>,
    deadline: Option<f64>,
    batch: Option<BatchSetting>,
    context: Option<String>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
    positions: Vec<usize>,
) -> Crossed<List> {
    let LoadedQuestion::Question(asked) = question(json)? else {
        return Err(usage("only a decide question takes a band"));
    };
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        Some(positions),
        move |engine, options, account| {
            let options = controlled(options, batch, context.as_deref());
            let mut rows =
                engine.details_many_with(&asked, texts.iter().map(String::as_str), options);
            let found = account::collect(&mut rows, account)?;
            if texts.is_empty() {
                account.no_work();
            }
            found
                .iter()
                .map(|row| raw(row.value().to_json()))
                .collect::<Crossed<Vec<_>>>()
        },
    )?;
    render::envelope(completed)
}

/// The audit view of one judgment: the command's `--details` document.
pub(crate) fn details(
    json: &str,
    text: String,
    deadline: Option<f64>,
    pending: Pending<'_>,
    receipt: Option<Arc<Receipt>>,
) -> Crossed<List> {
    let asked = question(json)?;
    let completed = call_owned(
        deadline,
        pending,
        receipt,
        None,
        move |engine, options, account| {
            let held = account.run(|| match &asked {
                LoadedQuestion::Question(held) => engine.details_with(held, &text, options),
                LoadedQuestion::Banded(held) => engine.details_with(held, &text, options),
            })?;
            raw(held.to_json())
        },
    )?;
    render::envelope(completed)
}

/// The counters of the engine in use, as JSON.
pub(crate) fn counters() -> Crossed<String> {
    serde_json::to_string(&engine()?.usage())
        .map_err(|_| defect("the counters could not be written as JSON"))
}
