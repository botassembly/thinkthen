//! Reserved activation is a local admission refusal through real public routes.

use conformance_backend::{Canned, Listener};
use thinkthen::{
    CallOptions, CodeThreshold, Engine, ErrorKind, ProxyActivation, ProxyId, ProxyMetadata,
    ProxyOverride, ProxyRequest, Question,
};

fn activations() -> Result<[ProxyActivation; 3], thinkthen::Error> {
    Ok([
        ProxyActivation::Null,
        ProxyActivation::Empty,
        ProxyActivation::Request(ProxyRequest {
            question_id: Some(ProxyId::new("private-marker")?),
            code_threshold: CodeThreshold::cut(0.5)?,
            code_relation_threshold: None,
        }),
    ])
}

#[test]
fn every_supplied_proxy_form_refuses_public_execution_with_zero_sends_and_no_started_facts() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .no_cache()
        .build()
        .unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    for activation in activations().unwrap() {
        let error = engine
            .decide_with(
                &question,
                "private-marker",
                CallOptions::new().proxy(&activation),
            )
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.to_string(),
            "proxy activation is reserved and is not supported in 0.2"
        );
        assert!(error.facts().is_none());
        assert!(!format!("{error:?}").contains("private-marker"));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn proxy_engine_admission_precedes_local_profile_and_replay_access() {
    let listener = Listener::answering(|_| Canned::ok("{}")).unwrap();
    for activation in activations().unwrap() {
        let builder = Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .replay("/this-reserved-input-must-not-open-a-store")
            .unwrap();
        let error = builder.proxy(&activation).build().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
        assert_eq!(
            error.to_string(),
            "proxy activation is reserved and is not supported in 0.2"
        );
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn reserved_carriers_reuse_threshold_grammar_and_withhold_opaque_ids_and_values_in_debug() {
    for value in ["", " leading", "line\nbreak", "é", "a/b"] {
        let error = ProxyId::new(value).unwrap_err();
        assert!(!error.to_string().contains(value) || value.is_empty());
    }
    assert!(ProxyId::new("a".repeat(128)).is_ok());
    assert!(ProxyId::new("a".repeat(129)).is_err());
    for value in [0.0, -0.1, 1.1, f64::NAN, f64::INFINITY] {
        assert!(CodeThreshold::cut(value).is_err());
    }
    assert!(CodeThreshold::band(0.7, 0.3).is_err());
    assert_eq!(
        serde_json::to_string(&CodeThreshold::none()).unwrap(),
        "null"
    );
    assert_eq!(
        serde_json::to_string(&CodeThreshold::cut(0.5).unwrap()).unwrap(),
        "0.5"
    );
    assert_eq!(
        serde_json::to_string(&CodeThreshold::band(0.2, 0.8).unwrap()).unwrap(),
        r#""0.2:0.8""#
    );
    assert_eq!(
        serde_json::to_string(&ProxyActivation::Null).unwrap(),
        "null"
    );
    assert_eq!(
        serde_json::to_string(&ProxyActivation::Empty).unwrap(),
        "{}"
    );
    struct Original;
    let metadata = ProxyMetadata {
        decision_id: ProxyId::new("private-marker").unwrap(),
        reading: ProxyOverride::Decision {
            code_threshold: CodeThreshold::cut(0.5).unwrap(),
            code_value: Original,
            value: Original,
        },
    };
    assert!(!format!("{metadata:?}").contains("private-marker"));
}
