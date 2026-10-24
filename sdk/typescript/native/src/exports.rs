//! Node classes and their macro-generated support methods.

use super::{Bridge, Event, request};
use napi::threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction};
use napi::{Env, JsBoolean, JsFunction, JsString, JsUnknown};
use napi_derive::napi;
use thinkthen::Cancel;

/// Reject a JavaScript number that has no JSON representation.
///
/// # Errors
/// Always returns the library's record encoding error.
#[napi]
pub fn reject_non_finite_json() -> napi::Result<()> {
    Err(host_error(thinkthen::Error::new(
        thinkthen::ErrorKind::Record,
        "JSON numbers must be finite",
    )))
}

/// A cancellation handle for the one running stream.
#[napi]
pub struct Operation {
    cancel: Cancel,
    callback: ThreadsafeFunction<Event>,
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format
            .debug_struct("Operation")
            .field("cancel", &self.cancel)
            .finish_non_exhaustive()
    }
}

#[napi]
impl Operation {
    /// Keep Node alive while the caller awaits the next event.
    #[napi]
    pub fn reference(&mut self, env: Env) -> napi::Result<()> {
        self.callback.refer(&env)
    }

    /// Let Node exit while iteration is suspended at a delivered row.
    #[napi]
    pub fn unreference(&mut self, env: Env) -> napi::Result<()> {
        self.callback.unref(&env)
    }

    /// Stop input, new requests and retry waits.
    #[napi]
    pub fn cancel(&self) {
        self.cancel.cancel();
    }
}

/// A client retains its backend engine and shared HTTP agent.
#[napi]
#[derive(Debug)]
pub struct Client {
    client: thinkthen::Client,
}

#[napi]
impl Client {
    /// Admit settings once; the key is read only by the library resolver.
    #[napi(constructor)]
    pub fn new(settings: String) -> napi::Result<Self> {
        let client = thinkthen::Client::from_json(&settings, |name| std::env::var(name).ok())
            .map_err(host_error)?;
        Ok(Self { client })
    }

    /// Start one engine operation off the JavaScript thread.
    #[napi]
    pub fn start(&self, request: String, callback: JsFunction) -> napi::Result<Operation> {
        let callback: ThreadsafeFunction<Event> = callback.create_threadsafe_function(
            1,
            |context: ThreadSafeCallContext<Event>| -> napi::Result<Vec<JsUnknown>> {
                let event = context.value;
                let reply = context
                    .env
                    .create_function_from_closure("reply", move |ctx| {
                        let value = ctx.get::<JsString>(0)?.into_utf16()?;
                        // napi retains its own terminating NUL in this buffer.
                        let units = value
                            .as_slice()
                            .split_last()
                            .map_or(&[][..], |(_, text)| text);
                        let value = thinkthen::utf16_text(units);
                        // A cancelled operation has already dropped its receiver.
                        let is_text = ctx.get::<JsBoolean>(1)?.get_value()?;
                        let _ = event.reply.send((is_text, value));
                        ctx.env.get_undefined()
                    })?;
                Ok(vec![
                    context.env.create_string(event.kind)?.into_unknown(),
                    context.env.create_string(&event.data)?.into_unknown(),
                    reply.into_unknown(),
                ])
            },
        )?;
        let cancel = Cancel::default();
        let bridge = Bridge {
            callback: callback.clone(),
            cancel: cancel.clone(),
        };
        let client = self.client.clone();
        std::thread::spawn(move || {
            let result = request::run(&request, &bridge, &client);
            let data = result
                .err()
                .map_or_else(String::new, |error| error_json(&error));
            let _ = bridge.exchange("done", data);
        });
        Ok(Operation { cancel, callback })
    }
}

fn host_error(error: thinkthen::Error) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, error_json(&error))
}

fn error_json(error: &thinkthen::Error) -> String {
    serde_json::json!({"code": error.kind().code(), "message": error.to_string(), "family": error.kind().family().as_str()}).to_string()
}
