//! The check a writing store passes before its call's first send.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{SQLITE, exists, make_folder, require_private};
use crate::engine::error::Error;

/// A writing store's folder, checked by a send worker before its call's
/// first send, so a folder it cannot write refuses before anything is paid
/// for (ticket 0367).
#[derive(Clone, Debug)]
pub(crate) struct Probe {
    folder: PathBuf,
    private: bool,
}

impl Probe {
    pub(super) const fn new(folder: PathBuf, private: bool) -> Self {
        Self { folder, private }
    }

    /// Make the folder as a write would, open an existing file for writing,
    /// and create and remove one file of its own in the folder, where SQLite
    /// puts its journal. The store file itself is neither made nor changed.
    pub(crate) fn check(&self) -> Result<(), Error> {
        static PROBES: AtomicU64 = AtomicU64::new(0);
        if self.private && self.folder.exists() {
            require_private(&self.folder)?;
        }
        make_folder(&self.folder)?;
        let sqlite = self.folder.join(SQLITE);
        if exists(&sqlite)? {
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&sqlite)
                .map_err(|_| Error::RecordingStorage)?;
        }
        let probe = self.folder.join(format!(
            ".thinkthen-probe-{}-{}",
            std::process::id(),
            PROBES.fetch_add(1, Ordering::Relaxed)
        ));
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&probe)
            .map_err(|_| Error::RecordingStorage)?;
        fs::remove_file(&probe).map_err(|_| Error::RecordingStorage)
    }
}
