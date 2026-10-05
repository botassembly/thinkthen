//! Provider setups through CLI and captured Rust builders, with counted sends.
use super::support::{Home, listener, said};
use crate::child::ChildEnvironment as _;
use serde_json::{Value, json};
use std::io::Write as _;
use std::process::Command;
use thinkthen::{Engine, EngineBuilder, Question};

const PROFILE: &str =
    r#"{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":3}"#;
const PAIR_ERROR: &str = "configuration backend fields `usd_per_million_input` and `usd_per_million_output` come together as decimal strings from 0 through 1000000 with at most six fractional digits";
const BUILTIN_ERROR: &str = "a configuration entry for a built-in backend holds only `requests_per_minute`, `usd_per_million_input`, `usd_per_million_output`, and `profile`";

fn config(home: &Home, base: &str, fields: Value) {
    let mut entry = json!({"url":base,"key_env":"LOCAL_D1_KEY","model":"m"});
    entry
        .as_object_mut()
        .expect("an entry object")
        .extend(fields.as_object().expect("setup field object").clone());
    home.config(&json!({"schema":"thinkthen.config/1","usd_per_million_input":"1","usd_per_million_output":"0","backends":{"small":entry}}).to_string());
}

fn ask(home: &Home, base: &str, extra: &[&str]) -> std::process::Output {
    let evidence = home.evidence(&["alpha"]);
    let mut args = vec![
        "decide",
        "a refund?",
        "--url",
        base,
        "--input",
        &evidence,
        "--no-cache",
    ];
    args.extend_from_slice(extra);
    home.run(&args, &[])
}

#[test]
fn malformed_setup_fields_refuse_with_exact_value_free_sentences_and_zero_sends() {
    let target = listener();
    let home = Home::new("setup-refusal-0400");
    let mut cases = vec![
        (
            json!({"both_sides":"sk-field-marker-0400"}),
            "configuration backend field `both_sides` must be true or false".to_owned(),
        ),
        (
            json!({"profile":"sk-field-marker-0400"}),
            "configuration backend field `profile` is one JSON object".to_owned(),
        ),
        (
            json!({"profile":{"schema":"sk-field-marker-0400"}}),
            "configuration backend field `profile` has schema `thinkthen.backend-profile/1`"
                .to_owned(),
        ),
        (
            json!({"profile":{"schema":"thinkthen.backend-profile/1","name":"small"}}),
            "configuration backend field `profile` names at least one limit".to_owned(),
        ),
        (
            json!({"profile":{"schema":"thinkthen.backend-profile/1","name":"small","max_questions":1,"unknown":"sk-field-marker-0400"}}),
            "configuration backend field `profile` holds no unknown keys".to_owned(),
        ),
    ];
    for value in [
        json!("1e3"),
        json!("1000000.000001"),
        json!("1.0000001"),
        json!(null),
        json!(true),
        json!(2),
        json!("sk-field-marker-0400"),
    ] {
        cases.push((
            json!({"usd_per_million_input":value,"usd_per_million_output":"0"}),
            PAIR_ERROR.to_owned(),
        ));
    }
    cases.push((json!({"usd_per_million_output":"0"}), PAIR_ERROR.to_owned()));
    for (fields, sentence) in cases {
        config(&home, target.base(), fields);
        let out = ask(&home, target.base(), &["--backend", "small"]);
        let (stdout, stderr) = said(&out);
        assert_eq!(out.status.code(), Some(5));
        assert_eq!(stdout, "");
        assert_eq!(stderr, format!("thinkthen: {sentence}\n"));
        assert!(!stderr.contains("sk-field-marker"));
        assert_eq!(target.count(), 0);
    }
    for fields in [
        json!({}),
        json!({"url":"sk-field-marker-0400"}),
        json!({"path":"decisions"}),
        json!({"key_env":"K"}),
        json!({"model":"m"}),
        json!({"both_sides":false}),
    ] {
        home.config(
            &json!({"schema":"thinkthen.config/1","backends":{"typesafe":fields}}).to_string(),
        );
        let out = ask(&home, target.base(), &[]);
        assert_eq!(out.status.code(), Some(5));
        assert!(said(&out).1.contains(BUILTIN_ERROR));
        assert_eq!(target.count(), 0);
    }
    let raw = format!(
        r#"{{"schema":"thinkthen.config/1","backends":{{"small":{{"url":"{}","key_env":"LOCAL_D1_KEY","model":"m","profile":{{"schema":"thinkthen.backend-profile/1","name":"small","max_questions":1,"max_questions":2}}}}}}}}"#,
        target.base()
    );
    home.config(&raw);
    let out = ask(&home, target.base(), &["--backend", "small"]);
    assert_eq!(out.status.code(), Some(5));
    assert!(
        said(&out)
            .1
            .contains("configuration backend field `profile` is not valid JSON")
    );
    assert_eq!(target.count(), 0);
}

#[test]
fn setup_profile_refuses_before_live_replay_cache_and_explicit_file_wins() {
    let target = listener();
    let home = Home::new("setup-profile-0400");
    config(
        &home,
        target.base(),
        json!({"profile":serde_json::from_str::<Value>(PROFILE).unwrap()}),
    );
    for extra in [vec![], vec!["--replay", "missing-recording"]] {
        let mut args = vec!["--backend", "small"];
        args.extend(extra);
        let out = ask(&home, target.base(), &args);
        assert_eq!(out.status.code(), Some(2), "{}", said(&out).1);
        assert!(said(&out).1.contains("profile small"));
        assert_eq!(target.count(), 0);
    }
    let profile = home.root.join("explicit.json");
    std::fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"explicit","max_evidence_bytes":100}"#,
    )
    .unwrap();
    let out = ask(
        &home,
        target.base(),
        &["--backend", "small", "--profile", profile.to_str().unwrap()],
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
    assert_eq!(target.count(), 1);
}

#[test]
fn named_prices_survive_address_model_overrides_unnamed_uses_top_level_and_bytes_match() {
    let target = listener();
    let home = Home::new("setup-prices-0400");
    config(
        &home,
        "http://127.0.0.1:1/v1",
        json!({"usd_per_million_input":"2","usd_per_million_output":"0"}),
    );
    for (extra, cost) in [
        (
            vec!["--backend", "small", "--model", "override", "--facts"],
            "0.000018",
        ),
        (vec!["--model", "override", "--facts"], "0.000009"),
    ] {
        let out = ask(&home, target.base(), &extra);
        assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
        let stderr = said(&out).1;
        let facts: Value = serde_json::from_str(stderr.lines().last().unwrap()).unwrap();
        assert_eq!(facts["estimated_cost_usd"], cost);
    }
    assert_eq!(target.count(), 2);
    let requests = target.requests();
    assert_eq!(requests[0].body, requests[1].body);
}

#[test]
#[ignore = "parent executes this with an isolated captured environment"]
fn backend_setup_builder_child() {
    let case = std::env::var("THINKTHEN_TEST_SETUP_CASE").expect("the parent selects a case");
    let mut builder = if case == "bare" {
        Engine::builder()
    } else {
        EngineBuilder::from_env().unwrap()
    };
    if case == "explicit-first" {
        builder = builder.prices_usd_per_million("3", "0").unwrap();
    }
    if case != "unnamed" && case != "bare" {
        builder = builder.backend("small").unwrap();
    }
    builder = builder
        .base_url(&std::env::var("THINKTHEN_BASE_URL").expect("the parent supplies a loopback URL"))
        .unwrap()
        .model("override")
        .unwrap();
    if case == "explicit-last" {
        builder = builder.prices_usd_per_million("3", "0").unwrap();
    }
    if case == "profile" {
        builder=builder.profile_json(r#"{"schema":"thinkthen.backend-profile/1","name":"explicit","max_evidence_bytes":100}"#).unwrap();
    }
    let mut out = std::io::stdout().lock();
    writeln!(out, "setup-debug {builder:?}").unwrap();
    let engine = builder.no_cache().build().unwrap();
    let q = Question::decide("a refund?").unwrap().cut();
    match engine.details(&q, "alpha") {
        Ok(call) => writeln!(out, "setup-cost {:?}", call.facts().estimated_cost_usd()).unwrap(),
        Err(error) => writeln!(out, "setup-error {error}").unwrap(),
    }
}

#[test]
fn captured_builder_price_provenance_and_explicit_profile_setters() {
    let target = listener();
    let home = Home::new("setup-builder-0400");
    for (case, cost) in [
        ("named", "0.000018"),
        ("unnamed", "0.000009"),
        ("explicit-first", "0.000027"),
        ("explicit-last", "0.000027"),
        ("bare", ""),
    ] {
        config(
            &home,
            target.base(),
            json!({"usd_per_million_input":"2","usd_per_million_output":"0"}),
        );
        let out = builder_child(&home, target.base(), case);
        let stdout = said(&out).0;
        if cost.is_empty() {
            assert!(stdout.contains("setup-cost None"), "{stdout}");
        } else {
            assert!(
                stdout.contains(&format!("setup-cost Some(\"{cost}\")")),
                "{stdout}"
            );
        }
    }
    assert_eq!(target.count(), 5);
    config(
        &home,
        target.base(),
        json!({"profile":serde_json::from_str::<Value>(PROFILE).unwrap()}),
    );
    let out = builder_child(&home, target.base(), "named");
    assert!(said(&out).0.contains("setup-error"));
    assert_eq!(target.count(), 5);
    let out = builder_child(&home, target.base(), "profile");
    assert!(said(&out).0.contains("setup-cost Some(\"0.000009\")"));
    assert_eq!(target.count(), 6);
}

fn builder_child(home: &Home, base: &str, case: &str) -> std::process::Output {
    let mut command = Command::new(std::env::current_exe().expect("this test binary"));
    command
        .clear_environment()
        .args([
            "named_backends::backend_setups::backend_setup_builder_child",
            "--exact",
            "--ignored",
            "--test-threads=1",
            "--nocapture",
        ])
        .env("THINKTHEN_TEST_SETUP_CASE", case)
        .env("THINKTHEN_BASE_URL", base);
    for (name, value) in home.environment() {
        command.env(name, value);
    }
    let out = crate::run::output(&mut command).expect("the child runs");
    assert!(out.status.success(), "{}", said(&out).0);
    out
}

#[test]
fn both_sides_reuses_wire_form_and_profiles_prices_keep_recording_identity() {
    let target = listener();
    let home = Home::new("setup-wire-0400");
    for both in [false, true] {
        config(
            &home,
            target.base(),
            json!({"both_sides":both,"path":"nested/decisions","requests_per_minute":60000,"usd_per_million_input":"2","usd_per_million_output":"0","profile":{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":100}}),
        );
        let out = ask(
            &home,
            target.base(),
            &["--backend", "small", "--true", "refund requested"],
        );
        assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
        let requests = target.requests();
        let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
        assert_eq!(requests[0].line, "POST /v1/nested/decisions HTTP/1.1");
        let criteria = &body["questions"]["q1"]["criteria"];
        assert_eq!(criteria.get("false").is_some(), both);
        if both {
            assert_eq!(criteria["false"], json!({}));
        }
    }
    let folder = home.path("recording");
    config(&home, target.base(), json!({}));
    let out = ask(
        &home,
        target.base(),
        &["--backend", "small", "--record", &folder],
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
    assert_eq!(target.count(), 3);
    config(
        &home,
        target.base(),
        json!({"usd_per_million_input":"2","usd_per_million_output":"0","profile":{"schema":"thinkthen.backend-profile/1","name":"different","max_evidence_bytes":100}}),
    );
    let out = ask(
        &home,
        target.base(),
        &["--backend", "small", "--replay", &folder, "--facts"],
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
    assert_eq!(
        target.count(),
        3,
        "setup prices/profile do not change replay identity"
    );
    assert!(said(&out).1.contains("\"estimated_cost_usd\":\"0.000000\""));
    home.assert_no_marker_in_files();
}

#[test]
fn configured_both_sides_satisfies_strict_server_and_absent_field_keeps_authored() {
    use conformance_backend::{Canned, Listener};
    let target = Listener::answering(|bytes| {
        let request: Value = serde_json::from_slice(bytes).unwrap();
        let criteria = &request["questions"]["q1"]["criteria"];
        if criteria.get("true").is_some() && criteria.get("false").is_none() {
            Canned::status(400, r#"{"error":"both criteria required"}"#)
        } else {
            Canned::ok(r#"{"model":"m","answers":{"q1":{"type":"noul","noul":0.92}}}"#)
        }
    })
    .unwrap();
    let home = Home::new("strict-0400");
    for (fields, code) in [
        (json!({}), 4),
        (json!({"both_sides":true}), 0),
        (json!({"both_sides":false}), 4),
    ] {
        config(&home, target.base(), fields);
        let out = ask(
            &home,
            target.base(),
            &["--backend", "small", "--true", "refund requested"],
        );
        assert_eq!(out.status.code(), Some(code), "{}", said(&out).1);
    }
    assert_eq!(target.count(), 3);
}

#[test]
fn setup_profile_refuses_cache_lookup_and_supplies_running_calibration_name() {
    let target = listener();
    let home = Home::new("setup-cache-warning-0400");
    let input = home.evidence(&["alpha"]);
    let cache = home.path("cache");
    config(
        &home,
        target.base(),
        json!({"profile":serde_json::from_str::<Value>(PROFILE).unwrap()}),
    );
    let out = home.run(
        &[
            "decide",
            "a refund?",
            "--backend",
            "small",
            "--input",
            &input,
            "--cache",
            &cache,
        ],
        &[],
    );
    assert_eq!(out.status.code(), Some(2), "{}", said(&out).1);
    assert!(
        said(&out)
            .1
            .contains("profile small allows at most 3 evidence bytes")
    );
    assert_eq!(target.count(), 0);
    config(
        &home,
        target.base(),
        json!({"profile":{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":100}}),
    );
    let question = home.root.join("question.json");
    std::fs::write(
        &question,
        r#"{"decide":"a refund?","threshold":0.8,"profile":"other"}"#,
    )
    .unwrap();
    let out = home.run(
        &[
            "decide",
            &format!("@{}", question.display()),
            "--backend",
            "small",
            "--input",
            &input,
            "--no-cache",
        ],
        &[],
    );
    assert_eq!(out.status.code(), Some(0), "{}", said(&out).1);
    assert!(
        said(&out)
            .1
            .contains("threshold tuned for profile other is running under profile small")
    );
    assert_eq!(target.count(), 1);
}
