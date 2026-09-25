//! SIGINT and the interrupt predicate (ticket 0110 decisions 7 and 8).
//!
//! LOAD takes SIGINT through `sigaction` and chains to the host's own action.
//! The handler only bumps atomics, so it never allocates, locks, or calls
//! into `thinkthen`. Each engine call builds its own token, and the waiting
//! DuckDB thread reads [`Invoke::stopped`] on every tick. No token outlives a
//! call, so a signal that ended one query never reaches the next (R1-21).

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::Instant;

mod ffi;

pub(crate) use ffi::install;

/// Every SIGINT the handler has seen in this process.
static SIGNALS: AtomicU64 = AtomicU64::new(0);

/// When the last SIGINT landed while an invoke ran, in milliseconds on
/// [`now_ms`]'s clock; 0 when none has.
static SIGNAL_AT: AtomicU64 = AtomicU64::new(0);

/// Scalar invokes, warm finalizes, and table scans running now. A query that
/// is still reading its chunk counts (R6-6).
static INVOKING: AtomicUsize = AtomicUsize::new(0);

/// A signal landing this close before an invoke begins belongs to the query
/// that invoke serves: its other threads were running when it landed.
const WAVE_MS: u64 = 10;

/// The clock the handler reads, started at LOAD so the handler never
/// initializes it.
static CLOCK: OnceLock<Instant> = OnceLock::new();

/// Milliseconds since LOAD, never 0 once a signal can land.
fn now_ms() -> u64 {
    CLOCK
        .get()
        .map_or(0, |start| {
            u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX)
        })
        .saturating_add(1)
}

/// Start the clock. LOAD calls this before it installs the handler.
pub(crate) fn start_clock() {
    let _ = CLOCK.get_or_init(Instant::now);
}

/// The handler's work, in plain atomics.
fn on_signal() {
    SIGNALS.fetch_add(1, Ordering::SeqCst);
    if INVOKING.load(Ordering::SeqCst) > 0 {
        SIGNAL_AT.store(now_ms(), Ordering::SeqCst);
    }
}

/// One running invoke, counted from entry to drop.
#[derive(Debug)]
pub(crate) struct Invoke {
    signals: u64,
    started: u64,
}

impl Invoke {
    /// Count the calling invoke as running.
    pub(crate) fn begin() -> Self {
        INVOKING.fetch_add(1, Ordering::SeqCst);
        Self {
            signals: SIGNALS.load(Ordering::SeqCst),
            started: now_ms(),
        }
    }

    /// The one interrupt predicate: a SIGINT landed inside this invoke, or
    /// landed within one wave before it while another invoke of the query ran.
    pub(crate) fn stopped(&self) -> bool {
        if SIGNALS.load(Ordering::SeqCst) > self.signals {
            return true;
        }
        let at = SIGNAL_AT.load(Ordering::SeqCst);
        at != 0 && at <= self.started && self.started - at < WAVE_MS
    }
}

impl Drop for Invoke {
    fn drop(&mut self) {
        INVOKING.fetch_sub(1, Ordering::SeqCst);
    }
}
