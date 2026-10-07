//! Node holds a native batch; callbacks only transfer its next typed packet.
#![allow(
    missing_docs,
    reason = "napi emits public registration and method glue"
)]
use crate::complete::stream::{Session, failure};
use crate::node::NativeEngine;
use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{JsFunction, Result};
use napi_derive::napi;
use std::sync::Arc;

#[napi]
#[derive(Debug)]
pub struct CompleteBatch {
    session: Arc<Session>,
}
#[napi]
pub fn complete_batch(
    engine: Option<&NativeEngine>,
    request: String,
    deadline: Option<f64>,
    context: Option<String>,
) -> Result<CompleteBatch> {
    let deadline =
        crate::door::deadline_of(deadline).map_err(|e| napi::Error::from_reason(e.envelope()))?;
    let engine = engine
        .map_or_else(thinkthen::Engine::from_env, |e| Ok(e.engine.clone()))
        .map_err(|e| napi::Error::from_reason(crate::door::Failure::from(e).envelope()))?;
    let session = Session::start(engine, request, deadline, None, context)
        .map_err(|e| napi::Error::from_reason(crate::door::Failure::from(e).envelope()))?;
    Ok(CompleteBatch {
        session: Arc::new(session),
    })
}
#[napi]
impl CompleteBatch {
    #[napi]
    pub fn pull(&self, done: JsFunction) -> Result<()> {
        let done: ThreadsafeFunction<String, ErrorStrategy::Fatal> =
            done.create_threadsafe_function(0, |ctx| Ok(vec![ctx.value]))?;
        self.session
            .advance()
            .map_err(|e| napi::Error::from_reason(crate::door::Failure::from(e).envelope()))?;
        let session = Arc::clone(&self.session);
        std::thread::Builder::new()
            .name("thinkthen-complete-next".into())
            .spawn(move || {
                let message = next(&session);
                let _sent = done.call(message, ThreadsafeFunctionCallMode::Blocking);
            })
            .map_err(|_| napi::Error::from_reason("complete next worker did not start"))?;
        Ok(())
    }
    #[napi]
    pub fn cancel(&self) {
        self.session.cancel();
    }
    #[napi]
    pub fn close(&self) {
        self.session.close();
    }
}

fn next(session: &Session) -> String {
    loop {
        match session.poll() {
            Ok(Some(event)) => return event,
            Ok(None) => {},
            Err(error) => return failure(&error).map_or_else(
                |_| "{\"error\":{\"kind\":\"defect\",\"message\":\"complete batch failed\",\"retryable\":false}}".into(),
                |e| format!("{{\"error\":{e}}}")),
        }
    }
}
