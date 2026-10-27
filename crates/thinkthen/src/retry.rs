//! Retry limits and shared cancellation.

use std::hash::BuildHasher;
use std::sync::{Arc, Condvar, Mutex, PoisonError, Weak};
use std::time::{Duration, SystemTime};

use crate::{Error, ErrorKind};

pub(crate) const DEFAULT_RETRIES: u32 = 3;
pub(crate) const DEFAULT_BACKOFF_MS: u64 = 500;
pub(crate) const DEFAULT_TIMEOUT_MS: std::num::NonZeroU64 =
    std::num::NonZeroU64::MIN.saturating_add(119_999);

/// Server hints cannot hold a worker indefinitely.
const RETRY_AFTER_CAP: Duration = Duration::from_secs(60);
const JITTER_CAP: Duration = Duration::from_secs(1);

pub(crate) fn retry_delay(backoff: Duration, retry_after: Option<&str>) -> Duration {
    let hinted = retry_after
        .and_then(|value| {
            value
                .parse::<u64>()
                .ok()
                .map(Duration::from_secs)
                .or_else(|| {
                    httpdate::parse_http_date(value)
                        .ok()
                        .map(|date| date.duration_since(SystemTime::now()).unwrap_or_default())
                })
        })
        .unwrap_or_default()
        .min(RETRY_AFTER_CAP);
    // RandomState seeds from the process random source without a shared worker lock.
    let random = std::collections::hash_map::RandomState::new().hash_one(());
    let range = backoff.max(Duration::from_millis(1)).min(JITTER_CAP);
    let jitter = Duration::from_nanos(random % (range.as_nanos() as u64 + 1));
    backoff.max(hinted).saturating_add(jitter)
}

/// Limits for one request. Each split half gets its own retry budget.
#[derive(Debug, Clone, Copy)]
pub(crate) struct RetryPolicy {
    /// Maximum resends after retryable HTTP statuses.
    pub max_retries: usize,
    /// First status retry delay; subsequent status delays double.
    pub initial_backoff: Duration,
    /// Maximum time for one send and its reply.
    pub attempt_timeout: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_RETRIES as usize,
            initial_backoff: Duration::from_millis(DEFAULT_BACKOFF_MS),
            attempt_timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS.get()),
        }
    }
}

/// A shared signal that stops new sends, closes open sockets and interrupts retry waits.
#[derive(Debug, Clone, Default)]
pub struct Cancel {
    inner: Arc<(Mutex<State>, Condvar)>,
    _parent: Option<Arc<ParentWake>>,
}

#[derive(Debug)]
struct ParentWake(Weak<(Mutex<State>, Condvar)>);
impl CancelWake for ParentWake {
    fn wake_cancel(&self) {
        if let Some(inner) = self.0.upgrade() {
            Cancel {
                inner,
                _parent: None,
            }
            .cancel();
        }
    }
}

#[derive(Debug, Default)]
struct State {
    cancelled: bool,
    listeners: Vec<Weak<dyn CancelWake>>,
}

impl Cancel {
    /// A call owns its cancellation; an external signal forwards only inward.
    pub(crate) fn child(&self) -> Self {
        let mut child = Self::default();
        let listener = Arc::new(ParentWake(Arc::downgrade(&child.inner)));
        self.listen(listener.clone());
        child._parent = Some(listener);
        child
    }
    /// Stop this call, close its open connections and wake retry waits. Repeated calls are harmless.
    pub fn cancel(&self) {
        let (cancelled, wake) = &*self.inner;
        let mut state = cancelled.lock().unwrap_or_else(PoisonError::into_inner);
        state.cancelled = true;
        let listeners: Vec<_> = state
            .listeners
            .drain(..)
            .filter_map(|listener| listener.upgrade())
            .collect();
        drop(state);
        for listener in listeners {
            listener.wake_cancel();
        }
        wake.notify_all();
    }

    pub(crate) fn listen(&self, listener: Arc<dyn CancelWake>) {
        let mut state = self.inner.0.lock().unwrap_or_else(PoisonError::into_inner);
        state
            .listeners
            .retain(|listener| listener.strong_count() > 0);
        if state.cancelled {
            drop(state);
            listener.wake_cancel();
        } else {
            state.listeners.push(Arc::downgrade(&listener));
        }
    }

    /// Whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.inner
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .cancelled
    }

    pub(crate) fn complete(&self, result: Result<(), Error>) -> Result<(), Error> {
        if self.is_cancelled() {
            Err(cancelled())
        } else {
            result
        }
    }

    pub(crate) fn wait(&self, delay: Duration) -> bool {
        let (cancelled, wake) = &*self.inner;
        let guard = cancelled.lock().unwrap_or_else(PoisonError::into_inner);
        let (guard, _) = wake
            .wait_timeout_while(guard, delay, |state| !state.cancelled)
            .unwrap_or_else(PoisonError::into_inner);
        guard.cancelled
    }
}

pub(crate) fn cancelled() -> Error {
    Error::new(ErrorKind::Cancelled, "the call was cancelled")
}

pub(crate) trait CancelWake: std::fmt::Debug + Send + Sync {
    fn wake_cancel(&self);
}
