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
        self.waiting(cancel, || {
            connection.pragma_update(None, "user_version", super::STORE_VERSION)
        })?;
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
        // BEGIN is deferred: acquire the snapshot under the cancellable wait
        // before entry decoding can turn lock contention into a storage failure.
        let marker = self.data_version(connection, cancel)?;
        Entries::read(connection)
            .and_then(Entries::normalized)
            .map_err(super::versioned::sqlite_error)?;
        Ok(marker)
    }
}

#[cfg(test)]
mod tests {
    //! Stays: force a competing writer between deferred admission and snapshot decoding without a production test hook.
    use std::{sync::mpsc, time::Duration};

    use super::*;
    use crate::engine::store::{Mode, SQLITE};

    #[test]
    fn validation_waits_for_a_competing_writer_before_reading_its_snapshot() {
        let folder = super::super::tests::scratch();
        let cancel = Cancel::default();
        let mut store =
            Store::open(folder.path(), Mode::Cache, false, None, &cancel).expect("store");
        store.connect(&cancel).expect("initialize");
        let connection = store.connection.take().expect("connection");
        let writer = Connection::open(folder.path().join(SQLITE)).expect("writer");
        writer
            .execute_batch("BEGIN EXCLUSIVE")
            .expect("writer lock");
        cancel.fire();
        assert!(matches!(
            store.validate(&connection, &cancel),
            Err(Error::Cancelled)
        ));
        let cancel = Cancel::default();
        let (started, starting) = mpsc::channel();
        let (finished, completed) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            started.send(()).expect("started");
            finished
                .send(store.validate(&connection, &cancel))
                .expect("result");
        });
        starting.recv().expect("worker started");
        let premature = completed.recv_timeout(Duration::from_millis(50));
        writer.execute_batch("ROLLBACK").expect("release writer");
        worker.join().expect("worker");
        assert!(premature.is_err(), "validation did not wait: {premature:?}");
        assert!(completed.recv().expect("validation completed").is_ok());
    }
}
