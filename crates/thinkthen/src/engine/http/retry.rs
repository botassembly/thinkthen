//! Existing bounded HTTP retry waits.

use std::time::Duration;

pub(super) const MAX_RETRY_WAIT: Duration = Duration::from_secs(60);
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
pub(super) fn bounded_wait(
    asked: Option<Duration>,
    exponential: Duration,
    timeout: Duration,
) -> Duration {
    asked.unwrap_or_else(|| exponential.min(timeout).min(MAX_RETRY_WAIT))
}
