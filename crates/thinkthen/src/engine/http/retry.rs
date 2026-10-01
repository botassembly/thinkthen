//! Bounded HTTP retry waits.

use std::hash::{BuildHasher as _, RandomState};
use std::time::Duration;

use crate::engine::error::Error;

const MAX_RETRY_WAIT: Duration = Duration::from_secs(60);
const MIN_HEADER_WAIT: Duration = Duration::from_secs(1);

/// The wait the two retry headers ask for, with zero given a one-second floor.
///
/// The backend sends `retry-after-ms` in whole milliseconds beside the standard
/// `retry-after` in whole seconds, and the finer one is read first.
/// `specification/backends.md` takes those two forms alone. The HTTP-date form
/// of `retry-after` needs a clock and a date reader, and neither belongs here.
pub(super) fn honored(millis: Option<&str>, seconds: Option<&str>) -> Option<Duration> {
    let asked = |header: Option<&str>| header?.trim().parse::<u64>().ok();
    let wait = match asked(millis) {
        Some(number) => Duration::from_millis(number),
        None => Duration::from_secs(asked(seconds)?),
    };
    Some(if wait.is_zero() {
        MIN_HEADER_WAIT
    } else {
        wait
    })
}

/// The server's valid delay is a floor; cap only an unheaded local wait.
///
/// Each unheaded wait is drawn at random, by `draw`, between half and all of
/// its capped doubling, so separate processes do not resend together; within
/// one process the URL's gate still opens once for all waiting requests.
pub(super) fn bounded_wait(
    asked: Option<Duration>,
    exponential: Duration,
    timeout: Duration,
    draw: u64,
) -> Duration {
    asked.unwrap_or_else(|| {
        let full = exponential.min(longest(timeout));
        let half = full / 2;
        let span = (full - half).as_nanos();
        let share = span * u128::from(draw) / u128::from(u64::MAX);
        half + Duration::from_nanos(u64::try_from(share).unwrap_or(u64::MAX))
    })
}

/// The longest retry wait: the attempt timeout, at most 60 seconds.
pub(super) fn longest(timeout: Duration) -> Duration {
    timeout.min(MAX_RETRY_WAIT)
}

/// A server wait past the longest retry wait ends the retries, so no request
/// waits without a fixed bound (ticket 0367).
pub(super) fn too_long(asked: Option<Duration>, timeout: Duration) -> bool {
    asked.is_some_and(|asked| asked > longest(timeout))
}

/// The status a gate's server floor carries: a status that asked a wait.
pub(super) fn floor(asked: Option<Duration>, failure: &Error) -> Option<u16> {
    match failure {
        Error::Status(status) => asked.map(|_| *status),
        _ => None,
    }
}

/// One random draw from the hasher keys the standard library seeds per thread.
pub(super) fn draw() -> u64 {
    RandomState::new().hash_one(())
}

#[cfg(test)]
#[path = "retry_tests.rs"]
mod tests;
