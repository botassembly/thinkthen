//! Caller-fed native stream over the core's one lazy Batch.
//!
//! Only `__next__` advances Python. The worker receives owned text through a
//! zero-slot handoff, and the core planner owns its bounded request window.

use std::sync::Mutex;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, SyncSender, channel, sync_channel};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use pyo3::exceptions::PyStopIteration;
use pyo3::prelude::*;
use thinkthen::{
    Answer, BatchSetting, CallOptions, CancelToken, Error, Facts, Judgment, Probabilities,
};

use crate::asked::{Asked, Question};
use crate::engine::context;
use crate::engine::{Arg, Engine, Held, answer, batch};
use crate::input::{controls, text};
use crate::result::python_facts;
use crate::tally::PyTally;
use crate::worker::Controls;
use crate::{caught, defect, guard, raise, raised};

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

fn run(
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
) {
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
            let _ = sender.send(Event::Failed(error));
            let _ = sender.send(Event::End(None));
            return;
        }
    };
    let facts = if verb == "decide" || verb == "filter" {
        let mut stream = engine.decide_many_with(asked.decision(), input, options);
        for row in stream.by_ref() {
            match row {
                Ok(row) => {
                    let value = *row.value();
                    let sent = if verb == "filter" {
                        if value == Answer::Yes {
                            sender.send(Event::Row(StreamRow::Text(row.input().clone())))
                        } else {
                            Ok(())
                        }
                    } else {
                        sender.send(Event::Row(StreamRow::Answer(
                            Judgment::Decision(value),
                            Some(row.probability()),
                        )))
                    };
                    if sent.is_err() {
                        stop.cancel();
                        break;
                    }
                }
                Err(error) => {
                    let _ = sender.send(Event::Failed(error));
                    break;
                }
            }
        }
        stream.facts().cloned()
    } else {
        let mut stream = engine.details_many_with(asked.detail(), input, options);
        for row in stream.by_ref() {
            match row {
                Ok(row) => {
                    let value = row.value();
                    if sender
                        .send(Event::Row(StreamRow::Answer(
                            value.value().clone(),
                            probability(value),
                        )))
                        .is_err()
                    {
                        stop.cancel();
                        break;
                    }
                }
                Err(error) => {
                    let _ = sender.send(Event::Failed(error));
                    break;
                }
            }
        }
        stream.facts().cloned()
    };
    if let (Some(started), Some(facts)) = (started, facts.as_ref()) {
        if let Err(error) = started.finish(facts) {
            let _ = sender.send(Event::Failed(error));
        }
    }
    let _ = sender.send(Event::End(facts));
}

#[pyclass(name = "_Stream", module = "thinkthen._thinkthen")]
pub(crate) struct PyStream {
    source: Py<PyAny>,
    input: Option<SyncSender<Option<String>>>,
    events: Mutex<Receiver<Event>>,
    worker: Option<JoinHandle<()>>,
    stop: CancelToken,
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
        guard(py, || {
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
                match event {
                    Ok(Event::Need) => {
                        let item = self.source.bind(py).call_method0("__next__");
                        let next = match item {
                            Ok(item) => Some(text(&item)?),
                            Err(error) if error.is_instance_of::<PyStopIteration>(py) => None,
                            Err(error) => {
                                self.stop.cancel();
                                self.input.take();
                                return Err(error);
                            }
                        };
                        if let Some(input) = &self.input {
                            let _ = py.detach(|| input.send(next));
                        }
                    }
                    Ok(Event::Row(StreamRow::Text(text))) => {
                        return Ok(Some(text.into_pyobject(py)?.into_any().unbind()));
                    }
                    Ok(Event::Row(StreamRow::Answer(value, probability))) => {
                        let value = match value {
                            Judgment::Decision(answered) => answer(py, answered),
                            Judgment::Choice(value) => value.into_pyobject(py)?.unbind(),
                            Judgment::Score(value) => value.into_pyobject(py)?.into_any().unbind(),
                            Judgment::Tags(value) => value.into_pyobject(py)?.unbind(),
                        };
                        let pair = (value, probability).into_pyobject(py)?.unbind();
                        return Ok(Some(pair.into_any()));
                    }
                    Ok(Event::Failed(error)) => {
                        self.stop.cancel();
                        self.input.take();
                        self.finish(py);
                        return Err(raised(py, &error));
                    }
                    Ok(Event::End(facts)) => {
                        self.facts = facts;
                        self.done = true;
                        self.join(py);
                        return Ok(None);
                    }
                    Ok(Event::Panicked) | Err(RecvTimeoutError::Disconnected) => {
                        self.stop.cancel();
                        self.input.take();
                        self.finish(py);
                        return Err(defect(py, "the stream worker ended without an answer"));
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        if let Err(error) = py.check_signals() {
                            self.stop.cancel();
                            self.input.take();
                            return Err(raise(
                                py,
                                thinkthen::ErrorKind::Cancelled,
                                &format!("the stream was interrupted: {error}"),
                                false,
                            ));
                        }
                    }
                }
            }
        })
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

impl PyStream {
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
    let (incoming, source_rx) = sync_channel(0);
    let (events_tx, events_rx) = channel();
    let stop = CancelToken::new();
    let work_stop = stop.clone();
    let work_sender = events_tx.clone();
    let engine = engine.0.clone();
    let asked = question.get().0.clone();
    let verb = verb.to_owned();
    let tally = tally.map(|one| one.get().0.clone());
    let worker = thread::Builder::new()
        .name("thinkthen-stream".to_owned())
        .spawn(move || {
            let source = Input {
                sender: work_sender.clone(),
                receiver: source_rx,
            };
            if caught(|| {
                run(
                    engine,
                    asked,
                    verb,
                    batch,
                    context,
                    controls,
                    work_stop,
                    source,
                    work_sender.clone(),
                    tally,
                )
            })
            .is_none()
            {
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
            facts: None,
            done: false,
        },
    )
}
