//! Filter caller originals through the admitted Request scheduler.
use super::{Batch, CallOptions, Engine, Error, InputEvidence, InputFunction, Question};

impl Engine {
    /// Keep the original records that reach the question's threshold, in input order.
    /// Dropping the batch cancels its remaining work and joins its workers.
    ///
    /// # Errors
    /// Refuses an unsupported question before sending. The batch stops at its
    /// first failed record and retains the call's joined facts.
    pub fn filter<'a, I, T>(&self, question: &Question, records: I) -> Result<Batch<'a, T>, Error>
    where
        I: IntoIterator<Item = T> + 'a,
        T: InputEvidence + 'a,
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
    ) -> Result<Batch<'a, T>, Error>
    where
        I: IntoIterator<Item = T> + 'a,
        T: InputEvidence + 'a,
    {
        super::request::pull::project(
            self,
            InputFunction::Filter,
            question.clone(),
            records
                .into_iter()
                .map(|record| Ok(super::record_input::plain(record))),
            controls,
            |original, _, result| {
                Ok(super::complete::records::filter(result)?
                    .value()
                    .then_some(original))
            },
        )
    }
}
