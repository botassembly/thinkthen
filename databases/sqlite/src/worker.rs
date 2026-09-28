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
use std::time::{Duration, Instant};

use rusqlite::ffi::sqlite3;
use thinkthen::{CallOptions, CancelToken, Engine, ErrorKind};

use crate::{Failure, budget, ffi, guard, settings};

/// How often the calling thread reads the interrupt flag.
const TICK: Duration = Duration::from_millis(50);

/// Run `work` on a detached worker over the process engine, under the call's
/// deadline in milliseconds, while this thread listens on `db` for SQLite's
/// interrupt.
/// A spent process request total refuses first, with no send.
pub(crate) fn run<T: Send + 'static>(
    db: *mut sqlite3,
    deadline: Option<i64>,
    work: impl FnOnce(&'static Engine, CallOptions<'_>) -> Result<T, Failure> + Send + 'static,
) -> Result<T, Failure> {
    let due = budget::remaining(db)?.map(|(_, due)| due);
    let engine = settings::engine()?;
    settings::remaining()?;
    let (send_budget, total) = settings::send_budget();
    let token = CancelToken::new();
    let theirs = token.clone();
    let (answers, _detached) = spawn(move || {
        let mut options = CallOptions::new()
            .cancel(&theirs)
            .send_budget(send_budget, total);
        let left = due
            .map(|due| {
                let left = due.checked_duration_since(Instant::now()).ok_or_else(|| {
                    Failure::of(
                        ErrorKind::Deadline,
                        "the connection's ThinkThen budget passed",
                    )
                })?;
                let millis = i64::try_from(left.as_millis()).unwrap_or(i64::MAX);
                if millis == 0 {
                    return Err(Failure::of(
                        ErrorKind::Deadline,
                        "the connection's ThinkThen budget passed",
                    ));
                }
                Ok(millis)
            })
            .transpose()?;
        let deadline = match (deadline, left) {
            (Some(call), Some(budget)) if call >= 0 => Some(call.min(budget)),
            (_, Some(budget)) => Some(budget),
            (call, None) => call,
        };
        if let Some(millis) = deadline {
            options = options.deadline_millis(millis)?;
        }
        work(engine, options)
    })?;
    wait(&answers, &token, || ffi::interrupted(db), due)
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
    due: Option<Instant>,
) -> Result<T, Failure> {
    loop {
        match answers.recv_timeout(TICK) {
            Ok(result) => {
                if due.is_some_and(|due| Instant::now() >= due) {
                    token.cancel();
                    return Err(Failure::of(
                        ErrorKind::Deadline,
                        "the connection's ThinkThen budget passed",
                    ));
                }
                return result;
            }
            Err(RecvTimeoutError::Timeout) if interrupted() => {
                token.cancel();
                return Err(Failure::of(ErrorKind::Cancelled, "the call was cancelled"));
            }
            Err(RecvTimeoutError::Timeout) if due.is_some_and(|due| Instant::now() >= due) => {
                token.cancel();
                return Err(Failure::of(
                    ErrorKind::Deadline,
                    "the connection's ThinkThen budget passed",
                ));
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
        let kind = wait(&answers, &token, || true, None)
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
        let failure = wait(&answers, &CancelToken::new(), || false, None).err();
        assert_eq!(
            failure.map(|failure| (failure.kind, failure.message)),
            Some((
                ErrorKind::Defect,
                "the call's worker ended with no result".to_owned()
            ))
        );
    }
}
