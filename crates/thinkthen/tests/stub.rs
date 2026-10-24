#![cfg(test)]
//! Public backend construction and key-based repeatability.

use thinkthen::{Backend, Decide, Engine, Record};

#[test]
fn stub_ignores_remote_settings_and_never_reads_credentials() {
    let backend = Backend::resolve_named(
        "stub",
        Some("http://remote.invalid"),
        Some("ignored"),
        |name| panic!("the stub read environment variable {name}"),
    )
    .unwrap();
    let question = Decide::new("Ready?").unwrap();
    let mut rows = Vec::new();
    Engine::new(backend)
        .call(
            &question,
            ["alpha", "beta", "alpha"].map(Record::text),
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["p"], rows[2]["p"]);
    assert_ne!(rows[0]["p"], rows[1]["p"]);
    assert!(rows.iter().all(|row| row["status"] == "answered"));
}

#[test]
fn every_library_construction_matches_the_shared_stub_rows() {
    use serde_json::Value;
    use thinkthen::{Cancel, SdkRequest, SdkSettings};

    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/stub.json")).unwrap();
    let mut failures = Vec::new();
    for case in cases {
        let mut request = case["request"].clone();
        // The public SDK admits settings and records separately from its question.
        // Each part comes from the same complete construction request.
        let fields = request.as_object_mut().unwrap();
        let records = fields.remove("records").unwrap();
        let settings = SdkSettings::from_json(&case["client"].to_string()).unwrap();
        let engine = settings
            .engine(|name| {
                assert_eq!(
                    name, "THINKTHEN_BATCH",
                    "the stub consulted a backend variable"
                );
                None
            })
            .unwrap();
        let request = SdkRequest::from_json(&request.to_string()).unwrap();
        let call = request.prepare(&engine, Cancel::default()).unwrap();
        let mut actual = Vec::new();
        call.run(
            records
                .as_array()
                .unwrap()
                .iter()
                .map(|value| (call.record(value), ())),
            |(), row| actual.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
        if Value::Array(actual.clone()) != case["expected"] {
            failures.push(serde_json::json!({"request":case["request"],"actual":actual}));
        }
    }
    assert!(
        failures.is_empty(),
        "construction rows: {}",
        Value::Array(failures)
    );
}

#[test]
fn annotation_stub_answers_use_each_questions_own_content_and_evidence() {
    use thinkthen::{Annotate, Pointer, Toolbox};
    let toolbox = Toolbox::from_bytes(
        br#"{"questions":{"a":{"decide":"Ready?","on":"/body"},"b":{"decide":"Ready?","on":"/body"},"c":{"decide":"Ready?","threshold":"0.8","on":"/body"},"d":{"decide":"Ready?","on":"/other"}}}"#,
    )
    .unwrap();
    let engine = Engine::new(Backend::stub());
    let mut rows = Vec::new();
    engine
        .annotate(
            &Annotate::new(&toolbox),
            [Record::json(
                r#"{"body":"evidence","other":"different"}"#,
                &Pointer::default(),
            )],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["answer"]["a"], rows[0]["answer"]["b"]);
    for (name, question, evidence) in [
        ("a", Decide::new("Ready?").unwrap(), "evidence"),
        (
            "c",
            Decide::new("Ready?")
                .unwrap()
                .threshold("0.8".parse().unwrap()),
            "evidence",
        ),
        ("d", Decide::new("Ready?").unwrap(), "different"),
    ] {
        let mut standalone = Vec::new();
        engine
            .call(&question, [Record::text(evidence)], |row| {
                standalone.push(serde_json::to_value(row).unwrap());
            })
            .unwrap();
        assert_eq!(standalone.len(), 1);
        for field in ["status", "answer", "p"] {
            assert_eq!(rows[0]["answer"][name][field], standalone[0][field]);
        }
    }
}

#[test]
fn nul_in_wording_and_record_data_remains_valid() {
    use thinkthen::{Pointer, RelationRule};
    let question = Decide::new("Ready?\0Still ready?").unwrap();
    RelationRule::new("linked", "person", "person")
        .unwrap()
        .reads("works\0with")
        .unwrap();
    let mut rows = Vec::new();
    Engine::new(Backend::stub())
        .call(
            &question,
            [Record::json(
                r#"{"id":{"data\u0000key":"id"},"record\u0000key":"before\u0000after"}"#,
                &Pointer::default(),
            )],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "answered");
    assert_eq!(rows[0]["id"], serde_json::json!({"data\0key":"id"}));
    assert!(rows[0]["answer"].is_boolean());
}
