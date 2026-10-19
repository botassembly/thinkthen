//! Safe typed native diagnostics never retain provider prose or caller evidence.
use conformance_backend::{Canned, Listener};
use thinkthen::{
    Engine, ErrorDiagnostic, ErrorKind, RequestArguments, RequestEnvironment, RequestInput,
    RequestOptions, RequestQuestion,
};

fn arguments() -> RequestArguments {
    RequestArguments {
        question: RequestQuestion::Text { text: "private_diagnostic_question_0639".into() },
        input: RequestInput::Text { text: "private_diagnostic_evidence_0639".into(), images: vec![] },
        options: RequestOptions::default(),
    }
}

#[test]
fn provider_identifiers_exclude_secrets_originals_and_raw_parser_bodies() {
    let key = "private_diagnostic_credential_0639";
    for reply in [
        Canned::status(400, r#"{"error":{"message":"private_diagnostic_body_0639","type":"private_diagnostic_credential_0639","code":"invalid_request"}}"#),
        Canned::status(400, r#"{"error":{"message":"private_diagnostic_body_0639","type":"private_diagnostic_evidence_0639","code":"invalid_request"}}"#),
        Canned::ok(r#"{"model":"fixed","answers":private_diagnostic_body_0639}"#),
    ] {
        let listener = Listener::serving(vec![reply]).unwrap();
        let engine = Engine::builder().base_url(listener.base()).unwrap()
            .model("fixed").unwrap().api_key(key).unwrap()
            .no_cache().max_retries(0).build().unwrap();
        let error = engine.call("decide", arguments(), RequestEnvironment::default()).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Backend);
        let diagnostic = error.diagnostic().expect("typed native cause");
        let mut cause = &diagnostic;
        while let ErrorDiagnostic::Range { cause: inner, .. } = cause {
            cause = inner;
        }
        match cause {
            ErrorDiagnostic::Provider { provider, .. } => {
                assert!(provider.error_type.is_none());
                assert_eq!(provider.code, Some("invalid_request"));
            }
            ErrorDiagnostic::Reply(message)
            | ErrorDiagnostic::PartialReply { cause: Some(message), .. } => assert!(message.contains("line")),
            other => panic!("unexpected diagnostic: {other:?}"),
        }
        let shown = [format!("{error:?}"), error.to_string(),
            serde_json::to_string(&error.complete()).unwrap(), format!("{diagnostic:?}")];
        for output in shown {
            for secret in [key, "private_diagnostic_body_0639", "private_diagnostic_evidence_0639", "private_diagnostic_question_0639"] {
                assert!(!output.contains(secret));
            }
        }
        assert!(std::error::Error::source(&error).is_none());
        assert_eq!(listener.count(), 1);
    }
}

fn assert_private(error: &thinkthen::Error, marker: &str) {
    for output in [format!("{error:?}"), error.to_string(),
        serde_json::to_string(&error.complete()).unwrap(), format!("{:?}", error.diagnostic())] {
        assert!(!output.contains(marker), "private bytes in {output}");
    }
    assert!(std::error::Error::source(error).is_none());
}

#[test]
fn native_json_categories_and_locations_never_reveal_parser_input() {
    use thinkthen::{JsonFailure, ParsedJson, RawRecord};
    let marker = "private_parser_evidence_0639";
    let deep = format!("{}\"{marker}\"{}", "[".repeat(130), "]".repeat(130));
    let cases = [
        (format!(r#"{{"{marker}":0,"{marker}":1}}"#), JsonFailure::DuplicateName),
        (format!(r#"{{"{marker}":1e9999}}"#), JsonFailure::NotFinite),
        (deep, JsonFailure::TooDeep),
    ];
    for (text, expected) in cases {
        let error = ParsedJson::from_json(&text).unwrap_err();
        assert!(matches!(error.diagnostic(), Some(ErrorDiagnostic::Json(actual)) if actual == expected));
        assert_private(&error, marker);
        let error = RawRecord::json(&text).unwrap_err();
        assert!(matches!(error.diagnostic(), Some(ErrorDiagnostic::RecordJson(actual)) if actual == expected));
        assert_private(&error, marker);
    }
    let error = ParsedJson::from_json(&format!("{{\n\"{marker}\":?}}")).unwrap_err();
    assert!(matches!(error.diagnostic(), Some(ErrorDiagnostic::Json(JsonFailure::Syntax { line: 2, column })) if column > 0));
    assert_private(&error, marker);
    let error = thinkthen::Question::from_json(&format!("{{\"{marker}\":?}}")).unwrap_err();
    assert!(matches!(error.diagnostic(), Some(ErrorDiagnostic::QuestionJson { failure: JsonFailure::Syntax { .. }, from_file: true })));
    assert_private(&error, marker);
}

#[test]
fn profile_grammar_has_closed_safe_classifications() {
    use thinkthen::{JsonFailure, ProfileFailure};
    let marker = "private_profile_evidence_0639";
    for (text, expected) in [
        (format!(r#"{{"{marker}":?}}"#), ProfileFailure::Json(JsonFailure::Syntax { line: 1, column: marker.len() + 5 })),
        ("[]".into(), ProfileFailure::Object),
        ("{}".into(), ProfileFailure::Schema),
        (format!(r#"{{"schema":"thinkthen.backend-profile/1","name":"{marker} !"}}"#), ProfileFailure::Name),
        (format!(r#"{{"{marker}":1}}"#), ProfileFailure::UnknownKey),
        (r#"{"schema":"thinkthen.backend-profile/1","name":"valid","max_questions":0}"#.into(), ProfileFailure::Limit("max_questions")),
        (r#"{"schema":"thinkthen.backend-profile/1","name":"valid"}"#.into(), ProfileFailure::NoLimit),
        (r#"{"schema":"thinkthen.backend-profile/2","name":"valid","image_profile":null}"#.into(), ProfileFailure::ImageProfile),
    ] {
        let error = Engine::builder().profile_json(&text).err().unwrap();
        assert!(matches!(error.diagnostic(), Some(ErrorDiagnostic::Profile { failure, from_file: false }) if failure == expected), "{:?}", error.diagnostic());
        assert_private(&error, marker);
    }
}

#[test]
fn model_validation_projects_closed_admission_without_authored_values() {
    for (model, control) in [(" ", false), ("private_model_marker\nembedded", true), ("private_model_marker\tembedded", true)] {
        let error = Engine::builder().model(model).err().unwrap();
        assert!(matches!((error.diagnostic(), control),
            (Some(ErrorDiagnostic::ModelControl), true) | (Some(ErrorDiagnostic::ModelBlank), false)));
        assert_private(&error, "private_model_marker");
    }
}
