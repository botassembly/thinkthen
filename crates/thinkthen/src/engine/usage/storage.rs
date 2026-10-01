//! One private file of five checked counts for each month (ticket 0360).

use std::fs::{self, File};
use std::io::{self, Read as _, Seek as _, SeekFrom, Write as _};
use std::path::Path;

use super::{
    Counts, Shared, Stage, make_private_directory, maybe_fail, note_initial_sync, open_private,
    open_stable_lock, open_verified, overflow, pause_after_creation, recognized_month,
    sync_directory, validate_directory, verify_identity,
};

const DIRECTORY: &str = "usage directory";

pub(crate) struct ReadFailure {
    pub(crate) name: String,
    source: io::Error,
}

impl std::fmt::Debug for ReadFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ReadFailure")
            .field("name", &self.name)
            .field("source", &"<withheld>")
            .finish()
    }
}

impl ReadFailure {
    fn at(name: &str, source: io::Error) -> Self {
        Self {
            name: name.to_owned(),
            source,
        }
    }

    /// Another process held the usage lock past the read's wait.
    pub(crate) fn busy(&self) -> bool {
        self.source.kind() == io::ErrorKind::TimedOut
    }

    pub(crate) fn category(&self) -> &'static str {
        match self.source.kind() {
            io::ErrorKind::InvalidData => "invalid contents",
            _ => "unsafe or unreadable state",
        }
    }

    /// The plain sentence a refusal or `status` gives. Engine errors pass no
    /// folder, so they name only a generated file name; `status` runs as the
    /// folder's owner and passes it, so the sentence names the full path.
    pub(crate) fn sentence(&self, folder: Option<&Path>) -> String {
        let subject = match (folder, self.name == DIRECTORY) {
            (Some(folder), true) => folder.display().to_string(),
            (Some(folder), false) => folder.join(&self.name).display().to_string(),
            (None, true) => "the usage folder that thinkthen status names".to_owned(),
            (None, false) => self.name.clone(),
        };
        let elsewhere = folder.is_none() && self.name != DIRECTORY;
        let head = "cannot read the usage totals";
        match self.source.kind() {
            io::ErrorKind::TimedOut => format!(
                "{head}: {subject} is locked by another process. Try again when it finishes."
            ),
            io::ErrorKind::InvalidData if elsewhere => format!(
                "{head}: {subject} has invalid contents. Move it out of the usage folder that thinkthen status names, and counting starts again."
            ),
            io::ErrorKind::InvalidData => format!(
                "{head}: {subject} has invalid contents. Move it aside, and counting starts again."
            ),
            _ if elsewhere => format!(
                "{head}: {subject} has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it out of the usage folder that thinkthen status names."
            ),
            _ => format!(
                "{head}: {subject} has unsafe or unreadable state. Make it private to your user (folder 0700, files 0600), or move it aside."
            ),
        }
    }
}

impl std::fmt::Display for ReadFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.name, self.category())
    }
}

impl std::error::Error for ReadFailure {}

impl From<ReadFailure> for io::Error {
    fn from(value: ReadFailure) -> Self {
        value.source
    }
}

#[derive(Debug)]
pub(crate) struct Totals {
    pub(crate) month: Counts,
    pub(crate) total: Counts,
}

const ZERO: Totals = Totals {
    month: Counts::ZERO,
    total: Counts::ZERO,
};

fn at(name: &'static str) -> impl Fn(io::Error) -> ReadFailure {
    move |error| ReadFailure::at(name, error)
}

/// Read the month and all-month totals under a shared lock that waits at
/// most one second. A missing folder holds no count. A folder without
/// `.lock` holds no month thinkthen wrote, because the writer makes `.lock`
/// first, so its months are read without the lock: a writer installs a month
/// only by rename, and a malformed one refuses as it would under the lock.
/// A scan that fails after a writer made `.lock` reads again under it.
pub(crate) fn read(path: &Path, month: &str) -> Result<Totals, ReadFailure> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(ZERO),
        Err(error) => return Err(ReadFailure::at(DIRECTORY, error)),
        Ok(metadata) => validate_directory(&metadata).map_err(at(DIRECTORY))?,
    }
    let directory = open_verified(path, true, 0o700).map_err(at(DIRECTORY))?;
    let lock_path = path.join(".lock");
    let held = match fs::symlink_metadata(&lock_path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(ReadFailure::at(".lock", error)),
        Ok(_) => {
            let lock = open_verified(&lock_path, false, 0o600).map_err(at(".lock"))?;
            super::lock::shared(&lock).map_err(at(".lock"))?;
            verify_identity(&lock_path, &lock, false).map_err(at(".lock"))?;
            Some(lock)
        }
    };
    verify_identity(path, &directory, true).map_err(at(DIRECTORY))?;
    let months = match scan(path) {
        // A first writer made `.lock` and replaced a month while the scan
        // ran unlocked, so read again under the lock it now holds.
        Err(_) if held.is_none() && fs::symlink_metadata(&lock_path).is_ok() => {
            return read(path, month);
        }
        scanned => scanned?,
    };
    let mut current = Counts::default();
    let mut total = Counts::default();
    for (name, counts) in months {
        total = total
            .checked_add(counts)
            .ok_or_else(|| ReadFailure::at(&name, overflow()))?;
        if name == format!("{month}.json") {
            current = counts;
        }
    }
    Ok(Totals {
        month: current,
        total,
    })
}

pub(super) fn update(path: &Path, month: &str, delta: Counts, shared: &Shared) -> io::Result<()> {
    maybe_fail(Stage::Setup)?;
    make_private_directory(path)?;
    let directory = open_verified(path, true, 0o700)?;
    let lock_path = path.join(".lock");
    let (lock, created) = open_stable_lock(&lock_path)?;
    pause_after_creation(created);
    maybe_fail(Stage::Lock)?;
    super::lock::acquire(&lock, shared)?;
    verify_identity(path, &directory, true)?;
    verify_identity(&lock_path, &lock, false)?;
    maybe_fail(Stage::Validation)?;
    let months = scan(path).map_err(|error| io::Error::new(error.source.kind(), error))?;
    let name = format!("{month}.json");
    let mut total = Counts::default();
    let mut old = None;
    for (found, counts) in &months {
        total = total.checked_add(*counts).ok_or_else(overflow)?;
        if *found == name {
            old = Some(*counts);
        }
    }
    let next = old
        .unwrap_or_default()
        .checked_add(delta)
        .ok_or_else(overflow)?;
    let _prospective_total = total.checked_add(delta).ok_or_else(overflow)?;
    if old.is_none() {
        lock.sync_all()?;
        sync_directory(&directory)?;
        note_initial_sync();
    }
    replace(path, &directory, &name, next)
}

/// Every recognized month, sorted. Any other name, such as an older build's
/// `retries-` file, is not a month and is ignored.
fn scan(path: &Path) -> Result<Vec<(String, Counts)>, ReadFailure> {
    let mut names = Vec::new();
    for entry in fs::read_dir(path).map_err(|error| ReadFailure::at(DIRECTORY, error))? {
        let entry = entry.map_err(|error| ReadFailure::at(DIRECTORY, error))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if recognized_month(&name) {
            names.push(name);
        }
    }
    names.sort();
    let mut rows = Vec::with_capacity(names.len());
    for name in names {
        let monthly = path.join(&name);
        let at = |error| ReadFailure::at(&name, error);
        let mut file = open_verified(&monthly, false, 0o600).map_err(at)?;
        let counts = read_counts(&mut file).map_err(at)?;
        verify_identity(&monthly, &file, false).map_err(at)?;
        rows.push((name, counts));
    }
    Ok(rows)
}

fn read_counts(file: &mut File) -> io::Result<Counts> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    serde_json::from_slice(&bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn replace(path: &Path, directory: &File, name: &str, counts: Counts) -> io::Result<()> {
    let temporary = path.join(".update.tmp");
    let mut file = open_private(&temporary, true)?;
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    maybe_fail(Stage::Write)?;
    serde_json::to_writer(&mut file, &counts).map_err(io::Error::other)?;
    file.write_all(b"\n")?;
    maybe_fail(Stage::FileSync)?;
    file.sync_all()?;
    verify_identity(&temporary, &file, false)?;
    drop(file);
    maybe_fail(Stage::Rename)?;
    fs::rename(&temporary, path.join(name))?;
    maybe_fail(Stage::DirectorySync)?;
    sync_directory(directory)
}
