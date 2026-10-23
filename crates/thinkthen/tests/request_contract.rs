#![cfg(test)]
//! Full-call admission and its published JSON Schemas.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use thinkthen::{Cancel, SdkRequest};

#[test]
fn cancelling_before_the_first_record_reports_the_same_terminal_error_as_pull() {
    use thinkthen::{Client, OffsetUnit};

    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    for function in ["decide", "filter", "rank", "annotate"] {
        let value = if function == "annotate" {
            json!({"questions":{"ready":{"decide":"Ready?"}}})
        } else {
            json!("Ready?")
        };
        for records in [json!([]), json!(["evidence"])] {
            let request = SdkRequest::from_json(
                &json!({
                    "function":function,"question":{"type":"inline","value":value},
                    "records":records
                })
                .to_string(),
            )
            .unwrap();
            let cancel = Cancel::default();
            cancel.cancel();
            let direct = request.execute(&client, cancel.clone(), |_| panic!("cancelled"));
            let pulled = client.open(&request, OffsetUnit::CodePoints, cancel);
            assert_eq!(direct.unwrap_err().kind().code(), "local.cancelled");
            assert_eq!(pulled.unwrap_err().kind().code(), "local.cancelled");
        }
    }
}

#[test]
fn clients_admit_settings_once_and_calls_never_read_question_paths() {
    use thinkthen::Client;
    let client = Client::from_json(r#"{"adapter":"stub","batch":1}"#, |name| {
        assert_eq!(name, "THINKTHEN_BATCH");
        None
    })
    .unwrap();
    let request = SdkRequest::from_json(r#"{"function":"decide","question":{"type":"inline","value":"literal.json"},"records":["evidence"]}"#).unwrap();
    let mut rows = Vec::new();
    request
        .execute(&client, Cancel::default(), |row| {
            rows.push(serde_json::to_value(row).unwrap())
        })
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["status"], "answered");
    assert_eq!(rows[0]["detail"]["source"]["backend"], "stub");
    assert_eq!(rows[0]["detail"]["source"]["model"], "stub");
    for (function, value, name, expected) in [
        (
            "annotate",
            json!({"questions":{"ready":{"decide":"Ready?"}}}),
            None,
            json!({"ready":{"status":"answered","answer":true,"p":0.76}}),
        ),
        (
            "rank",
            json!({"questions":{"ready":{"decide":"Ready?"}}}),
            Some("ready"),
            json!(1),
        ),
    ] {
        let request = SdkRequest::from_json(&json!({"function":function,"question":{"type":"toolbox","value":value,"name":name},"records":["evidence"]}).to_string()).unwrap();
        let mut rows = Vec::new();
        request
            .execute(&client, Cancel::default(), |row| {
                rows.push(serde_json::to_value(row).unwrap())
            })
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["answer"], expected);
    }
    for name in [None, Some("missing")] {
        let request = SdkRequest::from_json(&json!({"function":"decide","question":{"type":"toolbox","value":{"questions":{"a":{"decide":"Ready?"},"b":{"decide":"Other?"}}},"name":name}}).to_string()).unwrap();
        assert_eq!(
            client
                .prepare(&request, Cancel::default())
                .unwrap_err()
                .kind()
                .code(),
            "usage.question"
        );
    }
    for request in [
        json!({"function":"decide","question":"Ready?"}),
        json!({"function":"decide","question":{"type":"other","value":"Ready?"}}),
        json!({"function":"decide","question":{"type":"inline","value":42}}),
        json!({"function":"decide","question":{"type":"reference","name":""}}),
        json!({"function":"decide","question":{"type":"inline","value":"Ready?","name":"extra"}}),
        json!({"function":"decide","question":{"type":"inline","value":"Ready?"},"settings":{}}),
        json!({"function":"decide","question":{"type":"inline","value":"Ready?"},"options":{"batch":1}}),
        json!({"function":"decide","question":{"type":"inline","value":"Ready?"},"options":{"levels":["low","high"]}}),
    ] {
        assert_eq!(
            SdkRequest::from_json(&request.to_string())
                .unwrap_err()
                .kind()
                .code(),
            "usage.question",
            "{request}"
        );
    }
    let reference = SdkRequest::from_json(
        r#"{"function":"decide","question":{"type":"reference","name":"refund"}}"#,
    )
    .unwrap();
    assert_eq!(
        client
            .prepare(&reference, Cancel::default())
            .unwrap_err()
            .kind()
            .code(),
        "usage.reference"
    );
    assert_eq!(
        Client::from_json(
            r#"{"mode":"proxy","url":"https://proxy.invalid"}"#,
            |_| panic!("proxy admission read backend keys")
        )
        .unwrap_err()
        .kind()
        .code(),
        "usage.mode"
    );
}

#[test]
fn client_key_source_resolves_only_the_selected_variable_once() {
    use std::cell::RefCell;
    use thinkthen::Client;
    let server = Loopback::noul(0.9).unwrap();
    let reads = RefCell::new(Vec::new());
    let settings = json!({"url":server.base(),"key_source":"THINKTHEN_API_KEY","batch":1});
    let client = Client::from_json(&settings.to_string(), |name| {
        reads.borrow_mut().push(name.to_owned());
        (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
    })
    .unwrap();
    let admitted_reads = reads.borrow().clone();
    assert_eq!(admitted_reads, ["THINKTHEN_API_KEY"]);
    let request = SdkRequest::from_json(r#"{"function":"decide","question":{"type":"inline","value":"Ready?"},"records":["evidence"]}"#).unwrap();
    let mut rows = Vec::new();
    request
        .execute(&client, Cancel::default(), |row| {
            rows.push(serde_json::to_value(row).unwrap())
        })
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["answer"], true);
    assert_eq!(server.count(), 1);
    assert_eq!(*reads.borrow(), admitted_reads);
}

#[test]
fn routes_do_not_change_question_identity_and_default_ids_never_become_evidence() {
    use thinkthen::{Backend, Decide, Engine, Function, Pointer, Record};
    let question = Decide::new("Ready?").unwrap();
    let identity = question.identity();
    let toolbox = thinkthen::Toolbox::from_bytes(br#"{"decide":"Ready?"}"#).unwrap();
    assert_eq!(
        toolbox.question(None).unwrap().identity_as("decide"),
        identity
    );
    let changed = question.clone().threshold("0.555".parse().unwrap());
    assert_ne!(identity, changed.identity());
    assert_ne!(
        changed.identity(),
        question
            .clone()
            .threshold("0.554".parse().unwrap())
            .identity()
    );
    let mut identities = Vec::new();
    for _ in 0..2 {
        let server = Loopback::noul(0.9).unwrap();
        let backend = Backend::resolve(Some(server.base()), Some("served-model"), |name| {
            (name == "THINKTHEN_API_KEY").then(|| KEY.to_owned())
        })
        .unwrap();
        Engine::new(backend)
            .call(&question, [Record::text("evidence")], |row| {
                identities.push(
                    serde_json::to_value(row).unwrap()["detail"]["source"]["question_identity"]
                        .clone(),
                )
            })
            .unwrap();
        assert_eq!(server.count(), 1);
    }
    assert_eq!(identities, vec![json!(identity), json!(identity)]);
    let mut rows = Vec::new();
    Engine::new(Backend::stub())
        .call(
            &question,
            [
                Record::json(r#"{"id":"a","body":"evidence"}"#, &Pointer::default()),
                Record::json(r#"{"id":"b","body":"evidence"}"#, &Pointer::default()),
            ],
            |row| rows.push(serde_json::to_value(row).unwrap()),
        )
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert_ne!(rows[0]["id"], rows[1]["id"]);
    assert_eq!(rows[0]["p"], rows[1]["p"]);
    for (pointer, evidence) in [
        ("", r#"{"body":"evidence"}"#),
        ("/body", "evidence"),
        ("/id", "chosen"),
    ] {
        let record = Record::json(
            r#"{"id":"chosen","body":"evidence"}"#,
            &pointer.parse().unwrap(),
        );
        assert_eq!(record.evidence_at(&Pointer::default()).unwrap(), evidence);
    }
}

#[test]
fn published_schema_data_matches_the_canonical_types() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
    for (name, schema) in [
        ("client", schemars::schema_for!(thinkthen::SdkSettings)),
        ("request", schemars::schema_for!(SdkRequest)),
        ("row", thinkthen::Row::schema()),
    ] {
        let published: Value = serde_json::from_slice(
            &std::fs::read(directory.join(format!("{name}.schema.json"))).unwrap(),
        )
        .unwrap();
        assert_eq!(
            published,
            serde_json::to_value(schema).unwrap(),
            "regenerate schema data with cargo run -p thinkthen --example schema"
        );
        jsonschema::validator_for(&published).unwrap();
    }
}

#[test]
fn construction_cases_use_the_same_request_and_row_contract() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../construction/stub.json");
    let cases: Vec<Value> = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let requests = jsonschema::validator_for(
        &serde_json::to_value(schemars::schema_for!(SdkRequest)).unwrap(),
    )
    .unwrap();
    let rows = jsonschema::validator_for(&serde_json::to_value(thinkthen::Row::schema()).unwrap())
        .unwrap();
    assert!(!cases.is_empty());
    for case in cases {
        requests.validate(&case["request"]).unwrap();
        for row in case["expected"].as_array().unwrap() {
            rows.validate(row).unwrap();
        }
    }
}

#[test]
fn key_sources_belong_to_the_selected_backend_before_environment_lookup() {
    use thinkthen::Client;
    let server = Loopback::noul(0.9).unwrap();
    for (adapter, source) in [
        ("systemone", "OPENAI_API_KEY"),
        ("openai", "THINKTHEN_API_KEY"),
        ("systemone", "CALLER_KEY"),
        ("openai", "CALLER_KEY"),
        ("stub", "THINKTHEN_API_KEY"),
        ("systemone", KEY),
    ] {
        for url in [None, Some(server.base())] {
            let settings = json!({"adapter":adapter,"url":url,"key_source":source});
            let reads = std::cell::Cell::new(0);
            let result = Client::from_json(&settings.to_string(), |_| {
                reads.set(reads.get() + 1);
                Some(KEY.to_owned())
            });
            assert!(result.is_err(), "a foreign key source was admitted");
            let error = result.unwrap_err();
            assert_eq!(error.kind().code(), "usage.key-source");
            assert!(!error.to_string().contains(source));
            assert_eq!(reads.get(), 0);
        }
    }
    assert_eq!(server.count(), 0);
}

#[test]
fn named_backends_own_their_key_variable_and_missing_key_errors_hide_its_text() {
    use std::num::NonZeroUsize;
    use thinkthen::{Adapter, BackendEntry, BackendTable, Client, Limits};
    let server = Loopback::noul(0.9).unwrap();
    let mut table = BackendTable::default();
    for variable in ["CALLER_KEY", KEY] {
        table.insert(BackendEntry::new(
            "caller",
            Adapter::SystemOne,
            server.base(),
            variable,
            "model",
            Limits {
                request_bytes: NonZeroUsize::new(96_000).unwrap(),
                questions: None,
                options: None,
            },
        ));
        let settings = json!({"adapter":"caller","key_source":variable,"batch":1});
        let error =
            Client::from_json_with_backends(&settings.to_string(), &table, |_| None).unwrap_err();
        assert_eq!(error.kind().code(), "usage.key-missing");
        assert!(!error.to_string().contains(variable));
        let client = Client::from_json_with_backends(&settings.to_string(), &table, |name| {
            assert_eq!(name, variable, "a backend read another variable");
            Some(KEY.to_owned())
        })
        .unwrap();
        let request = SdkRequest::from_json(r#"{"function":"decide","question":{"type":"inline","value":"Ready?"},"records":["evidence"]}"#).unwrap();
        let mut rows = Vec::new();
        request
            .execute(&client, Cancel::default(), |row| {
                rows.push(serde_json::to_value(row).unwrap())
            })
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["answer"], true);
    }
    assert_eq!(server.count(), 2);
}
