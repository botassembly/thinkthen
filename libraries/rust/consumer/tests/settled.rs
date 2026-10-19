//! The real Rust consumer reads final native answers from the shared loopback cases.
use conformance_backend::{Backend, CASES};
use serde::Deserialize;
use serde_json::{Value, json, value::RawValue};
use thinkthen::{
    BatchSetting, Engine, Request, RequestEnvironment, RequestFeed, RequestInput,
    RequestSessionRead, RequestSessionResult, RequestSessionWaitInterest,
};

#[derive(Deserialize)]
struct Cases {
    cases: Vec<Case>,
}
#[derive(Deserialize)]
struct Case {
    id: String,
    verb: String,
    question: Option<Box<RawValue>>,
    question_set: Option<Box<RawValue>>,
    #[serde(flatten)]
    rest: serde_json::Map<String, Value>,
}
fn failure(error: impl std::fmt::Display) -> String {
    error.to_string()
}
fn engine(backend: &Backend, id: &str) -> Result<Engine, String> {
    Engine::builder()
        .base_url(&format!("{}/case/{id}/v1", backend.origin()))
        .and_then(|builder| builder.api_key("sk-settled-loopback"))
        .map(|builder| {
            builder
                .no_cache()
                .max_retries(0)
                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
        })
        .and_then(thinkthen::EngineBuilder::build)
        .map_err(failure)
}
#[expect(
    clippy::indexing_slicing,
    reason = "shared fixture fields and expected outputs must exist or fail the consumer test"
)]
fn request(case: &Case, collection: bool) -> Result<Request, String> {
    let raw = case
        .question_set
        .as_ref()
        .or(case.question.as_ref())
        .ok_or("missing question")?;
    let mut question: Value = serde_json::from_str(raw.get()).map_err(failure)?;
    let mut options = json!({});
    let mut texts = if let Some(units) = question.get("units").cloned() {
        question
            .as_object_mut()
            .ok_or("invalid find question")?
            .remove("units");
        options["none"] = question
            .as_object_mut()
            .ok_or("invalid find question")?
            .remove("none")
            .unwrap_or(json!(false));
        units.as_array().ok_or("invalid units")?.clone()
    } else if let Some(entities) = case.rest.get("entities") {
        entities.as_array().ok_or("invalid entities")?.clone()
    } else if let Some(text) = case.rest.get("text") {
        vec![text.clone()]
    } else {
        case.rest["exchanges"]
            .as_array()
            .ok_or("missing exchanges")?
            .iter()
            .map(|exchange| exchange["evidence"].clone())
            .collect()
    };
    if !collection && matches!(case.verb.as_str(), "filter" | "rank") {
        texts.truncate(1);
    }
    let input = if !collection
        && texts.len() == 1
        && !matches!(case.verb.as_str(), "rank" | "find" | "relate")
    {
        json!({"kind":"text","text":texts.first().ok_or("missing input")?})
    } else {
        let items = texts
            .iter()
            .map(|text| {
                if text.is_string() {
                    json!({"original":{"kind":"text","text":text}})
                } else {
                    json!({"original":{"kind":"json","value":text}})
                }
            })
            .collect::<Vec<_>>();
        json!({"kind": if !collection && case.verb == "find" { "units" } else if !collection && case.verb == "relate" { "entities" } else { "records" },"items":items})
    };
    // Preserve question-set member order exactly as the shared request recording.
    let question = if case.verb == "find" {
        question.to_string()
    } else {
        raw.get().to_owned()
    };
    let verb = serde_json::to_string(&case.verb).map_err(failure)?;
    Request::from_json(&format!(r#"{{"schema":"thinkthen.request/1","call":{{"function":{verb},"question":{{"kind":"definition","value":{question}}},"input":{input},"options":{options}}}}}"#)).map_err(failure)
}
fn terminal(engine: &Engine, request: Request) -> Result<Value, String> {
    let session = engine.request_session(request).map_err(failure)?;
    loop {
        match session
            .next_packet(RequestSessionWaitInterest::Settlement)
            .map_err(failure)?
        {
            RequestSessionRead::Result(packet @ RequestSessionResult::Terminal(_)) => {
                return serde_json::from_str(&packet.to_json().map_err(failure)?).map_err(failure);
            }
            RequestSessionRead::Result(_) => {}
            read => return Err(format!("terminal packet missing: {read:?}")),
        }
    }
}
fn expected(case: &Case, collection: bool) -> Result<Value, String> {
    case.rest
        .get("expect")
        .and_then(|expect| expect.get("settled"))
        .and_then(|settled| settled.get(if collection { "collection" } else { "single" }))
        .cloned()
        .ok_or_else(|| format!("{}: missing fixed settled answer", case.id))
}

#[test]
fn all_functions_settle_shared_answers_in_final_packets() -> Result<(), String> {
    let cases: Cases = serde_json::from_str(CASES).map_err(failure)?;
    let backend = Backend::start().map_err(failure)?;
    for id in [
        "04-decide-band-yes",
        "06-choose-billing",
        "07-choose-unsure",
        "09-tag-two",
        "11-score-middle",
        "13-filter-records",
        "15-rank-records",
        "17-annotate-mixed",
        "17-annotate-partial",
        "18-find-second",
        "19-find-none",
        "41-offsets-past-an-accent-and-an-emoji",
        "51-same-kind-alerts",
    ] {
        let case = cases
            .cases
            .iter()
            .find(|case| case.id == id)
            .ok_or("shared case missing")?;
        let engine = engine(&backend, id)?;
        for collection in [false, true] {
            let request = request(case, collection)?;
            let expected: Value =
                serde_json::from_str(&expected(case, collection)?.to_string()).map_err(failure)?;
            let call = engine
                .call(
                    &case.verb,
                    request.call.arguments.clone(),
                    RequestEnvironment::default(),
                )
                .map_err(failure)?;
            let answer: Value =
                serde_json::from_str(&serde_json::to_string(&call.answer()).map_err(failure)?)
                    .map_err(failure)?;
            assert_eq!(answer, expected, "{id}, collection={collection}");
            if collection {
                let mut arguments = request.call.arguments.clone();
                let items = match arguments.input {
                    RequestInput::Records { items }
                    | RequestInput::Units { items }
                    | RequestInput::Entities { items } => items,
                    RequestInput::Text { text, images } => vec![
                        thinkthen::RequestItem::from_json(
                            &json!({"original":{"kind":"text","text":text},"images":images})
                                .to_string(),
                        )
                        .map_err(failure)?,
                    ],
                    _ => return Err("unexpected fixture input".into()),
                };
                arguments.input = RequestInput::Feed {
                    name: "caller".into(),
                    framing: Default::default(),
                    reading: Default::default(),
                    images: vec![],
                };
                let call = engine
                    .call(
                        &case.verb,
                        arguments,
                        RequestEnvironment {
                            controls: Default::default(),
                            feed: Some(RequestFeed::new("caller", items.into_iter().map(Ok))),
                        },
                    )
                    .map_err(failure)?;
                let answer: Value =
                    serde_json::from_str(&serde_json::to_string(&call.answer()).map_err(failure)?)
                        .map_err(failure)?;
                assert_eq!(answer, expected, "fed {id}");
            }
            let packet = terminal(&engine, request)?;
            assert_eq!(packet["resolution"], "resolved", "{id}");
            assert_eq!(
                packet["answer"], expected,
                "final {id}, collection={collection}"
            );
        }
    }
    Ok(())
}

#[test]
fn authored_null_unresolved_and_failed_answers_stay_distinct() -> Result<(), String> {
    for (question, reply, resolution, terminal_resolution) in [
        (
            json!({"decide":"Fits?","true":null}),
            json!({"q1":{"type":"noul","noul":0.9}}),
            "resolved",
            "resolved",
        ),
        (
            json!({"decide":"Fits?","threshold":"0.1:0.9"}),
            json!({"q1":{"type":"noul","noul":0.5}}),
            "unresolved",
            "resolved",
        ),
        (json!({"decide":"Fits?"}), json!({}), "failed", "failed"),
    ] {
        let response = json!({"model":"fixed","answers":reply}).to_string();
        let listener = conformance_backend::Listener::answering(move |_| {
            conformance_backend::Canned::ok(&response)
        })
        .map_err(failure)?;
        let engine = Engine::builder()
            .base_url(listener.base())
            .and_then(|builder| builder.api_key("sk-null-loopback"))
            .map(|builder| builder.no_cache().max_retries(0))
            .and_then(thinkthen::EngineBuilder::build)
            .map_err(failure)?;
        let request = Request::from_json(&json!({"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"definition","value":question},"input":{"kind":"json","value":null}}}).to_string()).map_err(failure)?;
        let packet = terminal(&engine, request)?;
        assert_eq!(packet["resolution"], terminal_resolution);
        if resolution == "failed" {
            assert_eq!(packet["failure"]["resolution"], "failed");
        } else {
            assert_eq!(packet["answer"]["value"], Value::Null);
            assert_eq!(packet["answer"]["resolution"], resolution);
        }
    }
    Ok(())
}

#[test]
fn recovered_failures_keep_their_place_beside_unresolved_and_resolved_rows() -> Result<(), String> {
    let listener = conformance_backend::Listener::answering(|body| {
        let text = String::from_utf8_lossy(body);
        let answers = if text.contains("failed row") { json!({}) }
            else { json!({"q1":{"type":"noul","noul":if text.contains("uncertain row") { 0.5 } else { 0.9 }}}) };
        conformance_backend::Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
    }).map_err(failure)?;
    let engine = Engine::builder()
        .base_url(listener.base())
        .and_then(|builder| builder.api_key("sk-recovered-loopback"))
        .map(|builder| {
            builder
                .no_cache()
                .max_retries(0)
                .batch(BatchSetting::Records(std::num::NonZeroUsize::MIN))
        })
        .and_then(thinkthen::EngineBuilder::build)
        .map_err(failure)?;
    let request = Request::from_json(&json!({"schema":"thinkthen.request/1","call":{"function":"decide","question":{"kind":"definition","value":{"decide":"Fits?","threshold":"0.1:0.9"}},"input":{"kind":"records","items":[{"original":{"kind":"text","text":"failed row"}},{"original":{"kind":"text","text":"uncertain row"}},{"original":{"kind":"text","text":"resolved row"}}]}}}).to_string()).map_err(failure)?;
    let expected = json!({"function":"decide","value":[null,null,true],"probability":[null,null,0.9],"pick":[null,null,null],"index":[null,null,null],"resolution":["failed","unresolved","resolved"]});
    let admitted = request.clone().admit().map_err(failure)?;
    let call = engine
        .execute_request_recovering(&admitted, RequestEnvironment::default())
        .map_err(failure)?;
    assert_eq!(
        serde_json::to_value(call.answer()).map_err(failure)?,
        expected
    );
    let session = engine
        .start_request_recovering_session(request, thinkthen::Surface::Rust)
        .map_err(failure)?;
    loop {
        match session
            .next_packet(RequestSessionWaitInterest::Settlement)
            .map_err(failure)?
        {
            thinkthen::RequestRecoveringRead::Result(
                thinkthen::RequestRecoveringResult::Terminal(terminal),
            ) => {
                assert!(terminal.error.is_none());
                assert_eq!(
                    serde_json::to_value(terminal.answer).map_err(failure)?,
                    expected
                );
                break;
            }
            thinkthen::RequestRecoveringRead::Result(_) => {}
            read => return Err(format!("recovering terminal missing: {read:?}")),
        }
    }
    assert_eq!(listener.count(), 6);
    Ok(())
}
