//! The folder of recorded exchanges, as files this process reads or writes.

use std::fs;
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
    /// file cannot be read, or when it records another exchange.
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
        fs::create_dir_all(folder).map_err(Failure::Recording)?;
        fs::write(&partial, written).map_err(Failure::Recording)?;
        fs::rename(&partial, folder.join(&name)).map_err(Failure::Recording)
    }
}
