//! Eager Python list and ordering operations over the Rust planner.

use super::collected;
use crate::asked::Asked;
use crate::result::Completed;
use crate::usage;
use pyo3::prelude::*;
use thinkthen::{Answer, CallOptions, Error, Evidence, Judgment};

/// Refuse a question whose kind differs from the selected public verb.
pub(super) fn of_kind(py: Python<'_>, asked: &Asked, verb: &str, kind: &str) -> PyResult<()> {
    if asked.kind() == kind {
        Ok(())
    } else {
        Err(usage(
            py,
            &format!("{verb} does not take a {} question", asked.kind()),
        ))
    }
}

/// The one-cut question an ordering or filter call requires.
pub(crate) fn only(
    py: Python<'_>,
    asked: &Asked,
    verb: &str,
    kind: &str,
) -> PyResult<thinkthen::Question> {
    of_kind(py, asked, verb, kind)?;
    match asked {
        Asked::Plain(question) => Ok(question.clone()),
        Asked::Banded(_) => Err(usage(py, "this call takes one cut, not a band")),
    }
}

/// One record and its place in the caller's list.
#[derive(Debug)]
struct Indexed(usize, String);

impl Evidence for Indexed {
    fn evidence(&self) -> &str {
        &self.1
    }
}

/// A record's place, text, and probability.
pub(super) type Placed = (usize, String, f64);

/// What `many` gives back from the worker.
pub(super) enum Many {
    Kept(Vec<String>),
    Answers(Vec<Answer>),
    Judgments(Vec<Judgment>),
}

/// The worker's side of `many`: `filter` under `cut`, or `decide_many`.
pub(super) fn many(
    engine: &thinkthen::Engine,
    cut: Option<&thinkthen::Question>,
    asked: &Asked,
    records: Vec<String>,
    options: CallOptions<'_>,
) -> Result<Completed<Many>, Error> {
    match (cut, asked) {
        (Some(cut), _) => {
            collected(engine.filter_with(cut, records, options)).map(|done| done.map(Many::Kept))
        }
        (None, asked) => collected(engine.decide_many_with(asked.decision(), records, options))
            .map(|done| {
                done.map(|rows| Many::Answers(rows.iter().map(|row| *row.value()).collect()))
            }),
    }
}

pub(super) fn labels(
    engine: &thinkthen::Engine,
    asked: &Asked,
    records: Vec<String>,
    options: CallOptions<'_>,
) -> Result<Completed<Many>, Error> {
    collected(engine.details_many_with(asked.detail(), records, options)).map(|done| {
        done.map(|rows| {
            Many::Judgments(rows.iter().map(|row| row.value().value().clone()).collect())
        })
    })
}

/// The worker's side of `order`: every ranked record, or the found unit.
pub(super) fn order(
    engine: &thinkthen::Engine,
    find: bool,
    asked: &thinkthen::Question,
    records: Vec<String>,
    options: CallOptions<'_>,
) -> Result<Completed<Vec<Placed>>, Error> {
    let records = records
        .into_iter()
        .enumerate()
        .map(|(at, text)| Indexed(at, text));
    if find {
        let found = engine.find_with(asked, records, options)?;
        let probability = |at: usize| {
            found
                .value()
                .candidates()
                .get(at)
                .map_or(0.0, thinkthen::Candidate::probability)
        };
        return Ok(Completed::new(
            found
                .value()
                .selected()
                .map(|Indexed(at, text)| (*at, text.clone(), probability(*at)))
                .into_iter()
                .collect(),
            found.facts(),
        ));
    }
    let ranked = engine.rank_with(asked, records, options)?;
    Ok(Completed::new(
        ranked
            .value()
            .iter()
            .map(|row| (row.input().0, row.input().1.clone(), row.probability()))
            .collect(),
        ranked.facts(),
    ))
}

/// Copy one native observation and its fixed advice into host-owned values.
pub(super) fn usage_status(
    state: thinkthen::UsagePersistence,
) -> PyResult<(String, Option<String>)> {
    let name = serde_json::to_value(state).map_err(|_| {
        pyo3::exceptions::PyRuntimeError::new_err("native usage state serialization failed")
    })?;
    let name = name.as_str().ok_or_else(|| {
        pyo3::exceptions::PyRuntimeError::new_err("native usage state is not text")
    })?;
    Ok((name.to_owned(), state.advice().map(str::to_owned)))
}
