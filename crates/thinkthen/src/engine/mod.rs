//! Private request execution, recording, locking, and bounded scheduling.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

const CANCEL_POLL: Duration = Duration::from_millis(50);

/// One private cooperative stop flag shared by a whole engine run.
#[derive(Clone, Debug, Default)]
pub(crate) struct Cancel {
    fired: Arc<AtomicBool>,
    #[cfg(test)]
    blocked: Option<std::sync::mpsc::Sender<()>>,
}

impl Cancel {
    #[allow(
        dead_code,
        reason = "the command token stays unfired until the separately authorized signal ticket"
    )]
    pub(crate) fn fire(&self) {
        self.fired.store(true, Ordering::Release);
    }

    pub(crate) fn fired(&self) -> bool {
        self.fired.load(Ordering::Acquire)
    }

    #[cfg(test)]
    pub(crate) fn observed(blocked: std::sync::mpsc::Sender<()>) -> Self {
        Self {
            blocked: Some(blocked),
            ..Self::default()
        }
    }

    pub(crate) fn observed_block(&self) {
        #[cfg(test)]
        if let Some(blocked) = &self.blocked {
            let _observed = blocked.send(());
        }
    }

    /// Wait for a bounded operation while observing cancellation every 50 ms.
    pub(crate) fn wait(&self, duration: Duration) -> bool {
        let started = Instant::now();
        loop {
            if self.fired() {
                return true;
            }
            let remaining = duration.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                return false;
            }
            thread::sleep(remaining.min(CANCEL_POLL));
        }
    }

    pub(crate) const fn poll() -> Duration {
        CANCEL_POLL
    }
}

pub(crate) mod annotate_schedule;
pub(crate) mod cache_lock;
pub(crate) mod cache_prune;
pub(crate) mod error;
pub(crate) mod http;
pub(crate) mod prepared_request;
pub(crate) mod recorder;
pub(crate) mod request;
pub(crate) mod schedule;
pub(crate) mod usage;
pub(crate) mod workers;
