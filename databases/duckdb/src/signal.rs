//! SIGINT and the interrupt predicate (ticket 0110 decisions 7 and 8).
//!
//! LOAD takes SIGINT through `sigaction` and chains to the host's own action.
//! The handler only bumps atomics, so it never allocates, locks, or calls
//! into `thinkthen`. Each engine call builds its own token, and the waiting
//! DuckDB thread reads [`Invoke::stopped`] on every tick. No token outlives a
//! call, so a signal that ended one query never reaches the next (R1-21).

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[path = "signal/ffi.rs"]
mod ffi;

#[allow(
    unused_imports,
    reason = "the C++ bridge uses the shared handler without relate's pipe"
)]
pub(crate) use ffi::{install, start_bridge};

/// Every SIGINT the handler has seen in this process.
static SIGNALS: AtomicU64 = AtomicU64::new(0);

/// When the last SIGINT landed while an invoke ran, in milliseconds on
/// [`now_ms`]'s clock; 0 when none has.
static SIGNAL_AT: AtomicU64 = AtomicU64::new(0);

/// Scalar invokes, warm finalizes, and table scans running now. A query that
/// is still reading its chunk counts (R6-6).
// The high half identifies one uninterrupted interval with live invokes.
// The low half counts them. Changing both in one CAS prevents a new query
// from inheriting the previous query's signal wave at the last-drop boundary.
static INVOKING: AtomicU64 = AtomicU64::new(0);
const COUNT_MASK: u64 = u32::MAX as u64;
const CLOCK_MASK: u64 = (1 << 40) - 1;

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
    // Capture the active interval before publishing the signal count. An
    // invoke starting after this snapshot must not inherit this signal.
    let active = INVOKING.load(Ordering::SeqCst);
    SIGNALS.fetch_add(1, Ordering::SeqCst);
    if active & COUNT_MASK != 0 {
        let epoch = active >> 32;
        SIGNAL_AT.store(
            ((epoch & 0x00ff_ffff) << 40) | (now_ms() & CLOCK_MASK),
            Ordering::SeqCst,
        );
    }
}

/// One running invoke, counted from entry to drop.
#[derive(Debug)]
pub(crate) struct Invoke {
    signals: u64,
    started: u64,
    epoch: u64,
}

impl Invoke {
    /// Count the calling invoke as running.
    pub(crate) fn begin() -> Self {
        let mut active = INVOKING.load(Ordering::SeqCst);
        let epoch = loop {
            let count = active & COUNT_MASK;
            assert!(count < COUNT_MASK, "too many simultaneous DuckDB invokes");
            let epoch = (active >> 32).wrapping_add(u64::from(count == 0)) & COUNT_MASK;
            let next = (epoch << 32) | (count + 1);
            match INVOKING.compare_exchange(active, next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break epoch,
                Err(observed) => active = observed,
            }
        };
        Self {
            signals: SIGNALS.load(Ordering::SeqCst),
            started: now_ms(),
            epoch,
        }
    }

    /// The one interrupt predicate: a SIGINT landed inside this invoke, or
    /// landed within one wave before it while another invoke of the query ran.
    pub(crate) fn stopped(&self) -> bool {
        if SIGNALS.load(Ordering::SeqCst) > self.signals {
            return true;
        }
        let at = SIGNAL_AT.load(Ordering::SeqCst);
        at != 0
            && (at >> 40) == (self.epoch & 0x00ff_ffff)
            && (self.started & CLOCK_MASK).wrapping_sub(at & CLOCK_MASK) < WAVE_MS
    }
}

impl Drop for Invoke {
    fn drop(&mut self) {
        let mut active = INVOKING.load(Ordering::SeqCst);
        loop {
            let count = active & COUNT_MASK;
            debug_assert!(count > 0);
            let next = (active & !COUNT_MASK) | (count - 1);
            match INVOKING.compare_exchange(active, next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(observed) => active = observed,
            }
        }
    }
}
