//! File admission shares cancellation checks across copies and decoders.

use crate::{FileError, local};
use std::io::{Read, Write};
use thinkthen::{Cancel, Error, ErrorKind};

pub(crate) fn check_cancel(cancel: &Cancel) -> Result<(), FileError> {
    if cancel.is_cancelled() {
        Err(Error::new(ErrorKind::Cancelled, "the call was cancelled").into())
    } else {
        Ok(())
    }
}

pub(crate) fn copy(
    reader: &mut impl Read,
    writer: &mut impl Write,
    cancel: &Cancel,
) -> Result<u64, FileError> {
    let mut bytes = [0; 65_536];
    let mut copied = 0;
    loop {
        check_cancel(cancel)?;
        let count = reader.read(&mut bytes).map_err(local)?;
        check_cancel(cancel)?;
        if count == 0 {
            return Ok(copied);
        }
        writer
            .write_all(
                bytes
                    .get(..count)
                    .ok_or_else(|| local("invalid read size"))?,
            )
            .map_err(local)?;
        copied += count as u64;
    }
}
