//! Prepared recording work around one backend exchange.

use std::fs::{self, File};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::core::recording::{Digest, Entry, Exchange};
use crate::engine::cache_lock::{self, CacheLock, FolderGate};
use crate::engine::error::Error;

use self::fault::{StorageStageName, maybe_fail, maybe_fail_io};

mod fault;
mod identity;

#[cfg(test)]
pub(crate) fn fail_cleanup() {
    fault::STORAGE_FAULT.with(|value| value.set(Some(fault::StorageStage::Cleanup)));
}

static WRITES: AtomicU64 = AtomicU64::new(0);

/// Which folders `--record` and `--replay` named, once they agree.
#[derive(Debug)]
pub(crate) struct Recorder {
    folder: Option<PathBuf>,
    recording: bool,
    replaying: bool,
    private_default: bool,
    cache_answers: bool,
    /// Whether the folder's backend identity has matched once. The folder
    /// gate itself belongs to each operation, so no lock outlives a request.
    checked: AtomicBool,
}

/// Work selected before a key is read or a request is sent.
///
/// A replayed body keys its odds by the labels a record gave, so this derives
/// `Debug` only in tests.
#[cfg_attr(test, derive(Debug))]
pub(crate) enum PreparedRecording {
    Replay(Vec<u8>),
    Live(WritePermit),
}

/// An already-open private temporary entry and any missing-entry lock, under
/// the shared folder gate held until installation.
#[derive(Debug)]
pub(crate) struct WritePermit {
    write: Option<PreparedWrite>,
    _gate: Option<FolderGate>,
}

#[derive(Debug)]
struct PreparedWrite {
    folder: PathBuf,
    entry: PathBuf,
    partial: PathBuf,
    file: Option<File>,
    replace_damaged: bool,
    lock: Option<CacheLock>,
}

enum Existing {
    Missing,
    Valid(Vec<u8>),
    Damaged(String),
}

impl Recorder {
    #[cfg(test)]
    pub(crate) fn of(record: Option<&Path>, replay: Option<&Path>) -> Result<Self, Error> {
        Self::of_private(record, replay, false, false)
    }

    pub(crate) fn of_private(
        record: Option<&Path>,
        replay: Option<&Path>,
        private_default: bool,
        cache_answers: bool,
    ) -> Result<Self, Error> {
        let folder = match (record, replay) {
            (Some(recorded), Some(replayed)) if recorded != replayed => {
                return Err(Error::Defect(
                    "record and replay folders differ below the command edge",
                ));
            }
            (Some(named), _) | (_, Some(named)) => Some(named.to_owned()),
            (None, None) => None,
        };
        if folder
            .as_ref()
            .and_then(|path| fs::metadata(path).ok())
            .is_some_and(|metadata| metadata.is_file())
        {
            return Err(Error::RecordingPathIsFile);
        }
        Ok(Self {
            folder,
            recording: record.is_some(),
            replaying: replay.is_some(),
            private_default,
            cache_answers,
            checked: AtomicBool::new(false),
        })
    }

    pub(crate) const fn counts_cache_answers(&self) -> bool {
        self.cache_answers
    }

    pub(crate) const fn reported(&self) -> bool {
        self.folder.is_some() && !self.private_default
    }

    /// Open the shared folder gate one operation holds, checking the
    /// folder's backend identity the first time.
    fn gate(
        &self,
        exchange: &Exchange<'_>,
        name: &str,
        cancel: &crate::engine::Cancel,
    ) -> Result<Option<FolderGate>, Error> {
        let Some(folder) = self.folder.as_deref() else {
            return Ok(None);
        };
        if let Some(stop) = cancel.stop() {
            return Err(stop);
        }
        if self.recording {
            make_folder(folder)?;
        } else if !folder.exists() {
            return Err(Error::ReplayMiss(name.to_owned()));
        }
        if self.private_default {
            identity::require_private(folder)?;
        }
        let opened = cache_lock::shared_folder_cancelled(folder, cancel).map_err(storage)??;
        if !self.checked.load(Ordering::Acquire) {
            identity::check(folder, &exchange.backend_identity(), self.recording)?;
            self.checked.store(true, Ordering::Release);
        }
        Ok(Some(opened))
    }

    /// Decide replay or prepare every write resource before live work.
    #[cfg(test)]
    pub(crate) fn prepare(
        &self,
        exchange: &Exchange<'_>,
        digest: &Digest,
    ) -> Result<PreparedRecording, Error> {
        self.prepare_cancelled(exchange, digest, &crate::engine::Cancel::default())
    }

    pub(crate) fn prepare_cancelled(
        &self,
        exchange: &Exchange<'_>,
        digest: &Digest,
        cancel: &crate::engine::Cancel,
    ) -> Result<PreparedRecording, Error> {
        let Some(folder) = self.folder.as_ref() else {
            return Ok(PreparedRecording::Live(WritePermit {
                write: None,
                _gate: None,
            }));
        };
        let name = digest.file_name();
        let gate = self.gate(exchange, &name, cancel)?;
        Ok(
            match self.prepare_in(folder, name, exchange, digest, cancel)? {
                PreparedRecording::Live(permit) => PreparedRecording::Live(permit.under(gate)),
                replay @ PreparedRecording::Replay(_) => replay,
            },
        )
    }

    /// Decide replay or prepare the write while the folder gate is held.
    fn prepare_in(
        &self,
        folder: &Path,
        name: String,
        exchange: &Exchange<'_>,
        digest: &Digest,
        cancel: &crate::engine::Cancel,
    ) -> Result<PreparedRecording, Error> {
        let entry = folder.join(&name);
        let first = existing(&entry, exchange)?;

        if !self.recording {
            return replay_only(first, name);
        }
        if self.replaying
            && let Existing::Valid(response) = first
        {
            return Ok(PreparedRecording::Replay(response));
        }

        make_folder(folder)?;
        match first {
            Existing::Valid(_) => prepared_write(folder, entry, false, None),
            Existing::Missing | Existing::Damaged(_) => {
                let lock = cache_lock::acquire_cancelled(folder, digest.as_str(), cancel)
                    .map_err(storage)??;
                match existing(&entry, exchange)? {
                    Existing::Valid(response) if self.replaying => {
                        remove_lock(lock)?;
                        Ok(PreparedRecording::Replay(response))
                    }
                    Existing::Valid(_) => {
                        remove_lock(lock)?;
                        prepared_write(folder, entry, false, None)
                    }
                    Existing::Missing => prepared_write(folder, entry, false, Some(lock)),
                    Existing::Damaged(_) => prepared_write(folder, entry, true, Some(lock)),
                }
            }
        }
    }
}

impl WritePermit {
    fn under(mut self, gate: Option<FolderGate>) -> Self {
        self._gate = gate;
        self
    }

    /// Install a decoded backend response or compare it with the valid winner.
    pub(crate) fn finish(
        mut self,
        exchange: &Exchange<'_>,
        response: &[u8],
        name: &str,
    ) -> Result<(), Error> {
        let Some(mut write) = self.write.take() else {
            return Ok(());
        };
        let result = write.complete(exchange, response, name);
        let cleanup = remove_partial(&write.partial);
        match (result, cleanup) {
            (_, Err(error)) => Err(storage(error)),
            (result, Ok(())) => result,
        }
    }

    /// Abandon live work and report a cleanup failure if one occurs.
    pub(crate) fn cancel(mut self) -> Result<(), Error> {
        let Some(write) = self.write.take() else {
            return Ok(());
        };
        remove_partial(&write.partial).map_err(storage)
    }
}

impl Drop for WritePermit {
    fn drop(&mut self) {
        if let Some(write) = self.write.take() {
            let _cleanup = remove_partial(&write.partial);
        }
    }
}

impl PreparedWrite {
    fn complete(
        &mut self,
        exchange: &Exchange<'_>,
        response: &[u8],
        name: &str,
    ) -> Result<(), Error> {
        let entry = Entry::of(exchange, response)
            .map_err(|_| Error::Defect("a recorded exchange is not JSON"))?;
        let written = entry
            .written()
            .map_err(|_| Error::Defect("a recorded exchange could not be written as JSON"))?;
        let mut file = self
            .file
            .take()
            .ok_or(Error::Defect("a recording write permit has no file"))?;
        maybe_fail(StorageStageName::Write)?;
        file.write_all(written.as_bytes()).map_err(storage)?;
        maybe_fail(StorageStageName::FileSync)?;
        file.sync_all().map_err(storage)?;
        drop(file);

        maybe_fail(StorageStageName::Install)?;
        let mut result = if self.replace_damaged {
            fs::rename(&self.partial, &self.entry).map_err(storage)?;
            Ok(())
        } else {
            install_or_compare(
                &self.partial,
                &self.entry,
                exchange,
                written.as_bytes(),
                name,
            )
        };
        let valid_final = match final_existing(&self.entry, exchange) {
            Ok(Existing::Valid(_)) => true,
            Ok(Existing::Missing | Existing::Damaged(_)) => false,
            Err(error) => {
                result = Err(error);
                false
            }
        };
        if result.is_ok()
            && let Err(error) = sync_recording_directory(&self.folder)
        {
            result = Err(storage(error));
        }
        if valid_final
            && let Some(lock) = self.lock.take()
            && let Err(error) = remove_lock(lock)
        {
            result = Err(error);
        }
        result
    }
}

fn replay_only(existing: Existing, name: String) -> Result<PreparedRecording, Error> {
    match existing {
        Existing::Valid(response) => Ok(PreparedRecording::Replay(response)),
        Existing::Missing => Err(Error::ReplayMiss(name)),
        Existing::Damaged(why) => Err(Error::Entry(name, why)),
    }
}

fn existing(entry: &Path, exchange: &Exchange<'_>) -> Result<Existing, Error> {
    match fs::read(entry) {
        Ok(bytes) => match Entry::replayed(&bytes, exchange) {
            Ok(response) => Ok(Existing::Valid(response)),
            Err(error) => Ok(Existing::Damaged(error.to_string())),
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Existing::Missing),
        Err(error) => Err(storage(error)),
    }
}

fn final_existing(entry: &Path, exchange: &Exchange<'_>) -> Result<Existing, Error> {
    maybe_fail(StorageStageName::FinalRead)?;
    existing(entry, exchange)
}

fn prepared_write(
    folder: &Path,
    entry: PathBuf,
    replace_damaged: bool,
    lock: Option<CacheLock>,
) -> Result<PreparedRecording, Error> {
    let attempt = WRITES.fetch_add(1, Ordering::Relaxed);
    let name = entry
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or(Error::Defect("a recording entry has no file name"))?;
    let partial = folder.join(format!(".{}.{attempt}.{name}", process::id()));
    match fs::remove_file(&partial) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(storage(error)),
    }
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(&partial).map_err(storage)?;
    Ok(PreparedRecording::Live(WritePermit {
        write: Some(PreparedWrite {
            folder: folder.to_owned(),
            entry,
            partial,
            file: Some(file),
            replace_damaged,
            lock,
        }),
        _gate: None,
    }))
}

fn install_or_compare(
    partial: &Path,
    entry: &Path,
    exchange: &Exchange<'_>,
    written: &[u8],
    name: &str,
) -> Result<(), Error> {
    match fs::hard_link(partial, entry) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            let existing = fs::read(entry).map_err(storage)?;
            let recorded = Entry::replayed(&existing, exchange)
                .map_err(|error| Error::Entry(name.to_owned(), error.to_string()))?;
            let new = Entry::replayed(written, exchange)
                .map_err(|_| Error::Defect("a written recording entry cannot be read"))?;
            if recorded == new {
                Ok(())
            } else {
                Err(Error::RecordingConflict(name.to_owned()))
            }
        }
        Err(error) => Err(storage(error)),
    }
}

fn remove_partial(path: &Path) -> io::Result<()> {
    maybe_fail_io(StorageStageName::Cleanup)?;
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn make_folder(folder: &Path) -> Result<(), Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(folder)
            .map_err(storage)
    }
    #[cfg(not(unix))]
    fs::create_dir_all(folder).map_err(storage)
}

fn storage(_error: io::Error) -> Error {
    Error::RecordingStorage
}

/// Remove a completed lock file without syncing `.locks`.
///
/// The entry and its folder are already synced. A power loss can bring back
/// the empty lock file, which holds no answer and which prune removes. A sync
/// here cost one of the three per-entry syncs (`sdlc/records/qf-request-cost.md`).
fn remove_lock(lock: CacheLock) -> Result<(), Error> {
    maybe_fail(StorageStageName::LockRemove)?;
    lock.unlink().map_err(storage)
}

fn sync_recording_directory(folder: &Path) -> io::Result<()> {
    maybe_fail_io(StorageStageName::DirectorySync)?;
    cache_lock::sync_directory(folder)
}
