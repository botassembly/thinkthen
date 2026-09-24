//! Private request execution, recording, locking, and bounded scheduling.

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::{Duration, Instant};

const CANCEL_POLL: Duration = Duration::from_millis(50);

/// One call's budget and the one instant made from it when the call began.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Deadline {
    budget: Duration,
    at: Instant,
}

impl Deadline {
    /// Start a budget now, or `None` when no clock instant can hold it.
    #[allow(
        dead_code,
        reason = "the command passes no deadline; tickets 0084 through 0086 expose it"
    )]
    pub(crate) fn after(budget: Duration) -> Option<Self> {
        let at = Instant::now().checked_add(budget)?;
        Some(Self { budget, at })
    }
}

/// One private cooperative stop flag shared by a whole engine run, and the
/// optional deadline of the one call that carries it.
#[derive(Clone, Debug, Default)]
pub(crate) struct Cancel {
    fired: Arc<AtomicBool>,
    deadline: Option<Deadline>,
    #[cfg(test)]
    blocked: Option<std::sync::mpsc::Sender<()>>,
    #[cfg(test)]
    keys: Arc<std::sync::atomic::AtomicUsize>,
}

impl Cancel {
    #[cfg(test)]
    pub(crate) fn fire(&self) {
        self.fired.store(true, Ordering::Release);
    }

    pub(crate) fn reset(&self) {
        self.fired.store(false, Ordering::Release);
    }

    pub(crate) fn flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.fired)
    }

    pub(crate) fn fired(&self) -> bool {
        self.fired.load(Ordering::Acquire)
    }

    /// Share this stop flag with one call that ends at `deadline`.
    #[allow(
        dead_code,
        reason = "the command passes no deadline; tickets 0084 through 0086 expose it"
    )]
    pub(crate) fn with_deadline(&self, deadline: Option<Deadline>) -> Self {
        Self {
            deadline,
            ..self.clone()
        }
    }

    /// Observe cancellation first and the deadline second, or the budget left.
    pub(crate) fn stop_or_remaining(&self) -> Result<Option<Duration>, error::Error> {
        if self.fired() {
            return Err(error::Error::Cancelled);
        }
        self.remaining()
    }

    /// Observe the deadline alone, or the budget left.
    pub(crate) fn remaining(&self) -> Result<Option<Duration>, error::Error> {
        let Some(deadline) = self.deadline else {
            return Ok(None);
        };
        let remaining = deadline.at.saturating_duration_since(Instant::now());
        match self.passed() {
            Some(passed) if remaining.is_zero() => Err(passed),
            _ => Ok(Some(remaining)),
        }
    }

    /// The error this call's deadline returns once it has passed.
    pub(crate) fn passed(&self) -> Option<error::Error> {
        let deadline = self.deadline?;
        Some(error::Error::Deadline(error::Budget(deadline.budget)))
    }

    /// The stop this checkpoint observes, if any.
    pub(crate) fn stop(&self) -> Option<error::Error> {
        self.stop_or_remaining().err()
    }

    #[cfg(test)]
    pub(crate) fn observed(blocked: std::sync::mpsc::Sender<()>) -> Self {
        Self {
            blocked: Some(blocked),
            ..Self::default()
        }
    }

    /// How many key lookups calls carrying this token have made.
    #[cfg(test)]
    pub(crate) fn keys(&self) -> usize {
        self.keys.load(Ordering::SeqCst)
    }

    pub(crate) fn key_lookup(&self) {
        #[cfg(test)]
        self.keys.fetch_add(1, Ordering::SeqCst);
    }

    pub(crate) fn observed_block(&self) {
        #[cfg(test)]
        if let Some(blocked) = &self.blocked {
            let _observed = blocked.send(());
        }
    }

    /// Wait for a bounded operation, observing cancellation every 50 ms and
    /// waking when the deadline passes. Return the stop that ended the wait.
    pub(crate) fn wait(&self, duration: Duration) -> Option<error::Error> {
        let started = Instant::now();
        loop {
            let budget = match self.stop_or_remaining() {
                Ok(budget) => budget.unwrap_or(Duration::MAX),
                Err(stop) => return Some(stop),
            };
            let remaining = duration.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                return None;
            }
            thread::sleep(remaining.min(CANCEL_POLL).min(budget));
        }
    }

    pub(crate) const fn poll() -> Duration {
        CANCEL_POLL
    }
}

/// How many live attempts may be in flight at once: 1 through 32.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Width(u8);

impl Width {
    /// The width every call follows until an explicit width is selected.
    pub(crate) const FALLBACK: Self = Self(4);

    /// Accept 1 through 32. Width 0 would block every caller forever.
    pub(crate) fn new(value: u64) -> Result<Self, error::Error> {
        u8::try_from(value)
            .ok()
            .filter(|width| (1..=32).contains(width))
            .map(Self)
            .ok_or(error::Error::Usage(
                "a width is a whole number from 1 through 32",
            ))
    }

    pub(crate) fn get(self) -> usize {
        usize::from(self.0)
    }
}

/// A later explicit width that differs from the one this process selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WidthActive(pub(crate) Width);

impl fmt::Display for WidthActive {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let active = self.0.get();
        write!(
            formatter,
            "width {active} is already active for this process; use width {active} or drop the width argument"
        )
    }
}

/// The width selection and the one attempt gate of one process.
///
/// Ticket 0096 replaces this whole value in a forked child, so it holds the
/// only lock and the only condition variable on the width path.
#[derive(Debug)]
pub(crate) struct Widths {
    state: Mutex<WidthState>,
    freed: Condvar,
}

#[derive(Debug)]
struct WidthState {
    selected: Option<Width>,
    active: usize,
}

impl Default for Widths {
    fn default() -> Self {
        Self::new()
    }
}

impl Widths {
    const fn new() -> Self {
        Self {
            state: Mutex::new(WidthState {
                selected: None,
                active: 0,
            }),
            freed: Condvar::new(),
        }
    }

    /// Register one engine's width and return the width its calls follow.
    ///
    /// `None` never selects. The first explicit width selects itself, the same
    /// width again is accepted, and a different one is refused.
    pub(crate) fn select(&self, asked: Option<Width>) -> Result<Width, WidthActive> {
        let mut state = self.lock();
        match (asked, state.selected) {
            (None, selected) => Ok(selected.unwrap_or(Width::FALLBACK)),
            (Some(asked), None) => {
                state.selected = Some(asked);
                self.freed.notify_all();
                Ok(asked)
            }
            (Some(asked), Some(active)) if asked == active => Ok(active),
            (Some(_), Some(active)) => Err(WidthActive(active)),
        }
    }

    /// The explicit width this process selected, if any.
    #[cfg(test)]
    pub(crate) fn selected(&self) -> Option<Width> {
        self.lock().selected
    }

    /// Attempts holding a permit now.
    #[cfg(test)]
    pub(crate) fn active(&self) -> usize {
        self.lock().active
    }

    /// Wait for room under the effective width, observing cancellation every
    /// 50 ms and the deadline when it passes. A stopped waiter holds nothing.
    pub(crate) fn acquire(&self, cancel: &Cancel) -> Result<Permit<'_>, error::Error> {
        let mut state = self.lock();
        let mut observed = false;
        loop {
            let budget = cancel.stop_or_remaining()?;
            let width = state.selected.unwrap_or(Width::FALLBACK).get();
            if state.active < width {
                state.active += 1;
                return Ok(Permit(self));
            }
            if !observed {
                observed = true;
                cancel.observed_block();
            }
            let poll = budget.map_or(CANCEL_POLL, |budget| budget.min(CANCEL_POLL));
            state = self
                .freed
                .wait_timeout(state, poll)
                .map_or_else(|poisoned| poisoned.into_inner().0, |(state, _)| state);
        }
    }

    fn lock(&self) -> MutexGuard<'_, WidthState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Room for one live attempt, given back when dropped.
#[derive(Debug)]
pub(crate) struct Permit<'a>(&'a Widths);

impl Drop for Permit<'_> {
    fn drop(&mut self) {
        let mut state = self.0.lock();
        state.active = state.active.saturating_sub(1);
        self.0.freed.notify_all();
    }
}

static PROCESS_WIDTH: Widths = Widths::new();

/// The one door to this process's width state.
pub(crate) fn process_width() -> &'static Widths {
    &PROCESS_WIDTH
}

/// A unit-test binary runs many unrelated tests in one process, so a client
/// there gets its own gate unless a width child test asked for the process one.
#[cfg(test)]
pub(crate) static WIDTH_CHILD: AtomicBool = AtomicBool::new(false);

/// The gate a new client sends through.
pub(crate) fn client_width() -> &'static Widths {
    #[cfg(test)]
    if !WIDTH_CHILD.load(Ordering::Acquire) {
        return Box::leak(Box::default());
    }
    process_width()
}

pub(crate) mod annotate_schedule;
pub(crate) mod cache_lock;
pub(crate) mod cache_prune;
#[cfg(test)]
mod deadline_tests;
pub(crate) mod error;
#[cfg(test)]
#[cfg(unix)]
mod host_signal_tests;
pub(crate) mod http;
pub(crate) mod prepared_request;
pub(crate) mod recorder;
pub(crate) mod request;
pub(crate) mod schedule;
pub(crate) mod usage;
#[cfg(test)]
mod width_tests;
pub(crate) mod workers;
