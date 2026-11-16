#![cfg(test)]
//! Saved question toolboxes use the command request envelope.
use std::collections::BTreeMap;
use thinkthen::{Decide, Record, Settings, Toolbox};

#[test]
fn saved_toolbox_round_trips_questions_and_preserves_execution() {
    let original = Toolbox::new(BTreeMap::from([(
        "ready".into(),
        Decide::new("Ready?").unwrap().into(),
    )]))
    .unwrap();
    let saved = original.to_json().unwrap();
    let value: serde_json::Value = serde_json::from_str(&saved).unwrap();
    assert_eq!(value["entries"]["ready"]["question"]["function"], "decide");
    assert!(value["entries"]["ready"].get("records").is_none());
    assert_eq!(serde_json::to_string(&original).unwrap(), saved);
    let loaded: Toolbox = serde_json::from_str(&saved).unwrap();
    assert_eq!(
        Toolbox::from_json(&saved).unwrap().to_json().unwrap(),
        saved
    );
    let engine = Settings::from_json(r#"{"backend":"stub"}"#)
        .unwrap()
        .engine(|_| None)
        .unwrap();
    let run = |toolbox: &Toolbox| {
        let thinkthen::SimpleQuestion::Decide(question) = toolbox.get("ready").unwrap() else {
            panic!("saved toolbox changed its question function")
        };
        serde_json::to_value(engine.run(question, Record::text("Ready to go")).unwrap()).unwrap()
    };
    assert_eq!(run(&original), run(&loaded));
}

#[test]
fn saved_toolbox_rejects_malformed_entries_with_shared_usage_codes() {
    for saved in [
        r#"{"entries":{"ready":{"question":{"function":"choose","question":"Which?","choices":["one"]}}}}"#,
        r#"{"entries":{"ready":{"question":{"function":"decide","question":"Ready?"}},"ready":{"question":{"function":"decide","question":"Other?"}}}}"#,
        r#"{"entries":{"ready":{"question":{"function":"decide","question":"Ready?"},"records":[]}}}"#,
    ] {
        assert_eq!(
            Toolbox::from_json(saved).unwrap_err().code().as_str(),
            "usage.question",
            "{saved}"
        );
    }
}

#[test]
fn saved_toolbox_names_are_portable_filenames() {
    for name in ["../ready", "..", "C:ready"] {
        let json = serde_json::json!({"entries": {name: {"question": {"function": "decide", "question": "Ready?"}}}}).to_string();
        assert_eq!(
            Toolbox::from_json(&json).unwrap_err().code().as_str(),
            "usage.name"
        );
    }
}

#[test]
fn saved_toolbox_bounds_both_input_and_output_bytes() {
    let original = Toolbox::new(BTreeMap::from([(
        "ready".into(),
        Decide::new("Ready?").unwrap().into(),
    )]))
    .unwrap();
    let mut saved = original.to_json().unwrap();
    saved.extend(std::iter::repeat_n(' ', 1024 * 1024 - saved.len()));
    assert!(Toolbox::from_json(&saved).is_ok());
    saved.push(' ');
    assert_eq!(
        Toolbox::from_json(&saved).unwrap_err().code().as_str(),
        "usage.question"
    );
    let oversized = Toolbox::new(BTreeMap::from([(
        "ready".into(),
        Decide::new("q".repeat(1024 * 1024)).unwrap().into(),
    )]))
    .unwrap();
    assert_eq!(
        oversized.to_json().unwrap_err().code().as_str(),
        "usage.question"
    );
}

#[test]
fn shared_saved_toolbox_preserves_every_simple_function_and_its_options() {
    let fixture = include_str!("../../../conformance/toolbox/questions.json");
    let original = Toolbox::from_json(fixture).unwrap();
    assert_eq!(original.iter().count(), 9);
    let saved = original.to_json().unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&saved).unwrap(),
        serde_json::from_str::<serde_json::Value>(fixture).unwrap()
    );
    let loaded: Toolbox = serde_json::from_str(&saved).unwrap();
    assert_eq!(loaded.to_json().unwrap(), saved);
}
