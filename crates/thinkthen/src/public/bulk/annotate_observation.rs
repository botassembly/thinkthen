//! One ordered annotation row's borrowed member details.

use super::Values;
use crate::public::annotated::NamedAnnotation;
use crate::public::error::Error;
use crate::public::options::Stop;
use crate::public::results::{ObservedRow, QuestionDetail, RecordObservation};

pub(crate) fn observe_annotated(row: &Values, index: usize, stop: &Stop<'_>) -> Result<(), Error> {
    observe_annotated_questions(row, index, stop)?;
    if !stop.observing() {
        return Ok(());
    }
    let values: Vec<_> = row
        .values
        .iter()
        .cloned()
        .map(|(name, value)| NamedAnnotation::of(name, value))
        .collect();
    stop.observe(RecordObservation::Row {
        index,
        value: ObservedRow::Annotated(&values),
    });
    if stop.observer_panicked() {
        return Err(Error::defect("the annotation observer panicked"));
    }
    Ok(())
}

pub(crate) fn observe_annotated_questions(
    row: &Values,
    index: usize,
    stop: &Stop<'_>,
) -> Result<(), Error> {
    if !stop.observing() {
        return Ok(());
    }
    for (position, (name, detail)) in row.observed.iter().enumerate() {
        let detail = detail.clone().qualified(
            crate::public::InputFunction::Annotate,
            index,
            Some(name),
            None,
            position,
        )?;
        stop.observe(RecordObservation::Question {
            index,
            member: Some(name),
            stage: None,
            position,
            detail: QuestionDetail::of(&detail),
        });
        if stop.observer_panicked() {
            return Err(Error::defect("the annotation observer panicked"));
        }
    }
    Ok(())
}
