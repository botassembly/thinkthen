//! `cache prune`, `cache unused` and the counts `status` reads, as short SQL
//! over the question store, by ADR 0111 section 10.
//!
//! Bytes are the store file's allocated bytes. Prune reckons each answer's
//! share of them by the length of its stored text, so a trimmed file sits
//! near its target.

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;
use std::time::Duration;

use rusqlite::{Connection, OpenFlags};

use super::{JSONL, SQLITE, exists, fixture, storage, waiting};
use crate::engine::Cancel;
use crate::engine::error::Error;

/// A model the requests asked for and no reply named is the alias, and
/// pruning by it would remove every answer.
const ALIAS: &str = "--answered-by-other-than names the model the requests asked for, \
    and no reply names it, so prune removed nothing; \
    name the version a result's meta.model shows, not the alias passed to --model";

/// A model no reply names would remove every answer, which is what a typo does.
const UNKNOWN: &str = "--answered-by-other-than names a model no reply in the folder names, \
    so prune removed nothing; name the version a result's meta.model shows, \
    or delete the folder to remove every answer";

/// How long prune waits for another process's write.
const BUSY: Duration = Duration::from_secs(30);

/// What one prune selects before the size trim.
#[derive(Debug)]
pub(crate) struct Prune {
    pub(crate) max_size: u64,
    pub(crate) older_than: Option<Duration>,
    pub(crate) answered_by_other_than: Option<String>,
}

/// The answers one prune chose, oldest first, and what stays.
#[derive(Debug, Default)]
pub(crate) struct Selection {
    /// Each chosen answer's question key, in deletion order.
    pub(crate) keys: Vec<Vec<u8>>,
    pub(crate) bytes: u64,
    pub(crate) kept: u64,
    pub(crate) kept_bytes: u64,
}

/// What a committed prune removed and what remains.
#[derive(Debug, Default)]
pub(crate) struct Pruned {
    pub(crate) removed: u64,
    pub(crate) removed_bytes: u64,
    pub(crate) remaining: u64,
    pub(crate) remaining_bytes: u64,
}

/// What `status` counts in a cache folder.
#[derive(Debug, Default)]
pub(crate) struct Counts {
    /// Answers in `thinkthen.sqlite`.
    pub(crate) answers: u64,
    /// The file's allocated bytes.
    pub(crate) bytes: u64,
    /// Old digest-named entries the store ignores until `cache convert`.
    pub(crate) old_entries: u64,
}

/// One stored answer as prune weighs it.
struct Weighed {
    key: Vec<u8>,
    model: String,
    answered_by: String,
    taken_at: i64,
    text: u64,
}

/// Preview what `run` would remove, changing nothing.
pub(crate) fn preview(folder: &Path, options: &Prune, now: i64) -> Result<Selection, Error> {
    // A read-only open writes nothing, so a preview changes no byte.
    let Some(connection) = open(folder, false)? else {
        return Ok(Selection::default());
    };
    let size = file_bytes(&connection)?;
    select(&weighed(&connection)?, options, now, size)
}

/// Remove what the selectors and the size target choose, then the states
/// no answer uses, then give the freed pages back to the file system.
pub(crate) fn run(folder: &Path, options: &Prune, now: i64) -> Result<Pruned, Error> {
    let Some(connection) = open(folder, true)? else {
        return Ok(Pruned::default());
    };
    let cancel = Cancel::default();
    let before = file_bytes(&connection)?;
    let answers = weighed(&connection)?;
    let selection = select(&answers, options, now, before)?;
    run_sql(&cancel, &connection, "BEGIN IMMEDIATE")?;
    let removed = remove(&connection, &selection.keys)
        .and_then(|()| {
            connection.execute(
                "DELETE FROM states WHERE id NOT IN (SELECT state FROM answers)",
                [],
            )
        })
        .map_err(storage)
        .and_then(|_| run_sql(&cancel, &connection, "COMMIT"));
    if removed.is_err() && !connection.is_autocommit() {
        let _rolled_back = connection.execute_batch("ROLLBACK");
    }
    removed?;
    run_sql(&cancel, &connection, "PRAGMA incremental_vacuum")?;
    let after = file_bytes(&connection)?;
    let count = |keys: usize| u64::try_from(keys).map_err(|_| Error::RecordingStorage);
    Ok(Pruned {
        removed: count(selection.keys.len())?,
        removed_bytes: before.saturating_sub(after),
        remaining: count(answers.len())? - count(selection.keys.len())?,
        remaining_bytes: after,
    })
}

/// The question keys a folder's store holds and `used` lacks, in order. A
/// fixture is read as it is replayed, and a folder holding both files is
/// refused as a replay refuses it.
pub(crate) fn unused(folder: &Path, used: &HashSet<String>) -> Result<Vec<String>, Error> {
    let metadata = fs::symlink_metadata(folder).map_err(|_| Error::RecordingStorage)?;
    if !metadata.is_dir() {
        return Err(Error::RecordingStorage);
    }
    let (sqlite, jsonl) = (folder.join(SQLITE), folder.join(JSONL));
    let keys: Vec<String> = match (exists(&sqlite)?, exists(&jsonl)?) {
        (true, true) => return Err(Error::StoreAmbiguous),
        (false, true) => fixture::read(&jsonl)?.answers.into_keys().collect(),
        (true, false) => {
            let connection = open(folder, false)?.ok_or(Error::RecordingStorage)?;
            let mut statement = connection
                .prepare("SELECT lower(hex(key)) FROM answers ORDER BY key")
                .map_err(storage)?;
            let rows = statement
                .query_map([], |row| row.get(0))
                .map_err(storage)?;
            rows.collect::<rusqlite::Result<_>>().map_err(storage)?
        }
        (false, false) => Vec::new(),
    };
    let mut unused: Vec<String> = keys.into_iter().filter(|key| !used.contains(key)).collect();
    unused.sort();
    Ok(unused)
}

/// Count a cache folder's store and its old entries. A missing folder
/// counts nothing.
pub(crate) fn counts(folder: &Path, private: bool) -> Result<Counts, Error> {
    let metadata = match fs::symlink_metadata(folder) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Counts::default()),
        Err(_) => return Err(Error::RecordingStorage),
        Ok(metadata) => metadata,
    };
    if !metadata.is_dir() {
        return Err(Error::CacheEntry);
    }
    if private {
        super::require_private(folder)?;
    }
    let mut old_entries = 0;
    for item in fs::read_dir(folder).map_err(|_| Error::RecordingStorage)? {
        let name = item.map_err(|_| Error::RecordingStorage)?.file_name();
        old_entries += u64::from(name.to_str().is_some_and(super::convert::digest_named));
    }
    let Some(connection) = open(folder, false)? else {
        return Ok(Counts {
            old_entries,
            ..Counts::default()
        });
    };
    let cancel = Cancel::default();
    let answers: i64 = waiting(BUSY, &cancel, || {
        connection.query_row("SELECT count(*) FROM answers", [], |row| row.get(0))
    })?;
    Ok(Counts {
        answers: u64::try_from(answers).map_err(|_| Error::RecordingStorage)?,
        bytes: file_bytes(&connection)?,
        old_entries,
    })
}

/// Choose what leaves: every answer a selector names, then the oldest
/// until the rest fit `options.max_size`. Taken-at time orders them, and
/// equal times sort by key.
fn select(
    answers: &[Weighed],
    options: &Prune,
    now: i64,
    size: u64,
) -> Result<Selection, Error> {
    if let Some(model) = options.answered_by_other_than.as_deref()
        && !answers.is_empty()
        && !answers.iter().any(|answer| answer.answered_by == model)
    {
        let alias = answers.iter().any(|answer| answer.model == model);
        return Err(Error::Usage(if alias { ALIAS } else { UNKNOWN }));
    }
    let edge = options
        .older_than
        .map(|age| now.saturating_sub(i64::try_from(age.as_secs()).unwrap_or(i64::MAX)));
    let chosen = |answer: &Weighed| {
        edge.is_some_and(|edge| answer.taken_at < edge)
            || options
                .answered_by_other_than
                .as_deref()
                .is_some_and(|model| answer.answered_by != model)
    };
    let text: u64 = answers.iter().map(|answer| answer.text).sum();
    // One answer's share of the file, by the length of its text.
    let share = |bytes: u64| {
        u64::try_from(u128::from(size) * u128::from(bytes) / u128::from(text.max(1)))
            .unwrap_or(u64::MAX)
    };
    let mut order: Vec<&Weighed> = answers.iter().collect();
    order.sort_by(|left, right| (left.taken_at, &left.key).cmp(&(right.taken_at, &right.key)));
    let mut kept_text = order
        .iter()
        .filter(|answer| !chosen(answer))
        .map(|answer| answer.text)
        .sum::<u64>();
    let mut selection = Selection::default();
    for answer in order {
        let leaves = chosen(answer) || share(kept_text) > options.max_size;
        if leaves {
            if !chosen(answer) {
                kept_text -= answer.text;
            }
            selection.keys.push(answer.key.clone());
        }
    }
    selection.kept = u64::try_from(answers.len() - selection.keys.len())
        .map_err(|_| Error::RecordingStorage)?;
    selection.kept_bytes = share(kept_text);
    selection.bytes = size.saturating_sub(selection.kept_bytes);
    Ok(selection)
}

/// Open a folder's `thinkthen.sqlite`, or `None` when it has none.
fn open(folder: &Path, writable: bool) -> Result<Option<Connection>, Error> {
    let metadata = fs::symlink_metadata(folder).map_err(|_| Error::RecordingStorage)?;
    if !metadata.is_dir() {
        return Err(Error::RecordingStorage);
    }
    let path = folder.join(SQLITE);
    if !exists(&path)? {
        return Ok(None);
    }
    let flags = if writable {
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX
    } else {
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX
    };
    let connection = Connection::open_with_flags(&path, flags).map_err(storage)?;
    connection.busy_timeout(Duration::ZERO).map_err(storage)?;
    let cancel = Cancel::default();
    if writable {
        run_sql(
            &cancel,
            &connection,
            "PRAGMA journal_mode = DELETE; PRAGMA synchronous = FULL; PRAGMA secure_delete = ON;",
        )?;
    }
    let version: i64 = waiting(BUSY, &cancel, || {
        connection.query_row("PRAGMA user_version", [], |row| row.get(0))
    })?;
    if version != 1 {
        return Err(Error::RecordingStorage);
    }
    Ok(Some(connection))
}

fn run_sql(cancel: &Cancel, connection: &Connection, sql: &str) -> Result<(), Error> {
    waiting(BUSY, cancel, || connection.execute_batch(sql))
}

/// Every answer's key, models, time and text length.
fn weighed(connection: &Connection) -> Result<Vec<Weighed>, Error> {
    let cancel = Cancel::default();
    waiting(BUSY, &cancel, || {
        let mut statement = connection.prepare(
            "SELECT key, model, answered_by, taken_at,
               length(key) + length(CAST(url AS BLOB)) + length(CAST(model AS BLOB))
               + length(CAST(question AS BLOB)) + length(CAST(answer AS BLOB))
               + length(CAST(answered_by AS BLOB))
             FROM answers",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(Weighed {
                key: row.get(0)?,
                model: row.get(1)?,
                answered_by: row.get(2)?,
                taken_at: row.get(3)?,
                text: u64::try_from(row.get::<_, i64>(4)?).unwrap_or(0),
            })
        })?;
        rows.collect()
    })
}

fn remove(connection: &Connection, keys: &[Vec<u8>]) -> rusqlite::Result<()> {
    let mut statement = connection.prepare("DELETE FROM answers WHERE key = ?1")?;
    for key in keys {
        statement.execute([key])?;
    }
    Ok(())
}

/// The file's allocated bytes: its pages times the page size.
fn file_bytes(connection: &Connection) -> Result<u64, Error> {
    let cancel = Cancel::default();
    let pages: i64 = waiting(BUSY, &cancel, || {
        connection.query_row("PRAGMA page_count", [], |row| row.get(0))
    })?;
    let page: i64 = waiting(BUSY, &cancel, || {
        connection.query_row("PRAGMA page_size", [], |row| row.get(0))
    })?;
    u64::try_from(pages.saturating_mul(page)).map_err(|_| Error::RecordingStorage)
}
