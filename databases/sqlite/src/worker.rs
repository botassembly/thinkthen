//! Every call that can send runs on a detachable worker (ticket 0109
//! decisions 6 and 7).
//!
//! The engine's interrupt check does not run during one blocking send, so a
//! call on the calling thread would hold Ctrl-C for up to the request
//! timeout. The worker owns its inputs and never touches SQLite. The calling
//! thread waits on a channel and reads `sqlite3_is_interrupted` on each 50 ms
//! tick. On an interrupt it fires the worker's token and returns at once.
//! The detached worker lets its sent requests finish, starts no new one, and
//! then ends. That worker is the known exception to ADR 0017's "no thread
//! outlives a call".

use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use rusqlite::ffi::sqlite3;
use thinkthen::{CallOptions, CancelToken, Engine, ErrorKind};

use crate::{Failure, ffi, guard, settings};

/// How often the calling thread reads the interrupt flag.
const TICK: Duration = Duration::from_millis(50);

/// Run `work` on a detached worker over the process engine, under the call's
/// deadline in milliseconds, while this thread listens on `db` for SQLite's
/// interrupt.
pub(crate) fn run<T: Send + 'static>(
    db: *mut sqlite3,
    deadline: Option<i64>,
    work: impl FnOnce(&'static Engine, CallOptions<'_>) -> Result<T, Failure> + Send + 'static,
) -> Result<T, Failure> {
    run_many(db, deadline, 1, work)
}

/// As [`run`], for a call over `records` records, which the process
/// request total may limit.
pub(crate) fn run_many<T: Send + 'static>(
    db: *mut sqlite3,
    deadline: Option<i64>,
    records: usize,
    work: impl FnOnce(&'static Engine, CallOptions<'_>) -> Result<T, Failure> + Send + 'static,
) -> Result<T, Failure> {
    let engine = settings::engine_for(records)?;
    let token = CancelToken::new();
    let theirs = token.clone();
    let (answers, _detached) = spawn(move || {
        let mut options = CallOptions::new().cancel(&theirs);
        if let Some(millis) = deadline {
            options = options.deadline_millis(millis)?;
        }
        work(engine, options)
    })?;
    wait(&answers, &token, || ffi::interrupted(db))
}

/// A worker's answers and its handle, which the caller drops to detach it.
type Worker<T> = (Receiver<Result<T, Failure>>, JoinHandle<()>);

/// Start one worker that sends its guarded result, or nothing if the caller left.
fn spawn<T: Send + 'static>(
    work: impl FnOnce() -> Result<T, Failure> + Send + 'static,
) -> Result<Worker<T>, Failure> {
    let (send, answers) = channel();
    let handle = thread::Builder::new()
        .name("thinkthen-call".to_owned())
        .spawn(move || {
            // A caller that returned on an interrupt closed the channel.
            let _ignored = send.send(guard("the call's worker", work));
        })
        .map_err(|error| Failure::defect(format!("the call's worker did not start: {error}")))?;
    Ok((answers, handle))
}

/// Wait for the worker's result. On an interrupt, fire the token and leave.
fn wait<T>(
    answers: &Receiver<Result<T, Failure>>,
    token: &CancelToken,
    interrupted: impl Fn() -> bool,
) -> Result<T, Failure> {
    loop {
        match answers.recv_timeout(TICK) {
            Ok(result) => return result,
            Err(RecvTimeoutError::Timeout) if interrupted() => {
                token.cancel();
                return Err(Failure::of(ErrorKind::Cancelled, "the call was cancelled"));
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return Err(Failure::defect("the call's worker ended with no result"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    use thinkthen::{CancelToken, ErrorKind};

    use super::{spawn, wait};
    use crate::Failure;

    /// A worker that answers after its caller left sends into a closed
    /// channel. It must end quietly, and the caller must hear `cancelled`.
    #[test]
    fn a_worker_whose_caller_left_ends_without_a_panic() {
        let token = CancelToken::new();
        let seen = token.clone();
        let (answers, handle) = spawn(move || {
            while !seen.is_cancelled() {
                thread::sleep(Duration::from_millis(5));
            }
            // Answer only once the caller has dropped its end.
            thread::sleep(Duration::from_millis(200));
            Ok(1)
        })
        .map_err(|failure| failure.message)
        .unwrap();
        let kind = wait(&answers, &token, || true)
            .err()
            .map(|failure| failure.kind);
        assert_eq!(kind, Some(ErrorKind::Cancelled));
        drop(answers);
        assert!(
            handle.join().is_ok(),
            "the worker panicked on the closed channel"
        );
    }

    /// A channel that closes with no result is `defect`.
    #[test]
    fn a_channel_closed_with_no_result_is_a_defect() {
        let (send, answers) = channel::<Result<(), Failure>>();
        drop(send);
        let failure = wait(&answers, &CancelToken::new(), || false).err();
        assert_eq!(
            failure.map(|failure| (failure.kind, failure.message)),
            Some((
                ErrorKind::Defect,
                "the call's worker ended with no result".to_owned()
            ))
        );
    }
}
