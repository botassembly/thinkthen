//! Python ownership and iteration around the shared SDK admission boundary.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError, Weak};
use std::time::Duration;

use pyo3::exceptions::{PyException, PyKeyboardInterrupt};
use pyo3::prelude::*;
use pyo3::sync::MutexExt;
use pyo3::types::PyIterator;
use thinkthen::{
    Cancel, Client, OffsetUnit, PullCall, PullEvent, SdkRequest, Toolbox, ToolboxQuestion,
};

pyo3::create_exception!(
    _native,
    Error,
    PyException,
    "A library error's family, code and message."
);

fn error(error: thinkthen::Error) -> PyErr {
    if error.kind() == thinkthen::ErrorKind::Cancelled {
        return PyKeyboardInterrupt::new_err(());
    }
    Error::new_err((
        error.kind().family().as_str(),
        error.kind().code(),
        error.to_string(),
    ))
}

#[derive(Debug, Default)]
#[pyclass(frozen)]
struct NativeCancel {
    state: Mutex<CancelState>,
}

#[derive(Debug, Default)]
struct CancelState {
    cancelled: bool,
    calls: Vec<Weak<Cancel>>,
}

impl NativeCancel {
    fn operation(&self) -> Arc<Cancel> {
        let call = Arc::new(Cancel::default());
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

    fn cancel(&self) {
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.cancelled = true;
        for call in state.calls.drain(..).filter_map(|call| call.upgrade()) {
            call.cancel();
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
        py.detach(|| Toolbox::from_bytes(definition.as_bytes()))
            .map(|value| Self { value })
            .map_err(error)
    }

    fn question(&self, name: Option<&str>) -> PyResult<NativeQuestion> {
        self.value
            .question(name)
            .map(|value| NativeQuestion {
                value: value.clone(),
            })
            .map_err(error)
    }
}

#[derive(Debug)]
#[pyclass(frozen)]
struct NativeQuestion {
    value: ToolboxQuestion,
}

#[pymethods]
impl NativeQuestion {
    #[getter]
    fn function(&self) -> &str {
        self.value.function().name()
    }
}

#[derive(Debug)]
#[pyclass(frozen)]
struct NativeEngine {
    client: Client,
}

#[pymethods]
impl NativeEngine {
    #[new]
    fn new(py: Python<'_>, settings_json: &str) -> PyResult<Self> {
        py.detach(|| {
            let client = Client::from_json(settings_json, |name| std::env::var(name).ok())?;
            Ok(Self { client })
        })
        .map_err(error)
    }

    #[pyo3(signature = (function, request_json, options_json, source, cancel, keep_originals, loaded=None, toolbox=None))]
    #[expect(
        clippy::too_many_arguments,
        reason = "the bridge carries an opaque question and Python iterator"
    )]
    fn start(
        &self,
        py: Python<'_>,
        function: &str,
        request_json: &str,
        options_json: &str,
        source: Py<PyIterator>,
        cancel: &NativeCancel,
        keep_originals: bool,
        loaded: Option<&NativeQuestion>,
        toolbox: Option<&NativeToolbox>,
    ) -> PyResult<NativeStream> {
        let operation = cancel.operation();
        let call = py
            .detach(|| {
                let request = if let Some(question) = loaded {
                    SdkRequest::from_question(function, question.value.clone(), options_json)?
                } else if let Some(toolbox) = toolbox {
                    SdkRequest::from_toolbox(toolbox.value.clone(), options_json)?
                } else {
                    SdkRequest::from_json(request_json)?
                };
                self.client
                    .open(&request, OffsetUnit::CodePoints, (*operation).clone())
            })
            .map_err(error)?;
        Ok(NativeStream {
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
}

#[derive(Debug)]
struct StreamState {
    call: Option<PullCall>,
    cancel: Option<Arc<Cancel>>,
    source: Option<Py<PyIterator>>,
    originals: BTreeMap<usize, Py<PyAny>>,
    position: usize,
    keep_originals: bool,
}

#[derive(Debug)]
#[pyclass]
struct NativeStream {
    state: Mutex<StreamState>,
}

type RowMessage = (String, Option<String>, bool, Py<PyAny>, usize);

#[pymethods]
impl NativeStream {
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
            let event = py.detach(|| call.poll(Duration::from_millis(20)));
            match event {
                None => {}
                Some(PullEvent::NeedsInput) => {
                    let item = match &state.source {
                        Some(source) => source.bind(py).clone().next().transpose()?,
                        None => None,
                    };
                    if let Some(item) = item {
                        let (is_text, text, original): (bool, String, Py<PyAny>) =
                            item.extract()?;
                        state.position += 1;
                        let position = state.position;
                        if state.keep_originals {
                            state.originals.insert(position, original);
                        }
                        if let Some(call) = state.call.as_mut() {
                            if is_text {
                                call.send_text(&text)
                            } else {
                                call.send_json(&text)
                            }
                            .map_err(error)?;
                        }
                    } else if let Some(call) = state.call.as_mut() {
                        call.end_input();
                    }
                }
                Some(PullEvent::Row(row)) => {
                    let original = state
                        .originals
                        .remove(&row.position)
                        .unwrap_or_else(|| py.None());
                    return Ok(Some((
                        row.json,
                        row.detail,
                        row.kept,
                        original,
                        row.position,
                    )));
                }
                Some(PullEvent::Finished) => {
                    state.cancel.take();
                    let failed = state
                        .call
                        .take()
                        .and_then(|call| call.final_error().cloned());
                    return match failed {
                        Some(failed) => Err(error(failed)),
                        None => Ok(None),
                    };
                }
            }
        }
    }

    fn close(&self, py: Python<'_>) {
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
    module.add("Error", module.py().get_type::<Error>())?;
    module.add_class::<NativeCancel>()?;
    module.add_class::<NativeToolbox>()?;
    module.add_class::<NativeQuestion>()?;
    module.add_class::<NativeEngine>()?;
    module.add_class::<NativeStream>()?;
    Ok(())
}
