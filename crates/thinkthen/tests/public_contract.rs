#![cfg(test)]
//! Admission and record failures keep distinct public codes.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;

use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;
use std::time::Duration;
use thinkthen::{BackendName, Code, Decide, Key, Record, Retry, Settings};

/// Gather each `code` or `error` string in a shared case.
fn case_codes(value: &Value, found: &mut BTreeSet<String>) {
    match value {
        Value::Array(items) => items.iter().for_each(|item| case_codes(item, found)),
        Value::Object(fields) => {
            for (name, field) in fields {
                match field.as_str() {
                    Some(code) if name == "code" || name == "error" => {
                        found.insert(code.to_owned());
                    }
                    _ => case_codes(field, found),
                }
            }
        }
        _ => {}
    }
}

#[test]
fn shared_cases_carry_only_registry_codes() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../conformance");
    let mut folders = vec![root];
    let mut found = BTreeSet::new();
    while let Some(folder) = folders.pop() {
        for entry in std::fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                folders.push(path);
            } else if path
                .extension()
                .is_some_and(|extension| extension == "json")
            {
                let case = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
                case_codes(&case, &mut found);
            }
        }
    }
    // One code from each spelling, so a walk that reads nothing cannot pass.
    for code in ["usage.question", "local.cancelled", "backend.transport"] {
        assert!(found.contains(code), "{code} left the shared cases");
    }
    found.retain(|code| !Code::ALL.iter().any(|known| known.as_str() == code));
    assert!(found.is_empty(), "not in the registry: {found:?}");
}

#[test]
fn settings_admission_preserves_distinct_codes() {
    for (raw, code) in [
        ("{", "usage.settings"),
        (r#"{"backend":"unknown"}"#, "usage.backend"),
        (r#"{"backend":"stub","batch":0}"#, "usage.batch"),
        (r#"{"backend":"stub","batch":1.5}"#, "usage.batch"),
        (r#"{"backend":"stub","unexpected":true}"#, "usage.settings"),
    ] {
        assert_eq!(Settings::from_json(raw).unwrap_err().code().as_str(), code);
    }
    let error = Settings::default().engine(|_| None).unwrap_err();
    assert_eq!(error.code().as_str(), "usage.key-missing");
}
#[test]
fn scalar_record_and_transport_failures_are_call_errors() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let question = Decide::new("Ready?").unwrap();
    let error = engine.run(&question, Record::from_json("{")).unwrap_err();
    assert_eq!(error.code().as_str(), "record.json");
    let unavailable = loopback::UnavailableBackend::new().unwrap();
    let engine = Settings::default()
        .url(unavailable.base())
        .api_key(Key::new(loopback::KEY).unwrap())
        .retry(Retry::new(0, Duration::ZERO, Duration::from_secs(2)).unwrap())
        .engine(|_| None)
        .unwrap();
    let error = engine.run(&question, Record::text("alpha")).unwrap_err();
    assert_eq!(error.code().as_str(), "backend.transport");
}
#[test]
fn diagnostic_controls_are_escaped_once_before_host_translation() {
    for (control, escaped) in [
        ("\0", "\\u0000"),
        ("\n", "\\n"),
        ("\t", "\\t"),
        ("\r", "\\r"),
        ("\u{1b}", "\\u001b"),
        ("\u{7f}", "\\u007f"),
        ("\u{85}", "\\u0085"),
    ] {
        let raw = serde_json::json!({"question":{"function":"decide","question":"Ready?",format!("x{control}suffix"):true}});
        let error = thinkthen::Request::from_json(&raw.to_string()).unwrap_err();
        let message = error.to_string();
        assert_eq!(error.code().as_str(), "usage.question");
        assert!(!message.contains(control), "{message:?}");
        assert!(message.contains(escaped), "{message:?}");
        assert_eq!(serde_json::to_value(error).unwrap()["message"], message);
    }
}

#[test]
fn shared_usage_diagnostics_name_the_path_shape_and_safe_received_value() {
    let cases: serde_json::Value = serde_json::from_str(include_str!(
        "../../../conformance/public-contract/usage-admission.json"
    ))
    .unwrap();
    for case in cases.as_array().unwrap() {
        assert_eq!(case["kind"], "admission");
        let input = case["input"].to_string();
        let error = match case["boundary"].as_str().unwrap() {
            "question" => thinkthen::Question::from_json(&input).unwrap_err(),
            "toolbox" => thinkthen::questions::Toolbox::from_json(&input).unwrap_err(),
            "settings" => Settings::from_json(&input).unwrap_err(),
            _ => unreachable!(),
        };
        #[cfg(feature = "native")]
        if case["boundary"] == "settings" {
            let sql = thinkthen::native::sql_engine_unrestricted(&input, |_| {
                panic!("admission must precede credentials")
            })
            .unwrap_err();
            assert_eq!(sql, error);
        }
        assert_eq!(
            error.code().as_str(),
            case["code"].as_str().unwrap(),
            "{}",
            case["name"]
        );
        if let Some(message) = case["message"].as_str() {
            assert_eq!(error.message(), message, "{}", case["name"]);
        }
        if let Some(parts) = case["contains"].as_array() {
            for part in parts {
                assert!(
                    error.message().contains(part.as_str().unwrap()),
                    "{}: {error}",
                    case["name"]
                );
            }
        }
        assert!(!error.message().contains("secret-placeholder"));
    }
}

#[test]
fn usage_diagnostics_redact_payloads_and_bound_unknown_names() {
    let secret = format!("private-{}", "x".repeat(1000));
    for field in ["batch", "retry", "api_key"] {
        let value = if field == "api_key" {
            serde_json::json!([secret])
        } else {
            serde_json::json!(secret)
        };
        let error =
            Settings::from_json(&serde_json::json!({field: value}).to_string()).unwrap_err();
        assert!(!error.message().contains(&secret));
        assert!(error.message().len() < 512, "{error}");
    }
    for field in [
        format!("{secret}: expected "),
        format!("{secret}`, expected `spoof"),
    ] {
        let error = thinkthen::Question::from_json(
            &serde_json::json!({"function":"decide","question":"Ready?",field:true}).to_string(),
        )
        .unwrap_err();
        assert!(!error.message().contains(&secret));
        assert!(error.message().len() < 512, "{error}");
    }
}

#[test]
fn invalid_numeric_credentials_are_never_rendered() {
    let error = Settings::from_json(r#"{"api_key":123456789}"#).unwrap_err();
    assert!(!error.message().contains("123456789"));
    assert!(error.message().contains("settings.api_key:"));
    assert!(error.message().contains("contents redacted"));
}

#[test]
fn duplicate_toolbox_names_keep_their_actionable_shared_diagnostic() {
    let error = thinkthen::questions::Toolbox::from_json(r#"{"entries":{"ready":{"question":{"function":"decide","question":"Ready?"}},"ready":{"question":{"function":"decide","question":"Ready?"}}}}"#).unwrap_err();
    assert_eq!(
        error.message(),
        "toolbox.entries.names: expected unique question names; received a repeated name"
    );
}

#[test]
fn typed_admission_identifies_fields_without_disclosing_values() {
    let errors = [
        thinkthen::Decide::new(" ").unwrap_err(),
        thinkthen::Score::with_strings("Ready?", ["only"]).unwrap_err(),
        thinkthen::Tag::with_strings("Ready?", ["same", "same"]).unwrap_err(),
        "secret-placeholder"
            .parse::<thinkthen::Pointer>()
            .unwrap_err(),
        thinkthen::Cut::new(0.0).unwrap_err(),
    ];
    for (error, field) in errors
        .iter()
        .zip(["question", "levels", "labels", "field", "threshold"])
    {
        assert!(
            error.message().starts_with(&format!("{field}: expected ")),
            "{error}"
        );
        assert!(error.message().contains("; received "), "{error}");
        assert!(!error.message().contains("secret-placeholder"));
    }
}

#[cfg(feature = "native")]
#[test]
fn native_builder_setters_report_the_field_and_preserve_the_question() {
    use thinkthen::{FunctionName, native::QuestionBuilder};
    let mut question = QuestionBuilder::new(FunctionName::Score, "Priority?").unwrap();
    question.add_item(FunctionName::Score, "low").unwrap();
    question.add_item(FunctionName::Score, "high").unwrap();
    for (error, field) in [
        (question.set_cut(0.5).unwrap_err(), "threshold"),
        (question.set_top(1).unwrap_err(), "top"),
        (
            question
                .add_item(FunctionName::Tag, "private-placeholder")
                .unwrap_err(),
            "domain",
        ),
    ] {
        assert!(
            error.message().starts_with(&format!("{field}: expected ")),
            "{error}"
        );
        assert!(error.message().contains("; received "), "{error}");
        assert!(!error.message().contains("private-placeholder"));
    }
    assert_eq!(question.build().unwrap().function(), FunctionName::Score);
    let error = "private-placeholder".parse::<BackendName>().unwrap_err();
    assert!(
        error
            .message()
            .contains("unknown value \"private-placeholder\""),
        "{error}"
    );
    assert!(error.message().contains("did you mean"));
}

#[cfg(feature = "native")]
#[test]
fn sql_relation_admission_identifies_the_rule_and_field() {
    for (field, expected) in [
        ("relation", "a string"),
        ("source", "a string"),
        ("target", "a string"),
        ("reads", "a string"),
        ("either", "a boolean"),
    ] {
        let mut rule = serde_json::json!({"relation":"supports","source":"fact","target":"claim"});
        rule[field] = serde_json::json!(1);
        let input = serde_json::json!({"function":"relate","rules":[rule]});
        let error = thinkthen::native::sql_question(&input.to_string()).unwrap_err();
        assert_eq!(error.code().as_str(), "usage.question");
        assert_eq!(
            error.message(),
            format!("question.rules[0].{field}: expected {expected}; received integer `1`")
        );
    }
}
