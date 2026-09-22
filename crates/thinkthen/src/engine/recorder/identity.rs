//! Safe atomic storage for one recording folder's backend identity.

use std::fs::{self, File};
use std::io::{self, Read as _, Write as _};
use std::path::Path;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::recording_identity::BackendIdentity;
use crate::engine::cache_lock;
use crate::engine::error::Error;
use crate::engine::recorder::fault::{StorageStageName, maybe_fail_io};

const NAME: &str = ".thinkthen-backend.json";
const LIMIT: u64 = 257;
const CREATE_ATTEMPTS: u64 = 16;
static WRITES: AtomicU64 = AtomicU64::new(0);

#[cfg(unix)]
pub(super) fn require_private(folder: &Path) -> Result<(), Error> {
    use std::os::unix::fs::PermissionsExt as _;
    let mode = fs::metadata(folder).map_err(storage)?.permissions().mode() & 0o777;
    if mode == 0o700 {
        Ok(())
    } else {
        Err(Error::DefaultCachePrivate)
    }
}

#[cfg(not(unix))]
pub(super) fn require_private(_folder: &Path) -> Result<(), Error> {
    Ok(())
}

pub(super) fn check(folder: &Path, expected: &BackendIdentity, writing: bool) -> Result<(), Error> {
    let marker = folder.join(NAME);
    match read(&marker)? {
        Some(found) => match_identity(folder, &found, expected, writing),
        None if !writing => Ok(()),
        None if has_entry(folder)? => Err(Error::RecordingFolderLegacy),
        None => publish(folder, &marker, expected),
    }
}

fn match_identity(
    folder: &Path,
    found: &BackendIdentity,
    expected: &BackendIdentity,
    writing: bool,
) -> Result<(), Error> {
    if found != expected {
        return Err(Error::RecordingBackendMismatch);
    }
    if writing {
        sync_directory(folder)
    } else {
        Ok(())
    }
}

fn sync_directory(folder: &Path) -> Result<(), Error> {
    maybe_fail_io(StorageStageName::IdentityDirectorySync).map_err(storage)?;
    cache_lock::sync_directory(folder).map_err(storage)
}

fn read(path: &Path) -> Result<Option<BackendIdentity>, Error> {
    read_after_open(path, || {})
}

fn read_after_open(
    path: &Path,
    after_open: impl FnOnce(),
) -> Result<Option<BackendIdentity>, Error> {
    let before = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(storage(error)),
    };
    if before.file_type().is_symlink() || !before.is_file() {
        return Err(Error::RecordingStorage);
    }
    let file = File::open(path).map_err(storage)?;
    let opened = file.metadata().map_err(storage)?;
    if !opened.is_file() || !same_identity(&before, &opened) {
        return Err(Error::RecordingStorage);
    }
    after_open();
    let mut bytes = Vec::with_capacity(LIMIT as usize);
    file.take(LIMIT).read_to_end(&mut bytes).map_err(storage)?;
    if bytes.len() == LIMIT as usize {
        return Err(Error::RecordingStorage);
    }
    let after = fs::symlink_metadata(path).map_err(storage)?;
    if after.file_type().is_symlink()
        || !after.is_file()
        || !same_identity(&before, &after)
        || !same_identity(&opened, &after)
    {
        return Err(Error::RecordingStorage);
    }
    BackendIdentity::read(&bytes)
        .map(Some)
        .map_err(|()| Error::RecordingStorage)
}

fn publish(folder: &Path, marker: &Path, expected: &BackendIdentity) -> Result<(), Error> {
    let first = WRITES.fetch_add(CREATE_ATTEMPTS, Ordering::Relaxed);
    publish_at(folder, marker, expected, first)
}

fn publish_at(
    folder: &Path,
    marker: &Path,
    expected: &BackendIdentity,
    first: u64,
) -> Result<(), Error> {
    let (file, partial) = create_partial(folder, first)?;
    let result = publish_from(folder, marker, &partial, file, expected);
    let cleanup = remove_owned(&partial);
    match (result, cleanup) {
        (_, Err(error)) => Err(storage(error)),
        (result, Ok(())) => result,
    }
}

fn publish_from(
    folder: &Path,
    marker: &Path,
    partial: &Path,
    mut file: File,
    expected: &BackendIdentity,
) -> Result<(), Error> {
    let bytes = expected
        .written()
        .map_err(|()| Error::Defect("a backend identity could not be written"))?;
    maybe_fail_io(StorageStageName::IdentityWrite).map_err(storage)?;
    file.write_all(&bytes).map_err(storage)?;
    maybe_fail_io(StorageStageName::IdentityFileSync).map_err(storage)?;
    file.sync_all().map_err(storage)?;
    drop(file);
    pause("before-install");
    maybe_fail_io(StorageStageName::IdentityInstall).map_err(storage)?;
    match fs::hard_link(partial, marker) {
        Ok(()) => {
            pause("after-install");
            sync_directory(folder)
        }
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            let found = read(marker)?.ok_or(Error::RecordingStorage)?;
            match_identity(folder, &found, expected, true)
        }
        Err(error) => Err(storage(error)),
    }
}

fn create_partial(folder: &Path, first: u64) -> Result<(File, std::path::PathBuf), Error> {
    for attempt in first..first + CREATE_ATTEMPTS {
        let partial = folder.join(format!(
            ".thinkthen-backend.{}.{attempt}.tmp",
            process::id()
        ));
        maybe_fail_io(StorageStageName::IdentityCreate).map_err(storage)?;
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        match options.open(&partial) {
            Ok(file) => return Ok((file, partial)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(storage(error)),
        }
    }
    Err(Error::RecordingStorage)
}

fn has_entry(folder: &Path) -> Result<bool, Error> {
    for entry in fs::read_dir(folder).map_err(storage)? {
        let name = entry.map_err(storage)?.file_name();
        if digest_name(&name.to_string_lossy()) {
            return Ok(true);
        }
    }
    Ok(false)
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

fn remove_owned(path: &Path) -> io::Result<()> {
    maybe_fail_io(StorageStageName::IdentityCleanup)?;
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
fn pause(stage: &str) {
    if std::env::var("THINKTHEN_TEST_IDENTITY_PAUSE").as_deref() != Ok(stage) {
        return;
    }
    let ready = std::env::var_os("THINKTHEN_TEST_IDENTITY_READY").expect("test ready path");
    fs::write(ready, []).expect("test pause signal");
    loop {
        std::thread::park();
    }
}

#[cfg(not(test))]
const fn pause(_stage: &str) {}

#[cfg(unix)]
fn same_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt as _;
    (left.dev(), left.ino()) == (right.dev(), right.ino())
}

#[cfg(not(unix))]
fn same_identity(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    left.len() == right.len() && left.file_type() == right.file_type()
}

fn storage(_error: io::Error) -> Error {
    Error::RecordingStorage
}

#[cfg(test)]
mod tests;
