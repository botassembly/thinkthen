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
