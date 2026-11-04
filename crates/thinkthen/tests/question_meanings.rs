#![cfg(test)]
//! Question selection has one public JSON spelling and no compatibility alias.

#[test]
fn field_is_the_only_serialized_selection_key() {
    let question: thinkthen::Question = serde_json::from_value(serde_json::json!({
        "function":"decide", "question":"Ready?", "field":"/body"
    }))
    .unwrap();
    let value = serde_json::to_value(question).unwrap();
    assert_eq!(value["field"], "/body");
    assert!(value.get("evidence").is_none());
    assert!(
        serde_json::from_value::<thinkthen::Question>(serde_json::json!({
            "function":"decide", "question":"Ready?", "evidence":"/body"
        }))
        .is_err()
    );
}
