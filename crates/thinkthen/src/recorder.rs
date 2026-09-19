//! The folder of recorded exchanges, as files this process reads or writes.

use std::fs;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process;

use thinkthen_core::recording::{Entry, Exchange};

use crate::failure::Failure;

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
    /// Returns [`Failure::TwoFolders`] when the options name two different
    /// folders, because one command records into one folder.
    pub(crate) fn of(record: Option<&Path>, replay: Option<&Path>) -> Result<Self, Failure> {
        let folder = match (record, replay) {
            (Some(recorded), Some(replayed)) if recorded != replayed => {
                return Err(Failure::TwoFolders);
            }
            (Some(named), _) | (_, Some(named)) => Some(named.to_owned()),
            (None, None) => None,
        };
        Ok(Self {
            folder,
            recording: record.is_some(),
            replaying: replay.is_some(),
        })
    }

    /// Answer this exchange from the folder, or say the folder cannot.
    ///
    /// A miss under `--replay` alone is a local failure naming the file that is
    /// absent. A miss when `--record` names the same folder is no failure, since
    /// the backend is asked and the answer is then written.
    ///
    /// # Errors
    ///
    /// Returns [`Failure`] when the entry is absent under replay alone, when the
    /// file is there and cannot be read, or when it records another exchange.
    pub(crate) fn replayed(&self, exchange: &Exchange<'_>) -> Result<Option<Vec<u8>>, Failure> {
        let Some(folder) = self.folder.as_ref().filter(|_| self.replaying) else {
            return Ok(None);
        };
        let name = exchange.digest().file_name();
        let Ok(bytes) = fs::read(folder.join(&name)) else {
            if self.recording {
                return Ok(None);
            }
            return Err(Failure::ReplayMiss(name));
        };
        Entry::replayed(&bytes, exchange)
            .map(Some)
            .map_err(|error| Failure::Entry(name, error.to_string()))
    }

    /// Write this exchange into the folder, replacing whatever it recorded before.
    ///
    /// The entry is written under a temporary name in the same folder and then
    /// renamed, so a reader never sees half a file.
    ///
    /// # Errors
    ///
    /// Returns [`Failure`] when the folder or the file cannot be written.
    pub(crate) fn record(&self, exchange: &Exchange<'_>, response: &[u8]) -> Result<(), Failure> {
        let Some(folder) = self.folder.as_ref().filter(|_| self.recording) else {
            return Ok(());
        };
        let entry = Entry::of(exchange, response)
            .map_err(|_| Failure::Defect("a recorded exchange is not JSON"))?;
        let written = entry
            .written()
            .map_err(|_| Failure::Defect("a recorded exchange could not be written as JSON"))?;
        let name = exchange.digest().file_name();
        let partial = folder.join(format!(".{}.{name}", process::id()));
        make_folder(folder).map_err(Failure::Recording)?;
        let _stale = fs::remove_file(&partial);
        write_private(&partial, &folder.join(&name), &written).map_err(Failure::Recording)
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
/// the rename carries that mode onto the entry. A write or a rename that fails
/// takes the temporary file with it, so no half-written private file is left in
/// a folder a user keeps.
fn write_private(partial: &Path, entry: &Path, written: &str) -> io::Result<()> {
    let attempt = || -> io::Result<()> {
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        options.open(partial)?.write_all(written.as_bytes())?;
        fs::rename(partial, entry)
    };
    attempt().inspect_err(|_| {
        let _left = fs::remove_file(partial);
    })
}
