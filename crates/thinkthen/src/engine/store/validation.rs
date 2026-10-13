//! Current store initialization, validation and lookup index maintenance.

use super::{Entries, SCHEMA, Store};
use crate::engine::{Cancel, error::Error};
use rusqlite::Connection;

impl Store {
    pub(super) fn data_version(
        &self,
        connection: &Connection,
        cancel: &Cancel,
    ) -> Result<i64, Error> {
        self.waiting(cancel, || {
            connection.query_row("PRAGMA data_version", [], |row| row.get(0))
        })
    }

    pub(super) fn ensure_index(
        &self,
        connection: &Connection,
        cancel: &Cancel,
    ) -> Result<(), Error> {
        if self.index_exists(connection, cancel)? {
            return Ok(());
        }
        self.transaction(cancel, connection, || {
            self.create_missing_index(connection, cancel)
        })
    }

    fn index_exists(&self, connection: &Connection, cancel: &Cancel) -> Result<bool, Error> {
        self.waiting(cancel, || {
            connection.query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='index' AND name='answers_route_question_state'",
                [],
                |row| row.get::<_, i64>(0),
            )
        })
        .map(|count| count != 0)
    }

    fn create_missing_index(&self, connection: &Connection, cancel: &Cancel) -> Result<(), Error> {
        if self.index_exists(connection, cancel)? {
            return Ok(());
        }
        self.waiting(cancel, || {
            connection.execute_batch(
                "CREATE INDEX IF NOT EXISTS answers_route_question_state ON answers(url,model,question,state)",
            )
        })
    }

    pub(super) fn initialize(
        &self,
        connection: &Connection,
        fixture: Option<&Entries>,
        cancel: &Cancel,
    ) -> Result<(), Error> {
        self.waiting(cancel, || {
            connection.execute_batch("PRAGMA auto_vacuum = INCREMENTAL")
        })?;
        self.transaction(cancel, connection, || {
            self.initialize_locked(connection, fixture, cancel)
        })
    }

    fn initialize_locked(
        &self,
        connection: &Connection,
        fixture: Option<&Entries>,
        cancel: &Cancel,
    ) -> Result<(), Error> {
        let version: i64 = self.waiting(cancel, || {
            connection.query_row("PRAGMA user_version", [], |row| row.get(0))
        })?;
        if version != 0 {
            return Ok(());
        }
        self.waiting(cancel, || connection.execute_batch(SCHEMA))?;
        fixture.map_or(Ok(()), |entries| {
            self.waiting(cancel, || entries.insert_all(connection))
        })
    }

    pub(super) fn validate(&self, connection: &Connection, cancel: &Cancel) -> Result<i64, Error> {
        self.waiting(cancel, || connection.execute_batch("BEGIN DEFERRED"))?;
        let checked = self.validate_current(connection, cancel);
        let ended = self.waiting(cancel, || connection.execute_batch("ROLLBACK"));
        let marker = checked?;
        ended?;
        Ok(marker)
    }

    fn validate_current(&self, connection: &Connection, cancel: &Cancel) -> Result<i64, Error> {
        Entries::read(connection)
            .and_then(Entries::normalized)
            .map_err(super::versioned::sqlite_error)?;
        self.data_version(connection, cancel)
    }
}
