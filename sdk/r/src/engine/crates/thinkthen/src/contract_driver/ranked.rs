//! Ranking yields each caller's own item with its typed row, best first.
use super::*;
use crate::questions::{Choice, Rank};
use std::collections::HashMap;
use std::sync::{Mutex, PoisonError};
/// Each original item with its rank row, from [`Engine::rank`].
///
/// Ranked items come best first, then failed records in input order. A failed
/// record is an `Ok` row, so its item comes back, and [`Row::into_answer`]
/// gives [`NoAnswer::Failed`](crate::NoAnswer::Failed). A call-level error ends
/// the iteration.
#[non_exhaustive]
#[derive(Debug)]
pub struct Ranked<T, C: Choice = String> {
    rows: Rows<Rank<C>>,
    items: Arc<Mutex<HashMap<u64, T>>>,
}
impl<T, C: Choice> Iterator for Ranked<T, C> {
    type Item = Result<Row<Rank<C>, T>, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().map(|row| {
            let row = row?;
            let item = self
                .items
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .remove(&row.position())
                .ok_or_else(results::mismatch)?;
            Ok(row.with_item(item))
        })
    }
}
impl<T, C: Choice> Ranked<T, C> {
    /// Cancel the operation and wake any blocked input owner.
    pub fn cancel(&self) {
        self.rows.cancel();
    }
}
impl Engine {
    /// Yield each item with its row, best first, then failed records in input
    /// order. `Record::from(&item)` uses the caller type's conversion. See [`Ranked`] for failures.
    ///
    /// Each item stays in memory until its row arrives or the iteration ends, so
    /// with [`Rank::top`] every item is held until then, and the items that made
    /// no place are dropped at the end.
    ///
    /// ```
    /// # use thinkthen::{BackendName, Rank, Settings};
    /// # let engine = Settings::default().with_backend(BackendName::Stub).engine(|_| None)?;
    /// let by_urgency = Rank::with_strings("How urgent is the request?", ["routine", "urgent"])?;
    /// let messages = ["Where is my parcel?", "I was charged twice."];
    /// for ranked in engine.rank(&by_urgency, messages)? {
    ///     let row = ranked?;
    ///     println!("{} {}", row.answer()?, row.item());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    /// # Errors
    /// Returns call setup errors before starting the iterator.
    pub fn rank<C: Choice, T: Send + 'static, I>(
        &self,
        question: &Rank<C>,
        items: I,
    ) -> Result<Ranked<T, C>, Error>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send,
        for<'a> Record: From<&'a T>,
    {
        let held = Arc::new(Mutex::new(HashMap::new()));
        let pending = held.clone();
        let records = items.into_iter().zip(1_u64..).map(move |(item, position)| {
            let value = Record::from(&item);
            pending
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .insert(position, item);
            value
        });
        Ok(Ranked {
            rows: self.rows_without_items(question, records)?,
            items: held,
        })
    }
}
