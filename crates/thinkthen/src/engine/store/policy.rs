//! Commit accepted storage permission at the existing transaction boundary.

use super::{Mode, Row, SQLITE, Store, exists};
use crate::engine::{Cancel, error::Error};

impl Store {
    pub(crate) fn accept(
        &mut self,
        rows: &[Row<'_>],
        storable: bool,
        cancel: &Cancel,
    ) -> Result<(), Error> {
        if storable {
            return self.write(rows, cancel);
        }
        match self.mode {
            Mode::Record => Err(Error::RecordingForbidden),
            Mode::Refresh => self.evict(rows, cancel),
            Mode::Cache | Mode::Replay => Ok(()),
        }
    }

    fn evict(&mut self, rows: &[Row<'_>], cancel: &Cancel) -> Result<(), Error> {
        // A never-created working cache has nothing stale to remove.
        if self.connection.is_none() {
            if !exists(&self.folder.join(SQLITE))? && !exists(&self.folder.join(super::JSONL))? {
                return Ok(());
            }
            self.connect(cancel)?;
        }
        let connection = self.connection.as_ref().ok_or(Error::RecordingStorage)?;
        self.transaction(cancel, connection, || self.delete(connection, rows, cancel))
    }

    fn delete(
        &self,
        connection: &rusqlite::Connection,
        rows: &[Row<'_>],
        cancel: &Cancel,
    ) -> Result<(), Error> {
        for row in rows {
            self.waiting(cancel, || {
                connection.execute(
                    "DELETE FROM answers WHERE key = ?1",
                    [row.key.bytes().as_slice()],
                )
            })?;
        }
        Ok(())
    }
}
