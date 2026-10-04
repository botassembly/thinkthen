//! Windows leaf privacy. No process handler is installed by these helpers.

pub(crate) mod files;
mod security;

#[cfg(test)]
pub(crate) mod checkpoint;

pub(crate) fn permission() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "unsafe Windows object",
    )
}
