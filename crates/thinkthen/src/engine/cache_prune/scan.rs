//! Final-entry and temporary-name inspection for explicit cache maintenance.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::core::recording::Entry;
use crate::engine::error::Error;

use super::storage;

pub(super) struct Scanned {
    pub(super) good: Vec<Found>,
    pub(super) bad: Vec<String>,
}

#[derive(Debug)]
pub(super) struct Found {
    pub(super) path: PathBuf,
    pub(super) name: String,
    pub(super) bytes: u64,
    pub(super) modified: SystemTime,
    pub(super) model: String,
    pub(super) requested: Option<String>,
    pub(super) remove: bool,
}

pub(super) struct Temporary {
    pub(super) path: PathBuf,
    pub(super) name: String,
    pub(super) bytes: u64,
}

pub(super) fn scan_temporary(folder: &Path) -> Result<Vec<Temporary>, Error> {
    let mut found = Vec::new();
    for item in fs::read_dir(folder).map_err(storage)? {
        let item = item.map_err(storage)?;
        let name = item.file_name().to_string_lossy().into_owned();
        if !temporary_name(&name) {
            continue;
        }
        let path = item.path();
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => continue,
            Err(error) => return Err(storage(error)),
        };
        if metadata.file_type().is_file() {
            found.push(Temporary {
                path,
                name,
                bytes: allocated(&metadata),
            });
        }
    }
    found.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(found)
}

fn temporary_name(name: &str) -> bool {
    let Some(body) = name
        .strip_prefix('.')
        .and_then(|text| text.strip_suffix(".json"))
    else {
        return false;
    };
    let mut parts = body.split('.');
    let (Some(pid), Some(attempt), Some(digest), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    !pid.is_empty()
        && pid.bytes().all(|byte| byte.is_ascii_digit())
        && !attempt.is_empty()
        && attempt.bytes().all(|byte| byte.is_ascii_digit())
        && digest.len() == 64
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(super) fn scan_final(folder: &Path) -> Result<Scanned, Error> {
    let mut found = Scanned {
        good: Vec::new(),
        bad: Vec::new(),
    };
    for item in fs::read_dir(folder).map_err(storage)? {
        let item = item.map_err(storage)?;
        let name = item.file_name().to_string_lossy().into_owned();
        if !digest_name(&name) {
            continue;
        }
        match read_entry(&item.path(), &name) {
            Ok(Some(entry)) => found.good.push(entry),
            Ok(None) => {}
            Err(()) => found.bad.push(name),
        }
    }
    found.bad.sort();
    Ok(found)
}

fn read_entry(path: &Path, name: &str) -> Result<Option<Found>, ()> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(()),
    };
    if !metadata.file_type().is_file() {
        return Err(());
    }
    let file = match open_entry(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(()),
    };
    let opened = file.metadata().map_err(|_| ())?;
    if !same_identity(&metadata, &opened) {
        return Err(());
    }
    let bytes = {
        use std::io::Read as _;
        let mut file = file;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(|_| ())?;
        bytes
    };
    let final_metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(()),
    };
    if final_metadata.file_type().is_symlink() || !same_identity(&opened, &final_metadata) {
        return Err(());
    }
    let (digest, model, requested) = Entry::inspected(&bytes).map_err(|_| ())?;
    if digest.file_name() != name {
        return Err(());
    }
    Ok(Some(Found {
        path: path.to_path_buf(),
        name: name.to_owned(),
        bytes: allocated(&metadata),
        modified: metadata.modified().map_err(|_| ())?,
        model,
        requested,
        remove: false,
    }))
}

fn open_entry(path: &Path) -> io::Result<fs::File> {
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        // The path can change after symlink_metadata. Reject a new symlink at
        // open, and never wait for a replacement FIFO before checking identity.
        options.custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt as _;
        // Open a reparse point itself, so a replaced link cannot redirect the read.
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    options.open(path)
}

#[cfg(unix)]
pub(super) fn same_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt as _;
    (left.dev(), left.ino()) == (right.dev(), right.ino())
}

#[cfg(not(unix))]
pub(super) fn same_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    left.len() == right.len() && left.file_type() == right.file_type()
}

fn digest_name(name: &str) -> bool {
    name.len() == 69
        && name.ends_with(".json")
        && name.as_bytes().get(..64).is_some_and(|bytes| {
            bytes
                .iter()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
        })
}

#[cfg(unix)]
pub(super) fn allocated(metadata: &fs::Metadata) -> u64 {
    use std::os::unix::fs::MetadataExt as _;
    metadata.blocks().saturating_mul(512)
}

#[cfg(not(unix))]
pub(super) fn allocated(metadata: &fs::Metadata) -> u64 {
    metadata.len()
}
