//! Pass through native packet JSON without restating its public carriers.
#![allow(clippy::print_stdout, reason = "the fixture exposes native producer readiness")]
use serde_json::{Value, json};
use std::io::Write;
use thinkthen::{EngineBuilder, Error, Request, RequestSessionRead, RequestSessionWaitInterest};

pub(crate) fn run(document: &Value) -> Result<Value, Box<dyn std::error::Error>> {
    match execute(document) {
        Ok(value) => Ok(value),
        Err(error) => Ok(json!({"packets": [], "error": error.complete()})),
    }
}

fn request(document: &Value) -> Result<Request, Error> {
    let mut selector = document.get("question").cloned().unwrap_or(Value::Null);
    let raw = selector.get("raw").and_then(Value::as_str).map(str::to_owned);
    if raw.is_some() {
        selector = json!({"kind":"definition","value":"$RAW_QUESTION"});
    }
    let value = json!({"schema":"thinkthen.request/1","call":{
        "function":document.get("verb"), "question":selector,
        "input":document.get("input"), "options":document.get("options")}});
    let mut text = value.to_string();
    if let Some(raw) = raw {
        text = text.replace("\"$RAW_QUESTION\"", &raw);
    }
    Request::from_json(&text)
}

fn execute(document: &Value) -> Result<Value, Error> {
    let mut request = request(document)?;
    let settings = document.get("settings").unwrap_or(&Value::Null).to_string();
    let engine = EngineBuilder::from_settings_json(&settings)?.build()?;
    if document.get("call_result_contract").and_then(Value::as_bool) == Some(true) {
        return scalar(&engine, &request);
    }
    let cancel = document.get("cancel").and_then(Value::as_bool) == Some(true);
    let probe = document.get("batch_probe").and_then(Value::as_bool) == Some(true);
    let held = document.get("held_cancel").and_then(Value::as_bool) == Some(true);
    let feed = if probe || cancel {
        let items = match &request.call.arguments.input {
            thinkthen::RequestInput::Records { items } => items.clone(),
            _ => Vec::new(),
        };
        request.call.arguments.input = thinkthen::RequestInput::Feed {
            name: "fixture".into(), framing: thinkthen::RequestFraming::Document,
            reading: thinkthen::ReaderOptions::default(), images: vec![],
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
        std::io::stdout().flush().map_err(|_| usage("fixture output failed"))?;
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).map_err(|_| usage("fixture input failed"))?;
    }
    std::thread::scope(|scope| {
        if let Some(items) = feed {
            let session = &session;
            scope.spawn(move || {
                for item in items {
                    if !matches!(session.try_push_with_wait(|| Ok::<_, Error>(thinkthen::RequestSessionDescriptor { item, location: None })), Ok(thinkthen::RequestSessionPushStatus::Accepted)) {
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
        let mut packets = Vec::new();
        loop {
            match session.next_packet(RequestSessionWaitInterest::Settlement)? {
                RequestSessionRead::Result(packet) => {
                    packets.push(serde_json::from_str::<Value>(&packet.to_json()?)
                        .map_err(|_| usage("invalid packet JSON"))?);
                }
                RequestSessionRead::End => return Ok(json!({"packets": packets})),
                RequestSessionRead::Pending => {}
            }
        }
    })
}

fn usage(text: &str) -> Error {
    Error::new(thinkthen::ErrorKind::Usage, text)
}

thinkthen::choices! {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "lowercase")]
    enum Team { Billing => "billing", Shipping => "shipping", Other => "other" }
}

fn scalar(engine: &thinkthen::Engine, request: &Request) -> Result<Value, Error> {
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
) -> Result<Value, Error> {
    Ok(json!({"packets": [], "host": {"native":true,
        "results":call.results(), "facts":call.facts().complete(), "call_result":call}}))
}
