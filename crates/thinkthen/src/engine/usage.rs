//! Private process counters and durable count-only monthly aggregates.

use std::io;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError, TryLockError};
use std::thread::{self, JoinHandle};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

mod counts;
pub(crate) use counts::Counts;
mod attempt;
mod facts;
mod timing;
pub(crate) use facts::Snapshot as RunSnapshot;
mod files;
mod lock;
pub(crate) use files::open_read;
use files::{
    make_private_directory, open_private, open_stable_lock, open_verified, sync_directory,
    validate_directory, verify_identity,
};
mod storage;
pub(crate) use storage::read;
use storage::{ReadFailure, update};

/// Persistence of this engine's current count-only deltas.
/// Written does not describe future calls or other engines and processes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(
    test,
    derive(schemars::JsonSchema),
    schemars(rename = "completeUsagePersistence")
)]
pub enum UsagePersistence {
    /// No usage storage was selected.
    Disabled,
    /// Deltas are queued, being written, or momentarily unavailable to observe.
    Pending,
    /// This engine's current deltas have drained successfully.
    Written,
    /// A writer or queue failure was latched; durable totals are incomplete.
    Failed,
}

impl UsagePersistence {
    /// Fixed safe advice, present only after a persistence failure.
    #[must_use]
    pub const fn advice(self) -> Option<&'static str> {
        match self {
            Self::Failed => Some("check the usage folder permissions and free space"),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct Counters {
    path: Option<PathBuf>,
    shared: Arc<Shared>,
    /// The one read before this process's first send (ticket 0360). It keeps
    /// its result, so after a refusal every later send refuses too.
    readable: std::sync::OnceLock<Result<(), String>>,
    held_model_mismatch: std::sync::atomic::AtomicBool,
}

/// What the requests hand the one writer thread, so no request waits on a file.
#[derive(Debug, Default)]
struct Shared {
    queue: Mutex<Queue>,
    /// The single failure latch, readable without waiting for the queue.
    failed: AtomicBool,
    changed: Condvar,
    http: timing::Timeline,
}

#[derive(Debug, Default)]
struct Queue {
    /// The process totals, which the command, library, and SQL surfaces read.
    totals: Counts,
    facts: facts::State,
    /// Deltas not yet written, one sum for each month they were counted in.
    pending: Vec<(String, Counts)>,
    writing: bool,
    failed_file: Option<(String, &'static str)>,
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
            readable: std::sync::OnceLock::new(),
            held_model_mismatch: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Refuse a send when the usage folder exists and cannot be read, so no
    /// surface stops counting without a word. A lock another process holds
    /// past the read's wait passes: it says nothing about the files.
    pub(crate) fn check_readable(&self) -> Result<(), crate::engine::error::Error> {
        let Some(path) = self.path.as_deref() else {
            return Ok(());
        };
        self.readable
            .get_or_init(|| match read(path, &month_now()) {
                Err(failure) if !failure.busy() => Err(failure.sentence(None)),
                _ => Ok(()),
            })
            .clone()
            .map_err(crate::engine::error::Error::UsageUnreadable)
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
        if settled.is_err() {
            self.shared.failed.store(true, Ordering::Release);
        }
        self.shared.failed.load(Ordering::Acquire)
    }

    /// Observe without waiting for the writer, filesystem, or queue mutex.
    pub(crate) fn persistence(&self) -> UsagePersistence {
        if self.shared.failed.load(Ordering::Acquire) {
            return UsagePersistence::Failed;
        }
        if self.path.is_none() {
            return UsagePersistence::Disabled;
        }
        let observed = match self.shared.queue.try_lock() {
            Ok(queue) if queue.writing || !queue.pending.is_empty() => UsagePersistence::Pending,
            Ok(_) => UsagePersistence::Written,
            Err(TryLockError::WouldBlock) => UsagePersistence::Pending,
            Err(TryLockError::Poisoned(_)) => {
                self.shared.failed.store(true, Ordering::Release);
                UsagePersistence::Failed
            }
        };
        if self.shared.failed.load(Ordering::Acquire) {
            UsagePersistence::Failed
        } else {
            observed
        }
    }

    /// Hold the queue lock, as a parent thread may hold it when its process forks.
    #[cfg(test)]
    pub(crate) fn hold_queue(&self) -> impl Sized + '_ {
        self.shared.queue.lock()
    }

    /// Only generated usage filenames and fixed categories may reach diagnostics.
    pub(crate) fn failed_file(&self) -> Option<(String, &'static str)> {
        self.shared.queue.lock().ok()?.failed_file.clone()
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
        if totals.is_none() && (delta.input_tokens != 0 || delta.output_tokens != 0) {
            queue.facts.token_sum_valid = false;
        }
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
        if queue.writer.is_none() && !self.shared.failed.load(Ordering::Acquire) {
            let (path, shared, carried) = (path.to_path_buf(), Arc::clone(&self.shared), carried());
            let writer =
                thread::Builder::new().spawn(move || write_behind(&path, &shared, carried));
            queue.writer = writer.ok();
        }
        if totals.is_none()
            || queued.is_none()
            || self.shared.failed.load(Ordering::Acquire)
            || queue.writer.is_none()
        {
            self.shared.failed.store(true, Ordering::Release);
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
    crate::engine::workers::mask_host_signals();
    carried();
    let mut queue = shared.queue.lock();
    let idle = |held: &mut Queue| held.pending.is_empty() && !held.closing;
    let write = |(month, sum): &(String, Counts)| update(path, month, *sum, shared);
    while let Ok(mut held) = queue {
        held = match shared.changed.wait_while(held, idle) {
            Ok(held) if !held.pending.is_empty() => held,
            _ => return,
        };
        let taken = std::mem::take(&mut held.pending);
        held.writing = true;
        drop(held);
        // A write that unwinds counts as failed, so `finish()` never waits on it.
        let result = catch_unwind(AssertUnwindSafe(|| taken.iter().try_for_each(write)));
        let failed_file = match &result {
            Ok(Err(error)) => error
                .get_ref()
                .and_then(|source| source.downcast_ref::<ReadFailure>())
                .map(|failure| (failure.name.clone(), failure.category())),
            _ => None,
        };
        let written = matches!(result, Ok(Ok(())));
        queue = shared.queue.lock().map(|mut held| {
            held.writing = false;
            if !written {
                shared.failed.store(true, Ordering::Release);
            }
            if held.failed_file.is_none() {
                held.failed_file = failed_file;
            }
            if shared.failed.load(Ordering::Acquire) {
                held.pending.clear();
            }
            shared.changed.notify_all();
            held
        });
    }
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
