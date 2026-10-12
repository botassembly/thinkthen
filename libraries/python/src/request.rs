//! Private canonical execution edge for the named Python family.
//!
//! Start transfers the constructed Request to its native worker. Poll never waits:
//! None means pending, a packet owns its output, and `end` means exhaustion.
//! Push transfers one descriptor only on `accepted`; `full` retains caller
//! ownership and `closed` stops the producer. Finish fixes EOF or a native
//! reader failure. Cancel stops intake without predicting final facts. Close
//! and Python finalization drop the session without joining a held provider.
//! Serialized packets are an internal bridge until generated native result
//! conversion replaces them; this edge is not the public Python API.
use std::sync::{Arc, Mutex};

use pyo3::prelude::*;
use thinkthen::{
    Request, RequestReaderFailure, RequestSession, RequestSessionPush, RequestSessionRead, Surface,
};

use crate::{guard, raised, usage};

#[pyclass(frozen, name = "_RequestItem", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Item(thinkthen::RequestItem);
#[pymethods]
impl Item {
    #[new]
    fn new(py: Python<'_>, value: &Bound<'_, PyAny>, verb: &str) -> PyResult<Self> {
        crate::inputs::item(py, value, verb).map(Self)
    }
}

#[pyclass(frozen, name = "_RequestSession", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Session(Mutex<Option<Arc<RequestSession>>>);

impl Session {
    pub(crate) fn start(
        py: Python<'_>,
        engine: &thinkthen::Engine,
        request: Request,
        surface: Surface,
    ) -> PyResult<Self> {
        guard(py, || {
            let session = engine
                .request_session_with_surface(request, surface)
                .map_err(|e| raised(py, &e))?;
            Ok(Self(Mutex::new(Some(Arc::new(session)))))
        })
    }
}

#[pymethods]
impl Session {
    fn _wait(&self, py: Python<'_>, input: bool) {
        let session = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        if let Some(session) = session {
            py.detach(move || {
                let interest = if input {
                    thinkthen::RequestSessionWaitInterest::Input
                } else {
                    thinkthen::RequestSessionWaitInterest::Settlement
                };
                let _ = session.wait(interest, std::time::Duration::from_millis(50));
            });
        }
    }

    fn _poll(&self, py: Python<'_>) -> PyResult<Option<String>> {
        guard(py, || {
            let held = self
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            match held.as_ref().map(|session| session.try_read()) {
                Some(RequestSessionRead::Result(packet)) => {
                    packet.to_json().map(Some).map_err(|e| raised(py, &e))
                }
                Some(RequestSessionRead::Pending) => Ok(None),
                Some(RequestSessionRead::End) | None => Ok(Some("{\"kind\":\"end\"}".into())),
            }
        })
    }

    fn _poll_typed(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        self._poll(py)?
            .map(|json| {
                if json == "{\"kind\":\"end\"}" {
                    let value = serde_json::from_str(&json)
                        .map_err(|_| crate::defect(py, "native end could not be read"))?;
                    return crate::native_result::plain(py, &value);
                }
                crate::native_result::_restore_native_result(py, "completesessionPacket", &json)
            })
            .transpose()
    }

    fn _push(&self, py: Python<'_>, value: &Item) -> PyResult<&'static str> {
        guard(py, || {
            let held = self
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(session) = held.as_ref() else {
                return Ok("closed");
            };
            session
                .try_push(thinkthen::RequestSessionDescriptor {
                    item: value.0.clone(),
                    location: None,
                })
                .map(|status| match status {
                    RequestSessionPush::Accepted => "accepted",
                    RequestSessionPush::Full(_) => "full",
                    RequestSessionPush::Closed(_) => "closed",
                })
                .map_err(|e| raised(py, &e))
        })
    }

    #[pyo3(signature = (failure=None))]
    fn _finish(&self, py: Python<'_>, failure: Option<bool>) -> PyResult<()> {
        guard(py, || {
            let held = self
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let session = held
                .as_ref()
                .ok_or_else(|| usage(py, "request session is closed"))?;
            let failure = failure
                .filter(|v| *v)
                .map(|_| RequestReaderFailure::Producer);
            session.finish(failure).map_err(|e| raised(py, &e))
        })
    }

    fn cancel(&self) {
        if let Some(session) = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_ref()
        {
            session.cancel();
        }
    }

    fn close(&self) {
        let session = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        if let Some(session) = session {
            session.cancel();
        }
    }

    fn __repr__(&self) -> &'static str {
        "<RequestSession>"
    }
}
