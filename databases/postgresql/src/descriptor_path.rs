//! The path the kernel holds for an open descriptor, so confinement judges
//! the file actually opened rather than a path that may have changed since
//! (review 4, item 7). Linux reads `/proc/self/fd`; macOS asks
//! `fcntl(F_GETPATH)`, because it has no `/proc` (review 5: confined reads
//! refused everything there). Any other platform answers nothing, and a
//! confined read then refuses. `open_beneath` opens a confined file
//! (review 6): Linux lets the kernel keep the walk inside the directory.

use std::fs::File;
use std::path::{Path, PathBuf};

/// Open one file for a read that never blocks and never follows its final
/// symlink.
pub(crate) fn open_plain(path: &Path) -> std::io::Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
        .open(path)
}

/// Open `rel` inside the directory `base`. `openat2` with `RESOLVE_BENEATH`
/// refuses any step that would leave `base`, a `..` or a symlink, before
/// it reaches the outside name. A kernel without `openat2`, or a sandbox
/// that forbids it, falls back to a plain open; the caller has already
/// checked the spelling and checks the opened descriptor after.
#[cfg(target_os = "linux")]
pub(crate) fn open_beneath(base: &Path, rel: &Path) -> std::io::Result<File> {
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::OpenOptionsExt;
    let dir = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_PATH | libc::O_DIRECTORY)
        .open(base)?;
    let name = std::ffi::CString::new(rel.as_os_str().as_bytes())?;
    // SAFETY: `open_how` is plain integers; zero is its documented default.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = (libc::O_RDONLY | libc::O_NONBLOCK | libc::O_NOFOLLOW | libc::O_CLOEXEC) as u64;
    how.resolve = libc::RESOLVE_BENEATH | libc::RESOLVE_NO_MAGICLINKS;
    // SAFETY: a live directory descriptor, a NUL-terminated name, and a
    // pointer and size that describe `how`.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            dir.as_raw_fd(),
            name.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if fd >= 0 {
        // SAFETY: the kernel just returned this descriptor to us alone.
        return Ok(unsafe { File::from_raw_fd(fd as i32) });
    }
    let error = std::io::Error::last_os_error();
    // Only a process where `openat2` itself is missing or forbidden falls
    // back. A security module that denies one file with EPERM keeps the
    // refusal instead of downgrading to the weaker open.
    if openat2_missing() {
        return open_plain(&base.join(rel));
    }
    Err(error)
}

/// Whether this process cannot call `openat2` at all: probed once by
/// opening the root directory as a path, which any working `openat2`
/// allows.
#[cfg(target_os = "linux")]
fn openat2_missing() -> bool {
    static MISSING: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *MISSING.get_or_init(|| {
        // SAFETY: as in `open_beneath`; zero is the default `open_how`.
        let mut how: libc::open_how = unsafe { std::mem::zeroed() };
        how.flags = (libc::O_PATH | libc::O_CLOEXEC) as u64;
        // SAFETY: a static NUL-terminated name and a pointer and size
        // that describe `how`; a returned descriptor is closed at once.
        let fd = unsafe {
            libc::syscall(
                libc::SYS_openat2,
                libc::AT_FDCWD,
                c"/".as_ptr(),
                &how as *const libc::open_how,
                std::mem::size_of::<libc::open_how>(),
            )
        };
        if fd >= 0 {
            // SAFETY: the probe's own descriptor.
            unsafe { libc::close(fd as i32) };
            return false;
        }
        matches!(
            std::io::Error::last_os_error().raw_os_error(),
            Some(libc::ENOSYS | libc::EPERM)
        )
    })
}

/// Open `rel` inside the directory `base`: a plain open, after the
/// caller's spelling check and before its descriptor check.
#[cfg(not(target_os = "linux"))]
pub(crate) fn open_beneath(base: &Path, rel: &Path) -> std::io::Result<File> {
    open_plain(&base.join(rel))
}

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
