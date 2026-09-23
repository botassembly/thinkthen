//! The path the kernel holds for an open descriptor, so confinement judges
//! the file actually opened rather than a path that may have changed since
//! (review 4, item 7). Linux reads `/proc/self/fd`; macOS asks
//! `fcntl(F_GETPATH)`, because it has no `/proc` (review 5: confined reads
//! refused everything there). Any other platform answers nothing, and a
//! confined read then refuses.

use std::fs::File;
use std::path::PathBuf;

/// The descriptor's path, or `None` when the platform cannot say.
#[cfg(target_os = "linux")]
pub(crate) fn of(file: &File) -> Option<PathBuf> {
    use std::os::fd::AsRawFd;
    std::fs::read_link(format!("/proc/self/fd/{}", file.as_raw_fd())).ok()
}

/// The descriptor's path, or `None` when the platform cannot say.
#[cfg(target_os = "macos")]
pub(crate) fn of(file: &File) -> Option<PathBuf> {
    use std::os::fd::AsRawFd;
    use std::os::unix::ffi::OsStrExt;
    let mut name = [0u8; libc::PATH_MAX as usize];
    // SAFETY: F_GETPATH writes at most PATH_MAX bytes, NUL included.
    let got = unsafe { libc::fcntl(file.as_raw_fd(), libc::F_GETPATH, name.as_mut_ptr()) };
    if got == -1 {
        return None;
    }
    let end = name.iter().position(|byte| *byte == 0)?;
    Some(PathBuf::from(std::ffi::OsStr::from_bytes(name.get(..end)?)))
}

/// The descriptor's path, or `None` when the platform cannot say.
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub(crate) fn of(_file: &File) -> Option<PathBuf> {
    None
}
