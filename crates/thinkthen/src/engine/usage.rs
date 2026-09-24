//! Private process counters and durable count-only monthly aggregates.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read as _, Seek as _, SeekFrom, Write as _};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::core::Usage;

const SCHEMA: &str = "thinkthen.usage/1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Counts {
    schema: UsageSchema,
    pub(crate) requests_sent: u64,
    pub(crate) input_tokens: u64,
    pub(crate) output_tokens: u64,
    pub(crate) cache_answers: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
enum UsageSchema {
    #[serde(rename = "thinkthen.usage/1")]
    One,
}

impl Default for Counts {
    fn default() -> Self {
        Self {
            schema: UsageSchema::One,
            requests_sent: 0,
            input_tokens: 0,
            output_tokens: 0,
            cache_answers: 0,
        }
    }
}

impl Counts {
    fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            schema: UsageSchema::One,
            requests_sent: self.requests_sent.checked_add(other.requests_sent)?,
            input_tokens: self.input_tokens.checked_add(other.input_tokens)?,
            output_tokens: self.output_tokens.checked_add(other.output_tokens)?,
            cache_answers: self.cache_answers.checked_add(other.cache_answers)?,
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct Counters {
    requests_sent: AtomicU64,
    input_tokens: AtomicU64,
    output_tokens: AtomicU64,
    cache_answers: AtomicU64,
    persistent: Mutex<Option<PathBuf>>,
    warned: AtomicBool,
}

impl Counters {
    pub(crate) fn new(path: Option<PathBuf>) -> Self {
        Self {
            requests_sent: AtomicU64::new(0),
            input_tokens: AtomicU64::new(0),
            output_tokens: AtomicU64::new(0),
            cache_answers: AtomicU64::new(0),
            persistent: Mutex::new(path),
            warned: AtomicBool::new(false),
        }
    }

    pub(crate) fn request_sent(&self) {
        self.add(Counts {
            requests_sent: 1,
            ..Counts::default()
        });
    }

    pub(crate) fn tokens(&self, usage: Usage) {
        let (input_tokens, output_tokens) = usage.token_counts();
        self.add(Counts {
            input_tokens,
            output_tokens,
            ..Counts::default()
        });
    }

    pub(crate) fn cache_answer(&self) {
        self.add(Counts {
            cache_answers: 1,
            ..Counts::default()
        });
    }

    pub(crate) fn warning(&self) -> bool {
        self.warned.load(Ordering::Relaxed)
    }

    /// The process totals so far. Reading them sends and writes nothing.
    #[allow(
        dead_code,
        reason = "the command reads durable totals; ticket 0086 exposes this snapshot"
    )]
    pub(crate) fn snapshot(&self) -> Counts {
        Counts {
            requests_sent: self.requests_sent.load(Ordering::Relaxed),
            input_tokens: self.input_tokens.load(Ordering::Relaxed),
            output_tokens: self.output_tokens.load(Ordering::Relaxed),
            cache_answers: self.cache_answers.load(Ordering::Relaxed),
            ..Counts::default()
        }
    }

    fn add(&self, delta: Counts) {
        let process_ok = checked_atomic_add(&self.requests_sent, delta.requests_sent)
            && checked_atomic_add(&self.input_tokens, delta.input_tokens)
            && checked_atomic_add(&self.output_tokens, delta.output_tokens)
            && checked_atomic_add(&self.cache_answers, delta.cache_answers);
        let Ok(mut state) = self.persistent.lock() else {
            self.warned.store(true, Ordering::Relaxed);
            return;
        };
        let Some(path) = state.as_deref() else { return };
        if !process_ok || update(path, &month_now(), delta).is_err() {
            *state = None;
            self.warned.store(true, Ordering::Relaxed);
        }
    }
}

fn checked_atomic_add(value: &AtomicU64, delta: u64) -> bool {
    value
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
            old.checked_add(delta)
        })
        .is_ok()
}

#[derive(Debug)]
pub(crate) struct Totals {
    pub(crate) month: Counts,
    pub(crate) total: Counts,
}

pub(crate) fn read(path: &Path, month: &str) -> io::Result<Totals> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Ok(Totals {
                month: Counts::default(),
                total: Counts::default(),
            });
        }
        Err(error) => return Err(error),
        Ok(metadata) => validate_directory(&metadata)?,
    }
    let directory = open_verified(path, true, 0o700)?;
    let lock_path = path.join(".lock");
    let lock = open_verified(&lock_path, false, 0o600)?;
    File::lock_shared(&lock)?;
    verify_identity(path, &directory, true)?;
    verify_identity(&lock_path, &lock, false)?;
    let mut current = Counts::default();
    let mut total = Counts::default();
    for item in fs::read_dir(path)? {
        let item = item?;
        let name = item.file_name().to_string_lossy().into_owned();
        if !recognized_month(&name) {
            continue;
        }
        let mut file = open_verified(&item.path(), false, 0o600)?;
        let counts = read_counts(&mut file)?;
        verify_identity(&item.path(), &file, false)?;
        total = total.checked_add(counts).ok_or_else(overflow)?;
        if name == format!("{month}.json") {
            current = counts;
        }
    }
    Ok(Totals {
        month: current,
        total,
    })
}

fn update(path: &Path, month: &str, delta: Counts) -> io::Result<()> {
    maybe_fail(Stage::Setup)?;
    make_private_directory(path)?;
    let directory = open_verified(path, true, 0o700)?;
    let lock_path = path.join(".lock");
    let (lock, created) = open_stable_lock(&lock_path)?;
    pause_after_creation(created);
    maybe_fail(Stage::Lock)?;
    File::lock(&lock)?;
    verify_identity(path, &directory, true)?;
    verify_identity(&lock_path, &lock, false)?;
    let monthly = path.join(format!("{month}.json"));
    let old = match fs::symlink_metadata(&monthly) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            lock.sync_all()?;
            directory.sync_all()?;
            note_initial_sync();
            Counts::default()
        }
        Err(error) => return Err(error),
        Ok(_) => {
            maybe_fail(Stage::Validation)?;
            let mut file = open_verified(&monthly, false, 0o600)?;
            let value = read_counts(&mut file)?;
            verify_identity(&monthly, &file, false)?;
            value
        }
    };
    let next = old.checked_add(delta).ok_or_else(overflow)?;
    let temporary = path.join(".update.tmp");
    let mut file = open_private(&temporary, true)?;
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    maybe_fail(Stage::Write)?;
    serde_json::to_writer(&mut file, &next).map_err(io::Error::other)?;
    file.write_all(b"\n")?;
    maybe_fail(Stage::FileSync)?;
    file.sync_all()?;
    verify_identity(&temporary, &file, false)?;
    drop(file);
    maybe_fail(Stage::Rename)?;
    fs::rename(&temporary, &monthly)?;
    maybe_fail(Stage::DirectorySync)?;
    directory.sync_all()
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Stage {
    Setup,
    Lock,
    Validation,
    Write,
    FileSync,
    Rename,
    DirectorySync,
}

#[cfg(not(test))]
#[derive(Clone, Copy)]
enum Stage {
    Setup,
    Lock,
    Validation,
    Write,
    FileSync,
    Rename,
    DirectorySync,
}

#[cfg(test)]
thread_local! {
    static FAILURE: std::cell::Cell<Option<Stage>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn maybe_fail(stage: Stage) -> io::Result<()> {
    FAILURE.with(|failure| {
        if failure.get() == Some(stage) {
            failure.set(None);
            Err(io::Error::other("injected usage failure"))
        } else {
            Ok(())
        }
    })
}

#[cfg(not(test))]
const fn maybe_fail(_stage: Stage) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
type CreationPause = (
    std::sync::Arc<std::sync::Barrier>,
    std::sync::Arc<std::sync::Barrier>,
);

#[cfg(test)]
thread_local! {
    static CREATION_PAUSE: std::cell::RefCell<Option<CreationPause>> = const { std::cell::RefCell::new(None) };
    static INITIAL_SYNC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
fn pause_after_creation(created: bool) {
    if created {
        CREATION_PAUSE.with(|pause| {
            if let Some((created, release)) = pause.borrow_mut().take() {
                created.wait();
                release.wait();
            }
        });
    }
}

#[cfg(not(test))]
const fn pause_after_creation(_created: bool) {}

#[cfg(test)]
fn note_initial_sync() {
    INITIAL_SYNC.with(|observed| observed.set(true));
}

#[cfg(not(test))]
const fn note_initial_sync() {}

fn read_counts(file: &mut File) -> io::Result<Counts> {
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(io::Error::other)?;
    if value.get("schema").and_then(serde_json::Value::as_str) != Some(SCHEMA) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "usage schema"));
    }
    serde_json::from_value(value).map_err(io::Error::other)
}

fn make_private_directory(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(path)?;
    }
    #[cfg(not(unix))]
    fs::create_dir_all(path)?;
    validate_directory(&fs::symlink_metadata(path)?)
}

fn open_private(path: &Path, create: bool) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(create);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let file = options.open(path)?;
    verify_private_file(path, &file)?;
    Ok(file)
}

fn open_stable_lock(path: &Path) -> io::Result<(File, bool)> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    match options.open(path) {
        Ok(file) => {
            verify_private_file(path, &file)?;
            Ok((file, true))
        }
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            open_private(path, false).map(|file| (file, false))
        }
        Err(error) => Err(error),
    }
}

fn verify_private_file(path: &Path, file: &File) -> io::Result<()> {
    verify_identity(path, file, false)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if file.metadata()?.permissions().mode() & 0o777 != 0o600 {
            return Err(permission());
        }
    }
    Ok(())
}

fn open_verified(path: &Path, directory: bool, mode: u32) -> io::Result<File> {
    let file = File::open(path)?;
    verify_identity(path, &file, directory)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if file.metadata()?.permissions().mode() & 0o777 != mode {
            return Err(permission());
        }
    }
    Ok(file)
}

fn verify_identity(path: &Path, file: &File, directory: bool) -> io::Result<()> {
    let named = fs::symlink_metadata(path)?;
    if named.file_type().is_symlink() || named.is_dir() != directory || named.is_file() == directory
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "unsafe usage object",
        ));
    }
    let opened = file.metadata()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt as _;
        if (named.dev(), named.ino()) != (opened.dev(), opened.ino()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "usage identity changed",
            ));
        }
    }
    Ok(())
}

fn validate_directory(metadata: &fs::Metadata) -> io::Result<()> {
    if !metadata.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "usage path"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if metadata.permissions().mode() & 0o777 != 0o700 {
            return Err(permission());
        }
    }
    Ok(())
}

fn permission() -> io::Error {
    io::Error::new(io::ErrorKind::PermissionDenied, "unsafe usage mode")
}
fn overflow() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "usage overflow")
}

fn recognized_month(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() != 12 || bytes.get(4) != Some(&b'-') || bytes.get(7..) != Some(b".json") {
        return false;
    }
    let Ok(year) = name[0..4].parse::<u16>() else {
        return false;
    };
    let Ok(month) = name[5..7].parse::<u8>() else {
        return false;
    };
    year > 0 && (1..=12).contains(&month)
}

pub(crate) fn month_now() -> String {
    let days = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 86_400;
    let (year, month) = year_month(days as i64);
    format!("{year:04}-{month:02}")
}

fn year_month(days_since_epoch: i64) -> (i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let mp = (5 * day_of_year + 2) / 153;
    let month = mp + if mp < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month)
}

#[cfg(test)]
#[path = "usage/tests.rs"]
mod tests;
