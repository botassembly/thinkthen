#![cfg(test)]
//! Public backend construction and key-based repeatability.

use thinkthen::{BackendName, Decide, OffsetUnit, Record, Request, Settings};

#[test]
fn stub_ignores_remote_settings_and_never_reads_credentials() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .url("http://remote.invalid")
        .model("ignored")
        .engine(|name| panic!("the stub read environment variable {name}"))
        .unwrap();
    let question = Decide::new("Ready?").unwrap();
    let mut rows = Vec::new();
    engine
        .rows(&question, ["alpha", "beta", "alpha"].map(Record::text))
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0]["probability"], rows[2]["probability"]);
    assert_ne!(rows[0]["probability"], rows[1]["probability"]);
    assert!(rows.iter().all(|row| row["status"] == "answered"));
}

#[test]
fn every_library_construction_matches_the_shared_stub_rows() {
    use serde_json::Value;
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("../../../construction/stub.json")).unwrap();
    let mut failures = Vec::new();
    for case in cases {
        let engine = Settings::from_json(&case["client"].to_string())
            .unwrap()
            .engine(|name| panic!("the stub consulted variable {name}"))
            .unwrap();
        let request = Request::from_json(&case["request"].to_string()).unwrap();
        let call = engine
            .open_request(&request, OffsetUnit::CodePoints)
            .unwrap();
        call.sender().close();
        let actual = call
            .rows()
            .map(|row| serde_json::to_value(row.unwrap()).unwrap())
            .collect::<Vec<_>>();
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
    use thinkthen::Annotate;
    let toolbox = serde_json::from_str::<Annotate>(r#"{"questions":{"a":{"function":"decide","question":"Ready?","evidence":"/body"},"b":{"function":"decide","question":"Ready?","evidence":"/body"},"c":{"function":"decide","question":"Ready?","threshold":0.8,"evidence":"/body"},"d":{"function":"decide","question":"Ready?","evidence":"/other"}}}"#)
    .unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let mut rows = Vec::new();
    engine
        .rows(
            &toolbox,
            [Record::from_json(
                r#"{"body":"evidence","other":"different"}"#,
            )],
        )
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
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
            .rows(&question, [Record::text(evidence)])
            .unwrap()
            .map(|row| row.unwrap())
            .for_each(|row| {
                standalone.push(serde_json::to_value(row).unwrap());
            });
        assert_eq!(standalone.len(), 1);
        for field in ["status", "answer", "probability"] {
            assert_eq!(rows[0]["answer"][name][field], standalone[0][field]);
        }
    }
}

#[test]
fn nul_in_wording_and_record_data_remains_valid() {
    use thinkthen::RelationRule;
    let question = Decide::new("Ready?\0Still ready?").unwrap();
    RelationRule::new("linked", "person", "person")
        .unwrap()
        .reads("works\0with")
        .unwrap();
    let mut rows = Vec::new();
    Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap()
        .rows(
            &question,
            [Record::from_json(
                r#"{"id":"data\u0000id","record\u0000key":"before\u0000after"}"#,
            )],
        )
        .unwrap()
        .map(|row| row.unwrap())
        .for_each(|row| rows.push(serde_json::to_value(row).unwrap()));
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "answered");
    assert_eq!(rows[0]["id"], serde_json::json!("data\0id"));
    assert!(rows[0]["answer"].is_boolean());
}
