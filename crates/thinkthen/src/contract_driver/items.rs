//! Typed many-record results retain the caller's values independently of the engine.
use super::*;
use std::collections::VecDeque;
use std::sync::{Mutex, PoisonError};
/// Ordered results carrying each caller-owned input.
#[derive(Debug)]
pub struct Items<F: Function, T> {
    pub(super) rows: Rows<F>,
    originals: Arc<Mutex<VecDeque<T>>>,
}
impl<F: Function, T> Items<F, T> {
    /// Cancel this operation without affecting another call.
    pub fn cancel(&self) {
        self.rows.cancel();
    }
}
impl<F: Function, T> Iterator for Items<F, T> {
    type Item = Result<Row<F, T>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| {
            let row = row?;
            let item = self
                .originals
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .pop_front()
                .ok_or_else(results::mismatch)?;
            Ok(row.with_item(item))
        })
    }
}
impl Engine {
    /// Evaluate each record in order, retaining the caller's original on its result.
    /// # Errors
    /// Returns call setup and cancellation errors.
    pub fn rows<F: results::SingleFunction, I>(
        &self,
        question: &F,
        items: I,
    ) -> Result<Items<F, I::Item>, Error>
    where
        I: IntoIterator + Send + 'static,
        I::Item: Send + 'static,
        I::IntoIter: Send,
        for<'a> Record: From<&'a I::Item>,
    {
        self.items(question, items)
    }
    pub(super) fn items<F: Function, I>(
        &self,
        question: &F,
        items: I,
    ) -> Result<Items<F, I::Item>, Error>
    where
        I: IntoIterator + Send + 'static,
        I::Item: Send + 'static,
        I::IntoIter: Send,
        for<'a> Record: From<&'a I::Item>,
    {
        let originals = Arc::new(Mutex::new(VecDeque::new()));
        let pending = originals.clone();
        let records = items.into_iter().map(move |item| {
            let record = Record::from(&item);
            pending
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push_back(item);
            record
        });
        Ok(Items {
            rows: self.rows_without_items(question, records)?,
            originals,
        })
    }
}

#[cfg(feature = "stream")]
/// Async results retaining each caller-owned input.
#[derive(Debug)]
pub struct ItemStream<F: Function, T> {
    rows: super::RowStream<F>,
    originals: Arc<Mutex<VecDeque<T>>>,
}
#[cfg(feature = "stream")]
impl<F: Function, T> ItemStream<F, T> {
    /// Cancel this operation without affecting other calls.
    pub fn cancel(&self) {
        self.rows.cancel();
    }
}
#[cfg(feature = "stream")]
impl<F: Function, T> Items<F, T> {
    /// Read these results asynchronously; dropping the stream cancels the call.
    #[must_use]
    pub fn into_stream(self) -> ItemStream<F, T> {
        ItemStream {
            rows: self.rows.into_stream(),
            originals: self.originals,
        }
    }
}
#[cfg(feature = "stream")]
impl<F: Function, T> futures_core::Stream for ItemStream<F, T> {
    type Item = Result<Row<F, T>, Error>;
    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match futures_core::Stream::poll_next(std::pin::Pin::new(&mut this.rows), cx) {
            Poll::Ready(Some(row)) => Poll::Ready(Some(row.and_then(|row| {
                let item = this
                    .originals
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .pop_front()
                    .ok_or_else(results::mismatch)?;
                Ok(row.with_item(item))
            }))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
