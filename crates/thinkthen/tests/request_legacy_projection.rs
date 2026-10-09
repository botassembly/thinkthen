//! Retained result/1 projections match native batch documents without another send.
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::panic,
    reason = "invalid isolated fixture setup must stop the regression"
)]
use conformance_backend::{Canned, Listener};
use serde_json::{Value, json};
use thinkthen::*;

fn engine(listener: &Listener) -> Engine {
    Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("projection-private")
        .unwrap()
        .no_cache()
        .max_retries(0)
        .build()
        .unwrap()
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
fn documents(details: &Details) -> (Value, Value) {
    (
        serde_json::from_str(&details.to_json()).unwrap(),
        serde_json::from_str(&details.to_scalar_json()).unwrap(),
    )
}

#[test]
fn all_four_record_and_scalar_projections_match_corresponding_native_batch() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let questions = [
        r#"{"decide":"Fits?","threshold":"0.2:0.8"}"#,
        r#"{"choose":"Which?","options":{"Sensitive choice":null,"Other":null}}"#,
        r#"{"tag":"Which?","labels":{"Sensitive tag":null,"Other":null}}"#,
        r#"{"score":"Grade?","levels":{"Sensitive score":null,"Other":null}}"#,
    ];
    for raw in questions {
        let loaded = Question::from_json(raw).unwrap();
        let controls = CallOptions::new().context("Separate context.");
        let before = listener.count();
        let (record_details, scalar_details) = match &loaded {
            LoadedQuestion::Banded(q) => {
                let call = engine
                    .decide_records_complete_with(q, [record()], controls)
                    .unwrap();
                let row = &call.value()[0];
                (
                    row.legacy_details().unwrap(),
                    row.result().legacy_details().unwrap(),
                )
            }
            LoadedQuestion::Question(q) => {
                macro_rules! project {
                    ($method:ident) => {{
                        let call = engine.$method(q, [record()], controls).unwrap();
                        let row = &call.value()[0];
                        (
                            row.legacy_details().unwrap(),
                            row.result().legacy_details().unwrap(),
                        )
                    }};
                }
                match q.kind() {
                    QuestionKind::Choose => project!(choose_records_complete_with),
                    QuestionKind::Tag => project!(tag_records_complete_with),
                    QuestionKind::Score => project!(score_records_complete_with),
                    _ => panic!("fixture kind"),
                }
            }
        };
        assert_eq!(listener.count(), before + 1, "projections send nothing");
        let original = record().original;
        let native = engine
            .try_details_input_many_with(&loaded, [Ok(original)], controls)
            .next()
            .unwrap()
            .unwrap();
        assert_eq!(documents(&record_details), documents(native.value()));
        assert_eq!(documents(&scalar_details).0, documents(native.value()).1);
        assert!(documents(&record_details).0.get("input").is_some());
        assert!(documents(&scalar_details).0.get("input").is_none());
        assert_eq!(record_details.cached(), native.value().cached());
        assert_eq!(
            record_details.reported_usage(),
            native.value().reported_usage()
        );
        for details in [&record_details, &scalar_details, native.value()] {
            let debug = format!("{details:?}");
            for secret in [
                "Sensitive original",
                "Sensitive choice",
                "Sensitive tag",
                "Sensitive score",
                listener.base(),
            ] {
                assert!(!debug.contains(secret), "Debug exposed {secret}");
            }
        }
        assert_eq!(listener.count(), before + 2);
    }
}

#[test]
fn annotation_primitive_null_and_failed_members_match_legacy_exactly() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"wrong":1}},"q3":{"type":"noul","noul":0.5}}}"#)).unwrap();
    let engine = engine(&listener);
    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"good":{"decide":"Good?","true":{"authored":"structured"}},"failed":{"decide":"Fails?"},"unsure":{"decide":"Sure?","threshold":"0.1:0.9"}}}"#).unwrap();
    let native = engine
        .annotate_with(&set, ["Original."], CallOptions::new())
        .next()
        .unwrap()
        .unwrap()
        .value_json();
    let complete = engine
        .annotate_complete_with(&set, ["Original."], CallOptions::new())
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
        .join(format!("legacy-projection-{}", std::process::id()));
    std::fs::create_dir(&folder).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("projection-private")
        .unwrap()
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
        let admitted = Request::new(RequestCall::Decide(RequestArguments {
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
        }))
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
        let details = rows[0].legacy_details().unwrap();
        let document = documents(&details).0;
        assert_eq!(
            document["input"],
            serde_json::from_str::<Value>(original).unwrap()
        );
        assert_eq!(document["value"], false);
        assert_eq!(details.cached(), original.contains("null"));
        assert_eq!(
            details.cached(),
            rows[0].result().legacy_details().unwrap().cached()
        );
        assert_eq!(
            details.observations(),
            rows[0].result().identity().observations()
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
fn explicit_saved_batch_tuning_and_opt_in_attempts_survive_native_projection() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let loaded =
        Question::from_json(r#"{"score":"Grade?","levels":["low","high"],"batch":2}"#).unwrap();
    let LoadedQuestion::Question(q) = &loaded else {
        panic!("score")
    };
    let controls = CallOptions::new().batch(BatchSetting::Max);
    let complete = engine
        .score_records_complete_with(q, [record()], controls)
        .unwrap();
    let details = complete.value()[0].legacy_details().unwrap();
    let original = record().original;
    let native = engine
        .try_details_input_many_with(&loaded, [Ok(original)], controls)
        .next()
        .unwrap()
        .unwrap();
    assert_eq!(documents(&details), documents(native.value()));
    assert_eq!(
        documents(&details).0["meta"]["batch_warning"],
        json!({"running":"max","tuned_for":2})
    );
    let q = Question::decide("Fits?").unwrap().cut();
    let call = engine
        .decide_records_complete_with(&q, [record()], controls.attempts(true))
        .unwrap();
    let before = listener.count();
    let scalar = call.value()[0].result().legacy_details().unwrap();
    assert_eq!(
        documents(&scalar).0["meta"]["attempts"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(listener.count(), before);
}
