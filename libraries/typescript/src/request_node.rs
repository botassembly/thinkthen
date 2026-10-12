//! Nonblocking owned Request sessions for the JavaScript scheduler.
#![allow(unsafe_code, reason = "napi emits registration glue")]
#![allow(missing_docs, reason = "napi emits public method glue")]
use crate::node::NativeEngine;
use napi::{Env, JsUnknown, Result};
use napi_derive::napi;
use thinkthen::{
    Request, RequestReaderFailure, RequestSession, RequestSessionPushStatus, RequestSessionRead,
};
fn plain(env: Env, value: &serde_json::Value) -> Result<JsUnknown> {
    Ok(match value {
        serde_json::Value::Null => env.get_null()?.into_unknown(),
        serde_json::Value::Bool(value) => env.get_boolean(*value)?.into_unknown(),
        serde_json::Value::String(value) => env.create_string(value)?.into_unknown(),
        serde_json::Value::Number(value) => env
            .create_double(value.as_f64().ok_or_else(|| {
                napi::Error::from_reason("native error number could not be represented")
            })?)?
            .into_unknown(),
        serde_json::Value::Array(values) => {
            let mut array = env.create_array_with_length(values.len())?;
            for (at, value) in values.iter().enumerate() {
                array.set_element(
                    u32::try_from(at)
                        .map_err(|_| napi::Error::from_reason("native error array is too long"))?,
                    plain(env, value)?,
                )?;
            }
            array.into_unknown()
        }
        serde_json::Value::Object(values) => {
            let mut object = env.create_object()?;
            for (name, value) in values {
                object.set_named_property(name, plain(env, value)?)?;
            }
            object.into_unknown()
        }
    })
}
fn failure(env: Env, error: thinkthen::Error) -> napi::Error {
    let built = (|| -> Result<JsUnknown> {
        let mut value = env.create_error(napi::Error::from_reason(error.to_string()))?;
        value.set_named_property("name", env.create_string("ClientError")?)?;
        value.set_named_property("kind", env.create_string(error.kind().name())?)?;
        value.set_named_property("retryable", env.get_boolean(error.retryable())?)?;
        let complete = serde_json::to_value(error.complete())
            .map_err(|_| napi::Error::from_reason("native error could not be converted"))?;
        value.set_named_property("complete", plain(env, &complete)?)?;
        value.set_named_property(
            "facts",
            plain(
                env,
                complete.get("facts").unwrap_or(&serde_json::Value::Null),
            )?,
        )?;
        Ok(value.into_unknown())
    })();
    match built {
        Ok(value) => value.into(),
        Err(error) => error,
    }
}
#[napi]
#[derive(Debug)]
pub struct NativeSession {
    session: Option<std::sync::Arc<RequestSession>>,
}
#[napi]
pub fn request_engine(env: Env, settings: String) -> Result<NativeEngine> {
    thinkthen::EngineBuilder::from_settings_json(&settings)
        .and_then(thinkthen::EngineBuilder::build)
        .map(|engine| NativeEngine { engine })
        .map_err(|error| failure(env, error))
}
/// Separate host values; native Rust owns the request variants and admission.
#[napi(object)]
#[derive(Debug)]
pub struct RequestValues {
    pub question: String,
    pub selector: bool,
    pub literal: bool,
    pub source: Option<String>,
    pub items: Vec<String>,
    pub feed: bool,
}
fn malformed(env: Env, _: serde_json::Error) -> napi::Error {
    failure(
        env,
        thinkthen::Error::new(thinkthen::ErrorKind::Usage, "invalid canonical request"),
    )
}

#[napi]
pub fn request_session(
    env: Env,
    engine: &NativeEngine,
    verb: String,
    values: RequestValues,
    options: String,
) -> Result<NativeSession> {
    use thinkthen::{RequestArguments, RequestInput, RequestQuestion};
    let question = if values.selector {
        RequestQuestion::from_json(&values.question).map_err(|error| failure(env, error))?
    } else if values.literal {
        RequestQuestion::Text {
            text: serde_json::from_str(&values.question).map_err(|error| malformed(env, error))?,
        }
    } else {
        RequestQuestion::Definition {
            value: thinkthen::RequestDefinition::from_request_json(&values.question)
                .map_err(|error| failure(env, error))?,
        }
    };
    let input = if values.feed {
        RequestInput::Feed {
            name: "javascript".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        }
    } else if let Some(source) = values.source {
        RequestInput::Source {
            source: serde_json::from_str(&source).map_err(|error| malformed(env, error))?,
        }
    } else {
        let items = values
            .items
            .iter()
            .map(|s| thinkthen::RequestItem::from_json(s).map_err(|error| failure(env, error)))
            .collect::<Result<_>>()?;
        RequestInput::Records { items }
    };
    let args = RequestArguments {
        question,
        input,
        options: serde_json::from_str(&options).map_err(|error| malformed(env, error))?,
    };
    let request = Request::named(verb.as_str(), args).map_err(|error| failure(env, error))?;
    let session = engine
        .engine
        .request_session_with_surface(request, thinkthen::Surface::Javascript)
        .map_err(|error| failure(env, error))?;
    Ok(NativeSession {
        session: Some(std::sync::Arc::new(session)),
    })
}
#[derive(Debug)]
pub struct SessionWaitTask {
    session: Option<std::sync::Arc<RequestSession>>,
    input: bool,
}
impl napi::Task for SessionWaitTask {
    type Output = ();
    type JsValue = ();
    fn compute(&mut self) -> Result<()> {
        if let Some(session) = &self.session {
            let interest = if self.input {
                thinkthen::RequestSessionWaitInterest::Input
            } else {
                thinkthen::RequestSessionWaitInterest::Settlement
            };
            let _ = session.wait(interest, std::time::Duration::from_millis(50));
        }
        Ok(())
    }
    fn resolve(&mut self, _env: Env, _: ()) -> Result<()> {
        Ok(())
    }
}
#[napi]
impl NativeSession {
    #[napi]
    pub fn wait(&self, input: bool) -> napi::bindgen_prelude::AsyncTask<SessionWaitTask> {
        napi::bindgen_prelude::AsyncTask::new(SessionWaitTask {
            session: self.session.clone(),
            input,
        })
    }
    #[napi]
    pub fn poll(&self, env: Env) -> Result<Option<String>> {
        match self.session.as_ref().map(|session| session.try_read()) {
            Some(RequestSessionRead::Result(packet)) => packet
                .to_json()
                .map(Some)
                .map_err(|error| failure(env, error)),
            Some(RequestSessionRead::Pending) => Ok(None),
            Some(RequestSessionRead::End) | None => Ok(Some("end".into())),
        }
    }
    #[napi]
    pub fn push(&self, env: Env, descriptor: String) -> Result<String> {
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
            .map_err(|error| failure(env, error))
    }
    #[napi]
    pub fn finish(&self, env: Env, error: Option<bool>) -> Result<()> {
        if let Some(session) = &self.session {
            let error = error.filter(|v| *v).map(|_| RequestReaderFailure::Producer);
            session.finish(error).map_err(|error| failure(env, error))?;
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
        if let Some(session) = self.session.take() {
            session.cancel();
        }
    }
}
