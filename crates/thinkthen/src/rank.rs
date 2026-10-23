//! Collection ranking uses the same engine calls as individual questions.

use crate::{Engine, Error, Rank, RankLimit, Ranking, Record, Row};

impl Engine {
    /// Evaluate all records and return their original values and ranked rows
    /// best first. Ties preserve input order and failures follow all answers.
    /// `top` retains only the best N records. Without it, all records stay in memory.
    ///
    /// # Errors
    /// Returns the first failed record's error, even if `top` discards its row,
    /// or `local.cancelled` on cancellation.
    #[expect(clippy::expect_used, reason = "each row belongs to one queued record")]
    pub fn rank<T>(
        &self,
        question: &Rank,
        records: impl IntoIterator<Item = (Record, T)>,
        top: Option<RankLimit>,
        mut sink: impl FnMut(T, Row),
    ) -> Result<(), Error> {
        let originals = std::cell::RefCell::new(std::collections::VecDeque::new());
        let records = records.into_iter().map(|(record, original)| {
            originals.borrow_mut().push_back(original);
            record
        });
        let mut ranking = Ranking::new(top);
        let result = self.call(question, records, |row| {
            let original = originals
                .borrow_mut()
                .pop_front()
                .expect("one original per row");
            ranking.push(original, row);
        });
        for (original, row) in ranking.finish() {
            sink(original, row);
        }
        self.cancel.complete(result)
    }
}
