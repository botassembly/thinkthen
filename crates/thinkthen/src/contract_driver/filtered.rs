//! Filtering retains every original item and its typed row.
use super::*;
use crate::questions::Filter;
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
impl Engine {
    /// Yield every original item and its filter row in input order.
    /// # Errors
    /// Returns call setup errors before starting the iterator.
    pub fn filter_items<T: Send + 'static, I, F>(
        &self,
        question: &Filter,
        items: I,
        mut record: F,
    ) -> Result<Filtered<T>, Error>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send,
        F: FnMut(&T) -> Record + Send + 'static,
    {
        let originals = Arc::new(Mutex::new(VecDeque::new()));
        let pending = originals.clone();
        let records = items.into_iter().map(move |item| {
            let value = record(&item);
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
