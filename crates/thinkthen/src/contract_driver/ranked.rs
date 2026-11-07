//! Ranking yields each caller's own item with its typed row, best first.
use super::*;
use crate::questions::Rank;
use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};
/// Each original item with its rank row, from [`Engine::rank`].
///
/// Ranked items come best first, then failed records in input order. A failed
/// record is an `Ok` pair, so its item comes back, and [`Row::into_answer`]
/// gives [`NoAnswer::Failed`](crate::NoAnswer::Failed). A call-level error ends
/// the iteration.
#[non_exhaustive]
#[derive(Debug)]
pub struct Ranked<T> {
    rows: Rows<Rank>,
    items: Arc<Mutex<HashMap<u64, T>>>,
}
impl<T> Iterator for Ranked<T> {
    type Item = Result<(T, Row<Rank>), Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| {
            let row = row?;
            let item = self
                .items
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .remove(&row.position())
                .ok_or_else(results::mismatch)?;
            Ok((item, row))
        })
    }
}
impl<T> Ranked<T> {
    /// Cancel the operation and wake any blocked input owner.
    pub fn cancel(&self) {
        self.rows.cancel();
    }
}
impl Engine {
    /// Yield each item with its row, best first, then failed records in input
    /// order. `record` turns each item into its record. See [`Ranked`] for failures.
    ///
    /// Each item stays in memory until its row arrives or the iteration ends, so
    /// with [`Rank::top`] every item is held until then, and the items that made
    /// no place are dropped at the end. For long input with `top`, use
    /// [`Engine::rows`] and [`Row::record`], which keep only the best records.
    ///
    /// ```
    /// # use thinkthen::{BackendName, Rank, Settings};
    /// # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
    /// let by_urgency = Rank::new("How urgent is the request?", ["routine", "urgent"])?;
    /// let messages = ["Where is my parcel?", "I was charged twice."];
    /// for ranked in engine.rank(&by_urgency, messages, |message| *message)? {
    ///     let (message, row) = ranked?;
    ///     println!("{} {message}", row.answer()?);
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Returns call setup errors before starting the iterator.
    pub fn rank<T: Send + 'static, I, F, R>(
        &self,
        question: &Rank,
        items: I,
        mut record: F,
    ) -> Result<Ranked<T>, Error>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send,
        F: FnMut(&T) -> R + Send + 'static,
        R: Into<Record>,
    {
        let held = Arc::new(Mutex::new(HashMap::new()));
        let pending = held.clone();
        let records = items.into_iter().zip(1_u64..).map(move |(item, position)| {
            let value: Record = record(&item).into();
            pending
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .insert(position, item);
            value
        });
        Ok(Ranked {
            rows: self.rows(question, records)?,
            items: held,
        })
    }
}
