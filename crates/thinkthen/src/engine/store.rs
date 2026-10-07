//! The one question store, by ADR 0111 section 3.
//!
//! A live cache is `thinkthen.sqlite` in its folder. A committed fixture is
//! `thinkthen.jsonl` with the same columns, which a replay loads into an
//! in-memory database. Writes take their lock before they read, a lookup
//! holds no lock past itself, and a busy file is waited on for at most 30
//! seconds with the call's stop checked between waits.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OpenFlags};

use crate::core::ReportedUsage;
use crate::core::pack::{Ask, QuestionKey, State};
use crate::engine::Cancel;
use crate::engine::error::Error;

mod convert;
mod fixture;
mod images;
mod rows;
use rows::{found, insert, signed};
mod migration;
mod mismatch;
mod original;
mod policy;
mod timing;
mod versioned;
pub(crate) use original::Original;
mod probe;
mod prune;
pub(crate) use convert::convert;
pub(crate) use fixture::{Answer, Entries, Replayed};
pub(crate) use probe::Probe;
pub(crate) use prune::{Prune, counts, preview, run as prune, unused};

/// The live container's file name.
pub(crate) const SQLITE: &str = "thinkthen.sqlite";
/// The committed fixture's file name.
pub(crate) const JSONL: &str = "thinkthen.jsonl";

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS states (
  id     INTEGER PRIMARY KEY,
  sha256 BLOB NOT NULL UNIQUE,
  state  TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS answers (
  id            INTEGER PRIMARY KEY,
  key           BLOB NOT NULL UNIQUE,
  url           TEXT NOT NULL,
  model         TEXT NOT NULL,
  state         INTEGER NOT NULL REFERENCES states(id),
  question      TEXT NOT NULL,
  answer        TEXT NOT NULL,
  answered_by   TEXT NOT NULL,
  input_tokens  INTEGER,
  output_tokens INTEGER,
  taken_at      INTEGER NOT NULL,
  origin        TEXT NOT NULL,
  key_version   INTEGER NOT NULL,
  adapter       TEXT NOT NULL,
  observation_id TEXT NOT NULL,
  batch_size    INTEGER
);
PRAGMA user_version = 2;
";

/// How a call uses its folder, by the modes table of ADR 0111 section 3.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Mode {
    /// Look up, send nothing, write nothing.
    Replay,
    /// Send every question and replace each entry.
    Record,
    /// Look up, send the misses, write their answers.
    Cache,
    /// A cache that sends every question and replaces each entry.
    Refresh,
}

/// One stored answer as a lookup returns it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Found {
    pub(crate) key: QuestionKey,
    pub(crate) observation_id: crate::core::ObservationId,
    pub(crate) batch_size: Option<std::num::NonZeroU32>,
    pub(crate) answer: String,
    pub(crate) answered_by: String,
    pub(crate) usage: Option<ReportedUsage>,
}

/// One answer to write, with every column of its row.
pub(crate) struct Row<'a> {
    pub(crate) observation_id: crate::core::ObservationId,
    pub(crate) batch_size: Option<std::num::NonZeroU32>,
    pub(crate) key: QuestionKey,
    pub(crate) url: &'a str,
    pub(crate) model: &'a str,
    pub(crate) state: &'a State,
    pub(crate) question: &'a str,
    pub(crate) answer: &'a str,
    pub(crate) answered_by: &'a str,
    pub(crate) usage: Option<ReportedUsage>,
    pub(crate) taken_at: i64,
    pub(crate) origin: &'a str,
}

/// One call's store: its folder, its mode, and at most one connection.
pub(crate) struct Store {
    folder: PathBuf,
    mode: Mode,
    private: bool,
    connection: Option<Connection>,
    replayed: Option<Arc<Replayed>>,
    busy_limit: Duration,
}

impl std::fmt::Debug for Store {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Store")
            .field("mode", &self.mode)
            .finish_non_exhaustive()
    }
}

impl Store {
    /// Open a folder's store for one call. A replay reads the fixture into
    /// memory or opens the file read-only, and a folder holding both is
    /// refused. A writing mode checks the folder now, so a folder it cannot
    /// use fails before any key is read or request sent. It opens the file
    /// when it exists, importing a fixture into a new file, when a lookup or
    /// a write first needs it. The call's first send proves the folder
    /// writable first (`Probe`), so a run that sends nothing creates nothing.
    ///
    /// A replay uses `replayed`, the engine's copy of the fixture, when
    /// given, rather than reading the file again.
    pub(crate) fn open(
        folder: &Path,
        mode: Mode,
        private: bool,
        replayed: Option<Arc<Replayed>>,
    ) -> Result<Self, Error> {
        if fs::metadata(folder).is_ok_and(|metadata| metadata.is_file()) {
            return Err(Error::RecordingPathIsFile);
        }
        let mut store = Self {
            folder: folder.to_owned(),
            mode,
            private,
            connection: None,
            replayed: None,
            busy_limit: Duration::from_secs(30),
        };
        let (sqlite, jsonl) = (folder.join(SQLITE), folder.join(JSONL));
        let (has_sqlite, has_jsonl) = (exists(&sqlite)?, exists(&jsonl)?);
        if mode == Mode::Replay {
            match (has_sqlite, has_jsonl) {
                (true, true) => return Err(Error::StoreAmbiguous),
                (false, true) => {
                    let read = replayed.map_or_else(|| Replayed::read(&jsonl).map(Arc::new), Ok);
                    store.replayed = Some(read?);
                }
                (true, false) => {
                    store.replayed = Some(Arc::new(Replayed::connection(&read_only(&sqlite)?)?))
                }
                (false, false) => {}
            }
        } else if fs::symlink_metadata(folder).is_ok() {
            // A dangling link, or a folder another user can read.
            fs::metadata(folder).map_err(|_| Error::RecordingStorage)?;
            if private {
                require_private(folder)?;
            }
        }
        Ok(store)
    }

    /// Writing modes validate/upgrade an existing store even when refresh skips lookup.
    pub(crate) fn prepared(mut self, cancel: &Cancel) -> Result<Self, Error> {
        if self.writes()
            && (exists(&self.folder.join(SQLITE))? || exists(&self.folder.join(JSONL))?)
        {
            self.connect(cancel)?;
        }
        Ok(self)
    }

    /// Whether this mode answers from the store.
    pub(crate) const fn looks_up(&self) -> bool {
        matches!(self.mode, Mode::Replay | Mode::Cache)
    }

    /// Whether this mode writes answers.
    pub(crate) const fn writes(&self) -> bool {
        !matches!(self.mode, Mode::Replay)
    }

    /// Whether this is a strict replay, where a miss is a failure.
    pub(crate) const fn replays(&self) -> bool {
        matches!(self.mode, Mode::Replay)
    }

    #[cfg(test)]
    pub(crate) const fn with_busy_limit(mut self, limit: Duration) -> Self {
        self.busy_limit = limit;
        self
    }

    /// What a send worker needs to prove this store can be written, or
    /// `None` for a replay, which writes nothing.
    pub(crate) fn probe(&self) -> Option<Probe> {
        self.writes()
            .then(|| Probe::new(self.folder.clone(), self.private))
    }

    /// Each key's stored answer, in key order, or `None` for a miss.
    pub(crate) fn lookup(
        &mut self,
        keys: &[QuestionKey],
        cancel: &Cancel,
    ) -> Result<Vec<Option<Found>>, Error> {
        self.lookup_with(keys, cancel, &HashMap::new())
    }

    fn lookup_with(
        &mut self,
        keys: &[QuestionKey],
        cancel: &Cancel,
        images: &HashMap<[u8; 32], &Ask>,
    ) -> Result<Vec<Option<Found>>, Error> {
        if self.connection.is_none()
            && self.writes()
            && (exists(&self.folder.join(SQLITE))? || exists(&self.folder.join(JSONL))?)
        {
            self.connect(cancel)?;
        }
        if let Some(replayed) = &self.replayed {
            return keys.iter().map(|key| replayed.get(key)).collect();
        }
        let Some(connection) = self.connection.as_ref() else {
            return Ok(vec![None; keys.len()]);
        };
        let mut by_key = HashMap::new();
        for chunk in keys.chunks(500) {
            by_key.extend(self.waiting(cancel, || images::select(connection, chunk, images))?);
        }
        keys.iter()
            .map(|key| by_key.get(key.bytes().as_slice()).cloned().transpose())
            .collect()
    }

    /// Write every answer of one reply in one transaction, replacing any
    /// entry under the same key.
    pub(crate) fn write(&mut self, rows: &[Row<'_>], cancel: &Cancel) -> Result<(), Error> {
        if rows.is_empty() || !self.writes() {
            return Ok(());
        }
        if self.connection.is_none() {
            self.connect(cancel)?;
        }
        let connection = self.connection.as_ref().ok_or(Error::RecordingStorage)?;
        self.transaction(cancel, connection, || {
            self.waiting(cancel, || insert(connection, rows))
        })
    }

    /// Run `body` inside one write transaction, rolling back on failure.
    fn transaction(
        &self,
        cancel: &Cancel,
        connection: &Connection,
        body: impl FnOnce() -> Result<(), Error>,
    ) -> Result<(), Error> {
        self.waiting(cancel, || connection.execute_batch("BEGIN IMMEDIATE"))?;
        let done =
            body().and_then(|()| self.waiting(cancel, || connection.execute_batch("COMMIT")));
        if done.is_err() && !connection.is_autocommit() {
            let _rolled_back = connection.execute_batch("ROLLBACK");
        }
        done
    }

    /// Open the file for writing, creating the folder, the file and the
    /// schema as needed. A fixture beside a file without the schema is
    /// imported in the schema's transaction, so the version commits with it.
    fn connect(&mut self, cancel: &Cancel) -> Result<(), Error> {
        let sqlite = self.folder.join(SQLITE);
        if self.private && self.folder.exists() {
            require_private(&self.folder)?;
        }
        make_folder(&self.folder)?;
        create_private(&sqlite)?;
        let connection = Connection::open(&sqlite).map_err(storage)?;
        connection.busy_timeout(Duration::ZERO).map_err(storage)?;
        self.waiting(cancel, || {
            connection.execute_batch(
                "PRAGMA journal_mode = DELETE; PRAGMA synchronous = FULL; PRAGMA secure_delete = ON;",
            )
        })?;
        let version: i64 = self.waiting(cancel, || {
            connection.query_row("PRAGMA user_version", [], |row| row.get(0))
        })?;
        if version == 0 {
            let jsonl = self.folder.join(JSONL);
            let fixture = if exists(&jsonl)? {
                Some(fixture::read(&jsonl)?.normalized()?)
            } else {
                None
            };
            self.waiting(cancel, || {
                connection.execute_batch("PRAGMA auto_vacuum = INCREMENTAL")
            })?;
            let import =
                |entries: &Entries| self.waiting(cancel, || entries.insert_all(&connection));
            self.transaction(cancel, &connection, || {
                self.waiting(cancel, || connection.execute_batch(SCHEMA))?;
                fixture.as_ref().map_or(Ok(()), import)
            })?;
        } else {
            self.migrate(&connection, cancel)?;
        }
        self.connection = Some(connection);
        Ok(())
    }

    /// Run one statement, waiting while another connection holds the file,
    /// checking the call's stop between waits, for at most the busy limit.
    fn waiting<T>(
        &self,
        cancel: &Cancel,
        run: impl FnMut() -> rusqlite::Result<T>,
    ) -> Result<T, Error> {
        waiting(self.busy_limit, cancel, run)
    }
}

/// Run one statement, waiting while another connection holds the file,
/// checking the stop between waits, for at most `limit`. Every connection
/// turns SQLite's own busy wait off, so this is the only one.
pub(crate) fn waiting<T>(
    limit: Duration,
    cancel: &Cancel,
    mut run: impl FnMut() -> rusqlite::Result<T>,
) -> Result<T, Error> {
    let started = Instant::now();
    let mut wait = Duration::from_millis(2);
    loop {
        let error = match run() {
            Ok(value) => return Ok(value),
            Err(error) => error,
        };
        if hot_journal(&error) {
            return Err(Error::StoreHotJournal);
        }
        if !busy(&error) {
            return Err(storage(error));
        }
        let left = limit.saturating_sub(started.elapsed());
        if left.is_zero() {
            return Err(Error::RecordingStorage);
        }
        if let Some(stop) = cancel.wait(wait.min(left)) {
            return Err(stop);
        }
        wait = (wait * 2).min(Duration::from_millis(100));
    }
}

/// One stored answer beside its key's bytes.
type Stored = (Vec<u8>, Result<Found, Error>);

/// Unix seconds now, the time a reply arrived.
pub(crate) fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            i64::try_from(elapsed.as_secs()).unwrap_or(i64::MAX)
        })
}

fn read_only(path: &Path) -> Result<Connection, Error> {
    let connection = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(storage)?;
    connection.busy_timeout(Duration::ZERO).map_err(storage)?;
    Ok(connection)
}

fn exists(path: &Path) -> Result<bool, Error> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(Error::RecordingStorage),
    }
}

/// Create the file with mode `0600` before SQLite opens it, and say whether
/// this call made it.
fn create_private(path: &Path) -> Result<bool, Error> {
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    match options.open(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(false),
        Err(_) => Err(Error::RecordingStorage),
    }
}

/// Refuse a platform default cache folder anyone but its owner may use.
#[cfg(unix)]
fn require_private(folder: &Path) -> Result<(), Error> {
    use std::os::unix::fs::PermissionsExt as _;
    let metadata = fs::metadata(folder).map_err(|_| Error::RecordingStorage)?;
    if metadata.permissions().mode() & 0o777 == 0o700 {
        Ok(())
    } else {
        Err(Error::DefaultCachePrivate)
    }
}

#[cfg(not(unix))]
#[expect(clippy::unnecessary_wraps, reason = "Unix checks the folder's mode")]
const fn require_private(_folder: &Path) -> Result<(), Error> {
    Ok(())
}

pub(crate) fn make_folder(folder: &Path) -> Result<(), Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(folder)
            .map_err(|_| Error::RecordingStorage)
    }
    #[cfg(not(unix))]
    fs::create_dir_all(folder).map_err(|_| Error::RecordingStorage)
}

fn busy(error: &rusqlite::Error) -> bool {
    matches!(
        error.sqlite_error_code(),
        Some(rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked)
    )
}

fn hot_journal(error: &rusqlite::Error) -> bool {
    matches!(error, rusqlite::Error::SqliteFailure(failure, _)
        if failure.extended_code == rusqlite::ffi::SQLITE_READONLY_ROLLBACK)
}

fn storage(error: rusqlite::Error) -> Error {
    if hot_journal(&error) {
        Error::StoreHotJournal
    } else {
        Error::RecordingStorage
    }
}

#[cfg(test)]
mod tests;
