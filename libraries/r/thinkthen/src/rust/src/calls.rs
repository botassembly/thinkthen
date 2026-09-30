//! The worker, the wait, and each verb's plain-data answer as R values.
//!
//! Every call runs on a fresh worker thread that owns its inputs, its
//! engine clone, and a clone of the call's cancel token. The main thread
//! waits on a channel in 100 ms ticks and runs R's interrupt check at each
//! one. A pending interrupt cancels the token and returns the marker at
//! once, so a request already on the wire never holds the caller. The
//! detached worker then sends nothing new and finishes what it sent.

use extendr_api::prelude::*;
use std::sync::Arc;
use thinkthen::{
    Annotated, Answer, BatchSetting, CallOptions, DecisionQuestion, Engine, Evidence, FailureCause,
    Judgment, LoadedQuestion, Probabilities, Question, QuestionSet,
};

use crate::{carry, defect, engine, usage};
use account::Account;
use receipt::Receipt;

pub(crate) mod account;
mod diagnostics;
pub(crate) mod receipt;
pub(crate) mod render;
mod worker;

#[cfg(test)]
pub(crate) use worker::on_worker;
pub(crate) use worker::{Crossed, Pending, call_owned};

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

/// 1 yes, 0 no, and -1 unsure, which the R half reads as `NA`.
const fn code(answer: Answer) -> i32 {
    match answer {
        Answer::Yes => 1,
        Answer::No => 0,
        Answer::Unsure => -1,
    }
}

/// Each record's answer code and yes probability.
type Judged = (Vec<i32>, Vec<f64>);

fn judged<Q: DecisionQuestion>(
    engine: &Engine,
    question: &Q,
    texts: &[String],
    options: CallOptions<'_>,
    account: &Account,
) -> Crossed<Judged> {
    let mut answers = (Vec::new(), Vec::new());
    let mut batch = engine.decide_many_with(question, texts.iter().map(String::as_str), options);
    let rows = account::collect(&mut batch, account)?;
    if texts.is_empty() {
        account.no_work();
    }
    for row in rows {
        answers.0.push(code(*row.value()));
        answers.1.push(row.probability());
    }
    Ok(answers)
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
            let options = batch.map_or(options, |value| options.batch(value));
            let options = context
                .as_deref()
                .map_or(options, |value| options.context(value));
            match &asked {
                LoadedQuestion::Question(held) => judged(engine, held, &texts, options, account),
                LoadedQuestion::Banded(held) => judged(engine, held, &texts, options, account),
            }
        },
    )?;
    Ok(render::envelope(completed, |(codes, probabilities)| {
        list!(answer = codes, probability = probabilities).into()
    }))
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
            let options = batch.map_or(options, |value| options.batch(value));
            let options = context
                .as_deref()
                .map_or(options, |value| options.context(value));
            let mut rows = engine.filter_with(&asked, placed(texts), options);
            let found = account::collect(&mut rows, account)?;
            if empty {
                account.no_work();
            }
            Ok(found.into_iter().map(|held| held.place).collect::<Vec<_>>())
        },
    )?;
    Ok(render::envelope(completed, |places| places.into()))
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
            let options = batch.map_or(options, |value| options.batch(value));
            let options = context
                .as_deref()
                .map_or(options, |value| options.context(value));
            let ranked = match engine.rank_with(&asked, placed(texts), options) {
                Ok(value) => value,
                Err(error) => {
                    return Err(account.failed(&error)?);
                }
            };
            account.add(ranked.facts())?;
            Ok(ranked.into_value())
        },
    )?;
    Ok(render::envelope(completed, |ranked| {
        let places: Vec<i32> = ranked.iter().map(|row| row.input().place).collect();
        let probabilities: Vec<f64> = ranked.iter().map(thinkthen::Ranked::probability).collect();
        list!(place = places, probability = probabilities).into()
    }))
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
            let found = match engine.find_with(&asked, placed(texts), options) {
                Ok(value) => value,
                Err(error) => {
                    return Err(account.failed(&error)?);
                }
            };
            account.add(found.facts())?;
            Ok(found.into_value())
        },
    )?;
    Ok(render::envelope(completed, |found| {
        let selected = found.candidates().iter().find(|held| {
            held.input()
                .zip(found.selected())
                .is_some_and(|(one, two)| one.place == two.place)
        });
        match selected {
            Some(held) => list!(
                place = held.input().map_or(0, |one| one.place),
                probability = held.probability()
            ),
            None => list!(
                place = Nullable::<i32>::Null,
                probability = Nullable::<f64>::Null
            ),
        }
        .into()
    }))
}

/// A failure cause as the shared cases spell it.
const fn cause_word(cause: FailureCause) -> &'static str {
    match cause {
        FailureCause::MissingAnswer => "missing_answer",
        FailureCause::WrongKind => "wrong_kind",
        FailureCause::MissingProbability => "missing_probability",
        FailureCause::InvalidProbability => "invalid_probability",
        FailureCause::InvalidDistribution => "invalid_distribution",
        FailureCause::UnexpectedProbability => "unexpected_probability",
    }
}

/// One value as R's own: a decision code, a choice or `NULL`, a position,
/// labels, or the ruled `list(failed = list(kind, cause))` marker.
fn cell(value: &Annotated) -> Robj {
    match value {
        Annotated::Decision(answer) => code(*answer).into(),
        Annotated::Choice(pick) => Nullable::from(pick.clone()).into(),
        Annotated::Score(position) => (*position).into(),
        Annotated::Tags(labels) => labels.clone().into(),
        Annotated::Failed(failed) => list!(
            failed = list!(
                kind = failed.kind().name(),
                cause = cause_word(failed.cause())
            )
        )
        .into(),
    }
}

/// `annotate` over a column from a question set: the members' names and
/// kinds, and one list of cells a member.
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
    let members: Vec<(String, String)> = set
        .members()
        .map(|(name, kind)| (name.to_owned(), format!("{kind:?}").to_lowercase()))
        .collect();
    if let Some((name, _)) = members.iter().find(|(name, _)| taken.contains(name)) {
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
            Ok(found
                .into_iter()
                .map(|held| {
                    held.values()
                        .iter()
                        .map(|one| one.value().clone())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>())
        },
    )?;
    let completed = completed.map(|rows| {
        let columns = (0..members.len())
            .map(|at| {
                let cells = rows.iter().map(|row| {
                    row.get(at)
                        .map(cell)
                        .ok_or_else(|| defect("an annotate record lacked a member"))
                });
                cells.collect::<Crossed<Vec<Robj>>>().map(List::from_values)
            })
            .collect::<Crossed<Vec<List>>>()?;
        let names: Vec<&str> = members.iter().map(|(name, _)| name.as_str()).collect();
        let kinds: Vec<&str> = members.iter().map(|(_, kind)| kind.as_str()).collect();
        Ok(list!(
            names = names,
            kinds = kinds,
            columns = List::from_values(columns)
        ))
    });
    Ok(render::envelope(completed, Into::into))
}

/// `choose`, `score`, or `tag` over a column as one `annotate` of a one-question set (0095).
/// The engine refuses a broken one-question reply whole, so a failed cell is a defect.
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
            let options = batch.map_or(options, |value| options.batch(value));
            let options = context
                .as_deref()
                .map_or(options, |value| options.context(value));
            let mut rows =
                engine.details_many_with(&asked, texts.iter().map(String::as_str), options);
            let found = account::collect(&mut rows, account)?;
            if texts.is_empty() {
                account.no_work();
            }
            found
                .into_iter()
                .map(|row| {
                    let detail = row.value();
                    let probability = match (detail.value(), detail.probabilities()) {
                        (Judgment::Choice(Some(selected)), Probabilities::Named(options)) => Some(
                            options
                                .iter()
                                .find(|one| one.name() == selected)
                                .ok_or_else(|| defect("the chosen label has no probability"))?
                                .probability(),
                        ),
                        (Judgment::Choice(None), Probabilities::Named(_)) => None,
                        (Judgment::Score(_), _) | (Judgment::Tags(_), _) => None,
                        _ => return Err(defect("a column detail held another probability shape")),
                    };
                    Ok((detail.value().clone(), probability))
                })
                .collect::<Crossed<Vec<_>>>()
        },
    )?;
    Ok(render::envelope(
        completed,
        |judged: Vec<(Judgment, Option<f64>)>| {
            let probability = Doubles::from_values(
                judged
                    .iter()
                    .map(|(_, one)| one.map_or_else(Rfloat::na, Rfloat::from)),
            );
            let value = List::from_values(judged.iter().map(|(value, _)| -> Robj {
                match value {
                    Judgment::Decision(answer) => code(*answer).into(),
                    Judgment::Choice(pick) => Nullable::from(pick.clone()).into(),
                    Judgment::Score(position) => (*position).into(),
                    Judgment::Tags(labels) => labels.clone().into(),
                }
            }));
            list!(value = value, probability = probability).into()
        },
    ))
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
            let held = match &asked {
                LoadedQuestion::Question(held) => engine.details_with(held, &text, options),
                LoadedQuestion::Banded(held) => engine.details_with(held, &text, options),
            };
            let held = match held {
                Ok(value) => value,
                Err(error) => {
                    return Err(account.failed(&error)?);
                }
            };
            account.add(held.facts())?;
            Ok(held.value().to_json())
        },
    )?;
    Ok(render::envelope(completed, Into::into))
}

/// The counters of the engine in use, as doubles.
pub(crate) fn counters() -> Crossed<List> {
    let counted = engine()?.usage();
    // R has no 64-bit integer, and counts stay far below 2^53.
    let as_double = |value: u64| value as f64;
    Ok(list!(
        requests_sent = as_double(counted.requests_sent()),
        retries = as_double(counted.retries()),
        cache_answers = as_double(counted.cache_answers()),
        input_tokens = as_double(counted.input_tokens()),
        output_tokens = as_double(counted.output_tokens())
    ))
}
