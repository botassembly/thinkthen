//! A bounded finalization wait for another process's advisory usage lock.

use std::fs::{File, TryLockError};
use std::io::{self, ErrorKind};
use std::time::{Duration, Instant};

use super::Shared;

const FINISH_WAIT: Duration = Duration::from_secs(1);
const FIRST_PAUSE: Duration = Duration::from_millis(1);
const LONGEST_PAUSE: Duration = Duration::from_millis(100);

fn poisoned<T>(_: T) -> io::Error {
    io::Error::other("usage counter lock poisoned")
}

pub(super) fn deadline() -> Instant {
    Instant::now() + FINISH_WAIT
}

/// The standard library's blocking lock takes no timeout, so retry the
/// nonblocking one with a doubling pause. The pause waits on the queue's
/// condition variable, so the finish deadline wakes it at once.
pub(super) fn acquire(file: &File, shared: &Shared) -> io::Result<()> {
    let mut pause = FIRST_PAUSE;
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(()),
            Err(TryLockError::WouldBlock) => {}
            Err(TryLockError::Error(error)) => return Err(error),
        }
        let queue = shared.queue.lock().map_err(poisoned)?;
        let wait = match queue.finish_deadline {
            Some(end) => match end.checked_duration_since(Instant::now()) {
                Some(left) if !left.is_zero() => pause.min(left),
                _ => return Err(io::Error::new(ErrorKind::TimedOut, "usage lock busy")),
            },
            None => pause,
        };
        drop(shared.changed.wait_timeout(queue, wait).map_err(poisoned)?);
        pause = pause.saturating_mul(2).min(LONGEST_PAUSE);
    }
}

/// Take the shared lock a reader needs, waiting at most one second. A lock
/// still held then gives a timed-out error the caller names as busy.
pub(super) fn shared(file: &File) -> io::Result<()> {
    let end = deadline();
    let mut pause = Duration::from_millis(10);
    loop {
        match file.try_lock_shared() {
            Ok(()) => return Ok(()),
            Err(TryLockError::WouldBlock) => {}
            Err(TryLockError::Error(error)) => return Err(error),
        }
        let left = end.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(io::Error::new(ErrorKind::TimedOut, "usage lock busy"));
        }
        std::thread::sleep(pause.min(left));
        pause = pause.saturating_mul(2).min(LONGEST_PAUSE);
    }
}
