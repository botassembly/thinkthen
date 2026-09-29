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
            settings["batch"] = json!(1);
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
            if step["verb"] == "relate" {
                let relation = case["relation"].as_str().expect("relation");
                let (name, ends) = relation.split_once('=').expect("rule name");
                let (source, target) = ends.split_once(':').expect("rule ends");
                let rule = json!({"version":1,"relate":{"relations":[{"name":name,"source":source,"target":target}]}}).to_string();
                let entities: Vec<String> = case["entities"]
                    .as_array()
                    .expect("entities")
                    .iter()
                    .map(ToString::to_string)
                    .collect();
                let mut fields = vec![base.as_str(), rule.as_str()];
                fields.extend(entities.iter().map(String::as_str));
                script.ask("relate", &fields);
            } else if step["verb"] == "decide_many" {
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
            } else if step["verb"] == "relate" {
                let edges: Value = serde_json::from_str(&said[1].1).expect("edges");
                assert_eq!(said[1].0, 0, "{id}: {:?}", said[1]);
                assert_eq!(
                    edges["edges"].as_array().map(Vec::len),
                    step["edges"].as_u64().map(|n| n as usize),
                    "{id}"
                );
            } else if let Some(model) = step["model"].as_str() {
                let details: Value = serde_json::from_str(&said[1].1).expect("details");
                assert_eq!(details["value"]["meta"]["model"], model, "{id}");
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
fn saved_calibration_keeps_its_digest_and_warning_through_the_c_door() {
    let fixture: Value =
        serde_json::from_str(include_str!("../../../../conformance/calibration.json"))
            .expect("calibration fixture");
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let folder = scratch("calibration-c");
    let profile = folder.join("profile.json");
    std::fs::write(&profile, fixture["runtime_profile"].to_string()).expect("profile");
    let mut script = Script::default();
    script.ask(
        "settings",
        &[&base, &json!({"profile":profile,"cache":false}).to_string()],
    );
    let mut request = fixture["question"].as_object().expect("question").clone();
    request.insert("evidence".to_owned(), fixture["evidence"].clone());
    request.insert("details".to_owned(), json!(true));
    script.ask("call", &[&base, &Value::Object(request).to_string()]);
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let output = run(&driver, &base, &script.0);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let said = replies(&output.stdout).expect("replies");
    assert_eq!(said[0].0, 0, "constructor: {:?}", said[0]);
    assert_eq!(said[1].0, 0, "call: {:?}", said[1]);
    let details: Value = serde_json::from_str(&said[1].1).expect("details");
    assert_eq!(
        details["value"]["meta"]["question_sha256"],
        fixture["question_sha256"]
    );
    assert_eq!(
        details["value"]["meta"]["profile_warning"],
        fixture["warning"]
    );
    assert_eq!(details["value"]["meta"]["model"], fixture["model"]);
    assert_eq!(backend.count(), 1);
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
        (r#"{"timeout":1,"timeout":2}"#, "repeats"),
        (r#"{"throttle":8.0}"#, "throttle"),
        (r#"{"max_retries":-1}"#, "max_retries"),
        (r#"{"max_request_bytes":0}"#, "max_request_bytes"),
        (r#"{"max_requests_total":-1}"#, "max_requests_total"),
        (
            r#"{"max_estimated_input_tokens_total":-1}"#,
            "max_estimated_input_tokens_total",
        ),
        (
            r#"{"max_estimated_input_tokens_total":1.5}"#,
            "max_estimated_input_tokens_total",
        ),
        (
            r#"{"max_estimated_input_tokens_total":"4"}"#,
            "max_estimated_input_tokens_total",
        ),
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
    script.ask(
        "settings",
        &[&base, r#"{"max_requests":null,"max_requests_total":3,"max_estimated_input_tokens_total":null}"#],
    );
    let output = run(&driver, &base, &script.0);
    assert_eq!(replies(&output.stdout).expect("null limit")[0].0, 0);
}

#[test]
fn the_c_constructor_zero_cap_refuses_before_the_listener() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let mut script = Script::default();
    script.ask(
        "settings",
        &[&base, r#"{"cache":false,"max_requests_total":0}"#],
    );
    script.ask(
        "decide",
        &[&base, r#"{"decide":"asks for a refund"}"#, "Refund me."],
    );
    let output = run(&driver, &base, &script.0);
    let said = replies(&output.stdout).expect("replies");
    assert_eq!(said[0].0, 0, "constructor accepts an active cap");
    assert_ne!(said[1].0, 0, "the live send is refused");
    assert!(said[1].1.contains("process send budget"), "{:?}", said[1]);
    assert_eq!(
        backend.count(),
        0,
        "the cap stops the C call before transport"
    );
}

#[test]
fn the_c_constructor_estimated_zero_refuses_before_the_listener() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let mut script = Script::default();
    script.ask(
        "settings",
        &[
            &base,
            r#"{"cache":false,"max_estimated_input_tokens_total":0}"#,
        ],
    );
    script.ask(
        "decide",
        &[&base, r#"{"decide":"asks for a refund"}"#, "Refund me."],
    );
    let output = run(&driver, &base, &script.0);
    let said = replies(&output.stdout).expect("replies");
    assert_eq!(said[0].0, 0, "the constructor accepted an active limit");
    assert_ne!(said[1].0, 0, "the live body was refused");
    assert!(
        said[1]
            .1
            .contains("max_estimated_input_tokens_total=0 (encoded-body-bytes-908-v1)"),
        "{:?}",
        said[1]
    );
    assert_eq!(backend.count(), 0, "no request reached the listener");
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
fn the_c_batch_one_stream_waits_for_a_held_request() {
    use std::io::Write;
    let backend = Backend::start().expect("held backend");
    let base = format!("{}/arm/held/v1", backend.origin());
    let question = r#"{"decide":"Does this need attention?"}"#;
    let mut script = Script::default();
    script.ask(
        "settings",
        &[&base, r#"{"throttle":2,"cache":false,"batch":1}"#],
    );
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
    assert_eq!(backend.wait(1), 1, "one request entered the held arm");
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert_eq!(backend.count(), 1, "the next request stays queued");
    backend.release();
    let output = finished(child);
    assert!(output.status.success(), "{}", text(&output.stderr));
    let said = replies(&output.stdout).expect("replies");
    assert_eq!((said[0].0, said[1].0), (0, 0));
    assert_eq!(backend.count(), 8);
}
