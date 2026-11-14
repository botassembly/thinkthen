#![cfg(test)]
//! Saved requests use the same explicit meaning names as native question builders.
use serde_json::json;
use thinkthen::{Decide, Question, Request};

#[test]
fn meaning_keys_round_trip_and_old_keys_are_refused() {
    let question = Decide::new("Refund?")
        .unwrap()
        .yes_means("refund requested")
        .no_means("keep the payment");
    let saved = serde_json::to_value(&question).unwrap();
    assert_eq!(saved["yes_means"], "refund requested");
    assert_eq!(saved["no_means"], "keep the payment");
    assert!(saved.get("true").is_none());
    assert!(saved.get("false").is_none());
    let request = json!({"question":{"function":"decide","question":"Refund?","yes_means":"refund requested","no_means":"keep the payment"}});
    let admitted = Request::from_json(&request.to_string()).unwrap();
    assert_eq!(
        serde_json::to_value(admitted).unwrap()["question"]["yes_means"],
        "refund requested"
    );
    for key in ["true", "false"] {
        let old = json!({"function":"decide","question":"Refund?",(key):"old meaning"});
        assert!(Question::from_json(&old.to_string()).is_err());
    }
}
