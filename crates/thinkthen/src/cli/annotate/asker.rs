//! CLI annotation framing and preview presentation; native execution owns answers.
use super::Judging;
use crate::core::{Reading, Record};
use crate::failure::Failure;

pub(crate) type Framed = crate::cli::intake::Item;

pub(super) struct Parser {
    reading: Reading,
    set: crate::core::QuestionSet,
    details: bool,
}
impl Parser {
    pub(super) fn of(judging: &Judging<'_>, reading: &Reading) -> Self {
        Self {
            reading: reading.clone(),
            set: judging.set().clone(),
            details: judging.details(),
        }
    }
    pub(super) fn record(&self, framed: Framed) -> Result<Record, Failure> {
        let record = match framed.data {
            crate::cli::intake::Data::Bytes(bytes) => self
                .reading
                .annotation_record(&bytes)
                .map_err(|error| Failure::record(error, self.reading.streams()))?,
            crate::cli::intake::Data::Record(record) => record,
            crate::cli::intake::Data::Images(_) => {
                return Err(Failure::Usage(
                    "annotate accepts text only; images are unsupported",
                ));
            }
        };
        if !self.details {
            super::collisions(&self.set, &record)?;
        }
        Ok(record)
    }
}
