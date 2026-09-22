//! The folder of recorded exchanges, as files this process reads or writes.

use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::recording::{Digest, Entry, Exchange};

use crate::engine::cache_lock::{self, CacheLock};
use crate::engine::error::Error;

/// How many entries this process has begun to write.
///
/// Two records that are byte for byte alike make one digest, so every worker
/// that misses it writes that same entry at once. The count gives each attempt
/// a temporary name of its own, and the hard link that follows is the atomic step.
static WRITES: AtomicU64 = AtomicU64::new(0);

/// Which folders `--record` and `--replay` named, once they agree.
#[derive(Debug)]
pub(crate) struct Recorder {
    folder: Option<PathBuf>,
    recording: bool,
    replaying: bool,
}

impl Recorder {
    /// Take the folders the two options named.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Defect`] when the options name two different folders,
    /// because one command records into one folder.
    pub(crate) fn of(record: Option<&Path>, replay: Option<&Path>) -> Result<Self, Error> {
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
        })
    }

    pub(crate) const fn named(&self) -> bool {
        self.folder.is_some()
    }

    /// Answer this exchange from the folder, or say the folder cannot.
    ///
    /// A miss under `--replay` alone is a local failure naming the file that is
    /// absent. A miss when `--record` names the same folder is no failure, since
    /// the backend is asked and the answer is then written.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the entry is absent under replay alone, when the
    /// file is there and cannot be read, or when it records another exchange.
    pub(crate) fn replayed(
        &self,
        exchange: &Exchange<'_>,
        digest: &Digest,
    ) -> Result<Option<Vec<u8>>, Error> {
        let Some(folder) = self.folder.as_ref().filter(|_| self.replaying) else {
            return Ok(None);
        };
        let name = digest.file_name();
        let bytes = match fs::read(folder.join(&name)) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                if self.recording {
                    return Ok(None);
                }
                return Err(Error::ReplayMiss(name));
            }
            Err(error) => return Err(Error::Recording(error)),
        };
        Entry::replayed(&bytes, exchange)
            .map(Some)
            .map_err(|error| Error::Entry(name, error.to_string()))
    }

    /// Wait until this cache caller owns the exchange digest.
    ///
    /// Record-only and replay-only runs need no lock. A cache caller keeps the
    /// returned guard through its second read, request, decode, and recording.
    pub(crate) fn lock(&self, digest: &Digest) -> Result<Option<CacheLock>, Error> {
        let Some(folder) = self
            .folder
            .as_ref()
            .filter(|_| self.recording && self.replaying)
        else {
            return Ok(None);
        };
        cache_lock::acquire(folder, digest.as_str())
            .map(Some)
            .map_err(Error::Recording)
    }

    /// Write this exchange into the folder without replacing an existing response.
    ///
    /// The entry is closed under a temporary name in the same folder and then
    /// hard-linked without replacement, so a reader never sees half a file.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the folder or the file cannot be written.
    pub(crate) fn record(
        &self,
        exchange: &Exchange<'_>,
        digest: &Digest,
        response: &[u8],
    ) -> Result<(), Error> {
        let Some(folder) = self.folder.as_ref().filter(|_| self.recording) else {
            return Ok(());
        };
        let entry = Entry::of(exchange, response)
            .map_err(|_| Error::Defect("a recorded exchange is not JSON"))?;
        let written = entry
            .written()
            .map_err(|_| Error::Defect("a recorded exchange could not be written as JSON"))?;
        let name = digest.file_name();
        let attempt = WRITES.fetch_add(1, Ordering::Relaxed);
        let partial = folder.join(format!(".{}.{attempt}.{name}", process::id()));
        make_folder(folder).map_err(Error::Recording)?;
        // A crashed run may have left this name behind, and no other worker
        // holds it, because the count above gives each attempt its own.
        let _stale = fs::remove_file(&partial);
        write_private(&partial, &folder.join(&name), &written, exchange, &name)
    }
}

/// Make the folder readable by its owner alone, since a recording is private.
///
/// The tool targets Unix. On any other system the umask alone settles the mode.
fn make_folder(folder: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(folder)
    }
    #[cfg(not(unix))]
    fs::create_dir_all(folder)
}

/// Write the entry readable by its owner alone, and leave nothing behind.
///
/// A recording holds the evidence, so the file is created at mode `0600` and
/// its hard link carries that mode onto the entry. Every returned path attempts
/// to remove the temporary name. A process crash can leave that complete private
/// file behind.
fn write_private(
    partial: &Path,
    entry: &Path,
    written: &str,
    exchange: &Exchange<'_>,
    name: &str,
) -> Result<(), Error> {
    let attempt = || -> Result<(), Error> {
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let mut file = options.open(partial).map_err(Error::Recording)?;
        file.write_all(written.as_bytes())
            .map_err(Error::Recording)?;
        drop(file);
        match fs::hard_link(partial, entry) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let existing = fs::read(entry).map_err(Error::Recording)?;
                let recorded = Entry::replayed(&existing, exchange)
                    .map_err(|error| Error::Entry(name.to_owned(), error.to_string()))?;
                let new = Entry::replayed(written.as_bytes(), exchange)
                    .map_err(|_| Error::Defect("a written recording entry cannot be read"))?;
                // The v1 envelope stores a JSON value, not whitespace around it.
                // Compare that value without coupling it to envelope formatting.
                if recorded == new {
                    Ok(())
                } else {
                    Err(Error::RecordingConflict(name.to_owned()))
                }
            }
            Err(error) => Err(Error::Recording(error)),
        }
    };
    let result = attempt();
    let _left = fs::remove_file(partial);
    result
}
