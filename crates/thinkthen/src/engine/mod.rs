//! Private request execution, recording, locking, and bounded scheduling.

use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::thread::{self, ThreadId};
use std::time::{Duration, Instant};

pub(crate) mod call_facts;
mod send_budget;
pub(crate) use call_facts::CallFacts;
pub(crate) use limits::{Permit, WidthActive, Widths};
pub(crate) use send_budget::estimated_total;

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
/// optional deadline, caller's token and host interrupt check of the one call
/// that carries it.
#[derive(Clone, Debug, Default)]
pub(crate) struct Cancel<'a> {
    fired: Arc<AtomicBool>,
    token: Option<Arc<AtomicBool>>,
    deadline: Option<Deadline>,
    check: Option<Check<'a>>,
    sends: Arc<AtomicUsize>,
    sent_any: Arc<AtomicBool>,
    send_budget: Option<(crate::engine::budget::SendBudget, Option<u64>)>,
    process_budget: Option<crate::engine::send_budget::ProcessBudget>,
    /// The call's own cap on the process request total.
    call_total: Option<u64>,
    facts: Option<CallFacts>,
    attempts: Arc<AtomicU64>,
    attempt_sink: Option<AttemptSink>,
    attempt_digest: Option<Arc<str>>,
    #[cfg(test)]
    blocked: Option<std::sync::mpsc::Sender<()>>,
    #[cfg(test)]
    keys: Arc<AtomicUsize>,
}

/// Private transport-only handoff. Its closure never invokes a host callback.
#[derive(Clone)]
pub(crate) struct AttemptSink(Arc<dyn Fn(crate::core::AttemptObservation) + Send + Sync>);

impl fmt::Debug for AttemptSink {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AttemptSink(<withheld>)")
    }
}

impl AttemptSink {
    pub(crate) fn new(
        send: impl Fn(crate::core::AttemptObservation) + Send + Sync + 'static,
    ) -> Self {
        Self(Arc::new(send))
    }
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
    pub(crate) fn with_attempt_sink(&self, sink: AttemptSink) -> Self {
        Self {
            attempt_sink: Some(sink),
            ..self.clone()
        }
    }

    pub(crate) fn with_attempt_digest(&self, digest: &str) -> Self {
        if self.attempt_sink.is_none() {
            return self.clone();
        }
        Self {
            attempt_digest: Some(Arc::from(digest)),
            ..self.clone()
        }
    }

    /// Allocate beside the durable send mark, before transport begins.
    pub(crate) fn attempt_started(&self) -> Option<u64> {
        self.attempt_sink.as_ref()?;
        self.attempt_digest.as_ref()?;
        Some(self.attempts.fetch_add(1, Ordering::Relaxed) + 1)
    }

    /// Hand an owned event to the private sink after transport and body read.
    pub(crate) fn attempt_completed(&self, observation: crate::core::AttemptObservation) {
        if let Some(sink) = &self.attempt_sink {
            (sink.0)(observation);
        }
    }

    pub(crate) fn attempt_digest(&self) -> Option<&str> {
        self.attempt_digest.as_deref()
    }
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

    /// Share this stop flag with one call that a caller's token also stops.
    pub(crate) fn with_token(&self, token: Option<Arc<AtomicBool>>) -> Self {
        Self {
            token,
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
        let token = self
            .token
            .as_ref()
            .is_some_and(|token| token.load(Ordering::Acquire));
        if self.fired() || token || self.checked() {
            return Err(error::Error::Cancelled);
        }
        self.remaining()
    }

    /// Final send check while the usage guard is held. Never calls a host callback.
    pub(crate) fn remaining_without_check(&self) -> Result<Option<Duration>, error::Error> {
        let token = self
            .token
            .as_ref()
            .is_some_and(|token| token.load(Ordering::Acquire));
        if self.fired() || token {
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
        let interrupted = workers::with_host_diagnostics(|| {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(check.run))
        })
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

    /// The stop this checkpoint observes, from the calling thread. The host
    /// check runs only while no worker is in a blocking send.
    pub(crate) fn stop_between_sends(&self) -> Option<error::Error> {
        if self.sends.load(Ordering::Acquire) == 0 {
            self.stop()
        } else {
            self.remaining_without_check().err()
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

    /// The widest throttle, which also sizes the connection pool.
    pub(crate) const MOST: Self = Self(32);

    /// Accept 1 through 32. Width 0 would block every caller forever.
    pub(crate) fn new(value: u64) -> Result<Self, error::Error> {
        u8::try_from(value)
            .ok()
            .filter(|width| (1..=Self::MOST.0).contains(width))
            .map(Self)
            .ok_or(error::Error::Usage(
                "a width is a whole number from 1 through 32",
            ))
    }

    pub(crate) fn get(self) -> usize {
        usize::from(self.0)
    }
}

pub(crate) mod backoff;
pub(crate) mod budget;
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
pub(crate) mod limits;
pub(crate) mod pipeline;
pub(crate) mod process;
pub(crate) mod roots;
pub(crate) mod store;
pub(crate) mod usage;
#[cfg(test)]
mod width_tests;
pub(crate) mod workers;
