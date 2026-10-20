//! Pass through native packet JSON without restating its public carriers.
#![allow(
    clippy::print_stdout,
    reason = "the fixture exposes native producer readiness"
)]
use serde::Deserialize;
use serde_json::{json, value::RawValue};
use std::io::Write;
use thinkthen::{EngineBuilder, Error, Request, RequestSessionRead, RequestSessionWaitInterest};

#[derive(Deserialize)]
struct Document {
    verb: String,
    question: Box<RawValue>,
    input: Box<RawValue>,
    options: Box<RawValue>,
    settings: Box<RawValue>,
    #[serde(default)]
    call_result_contract: bool,
    #[serde(default)]
    cancel: bool,
    #[serde(default)]
    batch_probe: bool,
    #[serde(default)]
    held_cancel: bool,
}

pub(crate) fn run(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = serde_json::from_str(text).map_err(|_| usage("invalid packet fixture"));
    match document.and_then(execute) {
        Ok(value) => Ok(value),
        Err(error) => Ok(json!({"packets": [], "error": error.complete()}).to_string()),
    }
}

fn request(document: &Document) -> Result<Request, Error> {
    #[derive(Deserialize)]
    struct Authored {
        raw: Option<String>,
    }
    let authored: Authored = serde_json::from_str(document.question.get())
        .map_err(|_| usage("invalid fixture question"))?;
    let question = match authored.raw {
        Some(raw) => format!(r#"{{"kind":"definition","value":{raw}}}"#),
        None => document.question.get().to_owned(),
    };
    let function = serde_json::to_string(&document.verb).map_err(|_| usage("invalid function"))?;
    Request::from_json(&format!(
        r#"{{"schema":"thinkthen.request/1","call":{{"function":{function},"question":{question},"input":{},"options":{}}}}}"#,
        document.input.get(),
        document.options.get()
    ))
}

fn execute(document: Document) -> Result<String, Error> {
    let mut request = request(&document)?;
    let settings = document.settings.get();
    let engine = EngineBuilder::from_settings_json(settings)?.build()?;
    if document.call_result_contract {
        return scalar(&engine, &request);
    }
    let cancel = document.cancel;
    let probe = document.batch_probe;
    let held = document.held_cancel;
    let feed = if probe || cancel {
        let items = match &request.call.arguments.input {
            thinkthen::RequestInput::Records { items } => items.clone(),
            _ => Vec::new(),
        };
        request.call.arguments.input = thinkthen::RequestInput::Feed {
            name: "fixture".into(),
            framing: thinkthen::RequestFraming::Document,
            reading: thinkthen::ReaderOptions::default(),
            images: vec![],
        };
        Some(items)
    } else {
        None
    };
    let session = engine.request_session(request)?;
    if cancel {
        session.cancel();
    }
    if probe {
        println!("ready");
        std::io::stdout()
            .flush()
            .map_err(|_| usage("fixture output failed"))?;
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .map_err(|_| usage("fixture input failed"))?;
    }
    std::thread::scope(|scope| {
        if let Some(items) = feed {
            let session = &session;
            scope.spawn(move || {
                for item in items {
                    if !matches!(
                        session.push_with(|| Ok::<_, Error>(thinkthen::RequestSessionDescriptor {
                            item,
                            location: None
                        })),
                        Ok(thinkthen::RequestSessionPushStatus::Accepted)
                    ) {
                        break;
                    }
                }
                let _ = session.finish(None);
            });
        }
        if held {
            let session = &session;
            scope.spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(150));
                session.cancel();
            });
        }
        drain(&session)
    })
}

fn drain(session: &thinkthen::RequestSession) -> Result<String, Error> {
    let mut packets = Vec::new();
    loop {
        match session.next_packet(RequestSessionWaitInterest::Settlement)? {
            RequestSessionRead::Result(packet) => packets.push(packet.to_json()?),
            RequestSessionRead::End => {
                return Ok(format!(r#"{{"packets":[{}]}}"#, packets.join(",")));
            }
            RequestSessionRead::Pending => {}
        }
    }
}

fn usage(text: &str) -> Error {
    Error::new(thinkthen::ErrorKind::Usage, text)
}

thinkthen::choices! {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "lowercase")]
    enum Team { Billing => "billing", Shipping => "shipping", Other => "other" }
}

fn scalar(engine: &thinkthen::Engine, request: &Request) -> Result<String, Error> {
    let thinkthen::RequestInput::Records { items } = &request.call.arguments.input else {
        return Err(usage("scalar fixture requires one text record"));
    };
    let [item] = items.as_slice() else {
        return Err(usage("scalar fixture requires one text record"));
    };
    let Some(thinkthen::RequestOriginal::Text { text }) = &item.original else {
        return Err(usage("scalar fixture requires text evidence"));
    };
    let admitted = request.clone().admit()?;
    let thinkthen::RequestDefinition::Atomic(question) = admitted.resolve_question()? else {
        return Err(usage("scalar fixture requires an atomic question"));
    };
    match request.call.function {
        thinkthen::RequestFunction::Decide => converted(&engine.decide(&question, text)?),
        thinkthen::RequestFunction::Choose => {
            let thinkthen::LoadedQuestion::Question(question) = question else {
                return Err(usage("scalar choose fixture requires an unbanded question"));
            };
            converted(&engine.choose(&question.into_choose::<Team>()?, text)?)
        }
        _ => Err(usage("scalar fixture requires decide or choose")),
    }
}

fn converted<T: serde::Serialize, R: serde::Serialize, P: serde::Serialize>(
    call: &thinkthen::Call<T, R, P>,
) -> Result<String, Error> {
    Ok(json!({"packets": [], "host": {"native":true,
        "results":call.results(), "facts":call.facts().complete(), "call_result":call}})
    .to_string())
}
