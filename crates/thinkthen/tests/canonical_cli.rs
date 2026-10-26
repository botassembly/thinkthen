#![cfg(test)]
#![cfg(feature = "command")]
//! The executable preserves complete typed rows across every shared wire fixture.
mod support;
use loopback::{KEY, Loopback};
use serde_json::{Value, json};
use std::{collections::BTreeSet, io::Write, path::Path, process::Stdio};

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
    if value.get("function").is_some_and(|f| f == "annotate") {
        for entry in value
            .get_mut("answer")
            .and_then(Value::as_object_mut)
            .into_iter()
            .flat_map(|a| a.values_mut())
        {
            semantic(entry);
        }
    }
}
fn arguments(request: &Value, directory: &Path) -> Vec<String> {
    let question = &request["question"];
    let function = question["function"].as_str().unwrap();
    let mut args = vec![function.to_owned()];
    if function == "annotate" {
        let path = directory.join("questions.json");
        let mut definition = question.clone();
        definition.as_object_mut().unwrap().remove("function");
        std::fs::write(&path, definition.to_string()).unwrap();
        args.push(path.to_str().unwrap().into());
    } else if let Some(text) = question["question"].as_str() {
        args.push(text.into());
    }
    for (key, flag) in [
        ("choices", "choice"),
        ("labels", "label"),
        ("levels", "level"),
        ("kinds", "kind"),
    ] {
        for item in question[key].as_array().into_iter().flatten() {
            args.extend([format!("--{flag}"), item.as_str().unwrap().into()]);
        }
    }
    for rule in question["rules"].as_array().into_iter().flatten() {
        args.extend([
            "--rule".into(),
            rule["relation"].as_str().unwrap().into(),
            rule["source"].as_str().unwrap().into(),
            rule["target"].as_str().unwrap().into(),
        ]);
    }
    if question["rules"]
        .as_array()
        .is_some_and(|rules| rules.iter().any(|r| r["either"] == true))
    {
        args.push("--either".into());
    }
    for key in ["evidence", "top", "threshold"] {
        if let Some(value) = question.get(key) {
            if key == "threshold" && value.is_object() {
                args.extend([
                    "--low".into(),
                    value["low"].to_string(),
                    "--high".into(),
                    value["high"].to_string(),
                ]);
            } else {
                args.extend([
                    format!("--{key}"),
                    value
                        .as_str()
                        .map_or_else(|| value.to_string(), str::to_owned),
                ]);
            }
        }
    }
    if let Some(name) = request["name"].as_str() {
        args.extend(["--name".into(), name.into()]);
    }
    args
}

#[test]
fn every_function_serializes_the_shared_rows_from_both_wires() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance/typed");
    let manifest = read(&fixtures.join("manifest.json"));
    let mut covered = BTreeSet::new();
    for name in manifest["cases"].as_array().unwrap() {
        let case = read(&fixtures.join(name.as_str().unwrap()));
        covered.insert(
            case["request"]["question"]["function"]
                .as_str()
                .unwrap()
                .to_owned(),
        );
        for backend in ["systemone", "openai"] {
            let directory = tempfile::tempdir().unwrap();
            let server = Loopback::case(&case).unwrap();
            let batch = case["client"]["batch"].as_u64().unwrap_or(32).to_string();
            let mut child = support::command()
                .args(arguments(&case["request"], directory.path()))
                .args([
                    "--backend",
                    backend,
                    "--url",
                    server.base(),
                    "--model",
                    "fixture-model",
                    "--from",
                    "jsonl",
                    "--json",
                    "--batch",
                    &batch,
                ])
                .env(
                    if backend == "systemone" {
                        "THINKTHEN_API_KEY"
                    } else {
                        "OPENAI_API_KEY"
                    },
                    KEY,
                )
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            let mut input = child.stdin.take().unwrap();
            if case["expected"].is_array() {
                for record in case["request"]["records"].as_array().unwrap() {
                    writeln!(input, "{record}").unwrap();
                }
            }
            drop(input);
            let output = child.wait_with_output().unwrap();
            if let Some(code) = case["expected"]["error"].as_str() {
                assert_eq!(output.status.code(), Some(2));
                assert!(output.stdout.is_empty());
                assert!(String::from_utf8_lossy(&output.stderr).contains(code));
                assert_eq!(server.count(), 0);
                continue;
            }
            let actual: Vec<Value> = String::from_utf8(output.stdout)
                .unwrap()
                .lines()
                .map(|line| {
                    let mut row: Value = serde_json::from_str(line).unwrap();
                    assert_eq!(row["source"]["backend"], backend);
                    assert_eq!(row["source"]["model"], "fixture-model");
                    semantic(&mut row);
                    row
                })
                .collect();
            assert_eq!(json!(actual), case["expected"], "{name} {backend}");
            let failed = actual.iter().any(|row| {
                row["status"] == "failed"
                    || row["answer"].as_object().is_some_and(|children| {
                        children.values().any(|child| child["status"] == "failed")
                    })
            });
            assert_eq!(
                output.status.code(),
                Some(if failed { 6 } else { 0 }),
                "{name} {backend}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let expected = case["requests"].as_u64().unwrap() as usize;
            if case["request"]["question"]["function"] == "rank" || expected == 0 {
                assert_eq!(server.count(), expected);
            } else {
                assert!(
                    server.count() >= expected
                        && server.count()
                            <= expected + case["request"]["records"].as_array().unwrap().len(),
                    "{name}: {} requests",
                    server.count()
                );
            }
        }
    }
    assert_eq!(covered.len(), thinkthen::FunctionName::ALL.len());
}
