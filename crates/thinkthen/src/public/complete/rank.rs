//! Complete ranking retains originals and graded judgments before assigning positions.
use crate::core::{self, Value};
use crate::public::{
    Call, CallOptions, CompleteRank, CompleteRecord, Engine, Error, InputEvidence, InputFunction,
    Question, RecordInput,
};

impl Engine {
    #[expect(
        clippy::type_complexity,
        reason = "rank retains the original and complete result together"
    )]
    pub(crate) fn request_rank_records_complete_with<'a, I, T>(
        &'a self,
        question: &'a Question,
        records: I,
        options: CallOptions<'a>,
        top: Option<usize>,
        release: Option<&dyn Fn(usize)>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>> + 'a,
        T: InputEvidence + 'a,
    {
        if let Some(release) = release {
            let mut batch = self.complete_stream(
                InputFunction::Rank,
                question.clone(),
                records,
                options,
                options.context_text().map(str::to_owned),
                Ok,
            )?;
            let mut held = Vec::new();
            for row in batch.by_ref() {
                retain_row(&mut held, top, question, row?, release)?;
            }
            let facts = batch
                .facts()
                .cloned()
                .ok_or_else(|| Error::defect("completed rank has no facts"))?;
            return Call::new(held.into_iter().map(|(_, row)| row).collect(), facts)
                .try_map(|rows| ranked(rows, None));
        }
        let records = self.try_within_admission(records, &options)?;
        self.execute_rank_records_complete_with(question, records, options)
    }
    /// Admit a fallible whole original set before any complete rank request.
    /// # Errors
    /// Input failures refuse the whole set before sending; started failures retain their facts.
    #[allow(
        clippy::type_complexity,
        reason = "Each ranked original retains its concrete complete result"
    )]
    pub(crate) fn try_rank_records_complete_with<I, T>(
        &self,
        question: &Question,
        records: I,
        options: CallOptions<'_>,
    ) -> Result<Call<Vec<CompleteRecord<T, CompleteRank>>>, Error>
    where
        I: IntoIterator<Item = Result<RecordInput<T>, Error>>,
        T: InputEvidence,
    {
        crate::public::request::native(
            self,
            crate::RequestFunction::Rank,
            crate::RequestDefinition::Rank(question.clone()),
            records,
            options,
            |engine, definition, records, controls| {
                let q = definition.rank()?;
                Ok(crate::public::request::restore(
                    engine.execute_rank_records_complete_with(&q, records, controls)?,
                ))
            },
        )
    }
    #[allow(
        clippy::type_complexity,
        reason = "the private Request primitive retains original and complete result types"
    )]
    pub(crate) fn execute_rank_records_complete_with<I, T>(
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
}

fn ranked<T>(
    rows: Vec<CompleteRecord<T, core::CompleteAtomic>>,
    threshold: Option<core::Threshold>,
) -> Result<Vec<CompleteRecord<T, CompleteRank>>, Error> {
    let weights = rows
        .iter()
        .map(|row| weight(&row.result))
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

fn retain_row<T>(
    held: &mut Vec<(f64, CompleteRecord<T, core::CompleteAtomic>)>,
    top: Option<usize>,
    question: &Question,
    row: CompleteRecord<T, core::CompleteAtomic>,
    release: &dyn Fn(usize),
) -> Result<(), Error> {
    let value = weight(&row.result)?;
    if question
        .threshold
        .is_some_and(|cut| !core::ranking_under(&[value], None, Some(cut)).contains(&0))
    {
        release(row.ordinal);
        return Ok(());
    }
    if let Some(old) = keep_best(held, top, value, row) {
        release(old.ordinal);
    }
    Ok(())
}

fn weight(result: &core::CompleteAtomic) -> Result<f64, Error> {
    match result.value() {
        Value::Score(value) => Ok(*value),
        Value::YesNo(_) => result.answer().yes().ok_or_else(super::wrong),
        _ => Err(super::wrong()),
    }
}

fn keep_best<T>(held: &mut Vec<(f64, T)>, top: Option<usize>, value: f64, row: T) -> Option<T> {
    let place = held.partition_point(|(earlier, _)| *earlier >= value);
    if top.is_some_and(|top| place >= top) {
        return Some(row);
    }
    let evicted = if top == Some(held.len()) {
        held.pop().map(|(_, row)| row)
    } else {
        None
    };
    held.insert(place, (value, row));
    evicted
}
