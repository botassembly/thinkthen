//! Filtering yields the kept items, or every original item with its typed row.
use super::*;
use crate::questions::Filter;
use crate::results::Outcome;
use std::collections::VecDeque;
use std::sync::{Mutex, PoisonError};
/// Original items paired with every filter result, including failures.
#[non_exhaustive]
#[derive(Debug)]
pub struct Filtered<T> {
    rows: Rows<Filter>,
    originals: Arc<Mutex<VecDeque<T>>>,
}
impl<T> Iterator for Filtered<T> {
    type Item = Result<(T, Row<Filter>), Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| {
            let row = row?;
            let original = self
                .originals
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .pop_front()
                .ok_or_else(results::mismatch)?;
            Ok((original, row))
        })
    }
}
impl<T> Filtered<T> {
    /// Cancel the operation and wake any blocked input owner.
    pub fn cancel(&self) {
        self.rows.cancel();
    }
}
/// Only the items a filter keeps, in input order, from [`Engine::filter`].
///
/// A not-sure item is left out. A failed record yields its [`FilterError`] in its
/// place, with the item, and reading can go on. A call-level error yields a
/// [`FilterError`] without an item and ends the iteration.
#[non_exhaustive]
#[derive(Debug)]
pub struct Kept<T>(Filtered<T>);
impl<T> Iterator for Kept<T> {
    type Item = Result<T, FilterError<T>>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (item, row) = match self.0.next()? {
                Ok(pair) => pair,
                Err(error) => return Some(Err(FilterError { item: None, error })),
            };
            match row.outcome() {
                Outcome::Answered { answer: true, .. } => return Some(Ok(item)),
                Outcome::Answered { answer: false, .. } | Outcome::Unsure { .. } => {}
                Outcome::Failed { error } => {
                    return Some(Err(FilterError {
                        item: Some(item),
                        error: error.clone(),
                    }));
                }
            }
        }
    }
}
impl<T> Kept<T> {
    /// Cancel the operation and wake any blocked input owner.
    pub fn cancel(&self) {
        self.0.cancel();
    }
}
/// A filter failure: a failed record with its item, or a call-level error.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct FilterError<T> {
    item: Option<T>,
    error: Error,
}
impl<T> FilterError<T> {
    /// The item whose record failed, or `None` for a call-level error.
    #[must_use]
    pub const fn item(&self) -> Option<&T> {
        self.item.as_ref()
    }
    /// Take the item whose record failed, or `None` for a call-level error.
    #[must_use]
    pub fn into_item(self) -> Option<T> {
        self.item
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
impl<T: std::fmt::Debug> std::error::Error for FilterError<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}
impl Engine {
    /// Yield only the kept items, in input order. `record` turns each item into
    /// its field. See [`Kept`] for not-sure items and failures.
    ///
    /// ```
    /// # use thinkthen::{BackendName, Filter, Settings};
    /// # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
    /// let complaint = Filter::new("Is this review a complaint?")?;
    /// let reviews = ["Arrived early.", "The zipper broke.", "Great fit.", "Never came."];
    /// let complaints: Vec<&str> = engine
    ///     .filter(&complaint, reviews, |review| *review)?
    ///     .collect::<Result<_, _>>()?;
    /// assert!(complaints.iter().all(|review| reviews.contains(review)));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Returns call setup errors before starting the iterator.
    pub fn filter<T: Send + 'static, I, F, R>(
        &self,
        question: &Filter,
        items: I,
        record: F,
    ) -> Result<Kept<T>, Error>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send,
        F: FnMut(&T) -> R + Send + 'static,
        R: Into<Record>,
    {
        self.filter_items(question, items, record).map(Kept)
    }
    /// Yield every original item and its filter row in input order, kept or not.
    /// # Errors
    /// Returns call setup errors before starting the iterator.
    pub fn filter_items<T: Send + 'static, I, F, R>(
        &self,
        question: &Filter,
        items: I,
        mut record: F,
    ) -> Result<Filtered<T>, Error>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send,
        F: FnMut(&T) -> R + Send + 'static,
        R: Into<Record>,
    {
        let originals = Arc::new(Mutex::new(VecDeque::new()));
        let pending = originals.clone();
        let records = items.into_iter().map(move |item| {
            let value: Record = record(&item).into();
            pending
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push_back(item);
            value
        });
        Ok(Filtered {
            rows: self.rows(question, records)?,
            originals,
        })
    }
}
