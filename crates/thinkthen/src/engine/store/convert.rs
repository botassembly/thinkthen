//! `thinkthen cache convert DIR`, by ADR 0111 section 9: one fixture from the
//! union of what a folder holds.
//!
//! Current fixtures and live stores merge only when saved histories agree.
//! Conversion holds the live store's write lock through its removal.

use std::fs;
use std::io::{self, Write as _};
use std::path::Path;

#[cfg(not(windows))]
use std::time::Duration;

use rusqlite::Connection;

use super::{Entries, JSONL, SQLITE, exists, fixture};
#[cfg(not(windows))]
use super::{storage, waiting};
#[cfg(not(windows))]
use crate::engine::Cancel;
use crate::engine::error::Error;

#[cfg(windows)]
mod windows;

/// What one conversion did.
#[derive(Debug, Default)]
pub(crate) struct Summary {
    /// Answers the written fixture holds.
    pub(crate) answers: usize,
}

/// Write `DIR/thinkthen.jsonl` from what the folder holds, then remove its
/// live file.
///
/// # Errors
///
/// A fixture that does not parse, a live file that cannot be read, or an old
/// entry refuses the whole conversion before anything is written.
pub(crate) fn convert(folder: &Path) -> Result<Summary, Error> {
    let (jsonl, sqlite) = (folder.join(JSONL), folder.join(SQLITE));
    let has_sqlite = exists(&sqlite)?;
    // Windows keeps this handle across the read-only SQLite snapshot and
    // removal. It refuses competing writers before publishing a fixture.
    #[cfg(windows)]
    let _guard = has_sqlite.then(|| windows::guard(&sqlite)).transpose()?;
    #[cfg(windows)]
    let previous = if exists(&jsonl)? {
        Some(fs::read(&jsonl).map_err(|_| Error::RecordingStorage)?)
    } else {
        None
    };
    let mut held = if exists(&jsonl)? {
        fixture::read(&jsonl)?
    } else {
        Entries::default()
    };
    let live = if has_sqlite {
        Some(locked(&sqlite)?)
    } else {
        None
    };
    if let Some(connection) = &live {
        held = held.normalized()?;
        held.merge(
            Entries::read(connection)
                .and_then(Entries::normalized)
                .map_err(super::versioned::sqlite_error)?,
        )?;
    }
    let mut summary = Summary::default();
    held = held.normalized()?;
    refuse_old_files(folder)?;
    summary.answers = held.answers.len();
    replace(folder, &jsonl, held.written()?.as_bytes())?;
    if let Some(connection) = live {
        // Windows cannot remove an open file, so the store closes first there.
        // Unix removes it while the lock is held (ticket 0373).
        #[cfg(windows)]
        drop(connection);
        if fs::remove_file(&sqlite).is_err() {
            #[cfg(windows)]
            windows::restore(folder, &jsonl, previous.as_deref())?;
            return Err(Error::RecordingStorage);
        }
        sync_folder(folder).map_err(|_| Error::RecordingStorage)?;
        #[cfg(not(windows))]
        drop(connection);
    }
    Ok(summary)
}

/// Open the live file and take its write lock, waiting up to 30 seconds
/// for another writer. The lock ends when the connection drops.
#[cfg(not(windows))]
fn locked(sqlite: &Path) -> Result<Connection, Error> {
    let connection = super::opening::open(sqlite, rusqlite::OpenFlags::default())?;
    connection.busy_timeout(Duration::ZERO).map_err(storage)?;
    waiting(Duration::from_secs(30), &Cancel::default(), || {
        connection.execute_batch("BEGIN IMMEDIATE")
    })?;
    Ok(connection)
}

#[cfg(windows)]
fn locked(sqlite: &Path) -> Result<Connection, Error> {
    super::read_only(sqlite)
}

pub(super) fn refuse_old_files(folder: &Path) -> Result<(), Error> {
    if !folder.is_dir() {
        return Ok(());
    }
    for item in fs::read_dir(folder).map_err(|_| Error::RecordingStorage)? {
        let name = item.map_err(|_| Error::RecordingStorage)?.file_name();
        if let Some(name) = name.to_str().filter(|name| digest_named(name)) {
            return Err(Error::StoreVersion(name.to_owned()));
        }
    }
    Ok(())
}

/// A name the old recorder gave an entry: 64 lowercase hex figures and `.json`.
pub(super) fn digest_named(name: &str) -> bool {
    name.strip_suffix(".json").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
    })
}

/// Write `path` through a temporary file in the same folder, then rename it.
fn replace(folder: &Path, path: &Path, text: &[u8]) -> Result<(), Error> {
    let temporary = folder.join(format!(".{JSONL}.{}.tmp", std::process::id()));
    let written = (|| -> io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(text)?;
        file.sync_all()?;
        fs::rename(&temporary, path)?;
        sync_folder(folder)
    })();
    if written.is_err() {
        let _removed = fs::remove_file(&temporary);
    }
    written.map_err(|_| Error::RecordingStorage)
}

/// Make a rename in `folder` durable.
#[cfg(unix)]
pub(super) fn sync_folder(folder: &Path) -> io::Result<()> {
    fs::File::open(folder)?.sync_all()
}

#[cfg(not(unix))]
pub(super) fn sync_folder(_folder: &Path) -> io::Result<()> {
    Ok(())
}
