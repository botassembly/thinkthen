//! Canonical public identities match native complete output, without sending to compute them.
use super::*;
use thinkthen::RelationIdentityMode;

fn completed_identity(value: &Value) -> Option<&str> {
    let value = value
        .as_array()
        .and_then(|rows| rows.first())
        .unwrap_or(value);
    let meta = value.get("meta")?;
    meta.get("question_sha256")
        .or_else(|| meta.get("questions_sha256"))?
        .as_str()
}

#[test]
fn canonical_definitions_match_complete_result_identities() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    for (function, text, input) in [
        (
            "decide",
            r#"{"decide":"Fits?"}"#,
            RequestInput::Text {
                text: "Alpha".into(),
                images: vec![],
            },
        ),
        (
            "decide",
            r#"{"decide":"Fits?","threshold":"0.2:0.8","profile":"calibrated"}"#,
            RequestInput::Text {
                text: "Alpha".into(),
                images: vec![],
            },
        ),
        (
            "annotate",
            r#"{"version":1,"profile":"calibrated","questions":{"z":{"decide":"Last?"},"a":{"decide":"First?"}}}"#,
            RequestInput::Text {
                text: "Alpha".into(),
                images: vec![],
            },
        ),
        (
            "recognize",
            r#"{"version":1,"recognize":{"stage_context":{"boundary":"stage_context_marker"}}}"#,
            RequestInput::Text {
                text: "Alpha".into(),
                images: vec![],
            },
        ),
        (
            "find",
            r#"{"find":"Which unit?"}"#,
            RequestInput::Units {
                items: vec![item("Alpha"), item("Beta")],
            },
        ),
    ] {
        let definition = RequestDefinition::from_authored_json(text).unwrap();
        let before = listener.count();
        let identity = definition.question_sha256().unwrap();
        assert_eq!(listener.count(), before);
        let call = engine
            .call(
                function,
                args(definition, input),
                RequestEnvironment::default(),
            )
            .unwrap();
        let complete = serde_json::to_value(call.value()).unwrap();
        assert_eq!(
            Some(identity.as_str()),
            completed_identity(&complete),
            "{function}: {complete}"
        );
    }
}

#[test]
fn definition_identity_retains_order_and_find_none_without_stage_context() {
    let forward = RequestDefinition::from_authored_json(
        r#"{"version":1,"questions":{"z":{"decide":"Last?"},"a":{"decide":"First?"}}}"#,
    )
    .unwrap();
    let reverse = RequestDefinition::from_authored_json(
        r#"{"version":1,"questions":{"a":{"decide":"First?"},"z":{"decide":"Last?"}}}"#,
    )
    .unwrap();
    assert_ne!(
        forward.question_sha256().unwrap(),
        reverse.question_sha256().unwrap()
    );
    let recognize = |context| {
        RequestDefinition::from_authored_json(&format!(
            r#"{{"version":1,"recognize":{{"stage_context":{{"boundary":"{context}"}}}}}}"#
        ))
        .unwrap()
        .question_sha256()
        .unwrap()
    };
    assert_eq!(recognize("first"), recognize("second"));
    let find = Question::find("Which?").unwrap();
    assert_ne!(
        RequestDefinition::from(find.clone())
            .question_sha256()
            .unwrap(),
        RequestDefinition::from(find.offering_none().unwrap())
            .question_sha256()
            .unwrap()
    );
}

#[test]
fn relation_modes_match_complete_readings_and_templates_have_no_invented_identity() {
    let listener = Listener::answering(response).unwrap();
    let engine = engine(&listener);
    let definition = RequestDefinition::from_authored_json(r#"{"version":1,"relate":{"relations":[{"name":"follows","source":"*","target":"*","reads":"follows"}]}}"#).unwrap();
    let records = definition
        .relation_sha256(RelationIdentityMode::Records)
        .unwrap();
    let lines = definition
        .relation_sha256(RelationIdentityMode::Lines)
        .unwrap();
    assert_ne!(records, lines);
    assert!(definition.question_sha256().is_err());
    assert_eq!(listener.count(), 0);
    for (expected, original) in [
        (
            records,
            RequestOriginal::Json {
                value: RawRecord::json(r#"{"name":"Ada","kind":"person"}"#).unwrap(),
            },
        ),
        (lines, RequestOriginal::Text { text: "Ada".into() }),
    ] {
        let mut record = item("unused");
        record.original = Some(original);
        let call = engine
            .call(
                "relate",
                args(
                    definition.clone(),
                    RequestInput::Records {
                        items: vec![record],
                    },
                ),
                RequestEnvironment::default(),
            )
            .unwrap();
        assert_eq!(
            Some(expected.as_str()),
            completed_identity(&serde_json::to_value(call.value()).unwrap())
        );
    }
    let template: RequestDefinition = Question::choose_records("Which?").unwrap().into();
    let error = template.question_sha256().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Usage);
    assert!(!format!("{error:?}").contains("Which?"));
    assert!(
        template
            .relation_sha256(RelationIdentityMode::Records)
            .is_err()
    );
    assert_eq!(listener.count(), 0);
}
