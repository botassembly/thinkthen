//! Private request execution, recording, locking, and bounded scheduling.

use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::{self, ThreadId};
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
/// optional deadline and host interrupt check of the one call that carries it.
#[derive(Clone, Debug, Default)]
pub(crate) struct Cancel<'a> {
    fired: Arc<AtomicBool>,
    deadline: Option<Deadline>,
    check: Option<Check<'a>>,
    sends: Arc<AtomicUsize>,
    #[cfg(test)]
    blocked: Option<std::sync::mpsc::Sender<()>>,
    #[cfg(test)]
    keys: Arc<AtomicUsize>,
}

/// A host's interrupt check and the one thread that may run it.
#[derive(Clone, Copy)]
struct Check<'a> {
    run: &'a (dyn Fn() -> bool + Sync),
    caller: ThreadId,
}

impl fmt::Debug for Check<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Check")
            .field("caller", &self.caller)
            .finish_non_exhaustive()
    }
}

/// One blocking send in flight, counted until it drops.
#[derive(Debug)]
pub(crate) struct Sending<'a>(&'a AtomicUsize);

impl Drop for Sending<'_> {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}

impl<'a> Cancel<'a> {
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

    /// Share this stop flag with one call whose host check runs on this thread.
    #[allow(
        dead_code,
        reason = "the command passes no check; ticket 0086 exposes it"
    )]
    pub(crate) fn with_check<'b>(&self, check: &'b (dyn Fn() -> bool + Sync)) -> Cancel<'b>
    where
        'a: 'b,
    {
        Cancel {
            check: Some(Check {
                run: check,
                caller: thread::current().id(),
            }),
            sends: Arc::default(),
            ..self.clone()
        }
    }

    /// Observe cancellation, then the host check on its calling thread, then
    /// the deadline, or return the budget left.
    pub(crate) fn stop_or_remaining(&self) -> Result<Option<Duration>, error::Error> {
        if self.fired() || self.checked() {
            return Err(error::Error::Cancelled);
        }
        self.remaining()
    }

    /// Run the host check on its calling thread. A `true` return fires this
    /// call's stop. A panic fires it too, then resumes unchanged, so every
    /// worker stops before the unwinding scope joins it.
    fn checked(&self) -> bool {
        let Some(check) = self
            .check
            .filter(|check| check.caller == thread::current().id())
        else {
            return false;
        };
        let interrupted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(check.run))
            .unwrap_or_else(|panic| {
                self.fired.store(true, Ordering::Release);
                std::panic::resume_unwind(panic)
            });
        if interrupted {
            self.fired.store(true, Ordering::Release);
        }
        interrupted
    }

    /// Count one blocking send, during which the calling thread runs no check.
    pub(crate) fn sending(&self) -> Sending<'_> {
        self.sends.fetch_add(1, Ordering::AcqRel);
        Sending(&self.sends)
    }

    /// Poll from the calling thread while a worker carries this call's attempt.
    pub(crate) fn poll_between_sends(&self) {
        if self.sends.load(Ordering::Acquire) == 0 {
            let _stop = self.stop();
        }
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
            "throttle {active} is already active for this process; use throttle {active} or drop the throttle argument"
        )
    }
}

/// The width selection and the one attempt gate of one process.
///
/// A forked child replaces this whole value, so it holds the only lock and
/// the only condition variable on the width path.
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

static PROCESS_WIDTH: process::Guarded<&'static Widths> = process::Guarded::empty();

/// How long a caller sleeps while another thread of its process rebuilds.
const REBUILD_POLL: Duration = Duration::from_millis(1);

/// The one door to this process's width state.
pub(crate) fn process_width() -> &'static Widths {
    let Ok(widths) = widths_of(std::process::id(), || {
        thread::sleep(REBUILD_POLL);
        Ok::<(), std::convert::Infallible>(())
    });
    widths
}

/// The width state process `pid` owns, fresh and unselected in a forked child.
pub(crate) fn process_width_of(pid: u32, cancel: &Cancel) -> Result<&'static Widths, error::Error> {
    widths_of(pid, rebuild_wait(cancel))
}

/// Each process leaks one width state, as the static it replaces never dropped.
fn widths_of<E>(pid: u32, wait: impl FnMut() -> Result<(), E>) -> Result<&'static Widths, E> {
    PROCESS_WIDTH
        .current(pid, wait, || Ok(&*Box::leak(Box::new(Widths::new()))))
        .map(|widths| *widths)
}

/// Sleep while another thread of this process rebuilds, and stop with the call.
pub(crate) fn rebuild_wait<'c>(
    cancel: &'c Cancel<'_>,
) -> impl FnMut() -> Result<(), error::Error> + 'c {
    move || cancel.wait(REBUILD_POLL).map_or(Ok(()), Err)
}

/// A unit-test binary runs many unrelated tests in one process, so a client
/// there gets its own gate unless a width child test asked for the process one.
#[cfg(test)]
pub(crate) static WIDTH_CHILD: AtomicBool = AtomicBool::new(false);

/// The gate a new client of `process` sends through.
pub(crate) fn client_width(process: &'static Widths) -> &'static Widths {
    #[cfg(test)]
    if !WIDTH_CHILD.load(Ordering::Acquire) {
        return Box::leak(Box::default());
    }
    process
}

pub(crate) mod annotate_schedule;
pub(crate) mod cache_lock;
pub(crate) mod cache_prune;
#[cfg(test)]
mod deadline_tests;
pub(crate) mod error;
pub(crate) mod facade;
#[cfg(test)]
mod facade_tests;
#[cfg(test)]
#[cfg(unix)]
mod host_signal_tests;
pub(crate) mod http;
pub(crate) mod prepared_request;
pub(crate) mod process;
pub(crate) mod recorder;
pub(crate) mod request;
pub(crate) mod schedule;
pub(crate) mod usage;
#[cfg(test)]
mod width_tests;
pub(crate) mod workers;
