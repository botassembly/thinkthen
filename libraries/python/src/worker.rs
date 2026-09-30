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
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use crate::diagnostics::{host, host_error};
use pyo3::exceptions::{PyKeyboardInterrupt, PyTimeoutError};
use pyo3::prelude::*;
use pyo3::types::PyBool;
use thinkthen::{
    CallOptions, CancelToken, Error, ErrorKind, Facts, RecordObservation, Tally, contained,
};

mod stream_receipt;
pub(crate) use stream_receipt::{finish_stream_receipt, stream_receipt};

use crate::result::{Completed, Observations, python_details, python_facts};
use crate::{defect, raise, raised};
use serde_json::Value;

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

/// The caller's controls for one call: its token and its deadline in milliseconds.
#[derive(Debug, Default)]
pub(crate) struct Controls {
    pub(crate) token: Option<CancelToken>,
    pub(crate) deadline: Option<i64>,
}

/// What a worker sends back: its result, or `None` when it panicked.
type Outcome<T, E = Error> = Option<Result<T, E>>;

pub(crate) trait WorkerError: From<Error> + Send + 'static {
    fn facts(&self) -> Option<Facts>;
    fn raised(&self, py: Python<'_>) -> PyErr;
    fn failure(&self) -> Failure;
    fn details(&self) -> Option<Vec<Value>> {
        None
    }
    /// Give facts to a refusal that stopped before any engine call. They
    /// span the job and add nothing to the caller's tally.
    fn unaccounted(self, _facts: Facts) -> Self {
        self
    }
}

impl WorkerError for Error {
    fn facts(&self) -> Option<Facts> {
        Error::facts(self).cloned()
    }
    fn raised(&self, py: Python<'_>) -> PyErr {
        raised(py, self)
    }
    fn failure(&self) -> Failure {
        Failure {
            kind: self.kind().name(),
            message: self.to_string(),
            retryable: self.retryable(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Failure {
    pub(crate) kind: &'static str,
    pub(crate) message: String,
    pub(crate) retryable: bool,
}

#[derive(Clone)]
struct Terminal {
    outcome: &'static str,
    facts: Option<Facts>,
    details: Option<Vec<Value>>,
    failure: Option<Failure>,
}

#[derive(Default)]
pub(crate) struct ReceiptState(Mutex<Option<Terminal>>, Condvar);

impl ReceiptState {
    fn finish(&self, result: Terminal) {
        *self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        self.1.notify_all();
    }
}

#[pyclass(frozen, name = "CompletionReceipt", module = "thinkthen._thinkthen")]
pub(crate) struct Receipt(Arc<ReceiptState>);

#[pyclass(frozen, name = "Completion", module = "thinkthen._thinkthen")]
pub(crate) struct Completion {
    #[pyo3(get)]
    outcome: &'static str,
    #[pyo3(get)]
    facts: Option<Py<PyAny>>,
    #[pyo3(get)]
    details: Option<Py<PyAny>>,
    #[pyo3(get)]
    kind: Option<&'static str>,
    #[pyo3(get)]
    message: Option<String>,
    #[pyo3(get)]
    retryable: Option<bool>,
}

#[pymethods]
impl Receipt {
    #[getter]
    fn done(&self) -> bool {
        self.0
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .is_some()
    }

    #[pyo3(signature = (timeout=None))]
    fn result(
        &self,
        py: Python<'_>,
        timeout: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Py<Completion>> {
        let refused = || {
            crate::usage(
                py,
                "completion timeout is a finite number of seconds of 0 or more",
            )
        };
        let due = timeout
            .map(|value| {
                if value.is_instance_of::<PyBool>() {
                    return Err(refused());
                }
                let seconds: f64 = host_error(host(|| value.extract()), refused)?;
                if !seconds.is_finite() || seconds < 0.0 {
                    return Err(refused());
                }
                let duration = Duration::try_from_secs_f64(seconds).map_err(|_| refused())?;
                Instant::now().checked_add(duration).ok_or_else(refused)
            })
            .transpose()?;
        loop {
            if let Some(finished) = self
                .0
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .clone()
            {
                return Py::new(
                    py,
                    Completion {
                        outcome: finished.outcome,
                        facts: finished
                            .facts
                            .as_ref()
                            .map(|facts| python_facts(py, facts))
                            .transpose()?,
                        details: finished
                            .details
                            .as_ref()
                            .map(|details| python_details(py, details))
                            .transpose()?,
                        kind: finished.failure.as_ref().map(|failure| failure.kind),
                        message: finished
                            .failure
                            .as_ref()
                            .map(|failure| failure.message.clone()),
                        retryable: finished.failure.as_ref().map(|failure| failure.retryable),
                    },
                );
            }
            let wait = due.map_or(TICK, |at| {
                at.saturating_duration_since(Instant::now()).min(TICK)
            });
            if wait.is_zero() {
                return Err(PyTimeoutError::new_err("the call has not completed"));
            }
            py.detach(|| {
                let held = self
                    .0
                    .0
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                let _waited = self.0.1.wait_timeout(held, wait);
            });
            py.check_signals()?;
        }
    }
}

pub(crate) fn attach_receipt(py: Python<'_>, error: &PyErr, receipt: Option<&Arc<ReceiptState>>) {
    if let Some(receipt) = receipt
        && let Ok(value) = Py::new(py, Receipt(Arc::clone(receipt)))
    {
        let _set = error.value(py).setattr("completion", value);
    }
}

/// Run one engine call on a detachable worker and wait for it here.
#[cfg(test)]
pub(crate) fn run<T, F>(py: Python<'_>, controls: Controls, job: F) -> PyResult<T>
where
    T: Send + 'static,
    F: FnOnce(CallOptions<'_>) -> Result<T, Error> + Send + 'static,
{
    run_inner(py, controls, job, |_| {}, None)
}

fn run_inner<T, E, F, C>(
    py: Python<'_>,
    controls: Controls,
    job: F,
    finish: C,
    receipt: Option<Arc<ReceiptState>>,
) -> PyResult<T>
where
    T: Send + 'static,
    E: WorkerError,
    F: FnOnce(CallOptions<'_>) -> Result<T, E> + Send + 'static,
    C: FnOnce(&Outcome<T, E>) + Send + 'static,
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
            let outcome = contained(|| work(&stop, controls, job));
            finish(&outcome);
            // The caller may have left. A closed channel is not an error.
            let _left = sender.send(outcome);
            LIVE.fetch_sub(1, Ordering::SeqCst);
        });
    if spawned.is_err() {
        LIVE.fetch_sub(1, Ordering::SeqCst);
        return Err(defect(py, "a call thread could not start"));
    }
    wait(py, receiver, &internal, caller.as_ref(), receipt.as_ref())
}

/// The same worker with borrowed Rust observations copied before each callback ends.
pub(crate) fn run_observed<T, E, F>(
    py: Python<'_>,
    controls: Controls,
    job: F,
) -> PyResult<Completed<T>>
where
    T: Send + 'static,
    E: WorkerError,
    F: FnOnce(CallOptions<'_>) -> Result<Completed<T>, E> + Send + 'static,
{
    run_tallied(py, controls, None, job)
}

/// Observe one call and add its complete or partial core facts to the
/// caller's explicit tally, without rebuilding counts in the host.
pub(crate) fn run_tallied<T, E, F>(
    py: Python<'_>,
    controls: Controls,
    tally: Option<Tally>,
    job: F,
) -> PyResult<Completed<T>>
where
    T: Send + 'static,
    E: WorkerError,
    F: FnOnce(CallOptions<'_>) -> Result<Completed<T>, E> + Send + 'static,
{
    let observations = Observations::default();
    let on_worker = observations.clone();
    let state = Arc::new(ReceiptState::default());
    let final_state = Arc::clone(&state);
    let final_observations = observations.clone();
    let finished = run_inner(
        py,
        controls,
        move |options| {
            let started = tally.as_ref().map(Tally::start);
            let span = Tally::new();
            let spanned = span.start();
            let observer = |event: RecordObservation<'_>| on_worker.push(event);
            let result = job(options.observe(&observer));
            if let Some(started) = started {
                let facts = match &result {
                    Ok(done) => Some(done.facts.clone()),
                    Err(error) => error.facts(),
                };
                if let Some(facts) = facts.as_ref() {
                    started.finish(facts).map_err(E::from)?;
                }
            }
            result.map_err(|error| match spanned.finish(&span.facts()) {
                Ok(()) => error.unaccounted(span.facts()),
                Err(defect) => E::from(defect),
            })
        },
        move |outcome| {
            final_state.finish(terminal(outcome, final_observations.snapshot()));
        },
        Some(state),
    );
    match finished {
        Ok(mut done) => match observations.snapshot() {
            Ok(observed) => {
                if done.details.is_empty() {
                    done.details = observed;
                }
                Ok(done)
            }
            Err(message) => {
                let error = defect(py, message);
                error
                    .value(py)
                    .setattr("facts", python_facts(py, &done.facts)?)?;
                Err(error)
            }
        },
        Err(error) => {
            let value = error.value(py);
            if value.getattr("facts").is_ok()
                && value.getattr("details").is_err()
                && let Ok(observed) = observations.snapshot()
                && let Ok(details) = python_details(py, &observed)
            {
                let _set = value.setattr("details", details);
            }
            Err(error)
        }
    }
}

/// A call's settled receipt. A question event that could not be written
/// fails a call that otherwise succeeded.
fn terminal<T, E: WorkerError>(
    outcome: &Outcome<Completed<T>, E>,
    observed: Result<Vec<Value>, &'static str>,
) -> Terminal {
    match outcome {
        Some(Ok(done)) if observed.is_err() => Terminal {
            outcome: "failed",
            facts: Some(done.facts.clone()),
            details: None,
            failure: Some(Failure {
                kind: ErrorKind::Defect.name(),
                message: crate::result::UNWRITTEN.to_owned(),
                retryable: false,
            }),
        },
        Some(Ok(done)) => Terminal {
            outcome: "succeeded",
            facts: Some(done.facts.clone()),
            details: if done.details.is_empty() {
                observed.ok()
            } else {
                Some(done.details.clone())
            },
            failure: None,
        },
        Some(Err(error)) => Terminal {
            outcome: "failed",
            facts: error.facts(),
            details: error
                .facts()
                .and_then(|_| error.details().or_else(|| observed.ok())),
            failure: Some(error.failure()),
        },
        None => Terminal {
            outcome: "panicked",
            facts: None,
            details: None,
            failure: None,
        },
    }
}

/// The worker's side: the call's options, then the call.
fn work<T, E: From<Error>>(
    stop: &CancelToken,
    controls: Controls,
    job: impl FnOnce(CallOptions<'_>) -> Result<T, E>,
) -> Result<T, E> {
    let caller = controls.token;
    let check = move || caller.as_ref().is_some_and(CancelToken::is_cancelled);
    let options = CallOptions::new().cancel(stop).interrupt(&check);
    let options = match controls.deadline {
        Some(millis) => options.deadline_millis(millis)?,
        None => options,
    };
    job(options)
}

/// The calling thread's side: wait in ticks, and stop on the caller's token
/// or a signal.
fn wait<T: Send, E: WorkerError>(
    py: Python<'_>,
    receiver: Receiver<Outcome<T, E>>,
    internal: &CancelToken,
    caller: Option<&CancelToken>,
    receipt: Option<&Arc<ReceiptState>>,
) -> PyResult<T> {
    let mut receiver = receiver;
    loop {
        // `Receiver` is not `Sync`, so it moves into the detached closure and back.
        let (waited, back) = py.detach(move || (receiver.recv_timeout(TICK), receiver));
        receiver = back;
        if caller.is_some_and(CancelToken::is_cancelled) {
            internal.cancel();
            let error = raise(py, ErrorKind::Cancelled, CANCELLED, false);
            attach_receipt(py, &error, receipt);
            return Err(error);
        }
        match waited {
            Ok(Some(Ok(value))) => return Ok(value),
            Ok(Some(Err(error))) => return Err(error.raised(py)),
            Ok(None) => return Err(defect(py, "the call panicked")),
            Err(RecvTimeoutError::Disconnected) => {
                return Err(defect(py, "the call ended with no result"));
            }
            Err(RecvTimeoutError::Timeout) => {}
        }
        if let Err(signal) = host(|| py.check_signals()) {
            internal.cancel();
            let error = if signal.is_instance_of::<PyKeyboardInterrupt>(py) {
                host(|| drop(signal));
                raise(py, ErrorKind::Cancelled, INTERRUPTED, false)
            } else {
                signal
            };
            attach_receipt(py, &error, receipt);
            return Err(error);
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
            let error = wait(py, receiver, &CancelToken::new(), None, None).err();
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
            let error = wait(py, receiver, &CancelToken::new(), Some(&caller), None).err();
            assert_eq!(
                error.map(|error| error.value(py).to_string()).as_deref(),
                Some("the call was cancelled")
            );
        });
    }
}
