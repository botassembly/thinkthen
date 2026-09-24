use super::RelateSpec;

fn digest(spec: &RelateSpec) -> String {
    use sha2::{Digest as _, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(crate::core::json_line(spec).expect("question").as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn file_defaults_precedence_and_canonical_identity_are_stable() {
    let mut spec = RelateSpec::parse(r#"{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"}]},"profile":"measured"}"#).expect("file");
    assert_eq!(
        crate::core::json_line(&spec).expect("canonical question"),
        r#"{"verb":"relate","fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}],"threshold":0.5,"profile":"measured"}"#
    );
    let first = digest(&spec);
    assert_eq!(
        first,
        "5bf4b6e3a94525500f77984526f1307cf751986e656f4bb1ededee24c01909d4"
    );
    spec.override_threshold("0.7").expect("command cut");
    spec.override_fields(Some("/title"), None)
        .expect("name pointer");
    assert_ne!(digest(&spec), first);
    assert_eq!(spec.name_field().as_str(), "/title");
    assert_eq!(spec.kind_field().as_str(), "/kind");
}

#[test]
fn inline_grammar_and_closed_file_shape_are_refused() {
    assert!(RelateSpec::inline(&["depends_on".to_owned()], true).is_ok());
    assert!(RelateSpec::inline(&["works_for=person:organization".to_owned()], false).is_ok());
    for relation in [
        "=person:organization",
        "works_for=person",
        "works_for=:organization",
        "works_for=person:organization:extra",
    ] {
        assert!(
            RelateSpec::inline(&[relation.to_owned()], false).is_err(),
            "{relation}"
        );
    }
    for text in [
        r#"{"version":1,"relate":{"relations":[]}}"#,
        r#"{"version":1,"relate":{"relations":[{"name":"x","source":"a","target":"b","extra":true}]}}"#,
        r#"{"version":2,"relate":{"relations":[{"name":"x","source":"a","target":"b"}]}}"#,
    ] {
        assert!(RelateSpec::parse(text).is_err(), "{text}");
    }
}
