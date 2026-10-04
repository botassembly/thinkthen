//! Safe handle-owning operations for usage and configuration leaves.

use std::fs::{self, File};
use std::io::{self, Read as _};
use std::path::Path;

mod ffi;

pub(crate) fn open_read(path: &Path) -> io::Result<File> {
    ffi::open(path, false, false, false).map(|(file, _)| file)
}

pub(crate) fn usage_read(path: &Path, directory: bool) -> io::Result<File> {
    let file = open_read(path)?;
    ffi::kind(&file, directory)?;
    super::security::usage(&file)?;
    verify_identity(path, &file, directory)?;
    Ok(file)
}

pub(crate) fn usage_write(path: &Path, create: bool, exclusive: bool) -> io::Result<(File, bool)> {
    let (file, created) = ffi::open(path, true, create, exclusive)?;
    ffi::kind(&file, false)?;
    super::security::usage(&file)?;
    verify_identity(path, &file, false)?;
    Ok((file, created))
}

pub(crate) fn verify_identity(path: &Path, file: &File, directory: bool) -> io::Result<()> {
    ffi::kind(file, directory)?;
    let named = open_read(path)?;
    ffi::kind(&named, directory)?;
    if ffi::identity(file)? != ffi::identity(&named)? {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "usage identity changed",
        ));
    }
    Ok(())
}

pub(crate) fn make_private_directory(path: &Path) -> io::Result<()> {
    make_missing(path)?;
    usage_read(path, true).map(|_file| ())
}

/// Only missing ancestors receive the descriptor; existing platform folders
/// retain their current policy. The final usage leaf is checked independently.
fn make_missing(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(_) => return Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    if let Some(parent) = path.parent() {
        make_missing(parent)?;
    }
    ffi::create_directory(path)
}

pub(crate) fn configuration(path: &Path) -> io::Result<(Vec<u8>, bool)> {
    let mut file = open_read(path)?;
    ffi::kind(&file, false)?;
    let shared = super::security::configuration(&file)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    verify_identity(path, &file, false)?;
    Ok((bytes, shared))
}

#[cfg(test)]
mod tests;
