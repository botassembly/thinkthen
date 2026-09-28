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

/// The latest signalled live interval and its millisecond clock. One atomic
/// value prevents a delayed handler from publishing an old interval over a
/// newer signal. The interval changes when the last invoke ends.
static SIGNAL_EVENT: AtomicU64 = AtomicU64::new(0);

/// Scalar invokes, warm finalizes, and table scans running now. A query that
/// is still reading its chunk counts (R6-6).
// The high half identifies one uninterrupted interval with live invokes.
// The low half counts them. Changing both in one CAS prevents a new query
// from inheriting the previous query's signal wave at the last-drop boundary.
static INVOKING: AtomicU64 = AtomicU64::new(0);
const COUNT_MASK: u64 = u32::MAX as u64;

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
fn publish_signal(active: u64) {
    if active & COUNT_MASK != 0 {
        let epoch = active >> 32;
        let event = (epoch << 32) | (now_ms() & COUNT_MASK);
        SIGNAL_EVENT.fetch_max(event, Ordering::SeqCst);
    }
}

fn on_signal() {
    publish_signal(INVOKING.load(Ordering::SeqCst));
}

/// One running invoke, counted from entry to drop.
#[derive(Debug)]
pub(crate) struct Invoke {
    started: u32,
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
            started: now_ms() as u32,
            epoch,
        }
    }

    /// The one interrupt predicate: a SIGINT landed inside this invoke, or
    /// landed within one wave before it while another invoke of the query ran.
    pub(crate) fn stopped(&self) -> bool {
        let event = SIGNAL_EVENT.load(Ordering::SeqCst);
        if event >> 32 != self.epoch {
            return false;
        }
        let age = self.started.wrapping_sub(event as u32);
        age > i32::MAX as u32 || age < WAVE_MS as u32
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

#[cfg(test)]
mod tests {
    use super::{INVOKING, Invoke, Ordering, publish_signal, start_clock};

    #[test]
    fn a_signal_wave_reaches_siblings_but_not_the_next_interval() {
        start_clock();
        let first = Invoke::begin();
        let old_interval = INVOKING.load(Ordering::SeqCst);

        // The sibling begins after the handler reads the live interval but
        // before it publishes the signal. It must still stop.
        let sibling = Invoke::begin();
        publish_signal(old_interval);
        assert!(first.stopped());
        assert!(sibling.stopped());
        drop(sibling);
        drop(first);

        // The handler read the old interval while it was live, then the
        // final owner ended and a new query began before publication.
        let old = Invoke::begin();
        let old_interval = INVOKING.load(Ordering::SeqCst);
        drop(old);
        let next = Invoke::begin();
        publish_signal(old_interval);
        assert!(!next.stopped(), "the next query inherited the prior signal");
    }
}
