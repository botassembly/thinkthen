use super::{EntitySetError, RelateSpec};

#[test]
fn file_defaults_precedence_and_canonical_identity_are_stable() {
    let mut spec = RelateSpec::parse(r#"{"version":1,"relate":{"relations":[{"name":"works_for","source":"person","target":"organization"}]},"profile":"measured"}"#).expect("file");
    let structured = spec.question(false);
    assert_eq!(
        crate::core::json_line(&structured).expect("canonical question"),
        r#"{"verb":"relate","fields":{"name":"/name","kind":"/kind"},"relations":[{"name":"works_for","source":"person","target":"organization","reads":"works for","either":false}],"threshold":0.5,"profile":"measured"}"#
    );
    let first = structured.sha256().expect("digest");
    assert_eq!(
        first,
        "5bf4b6e3a94525500f77984526f1307cf751986e656f4bb1ededee24c01909d4"
    );
    spec.override_threshold("0.7").expect("command cut");
    spec.override_fields(Some("/title"), None)
        .expect("name pointer");
    assert_ne!(spec.question(false).sha256().expect("digest"), first);
    assert_eq!(spec.name_field().as_str(), "/title");
    assert_eq!(spec.kind_field().as_str(), "/kind");
}

#[test]
fn a_line_question_hashes_null_fields_and_its_digest_ignores_the_runtime_model() {
    let mut spec = RelateSpec::inline(&["linked".to_owned()], true).expect("inline");
    let question = spec.question(true);
    assert_eq!(
        crate::core::json_line(&question).expect("canonical question"),
        r#"{"verb":"relate","fields":null,"relations":[{"name":"linked","source":"*","target":"*","reads":"linked","either":true}],"threshold":0.5}"#
    );
    let digest = question.sha256().expect("digest");
    assert_eq!(
        digest,
        "5d1aa1f27838358f922a082922c989994bb79461f6b6e694be95b5a3577af794"
    );
    spec.model = Some(crate::core::ModelName::new("another").expect("model"));
    assert_eq!(spec.question(true).sha256().expect("digest"), digest);
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
        "works_for:person",
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

fn entities(count: usize) -> Vec<(String, String)> {
    (0..count)
        .map(|place| (format!("entity-{place}"), "record".to_owned()))
        .collect()
}

#[test]
fn a_complete_set_admits_255_entities_and_refuses_the_256th() {
    let spec = RelateSpec::inline(&["linked".to_owned()], false).expect("inline");
    assert_eq!(spec.admit(&entities(255)).expect("255 entities").len(), 255);
    assert_eq!(spec.admit(&entities(256)), Err(EntitySetError::TooMany));
}

#[test]
fn a_complete_set_refuses_blanks_duplicates_and_absent_concrete_kinds() {
    let pair = |name: &str, kind: &str| (name.to_owned(), kind.to_owned());
    let typed =
        RelateSpec::inline(&["works_for=person:organization".to_owned()], false).expect("inline");
    assert_eq!(
        typed.admit(&[pair("Ada", "person"), pair("Ada", "person")]),
        Err(EntitySetError::Duplicate)
    );
    assert_eq!(
        typed.admit(&[pair(" ", "person")]),
        Err(EntitySetError::Blank)
    );
    assert_eq!(
        typed.admit(&[pair("Ada", "person")]),
        Err(EntitySetError::AbsentKind)
    );
    let admitted = typed
        .admit(&[pair("Ada", "person"), pair("Ada", "organization")])
        .expect("one name under two kinds is two entities");
    assert_eq!(admitted.len(), 2);
    assert_eq!(typed.admit(&[]), Ok(Vec::new()));
}

#[test]
fn line_input_takes_only_bare_or_wildcard_rules() {
    for rule in ["linked", "linked=*:*"] {
        let spec = RelateSpec::inline(&[rule.to_owned()], false).expect("inline");
        assert_eq!(spec.check_lines(), Ok(()), "{rule}");
    }
    for rule in ["linked=record:record", "linked=*:record"] {
        let spec = RelateSpec::inline(&[rule.to_owned()], false).expect("inline");
        assert_eq!(spec.check_lines(), Err(EntitySetError::LineRule), "{rule}");
    }
}
