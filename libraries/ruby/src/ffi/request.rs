//! Private bounded native sessions for the named Ruby calls.
use super::{EngineValue, Fault, checked};
use magnus::{Error, RClass, RModule, Ruby, Value, function, method, prelude::*};
use std::sync::{Mutex, PoisonError};
use thinkthen::{
    Request, RequestReaderFailure, RequestSession, RequestSessionPushStatus, RequestSessionRead,
};
#[magnus::wrap(class = "ThinkThen::Native::RequestSession", free_immediately, size)]
#[derive(Debug)]
struct Session(Mutex<Option<RequestSession>>);
impl Session {
    fn poll(ruby: &Ruby, this: &Self) -> Result<Option<Value>, Error> {
        let held = this.0.lock().unwrap_or_else(PoisonError::into_inner);
        match held.as_ref().map(RequestSession::try_read) {
            Some(RequestSessionRead::Result(packet)) => {
                let json = checked(ruby, packet.to_json().map_err(Fault::from))?;
                super::native_result::packet(ruby, &json).map(Some)
            }
            Some(RequestSessionRead::Pending) => Ok(None),
            Some(RequestSessionRead::End) | None => Ok(Some(ruby.into_value("end"))),
        }
    }
    fn push(ruby: &Ruby, this: &Self, descriptor: String) -> Result<&'static str, Error> {
        let held = this.0.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(session) = held.as_ref() else {
            return Ok("closed");
        };
        checked(
            ruby,
            session.try_push_json(&descriptor).map_err(Fault::from),
        )
        .map(|s| match s {
            RequestSessionPushStatus::Accepted => "accepted",
            RequestSessionPushStatus::Full => "full",
            RequestSessionPushStatus::Closed => "closed",
        })
    }
    fn finish(ruby: &Ruby, this: &Self, failure: Option<String>) -> Result<(), Error> {
        let held = this.0.lock().unwrap_or_else(PoisonError::into_inner);
        let session = checked(
            ruby,
            held.as_ref()
                .ok_or_else(|| Fault::usage("request session is closed")),
        )?;
        let failure = checked(
            ruby,
            failure
                .as_deref()
                .map(RequestReaderFailure::from_json)
                .transpose()
                .map_err(Fault::from),
        )?;
        checked(ruby, session.finish(failure).map_err(Fault::from))
    }
    fn cancel(&self) {
        if let Some(session) = self
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .as_ref()
        {
            session.cancel();
        }
    }
    fn close(&self) {
        drop(self.0.lock().unwrap_or_else(PoisonError::into_inner).take());
    }
}
fn start(ruby: &Ruby, engine: &EngineValue, text: String) -> Result<Session, Error> {
    let request = checked(ruby, Request::from_json(&text).map_err(Fault::from))?;
    let session = checked(
        ruby,
        engine
            .engine
            .request_session_with_surface(request, thinkthen::Surface::Ruby)
            .map_err(Fault::from),
    )?;
    Ok(Session(Mutex::new(Some(session))))
}
fn request_engine(ruby: &Ruby, text: String) -> Result<EngineValue, Error> {
    checked(
        ruby,
        thinkthen::EngineBuilder::from_settings_json(&text)
            .and_then(thinkthen::EngineBuilder::build)
            .map_err(Fault::from),
    )
    .map(|engine| EngineValue { engine })
}
pub(super) fn register(ruby: &Ruby, native: RModule, engine: RClass) -> Result<(), Error> {
    native.define_module_function("request_engine", function!(request_engine, 1))?;
    let session = native.define_class("RequestSession", ruby.class_object())?;
    session.define_method("poll", method!(Session::poll, 0))?;
    session.define_method("push", method!(Session::push, 1))?;
    session.define_method("finish", method!(Session::finish, 1))?;
    session.define_method("cancel", method!(Session::cancel, 0))?;
    session.define_method("close", method!(Session::close, 0))?;
    engine.define_method("request_session", method!(start, 1))?;
    Ok(())
}
