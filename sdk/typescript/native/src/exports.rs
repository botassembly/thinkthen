//! Node classes and their macro-generated support methods.

use super::{Bridge, Event, Replies, request};
use napi::threadsafe_function::{ThreadSafeCallContext, ThreadsafeFunction};
use napi::{Env, JsBoolean, JsFunction, JsString, JsUnknown};
use napi_derive::napi;
use std::sync::{Arc, OnceLock, mpsc::channel};
use thinkthen::Cancel;

/// Reject a JavaScript number that has no JSON representation.
///
/// # Errors
/// Always returns the library's record encoding error.
#[napi]
pub fn reject_non_finite_json() -> napi::Result<()> {
    Err(host_error(thinkthen::native::record_conversion_error()))
}

/// Reject host settings that cannot enter the canonical representation.
/// # Errors
/// Always returns the library's settings conversion error.
#[napi]
pub fn reject_settings_conversion() -> napi::Result<()> {
    Err(host_error(thinkthen::native::settings_conversion_error()))
}

/// Reject a host question that cannot enter the canonical representation.
/// # Errors
/// Always returns the library's question conversion error.
#[napi]
pub fn reject_question_conversion() -> napi::Result<()> {
    Err(host_error(thinkthen::native::question_conversion_error()))
}

/// Return the engine cancellation error without starting an operation.
/// # Errors
/// Always returns the library cancellation error.
#[napi]
pub fn reject_cancellation() -> napi::Result<()> {
    Err(host_error(thinkthen::native::cancelled_error()))
}

/// A cancellation handle for the one running stream.
#[napi]
pub struct Operation {
    cancel: Cancel,
    callback: Option<ThreadsafeFunction<Event>>,
    replies: Arc<Replies>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl Drop for Operation {
    fn drop(&mut self) {
        self.cancel();
        self.close();
    }
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
        self.callback
            .as_mut()
            .map_or(Ok(()), |callback| callback.refer(&env))
    }

    /// Let Node exit while iteration is suspended at a delivered row.
    #[napi]
    pub fn unreference(&mut self, env: Env) -> napi::Result<()> {
        self.callback
            .as_mut()
            .map_or(Ok(()), |callback| callback.unref(&env))
    }

    /// Release the callback after the native completion event.
    #[napi]
    pub fn close(&mut self) {
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
        self.callback = None;
    }

    /// Stop input, new requests and retry waits.
    #[napi]
    pub fn cancel(&self) {
        self.cancel.cancel();
        self.replies.send("cancel", None);
    }
}

/// A client retains its backend engine and shared HTTP agent.
#[napi]
#[derive(Debug)]
pub struct Client {
    client: Option<thinkthen::Engine>,
}

#[napi]
impl Client {
    /// Admit settings once; the key is read only by the library resolver.
    #[napi(constructor)]
    pub fn new(settings: String) -> napi::Result<Self> {
        let client = thinkthen::Settings::from_json(&settings)
            .and_then(|settings| settings.engine(|name| std::env::var(name).ok()))
            .map_err(host_error)?;
        Ok(Self {
            client: Some(client),
        })
    }

    /// Release client settings and connections after active operations finish.
    #[napi]
    pub fn close(&mut self) {
        self.client = None;
    }

    /// Start one lazy pulled operation off the JavaScript thread.
    #[napi]
    pub fn start(&self, request: String, callback: JsFunction) -> napi::Result<Operation> {
        start(self.engine()?, request, callback, false)
    }

    /// Admit a finite collection before opening the same bounded native driver.
    #[napi]
    pub fn start_finite(&self, request: String, callback: JsFunction) -> napi::Result<Operation> {
        start(self.engine()?, request, callback, true)
    }
}

impl Client {
    fn engine(&self) -> napi::Result<thinkthen::Engine> {
        self.client
            .clone()
            .ok_or_else(|| host_error(thinkthen::native::closed_client_error()))
    }
}

fn start(
    client: thinkthen::Engine,
    request: String,
    callback: JsFunction,
    finite: bool,
) -> napi::Result<Operation> {
    let callback: ThreadsafeFunction<Event> = callback.create_threadsafe_function(
        0,
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
                    let invalid = ctx.length > 2 && ctx.get::<JsBoolean>(2)?.get_value()?;
                    let is_text = ctx.get::<JsBoolean>(1)?.get_value()?;
                    let id = if ctx.length > 4 {
                        Some((
                            ctx.get::<JsString>(3)?.into_utf8()?.as_str()?.to_owned(),
                            ctx.get::<JsString>(4)?.into_utf8()?.as_str()?.to_owned(),
                        ))
                    } else {
                        None
                    };
                    let record = input_record(units, is_text, invalid, id);
                    // A cancelled operation has already dropped its receiver.
                    event.replies.send(event.kind, record);
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
    let (sender, receive) = channel();
    let replies = Arc::new(Replies {
        sender,
        events: OnceLock::new(),
    });
    let bridge = Bridge {
        callback: callback.clone(),
        cancel: cancel.clone(),
        replies: replies.clone(),
        receive,
    };
    let worker = std::thread::Builder::new()
        .name("thinkthen-node".to_owned())
        .spawn(move || {
            let result = request::run(&request, &bridge, &client, finite);
            let data = result
                .err()
                .map_or_else(String::new, |error| error_json(&error));
            bridge.publish("done", data);
        })
        .map_err(|_| host_error(thinkthen::native::internal_error()))?;
    Ok(Operation {
        cancel,
        callback: Some(callback),
        replies,
        worker: Some(worker),
    })
}

fn host_error(error: thinkthen::Error) -> napi::Error {
    napi::Error::new(napi::Status::GenericFailure, error_json(&error))
}

fn error_json(error: &thinkthen::Error) -> String {
    serde_json::json!({"code": error.code().as_str(), "message": error.to_string(), "family": error.code().family().as_str()}).to_string()
}

/// Report the direct native module's contract before any client creation.
#[napi]
pub fn abi_version() -> u32 {
    thinkthen::ABI_VERSION
}

fn input_record(
    units: &[u16],
    text: bool,
    invalid: bool,
    id: Option<(String, String)>,
) -> Option<thinkthen::records::Record> {
    use thinkthen::records::{Record, RecordId};
    if invalid {
        return Some(Record::invalid());
    }
    let Ok(value) = String::from_utf16(units) else {
        return Some(Record::invalid());
    };
    if !text && value.is_empty() && id.is_none() {
        return None;
    }
    if let Some((kind, id)) = id {
        let id = match kind.as_str() {
            "string" => RecordId::from(id),
            "integer" => match RecordId::integer_decimal(&id) {
                Ok(id) => id,
                Err(_) => return Some(Record::invalid()),
            },
            _ => return Some(Record::invalid()),
        };
        return Some(if text {
            Record::text(value).with_id(id)
        } else {
            Record::from_json_with_id(&value, id)
        });
    }
    Some(if text {
        Record::text(value)
    } else {
        Record::from_json(&value)
    })
}

/// Admit saved toolbox JSON and return canonical flat question definitions.
/// # Errors
/// Returns the shared toolbox usage error.
#[napi]
pub fn parse_toolbox(json: String) -> napi::Result<String> {
    let toolbox = thinkthen::Toolbox::from_json(&json).map_err(host_error)?;
    toolbox.definitions_json().map_err(host_error)
}

/// Admit flat question definitions and save the command's toolbox JSON.
/// # Errors
/// Returns the shared question or saved-name admission error.
#[napi]
pub fn serialize_toolbox(json: String) -> napi::Result<String> {
    thinkthen::Toolbox::from_definitions_json(&json)
        .and_then(|toolbox| toolbox.to_json())
        .map_err(host_error)
}
