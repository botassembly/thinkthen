#![cfg(test)]
//! External C consumers read the canonical corpus without a JSON parser.
mod support;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::path::Path;
fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn discriminator(value: &mut Value, registry: &Value, key: &str, domain: &str) {
    let number = value[key].as_u64().unwrap();
    value[key] = registry[domain]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["value"].as_u64() == Some(number))
        .unwrap()["name"]
        .clone();
}
fn normalize(value: &mut Value, registry: &Value, backend: &Value, model: &Value) {
    discriminator(value, registry, "function", "functions");
    discriminator(value, registry, "status", "statuses");
    if let Some(source) = value.get_mut("source") {
        discriminator(source, registry, "backend", "backends");
        assert_eq!(&source["backend"], backend);
        assert_eq!(&source["model"], model);
        let identity = source["question_identity"].as_str().unwrap();
        assert_eq!(identity.len(), 64);
        assert!(identity.chars().all(|c| c.is_ascii_hexdigit()));
        value.as_object_mut().unwrap().remove("source");
    }
    if value["function"] == "annotate"
        && let Some(entries) = value.get_mut("answer").and_then(Value::as_object_mut)
    {
        for child in entries.values_mut() {
            normalize(child, registry, backend, model);
        }
    }
}
#[test]
fn typed_c_projections_preserve_every_answer_detail_error_and_id_through_both_wires() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let directory = root.join("conformance/typed");
    let manifest = read(&directory.join("manifest.json"));
    let registry = read(&root.join("schemas/contract.json"));
    for entry in manifest["cases"].as_array().unwrap() {
        let case = read(&directory.join(entry.as_str().unwrap()));
        for backend in manifest["backends"].as_array().unwrap() {
            let server = Loopback::case(&case).unwrap();
            let mut client = manifest["client"].clone();
            if let Some(extra) = case["client"].as_object() {
                for (key, value) in extra {
                    client[key] = value.clone();
                }
            }
            client["backend"] = backend.clone();
            client["url"] = json!(server.base());
            let key = if backend == "systemone" {
                "THINKTHEN_API_KEY"
            } else {
                "OPENAI_API_KEY"
            };
            let output = support::command("thinkthen-typed-projections")
                .args([client.to_string(), case["request"].to_string()])
                .env(key, KEY)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{} / {}: {}",
                case["name"],
                backend,
                String::from_utf8_lossy(&output.stderr)
            );
            let actual: Vec<Value> = String::from_utf8(output.stdout)
                .unwrap()
                .lines()
                .map(|line| {
                    let mut value: Value = serde_json::from_str(line).unwrap();
                    if value.get("function").is_some() {
                        normalize(&mut value, &registry, backend, &manifest["client"]["model"]);
                    }
                    value
                })
                .collect();
            assert_eq!(
                json!(actual),
                if case["expected"].is_object() {
                    json!([case["expected"]])
                } else {
                    case["expected"].clone()
                },
                "{} / {}",
                case["name"],
                backend
            );
            assert_eq!(server.count() as u64, case["requests"].as_u64().unwrap());
        }
    }
}
#[test]
fn typed_c_builders_and_sized_outputs_keep_their_ownership_contract() {
    let result = support::command("thinkthen-typed-consumer")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn database_caller_retains_null_invalid_and_terminal_error_protocol() {
    let result = support::command("thinkthen-typed-sql").output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
}

#[test]
fn typed_c_execution_names_match_named_requests_and_toolbox_selection_stays_unnamed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let options = read(&root.join("construction/options.json"));
    let registry = read(&root.join("schemas/contract.json"));
    for named in options.as_array().unwrap().iter().filter(|case| {
        matches!(
            case["name"].as_str(),
            Some("named-choose-domain" | "named-rank-levels")
        )
    }) {
        let unnamed_name = named["name"]
            .as_str()
            .unwrap()
            .replacen("named-", "unnamed-", 1);
        let unnamed = options
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["name"] == unnamed_name)
            .unwrap();
        let request = &named["request"];
        let question = &request["question"];
        let function = question["function"].as_str().unwrap();
        let domain = if function == "choose" {
            "choices"
        } else {
            "levels"
        };
        let arguments = |mode: &str, settings: &Value, name: &str| {
            vec![
                mode.to_owned(),
                settings.to_string(),
                function.to_owned(),
                question["question"].as_str().unwrap().to_owned(),
                question[domain][0].as_str().unwrap().to_owned(),
                question[domain][1].as_str().unwrap().to_owned(),
                name.to_owned(),
                request["records"][0].as_str().unwrap().to_owned(),
                request["records"][1].as_str().unwrap().to_owned(),
            ]
        };
        for mode in ["named", "unnamed", "selected"] {
            let output = support::command("thinkthen-typed-consumer")
                .args(arguments(
                    mode,
                    &named["client"],
                    request["name"].as_str().unwrap(),
                ))
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let rows: Vec<Value> = String::from_utf8(output.stdout)
                .unwrap()
                .lines()
                .map(|line| {
                    let mut row: Value = serde_json::from_str(line).unwrap();
                    discriminator(&mut row, &registry, "function", "functions");
                    discriminator(&mut row, &registry, "status", "statuses");
                    if let Some(probability) = row.get_mut("probability") {
                        *probability = json!(probability.as_f64().unwrap());
                    }
                    row
                })
                .collect();
            assert_eq!(
                json!(rows),
                if mode == "named" {
                    &named["expected"]
                } else {
                    &unnamed["expected"]
                }
                .clone(),
                "{mode}"
            );
        }
        let server =
            Loopback::case(&read(&root.join("conformance/typed/choose-tie.json"))).unwrap();
        let settings = json!({"backend":"systemone", "url":server.base()});
        for invalid in ["", "   "] {
            let output = support::command("thinkthen-typed-consumer")
                .args(arguments("invalid", &settings, invalid))
                .env("THINKTHEN_API_KEY", KEY)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(
                String::from_utf8(output.stdout).unwrap().trim(),
                "usage.name"
            );
            assert_eq!(server.count(), 0);
        }
    }
}
