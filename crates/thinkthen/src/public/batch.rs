//! The lazy batch behind `filter`, `decide_many`, and `annotate`. Its
//! source is a pulled pipeline call in `pull`, or one error.

use std::fmt;

use crate::public::Call;
use crate::public::error::Error;
use crate::public::results::Facts;

/// The records a batch answers, pulled lazily, with each answer in input order.
///
/// A batch stops at its first failed record: every earlier row comes first,
/// then the error, then nothing. Dropping a batch stops its work and joins
/// every worker. A batch is neither `Send` nor `Sync`, so it stays on the
/// thread whose interrupt check it runs.
pub struct Batch<'a, T> {
    source: Box<dyn Source<T> + 'a>,
}

impl<T> fmt::Debug for Batch<'_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Batch").finish_non_exhaustive()
    }
}

impl<T> Iterator for Batch<'_, T> {
    type Item = Result<T, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.pull()
    }
}

impl<'a, T: 'a> Batch<'a, T> {
    /// Final facts after exhaustion or the one terminal error.
    #[must_use]
    pub fn facts(&self) -> Option<&Facts> {
        self.source.facts()
    }

    /// Exhaust this native batch and return its ordered values with joined final facts.
    /// # Errors
    /// Returns the first terminal error with its final facts; earlier observer rows remain visible.
    pub fn into_call(mut self) -> Result<Call<Vec<T>>, Error> {
        let values = self.by_ref().collect::<Result<Vec<_>, _>>()?;
        let facts = self
            .facts()
            .cloned()
            .ok_or_else(|| Error::defect("a completed batch has no facts"))?;
        Ok(Call::new(values, facts))
    }

    /// A batch that yields one error, then nothing.
    pub(crate) fn failed(error: Error) -> Self {
        Self {
            source: Box::new(Some(error)),
        }
    }

    pub(crate) fn of(result: Result<Self, Error>) -> Self {
        result.unwrap_or_else(Self::failed)
    }

    pub(crate) fn from_source(source: Box<dyn Source<T> + 'a>) -> Self {
        Self { source }
    }
}

pub(crate) trait Source<T> {
    fn pull(&mut self) -> Option<Result<T, Error>>;
    fn facts(&self) -> Option<&Facts>;
}

impl<T> Source<T> for Option<Error> {
    fn pull(&mut self) -> Option<Result<T, Error>> {
        self.take().map(Err)
    }

    fn facts(&self) -> Option<&Facts> {
        None
    }
}
