//! Shared engine-setting cases and constructor refusals through the C ABI.
use crate::cases::{Script, replies};
use crate::{compile, crate_dir, finished, run, scratch, start, text};
use conformance_backend::Backend;
use serde_json::{Value, json};

#[test]
#[expect(
    clippy::excessive_nesting,
    clippy::cognitive_complexity,
    clippy::too_many_lines,
    reason = "one shared corpus row maps settings, runs the C driver, and checks its wire effect"
)]
fn shared_settings_reach_the_c_constructor() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../../conformance/settings.json")).expect("cases");
    assert_eq!(corpus["schema"], "thinkthen.settings-cases/1");
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    for case in corpus["cases"].as_array().expect("cases") {
        let backend = Backend::start().expect("backend");
        let id = case["id"].as_str().expect("id");
        let base = format!(
            "{}/{}",
            backend.origin(),
            case["arm"].as_str().expect("arm")
        );
        let folder = scratch(&format!("settings-{id}"));
        let profile = folder.join("profile.json");
        if !case["profile"].is_null() {
            std::fs::write(&profile, case["profile"].to_string()).expect("profile");
        }
        for step in case["steps"].as_array().expect("steps") {
            let mut settings = step["settings"].clone();
            for value in settings.as_object_mut().expect("settings").values_mut() {
                if value == "$FOLDER" {
                    *value = json!(folder);
                }
                if value == "$PROFILE" {
                    *value = json!(profile);
                }
            }
            let mut script = Script::default();
            script.ask("env", &["THINKTHEN_CACHE", &folder.to_string_lossy()]);
            script.ask("settings", &[&base, &settings.to_string()]);
            let question = json!({"decide": corpus["question"]}).to_string();
            if step["verb"] == "decide_many" {
                let records = step["records"].as_array().expect("records");
                let mut fields = vec![base.as_str(), question.as_str()];
                fields.extend(
                    records
                        .iter()
                        .map(|record| record.as_str().expect("record")),
                );
                script.ask("many", &fields);
            } else if step["model"].is_string() {
                let request = json!({"decide": corpus["question"], "evidence": step["text"], "details": true}).to_string();
                script.ask("call", &[&base, &request]);
            } else {
                script.ask(
                    "decide",
                    &[&base, &question, step["text"].as_str().expect("text")],
                );
            }
            let output = run(&driver, &base, &script.0);
            assert!(
                output.status.success(),
                "{id}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let said = replies(&output.stdout).expect("replies");
            assert_eq!(said[0].0, 0, "{id}: constructor: {:?}", said[0]);
            if let Some(kind) = step["error"].as_str() {
                let code = if kind == "usage" {
                    1
                } else if kind == "backend" {
                    2
                } else {
                    4
                };
                assert_eq!(said[1].0, code, "{id}: {:?}", said[1]);
            } else if let Some(model) = step["model"].as_str() {
                let details: Value = serde_json::from_str(&said[1].1).expect("details");
                assert_eq!(details["meta"]["model"], model, "{id}");
            } else {
                assert_eq!(
                    (said[1].0, said[1].1.starts_with("1 ")),
                    (0, true),
                    "{id}: {:?}",
                    said[1]
                );
            }
            assert_eq!(
                backend.count(),
                step["count"].as_u64().expect("count") as usize,
                "{id}"
            );
        }
        if let Some(entries) = case["entries"].as_u64() {
            let count = std::fs::read_dir(&folder)
                .expect("folder")
                .filter(|entry| {
                    entry.as_ref().is_ok_and(|entry| {
                        entry.file_name() != ".thinkthen-backend.json"
                            && entry.path().extension().is_some_and(|ext| ext == "json")
                    })
                })
                .count();
            assert_eq!(count, entries as usize, "{id}");
        }
    }
}

#[test]
fn the_c_settings_object_refuses_bad_shapes_and_keys() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    for (given, named) in [
        ("[]", "one object"),
        ("not json", "one object"),
        (r#"{"timeout":"30"}"#, "timeout"),
        (r#"{"nope":1}"#, "nope"),
        (r#"{"api_key":"k"}"#, "api_key"),
        (r#"{"cache":true}"#, "cache"),
        (r#"{"model":null}"#, "model"),
        (r#"{"timeout":1,"timeout":2}"#, "timeout"),
        (r#"{"throttle":8.0}"#, "throttle"),
        (r#"{"max_retries":-1}"#, "max_retries"),
    ] {
        let mut script = Script::default();
        script.ask("settings", &[&base, given]);
        let output = run(&driver, &base, &script.0);
        let said = replies(&output.stdout).expect("reply");
        assert_eq!(said[0].0, 1, "{given}: {:?}", said[0]);
        assert!(said[0].1.contains(named), "{given}: {:?}", said[0]);
        assert_eq!(backend.count(), 0);
    }
    let mut script = Script::default();
    script.ask("settings", &[&base, r#"{"max_requests":null}"#]);
    let output = run(&driver, &base, &script.0);
    assert_eq!(replies(&output.stdout).expect("null limit")[0].0, 0);
}

#[test]
fn the_c_constructor_keeps_its_boundary_rules() {
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let source = crate_dir().join("tests/c/settings.c");
    let output = run(&compile(&source), &base, b"");
    assert_eq!(
        (output.status.code(), text(&output.stderr)),
        (Some(0), String::new())
    );
    assert_eq!(backend.count(), 0, "constructor refusals send nothing");
}

#[test]
fn the_c_throttle_holds_two_requests_until_release() {
    use std::io::Write;
    let backend = Backend::start().expect("held backend");
    let base = format!("{}/arm/held/v1", backend.origin());
    let question = r#"{"decide":"Does this need attention?"}"#;
    let mut script = Script::default();
    script.ask("settings", &[&base, r#"{"throttle":2,"cache":false}"#]);
    let records: Vec<String> = (0..8).map(|place| format!("record {place}")).collect();
    let mut fields = vec![base.as_str(), question];
    fields.extend(records.iter().map(String::as_str));
    script.ask("many", &fields);
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let mut child = start(&driver, &base);
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(&script.0)
        .expect("script");
    assert_eq!(backend.wait(2), 2, "two requests entered the held arm");
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert_eq!(backend.count(), 2, "a third request stays queued");
    backend.release();
    let output = finished(child);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let said = replies(&output.stdout).expect("replies");
    assert_eq!((said[0].0, said[1].0), (0, 0));
    assert_eq!(backend.count(), 8);
}
