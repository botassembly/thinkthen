//! Shared private access and file identity for library stores and command publication.
#[cfg(windows)]
#[allow(unsafe_code, reason = "the standard library has no access-list call")]
mod windows;
#[cfg(windows)]
pub(crate) use windows::AccessList;
#[cfg(unix)]
mod unix {
    //! Elsewhere the file mode does this job, so there is no list to carry.

    use std::fs::File;
    use std::io;
    use std::path::Path;

    #[derive(Debug)]
    pub(crate) struct AccessList;

    impl AccessList {
        pub(crate) fn private(file: &File, _path: &Path) -> io::Result<()> {
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(std::fs::Permissions::from_mode(
                if file.metadata()?.is_dir() {
                    0o700
                } else {
                    0o600
                },
            ))
        }
        pub(crate) fn same_file(left: &File, right: &File) -> io::Result<bool> {
            use std::os::unix::fs::MetadataExt;
            let (left, right) = (left.metadata()?, right.metadata()?);
            Ok(left.dev() == right.dev() && left.ino() == right.ino())
        }

        #[cfg(feature = "command")]
        pub(crate) fn read(_file: &File) -> io::Result<Option<Self>> {
            Ok(Some(Self))
        }

        #[cfg(feature = "command")]
        pub(crate) fn apply(&self, _path: &Path) -> io::Result<()> {
            Ok(())
        }
    }
}

use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
#[cfg(unix)]
pub(crate) use unix::AccessList;

pub(crate) fn directory(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
        };
        options.custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT);
    }
    options.open(path)
}
#[cfg(unix)]
pub(crate) fn validate_private(file: &File) -> io::Result<()> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file.metadata()?;
    if metadata.uid() != rustix::process::geteuid().as_raw()
        || metadata.mode() & 0o077 != 0
        || (!metadata.is_dir() && metadata.nlink() != 1)
    {
        return Err(io::Error::other(
            "response store requires private access owned by the current user",
        ));
    }
    Ok(())
}
#[cfg(windows)]
pub(crate) use windows::validate_private;
