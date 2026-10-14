//! Filter caller originals through the admitted Request scheduler.
use super::{
    Call, CallOptions, CompleteFilter, CompleteRecord, Engine, Error, InputEvidence, InputFunction,
    Question,
};

impl Engine {
    /// Keep the original records that reach the question's threshold, in input order.
    /// Dropping the batch cancels its remaining work and joins its workers.
    ///
    /// # Errors
    /// Refuses an unsupported question before sending. The batch stops at its
    /// first failed record and retains the call's joined facts.
    pub fn filter<'a, I, T>(
        &self,
        question: &Question,
        records: I,
    ) -> Result<Call<Vec<T>, Vec<CompleteRecord<T, CompleteFilter>>, Vec<Option<f64>>>, Error>
    where
        I: IntoIterator<Item = T> + 'a,
        T: InputEvidence + Clone + 'a,
    {
        self.filter_with(question, records, CallOptions::new())
    }

    /// [`Engine::filter`] under the caller's controls.
    ///
    /// # Errors
    /// As [`Engine::filter`].
    pub fn filter_with<'a, I, T>(
        &self,
        question: &Question,
        records: I,
        controls: CallOptions<'a>,
    ) -> Result<Call<Vec<T>, Vec<CompleteRecord<T, CompleteFilter>>, Vec<Option<f64>>>, Error>
    where
        I: IntoIterator<Item = T> + 'a,
        T: InputEvidence + Clone + 'a,
    {
        super::request::pull::project(
            self,
            InputFunction::Filter,
            question.clone(),
            records
                .into_iter()
                .map(|record| Ok(super::record_input::plain(record))),
            controls,
            |original, ordinal, result| {
                Ok(Some(CompleteRecord {
                    original,
                    ordinal,
                    result: super::complete::records::filter(result)?,
                }))
            },
        )?
        .into_call()?
        .project(|rows| {
            let mut values = Vec::new();
            let mut probabilities = Vec::new();
            for row in &rows {
                if row.result.value() {
                    values.push(row.original.clone());
                    probabilities.push(row.result.probability());
                }
            }
            Ok((values, rows, probabilities))
        })
    }
}
