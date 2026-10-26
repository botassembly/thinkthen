#![cfg(test)]
//! Named typed question domains retain order and reject malformed members.
use serde_json::json;
use thinkthen::Toolbox;
#[test]
fn toolbox_names_are_ordered_and_selection_is_explicit() {
    let toolbox = Toolbox::from_definitions_json(&json!({"own":{"function":"decide","question":"Yes?","evidence":"/text"},"inherited":{"function":"decide","question":"Yes?"}}).to_string()).unwrap();
    assert_eq!(
        toolbox.iter().map(|(name, _)| name).collect::<Vec<_>>(),
        ["inherited", "own"]
    );
    assert!(toolbox.get("missing").is_none());
    let question: thinkthen::Question = toolbox.get("own").unwrap().clone().into();
    assert_eq!(question.evidence_pointer().to_string(), "/text");
}
#[test]
fn malformed_toolboxes_refuse_duplicate_names_and_invalid_domains() {
    for text in [
        "",
        "[]",
        "{}",
        "{",
        r#"{" ":{"function":"decide","question":"Yes?"}}"#,
        r#"{"q":{"function":"decide","question":"Yes?"},"q":{"function":"decide","question":"No?"}}"#,
        r#"{"q":{"function":"decide","question":"Yes?","model":"route"}}"#,
        r#"{"q":{"function":"choose","question":"Which?","choices":["a","a"]}}"#,
        r#"{"q":{"function":"tag","question":"Which?","labels":[]}}"#,
        r#"{"q":{"function":"score","question":"How?","levels":["only"]}}"#,
    ] {
        assert_eq!(
            Toolbox::from_definitions_json(text)
                .unwrap_err()
                .code()
                .as_str(),
            "usage.question",
            "{text}"
        );
    }
}

#[test]
fn toolbox_admission_bounds_input_bytes() {
    let mut text = r#"{"ready":{"function":"decide","question":"Yes?"}}"#.to_owned();
    text.extend(std::iter::repeat_n(' ', 1024 * 1024 - text.len()));
    assert!(Toolbox::from_definitions_json(&text).is_ok());
    text.push(' ');
    assert_eq!(
        Toolbox::from_definitions_json(&text)
            .unwrap_err()
            .code()
            .as_str(),
        "usage.question"
    );
}
