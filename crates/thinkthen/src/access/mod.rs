//! Private access and file identity for command publication.
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

#[cfg(unix)]
pub(crate) use unix::AccessList;
