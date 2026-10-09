//! Complete ranking retains originals and graded judgments before assigning positions.
use crate::core::{self, Value};
use crate::public::{
    Call, CallOptions, CompleteRank, CompleteRecord, Engine, Error, InputEvidence, InputFunction,
    Question, RecordInput,
};

impl Engine {
    pub(crate) fn request_rank_records_complete_with<'a, I, T>(
        &'a self,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        #[cfg(feature = "cli")]
        if options.cli_reader.is_some() {
            return self
                .complete_stream(InputFunction::Rank, question, records, options, Ok)?
                .into_call()?
                .try_map(|rows| ranked(rows, question.threshold));
        }
        self.try_rank_records_complete_with(question, records, options)
    }
    /// Admit a fallible whole original set before any complete rank request.
    /// # Errors
    /// Input failures refuse the whole set before sending; otherwise as rank_records_complete_with.
    #[allow(
        clippy::type_complexity,
        reason = "Each ranked original retains its concrete complete result"
    )]
    pub fn try_rank_records_complete_with<I, T>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        let options = options.started()?;
        let records = self.try_within_admission(
            records
                .into_iter()
                .enumerate()
                .map(|(at, record)| record.map_err(|error| error.at_record(at))),
            &options,
        )?;
        self.rank_records_complete_with(question, records, options)
    }
    /// Complete rank of the whole original set using described decide or saved score.
    /// Positions are assigned after stable ordering, retaining the original ordinals.
    ///
    /// # Errors
    /// Invalid questions or inputs refuse before sending; started failures retain facts.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn rank_records_complete_with<I, T>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error>
    where
        I: IntoIterator<Item = RecordInput<T>>,
        T: InputEvidence,
    {
        self.records_complete(InputFunction::Rank, question, records, options)?
            .try_map(|rows| ranked(rows, question.threshold))
    }

    /// Complete rank of plain originals under one optional shared context.
    ///
    /// # Errors
    /// As rank_records_complete_with, retaining existing rank convenience calls.
    #[allow(
        clippy::type_complexity,
        reason = "Each occurrence retains its original and concrete complete result"
    )]
    pub fn rank_complete_with<I, T>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error>
    where
        I: IntoIterator<Item = T>,
        T: InputEvidence,
    {
        self.rank_records_complete_with(
            question,
            records.into_iter().map(|original| RecordInput {
                examples: None,
                seed_spans: None,
                original,
                context: None,
                options: None,
            }),
            options,
        )
    }
}

fn ranked<T>(
    rows: Vec<CompleteRecord<T, core::CompleteAtomic>>,
    threshold: Option<core::Threshold>,
) -> Result<Vec<CompleteRecord<T, CompleteRank>>, Error> {
    let weights = rows
        .iter()
        .map(|row| match row.result.value() {
            Value::Score(value) => Ok(*value),
            Value::YesNo(_) => row.result.answer().yes().ok_or_else(super::wrong),
            _ => Err(super::wrong()),
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let order = core::ranking_under(&weights, None, threshold);
    let mut rows: Vec<_> = rows.into_iter().map(Some).collect();
    order
        .into_iter()
        .enumerate()
        .map(|(at, original)| {
            let row = rows
                .get_mut(original)
                .and_then(Option::take)
                .ok_or_else(|| Error::defect("rank lost an original occurrence"))?;
            let value = at
                .checked_add(1)
                .and_then(std::num::NonZeroUsize::new)
                .ok_or_else(|| Error::defect("rank position overflow"))?;
            let canonical = row
                .result
                .ranked(row.ordinal, value)
                .map_err(|_| Error::defect("rank identity could not be finalized"))?;
            Ok(CompleteRecord {
                original: row.original,
                ordinal: row.ordinal,
                result: CompleteRank { canonical, value },
            })
        })
        .collect()
}
