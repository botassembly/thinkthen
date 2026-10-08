//! Public admission of per-record candidates and descriptions.

use thinkthen::{Description, RecordInput, RecordOption, RecordOptions};

#[test]
fn record_options_preserve_candidate_and_description_order() {
    let projected = RecordOptions::project(
        r#"{"candidates":{"second":{"why":"marker","examples":[1,null]},"first":null}}"#,
        "/candidates",
    )
    .unwrap();
    assert_eq!(
        projected
            .options()
            .iter()
            .map(|o| o.name.as_str())
            .collect::<Vec<_>>(),
        ["second", "first"]
    );
    assert_eq!(
        projected.options()[0]
            .description
            .as_ref()
            .unwrap()
            .as_json(),
        r#"{"why":"marker","examples":[1,null]}"#
    );
    assert!(projected.options()[1].description.is_none());
    let direct = RecordOptions::new(vec![
        RecordOption {
            name: "second".into(),
            description: Some(
                Description::from_json(r#"{"why":"marker","examples":[1,null]}"#).unwrap(),
            ),
        },
        RecordOption {
            name: "first".into(),
            description: None,
        },
    ])
    .unwrap();
    assert_eq!(direct, projected);
}

#[test]
fn invalid_record_candidates_fail_at_admission_without_disclosing_content() {
    for json in [
        r#"{"private-marker":[]}"#,
        r#"{"private-marker":["a","a"]}"#,
        r#"{"private-marker":{"a":true,"b":null}}"#,
        r#"{"private-marker":["a",false]}"#,
        r#"{"private-marker":{"a":null,"a":null}}"#,
    ] {
        assert!(RecordOptions::project(json, "/private-marker").is_err());
    }
    let error = Description::from_json(r#"{"private-marker":1,"private-marker":2}"#).unwrap_err();
    assert!(!format!("{error:?} {error}").contains("private-marker"));
    for json in ["true", "1", "NaN"] {
        assert!(Description::from_json(json).is_err());
    }
}

#[test]
fn record_debug_requires_no_original_traits_and_withholds_caller_content() {
    struct Original;
    let input = RecordInput {
        examples: None,
        seed_spans: None,
        original: Original,
        context: Some("private-marker".into()),
        options: Some(
            RecordOptions::new(vec![
                RecordOption {
                    name: "private-marker".into(),
                    description: None,
                },
                RecordOption {
                    name: "other".into(),
                    description: Some(Description::text("private-marker").unwrap()),
                },
            ])
            .unwrap(),
        ),
    };
    assert!(!format!("{input:?}").contains("private-marker"));
    assert!(!format!("{:?}", input.options.unwrap().options()[0]).contains("private-marker"));
}
