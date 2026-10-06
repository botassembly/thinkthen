//! Atomic upgrade under the existing bounded transaction and cancellation rules.

use super::{Entries, Store};
use crate::engine::{Cancel, error::Error};
use rusqlite::Connection;

impl Store {
    pub(super) fn migrate(&self, connection: &Connection, cancel: &Cancel) -> Result<(), Error> {
        self.transaction(cancel, connection, || {
            let version: u32 = self.waiting(cancel, || {
                connection.query_row("PRAGMA user_version", [], |row| row.get(0))
            })?;
            let entries = Entries::read(connection)?.normalized()?;
            if version == 2 {
                return Ok(());
            }
            self.waiting(cancel, || {
                connection.execute_batch(
                    "ALTER TABLE answers ADD COLUMN key_version INTEGER;
                 ALTER TABLE answers ADD COLUMN adapter TEXT;
                 ALTER TABLE answers ADD COLUMN observation_id TEXT;
                 ALTER TABLE answers ADD COLUMN batch_size INTEGER;
                 DELETE FROM answers;",
                )
            })?;
            self.waiting(cancel, || entries.insert_all(connection))?;
            self.waiting(cancel, || connection.execute_batch("PRAGMA user_version=2"))
        })
    }
}
