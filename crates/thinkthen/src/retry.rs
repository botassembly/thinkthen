//! Retry limits and shared cancellation.

use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Condvar, Mutex, PoisonError, Weak};
use std::time::Duration;

use crate::{Error, ErrorKind};

/// Limits for one request. Each split half gets its own retry budget.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
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
            max_retries: 3,
            initial_backoff: Duration::from_millis(500),
            attempt_timeout: Duration::from_secs(120),
        }
    }
}

/// A shared signal that stops new sends, closes open sockets and interrupts retry waits.
#[derive(Debug, Clone, Default)]
pub struct Cancel(Arc<(Mutex<State>, Condvar)>);

#[derive(Debug, Default)]
struct State {
    cancelled: bool,
    sockets: Vec<Weak<TcpStream>>,
}

impl Cancel {
    /// Stop this call, close its open connections and wake retry waits. Repeated calls are harmless.
    pub fn cancel(&self) {
        let (cancelled, wake) = &*self.0;
        let mut state = cancelled.lock().unwrap_or_else(PoisonError::into_inner);
        state.cancelled = true;
        for socket in state
            .sockets
            .drain(..)
            .filter_map(|socket| socket.upgrade())
        {
            // A peer may already have closed. Either outcome leaves no open exchange.
            let _ = socket.shutdown(Shutdown::Both);
        }
        wake.notify_all();
    }

    /// Whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0
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

    pub(crate) fn register(&self, socket: &Arc<TcpStream>) -> std::io::Result<()> {
        let mut state = self.0.0.lock().unwrap_or_else(PoisonError::into_inner);
        if state.cancelled {
            socket.shutdown(Shutdown::Both)?;
        } else {
            state.sockets.retain(|socket| socket.strong_count() > 0);
            state.sockets.push(Arc::downgrade(socket));
        }
        Ok(())
    }

    pub(crate) fn wait(&self, delay: Duration) -> bool {
        let (cancelled, wake) = &*self.0;
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
