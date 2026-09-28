//! One ordered annotation row's borrowed member details.

use super::Values;
use crate::public::annotated::NamedAnnotation;
use crate::public::error::Error;
use crate::public::options::Stop;
use crate::public::results::{ObservedRow, QuestionDetail, RecordObservation};

pub(super) fn observe_annotated(row: &Values, index: usize, stop: &Stop<'_>) -> Result<(), Error> {
    if !stop.observing() {
        return Ok(());
    }
    for (position, (name, detail)) in row.observed.iter().enumerate() {
        stop.observe(RecordObservation::Question {
            index,
            member: Some(name),
            stage: None,
            position,
            detail: QuestionDetail::of(detail),
        });
        if stop.observer_panicked() {
            return Err(Error::defect("the annotation observer panicked"));
        }
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
