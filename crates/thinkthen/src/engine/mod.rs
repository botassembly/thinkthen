//! Private request execution, recording, locking, and bounded scheduling.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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

pub(crate) mod annotate_schedule;
pub(crate) mod cache_lock;
pub(crate) mod cache_prune;
#[cfg(test)]
mod deadline_tests;
pub(crate) mod error;
pub(crate) mod http;
pub(crate) mod prepared_request;
pub(crate) mod recorder;
pub(crate) mod request;
pub(crate) mod schedule;
pub(crate) mod usage;
pub(crate) mod workers;
