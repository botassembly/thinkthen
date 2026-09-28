//! A bounded finalization wait for another process's advisory usage lock.

use std::fs::File;
use std::io::{self, ErrorKind};
use std::thread;
use std::time::{Duration, Instant};

use super::Shared;

const POLL: Duration = Duration::from_millis(10);
const FINISH_WAIT: Duration = Duration::from_secs(1);

pub(super) fn deadline() -> Instant {
    Instant::now() + FINISH_WAIT
}

/// Try without waiting in the OS. The writer can observe the one deadline
/// that finish or drop published while another process owns this file.
pub(super) fn acquire(file: &File, shared: &Shared) -> io::Result<()> {
    loop {
        match file.try_lock() {
            Ok(()) => return Ok(()),
            Err(std::fs::TryLockError::WouldBlock) => {
                let finish_deadline = shared
                    .queue
                    .lock()
                    .map_err(|_| io::Error::other("usage counter lock poisoned"))?
                    .finish_deadline;
                let pause = match finish_deadline {
                    Some(end) if Instant::now() >= end => {
                        return Err(io::Error::new(ErrorKind::TimedOut, "usage lock busy"));
                    }
                    Some(end) => POLL.min(end.saturating_duration_since(Instant::now())),
                    None => POLL,
                };
                thread::sleep(pause);
            }
            Err(error) => return Err(error.into()),
        }
    }
}
