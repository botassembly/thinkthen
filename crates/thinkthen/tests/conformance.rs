#![cfg(test)]
//! Shared language-neutral contracts through the public Rust library.

use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::path::Path;
use thinkthen::{Adapter, BackendEntry, BackendTable, Cancel, Client, Error, Limits, SdkRequest};

#[test]
fn every_surface_obeys_the_shared_cases() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance");
    let mut paths: Vec<_> = std::fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();
    assert!(!paths.is_empty());
    let request_schema = jsonschema::validator_for(
        &serde_json::to_value(schemars::schema_for!(SdkRequest)).unwrap(),
    )
    .unwrap();
    let row_schema =
        jsonschema::validator_for(&serde_json::to_value(thinkthen::Row::schema()).unwrap())
            .unwrap();
    let mut failures = Vec::new();
    for path in paths {
        let case: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(
            request_schema.is_valid(&case["request"]),
            case["request_valid"] != false,
            "{}: request schema admission differs",
            path.display()
        );
        let server = Loopback::case(&case).unwrap();
        let actual = run(&case, &server)
            .map(Value::Array)
            .unwrap_or_else(|error| json!({"error":error.kind().code()}));
        for row in actual.as_array().into_iter().flatten() {
            row_schema
                .validate(row)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        }
        let mut actual = actual;
        omit_messages(&mut actual, case["request"]["function"] == "annotate");
        if actual != case["expected"] {
            failures.push(format!(
                "{}: expected {}, got {actual}",
                path.display(),
                case["expected"]
            ));
        }
        if server.count() != case["requests"].as_u64().unwrap() as usize {
            failures.push(format!(
                "{}: expected {} requests, got {}",
                path.display(),
                case["requests"],
                server.count()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn run(case: &Value, server: &Loopback) -> Result<Vec<Value>, Error> {
    let request = SdkRequest::from_json(&case["request"].to_string())?;
    let mut table = BackendTable::default();
    table.insert(BackendEntry::new(
        "clef",
        Adapter::SystemOne,
        server.base(),
        "CLEF_KEY",
        "served-clef",
        Limits {
            request_bytes: std::num::NonZeroUsize::new(96_000).unwrap(),
            questions: None,
            options: std::num::NonZeroUsize::new(255),
        },
    ));
    let mut settings = case["client"].clone();
    settings["adapter"] = json!("clef");
    let client =
        Client::from_json_with_backends(&settings.to_string(), &table, |name| match name {
            "CLEF_KEY" => Some(KEY.to_owned()),
            "THINKTHEN_BATCH" => None,
            _ => panic!("the selected entry read another backend's variable {name}"),
        })?;
    let cancel = Cancel::default();
    if case["cancelled"] == true {
        cancel.cancel();
    }
    let call = client.prepare(&request, cancel)?;
    let mut rows = Vec::new();
    let result = call.run(
        request
            .records()
            .iter()
            .map(|value| (call.record(value), ())),
        |(), row| rows.push(serde_json::to_value(row).unwrap()),
    );
    for row in &mut rows {
        if let Some(source) = row.pointer_mut("/detail/source") {
            assert_eq!(source["backend"], "clef");
            assert_eq!(source["model"], "served-clef");
            source["backend"] = json!("systemone");
            source["model"] = json!("jev-1.13.0");
        }
    }
    for sent in server.requests() {
        assert_eq!(
            sent.header("authorization"),
            Some(format!("Bearer {KEY}").as_str())
        );
        assert_eq!(sent.path, "/v1/systemone");
        assert_eq!(sent.json().unwrap()["model"], "served-clef");
    }
    if let Some(expected) = case.get("expected_rows") {
        let mut actual = json!(rows);
        omit_messages(&mut actual, case["request"]["function"] == "annotate");
        assert_eq!(&actual, expected);
    }
    result?;
    Ok(rows)
}

fn omit_messages(value: &mut Value, annotate: bool) {
    for row in value.as_array_mut().into_iter().flatten() {
        omit_error_message(row);
        if annotate {
            for entry in row
                .get_mut("answer")
                .and_then(Value::as_object_mut)
                .into_iter()
                .flat_map(|x| x.values_mut())
            {
                omit_error_message(entry);
            }
        }
    }
}

fn omit_error_message(outcome: &mut Value) {
    if let Some(Value::Object(error)) = outcome.get_mut("error") {
        error.remove("message");
    }
}
