#![cfg(test)]
//! Published admission codes, execution identities and safe diagnostics.

use serde_json::{Value, json};
use thinkthen::{Cancel, Client, SdkRequest};

#[test]
fn client_admission_reports_distinct_public_codes() {
    let cases: Vec<Value> = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/clients.json"
    ))
    .unwrap();
    for case in cases {
        let error = Client::from_json(case["settings"].as_str().unwrap(), |_| None).unwrap_err();
        assert_eq!(error.kind().code(), case["code"], "{case}");
        assert_eq!(error.kind().family().as_str(), "usage");
    }
}

#[test]
fn published_identity_vectors_survive_private_representation_changes() {
    let cases: Vec<Value> = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/identity.json"
    ))
    .unwrap();
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    for case in cases {
        let raw = case
            .get("raw_request")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .unwrap_or_else(|| case["request"].to_string());
        let request = SdkRequest::from_json(&raw).unwrap();
        let mut rows = Vec::new();
        request
            .execute(&client, Cancel::default(), |row| {
                rows.push(serde_json::to_value(row).unwrap());
            })
            .unwrap();
        assert_eq!(rows.len(), 1, "{case}");
        assert_eq!(
            rows[0]["detail"]["source"]["question_identity"], case["identity"],
            "{}",
            case["canonical"]
        );
        for field in ["p", "answer"] {
            if let Some(expected) = case.get(field) {
                assert_eq!(&rows[0][field], expected, "{case}");
            }
        }
    }
}

#[test]
fn diagnostic_controls_are_escaped_once_before_host_translation() {
    let cases: Vec<Value> = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/diagnostics.json"
    ))
    .unwrap();
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    for case in cases {
        let request = SdkRequest::from_json(
            &json!({
                "function":"annotate", "question":{"type":"inline","value":case["question"]}
            })
            .to_string(),
        )
        .unwrap();
        let error = client.prepare(&request, Cancel::default()).unwrap_err();
        assert_eq!(error.kind().code(), case["code"]);
        assert_eq!(error.to_string(), case["message"]);
        assert_eq!(
            serde_json::to_value(error).unwrap()["message"],
            case["message"]
        );
    }
}

#[test]
fn owning_boundaries_keep_their_distinct_codes() {
    use thinkthen::{Backend, Decide, Engine, Key, Pointer, Record, RetryPolicy, Toolbox};
    let cases: Value = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/boundaries.json"
    ))
    .unwrap();
    let client = Client::from_json(r#"{"adapter":"stub"}"#, |_| None).unwrap();
    for case in cases["requests"].as_array().unwrap() {
        let request = SdkRequest::from_json(&case["request"].to_string()).unwrap();
        assert_eq!(
            client
                .prepare(&request, Cancel::default())
                .unwrap_err()
                .kind()
                .code(),
            case["code"]
        );
    }
    let toolbox = &cases["toolbox"];
    assert_eq!(
        Toolbox::from_bytes(toolbox["input"].as_str().unwrap().as_bytes())
            .unwrap_err()
            .kind()
            .code(),
        toolbox["code"]
    );
    let record = &cases["record"];
    let error = Engine::new(Backend::stub())
        .call(
            &Decide::new("Ready?").unwrap(),
            [Record::json(
                record["input"].as_str().unwrap(),
                &Pointer::default(),
            )],
            |_| {},
        )
        .unwrap_err();
    assert_eq!(error.kind().code(), record["code"]);
    let unavailable = loopback::UnavailableBackend::new().unwrap();
    let transport = &cases["transport"];
    let error = Engine::new(Backend::new(
        unavailable.base(),
        Key::new(loopback::KEY).unwrap(),
        "test",
    ))
    .retry_policy(RetryPolicy {
        max_retries: 0,
        attempt_timeout: std::time::Duration::from_secs(2),
        ..RetryPolicy::default()
    })
    .call(
        &Decide::new("Ready?").unwrap(),
        [Record::text(transport["record"].as_str().unwrap())],
        |_| {},
    )
    .unwrap_err();
    assert_eq!(error.kind().code(), transport["code"]);
    let limit = &cases["limit"];
    let mut table = thinkthen::BackendTable::default();
    table.insert(thinkthen::BackendEntry::new(
        "limited",
        thinkthen::Adapter::SystemOne,
        unavailable.base(),
        "TEST_KEY",
        "test",
        thinkthen::Limits {
            request_bytes: std::num::NonZeroUsize::new(
                limit["request_bytes"].as_u64().unwrap() as usize
            )
            .unwrap(),
            questions: None,
            options: None,
        },
    ));
    let client = Client::from_json_with_backends(r#"{"adapter":"limited"}"#, &table, |name| {
        (name == "TEST_KEY").then(|| loopback::KEY.to_owned())
    })
    .unwrap();
    let request = SdkRequest::from_json(&json!({"function":"decide","question":{"type":"inline","value":"Ready?"},"records":[limit["record"]]}).to_string()).unwrap();
    assert_eq!(
        request
            .execute(&client, Cancel::default(), |_| {})
            .unwrap_err()
            .kind()
            .code(),
        limit["code"]
    );
}
