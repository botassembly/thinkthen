//! Serialize cooperating in-place writers before opening the source snapshot.

use crate::files::{FileError, local};
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

pub(crate) struct SourceLock {
    path: PathBuf,
    _file: File,
    paths: crate::files::cleanup::TemporaryPaths,
}

impl SourceLock {
    pub(crate) fn acquire(source: &Path) -> Result<Self, FileError> {
        let source = fs::canonicalize(source).map_err(local)?;
        let name = source
            .file_name()
            .ok_or_else(|| local("input must name a file"))?
            .to_string_lossy();
        let path = source.with_file_name(format!(".{name}.thinkthen.lock"));
        let mut paths = crate::files::cleanup::TemporaryPaths::default();
        let file = paths.lock(&path, || OpenOptions::new().write(true).create_new(true).open(&path)).map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                FileError::Local(format!("another in-place operation holds {}; close that operation before retrying; remove this lock only after verifying its owner stopped", path.display()))
            } else { local(error) }
        })?;
        Ok(Self {
            path,
            _file: file,
            paths,
        })
    }
}

impl Drop for SourceLock {
    fn drop(&mut self) {
        // Keep publication errors intact; a stale lock safely refuses later writes.
        self.paths.remove_lock(&self.path);
    }
}
