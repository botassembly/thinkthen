//! Python ownership and iteration around the shared SDK admission boundary.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::time::Duration;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::sync::MutexExt;
use pyo3::types::PyIterator;
use thinkthen::{
    Cancel, Engine, OffsetUnit, Request, Settings,
    native::{Call, Event},
    questions::{Question, Toolbox},
    records::Record,
};

pyo3::create_exception!(
    _native,
    Error,
    PyException,
    "A library error's family, code and message."
);

fn error(error: thinkthen::Error) -> PyErr {
    Error::new_err((
        error.code().family().as_str(),
        error.code().as_str(),
        error.to_string(),
    ))
}

#[pyfunction]
fn seconds_to_millis(value: f64) -> PyResult<u64> {
    let invalid = || error(thinkthen::native::settings_conversion_error());
    let duration = Duration::try_from_secs_f64(value).map_err(|_| invalid())?;
    if (value * 1000.0).fract() != 0.0 || !duration.subsec_nanos().is_multiple_of(1_000_000) {
        return Err(invalid());
    }
    u64::try_from(duration.as_millis()).map_err(|_| invalid())
}

#[pyfunction]
fn reject_closed_client() -> PyResult<()> {
    Err(error(thinkthen::native::closed_client_error()))
}

#[pyfunction]
fn reject_cancelled() -> PyResult<()> {
    Err(error(thinkthen::native::cancelled_error()))
}

#[pyfunction]
fn reject_settings_conversion() -> PyResult<()> {
    Err(error(thinkthen::native::settings_conversion_error()))
}

#[pyfunction]
fn reject_question_conversion() -> PyResult<()> {
    Err(error(thinkthen::native::question_conversion_error()))
}

#[derive(Debug, Default)]
#[pyclass(frozen)]
struct NativeCancel {
    state: Arc<Mutex<CancelState>>,
}

#[derive(Debug, Default)]
struct CancelState {
    cancelled: bool,
    calls: Vec<Weak<Cancel>>,
    children: Vec<Weak<Mutex<CancelState>>>,
}

impl NativeCancel {
    fn operation(&self, cancel: Cancel) -> Arc<Cancel> {
        let call = Arc::new(cancel);
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.calls.retain(|call| call.strong_count() > 0);
        if state.cancelled {
            call.cancel();
        } else {
            state.calls.push(Arc::downgrade(&call));
        }
        call
    }
}

#[pymethods]
impl NativeCancel {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn child(&self) -> Self {
        let child = Self::default();
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.children.retain(|child| child.strong_count() > 0);
        if state.cancelled {
            child.cancel();
        } else {
            state.children.push(Arc::downgrade(&child.state));
        }
        child
    }

    fn cancel(&self) {
        let (calls, children) = {
            let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
            state.cancelled = true;
            (
                std::mem::take(&mut state.calls),
                std::mem::take(&mut state.children),
            )
        };
        for call in calls.into_iter().filter_map(|call| call.upgrade()) {
            call.cancel();
        }
        for state in children.into_iter().filter_map(|child| child.upgrade()) {
            Self { state }.cancel();
        }
    }
}

#[derive(Debug)]
#[pyclass(frozen)]
struct NativeToolbox {
    value: Toolbox,
}

#[pymethods]
impl NativeToolbox {
    #[new]
    fn new(py: Python<'_>, definition: String) -> PyResult<Self> {
        py.detach(|| Toolbox::from_definitions_json(&definition))
            .map(|value| Self { value })
            .map_err(error)
    }

    #[staticmethod]
    fn from_json(py: Python<'_>, json: String) -> PyResult<Self> {
        py.detach(|| Toolbox::from_json(&json))
            .map(|value| Self { value })
            .map_err(error)
    }

    fn to_json(&self) -> PyResult<String> {
        self.value.to_json().map_err(error)
    }

    fn question(&self, name: Option<&str>) -> PyResult<NativeQuestion> {
        thinkthen::native::select_toolbox(&self.value, name)
            .map(|value| NativeQuestion { value })
            .map_err(error)
    }

    #[getter]
    fn definition(&self) -> PyResult<String> {
        self.value.definitions_json().map_err(error)
    }
}
#[derive(Debug)]
#[pyclass(frozen)]
struct NativeQuestion {
    value: Question,
}
#[pymethods]
impl NativeQuestion {
    #[new]
    fn new(definition: &str) -> PyResult<Self> {
        Question::from_json(definition)
            .map(|value| Self { value })
            .map_err(error)
    }
    #[getter]
    fn function(&self) -> PyResult<String> {
        serde_json::to_string(&self.value.function())
            .map(|text| text.trim_matches('"').to_owned())
            .map_err(|_| error(thinkthen::native::internal_error()))
    }
    #[getter]
    fn definition(&self) -> PyResult<String> {
        serde_json::to_string(&self.value).map_err(|_| error(thinkthen::native::internal_error()))
    }
}

fn admitted_request(
    definition: &str,
    question: Option<&NativeQuestion>,
) -> Result<Request, thinkthen::Error> {
    let Some(question) = question else {
        return Request::from_json(definition);
    };
    let metadata: BTreeMap<String, Option<String>> = serde_json::from_str(definition)
        .map_err(|_| thinkthen::native::question_conversion_error())?;
    let request = Request::new(question.value.clone());
    match metadata.get("name").and_then(Option::as_ref) {
        Some(name) => request.named(name),
        None => Ok(request),
    }
}

#[derive(Debug)]
#[pyclass(frozen)]
struct NativeEngine {
    client: Engine,
}

#[pymethods]
impl NativeEngine {
    #[new]
    fn new(py: Python<'_>, settings_json: &str) -> PyResult<Self> {
        py.detach(|| {
            let client =
                Settings::from_json(settings_json)?.engine(|name| std::env::var(name).ok())?;
            Ok(Self { client })
        })
        .map_err(error)
    }

    #[getter]
    fn settings(&self) -> PyResult<String> {
        serde_json::to_string(&self.client.resolved_settings())
            .map_err(|_| error(thinkthen::native::internal_error()))
    }

    #[pyo3(signature = (request_json, source, cancel, keep_originals, question=None))]
    fn start(
        &self,
        py: Python<'_>,
        request_json: &str,
        source: Py<PyIterator>,
        cancel: &NativeCancel,
        keep_originals: bool,
        question: Option<&NativeQuestion>,
    ) -> PyResult<NativeStream> {
        let call = py
            .detach(|| {
                let request = admitted_request(request_json, question)?;
                Call::open(&self.client, &request, OffsetUnit::CodePoints)
            })
            .map_err(error)?;
        let operation = cancel.operation(call.cancellation());
        Ok(NativeStream {
            cancellation: Arc::clone(&operation),
            state: Mutex::new(StreamState {
                call: Some(call),
                cancel: Some(operation),
                source: Some(source),
                originals: BTreeMap::new(),
                position: 0,
                keep_originals,
            }),
        })
    }

    #[pyo3(signature = (request_json, source, cancel, keep_originals, question=None))]
    fn start_finite(
        &self,
        py: Python<'_>,
        request_json: &str,
        source: Py<PyIterator>,
        cancel: &NativeCancel,
        keep_originals: bool,
        question: Option<&NativeQuestion>,
    ) -> PyResult<NativeStream> {
        // Question admission precedes inspection of the known finite source.
        let request = py
            .detach(|| admitted_request(request_json, question))
            .map_err(error)?;
        if !request.records().is_empty() {
            return Err(error(thinkthen::native::abi_error()));
        }
        let operation = cancel.operation(Cancel::default());
        let mut records = Vec::new();
        let mut originals = BTreeMap::new();
        loop {
            py.check_signals()?;
            if operation.is_cancelled() {
                return Err(error(thinkthen::native::cancelled_error()));
            }
            let Some(item) = source.bind(py).clone().next().transpose()? else {
                break;
            };
            let (record, original) = input_record(item)?;
            records.push(record);
            if keep_originals {
                originals.insert(records.len(), original);
            }
        }
        let position = records.len();
        let client = self.client.clone().cancel((*operation).clone());
        let call = py
            .detach(|| Call::open_records(&client, &request, records, OffsetUnit::CodePoints))
            .map_err(error)?;
        Ok(NativeStream {
            cancellation: Arc::clone(&operation),
            state: Mutex::new(StreamState {
                call: Some(call),
                cancel: Some(operation),
                source: None,
                originals,
                position,
                keep_originals,
            }),
        })
    }
}

// This is the same host conversion marker for finite and pulled input.
// Iterator exceptions remain errors at next().transpose() in each caller.
fn input_record(item: Bound<'_, PyAny>) -> PyResult<(Record, Py<PyAny>)> {
    let (is_text, text, original): (bool, Option<String>, Py<PyAny>) = item.extract()?;
    let record = match text {
        Some(text) if is_text => Record::text(text),
        Some(text) => Record::from_json(&text),
        None => Record::invalid(),
    };
    Ok((record, original))
}

#[derive(Debug)]
struct StreamState {
    call: Option<Call>,
    cancel: Option<Arc<Cancel>>,
    source: Option<Py<PyIterator>>,
    originals: BTreeMap<usize, Py<PyAny>>,
    position: usize,
    keep_originals: bool,
}

#[derive(Debug)]
#[pyclass]
struct NativeStream {
    cancellation: Arc<Cancel>,
    state: Mutex<StreamState>,
}

type RowMessage = (String, Option<String>, bool, Py<PyAny>, usize);

#[pymethods]
impl NativeStream {
    fn cancel(&self) {
        self.cancellation.cancel();
    }

    fn receive(&self, py: Python<'_>) -> PyResult<Option<RowMessage>> {
        let mut state = self
            .state
            .lock_py_attached(py)
            .unwrap_or_else(PoisonError::into_inner);
        loop {
            if let Err(signal) = py.check_signals() {
                if let Some(call) = &state.call {
                    call.cancel();
                }
                return Err(signal);
            }
            let Some(call) = state.call.as_mut() else {
                return Ok(None);
            };
            let event = match py.detach(|| call.poll(Duration::from_millis(20))) {
                Ok(event) => event,
                Err(failure) => {
                    state.call.take();
                    state.cancel.take();
                    return Err(error(failure));
                }
            };
            match event {
                None => {}
                Some(Event::NeedsInput) => {
                    let item = match &state.source {
                        Some(source) => source.bind(py).clone().next().transpose()?,
                        None => None,
                    };
                    if let Some(item) = item {
                        let (record, original) = input_record(item)?;
                        state.position += 1;
                        let position = state.position;
                        if state.keep_originals {
                            state.originals.insert(position, original);
                        }
                        if let Some(call) = state.call.as_mut() {
                            call.send(record).map_err(error)?;
                        }
                    } else if let Some(call) = state.call.as_mut() {
                        call.end_input();
                    }
                }
                Some(Event::Row(row)) => {
                    let position = usize::try_from(row.position())
                        .map_err(|_| error(thinkthen::native::abi_error()))?;
                    let original = state
                        .originals
                        .remove(&position)
                        .unwrap_or_else(|| py.None());
                    let json = serde_json::to_string(&row)
                        .map_err(|_| error(thinkthen::native::internal_error()))?;
                    return Ok(Some((json, None, true, original, position)));
                }
                Some(Event::Finished) => {
                    state.call.take();
                    state.cancel.take();
                    return Ok(None);
                }
                Some(_) => return Err(error(thinkthen::native::abi_error())),
            }
        }
    }

    fn close(&self, py: Python<'_>) {
        self.cancellation.cancel();
        let mut state = self
            .state
            .lock_py_attached(py)
            .unwrap_or_else(PoisonError::into_inner);
        state.call.take();
        state.cancel.take();
        state.source.take();
        state.originals.clear();
    }
}

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("ABI_VERSION", thinkthen::ABI_VERSION)?;
    let codes = thinkthen::Code::ALL
        .iter()
        .map(|code| (code.abi(), code.as_str()))
        .collect::<Vec<_>>();
    module.add("ERROR_CODES", codes)?;
    let closed = wrap_pyfunction!(reject_closed_client, module)?;
    module.add("reject_closed_client", closed)?;
    let seconds = wrap_pyfunction!(seconds_to_millis, module)?;
    module.add("seconds_to_millis", seconds)?;
    let cancelled = wrap_pyfunction!(reject_cancelled, module)?;
    module.add("reject_cancelled", cancelled)?;
    let settings = wrap_pyfunction!(reject_settings_conversion, module)?;
    let question = wrap_pyfunction!(reject_question_conversion, module)?;
    module.add("reject_settings_conversion", settings)?;
    module.add("reject_question_conversion", question)?;
    module.add("Error", module.py().get_type::<Error>())?;
    module.add_class::<NativeCancel>()?;
    module.add_class::<NativeToolbox>()?;
    module.add_class::<NativeQuestion>()?;
    module.add_class::<NativeEngine>()?;
    module.add_class::<NativeStream>()?;
    Ok(())
}
