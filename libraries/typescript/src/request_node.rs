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
fn usage(message: &str) -> napi::Error {
    napi::Error::from_reason(
        serde_json::json!({"err":{"kind":"usage","message":message,"retryable":false}}).to_string(),
    )
}
fn malformed(error: serde_json::Error) -> napi::Error {
    usage(
        if error
            .to_string()
            .starts_with("per-item context is text or an object")
        {
            "the per-item context does not match context_schema"
        } else {
            "invalid canonical request"
        },
    )
}
#[napi]
pub fn request_session(
    engine: &NativeEngine,
    verb: String,
    values: RequestValues,
    options: String,
) -> Result<NativeSession> {
    use thinkthen::{RequestArguments, RequestCall, RequestInput, RequestQuestion};
    let question = if values.selector {
        serde_json::from_str(&values.question).map_err(malformed)?
    } else if values.literal {
        RequestQuestion::Text {
            text: serde_json::from_str(&values.question).map_err(malformed)?,
        }
    } else {
        RequestQuestion::Definition {
            value: thinkthen::RequestDefinition::from_request_json(&values.question)
                .map_err(failure)?,
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
            source: serde_json::from_str(&source).map_err(malformed)?,
        }
    } else {
        let items = values
            .items
            .iter()
            .map(|s| serde_json::from_str(s).map_err(malformed))
            .collect::<Result<_>>()?;
        match verb.as_str() {
            "find" => RequestInput::Units { items },
            "relate" => RequestInput::Entities { items },
            _ => RequestInput::Records { items },
        }
    };
    let args = RequestArguments {
        question,
        input,
        options: serde_json::from_str(&options).map_err(malformed)?,
    };
    let call = match verb.as_str() {
        "decide" => RequestCall::Decide(args),
        "choose" => RequestCall::Choose(args),
        "tag" => RequestCall::Tag(args),
        "score" => RequestCall::Score(args),
        "filter" => RequestCall::Filter(args),
        "rank" => RequestCall::Rank(args),
        "find" => RequestCall::Find(args),
        "annotate" => RequestCall::Annotate(args),
        "recognize" => RequestCall::Recognize(args),
        "relate" => RequestCall::Relate(args),
        _ => return Err(usage("invalid canonical request")),
    };
    let session = engine
        .engine
        .request_session_with_surface(Request::new(call), thinkthen::Surface::Javascript)
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
    pub fn finish(&self, error: Option<bool>) -> Result<()> {
        if let Some(session) = &self.session {
            let error = error
                .filter(|v| *v)
                .map(|_| RequestReaderFailure::InvalidInput { location: None });
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
