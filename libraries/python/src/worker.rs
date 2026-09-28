//! The detachable worker every call runs on (decision 6, amendment change 2).
//!
//! The worker owns its inputs and never touches Python. The calling thread
//! releases the interpreter and waits in 50 ms ticks. On each tick it reads
//! the caller's token and runs the interpreter's signal handlers. On a stop
//! it cancels the worker's own token, leaves the worker behind, and raises at
//! once. The engine sends nothing new under a cancelled token, and a request
//! already sent ends on its own. An interrupt never fires the caller's token.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, sync_channel};
use std::time::Duration;

use crate::diagnostics::host;
use pyo3::exceptions::PyKeyboardInterrupt;
use pyo3::prelude::*;
use thinkthen::{CallOptions, CancelToken, Error, ErrorKind};

use crate::{caught, defect, raise, raised};

/// How often the calling thread checks the caller's token and signals.
const TICK: Duration = Duration::from_millis(50);

/// Workers that have not yet finished, for the `probe` hook `_live_workers`.
static LIVE: AtomicUsize = AtomicUsize::new(0);

const CANCELLED: &str = "the call was cancelled";
const INTERRUPTED: &str =
    "the call was interrupted; no new request starts, and sent requests end on their own";

/// A cancel flag any thread may set. Every call handed it stops, and an
/// interrupt never sets it.
#[pyclass(frozen, name = "CancelToken", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Token(pub(crate) CancelToken);

#[pymethods]
impl Token {
    #[new]
    fn new() -> Self {
        Self(CancelToken::new())
    }

    /// Stop every call that holds this token.
    fn cancel(&self) {
        self.0.cancel();
    }

    /// Whether `cancel` has run.
    #[getter]
    fn cancelled(&self) -> bool {
        self.0.is_cancelled()
    }

    fn __repr__(&self) -> String {
        format!(
            "CancelToken(cancelled={})",
            if self.0.is_cancelled() {
                "True"
            } else {
                "False"
            }
        )
    }
}

/// The caller's controls for one call: its token and its deadline in seconds.
#[derive(Debug, Default)]
pub(crate) struct Controls {
    pub(crate) token: Option<CancelToken>,
    pub(crate) deadline: Option<f64>,
}

/// What a worker sends back: its result, or `None` when it panicked.
type Outcome<T> = Option<Result<T, Error>>;

/// Run one engine call on a detachable worker and wait for it here.
pub(crate) fn run<T, F>(py: Python<'_>, controls: Controls, job: F) -> PyResult<T>
where
    T: Send + 'static,
    F: FnOnce(CallOptions<'_>) -> Result<T, Error> + Send + 'static,
{
    let caller = controls.token.clone();
    if caller.as_ref().is_some_and(CancelToken::is_cancelled) {
        return Err(raise(py, ErrorKind::Cancelled, CANCELLED, false));
    }
    let internal = CancelToken::new();
    let stop = internal.clone();
    let (sender, receiver) = sync_channel(1);
    LIVE.fetch_add(1, Ordering::SeqCst);
    let spawned = std::thread::Builder::new()
        .name("thinkthen-call".to_owned())
        .spawn(move || {
            let outcome = caught(|| work(&stop, controls, job));
            // The caller may have left. A closed channel is not an error.
            let _left = sender.send(outcome);
            LIVE.fetch_sub(1, Ordering::SeqCst);
        });
    if spawned.is_err() {
        LIVE.fetch_sub(1, Ordering::SeqCst);
        return Err(defect(py, "a call thread could not start"));
    }
    wait(py, receiver, &internal, caller.as_ref())
}

/// The worker's side: the call's options, then the call.
fn work<T>(
    stop: &CancelToken,
    controls: Controls,
    job: impl FnOnce(CallOptions<'_>) -> Result<T, Error>,
) -> Result<T, Error> {
    let caller = controls.token;
    let check = move || caller.as_ref().is_some_and(CancelToken::is_cancelled);
    let options = CallOptions::new().cancel(stop).interrupt(&check);
    let options = match controls.deadline {
        Some(seconds) => options.deadline_seconds(seconds)?,
        None => options,
    };
    job(options)
}

/// The calling thread's side: wait in ticks, and stop on the caller's token
/// or a signal.
fn wait<T: Send>(
    py: Python<'_>,
    receiver: Receiver<Outcome<T>>,
    internal: &CancelToken,
    caller: Option<&CancelToken>,
) -> PyResult<T> {
    let mut receiver = receiver;
    loop {
        // `Receiver` is not `Sync`, so it moves into the detached closure and back.
        let (waited, back) = py.detach(move || (receiver.recv_timeout(TICK), receiver));
        receiver = back;
        if caller.is_some_and(CancelToken::is_cancelled) {
            internal.cancel();
            return Err(raise(py, ErrorKind::Cancelled, CANCELLED, false));
        }
        match waited {
            Ok(Some(Ok(value))) => return Ok(value),
            Ok(Some(Err(error))) => return Err(raised(py, &error)),
            Ok(None) => return Err(defect(py, "the call panicked")),
            Err(RecvTimeoutError::Disconnected) => {
                return Err(defect(py, "the call ended with no result"));
            }
            Err(RecvTimeoutError::Timeout) => {}
        }
        if let Err(signal) = host(|| py.check_signals()) {
            internal.cancel();
            return Err(if signal.is_instance_of::<PyKeyboardInterrupt>(py) {
                raise(py, ErrorKind::Cancelled, INTERRUPTED, false)
            } else {
                signal
            });
        }
    }
}

/// The workers still running. Built only under the test feature `probe`.
#[cfg(feature = "probe")]
#[pyfunction]
pub(crate) fn _live_workers() -> usize {
    LIVE.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use std::{sync::atomic::Ordering, sync::mpsc::sync_channel, time::Duration, time::Instant};

    use pyo3::prelude::*;
    use thinkthen::CancelToken;

    use super::{Controls, LIVE, Outcome, run, wait};

    /// A worker whose caller has left finishes its call and ends without a
    /// panic. Only this test starts a worker, so the count is its own.
    #[test]
    fn a_worker_outlives_a_caller_that_left() {
        Python::initialize();
        let token = CancelToken::new();
        let stopper = token.clone();
        let controls = Controls {
            token: Some(token),
            deadline: None,
        };
        let left = Python::attach(|py| {
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(100));
                stopper.cancel();
            });
            let raised = run(py, controls, |_| {
                std::thread::sleep(Duration::from_millis(400));
                Ok(3_u8)
            });
            raised.err().map(|error| error.value(py).to_string())
        });
        assert_eq!(left.as_deref(), Some("the call was cancelled"));
        assert_eq!(LIVE.load(Ordering::SeqCst), 1, "the worker still runs");
        let ended = Instant::now();
        while LIVE.load(Ordering::SeqCst) > 0 && ended.elapsed() < Duration::from_secs(2) {
            std::thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(LIVE.load(Ordering::SeqCst), 0, "the worker ended");
    }

    /// A channel that closes with no result raises `DefectError`.
    #[test]
    fn a_closed_channel_with_no_result_is_a_defect() {
        Python::initialize();
        let (sender, receiver) = sync_channel::<Outcome<u8>>(1);
        drop(sender);
        Python::attach(|py| {
            let error = wait(py, receiver, &CancelToken::new(), None).err();
            assert_eq!(
                error.map(|error| error.value(py).to_string()).as_deref(),
                Some("defect: the call ended with no result")
            );
        });
    }

    /// A fired caller token wins when the answer already waits in the channel.
    #[test]
    fn a_fired_token_beats_an_answer_already_waiting() {
        Python::initialize();
        let (sender, receiver) = sync_channel::<Outcome<u8>>(1);
        assert!(sender.send(Some(Ok(3))).is_ok());
        let caller = CancelToken::new();
        caller.cancel();
        Python::attach(|py| {
            let error = wait(py, receiver, &CancelToken::new(), Some(&caller)).err();
            assert_eq!(
                error.map(|error| error.value(py).to_string()).as_deref(),
                Some("the call was cancelled")
            );
        });
    }
}
