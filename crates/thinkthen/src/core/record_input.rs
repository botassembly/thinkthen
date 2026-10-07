//! Native record options reuse record projection and question validation.

use super::{Framing, Labels, Pointer, Reading, ReadingError, RecordError};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum OptionsError {
    #[error(transparent)]
    Reading(#[from] ReadingError),
    #[error(transparent)]
    Record(#[from] RecordError),
}

pub(crate) fn project_options(record: &str, pointer: &Pointer) -> Result<Labels, OptionsError> {
    let reading = Reading::new(Framing::Jsonl, Vec::new())?;
    Ok(reading.record(record.as_bytes())?.choices(pointer)?)
}
