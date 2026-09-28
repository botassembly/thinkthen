//! Private process counters and durable count-only monthly aggregates.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read as _, Seek as _, SeekFrom, Write as _};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::core::Usage;

mod counts;
pub(crate) use counts::Counts;
mod attempt;
mod facts;
pub(crate) use facts::Snapshot as RunSnapshot;
mod lock;

const SCHEMA: &str = "thinkthen.usage/1";

#[derive(Debug, Default)]
pub(crate) struct Counters {
    path: Option<PathBuf>,
    shared: Arc<Shared>,
}

/// What the requests hand the one writer thread, so no request waits on a file.
#[derive(Debug, Default)]
struct Shared {
    queue: Mutex<Queue>,
    changed: Condvar,
}

#[derive(Debug, Default)]
struct Queue {
    /// The process totals, which the command, library, and SQL surfaces read.
    totals: Counts,
    facts: facts::State,
    /// Deltas not yet written, one sum for each month they were counted in.
    pending: Vec<(String, Counts)>,
    writing: bool,
    /// Set by the first failure. Nothing is written after it.
    failed: bool,
    closing: bool,
    /// One deadline for every lock acquisition left after finalization starts.
    finish_deadline: Option<Instant>,
    writer: Option<JoinHandle<()>>,
}

impl Counters {
    pub(crate) fn new(path: Option<PathBuf>) -> Self {
        Self {
            path,
            shared: Arc::default(),
        }
    }

    /// The usage folder these counters add to, fixed when they were made.
    pub(crate) fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn request_sent(&self) {
        self.attempt_sent(false);
    }

    #[cfg(test)]
    pub(crate) fn attempt_sent(&self, retry: bool) {
        self.add(Counts {
            requests_sent: 1,
            retries: u64::from(retry),
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

    /// Wait for the writer, giving all remaining usage-lock acquisitions one
    /// finalization deadline. Other filesystem operations remain unbounded.
    pub(crate) fn finish(&self) -> bool {
        let busy = |queue: &mut Queue| {
            queue.writer.is_some() && (queue.writing || !queue.pending.is_empty())
        };
        let settled = self.shared.queue.lock().and_then(|mut queue| {
            queue.finish_deadline.get_or_insert_with(lock::deadline);
            self.shared.changed.notify_all();
            self.shared.changed.wait_while(queue, busy)
        });
        settled.map_or(true, |queue| queue.failed)
    }

    /// The process totals so far. Reading them sends and writes nothing.
    pub(crate) fn snapshot(&self) -> Counts {
        let queue = self.shared.queue.lock();
        queue.unwrap_or_else(PoisonError::into_inner).totals
    }

    fn add(&self, delta: Counts) {
        let Ok(mut queue) = self.shared.queue.lock() else {
            return;
        };
        let totals = queue.totals.checked_add(delta);
        queue.totals = totals.unwrap_or(queue.totals);
        let Some(path) = self.path.as_deref() else {
            return;
        };
        let month = month_now();
        let queued = match queue.pending.last_mut() {
            Some((last, sum)) if *last == month => sum.checked_add(delta).map(|next| *sum = next),
            _ => {
                queue.pending.push((month, delta));
                Some(())
            }
        };
        if queue.writer.is_none() && !queue.failed {
            let (path, shared, carried) = (path.to_path_buf(), Arc::clone(&self.shared), carried());
            let writer =
                thread::Builder::new().spawn(move || write_behind(&path, &shared, carried));
            queue.writer = writer.ok();
        }
        if totals.is_none() || queued.is_none() || queue.failed || queue.writer.is_none() {
            queue.failed = true;
            queue.pending.clear();
            return;
        }
        self.shared.changed.notify_all();
    }
}

impl Drop for Counters {
    /// Write what is pending, then stop the writer.
    fn drop(&mut self) {
        let writer = self.shared.queue.lock().ok().and_then(|mut queue| {
            queue.closing = true;
            queue.finish_deadline.get_or_insert_with(lock::deadline);
            queue.writer.take()
        });
        self.shared.changed.notify_all();
        if let Some(writer) = writer {
            let _stopped = writer.join();
        }
    }
}

/// The writer thread: each pass writes everything that piled up since the last.
fn write_behind(path: &Path, shared: &Shared, carried: impl FnOnce()) {
    carried();
    let mut queue = shared.queue.lock();
    let idle = |held: &mut Queue| held.pending.is_empty() && !held.closing;
    let write = |(month, sum): &(String, Counts)| update(path, month, *sum, shared).is_ok();
    while let Ok(mut held) = queue {
        held = match shared.changed.wait_while(held, idle) {
            Ok(held) if !held.pending.is_empty() => held,
            _ => return,
        };
        let taken = std::mem::take(&mut held.pending);
        held.writing = true;
        drop(held);
        // A write that unwinds counts as failed, so `finish()` never waits on it.
        let written = catch_unwind(AssertUnwindSafe(|| taken.iter().all(write))).unwrap_or(false);
        queue = shared.queue.lock().map(|mut held| {
            held.writing = false;
            held.failed |= !written;
            if held.failed {
                held.pending.clear();
            }
            shared.changed.notify_all();
            held
        });
    }
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

fn update(path: &Path, month: &str, delta: Counts, shared: &Shared) -> io::Result<()> {
    maybe_fail(Stage::Setup)?;
    make_private_directory(path)?;
    let directory = open_verified(path, true, 0o700)?;
    let lock_path = path.join(".lock");
    let (lock, created) = open_stable_lock(&lock_path)?;
    pause_after_creation(created);
    maybe_fail(Stage::Lock)?;
    lock::acquire(&lock, shared)?;
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

/// A test's injected failure, carried from the thread that starts the writer.
#[cfg(test)]
fn carried() -> impl FnOnce() {
    let stage = FAILURE.with(std::cell::Cell::take);
    move || FAILURE.with(|failure| failure.set(stage))
}

#[cfg(not(test))]
const fn carried() -> impl FnOnce() {
    || ()
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
    let mut month = String::with_capacity(32);
    month_now_into(&mut month);
    month
}

/// Write the admission month into a buffer reserved before the final stop check.
fn month_now_into(month: &mut String) {
    use std::fmt::Write as _;

    let days = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 86_400;
    let (year, number) = year_month(days as i64);
    // The largest u64 second reaches a 12-digit year. Thirty-two bytes hold it.
    month.clear();
    let _written = write!(month, "{year:04}-{number:02}");
    debug_assert!(month.len() <= 32);
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
