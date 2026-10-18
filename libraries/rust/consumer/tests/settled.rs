//! The real Rust consumer reads final native answers from the shared loopback cases.
use conformance_backend::{Backend, CASES};
use serde::Deserialize;
use serde_json::{Value, json, value::RawValue};
use thinkthen::{
    BatchSetting, Engine, Request, RequestEnvironment, RequestOutcome, RequestSessionRead,
    RequestSessionResult, RequestSessionWaitInterest,
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
    let texts = if let Some(units) = question.get("units").cloned() {
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
    let input = if !collection && texts.len() == 1 {
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
        json!({"kind":"records","items":items})
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
fn shaped(value: Value, collection: bool) -> Value {
    if collection { json!([value]) } else { value }
}
#[expect(
    clippy::indexing_slicing,
    reason = "shared fixture fields and expected outputs must exist or fail the consumer test"
)]
fn expected(case: &Case, collection: bool) -> Result<Value, String> {
    let success = &case.rest["expect"]["success"];
    let answers = success["answers"]
        .as_array()
        .ok_or("missing expectations")?;
    let first = answers.first().ok_or("missing first expectation")?;
    let bare = first["bare"].clone();
    let detail = &first["details"]["answer"];
    let mut expected = json!({"function":case.verb,"value":bare,"probability":null,"pick":null,"index":null,"resolution":"resolved"});
    match case.verb.as_str() {
        "decide" | "choose" | "tag" | "score" => {
            if matches!(case.verb.as_str(), "decide" | "choose") {
                if bare.is_null() {
                    expected["resolution"] = json!("unresolved");
                } else if case.verb == "decide" {
                    expected["probability"] = detail["probability"].clone();
                } else {
                    expected["probability"] = detail["probabilities"]
                        [detail["pick"].as_str().ok_or("missing pick")?]
                    .clone();
                }
            }
            if case.verb == "choose" {
                expected["pick"] = detail["pick"].clone();
            }
            if case.verb == "score" {
                expected["pick"] = detail["level"].clone();
            }
            for key in ["value", "probability", "pick", "index", "resolution"] {
                expected[key] = shaped(expected[key].clone(), collection);
            }
        }
        "annotate" => {
            let mut values = serde_json::Map::new();
            let mut resolutions = serde_json::Map::new();
            for answer in answers {
                let name = answer["name"].as_str().ok_or("missing member name")?;
                let bare = &answer["bare"];
                resolutions.insert(
                    name.to_owned(),
                    json!(if bare.get("failed").is_some() {
                        "failed"
                    } else if bare.is_null() {
                        "unresolved"
                    } else {
                        "resolved"
                    }),
                );
                if bare.get("failed").is_none() {
                    values.insert(name.to_owned(), bare.clone());
                }
            }
            expected["value"] = Value::Object(values);
            expected["resolution"] = Value::Object(resolutions);
            for key in ["value", "probability", "pick", "index", "resolution"] {
                expected[key] = shaped(expected[key].clone(), collection);
            }
        }
        "filter" => {
            let indexes = success["operation"]["indexes"]
                .as_array()
                .ok_or("missing selected inputs")?;
            let indexes = indexes
                .iter()
                .map(|index| {
                    index
                        .as_u64()
                        .and_then(|at| usize::try_from(at).ok())
                        .ok_or("invalid index")
                })
                .collect::<Result<Vec<_>, _>>()?;
            expected["value"] = json!(
                indexes
                    .iter()
                    .map(|index| case.rest["exchanges"][*index]["evidence"].clone())
                    .collect::<Vec<_>>()
            );
            expected["probability"] = json!(
                indexes
                    .iter()
                    .map(|index| answers[*index]["details"]["answer"]["probability"].clone())
                    .collect::<Vec<_>>()
            );
        }
        "rank" => {
            let rows = success["operation"]["ranking"]
                .as_array()
                .ok_or("missing ranking")?;
            expected["value"] = json!(
                rows.iter()
                    .map(|row| row["index"].clone())
                    .collect::<Vec<_>>()
            );
            expected["probability"] = json!(
                rows.iter()
                    .map(|row| row["probability"].clone())
                    .collect::<Vec<_>>()
            );
        }
        "find" => {
            let index = &success["operation"]["selected"];
            expected["pick"] = detail["pick"].clone();
            expected["index"] = index.clone();
            expected["value"] = match index.as_u64().and_then(|at| usize::try_from(at).ok()) {
                Some(index) => serde_json::from_str::<Value>(
                    case.question.as_ref().ok_or("missing find question")?.get(),
                )
                .map_err(failure)?["units"][index]
                    .clone(),
                None => Value::Null,
            };
            if !index.is_null() {
                expected["probability"] = detail["probabilities"]
                    [detail["pick"].as_str().ok_or("missing find pick")?]
                .clone();
            }
        }
        "recognize" => {
            // specification/recognize.md retains the unrounded valid-path posterior, before kind strength.
            expected["probability"] = json!([0.9976489290347367]);
            for key in ["value", "probability", "pick", "index", "resolution"] {
                expected[key] = shaped(expected[key].clone(), collection);
            }
        }
        "relate" => {
            expected["probability"] = json!(
                bare.as_array()
                    .ok_or("missing edges")?
                    .iter()
                    .map(|edge| edge["probability"].clone())
                    .collect::<Vec<_>>()
            );
        }
        other => return Err(format!("unexpected function {other}")),
    }
    Ok(expected)
}

#[test]
#[expect(
    clippy::indexing_slicing,
    reason = "shared fixture fields and expected outputs must exist or fail the consumer test"
)]
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
            let expected = expected(case, collection)?;
            let admitted = request.clone().admit().map_err(failure)?;
            let call = match engine
                .execute_request(&admitted, RequestEnvironment::default())
                .map_err(failure)?
            {
                RequestOutcome::Complete(call) => call,
                RequestOutcome::Failed { error, .. } => return Err(format!("{id}: {error}")),
            };
            let answer = serde_json::to_value(call.answer()).map_err(failure)?;
            assert_eq!(answer, expected, "{id}, collection={collection}");
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
