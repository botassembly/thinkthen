//! CLI annotation framing and preview presentation; native execution owns answers.
use super::{Judging, PrepareError, plan_for};
use crate::core::pack::{self, Ask};
use crate::core::{Reading, Record, Url};
use crate::failure::Failure;

pub(crate) type Framed = crate::cli::intake::Item;

/// One group's number and its wire questions.
pub(super) type GroupAsks = (usize, Vec<Ask>);

/// Every group's wire questions for one record, in set order, each beside
/// its group's number.
pub(super) fn asks(
    judging: &Judging<'_>,
    reading: &Reading,
    url: &Url,
    record: &Record,
) -> Result<Vec<GroupAsks>, PrepareError> {
    let context = judging.context_for(record).map_err(PrepareError::Other)?;
    judging
        .groups()
        .into_iter()
        .enumerate()
        .map(|(group, places)| {
            let plan = plan_for(
                judging.set(),
                &places,
                judging.engine().backend(),
                (judging.engine().profile(), context.as_ref()),
                reading,
                record,
            )?;
            let asks = pack::asks_for(judging.engine().backend().api_type(), url, &plan).map_err(
                |_| PrepareError::Other(Failure::Defect("a request could not be written as JSON")),
            )?;
            Ok((group, asks))
        })
        .collect()
}

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
