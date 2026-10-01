//! A channel whose waits survive `fork()` on macOS (ticket 0365).
//!
//! On macOS, a wait on a std channel parks the thread on a libdispatch
//! semaphore that a forked child cannot use. A host that asks, forks, and
//! asks again in the child crashed there. This channel waits on a `Mutex`
//! and a `Condvar`, which are pthread calls on macOS and survive the fork.
//! Every wait that can block a host's calling thread goes through it.

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

pub use std::sync::mpsc::{RecvError, RecvTimeoutError, SendError};

struct State<T> {
    queue: VecDeque<T>,
    senders: usize,
    receiving: bool,
}

struct Shared<T> {
    state: Mutex<State<T>>,
    changed: Condvar,
}

impl<T> Shared<T> {
    fn lock(&self) -> MutexGuard<'_, State<T>> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Make a channel. Sends never block; the receiver waits.
#[must_use]
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        state: Mutex::new(State {
            queue: VecDeque::new(),
            senders: 1,
            receiving: true,
        }),
        changed: Condvar::new(),
    });
    (Sender(Arc::clone(&shared)), Receiver(shared))
}

/// The sending side. Clone it for each sender.
pub struct Sender<T>(Arc<Shared<T>>);

impl<T> Sender<T> {
    /// Queue `value` for the receiver.
    ///
    /// # Errors
    ///
    /// Returns the value when the receiver is gone.
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        let mut state = self.0.lock();
        if !state.receiving {
            return Err(SendError(value));
        }
        state.queue.push_back(value);
        drop(state);
        self.0.changed.notify_one();
        Ok(())
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.0.lock().senders += 1;
        Self(Arc::clone(&self.0))
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut state = self.0.lock();
        state.senders -= 1;
        if state.senders == 0 {
            drop(state);
            self.0.changed.notify_all();
        }
    }
}

/// The receiving side. Threads may share it by reference.
pub struct Receiver<T>(Arc<Shared<T>>);

impl<T> Receiver<T> {
    /// Wait for the next value.
    ///
    /// # Errors
    ///
    /// Returns [`RecvError`] once the queue is empty and every sender is gone.
    pub fn recv(&self) -> Result<T, RecvError> {
        let mut state = self
            .0
            .changed
            .wait_while(self.0.lock(), |state| {
                state.queue.is_empty() && state.senders > 0
            })
            .unwrap_or_else(PoisonError::into_inner);
        state.queue.pop_front().ok_or(RecvError)
    }

    /// Wait up to `timeout` for the next value.
    ///
    /// # Errors
    ///
    /// Returns [`RecvTimeoutError::Timeout`] when no value came in time, and
    /// [`RecvTimeoutError::Disconnected`] once the queue is empty and every
    /// sender is gone.
    pub fn recv_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError> {
        let (mut state, _) = self
            .0
            .changed
            .wait_timeout_while(self.0.lock(), timeout, |state| {
                state.queue.is_empty() && state.senders > 0
            })
            .unwrap_or_else(PoisonError::into_inner);
        match state.queue.pop_front() {
            Some(value) => Ok(value),
            None if state.senders == 0 => Err(RecvTimeoutError::Disconnected),
            None => Err(RecvTimeoutError::Timeout),
        }
    }
}

impl<T> Drop for Receiver<T> {
    /// Refuse later sends and drop what was queued, outside the lock.
    fn drop(&mut self) {
        let mut state = self.0.lock();
        state.receiving = false;
        let queued = std::mem::take(&mut state.queue);
        drop(state);
        drop(queued);
    }
}

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sender").finish_non_exhaustive()
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Receiver").finish_non_exhaustive()
    }
}
