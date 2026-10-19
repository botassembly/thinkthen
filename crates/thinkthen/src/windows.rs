//! Windows leaf privacy for library-owned local state.

pub(crate) mod files;
mod security;

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
