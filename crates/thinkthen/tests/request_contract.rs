#![cfg(test)]
//! Request cancellation, canonical schema data and evidence identity.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{BackendName, Batch, Cancel, Decide, Key, OffsetUnit, Record, Request, Settings};
#[test]
fn cancelling_before_the_first_record_stops_empty_and_nonempty_calls() {
    for question in [
        json!({"function":"decide","question":"Ready?"}),
        json!({"function":"filter","question":"Ready?"}),
        json!({"function":"rank","question":"Ready?","levels":["low","high"]}),
        json!({"function":"annotate","questions":{"ready":{"function":"decide","question":"Ready?"}}}),
    ] {
        for records in [json!([]), json!(["evidence"])] {
            let request =
                Request::from_json(&json!({"question":question,"records":records}).to_string())
                    .unwrap();
            let cancel = Cancel::default();
            cancel.cancel();
            let engine = Settings::default()
                .with_backend(BackendName::Stub)
                .engine(|_| None)
                .unwrap()
                .cancel(cancel);
            assert_eq!(
                engine
                    .open_request(&request, OffsetUnit::CodePoints)
                    .unwrap_err()
                    .code()
                    .as_str(),
                "local.cancelled"
            );
        }
    }
}
#[test]
fn settings_resolve_credentials_once_and_literal_questions_do_not_read_paths() {
    let server = Loopback::noul(0.9).unwrap();
    let reads = std::cell::RefCell::new(Vec::new());
    let engine = Settings::default()
        .url(server.base())
        .batch(Batch::try_from(1_u64).unwrap())
        .engine(|name| {
            reads.borrow_mut().push(name.to_owned());
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
    let admitted = reads.borrow().clone();
    assert_eq!(admitted, ["THINKTHEN_API_KEY"]);
    let question = Decide::new("literal.json").unwrap();
    assert_eq!(
        engine
            .run(&question, Record::text("evidence"))
            .unwrap()
            .into_result()
            .unwrap(),
        Some(true)
    );
    assert_eq!(*reads.borrow(), admitted);
    assert_eq!(server.count(), 1);
}
#[test]
fn routes_preserve_identity_and_default_ids_do_not_become_evidence() {
    let question = Decide::new("Ready?").unwrap();
    let mut identities = Vec::new();
    for _ in 0..2 {
        let server = Loopback::noul(0.9).unwrap();
        let engine = Settings::default()
            .url(server.base())
            .model("served-model")
            .batch(Batch::default())
            .api_key(Key::new(KEY).unwrap())
            .engine(|_| None)
            .unwrap();
        identities.push(
            engine
                .run(&question, Record::text("evidence"))
                .unwrap()
                .source()
                .question_identity()
                .to_owned(),
        );
        assert_eq!(server.count(), 1);
    }
    assert_eq!(identities[0], identities[1]);
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let rows = engine
        .rows(
            &question,
            [
                Record::json(json!({"id":"a","body":"evidence"})),
                Record::json(json!({"id":"b","body":"evidence"})),
            ],
        )
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0].id(), rows[1].id());
    assert_eq!(rows[0].probability(), rows[1].probability());
    let changed = question.threshold("0.555".parse().unwrap());
    let changed_row = engine.run(&changed, Record::text("evidence")).unwrap();
    assert_ne!(identities[0], changed_row.source().question_identity());
    let nearby = changed.threshold("0.554".parse().unwrap());
    let nearby_row = engine.run(&nearby, Record::text("evidence")).unwrap();
    assert_ne!(
        changed_row.source().question_identity(),
        nearby_row.source().question_identity()
    );
}
#[test]
fn published_schema_data_matches_the_canonical_types() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    for (name, schema) in [
        ("client", schemars::schema_for!(Settings)),
        ("request", schemars::schema_for!(Request)),
        ("row", schemars::schema_for!(thinkthen::AnyRow)),
    ] {
        let published: Value = serde_json::from_slice(
            &std::fs::read(directory.join(format!("{name}.schema.json"))).unwrap(),
        )
        .unwrap();
        assert_eq!(
            published,
            serde_json::to_value(schema).unwrap(),
            "regenerate canonical schemas"
        );
        jsonschema::validator_for(&published).unwrap();
    }
}

#[test]
fn execution_names_and_resolved_settings_are_shared_without_credentials() {
    let settings = Settings::from_json(r#"{"backend":"openai","model":"chosen","url":"http://127.0.0.1:9123","api_key":"private-canary","batch":7,"retry":{"max_retries":1,"initial_backoff_ms":2,"attempt_timeout_ms":3}}"#).unwrap();
    let engine = settings
        .engine(|_| panic!("explicit settings read no environment"))
        .unwrap();
    let resolved = serde_json::to_value(engine.resolved_settings()).unwrap();
    assert_eq!(
        resolved,
        json!({"backend":"openai","model":"chosen","url":"http://127.0.0.1:9123","batch":7,"retry":{"max_retries":1,"initial_backoff_ms":2,"attempt_timeout_ms":3}})
    );
    assert!(!format!("{:?}", engine.resolved_settings()).contains("private-canary"));
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| panic!("stub reads no environment"))
        .unwrap();
    for name in [None, Some("review")] {
        let mut request =
            json!({"question":{"function":"decide","question":"Ready?"},"records":["evidence"]});
        if let Some(name) = name {
            request["name"] = json!(name);
        }
        let request = Request::from_json(&request.to_string()).unwrap();
        let rows = engine
            .open_request(&request, OffsetUnit::CodePoints)
            .unwrap()
            .rows()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rows[0].source().name(), name);
        let row = serde_json::to_value(&rows[0]).unwrap();
        assert_eq!(row["source"].get("name").and_then(Value::as_str), name);
    }
    for backend in BackendName::ALL {
        assert!(!backend.default_model().is_empty());
    }
}
