#![cfg(test)]
//! Every canonical function obeys one shared semantic corpus through both wires.
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::Path;
use thinkthen::{BackendName, FunctionName, Key, OffsetUnit, Request, Settings, results::AnyRow};

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn semantic(value: &mut Value) {
    if let Some(fields) = value.as_object_mut() {
        fields.remove("source");
    }
    if let Some(error) = value.get_mut("error").and_then(Value::as_object_mut) {
        error.remove("message");
    }
    if value
        .get("function")
        .is_some_and(|function| function == "annotate")
        && let Some(entries) = value.get_mut("answer").and_then(Value::as_object_mut)
    {
        for entry in entries.values_mut() {
            semantic(entry);
        }
    }
}
#[test]
fn all_functions_share_typed_rows_from_both_backend_wires() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/typed");
    let manifest = read(&directory.join("manifest.json"));
    let row_schema = serde_json::to_value(schemars::schema_for!(AnyRow)).unwrap();
    let rows = jsonschema::validator_for(&row_schema).unwrap();
    let request_schema = serde_json::to_value(schemars::schema_for!(Request)).unwrap();
    let requests = jsonschema::validator_for(&request_schema).unwrap();
    let mut covered = BTreeSet::new();
    let mut failures = Vec::new();
    for file in manifest["cases"].as_array().unwrap() {
        let case = read(&directory.join(file.as_str().unwrap()));
        requests.validate(&case["request"]).unwrap();
        if let Some(code) = case["expected"]["error"].as_str() {
            assert_eq!(
                Request::from_json(&case["request"].to_string())
                    .unwrap_err()
                    .code()
                    .as_str(),
                code
            );
            continue;
        }
        let request = Request::from_json(&case["request"].to_string()).unwrap();
        covered.insert(serde_json::to_string(&request.question().function()).unwrap());
        let mut identity = None;
        for backend in manifest["backends"].as_array().unwrap() {
            let backend: BackendName = serde_json::from_value(backend.clone()).unwrap();
            let server = Loopback::case(&case).unwrap();
            let mut client = manifest["client"].clone();
            if let Some(extra) = case["client"].as_object() {
                for (key, value) in extra {
                    client[key] = value.clone();
                }
            }
            client["backend"] = serde_json::to_value(backend).unwrap();
            let engine = Settings::from_json(&client.to_string())
                .unwrap()
                .url(server.base())
                .api_key(Key::new(KEY).unwrap())
                .engine(|_| panic!("explicit settings must not read environment"))
                .unwrap();
            let call = engine
                .open_request(&request, OffsetUnit::CodePoints)
                .unwrap();
            let sender = call.sender();
            sender.close();
            let actual: Vec<_> = call.rows().collect::<Result<Vec<_>, _>>().unwrap();
            let actual: Vec<_> = actual
                .into_iter()
                .map(|row| {
                    assert_eq!(row.source().backend(), backend);
                    assert_eq!(
                        row.source().model(),
                        manifest["client"]["model"].as_str().unwrap()
                    );
                    let hash = row.source().question_identity();
                    assert_eq!(hash.len(), 64);
                    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
                    if let Some(identity) = &identity {
                        assert_eq!(hash, identity);
                    } else {
                        identity = Some(hash.to_owned());
                    }
                    let mut value = serde_json::to_value(row).unwrap();
                    rows.validate(&value)
                        .unwrap_or_else(|error| panic!("{}: {error}", case["name"]));
                    let restored: AnyRow = serde_json::from_value(value.clone()).unwrap();
                    assert_eq!(serde_json::to_value(restored).unwrap(), value);
                    semantic(&mut value);
                    value
                })
                .collect();
            if json!(actual) != case["expected"] {
                failures.push(format!(
                    "{} / {backend:?}: expected {}, got {}",
                    case["name"],
                    case["expected"],
                    json!(actual)
                ));
            }
            assert_eq!(
                server.count() as u64,
                case["requests"].as_u64().unwrap(),
                "{} / {backend:?}",
                case["name"]
            );
            for sent in server.requests() {
                assert_eq!(
                    sent.path,
                    if backend == BackendName::SystemOne {
                        "/v1/systemone"
                    } else {
                        "/v1/decisions"
                    }
                );
            }
        }
    }
    assert_eq!(
        covered,
        FunctionName::ALL
            .iter()
            .map(|name| serde_json::to_string(name).unwrap())
            .collect()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn row_admission_and_schema_reject_fields_that_disagree_with_their_tags() {
    let schema = serde_json::to_value(schemars::schema_for!(AnyRow)).unwrap();
    let rows = jsonschema::validator_for(&schema).unwrap();
    let source = json!({"backend":"stub","model":"stub","question_identity":"fixture"});
    let valid = [
        json!({"function":"decide","position":1,"source":source,"status":"answered","answer":true,"probability":0.8123456789012345}),
        json!({"function":"find","position":1,"source":source,"status":"answered","answer":null}),
        json!({"function":"decide","position":1,"source":source,"status":"unsure","probability":0.6}),
        json!({"function":"decide","position":1,"source":source,"status":"failed","error":{"code":"record.missing","message":"missing"}}),
    ];
    for value in &valid {
        assert!(rows.is_valid(value), "{value}");
        assert!(serde_json::from_value::<AnyRow>(value.clone()).is_ok());
    }
    let mut invalid = Vec::new();
    let mut value = valid[0].clone();
    value["answer"] = json!("true");
    invalid.push(value);
    let mut value = valid[0].clone();
    value["position"] = json!(0);
    invalid.push(value);
    let mut value = valid[0].clone();
    value["raw_backend"] = json!({});
    invalid.push(value);
    let mut value = valid[2].clone();
    value["answer"] = json!(null);
    invalid.push(value);
    let mut value = valid[3].clone();
    value["probability"] = json!(0.5);
    invalid.push(value);
    let mut value = valid[3].clone();
    value["error"]["code"] = json!("backend.unknown");
    invalid.push(value);
    let mut value = valid[3].clone();
    value["error"]["code"] = json!("usage.threshold");
    invalid.push(value);
    let mut value = valid[0].clone();
    value["probability"] = json!(null);
    invalid.push(value);
    for value in invalid {
        assert!(!rows.is_valid(&value), "schema admitted {value}");
        assert!(
            serde_json::from_value::<AnyRow>(value.clone()).is_err(),
            "serde admitted {value}"
        );
    }
}

#[test]
fn generated_shape_boundaries_match_schema_and_typed_row_admission() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/typed/shapes.json");
    let data = read(&path);
    let schema = serde_json::to_value(schemars::schema_for!(AnyRow)).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    for case in data["cases"].as_array().unwrap() {
        let expected = case["valid"].as_bool().unwrap();
        let value = case["row"].clone();
        assert_eq!(
            validator.is_valid(&value),
            expected,
            "schema {}: {value}",
            case["name"]
        );
        assert_eq!(
            serde_json::from_value::<AnyRow>(value.clone()).is_ok(),
            expected,
            "serde {}: {value}",
            case["name"]
        );
    }
}
