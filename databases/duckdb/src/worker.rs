//! Every engine call runs on a detachable worker (ticket 0110 decision 7).
//!
//! The worker owns its inputs and an internal cancel token. The DuckDB
//! thread waits on a channel in 50 ms ticks and reads the interrupt
//! predicate on each tick. On a stop it cancels the token, detaches the
//! worker, and raises `thinkthen cancelled: ` at once. The detached worker
//! finishes the sends already started and starts no retry.

use std::sync::mpsc::{RecvTimeoutError, channel};
use std::thread;
use std::time::Duration;

use thinkthen::{CancelToken, Error};

use crate::errors::{defect, failure, prefix};
use crate::signal::Invoke;

/// How often the waiting thread reads the interrupt predicate.
const TICK: Duration = Duration::from_millis(50);

/// The sentence a stopped call raises.
const CANCELLED: &str = "the call was cancelled";

/// Run `work` on a worker and wait for it under `invoke`'s predicate.
pub(crate) fn run<T: Send + 'static>(
    invoke: &Invoke,
    work: impl FnOnce(&CancelToken) -> Result<T, Error> + Send + 'static,
) -> Result<T, String> {
    let token = CancelToken::new();
    let (answer, answered) = channel();
    let owned = token.clone();
    thread::Builder::new()
        .name("thinkthen-duckdb-call".to_owned())
        .spawn(move || {
            let _ = answer.send(work(&owned));
        })
        .map_err(|_| defect("the engine worker could not start"))?;
    wait(|| answered.recv_timeout(TICK), || invoke.stopped(), &token)
}

/// Wait for one answer, reading `stopped` at every tick.
fn wait<T>(
    mut next: impl FnMut() -> Result<Result<T, Error>, RecvTimeoutError>,
    stopped: impl Fn() -> bool,
    token: &CancelToken,
) -> Result<T, String> {
    loop {
        if stopped() {
            token.cancel();
            return Err(format!(
                "{}{CANCELLED}",
                prefix(thinkthen::ErrorKind::Cancelled)
            ));
        }
        match next() {
            Ok(answer) => return answer.map_err(|error| failure(&error)),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return Err(defect("the engine worker ended with no answer"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A worker that dies before it answers reads `defect`, never a hang.
    #[test]
    fn a_closed_channel_reads_defect() {
        let token = CancelToken::new();
        let answered: Result<(), String> =
            wait(|| Err(RecvTimeoutError::Disconnected), || false, &token);
        assert_eq!(
            answered,
            Err("thinkthen defect: the engine worker ended with no answer".to_owned())
        );
    }

    /// A stop cancels the worker's token and answers at once.
    #[test]
    fn a_stop_cancels_the_token() {
        let token = CancelToken::new();
        let answered: Result<(), String> = wait(|| Err(RecvTimeoutError::Timeout), || true, &token);
        assert_eq!(
            answered,
            Err("thinkthen cancelled: the call was cancelled".to_owned())
        );
        assert!(token.is_cancelled());
    }
}
