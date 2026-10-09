//! Private canonical execution edge for the named Python family.
//!
//! Start transfers the decoded Request to its native worker. Poll never waits:
//! None means pending, a packet owns its output, and `end` means exhaustion.
//! Push transfers one descriptor only on `accepted`; `full` retains caller
//! ownership and `closed` stops the producer. Finish fixes EOF or a native
//! reader failure. Cancel stops intake without predicting final facts. Close
//! and Python finalization drop the session without joining a held provider.
//! Serialized packets are an internal bridge until generated native result
//! conversion replaces them; this edge is not the public Python API.
use std::sync::Mutex;

use pyo3::prelude::*;
use thinkthen::{
    Request, RequestReaderFailure, RequestSession, RequestSessionPushStatus, RequestSessionRead,
    Surface,
};

use crate::{guard, raised, usage};

#[pyclass(frozen, name = "_RequestSession", module = "thinkthen._thinkthen")]
#[derive(Debug)]
pub(crate) struct Session(Mutex<Option<RequestSession>>);

impl Session {
    pub(crate) fn start(
        py: Python<'_>,
        engine: &thinkthen::Engine,
        request: &str,
        surface: Surface,
    ) -> PyResult<Self> {
        guard(py, || {
            let request = Request::from_json(request).map_err(|e| raised(py, &e))?;
            let session = engine
                .request_session_with_surface(request, surface)
                .map_err(|e| raised(py, &e))?;
            Ok(Self(Mutex::new(Some(session))))
        })
    }
}

#[pymethods]
impl Session {
    fn _poll(&self, py: Python<'_>) -> PyResult<Option<String>> {
        guard(py, || {
            let held = self
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            match held.as_ref().map(RequestSession::try_read) {
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

    fn _push(&self, py: Python<'_>, descriptor: &str) -> PyResult<&'static str> {
        guard(py, || {
            let held = self
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(session) = held.as_ref() else {
                return Ok("closed");
            };
            session
                .try_push_json(descriptor)
                .map(|status| match status {
                    RequestSessionPushStatus::Accepted => "accepted",
                    RequestSessionPushStatus::Full => "full",
                    RequestSessionPushStatus::Closed => "closed",
                })
                .map_err(|e| raised(py, &e))
        })
    }

    #[pyo3(signature = (failure=None))]
    fn _finish(&self, py: Python<'_>, failure: Option<&str>) -> PyResult<()> {
        guard(py, || {
            let held = self
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let session = held
                .as_ref()
                .ok_or_else(|| usage(py, "request session is closed"))?;
            let failure = failure
                .map(RequestReaderFailure::from_json)
                .transpose()
                .map_err(|e| raised(py, &e))?;
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
        drop(session);
    }

    fn __repr__(&self) -> &'static str {
        "<RequestSession>"
    }
}
