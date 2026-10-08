//! Atomic upgrade under the existing bounded transaction and cancellation rules.

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

    pub(super) fn migrate(&self, connection: &Connection, cancel: &Cancel) -> Result<i64, Error> {
        let version: u32 = self.waiting(cancel, || {
            connection.query_row("PRAGMA user_version", [], |row| row.get(0))
        })?;
        if version == 2 {
            self.waiting(cancel, || connection.execute_batch("BEGIN DEFERRED"))?;
            let checked = self.validate_current(connection, cancel);
            let ended = self.waiting(cancel, || connection.execute_batch("ROLLBACK"));
            let marker = checked?;
            ended?;
            return Ok(marker);
        }
        let mut changed = false;
        let mut marker = 0;
        self.transaction(cancel, connection, || {
            self.migrate_legacy_locked(connection, cancel, &mut changed, &mut marker)
        })?;
        if changed {
            self.migrate(connection, cancel)
        } else {
            Ok(marker)
        }
    }

    fn validate_current(&self, connection: &Connection, cancel: &Cancel) -> Result<i64, Error> {
        let version: u32 = self.waiting(cancel, || {
            connection.query_row("PRAGMA user_version", [], |row| row.get(0))
        })?;
        if version != 2 {
            return Err(super::versioned::invalid());
        }
        Entries::read(connection)
            .and_then(Entries::normalized)
            .map_err(super::versioned::sqlite_error)?;
        self.data_version(connection, cancel)
    }

    fn migrate_legacy_locked(
        &self,
        connection: &Connection,
        cancel: &Cancel,
        changed: &mut bool,
        marker: &mut i64,
    ) -> Result<(), Error> {
        let version: u32 = self.waiting(cancel, || {
            connection.query_row("PRAGMA user_version", [], |row| row.get(0))
        })?;
        if version == 2 {
            *changed = true;
            return Ok(());
        }
        let entries = Entries::read(connection)
            .and_then(Entries::normalized)
            .map_err(super::versioned::sqlite_error)?;
        *marker = self.data_version(connection, cancel)?;
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
    }
}
