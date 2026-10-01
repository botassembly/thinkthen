//! Caller-fed native stream over the core's one lazy Batch.
//!
//! Only `__next__` advances Python. The worker receives owned text through a
//! zero-slot handoff, and the core planner owns its bounded request window.

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use pyo3::exceptions::{PyKeyboardInterrupt, PyStopIteration};
use pyo3::prelude::*;
use thinkthen::fork_safe::{Receiver, RecvTimeoutError, Sender, channel};
use thinkthen::{
    Answer, BatchSetting, CallOptions, CancelToken, Error, Facts, Judgment, Probabilities,
    contained,
};

use crate::asked::{Asked, Question};
use crate::engine::context;
use crate::engine::{Arg, Engine, Held, answer, batch, only};
use crate::input::{controls, text};
use crate::result::python_facts;
use crate::tally::PyTally;
use crate::worker::{
    Controls, Failure, ReceiptState, WorkerError, attach_receipt, finish_stream_receipt,
    stream_receipt,
};
use crate::{defect, guard, raise, raised};

const TICK: Duration = Duration::from_millis(50);

enum Event {
    Need,
    Row(StreamRow),
    Failed(Error),
    End(Option<Facts>),
    Panicked,
}

enum StreamRow {
    Text(String),
    Answer(Judgment, Option<f64>),
}

struct Input {
    sender: Sender<Event>,
    receiver: Receiver<Option<String>>,
}

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.sender.send(Event::Need).ok()?;
        self.receiver.recv().ok().flatten()
    }
}

fn probability(details: &thinkthen::Details) -> Option<f64> {
    match (details.value(), details.probabilities()) {
        (Judgment::Decision(_), Probabilities::YesNo { yes }) => Some(*yes),
        (Judgment::Choice(Some(selected)), Probabilities::Named(values)) => values
            .iter()
            .find(|one| one.name() == selected)
            .map(thinkthen::NamedProbability::probability),
        _ => None,
    }
}

struct Run {
    engine: thinkthen::Engine,
    asked: Asked,
    verb: String,
    batch: Option<BatchSetting>,
    context: Option<String>,
    controls: Controls,
    stop: CancelToken,
    input: Input,
    sender: Sender<Event>,
    tally: Option<thinkthen::Tally>,
    receipt: Arc<ReceiptState>,
}

struct Emitter<'a> {
    stop: &'a CancelToken,
    sender: &'a Sender<Event>,
}

fn decisions(
    engine: &thinkthen::Engine,
    asked: &Asked,
    filter: bool,
    input: Input,
    options: CallOptions<'_>,
    emitter: Emitter<'_>,
) -> (Option<Facts>, Option<Failure>) {
    let mut stream = engine.decide_many_with(asked.decision(), input, options);
    let mut failure = None;
    for row in stream.by_ref() {
        let row = match row {
            Ok(row) => row,
            Err(error) => {
                failure = Some(error.failure());
                let _ = emitter.sender.send(Event::Failed(error));
                break;
            }
        };
        let sent = if filter {
            (*row.value() == Answer::Yes).then(|| Event::Row(StreamRow::Text(row.input().clone())))
        } else {
            Some(Event::Row(StreamRow::Answer(
                Judgment::Decision(*row.value()),
                Some(row.probability()),
            )))
        };
        if sent.is_some_and(|event| emitter.sender.send(event).is_err()) {
            emitter.stop.cancel();
            break;
        }
    }
    (stream.facts().cloned(), failure)
}

fn details(
    engine: &thinkthen::Engine,
    asked: &Asked,
    input: Input,
    options: CallOptions<'_>,
    stop: &CancelToken,
    sender: &Sender<Event>,
) -> (Option<Facts>, Option<Failure>) {
    let mut stream = engine.details_many_with(asked.detail(), input, options);
    let mut failure = None;
    for row in stream.by_ref() {
        let row = match row {
            Ok(row) => row,
            Err(error) => {
                failure = Some(error.failure());
                let _ = sender.send(Event::Failed(error));
                break;
            }
        };
        let value = row.value();
        let event = Event::Row(StreamRow::Answer(value.value().clone(), probability(value)));
        if sender.send(event).is_err() {
            stop.cancel();
            break;
        }
    }
    (stream.facts().cloned(), failure)
}

fn run(job: Run) {
    let Run {
        engine,
        asked,
        verb,
        batch,
        context,
        controls,
        stop,
        input,
        sender,
        tally,
        receipt,
    } = job;
    let started = tally.as_ref().map(thinkthen::Tally::start);
    let caller = controls.token;
    let check = || caller.as_ref().is_some_and(CancelToken::is_cancelled);
    let options = CallOptions::new().cancel(&stop).interrupt(&check);
    let result = controls
        .deadline
        .map_or(Ok(options), |ms| options.deadline_millis(ms));
    let result = result.map(|options| {
        let options = batch.map_or(options, |batch| options.batch(batch));
        context
            .as_deref()
            .map_or(options, |context| options.context(context))
    });
    let options = match result {
        Ok(options) => options,
        Err(error) => {
            finish_stream_receipt(&receipt, None, Some(error.failure()), false);
            let _ = sender.send(Event::Failed(error));
            let _ = sender.send(Event::End(None));
            return;
        }
    };
    let (facts, mut failure) = if verb == "decide" || verb == "filter" {
        decisions(
            &engine,
            &asked,
            verb == "filter",
            input,
            options,
            Emitter {
                stop: &stop,
                sender: &sender,
            },
        )
    } else {
        details(&engine, &asked, input, options, &stop, &sender)
    };
    if let (Some(started), Some(facts)) = (started, facts.as_ref())
        && let Err(error) = started.finish(facts)
    {
        failure = Some(error.failure());
        let _ = sender.send(Event::Failed(error));
    }
    finish_stream_receipt(&receipt, facts.as_ref(), failure, false);
    let _ = sender.send(Event::End(facts));
}

#[pyclass(name = "_Stream", module = "thinkthen._thinkthen")]
pub(crate) struct PyStream {
    source: Py<PyAny>,
    input: Option<Sender<Option<String>>>,
    events: Mutex<Receiver<Event>>,
    worker: Option<JoinHandle<()>>,
    stop: CancelToken,
    receipt: Arc<ReceiptState>,
    facts: Option<Facts>,
    done: bool,
}

impl std::fmt::Debug for PyStream {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("PyStream").finish_non_exhaustive()
    }
}

impl Drop for PyStream {
    fn drop(&mut self) {
        self.stop.cancel();
        self.input.take();
    }
}

#[pymethods]
impl PyStream {
    fn __iter__(self_: PyRef<'_, Self>) -> PyRef<'_, Self> {
        self_
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        guard(py, || self.read(py))
    }

    fn close(&mut self, py: Python<'_>) {
        self.stop.cancel();
        self.input.take();
        self.finish(py);
    }

    #[getter]
    fn facts(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self.facts
            .as_ref()
            .map(|facts| python_facts(py, facts))
            .transpose()
    }
}

enum Step {
    Continue,
    Item(Py<PyAny>),
    End,
}

impl PyStream {
    fn read(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        if self.done {
            return Ok(None);
        }
        loop {
            let event = py.detach(|| {
                self.events
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .recv_timeout(TICK)
            });
            match self.advance(py, event)? {
                Step::Continue => {}
                Step::Item(value) => return Ok(Some(value)),
                Step::End => return Ok(None),
            }
        }
    }

    fn feed(&mut self, py: Python<'_>) -> PyResult<()> {
        let item = self.source.bind(py).call_method0("__next__");
        let next = match item {
            Ok(item) => match text(&item) {
                Ok(text) => Some(text),
                Err(error) => {
                    self.stop.cancel();
                    self.input.take();
                    self.finish(py);
                    return Err(error);
                }
            },
            Err(error) if error.is_instance_of::<PyStopIteration>(py) => None,
            Err(error) => {
                self.stop.cancel();
                self.input.take();
                self.finish(py);
                return Err(error);
            }
        };
        if let Some(input) = &self.input {
            let _ = input.send(next);
        }
        Ok(())
    }

    fn advance(
        &mut self,
        py: Python<'_>,
        event: Result<Event, RecvTimeoutError>,
    ) -> PyResult<Step> {
        match event {
            Ok(Event::Need) => {
                self.feed(py)?;
                Ok(Step::Continue)
            }
            Ok(Event::Row(StreamRow::Text(text))) => {
                Ok(Step::Item(text.into_pyobject(py)?.into_any().unbind()))
            }
            Ok(Event::Row(StreamRow::Answer(value, probability))) => {
                let value = match value {
                    Judgment::Decision(answered) => answer(py, answered),
                    Judgment::Choice(value) => value.into_pyobject(py)?.unbind(),
                    Judgment::Score(value) => value.into_pyobject(py)?.into_any().unbind(),
                    Judgment::Tags(value) => value.into_pyobject(py)?.unbind(),
                };
                Ok(Step::Item(
                    (value, probability).into_pyobject(py)?.unbind().into_any(),
                ))
            }
            Ok(Event::Failed(error)) => {
                self.stop.cancel();
                self.input.take();
                self.finish(py);
                Err(raised(py, &error))
            }
            Ok(Event::End(facts)) => {
                self.facts = facts;
                self.done = true;
                self.join(py);
                Ok(Step::End)
            }
            Ok(Event::Panicked) | Err(RecvTimeoutError::Disconnected) => {
                self.stop.cancel();
                self.input.take();
                self.finish(py);
                Err(defect(py, "the stream worker ended without an answer"))
            }
            Err(RecvTimeoutError::Timeout) => {
                self.interrupt(py)?;
                Ok(Step::Continue)
            }
        }
    }

    fn interrupt(&mut self, py: Python<'_>) -> PyResult<()> {
        let Err(error) = py.check_signals() else {
            return Ok(());
        };
        self.stop.cancel();
        self.input.take();
        self.done = true;
        if error.is_instance_of::<PyKeyboardInterrupt>(py) {
            let stopped = raise(
                py,
                thinkthen::ErrorKind::Cancelled,
                "the stream was interrupted; no new request starts, and sent requests end on their own",
                false,
            );
            attach_receipt(py, &stopped, Some(&self.receipt));
            return Err(stopped);
        }
        Err(error)
    }

    fn join(&mut self, py: Python<'_>) {
        if let Some(worker) = self.worker.take() {
            let _ = py.detach(|| worker.join());
        }
    }

    fn finish(&mut self, py: Python<'_>) {
        if self.done {
            return;
        }
        loop {
            match py.detach(|| {
                self.events
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .recv_timeout(TICK)
            }) {
                Ok(Event::End(facts)) => {
                    self.facts = facts;
                    break;
                }
                Err(RecvTimeoutError::Disconnected) => break,
                _ => {}
            }
        }
        self.done = true;
        self.join(py);
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "the native stream constructor forwards the same validated controls as the PyO3 entry point"
)]
pub(crate) fn prepare(
    py: Python<'_>,
    engine: &Engine,
    verb: &str,
    question: &Bound<'_, Question>,
    source: Py<PyAny>,
    batch_value: Arg<'_, '_>,
    context_value: Arg<'_, '_>,
    deadline: Arg<'_, '_>,
    token: Held<'_, '_>,
    tally: Option<&Bound<'_, PyTally>>,
) -> PyResult<Py<PyStream>> {
    let (batch, context) = (batch(batch_value)?, context(context_value)?);
    let controls = controls(py, deadline, token)?;
    let asked = question.get().0.clone();
    if verb == "filter" {
        only(py, &asked, verb, "decide")?;
    }
    let (incoming, source_rx) = channel();
    let (events_tx, events_rx) = channel();
    let stop = CancelToken::new();
    let receipt = stream_receipt();
    let work_receipt = Arc::clone(&receipt);
    let work_stop = stop.clone();
    let work_sender = events_tx.clone();
    let engine = engine.0.clone();
    let verb = verb.to_owned();
    let tally = tally.map(|one| one.get().0.clone());
    let worker = thread::Builder::new()
        .name("thinkthen-stream".to_owned())
        .spawn(move || {
            let source = Input {
                sender: work_sender.clone(),
                receiver: source_rx,
            };
            if contained(|| {
                run(Run {
                    engine,
                    asked,
                    verb,
                    batch,
                    context,
                    controls,
                    stop: work_stop,
                    input: source,
                    sender: work_sender.clone(),
                    tally,
                    receipt: Arc::clone(&work_receipt),
                })
            })
            .is_none()
            {
                finish_stream_receipt(&work_receipt, None, None, true);
                let _ = work_sender.send(Event::Panicked);
            }
        })
        .map_err(|_| defect(py, "a stream thread could not start"))?;
    Py::new(
        py,
        PyStream {
            source,
            input: Some(incoming),
            events: Mutex::new(events_rx),
            worker: Some(worker),
            stop,
            receipt,
            facts: None,
            done: false,
        },
    )
}
