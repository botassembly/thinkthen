//! Native engine settings admission before environment capture.
use super::*;

#[test]
fn independent_settings_refusals_precede_environment_capture_and_hide_values() {
    for text in [
        "[]",
        "[null]",
        r#"{"proxy":null,"proxy":{}}"#,
        r#"{"proxy":{"private-marker":}}"#,
        r#"{"timeout":0}"#,
        r#"{"timeout":86401}"#,
        r#"{"throttle":0}"#,
        r#"{"max_requests":0}"#,
        r#"{"max_request_bytes":0}"#,
        r#"{"cache":""}"#,
        r#"{"model":" "}"#,
        r#"{"record":""}"#,
        r#"{"replay":""}"#,
        r#"{"profile":""}"#,
        r#"{"usd_per_million_input":"1"}"#,
        r#"{"usd_per_million_input":"private-price","usd_per_million_output":"2"}"#,
        r#"{"batch":"private-batch"}"#,
        r#"{"timeout":"private-timeout"}"#,
        r#"{"cache":true}"#,
        r#"{"refresh_cache":null}"#,
        r#"{"max_requests_total":"private-limit"}"#,
        r#"{"max_requests_total":null,"max_requests_total":2}"#,
        r#"{"unknown_control":"private-value"}"#,
    ] {
        let error = EngineBuilder::validate_settings_json(text).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        let expected = shown::<()>(Err(error));
        let said = in_child(
            "settings-json",
            &[(ARGUMENT, text), ("THINKTHEN_BASE_URL", "ftp://invalid")],
        );
        assert_eq!(said, expected, "{text}");
        assert!(!said.contains("private-"), "{said}");
    }
    let said = in_child(
        "settings-json",
        &[(ARGUMENT, "{}"), ("THINKTHEN_BASE_URL", "ftp://invalid")],
    );
    assert!(said.starts_with("Usage: THINKTHEN_BASE_URL:"), "{said}");
}

#[test]
fn supplied_proxy_settings_refuse_before_capture_and_hide_opaque_values() {
    let listener = listener();
    for value in [
        "null",
        "{}",
        "[]",
        "false",
        "42",
        r#""private-marker""#,
        r#"{"unknown":"private-marker"}"#,
    ] {
        let text = format!(r#"{{"proxy":{value},"replay":"/private-marker-missing-store"}}"#);
        let error = EngineBuilder::validate_settings_json(&text).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.to_string(),
            "proxy activation is reserved and is not supported in 0.2"
        );
        assert!(!format!("{error:?}").contains("private-marker"));
        for base in ["ftp://invalid", listener.base()] {
            let said = in_child(
                "settings-json",
                &[(ARGUMENT, &text), ("THINKTHEN_BASE_URL", base)],
            );
            assert_eq!(said, format!("Usage: {}", error));
            assert!(!said.contains("private-marker"));
        }
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn settings_preserve_null_budgets_and_omitted_defaults() {
    for text in [
        "{}",
        r#"{"batch":"max","timeout":30,"cache":false}"#,
        r#"{"max_requests":null,"max_requests_total":null,"max_estimated_input_tokens_total":null}"#,
        r#"{"max_requests_total":0,"max_estimated_input_tokens_total":0}"#,
        r#"{"refresh_cache":false}"#,
        r#"{"backend":"configured-later"}"#,
    ] {
        EngineBuilder::validate_settings_json(text).expect(text);
    }
    let listener = listener();
    for (text, sent) in [
        ("{}", false),
        (r#"{"max_estimated_input_tokens_total":null}"#, true),
        (r#"{"max_estimated_input_tokens_total":0}"#, false),
    ] {
        let before = listener.count();
        let said = in_child(
            "settings-budget",
            &[
                (ARGUMENT, text),
                ("THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL", "1"),
                ("THINKTHEN_BASE_URL", listener.base()),
                ("THINKTHEN_API_KEY", "sk-fake-settings-loopback"),
            ],
        );
        assert_eq!(said.starts_with("sent 1 cached false"), sent, "{said}");
        assert_eq!(listener.count() - before, usize::from(sent));
        if !sent {
            assert!(said.starts_with("Usage:"), "{said}");
        }
    }
}

pub(super) fn budget(text: &str) -> Vec<String> {
    let engine = EngineBuilder::from_settings_json(text)
        .expect("admitted settings")
        .no_cache()
        .build()
        .expect("configured engine");
    vec![ask(&engine)]
}
