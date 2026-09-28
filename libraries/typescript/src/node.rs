//! The one module holding Node-API items: the engine class, the call handle,
//! and the three functions `index.js` calls.
#![allow(
    unsafe_code,
    reason = "the napi macros emit the module registration code"
)]
#![allow(
    missing_docs,
    reason = "napi on a class emits undocumented constructor and method glue"
)]

use std::fmt;
use std::sync::Mutex;

use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::{Env, JsFunction, Result};
use napi_derive::napi;
use thinkthen::{CancelToken, Engine};

use crate::door::{self, Call, Failure};

type Done = ThreadsafeFunction<String, ErrorStrategy::Fatal>;

/// One engine with its own settings, built by `new tt.Engine(options)`.
#[napi]
#[derive(Debug)]
pub struct NativeEngine {
    engine: Engine,
}

/// Build an engine. A refused setting throws its envelope as the message.
#[napi]
pub fn engine(options: String) -> Result<NativeEngine> {
    door::engine(&options)
        .map(|engine| NativeEngine { engine })
        .map_err(|failure| napi::Error::from_reason(failure.envelope()))
}

/// The four counters of this engine, or of the default engine, as an envelope.
#[napi]
pub fn usage(engine: Option<&NativeEngine>) -> String {
    door::usage(engine.map(|held| &held.engine))
}

/// The threadsafe function, held so `detach` can close it.
struct Held(Done);

impl fmt::Debug for Held {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Held")
    }
}

/// One call in flight.
#[napi]
#[derive(Debug)]
pub struct CallHandle {
    token: CancelToken,
    done: Mutex<Option<Held>>,
}

#[napi]
impl CallHandle {
    /// Cancel promptly but retain the worker's final callback. Without a
    /// waiter the callback does not keep the Node process alive.
    #[napi]
    pub fn stop(&self, env: Env) -> Result<()> {
        self.token.cancel();
        if let Ok(mut slot) = self.done.lock()
            && let Some(Held(done)) = slot.as_mut()
        {
            done.unref(&env)?;
        }
        Ok(())
    }

    /// An observed completion keeps Node alive until its final callback.
    #[napi]
    pub fn wait(&self, env: Env) -> Result<()> {
        if let Ok(mut slot) = self.done.lock()
            && let Some(Held(done)) = slot.as_mut()
        {
            done.refer(&env)?;
        }
        Ok(())
    }

    /// Fire the call's token and close its threadsafe function, so a late
    /// envelope is dropped and Node may exit. A second run does nothing.
    #[napi]
    pub fn detach(&self) {
        self.token.cancel();
        let held = self.done.lock().ok().and_then(|mut slot| slot.take());
        if let Some(Held(done)) = held {
            let _closed = done.abort();
        }
    }
}

/// Start one call on its own named worker thread with Rust's default stack.
/// The envelope reaches `done` once, unless the handle was detached first.
#[napi]
#[allow(
    clippy::too_many_arguments,
    reason = "the Node door carries the existing call fields plus batch and context"
)]
pub fn call(
    engine: Option<&NativeEngine>,
    op: String,
    spec: Option<String>,
    payload: String,
    deadline_ms: Option<f64>,
    batch: Option<String>,
    context: Option<String>,
    done: JsFunction,
) -> Result<CallHandle> {
    let done: Done = done.create_threadsafe_function(0, |context| Ok(vec![context.value]))?;
    let token = CancelToken::new();
    let (worker_token, worker_done) = (token.clone(), done.clone());
    let engine = engine.map(|held| held.engine.clone());
    let call = Call {
        op,
        spec,
        payload,
        deadline_ms,
        batch,
        context,
    };
    let started = std::thread::Builder::new()
        .name("thinkthen-call".to_owned())
        .spawn(move || {
            let envelope = door::answer(engine.as_ref(), &call, &worker_token);
            let _status = worker_done.call(envelope, ThreadsafeFunctionCallMode::NonBlocking);
        });
    if let Err(error) = started {
        let refused = Failure::local(format!("the call's worker thread could not start: {error}"));
        let _status = done.call(refused.envelope(), ThreadsafeFunctionCallMode::NonBlocking);
    }
    Ok(CallHandle {
        token,
        done: Mutex::new(Some(Held(done))),
    })
}
