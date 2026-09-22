//! The Series door: Polars columns and frames cross on Arrow buffers.
//!
//! Gated by the `polars` feature, off by default: the core surface
//! compiles with the pinned 1.93.1 toolchain and no Polars, and a caller
//! who wants the door turns the feature on — Polars itself requires the
//! 1.95 toolchain (experiment 228 records why: `sysinfo 0.39` refuses
//! 1.93).
//!
//! A text column crosses zero-copy: each row is a `&str` read out of the
//! producer's own UTF-8 buffer, with no copy and no per-row object. Every
//! call goes through the same engine, the same process-wide width gate,
//! and the same batch spine as a slice of strings — the equality
//! expectation: a column crosses at the same width as a slice, one
//! crossing, answers in input order.
//!
//! The stand-in's contract carries no bulk `choose`, `score`, or `tag`
//! (only `decide_many` is the batch spine), so those three columns run
//! one call a record through this door; the `decide` column takes the
//! spine. The real engine's ticket can widen them without a surface
//! change.

use polars::prelude::{DataFrame, IntoSeries, ListChunked, NamedFrom, Series};

use thinkthen_contract::{Annotated, Error, Options, Question, QuestionSet};

use crate::{Engine, IntoQuestion};

/// The texts of a column, borrowed in place; a null row and a non-text
/// column refuse with the usage kind naming what is wrong.
fn text_refs(column: &Series) -> Result<Vec<&str>, Error> {
    let text = column.str().map_err(|_| {
        Error::usage(format!(
            "the column {} is {:?}, not text; the door reads a text column",
            column.name(),
            column.dtype()
        ))
    })?;
    let mut refs = Vec::with_capacity(text.len());
    for place in 0..text.len() {
        let held = text.get(place).ok_or_else(|| {
            Error::usage(format!(
                "row {place} of the column {} is null; the engine judges text and has no NA row",
                column.name()
            ))
        })?;
        refs.push(held);
    }
    Ok(refs)
}

impl Engine {
    /// Ask one question of every text in a Polars column.
    ///
    /// The whole column crosses once through the batch spine at the
    /// process width gate, and the answers come back in input order as a
    /// boolean column whose nulls are "not sure".
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many`], and the usage kind when the
    /// column is not text or a row is null.
    pub fn decide_column<Q: IntoQuestion>(
        &self,
        question: Q,
        column: &Series,
    ) -> Result<Series, Error> {
        let asked = question.into_question()?;
        let texts = text_refs(column)?;
        let judgments = self.decide_many_opts(&asked, &texts, Options::new(), None)?;
        let values: Vec<Option<bool>> = judgments.iter().map(|one| one.answer.value()).collect();
        Ok(Series::new(column.name().clone(), values))
    }

    /// Pick, per text in the column, the option the evidence fits best.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::choose`], and the usage kind for a
    /// non-text column or a null row.
    pub fn choose_column(&self, question: &Question, column: &Series) -> Result<Series, Error> {
        let texts = text_refs(column)?;
        let mut values: Vec<Option<String>> = Vec::with_capacity(texts.len());
        for text in &texts {
            values.push(self.choose(question, text)?);
        }
        Ok(Series::new(column.name().clone(), values))
    }

    /// Place every text in the column on the question's levels.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::score`], and the usage kind for a
    /// non-text column or a null row.
    pub fn score_column(&self, question: &Question, column: &Series) -> Result<Series, Error> {
        let texts = text_refs(column)?;
        let mut values: Vec<f64> = Vec::with_capacity(texts.len());
        for text in &texts {
            values.push(self.score(question, text)?.value);
        }
        Ok(Series::new(column.name().clone(), values))
    }

    /// Name, per text in the column, the labels that held.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::tag`], and the usage kind for a non-text
    /// column or a null row.
    pub fn tag_column(&self, question: &Question, column: &Series) -> Result<Series, Error> {
        let texts = text_refs(column)?;
        let mut values: Vec<Vec<String>> = Vec::with_capacity(texts.len());
        for text in &texts {
            values.push(self.tag(question, text)?);
        }
        let rows: ListChunked = values
            .into_iter()
            .map(|row| Some(Series::new("".into(), row)))
            .collect();
        let mut series = rows.into_series();
        series.rename(column.name().clone());
        Ok(series)
    }

    /// Ask every question in the set of every record in a frame's column:
    /// the caller's own columns come back unchanged, one new column a
    /// question, in the set's name order.
    ///
    /// A question whose logical answer failed while its neighbours
    /// answered widens that question's whole column to text carrying the
    /// ruled marker, the same widening the other surfaces' frame doors
    /// use, so no failed cell reads as `false` or a null.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::annotate`], and the usage kind when the
    /// frame holds no column `on`, the column is not text, or a row is
    /// null.
    pub fn annotate_frame(
        &self,
        set: &QuestionSet,
        frame: &DataFrame,
        on: &str,
    ) -> Result<DataFrame, Error> {
        let column = frame
            .column(on)
            .map_err(|_| Error::usage(format!("the frame holds no column {on}")))?;
        let texts = text_refs(column.as_materialized_series())?;
        let rows = self.annotate(set, &texts)?;
        let mut out = frame.clone();
        for name in set.names() {
            let series = question_column(name, &rows)?;
            out.with_column(series.into()).map_err(|error| {
                Error::defect(format!("the frame refused the column {name}: {error}"))
            })?;
        }
        Ok(out)
    }
}

/// One question's answers over every row, as a column of its own type; a
/// failed member anywhere widens the column to text.
fn question_column(name: &str, rows: &[Vec<(String, Annotated)>]) -> Result<Series, Error> {
    fn field_of<'r>(row: &'r [(String, Annotated)], name: &str) -> Option<&'r Annotated> {
        row.iter()
            .find(|(held, _)| held.as_str() == name)
            .map(|(_, field)| field)
    }
    let any_failed = rows
        .iter()
        .any(|row| matches!(field_of(row, name), Some(Annotated::Failed(_))));
    if any_failed {
        let values: Vec<Option<String>> = rows
            .iter()
            .map(|row| match field_of(row, name) {
                Some(Annotated::Failed(failed)) => serde_json::to_string(failed).ok(),
                Some(Annotated::Decision(answer)) => answer.value().map(|value| value.to_string()),
                Some(Annotated::Choice(picked)) => picked.clone(),
                Some(Annotated::Score(scored)) => Some(scored.value.to_string()),
                Some(Annotated::Tags(tags)) => serde_json::to_string(tags).ok(),
                None => None,
            })
            .collect();
        return Ok(Series::new(name.into(), values));
    }
    // The set's grammar fixes each name's type; the first row names it.
    match rows.first().and_then(|row| field_of(row, name)) {
        Some(Annotated::Decision(_)) => {
            let values: Vec<Option<bool>> = rows
                .iter()
                .map(|row| match field_of(row, name) {
                    Some(Annotated::Decision(answer)) => answer.value(),
                    _ => None,
                })
                .collect();
            Ok(Series::new(name.into(), values))
        }
        Some(Annotated::Choice(_)) => {
            let values: Vec<Option<String>> = rows
                .iter()
                .map(|row| match field_of(row, name) {
                    Some(Annotated::Choice(picked)) => picked.clone(),
                    _ => None,
                })
                .collect();
            Ok(Series::new(name.into(), values))
        }
        Some(Annotated::Score(_)) => {
            let values: Vec<f64> = rows
                .iter()
                .map(|row| match field_of(row, name) {
                    Some(Annotated::Score(scored)) => scored.value,
                    _ => f64::NAN,
                })
                .collect();
            Ok(Series::new(name.into(), values))
        }
        Some(Annotated::Tags(_)) => {
            let values: Vec<Vec<String>> = rows
                .iter()
                .map(|row| match field_of(row, name) {
                    Some(Annotated::Tags(tags)) => tags.clone(),
                    _ => Vec::new(),
                })
                .collect();
            let held: ListChunked = values
                .into_iter()
                .map(|row| Some(Series::new("".into(), row)))
                .collect();
            let mut series = held.into_series();
            series.rename(name.into());
            Ok(series)
        }
        _ => Ok(Series::new(name.into(), Vec::<Option<String>>::new())),
    }
}
