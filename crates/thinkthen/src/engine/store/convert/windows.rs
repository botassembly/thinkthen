//! Deny Windows writers while the converter snapshots and removes the live file.
use std::fs::{self, File, OpenOptions};
use std::os::windows::fs::OpenOptionsExt as _;
use std::path::Path;

use windows_sys::Win32::Storage::FileSystem::{FILE_SHARE_DELETE, FILE_SHARE_READ};

use crate::engine::error::Error;

pub(super) fn guard(sqlite: &Path) -> Result<File, Error> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_DELETE)
        .open(sqlite)
        .map_err(|_| Error::RecordingStorage)
}

/// Undo publication while the guard still denies writers. Preserve exact old bytes.
pub(super) fn restore(folder: &Path, jsonl: &Path, previous: Option<&[u8]>) -> Result<(), Error> {
    let restored = match previous {
        Some(bytes) => super::replace(folder, jsonl, bytes),
        None => fs::remove_file(jsonl)
            .and_then(|()| super::sync_folder(folder))
            .map_err(|_| Error::RecordingStorage),
    };
    restored.map_err(|_| {
        Error::Entry(
            super::JSONL.to_owned(),
            "conversion failed and the original fixture could not be restored; retain the store files for recovery"
                .to_owned(),
        )
    })
}
