//! Deadlines for test subprocesses, on both sides of the pipe.

#[cfg(unix)]
use std::time::{Duration, Instant};

pub(crate) mod child;
#[cfg(test)]
mod child_tests;
mod run;
mod wait;

pub(crate) use run::output;
pub(crate) use wait::finish;

/// How long a test child parks for the signal its parent sends. Only the
/// Unix signal tests park.
#[cfg(unix)]
const SIGNAL_DEADLINE: Duration = Duration::from_secs(30);

/// Park until a signal ends this process, and fail after `SIGNAL_DEADLINE`.
///
/// A parent that stops before it signals no longer leaves this child parked.
#[cfg(unix)]
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

/// Read the whole `{}` request before replying. A socket closed with unread
/// bytes resets the connection, and on Windows the reset can reach the client
/// before the reply does.
pub(crate) fn whole_request(stream: &mut std::net::TcpStream) {
    let (mut chunk, mut whole) = ([0_u8; 1024], Vec::new());
    while !whole.ends_with(b"\r\n\r\n{}") {
        let read = std::io::Read::read(stream, &mut chunk).expect("request bytes");
        assert!(read > 0, "the whole request arrives");
        whole.extend_from_slice(&chunk[..read]);
    }
}
