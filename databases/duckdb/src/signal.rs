//! SIGINT and the interrupt predicate (ticket 0110 decisions 7 and 8).
//!
//! LOAD takes SIGINT through `sigaction` and chains to the host's own action.
//! The handler only updates atomics, so it never allocates, locks, or calls
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

/// Full-width time of the latest signal. A handler validates its captured
/// interval after sampling time, before it can advance this value.
static SIGNAL_TIME: AtomicU64 = AtomicU64::new(0);

/// Scalar invokes, warm finalizes, and table scans running now. A query that
/// is still reading its chunk counts (R6-6).
// The high 48 bits identify one uninterrupted interval with live invokes.
// The low 15 bits count them; bit 15 records a signal in that interval.
// Changing all three in one CAS prevents a new query
// from inheriting the previous query's signal wave at the last-drop boundary.
static INVOKING: AtomicU64 = AtomicU64::new(0);
const COUNT_MASK: u64 = 0x7fff;
const SIGNAL_BIT: u64 = 0x8000;
const EPOCH_SHIFT: u32 = 16;
const MAX_EPOCH: u64 = u64::MAX >> EPOCH_SHIFT;

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
fn publish_signal(active: u64, at: u64) {
    publish_signal_to(&INVOKING, &SIGNAL_TIME, active, at);
}

fn publish_signal_to(invoking: &AtomicU64, signal_time: &AtomicU64, active: u64, at: u64) {
    if active & COUNT_MASK != 0 {
        let epoch = active >> EPOCH_SHIFT;
        let mut current = invoking.load(Ordering::SeqCst);
        if current >> EPOCH_SHIFT != epoch || current & COUNT_MASK == 0 {
            return;
        }
        // `at` was sampled before this epoch check. If the interval ends
        // after the check, its later successor signal has a later time.
        signal_time.fetch_max(at, Ordering::SeqCst);
        while current >> EPOCH_SHIFT == epoch && current & COUNT_MASK != 0 {
            match invoking.compare_exchange(
                current,
                current | SIGNAL_BIT,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }
    }
}

fn on_signal() {
    let active = INVOKING.load(Ordering::SeqCst);
    publish_signal(active, now_ms());
}

fn observes_signal(started: u64, epoch: u64, active: u64, at: u64) -> bool {
    active >> EPOCH_SHIFT == epoch
        && active & SIGNAL_BIT != 0
        && (at >= started || started - at < WAVE_MS)
}

fn next_active(active: u64) -> u64 {
    let count = active & COUNT_MASK;
    assert!(count < COUNT_MASK, "too many simultaneous DuckDB invokes");
    let epoch = active >> EPOCH_SHIFT;
    let epoch = if count == 0 {
        assert!(epoch < MAX_EPOCH, "DuckDB invoke generation exhausted");
        epoch + 1
    } else {
        epoch
    };
    let signal = if count == 0 { 0 } else { active & SIGNAL_BIT };
    (epoch << EPOCH_SHIFT) | signal | (count + 1)
}

/// One running invoke, counted from entry to drop.
#[derive(Debug)]
pub(crate) struct Invoke {
    started: u64,
    epoch: u64,
}

impl Invoke {
    /// Count the calling invoke as running.
    pub(crate) fn begin() -> Self {
        let started = now_ms();
        let mut active = INVOKING.load(Ordering::SeqCst);
        let epoch = loop {
            let next = next_active(active);
            match INVOKING.compare_exchange(active, next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break next >> EPOCH_SHIFT,
                Err(observed) => active = observed,
            }
        };
        Self { started, epoch }
    }

    /// The one interrupt predicate: a SIGINT landed inside this invoke, or
    /// landed within one wave before it while another invoke of the query ran.
    pub(crate) fn stopped(&self) -> bool {
        observes_signal(
            self.started,
            self.epoch,
            INVOKING.load(Ordering::SeqCst),
            SIGNAL_TIME.load(Ordering::SeqCst),
        )
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
    use super::{
        AtomicU64, INVOKING, Invoke, Ordering, next_active, observes_signal, publish_signal,
        publish_signal_to, start_clock,
    };

    #[test]
    fn a_signal_wave_reaches_siblings_but_not_the_next_interval() {
        start_clock();
        let first = Invoke::begin();
        let old_interval = INVOKING.load(Ordering::SeqCst);

        // The sibling begins after the handler reads the live interval but
        // before it publishes the signal. It must still stop.
        let sibling = Invoke::begin();
        publish_signal(old_interval, first.started);
        assert!(first.stopped());
        assert!(sibling.stopped());
        drop(sibling);
        drop(first);

        // The handler read the old interval while it was live, then the
        // final owner ended and a new query began before publication.
        let old = Invoke::begin();
        let old_interval = INVOKING.load(Ordering::SeqCst);
        let old_started = old.started;
        drop(old);
        let next = Invoke::begin();
        publish_signal(old_interval, old_started);
        assert!(!next.stopped(), "the next query inherited the prior signal");
        drop(next);
    }

    #[test]
    fn epoch_and_millisecond_rollovers_keep_the_wave() {
        // The old 32/32 event's fetch_max would retain the pre-wrap epoch.
        let before = next_active((u64::from(u32::MAX) - 1) << 16);
        assert_eq!(before >> 16, u64::from(u32::MAX));
        let invoking = AtomicU64::new(before);
        let time = AtomicU64::new(0);
        let old_ms = u64::from(u32::MAX) - 2;
        let new_ms = u64::from(u32::MAX) + 3;
        publish_signal_to(&invoking, &time, before, old_ms);
        let after = next_active(before & !0x7fff);
        assert_eq!(after >> 16, u64::from(u32::MAX) + 1);
        invoking.store(next_active(after), Ordering::SeqCst);
        publish_signal_to(&invoking, &time, after, new_ms);
        assert_eq!(time.load(Ordering::SeqCst), new_ms);
        assert!(observes_signal(
            old_ms,
            after >> 16,
            invoking.load(Ordering::SeqCst),
            time.load(Ordering::SeqCst)
        ));
        assert!(observes_signal(
            new_ms + 5,
            after >> 16,
            invoking.load(Ordering::SeqCst),
            time.load(Ordering::SeqCst)
        ));
        let later = next_active((after >> 16) << 16);
        invoking.store(later, Ordering::SeqCst);
        publish_signal_to(&invoking, &time, after, new_ms);
        assert!(!observes_signal(
            new_ms + 1,
            later >> 16,
            invoking.load(Ordering::SeqCst),
            time.load(Ordering::SeqCst)
        ));

        assert!(!observes_signal(
            new_ms + 10,
            after >> 16,
            (after & !0x7fff) | 0x8001,
            new_ms
        ));
    }

    #[test]
    fn old_handler_cannot_extend_a_new_intervals_real_wave() {
        let old = next_active(0);
        let new = next_active(old & !0x7fff);
        let invoking = AtomicU64::new(new);
        let time = AtomicU64::new(0);
        publish_signal_to(&invoking, &time, new, 100);
        assert!(!observes_signal(
            115,
            new >> 16,
            invoking.load(Ordering::SeqCst),
            time.load(Ordering::SeqCst)
        ));

        // The old handler captured `old`, but resumes after the new signal.
        publish_signal_to(&invoking, &time, old, 200);
        assert_eq!(time.load(Ordering::SeqCst), 100);
        assert!(!observes_signal(
            205,
            new >> 16,
            invoking.load(Ordering::SeqCst),
            time.load(Ordering::SeqCst)
        ));
    }
}
