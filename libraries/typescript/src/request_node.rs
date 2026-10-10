//! Nonblocking owned Request sessions for the JavaScript scheduler.
#![allow(unsafe_code, reason = "napi emits registration glue")]
#![allow(missing_docs, reason = "napi emits public method glue")]
use crate::node::NativeEngine;
use napi::Result;
use napi_derive::napi;
use thinkthen::{
    Request, RequestReaderFailure, RequestSession, RequestSessionPushStatus, RequestSessionRead,
};
fn failure(error: thinkthen::Error) -> napi::Error {
    napi::Error::from_reason(serde_json::json!({"err":{"kind":error.kind().name(),"message":error.to_string(),"retryable":error.retryable()}}).to_string())
}
#[napi]
#[derive(Debug)]
pub struct NativeSession {
    session: Option<RequestSession>,
}
#[napi]
pub fn request_engine(settings: String) -> Result<NativeEngine> {
    thinkthen::EngineBuilder::from_settings_json(&settings)
        .and_then(thinkthen::EngineBuilder::build)
        .map(|engine| NativeEngine { engine })
        .map_err(failure)
}
#[napi]
pub fn request_session(engine: &NativeEngine, text: String) -> Result<NativeSession> {
    let request = Request::from_json(&text).map_err(failure)?;
    let session = engine
        .engine
        .request_session_with_surface(request, thinkthen::Surface::Javascript)
        .map_err(failure)?;
    Ok(NativeSession {
        session: Some(session),
    })
}
#[napi]
impl NativeSession {
    #[napi]
    pub fn poll(&self) -> Result<Option<String>> {
        match self.session.as_ref().map(RequestSession::try_read) {
            Some(RequestSessionRead::Result(packet)) => packet.to_json().map(Some).map_err(failure),
            Some(RequestSessionRead::Pending) => Ok(None),
            Some(RequestSessionRead::End) | None => Ok(Some("end".into())),
        }
    }
    #[napi]
    pub fn push(&self, descriptor: String) -> Result<String> {
        let Some(session) = &self.session else {
            return Ok("closed".into());
        };
        session
            .try_push_json(&descriptor)
            .map(|s| {
                match s {
                    RequestSessionPushStatus::Accepted => "accepted",
                    RequestSessionPushStatus::Full => "full",
                    RequestSessionPushStatus::Closed => "closed",
                }
                .into()
            })
            .map_err(failure)
    }
    #[napi]
    pub fn finish(&self, error: Option<String>) -> Result<()> {
        if let Some(session) = &self.session {
            let error = error
                .as_deref()
                .map(RequestReaderFailure::from_json)
                .transpose()
                .map_err(failure)?;
            session.finish(error).map_err(failure)?;
        }
        Ok(())
    }
    #[napi]
    pub fn cancel(&self) {
        if let Some(session) = &self.session {
            session.cancel();
        }
    }
    #[napi]
    pub fn close(&mut self) {
        drop(self.session.take());
    }
}
