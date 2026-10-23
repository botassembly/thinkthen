//! Bounded toolbox input.
use crate::core::{Error, ErrorKind, TOOLBOX_LIMIT, Toolbox};
use std::io::Read;

/// Read at most one byte beyond the toolbox limit and validate its contents.
///
/// # Errors
/// Returns `local.io` for an unreadable stream and `local.toolbox` for invalid contents.
pub fn read_toolbox(reader: impl Read) -> Result<Toolbox, Error> {
    let mut bytes = Vec::new();
    reader
        .take((TOOLBOX_LIMIT + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::new(ErrorKind::Io, "the toolbox could not be read"))?;
    Toolbox::from_bytes(&bytes)
}
