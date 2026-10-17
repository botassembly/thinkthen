//! Current typed projections retain originals and execution facts without another send.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "invalid isolated fixture setup must stop the regression"
)]
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use thinkthen::RequestFunction;
use thinkthen::*;

fn engine(listener: &Listener) -> Engine {
    crate::api::loopback(listener.base(), "projection-private")
}
fn response(body: &[u8]) -> Canned {
    let body: Value = serde_json::from_slice(body).unwrap();
    let answers = body["questions"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(name, q)| {
            let answer = if q["type"] == "noul" {
                json!({"type":"noul","noul":0.9})
            } else {
                let labels: Vec<String> = match &q["criteria"] {
                    Value::Object(fields) => fields.keys().cloned().collect(),
                    Value::Array(values) => (0..values.len()).map(|n| n.to_string()).collect(),
                    _ => panic!("fixture criteria"),
                };
                let probabilities = labels
                    .iter()
                    .enumerate()
                    .map(|(index, label)| {
                        (
                            label.clone(),
                            json!([0.9, 0.1].get(index).copied().unwrap_or(0.1)),
                        )
                    })
                    .collect::<serde_json::Map<_, _>>();
                json!({"type":q["type"],"probabilities":probabilities,"confidence":0.8})
            };
            (name.clone(), answer)
        })
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"fixed","answers":answers,"usage":{"input_tokens":9}}).to_string())
}
fn record() -> RecordInput<QuestionInput> {
    RecordReading::new(&[], None, None)
        .unwrap()
        .compose(RawRecord::json(r#"{"body":"Sensitive original."}"#).unwrap())
        .unwrap()
        .map_original(|record| record.question_input())
}
#[test]
fn annotation_primitive_null_and_failed_members_retain_values() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"wrong":1}},"q3":{"type":"noul","noul":0.5}}}"#)).unwrap();
    let engine = engine(&listener);
    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"good":{"decide":"Good?","true":{"authored":"structured"}},"failed":{"decide":"Fails?"},"unsure":{"decide":"Sure?","threshold":"0.1:0.9"}}}"#).unwrap();
    let native_call = engine
        .annotate_with(&set, ["Original."], CallOptions::new())
        .unwrap();
    let values = &native_call.value()[0];
    assert_eq!(
        values.iter().map(NamedAnnotation::name).collect::<Vec<_>>(),
        ["good", "failed", "unsure"]
    );
    assert_eq!(values[0].value(), &Annotated::Decision(Answer::Yes));
    let Annotated::Failed(failure) = values[1].value() else {
        panic!("failed member");
    };
    assert_eq!(failure.kind(), ErrorKind::Backend);
    assert_eq!(failure.cause(), FailureCause::WrongKind);
    assert_eq!(values[2].value(), &Annotated::Decision(Answer::Unsure));
    let native = native_call.results()[0].result().value_json().unwrap();
    let complete = crate::api::annotate(
        &engine,
        &set,
        crate::api::plain_inputs(["Original."]),
        CallOptions::new(),
    )
    .unwrap();
    let before = listener.count();
    let projected = complete.value()[0].result().value_json().unwrap();
    assert_eq!(projected, native);
    assert_eq!(
        serde_json::from_str::<Value>(&projected).unwrap(),
        json!({"good":true,"failed":{"failed":{"kind":"backend","cause":"wrong_kind"}},"unsure":null})
    );
    assert_eq!(listener.count(), before);
    assert!(!format!("{:?}", complete.value()[0].result()).contains("structured"));
}

#[test]
fn request_originals_and_cache_metadata_survive_owning_projections() {
    let listener = Listener::answering(response).unwrap();
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("result-projection-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    let engine = crate::api::fixed(listener.base(), "projection-private")
        .cache_at(&folder)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let definition = Question::from_json(
        r#"{"decide":"Fits?","on":["/body"],"true":null,"threshold":"0.95:1"}"#,
    )
    .unwrap();
    for original in [
        r#"{"body":"Same.","extra":false}"#,
        r#"{"body":"Same.","extra":null}"#,
    ] {
        let admitted = Request::new(RequestCall::new(
            RequestFunction::Decide,
            RequestArguments {
                question: RequestQuestion::Definition {
                    value: definition.clone().into(),
                },
                input: RequestInput::Records {
                    items: vec![RequestItem {
                        original: Some(RequestOriginal::Json {
                            value: RawRecord::json(original).unwrap(),
                        }),
                        context: None,
                        options: None,
                        examples: None,
                        seed_spans: None,
                        images: vec![],
                    }],
                },
                options: RequestOptions::default(),
            },
        ))
        .admit()
        .unwrap();
        let RequestOutcome::Complete(call) = engine
            .execute_request(&admitted, RequestEnvironment::default())
            .unwrap()
        else {
            panic!("complete")
        };
        let RequestValue::Decisions(rows) = call.value() else {
            panic!("decide")
        };
        let before = listener.count();
        let result = rows[0].result();
        let document = serde_json::to_value(&rows[0]).unwrap();
        assert_eq!(
            document["input"],
            serde_json::from_str::<Value>(original).unwrap()
        );
        assert_eq!(document["value"], false);
        assert_eq!(result.meta().cached(), original.contains("null"));
        assert_eq!(
            result.meta().cached(),
            document["meta"]["cached"].as_bool().unwrap()
        );
        assert_eq!(
            serde_json::to_value(result.identity().observations()).unwrap(),
            document["meta"]["observations"]
        );
        assert_eq!(listener.count(), before);
    }
    assert_eq!(
        listener.count(),
        1,
        "same selected evidence reuses the cache"
    );
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn saved_batch_without_threshold_and_opt_in_attempts_survive_native_projection() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let loaded =
        Question::from_json(r#"{"score":"Grade?","levels":["low","high"],"batch":2}"#).unwrap();
    let LoadedQuestion::Question(q) = &loaded else {
        panic!("score")
    };
    let controls = CallOptions::new().batch(BatchSetting::Max);
    let complete = crate::api::score(&engine, q, [record()], controls).unwrap();
    let before = listener.count();
    assert!(
        complete.value()[0]
            .result()
            .meta()
            .batch_warning()
            .is_none()
    );
    assert_eq!(complete.value()[0].original(), &record().original);
    assert_eq!(listener.count(), before);
    let q = Question::decide("Fits?").unwrap().cut();
    let call =
        crate::api::decide_records(&engine, &q, [record()], controls.attempts(true)).unwrap();
    let before = listener.count();
    let scalar = call.value()[0].result();
    assert_eq!(scalar.meta().attempts().unwrap().len(), 1);
    assert_eq!(listener.count(), before);
}

#[path = "support/api.rs"]
mod api;
