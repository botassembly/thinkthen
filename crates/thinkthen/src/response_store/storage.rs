//! Transactional immutable exchanges and separately mutable cache eligibility.
use super::{ResponseMode, ResponseStore, error, key, sql};
use crate::{Cancel, Code, Error};
use rusqlite::{
    Connection, OpenFlags, OptionalExtension, Transaction, TransactionBehavior, params,
};
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, TryLockError};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const APPLICATION: i64 = 1_414_812_243;

#[derive(Debug)]
pub(crate) struct Store {
    connection: Mutex<Connection>,
    directory: super::access::Directory,
    config: ResponseStore,
    high_water: i64,
}

impl Store {
    #[expect(
        clippy::disallowed_methods,
        reason = "construction polls a competing process's initial schema commit only until the configured busy deadline"
    )]
    pub(crate) fn open(config: ResponseStore) -> Result<Self, Error> {
        let replay = config.mode == ResponseMode::Replay;
        let (directory, new) = super::access::Directory::open(&config.path, replay)?;
        directory.check()?;
        let flags = if replay {
            OpenFlags::SQLITE_OPEN_READ_ONLY
        } else {
            OpenFlags::SQLITE_OPEN_READ_WRITE
        } | OpenFlags::SQLITE_OPEN_NO_MUTEX
            | OpenFlags::SQLITE_OPEN_NOFOLLOW;
        let connection = Connection::open_with_flags(directory.database(), flags).map_err(sql)?;
        directory.check()?;
        connection
            .busy_timeout(Duration::from_millis(config.busy_ms))
            .map_err(sql)?;
        connection
            .set_limit(
                rusqlite::limits::Limit::SQLITE_LIMIT_LENGTH,
                i32::try_from(config.max_bytes.min(i32::MAX as u64))
                    .map_err(|_| error(Code::LocalStoreQuota))?,
            )
            .map_err(sql)?;
        connection
            .execute_batch("PRAGMA foreign_keys=ON; PRAGMA trusted_schema=OFF;")
            .map_err(sql)?;
        if new {
            connection.execute_batch("BEGIN IMMEDIATE").map_err(sql)?;
            connection
                .execute_batch(include_str!("schema.sql"))
                .map_err(sql)?;
            connection.execute_batch("COMMIT").map_err(sql)?;
        }
        // A competing constructor may observe the private empty file before its
        // creator commits the schema. It may wait, but must never initialize an
        // existing truncated store itself.
        let deadline = std::time::Instant::now() + Duration::from_millis(config.busy_ms);
        let application: i64 = loop {
            let application = connection
                .query_row("PRAGMA application_id", [], |row| row.get(0))
                .map_err(sql)?;
            if application != 0 {
                break application;
            }
            if std::time::Instant::now() >= deadline {
                return Err(error(Code::LocalStoreCorrupt));
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        if application != APPLICATION {
            return Err(error(Code::LocalStoreFormat));
        }
        let format: i64 = connection
            .query_row("SELECT format FROM store", [], |row| row.get(0))
            .map_err(|_| error(Code::LocalStoreFormat))?;
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .map_err(sql)?;
        if format != key::FORMAT || version != key::FORMAT {
            return Err(error(Code::LocalStoreFormat));
        }
        let healthy: String = connection
            .query_row("PRAGMA quick_check(1)", [], |row| row.get(0))
            .map_err(sql)?;
        if healthy != "ok"
            || connection
                .prepare("PRAGMA foreign_key_check")
                .map_err(sql)?
                .exists([])
                .map_err(sql)?
        {
            return Err(error(Code::LocalStoreCorrupt));
        }
        let malformed: bool = connection.query_row("SELECT (SELECT COUNT(*) FROM store)!=1 OR EXISTS(SELECT 1 FROM exchanges WHERE size!=length(metadata)+length(request)+length(response)+length(digest) OR length(digest)!=32 OR received<0 OR recorded NOT IN (0,1)) OR EXISTS(SELECT 1 FROM answers WHERE size!=length(answer)+length(key)+length(digest) OR length(key)!=32 OR length(digest)!=32 OR slot<0)", [], |row| row.get(0)).map_err(sql)?;
        if malformed {
            return Err(error(Code::LocalStoreCorrupt));
        }
        if !replay {
            connection
                .pragma_update(None, "journal_mode", "DELETE")
                .map_err(sql)?;
            connection
                .pragma_update(None, "synchronous", "FULL")
                .map_err(sql)?;
            let page_size: u64 = connection
                .query_row("PRAGMA page_size", [], unsigned)
                .map_err(sql)?;
            let pages = config.allocation()? / page_size;
            let actual: u64 = connection
                .query_row(&format!("PRAGMA max_page_count={pages}"), [], unsigned)
                .map_err(sql)?;
            if actual > pages {
                return Err(error(Code::LocalStoreQuota));
            }
        }
        let high_water = connection
            .query_row("SELECT COALESCE(MAX(id),0) FROM exchanges", [], |row| {
                row.get(0)
            })
            .map_err(sql)?;
        connection
            .busy_handler(Some(super::waiting::busy))
            .map_err(sql)?;
        Ok(Self {
            connection: Mutex::new(connection),
            directory,
            config,
            high_water,
        })
    }
    fn connection(
        &self,
        waiting: &super::waiting::Wait,
    ) -> Result<MutexGuard<'_, Connection>, Error> {
        loop {
            match self.connection.try_lock() {
                Ok(held) => return Ok(held),
                Err(TryLockError::Poisoned(held)) => return Ok(held.into_inner()),
                Err(TryLockError::WouldBlock) => waiting.pause()?,
            }
        }
    }
    pub(crate) fn config(&self) -> &ResponseStore {
        &self.config
    }
    pub(crate) fn replay(&self) -> bool {
        self.config.mode == ResponseMode::Replay
    }
    pub(crate) fn body_limit(&self) -> u64 {
        self.config.max_bytes
    }

    pub(crate) fn lookup(
        &self,
        keys: &[[u8; 32]],
        cancel: &Cancel,
    ) -> Result<Vec<Option<Vec<u8>>>, Error> {
        self.directory.check()?;
        if self.config.mode == ResponseMode::Record {
            return Ok(keys.iter().map(|_| None).collect());
        }
        let waiting = super::waiting::Wait::enter(cancel, self.config.busy_ms);
        let mut connection = self.connection(&waiting)?;
        let tx = connection
            .transaction_with_behavior(if self.replay() {
                TransactionBehavior::Deferred
            } else {
                TransactionBehavior::Immediate
            })
            .map_err(sql)?;
        let now = if self.replay() { 0 } else { now()? };
        let mut checked = HashMap::new();
        let touch = if self.replay() { 0 } else { next_access(&tx)? };
        let mut answers = Vec::with_capacity(keys.len());
        for key in keys {
            cancelled(cancel)?;
            let location: Option<(i64, i64, i64)> = if self.replay() {
                tx.query_row("SELECT a.exchange,a.slot,e.received FROM answers a JOIN exchanges e ON e.id=a.exchange WHERE a.key=?1 AND e.recorded=1 AND e.id<=?2 ORDER BY e.id DESC LIMIT 1", params![key.as_slice(),self.high_water], |row| Ok((row.get(0)?,row.get(1)?,row.get(2)?))).optional().map_err(sql)?
            } else {
                tx.query_row("SELECT c.exchange,c.slot,e.received FROM eligible c LEFT JOIN exchanges e ON e.id=c.exchange WHERE c.key=?1", [key.as_slice()], |row| Ok((row.get(0)?,row.get(1)?,row.get(2)?))).optional().map_err(sql)?
            };
            let Some((exchange, slot, received)) = location else {
                if self.replay() {
                    return Err(error(Code::LocalReplayMiss));
                }
                answers.push(None);
                continue;
            };
            let digest = match checked.get(&exchange) {
                Some(value) => value,
                None => {
                    let digest = check_exchange(&tx, exchange, self.config.max_bytes)?;
                    checked.entry(exchange).or_insert(digest)
                }
            };
            let len: u64 = tx
                .query_row(
                    "SELECT length(answer) FROM answers WHERE exchange=?1 AND slot=?2",
                    params![exchange, slot],
                    unsigned,
                )
                .map_err(sql)?;
            if len > self.config.max_bytes {
                return Err(error(Code::LocalStoreQuota));
            }
            let (answer, checksum): (Vec<u8>, Vec<u8>) = tx
                .query_row(
                    "SELECT answer,digest FROM answers WHERE exchange=?1 AND slot=?2 AND key=?3",
                    params![exchange, slot, key.as_slice()],
                    |row| Ok((row.get(0)?, row.get(1)?)),
                )
                .map_err(sql)?;
            if key::digest(&[digest, key.as_slice(), &slot.to_be_bytes(), &answer]) != checksum {
                return Err(error(Code::LocalStoreCorrupt));
            }
            if !self.replay()
                && (received > now || now.saturating_sub(received) as u64 >= self.config.max_age)
            {
                answers.push(None);
                continue;
            }
            if !self.replay() {
                tx.execute(
                    "UPDATE eligible SET touched=?1 WHERE key=?2",
                    params![touch, key.as_slice()],
                )
                .map_err(sql)?;
            }
            answers.push(Some(answer));
        }
        cancelled(cancel)?;
        tx.commit().map_err(sql)?;
        Ok(answers)
    }

    pub(crate) fn append(
        &self,
        metadata: &[u8],
        request: &[u8],
        response: &[u8],
        answers: &[([u8; 32], Vec<u8>)],
        cancel: &Cancel,
    ) -> Result<(), Error> {
        self.directory.check()?;
        if self.replay() {
            return Err(error(Code::LocalResponseStore));
        }
        cancelled(cancel)?;
        let received = now()?;
        let recorded = i64::from(self.config.mode == ResponseMode::Record);
        let digest = key::digest(&[
            metadata,
            request,
            response,
            &received.to_be_bytes(),
            &recorded.to_be_bytes(),
        ]);
        let size = metadata
            .len()
            .checked_add(request.len())
            .and_then(|n| n.checked_add(response.len()))
            .and_then(|n| n.checked_add(digest.len()))
            .ok_or_else(|| error(Code::LocalStoreQuota))?;
        let total = answers.iter().try_fold(size as u64, |total, (_, answer)| {
            total
                .checked_add(answer.len() as u64 + 64)
                .ok_or_else(|| error(Code::LocalStoreQuota))
        })?;
        if total > self.config.max_bytes {
            return Err(error(Code::LocalStoreQuota));
        }
        let waiting = super::waiting::Wait::enter(cancel, self.config.busy_ms);
        let mut connection = self.connection(&waiting)?;
        let tx = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(sql)?;
        let touch = next_access(&tx)?;
        tx.execute("INSERT INTO exchanges(recorded,received,metadata,request,response,digest,size) VALUES(?1,?2,?3,?4,?5,?6,?7)",params![recorded,received,metadata,request,response,digest,size as i64]).map_err(sql)?;
        let exchange = tx.last_insert_rowid();
        for (slot, (key, answer)) in answers.iter().enumerate() {
            cancelled(cancel)?;
            let slot = i64::try_from(slot).map_err(|_| error(Code::LocalStoreQuota))?;
            let checksum = key::digest(&[&digest, key.as_slice(), &slot.to_be_bytes(), answer]);
            tx.execute("INSERT INTO answers(exchange,slot,key,answer,digest,size) VALUES(?1,?2,?3,?4,?5,?6)",params![exchange,slot,key.as_slice(),answer,checksum,answer.len() as i64+64]).map_err(sql)?;
            tx.execute("INSERT INTO eligible(key,exchange,slot,touched) VALUES(?1,?2,?3,?4) ON CONFLICT(key) DO UPDATE SET exchange=excluded.exchange,slot=excluded.slot,touched=excluded.touched",params![key.as_slice(),exchange,slot,touch]).map_err(sql)?;
        }
        prune_orphans(&tx)?;
        while count(&tx, "SELECT COUNT(*) FROM eligible")? > self.config.max_entries {
            evict(&tx)?;
        }
        while retained(&tx)? > self.config.max_bytes {
            if count(&tx, "SELECT COUNT(*) FROM eligible")? == 0 {
                return Err(error(Code::LocalStoreQuota));
            }
            evict(&tx)?;
        }
        if !tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM exchanges WHERE id=?1)",
                [exchange],
                |row| row.get::<_, bool>(0),
            )
            .map_err(sql)?
        {
            return Err(error(Code::LocalStoreQuota));
        }
        cancelled(cancel)?;
        tx.commit().map_err(sql)?;
        Ok(())
    }
}

fn check_exchange(tx: &Transaction<'_>, id: i64, maximum: u64) -> Result<Vec<u8>, Error> {
    let size: u64 = tx.query_row("SELECT length(metadata)+length(request)+length(response)+length(digest) FROM exchanges WHERE id=?1",[id],unsigned).map_err(sql)?;
    if size > maximum {
        return Err(error(Code::LocalStoreQuota));
    }
    let (metadata, request, response, digest, received, recorded): (
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        Vec<u8>,
        i64,
        i64,
    ) = tx
        .query_row(
            "SELECT metadata,request,response,digest,received,recorded FROM exchanges WHERE id=?1",
            [id],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            },
        )
        .map_err(sql)?;
    if key::digest(&[
        &metadata,
        &request,
        &response,
        &received.to_be_bytes(),
        &recorded.to_be_bytes(),
    ]) != digest
    {
        return Err(error(Code::LocalStoreCorrupt));
    }
    Ok(digest)
}
fn count(tx: &Transaction<'_>, query: &str) -> Result<u64, Error> {
    tx.query_row(query, [], unsigned).map_err(sql)
}
fn retained(tx: &Transaction<'_>) -> Result<u64, Error> {
    count(
        tx,
        "SELECT COALESCE((SELECT SUM(size) FROM exchanges),0)+COALESCE((SELECT SUM(size) FROM answers),0)",
    )
}
fn prune_orphans(tx: &Transaction<'_>) -> Result<(), Error> {
    tx.execute("DELETE FROM answers WHERE exchange IN (SELECT id FROM exchanges WHERE recorded=0) AND NOT EXISTS(SELECT 1 FROM eligible WHERE eligible.exchange=answers.exchange AND eligible.slot=answers.slot)", []).map_err(sql)?;
    tx.execute("DELETE FROM exchanges WHERE recorded=0 AND NOT EXISTS(SELECT 1 FROM eligible WHERE eligible.exchange=exchanges.id)",[]).map_err(sql)?;
    Ok(())
}
fn evict(tx: &Transaction<'_>) -> Result<(), Error> {
    tx.execute(
        "DELETE FROM eligible WHERE key=(SELECT key FROM eligible ORDER BY touched,key LIMIT 1)",
        [],
    )
    .map_err(sql)?;
    prune_orphans(tx)
}
fn next_access(tx: &Transaction<'_>) -> Result<i64, Error> {
    tx.execute(
        "UPDATE store SET access=access+1 WHERE access<9223372036854775807",
        [],
    )
    .map_err(sql)?;
    tx.query_row(
        "SELECT access FROM store WHERE access<9223372036854775807",
        [],
        |row| row.get(0),
    )
    .map_err(|_| error(Code::LocalStoreQuota))
}
fn now() -> Result<i64, Error> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| i64::try_from(duration.as_secs()).ok())
        .ok_or_else(|| error(Code::LocalResponseStore))
}
fn cancelled(cancel: &Cancel) -> Result<(), Error> {
    if cancel.is_cancelled() {
        Err(crate::retry::cancelled())
    } else {
        Ok(())
    }
}

fn unsigned(row: &rusqlite::Row<'_>) -> rusqlite::Result<u64> {
    let value: i64 = row.get(0)?;
    value
        .try_into()
        .map_err(|_| rusqlite::Error::IntegralValueOutOfRange(0, value))
}
