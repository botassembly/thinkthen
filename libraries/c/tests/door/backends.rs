//! Named backend selection through the public C settings JSON door.

use std::collections::BTreeMap;

use conformance_backend::Backend;
use serde_json::{Value, json};

use crate::cases::{Script, replies};
use crate::{compile, crate_dir, run, run_with, scratch, text};

const REQUEST: &str = r#"{"decide":"Does it need attention?","true":{"what":"yes","examples":["refund"]},"evidence":"refund"}"#;

#[test]
fn c_constructor_selects_all_provider_keys_models_paths_and_forms() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../../conformance/binding-backends.json"
    ))
    .expect("rows");
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    for row in corpus["backends"].as_array().expect("backends") {
        let name = row["name"].as_str().expect("name");
        let marker = format!("fake-binding-{name}");
        let backend = Backend::with_markers(BTreeMap::from([(name.to_owned(), marker.clone())]))
            .expect("backend");
        let base = format!("{}/arm/full/capture/v1", backend.origin());
        let mut script = Script::default();
        script.ask("env", &[row["key"].as_str().expect("key"), &marker]);
        script.ask(
            "settings",
            &[
                &base,
                &json!({"backend":name,"base_url":base,"cache":false}).to_string(),
            ],
        );
        script.ask("call", &[&base, REQUEST]);
        let output = run(&driver, &base, &script.0);
        assert!(output.status.success(), "{}", text(&output.stderr));
        let said = replies(&output.stdout).expect("replies");
        assert_eq!(said.len(), 2);
        assert_eq!(said[0].0, 0, "constructor");
        assert_eq!(said[1].0, 0, "call");
        let result: Value = serde_json::from_str(&said[1].1).expect("result");
        assert_eq!(result["value"], true);
        let capture: Value = serde_json::from_str(&backend.capture()).expect("capture");
        let body: Value =
            serde_json::from_str(capture["bodies"][0].as_str().expect("body")).expect("wire");
        assert_eq!(body["model"], row["model"]);
        let criteria = &body["questions"]["q1"]["criteria"];
        assert_eq!(
            criteria["true"],
            if row["form"] == "text" {
                json!("yes")
            } else {
                json!({"what":"yes","examples":["refund"]})
            }
        );
        if row["form"] == "both" {
            assert_eq!(criteria["false"], json!({}));
        } else {
            assert!(criteria.get("false").is_none());
        }
        assert_eq!(backend.count(), 1);
        let mut expected = conformance_backend::Paths::default();
        if name == "perplexity" {
            expected.capture_decisions = 1;
        } else {
            expected.capture_systemone = 1;
        }
        assert_eq!(backend.paths(), expected, "posting_path_mismatch: {name}");
        let bearer: Value = serde_json::from_str(&backend.bearers()).expect("counts");
        assert_eq!(
            bearer,
            json!({"markers":{name:1},"absent":0,"unknown":0,"overflow":false})
        );
        assert!(!text(&output.stdout).contains(&marker));
        assert!(!text(&output.stderr).contains(&marker));
    }
}

#[test]
fn c_backend_refusals_remain_usage_failures_and_send_nothing() {
    let backend = Backend::start().expect("backend");
    let base = format!("{}/generic/v1", backend.origin());
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    for setting in [
        r#"{"backend":1}"#,
        r#"{"backend":null}"#,
        r#"{"backend":""}"#,
        r#"{"backend":"nowhere"}"#,
        r#"{"backend":"local","backend":"other"}"#,
    ] {
        let mut script = Script::default();
        script.ask("settings", &[&base, setting]);
        let output = run(&driver, &base, &script.0);
        let said = replies(&output.stdout).expect("refusal");
        assert_eq!(said[0].0, 1, "{setting}");
        assert!(!said[0].1.contains("sk-c-door-loopback"));
    }
    assert_eq!(backend.count(), 0);
    assert_eq!(backend.paths(), conformance_backend::Paths::default());
}

#[test]
fn c_selected_setup_keeps_path_prices_and_profile_through_overrides() {
    let backend = Backend::with_markers(BTreeMap::from([("local".into(), "fake-local".into())]))
        .expect("backend");
    let base = format!("{}/arm/full/capture/v1", backend.origin());
    let home = scratch("backend-config");
    #[cfg(target_os = "macos")]
    let config = home.join("Library/Application Support/thinkthen");
    #[cfg(not(target_os = "macos"))]
    let config = home.join("thinkthen");
    std::fs::create_dir_all(&config).expect("config");
    std::fs::write(config.join("config.json"), json!({"schema":"thinkthen.config/1", "usd_per_million_input":"9", "usd_per_million_output":"9", "backends":{"local":{
        "url":base,"model":"setup","key_env":"LOCAL_KEY","path":"judgements/v2/decide", "usd_per_million_input":"1", "usd_per_million_output":"2", "profile":{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":3}
    }}}).to_string()).expect("config");
    crate::child::private_file(&config.join("config.json")).expect("private fixture configuration");
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    let mut script = Script::default();
    script.ask("env", &["LOCAL_KEY", "fake-local"]);
    script.ask(
        "settings",
        &[
            &base,
            r#"{"backend":"local","model":"override","cache":false}"#,
        ],
    );
    script.ask("call", &[&base, REQUEST]);
    let output = run_with(
        &driver,
        &base,
        &script.0,
        &[
            ("XDG_CONFIG_HOME", &home),
            ("HOME", &home),
            ("APPDATA", &home),
        ],
    );
    let said = replies(&output.stdout).expect("replies");
    assert_eq!(said[0].0, 0);
    assert_eq!(said[1].0, 1);
    assert!(said[1].1.contains("profile small"));
    assert_eq!(backend.count(), 0);

    let profile = home.join("explicit.json");
    std::fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"explicit","max_evidence_bytes":100}"#,
    )
    .expect("profile");
    let mut script = Script::default();
    script.ask("env", &["LOCAL_KEY", "fake-local"]);
    script.ask("settings", &[&base, &json!({"backend":"local","base_url":base,"model":"override","profile":profile,"cache":false}).to_string()]);
    script.ask("call", &[&base, REQUEST]);
    let output = run_with(
        &driver,
        &base,
        &script.0,
        &[
            ("XDG_CONFIG_HOME", &home),
            ("HOME", &home),
            ("APPDATA", &home),
        ],
    );
    let said = replies(&output.stdout).expect("replies");
    assert_eq!(said[0].0, 0);
    assert_eq!(said[1].0, 0);
    let result: Value = serde_json::from_str(&said[1].1).expect("result");
    assert_eq!(result["facts"]["estimated_cost_usd"], "0.000003");
    let capture: Value = serde_json::from_str(&backend.capture()).expect("capture");
    let body: Value =
        serde_json::from_str(capture["bodies"][0].as_str().expect("body")).expect("wire");
    assert_eq!(body["model"], "override");
    assert_eq!(backend.count(), 1);
    assert_eq!(
        backend.paths(),
        conformance_backend::Paths {
            capture_custom: 1,
            ..Default::default()
        }
    );
    assert!(!text(&output.stdout).contains("fake-local"));
}

#[test]
fn c_local_runtime_names_send_no_authorization_when_their_keys_are_absent() {
    let driver = compile(&crate_dir().join("tests/c/driver.c"));
    for (name, model) in [("llamacpp", "local"), ("mlx", "strands-decider-2B-hobson-v19")] {
        let backend = Backend::start().expect("backend");
        let base = format!("{}/arm/full/capture/v1", backend.origin());
        let mut script = Script::default();
        script.ask("settings", &[&base, &json!({"backend":name,"base_url":base,"cache":false}).to_string()]);
        script.ask("call", &[&base, REQUEST]);
        let output = run(&driver, &base, &script.0);
        let said = replies(&output.stdout).expect("replies");
        assert_eq!(said.iter().map(|reply| reply.0).collect::<Vec<_>>(), [0, 0]);
        let capture: Value = serde_json::from_str(&backend.capture()).expect("capture");
        let body: Value = serde_json::from_str(capture["bodies"][0].as_str().expect("body")).expect("wire");
        assert_eq!(body["model"], model);
        let bearer: Value = serde_json::from_str(&backend.bearers()).expect("counts");
        assert_eq!(bearer, json!({"markers":{},"absent":1,"unknown":0,"overflow":false}));
        assert_eq!(backend.count(), 1);
    }
}
