//! Deadlines for test subprocesses, on both sides of the pipe.

use std::time::{Duration, Instant};

pub(crate) mod child;
#[cfg(test)]
mod child_tests;
mod run;
mod wait;

pub(crate) use run::output;
pub(crate) use wait::finish;

/// How long a test child parks for the signal its parent sends.
const SIGNAL_DEADLINE: Duration = Duration::from_secs(30);

/// Park until a signal ends this process, and fail after `SIGNAL_DEADLINE`.
///
/// A parent that stops before it signals no longer leaves this child parked.
pub(crate) fn park_for_signal() -> ! {
    let end = Instant::now() + SIGNAL_DEADLINE;
    while let Some(left) = end.checked_duration_since(Instant::now()) {
        std::thread::park_timeout(left);
    }
    panic!(
        "no signal arrived within {} seconds",
        SIGNAL_DEADLINE.as_secs()
    );
}
