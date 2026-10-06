//! Explicit recordings retain original bodies in the answers' transaction.

use super::{Mode, Row, Store, insert};
use crate::engine::{Cancel, error::Error};

pub(crate) struct Original {
    pub(crate) digest: String,
    pub(crate) url: String,
    pub(crate) request: Vec<u8>,
    pub(crate) response: Vec<u8>,
}

impl Store {
    pub(super) fn write_original(
        &mut self,
        rows: &[Row<'_>],
        original: Option<&Original>,
        cancel: &Cancel,
    ) -> Result<(), Error> {
        let Some(original) = original.filter(|_| self.mode == Mode::Record) else {
            return self.write(rows, cancel);
        };
        if self.connection.is_none() {
            self.connect(cancel)?;
        }
        let connection = self.connection.as_ref().ok_or(Error::RecordingStorage)?;
        self.transaction(cancel, connection, || {
            self.waiting(cancel, || {
                connection.execute_batch(
                    "CREATE TABLE IF NOT EXISTS exchanges (
                        digest TEXT PRIMARY KEY, url TEXT NOT NULL,
                        request BLOB NOT NULL, response BLOB NOT NULL
                    )",
                )?;
                insert(connection, rows)?;
                connection.execute(
                    "INSERT INTO exchanges (digest, url, request, response) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(digest) DO UPDATE SET response=excluded.response",
                    (
                        &original.digest,
                        &original.url,
                        &original.request,
                        &original.response,
                    ),
                )?;
                Ok(())
            })
        })
    }
}
