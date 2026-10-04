//! Platform leaf creation, validation and identity checks.

#[cfg(not(windows))]
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io;
use std::path::Path;

#[cfg(not(windows))]
pub(super) fn make_private_directory(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(path)?;
    }
    #[cfg(not(unix))]
    fs::create_dir_all(path)?;
    validate_directory(&fs::symlink_metadata(path)?)
}

#[cfg(not(windows))]
pub(super) fn open_private(path: &Path, create: bool) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(create);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(path)?;
    verify_private_file(path, &file)?;
    Ok(file)
}

#[cfg(not(windows))]
pub(super) fn open_stable_lock(path: &Path) -> io::Result<(File, bool)> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    match options.open(path) {
        Ok(file) => {
            verify_private_file(path, &file)?;
            Ok((file, true))
        }
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            open_private(path, false).map(|file| (file, false))
        }
        Err(error) => Err(error),
    }
}

#[cfg(not(windows))]
pub(super) fn verify_private_file(path: &Path, file: &File) -> io::Result<()> {
    verify_identity(path, file, false)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if file.metadata()?.permissions().mode() & 0o777 != 0o600 {
            return Err(permission());
        }
    }
    Ok(())
}

#[cfg(not(windows))]
pub(super) fn open_verified(path: &Path, directory: bool, mode: u32) -> io::Result<File> {
    let file = open_read(path)?;
    verify_identity(path, &file, directory)?;
    verify_mode(&file, mode)?;
    Ok(file)
}

/// Open a file or a folder for reading.
#[cfg(not(windows))]
pub(crate) fn open_read(path: &Path) -> io::Result<File> {
    File::open(path)
}

/// The general command input reader retains its existing Windows behavior.
#[cfg(windows)]
pub(crate) fn open_read(path: &Path) -> io::Result<File> {
    use std::os::windows::fs::OpenOptionsExt as _;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(0x0200_0000)
        .open(path)
}

#[cfg(windows)]
pub(super) use crate::windows::files::make_private_directory;

#[cfg(windows)]
pub(super) fn open_private(path: &Path, create: bool) -> io::Result<File> {
    crate::windows::files::usage_write(path, create, false).map(|(file, _)| file)
}

#[cfg(windows)]
pub(super) fn open_stable_lock(path: &Path) -> io::Result<(File, bool)> {
    match crate::windows::files::usage_write(path, true, true) {
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            open_private(path, false).map(|file| (file, false))
        }
        result => result,
    }
}

#[cfg(windows)]
pub(super) fn open_verified(path: &Path, directory: bool, _mode: u32) -> io::Result<File> {
    crate::windows::files::usage_read(path, directory)
}

#[cfg(unix)]
pub(super) fn verify_mode(file: &File, mode: u32) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt as _;
    if file.metadata()?.permissions().mode() & 0o777 != mode {
        return Err(permission());
    }
    Ok(())
}

/// Windows has no mode bits; stage 1 owns its privacy check (ticket 0373).
#[cfg(not(any(unix, windows)))]
pub(super) const fn verify_mode(_file: &File, _mode: u32) -> io::Result<()> {
    Ok(())
}

/// Syncs a folder after a rename. Windows cannot flush a folder handle, so
/// it skips the sync, as `engine/store/convert.rs` does (ticket 0373).
pub(super) fn sync_directory(directory: &File) -> io::Result<()> {
    #[cfg(unix)]
    {
        directory.sync_all()
    }
    #[cfg(not(unix))]
    {
        let _unsynced = directory;
        Ok(())
    }
}

#[cfg(not(windows))]
pub(super) fn verify_identity(path: &Path, file: &File, directory: bool) -> io::Result<()> {
    let named = fs::symlink_metadata(path)?;
    if named.file_type().is_symlink() || named.is_dir() != directory || named.is_file() == directory
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsafe usage object",
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        let opened = file.metadata()?;
        if (named.dev(), named.ino()) != (opened.dev(), opened.ino()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "usage identity changed",
            ));
        }
    }
    #[cfg(not(unix))]
    let _unchecked = file;
    Ok(())
}

#[cfg(windows)]
pub(super) use crate::windows::files::verify_identity;

pub(super) fn validate_directory(metadata: &fs::Metadata) -> io::Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        if metadata.file_attributes()
            & windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT
            != 0
        {
            return Err(crate::windows::permission());
        }
    }
    if !metadata.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "usage path"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o777 != 0o700 {
            return Err(permission());
        }
    }
    Ok(())
}

#[cfg(unix)]
pub(super) fn permission() -> io::Error {
    io::Error::new(io::ErrorKind::PermissionDenied, "unsafe usage mode")
}
