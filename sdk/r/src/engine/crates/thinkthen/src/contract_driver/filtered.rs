//! Filtering keeps successful records before one aggregate failure.
use super::*;
use crate::questions::Filter;
use crate::results::Outcome;
/// Only the items a filter keeps, in input order, from [`Engine::filter`].
///
/// A single cut decides every successful record. All kept items arrive first;
/// if any record failed, one terminal [`FilterError`] retains every failed result.
/// A call-level error terminates the iteration immediately.
#[non_exhaustive]
#[derive(Debug)]
pub struct Kept<T> {
    rows: Items<Filter, T>,
    state: KeptState<T>,
}
#[derive(Debug)]
struct KeptState<T> {
    failures: Vec<Row<Filter, T>>,
    finished: bool,
    total: u64,
}
impl<T> KeptState<T> {
    fn accept(
        &mut self,
        next: Option<Result<Row<Filter, T>, Error>>,
    ) -> Option<Result<T, FilterError<T>>> {
        match next {
            None => {
                self.finished = true;
                let rows = std::mem::take(&mut self.failures);
                let first = rows.first()?;
                let error = first
                    .error()
                    .map(|error| error.records_failed(rows.len(), self.total, first.position()))
                    .unwrap_or_else(results::mismatch);
                Some(Err(FilterError { rows, error }))
            }
            Some(Err(error)) => {
                self.finished = true;
                Some(Err(FilterError {
                    rows: std::mem::take(&mut self.failures),
                    error,
                }))
            }
            Some(Ok(row)) => {
                self.total = row.position();
                match row.outcome() {
                    Outcome::Answered { answer: true, .. } => Some(Ok(row.into_item())),
                    Outcome::Answered { answer: false, .. } | Outcome::Unsure { .. } => None,
                    Outcome::Failed { .. } => {
                        self.failures.push(row);
                        None
                    }
                }
            }
        }
    }
}
impl<T> Iterator for Kept<T> {
    type Item = Result<T, FilterError<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        while !self.state.finished {
            if let Some(value) = self.state.accept(self.rows.next()) {
                return Some(value);
            }
        }
        None
    }
}

impl<T> Kept<T> {
    /// Cancel the operation and wake any blocked input owner.
    pub fn cancel(&self) {
        self.rows.cancel();
    }
}
/// A filter failure: a failed record with its item, or a call-level error.
#[non_exhaustive]
#[derive(Clone)]
pub struct FilterError<T> {
    rows: Vec<Row<Filter, T>>,
    error: Error,
}
impl<T> std::fmt::Debug for FilterError<T> {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.debug_struct("FilterError")
            .field("error", &self.error)
            .field("failed_records", &self.rows.len())
            .finish()
    }
}
impl<T, U> PartialEq<FilterError<U>> for FilterError<T> {
    fn eq(&self, other: &FilterError<U>) -> bool {
        self.error == other.error && self.rows == other.rows
    }
}
impl<T> FilterError<T> {
    /// Every failed result, with the caller's original item retained.
    #[must_use]
    pub fn rows(&self) -> &[Row<Filter, T>] {
        &self.rows
    }
    /// The record's or the call's error.
    #[must_use]
    pub const fn error(&self) -> &Error {
        &self.error
    }
}
impl<T> std::fmt::Display for FilterError<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.error, formatter)
    }
}
impl<T> std::error::Error for FilterError<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}
impl Engine {
    /// Yield the caller's kept records, then one failure holding every failed result.
    /// # Errors
    /// Returns call setup errors before starting the iterator.
    pub fn filter<I>(&self, question: &Filter, items: I) -> Result<Kept<I::Item>, Error>
    where
        I: IntoIterator + Send + 'static,
        I::Item: Send + 'static,
        I::IntoIter: Send,
        for<'a> Record: From<&'a I::Item>,
    {
        Ok(Kept {
            rows: self.items(question, items)?,
            state: KeptState {
                failures: Vec::new(),
                finished: false,
                total: 0,
            },
        })
    }
}

#[cfg(feature = "stream")]
/// Kept records as an asynchronous stream, with the same terminal failure as filtering.
#[derive(Debug)]
pub struct KeptStream<T> {
    rows: ItemStream<Filter, T>,
    state: KeptState<T>,
}
// The stream never pins or projects a retained caller value. Records leave it by
// value, so moving the stream is safe even when a caller's type is not Unpin.
#[cfg(feature = "stream")]
impl<T> Unpin for KeptStream<T> {}
#[cfg(feature = "stream")]
impl<T> Kept<T> {
    /// Read kept records asynchronously without bringing an async runtime.
    #[must_use]
    pub fn into_stream(self) -> KeptStream<T> {
        KeptStream {
            rows: self.rows.into_stream(),
            state: self.state,
        }
    }
}
#[cfg(feature = "stream")]
impl<T> KeptStream<T> {
    /// Stop this call without affecting another operation.
    pub fn cancel(&self) {
        self.rows.cancel();
    }
}
#[cfg(feature = "stream")]
impl<T> futures_core::Stream for KeptStream<T> {
    type Item = Result<T, FilterError<T>>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        while !this.state.finished {
            match futures_core::Stream::poll_next(std::pin::Pin::new(&mut this.rows), cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(row) => {
                    if let Some(value) = this.state.accept(row) {
                        return Poll::Ready(Some(value));
                    }
                }
            }
        }
        Poll::Ready(None)
    }
}
