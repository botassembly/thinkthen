//! Private bounded native sessions for the named Ruby calls.
use super::{EngineValue, Fault, checked};
use magnus::{Error, RClass, RModule, Ruby, TryConvert, Value, function, method, prelude::*};
use std::sync::{Arc, Mutex, PoisonError};
use thinkthen::{
    Request, RequestReaderFailure, RequestSession, RequestSessionPush, RequestSessionRead,
};
#[magnus::wrap(class = "ThinkThen::Native::RequestItem", free_immediately, size)]
#[derive(Debug)]
struct Item(thinkthen::RequestItem);
#[magnus::wrap(class = "ThinkThen::Native::RequestSession", free_immediately, size)]
#[derive(Debug)]
struct Session(Mutex<Option<Arc<RequestSession>>>);
impl Session {
    fn wait(ruby: &Ruby, this: &Self, input: bool) -> Result<(), Error> {
        let session = this
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        if let Some(session) = session {
            super::session_wait::wait(ruby, session, input)?;
        }
        Ok(())
    }
    fn poll(ruby: &Ruby, this: &Self) -> Result<Option<Value>, Error> {
        let held = this.0.lock().unwrap_or_else(PoisonError::into_inner);
        match held.as_ref().map(|session| session.try_read()) {
            Some(RequestSessionRead::Result(packet)) => {
                let json = checked(ruby, packet.to_json().map_err(Fault::from))?;
                super::native_result::packet(ruby, &json).map(Some)
            }
            Some(RequestSessionRead::Pending) => Ok(None),
            Some(RequestSessionRead::End) | None => Ok(Some(ruby.into_value("end"))),
        }
    }
    fn item(ruby: &Ruby, _this: &Self, descriptor: Value) -> Result<Item, Error> {
        item(ruby, descriptor).map(Item)
    }
    fn push(ruby: &Ruby, this: &Self, descriptor: &Item) -> Result<&'static str, Error> {
        let held = this.0.lock().unwrap_or_else(PoisonError::into_inner);
        let Some(session) = held.as_ref() else {
            return Ok("closed");
        };
        checked(
            ruby,
            session
                .try_push(thinkthen::RequestSessionDescriptor {
                    item: descriptor.0.clone(),
                    location: None,
                })
                .map_err(Fault::from),
        )
        .map(|s| match s {
            RequestSessionPush::Accepted => "accepted",
            RequestSessionPush::Full(_) => "full",
            RequestSessionPush::Closed(_) => "closed",
        })
    }
    fn finish(ruby: &Ruby, this: &Self, failure: Option<bool>) -> Result<(), Error> {
        let held = this.0.lock().unwrap_or_else(PoisonError::into_inner);
        let session = checked(
            ruby,
            held.as_ref()
                .ok_or_else(|| Fault::usage("request session is closed")),
        )?;
        let failure = failure
            .filter(|v| *v)
            .map(|_| RequestReaderFailure::Producer);
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
        if let Some(session) = self.0.lock().unwrap_or_else(PoisonError::into_inner).take() {
            session.cancel();
        }
    }
}
// Decode authored definitions, record descriptors and controls separately. The
// host never serializes a Request or chooses a native input/call variant.
fn client(ruby: &Ruby) -> Result<RClass, Error> {
    let module: RModule = ruby.class_object().const_get("ThinkThen")?;
    module.const_get("Client")
}
fn decode<T: serde::de::DeserializeOwned>(ruby: &Ruby, value: Value) -> Result<T, Error> {
    let text: String = client(ruby)?.funcall("dump", (value,))?;
    checked(
        ruby,
        serde_json::from_str(&text).map_err(|_| Fault::usage("invalid canonical request")),
    )
}
fn item(ruby: &Ruby, value: Value) -> Result<thinkthen::RequestItem, Error> {
    let text: String = client(ruby)?.funcall("dump", (value,))?;
    checked(
        ruby,
        thinkthen::RequestItem::from_json(&text).map_err(Fault::from),
    )
}

fn build(
    ruby: &Ruby,
    verb: &str,
    question: Value,
    input: Value,
    options: Value,
    feed: bool,
) -> Result<Request, Error> {
    use thinkthen::{RequestArguments, RequestInput, RequestQuestion};
    let client = client(ruby)?;
    let selector: RClass = client.const_get("Question")?;
    let source: RClass = client.const_get("Source")?;
    let question = if question.is_kind_of(selector) {
        {
            let text: String =
                client.funcall("dump", (question.funcall::<_, _, Value>("selector", ())?,))?;
            checked(
                ruby,
                thinkthen::RequestQuestion::from_json(&text).map_err(Fault::from),
            )?
        }
    } else if question.is_kind_of(ruby.class_string()) {
        RequestQuestion::Text {
            text: checked(
                ruby,
                String::try_convert(question)
                    .map_err(|_| Fault::usage("input cannot be converted to native JSON")),
            )?,
        }
    } else {
        RequestQuestion::Definition {
            value: {
                let text: String = client.funcall("dump", (question,))?;
                checked(
                    ruby,
                    thinkthen::RequestDefinition::from_request_json(&text).map_err(Fault::from),
                )?
            },
        }
    };
    let input = if feed {
        RequestInput::Feed {
            name: "ruby".into(),
            framing: Default::default(),
            reading: Default::default(),
            images: Vec::new(),
        }
    } else if input.is_kind_of(source) {
        RequestInput::Source {
            source: decode(ruby, input.funcall("source", ())?)?,
        }
    } else {
        let item = |value| {
            let descriptor: Value = client.funcall("descriptor", (value,))?;
            item(ruby, descriptor)
        };
        let items = if let Some(array) = magnus::RArray::from_value(input) {
            array.into_iter().map(item).collect::<Result<_, Error>>()?
        } else {
            vec![item(input)?]
        };
        RequestInput::Records { items }
    };
    let args = RequestArguments {
        question,
        input,
        options: decode(ruby, options)?,
    };
    let request = checked(ruby, Request::named(verb, args).map_err(Fault::from))?;
    Ok(request)
}
fn start(
    ruby: &Ruby,
    engine: &EngineValue,
    verb: String,
    question: Value,
    input: Value,
    controls: (Value, bool),
) -> Result<Session, Error> {
    let request = build(ruby, &verb, question, input, controls.0, controls.1)?;
    let session = checked(
        ruby,
        engine
            .engine
            .request_session_with_surface(request, thinkthen::Surface::Ruby)
            .map_err(Fault::from),
    )?;
    Ok(Session(Mutex::new(Some(Arc::new(session)))))
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
fn plan(
    ruby: &Ruby,
    engine: &EngineValue,
    verb: String,
    question: Value,
    input: Value,
    controls: (Value, bool),
) -> Result<Value, Error> {
    let request = checked(
        ruby,
        build(ruby, &verb, question, input, controls.0, controls.1)?
            .admit()
            .map_err(Fault::from),
    )?;
    let planned = checked(
        ruby,
        engine
            .engine
            .plan_request(&request, thinkthen::RequestEnvironment::default())
            .map_err(Fault::from),
    )?;
    let json = checked(
        ruby,
        serde_json::to_value(planned).map_err(|_| {
            Fault::of(
                thinkthen::ErrorKind::Defect,
                "native preview could not be read",
            )
        }),
    )?;
    super::native_result::convert(ruby, "completeplan", &json)
}
pub(super) fn register(ruby: &Ruby, native: RModule, engine: RClass) -> Result<(), Error> {
    native.define_module_function("request_engine", function!(request_engine, 1))?;
    native.define_class("RequestItem", ruby.class_object())?;
    let session = native.define_class("RequestSession", ruby.class_object())?;
    session.define_method("wait", method!(Session::wait, 1))?;
    session.define_method("poll", method!(Session::poll, 0))?;
    session.define_method("item", method!(Session::item, 1))?;
    session.define_method("push", method!(Session::push, 1))?;
    session.define_method("finish", method!(Session::finish, 1))?;
    session.define_method("cancel", method!(Session::cancel, 0))?;
    session.define_method("close", method!(Session::close, 0))?;
    engine.define_method("request_session", method!(start, 4))?;
    engine.define_method("plan", method!(plan, 4))?;
    Ok(())
}
