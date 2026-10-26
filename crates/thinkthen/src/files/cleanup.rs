//! Register operation-owned paths before they become visible to forced cleanup.

use crate::{Cancel, Error, ErrorKind};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use tempfile::NamedTempFile;

#[derive(Default)]
struct Paths {
    stopped: bool,
    paths: BTreeSet<PathBuf>,
}
static PATHS: Mutex<Paths> = Mutex::new(Paths {
    stopped: false,
    paths: BTreeSet::new(),
});

fn paths() -> MutexGuard<'static, Paths> {
    PATHS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Remove this process's file-operation scratch and locks before forced exit.
///
/// This is terminal: subsequent file resource creation fails. Completed outputs
/// and completed backups remain untouched. Provisional backups are removed.
///
/// # Errors
/// Returns the first removal error after attempting every owned path.
pub fn force_cleanup() -> Result<(), Error> {
    let mut paths = paths();
    paths.stopped = true;
    let mut failure = None;
    for path in std::mem::take(&mut paths.paths) {
        if let Err(error) = fs::remove_file(&path)
            && error.kind() != io::ErrorKind::NotFound
        {
            failure.get_or_insert(error);
        }
    }
    failure.map_or(Ok(()), |error| {
        Err(Error::new(
            ErrorKind::Io,
            format!("temporary files could not be removed: {error}"),
        ))
    })
}

#[derive(Default)]
pub(crate) struct TemporaryPaths(Vec<PathBuf>);

impl TemporaryPaths {
    pub(crate) fn remove_lock(&mut self, path: &Path) {
        let mut registry = paths();
        // Keep unregistering and unlinking together: a new cooperating writer
        // may acquire this name as soon as the old lock disappears.
        if registry.paths.remove(path) {
            let _ = fs::remove_file(path);
        }
        self.0.retain(|owned| owned != path);
    }

    pub(crate) fn temporary(&mut self, parent: Option<&Path>) -> io::Result<NamedTempFile> {
        self.temporary_with_suffix(parent, "")
    }

    pub(crate) fn temporary_with_suffix(
        &mut self,
        parent: Option<&Path>,
        suffix: &str,
    ) -> io::Result<NamedTempFile> {
        let mut registry = paths();
        active(&registry)?;
        let mut builder = tempfile::Builder::new();
        builder.suffix(suffix);
        let file = match parent {
            Some(parent) => builder.tempfile_in(parent)?,
            None => builder.tempfile()?,
        };
        registry.paths.insert(file.path().to_owned());
        self.0.push(file.path().to_owned());
        Ok(file)
    }

    pub(crate) fn output(&mut self, parent: &Path) -> io::Result<NamedTempFile> {
        let mut registry = paths();
        active(&registry)?;
        let builder = tempfile::Builder::new();
        #[cfg(unix)]
        let builder = {
            use std::os::unix::fs::PermissionsExt;
            // The builder applies the caller's umask at creation without changing it.
            let mut builder = builder;
            builder.permissions(fs::Permissions::from_mode(0o666));
            builder
        };
        let file = builder.tempfile_in(parent)?;
        registry.paths.insert(file.path().to_owned());
        self.0.push(file.path().to_owned());
        Ok(file)
    }

    fn backup(&mut self, file: NamedTempFile, path: &Path) -> io::Result<File> {
        let mut registry = paths();
        active(&registry)?;
        let file = file.persist_noclobber(path).map_err(|error| error.error)?;
        registry.paths.insert(path.to_owned());
        self.0.push(path.to_owned());
        Ok(file)
    }

    pub(crate) fn lock(
        &mut self,
        path: &Path,
        create: impl FnOnce() -> io::Result<File>,
    ) -> io::Result<File> {
        let mut registry = paths();
        active(&registry)?;
        let file = create()?;
        registry.paths.insert(path.to_owned());
        self.0.push(path.to_owned());
        Ok(file)
    }
}

impl Drop for TemporaryPaths {
    fn drop(&mut self) {
        let mut registry = paths();
        for path in &self.0 {
            registry.paths.remove(path);
        }
    }
}

fn active(paths: &Paths) -> io::Result<()> {
    if paths.stopped {
        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "file operation interrupted",
        ))
    } else {
        Ok(())
    }
}

// A backup remains provisional through the last source and cancellation checks.
// Ordinary failures retain it; cancellation also removes it from forced cleanup.
pub(crate) struct Backup {
    pub(crate) path: PathBuf,
    paths: TemporaryPaths,
    cancel: Cancel,
    provisional: bool,
}

impl Backup {
    pub(crate) fn publish(file: NamedTempFile, path: PathBuf, cancel: &Cancel) -> io::Result<Self> {
        let mut paths = TemporaryPaths::default();
        let backup = paths.backup(file, &path)?;
        let guard = Self {
            path,
            paths,
            cancel: cancel.clone(),
            provisional: true,
        };
        backup.sync_all()?;
        Ok(guard)
    }

    pub(crate) fn replace(
        &mut self,
        file: NamedTempFile,
        output: &Path,
    ) -> Result<File, crate::files::FileError> {
        let mut registry = paths();
        active(&registry).map_err(crate::files::local)?;
        crate::files::admission::check_cancel(&self.cancel)?;
        let file = file.persist(output).map_err(crate::files::local)?;
        // Atomic replacement commits this publication. Forced cleanup must not
        // remove the retained original after that point.
        registry.paths.remove(&self.path);
        self.provisional = false;
        Ok(file)
    }
}

impl Drop for Backup {
    fn drop(&mut self) {
        if self.provisional && self.cancel.is_cancelled() {
            self.paths.remove_lock(&self.path);
        }
    }
}
