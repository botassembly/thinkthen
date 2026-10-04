//! Distinct privacy policies for aggregate usage and read-only configuration.

use std::fs::File;
use std::io;

pub(super) mod ffi;

pub(super) fn usage(file: &File) -> io::Result<()> {
    let inspected = ffi::inspect(file)?;
    if !inspected.owned || inspected.unrestricted || inspected.foreign_access {
        return Err(super::permission());
    }
    Ok(())
}

pub(super) fn configuration(file: &File) -> io::Result<bool> {
    let inspected = ffi::inspect(file)?;
    Ok(!inspected.owned || inspected.unrestricted || inspected.foreign_write)
}
