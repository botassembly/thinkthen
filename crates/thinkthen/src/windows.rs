//! Windows leaf privacy and command-only console disposition.

pub(crate) mod files;
mod security;

#[cfg(feature = "cli")]
#[path = "windows/console/ffi.rs"]
mod console;
#[cfg(feature = "cli")]
pub(crate) use console::enable_interrupts;

#[cfg(test)]
#[allow(
    clippy::expect_used,
    clippy::panic,
    reason = "native writer fixtures stop the proof on a broken test precondition"
)]
pub(crate) mod checkpoint;

#[cfg(test)]
pub(crate) mod test_support;

pub(crate) fn permission() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "unsafe Windows object",
    )
}
