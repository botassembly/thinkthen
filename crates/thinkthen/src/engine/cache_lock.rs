//! One operating-system lock for one missing or damaged recording entry.

use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

/// An exclusive digest lock, released when its file closes.
#[derive(Debug)]
pub(crate) struct CacheLock {
    _file: File,
    path: PathBuf,
    folder: PathBuf,
}

/// A shared or exclusive lock on an already-open recording directory.
#[derive(Debug)]
pub(crate) struct FolderGate {
    _directory: File,
}

pub(crate) enum TryAcquire {
    Acquired(CacheLock),
    Active,
}

pub(crate) fn shared_folder(folder: &Path) -> io::Result<FolderGate> {
    let directory = File::open(folder)?;
    File::lock_shared(&directory)?;
    Ok(FolderGate {
        _directory: directory,
    })
}

pub(crate) fn exclusive_folder(folder: &Path) -> io::Result<FolderGate> {
    let directory = File::open(folder)?;
    File::lock(&directory)?;
    Ok(FolderGate {
        _directory: directory,
    })
}

/// Wait for exclusive ownership of one digest in this recording folder.
pub(crate) fn acquire(folder: &Path, digest: &str) -> io::Result<CacheLock> {
    acquire_after_open(folder, digest, || {})
}

/// Take an inactive digest lock without waiting for an older process.
pub(crate) fn try_acquire(folder: &Path, digest: &str) -> io::Result<TryAcquire> {
    use std::fs::TryLockError;

    let (file, path, locks) = opened_lock(folder, digest)?;
    match file.try_lock() {
        Ok(()) => Ok(TryAcquire::Acquired(CacheLock {
            _file: file,
            path,
            folder: locks,
        })),
        Err(TryLockError::WouldBlock) => Ok(TryAcquire::Active),
        Err(TryLockError::Error(error)) => Err(error),
    }
}

fn acquire_after_open(folder: &Path, digest: &str, opened: impl FnOnce()) -> io::Result<CacheLock> {
    let (file, path, locks) = opened_lock(folder, digest)?;
    opened();
    file.lock()?;
    Ok(CacheLock {
        _file: file,
        path,
        folder: locks,
    })
}

fn opened_lock(folder: &Path, digest: &str) -> io::Result<(File, PathBuf, PathBuf)> {
    let locks = folder.join(".locks");
    make_private(&locks)?;
    let path = locks.join(digest);
    let mut options = File::options();
    options.read(true).write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(&path)?;
    Ok((file, path, locks))
}

impl CacheLock {
    /// Remove a completed lock while this owner still holds its inode.
    pub(crate) fn unlink(&self) -> io::Result<()> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub(crate) fn sync_folder(&self) -> io::Result<()> {
        sync_directory(&self.folder)
    }

    #[cfg(test)]
    pub(crate) fn file(&self) -> &File {
        &self._file
    }
}

/// Sync one directory after changing the names it contains.
pub(crate) fn sync_directory(folder: &Path) -> io::Result<()> {
    File::open(folder)?.sync_all()
}

fn make_private(folder: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::{DirBuilderExt as _, PermissionsExt as _};
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(folder)?;
        fs::set_permissions(folder, fs::Permissions::from_mode(0o700))
    }
    #[cfg(not(unix))]
    fs::create_dir_all(folder)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use super::{acquire, acquire_after_open};

    #[test]
    fn a_waiter_keeps_the_original_inode_after_the_owner_unlinks_it() -> io::Result<()> {
        let folder = std::env::temp_dir().join(format!("thinkthen-lock-{}", std::process::id()));
        let _absent = fs::remove_dir_all(&folder);
        let owner = acquire(&folder, "digest")?;
        let owner_inode = inode(owner.file())?;
        let opened = fs::File::open(folder.join(".locks/digest"))?;
        assert_eq!(inode(&opened)?, owner_inode);
        let (attempted_send, attempted) = mpsc::channel();
        let (owned_send, owned) = mpsc::channel();

        let waiter_folder = folder.clone();
        let waiter = thread::spawn(move || -> io::Result<()> {
            let guard = acquire_after_open(&waiter_folder, "digest", || {
                let _sent = attempted_send.send(());
            })?;
            owned_send
                .send(inode(guard.file())?)
                .map_err(io::Error::other)
        });
        attempted
            .recv_timeout(Duration::from_secs(2))
            .map_err(io::Error::other)?;
        owner.unlink()?;
        owner.sync_folder()?;
        drop(owner);
        let waiter_inode = owned
            .recv_timeout(Duration::from_secs(2))
            .map_err(io::Error::other)?;
        waiter
            .join()
            .map_err(|_| io::Error::other("waiter panicked"))??;

        assert_eq!(waiter_inode, owner_inode);
        assert!(!folder.join(".locks/digest").exists());
        fs::remove_dir_all(folder)?;
        Ok(())
    }

    #[cfg(unix)]
    fn inode(file: &fs::File) -> io::Result<u64> {
        use std::os::unix::fs::MetadataExt as _;
        Ok(file.metadata()?.ino())
    }

    #[cfg(not(unix))]
    fn inode(_file: &fs::File) -> io::Result<u64> {
        Ok(1)
    }
}
