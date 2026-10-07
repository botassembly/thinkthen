use super::{join, parts, split, stored, validate_exchange};
use crate::core::adapters::{ApiType, built_in::DecodeError};
use crate::core::{Evidence, Json, Labels, ModelName, Plan, Question, QuestionText};
#[cfg(test)]
fn predicate(text: &str) -> Question {
    Question::Decide {
        text: QuestionText::new(text).unwrap(),
        yes: None,
        no: None,
    }
}
#[cfg(test)]
fn plan(questions: Vec<Question>) -> Plan {
    Plan::authored(
        Evidence::new("Evidence.").unwrap(),
        ModelName::new("fixed").unwrap(),
        questions,
    )
    .unwrap()
}
#[test]
fn captured_actual_bodies_validate_against_original_names_and_typed_menus() {
    for (request, response) in [
        (
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/01-decide.request.json"
            )
            .as_slice(),
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/01-decide.response.json"
            )
            .as_slice(),
        ),
        (
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/02-choose.request.json"
            )
            .as_slice(),
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/02-choose.response.json"
            )
            .as_slice(),
        ),
        (
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/03-score.request.json"
            )
            .as_slice(),
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/03-score.response.json"
            )
            .as_slice(),
        ),
        (
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/04-tag20.request.json"
            )
            .as_slice(),
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/04-tag20.response.json"
            )
            .as_slice(),
        ),
        (
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/05-find100.request.json"
            )
            .as_slice(),
            include_bytes!(
                "../../../../../../specification/fixtures/openai-decisions/05-find100.response.json"
            )
            .as_slice(),
        ),
    ] {
        validate_exchange(request, response).unwrap();
    }
}
#[test]
fn structured_wording_both_meanings_and_empty_descriptions_are_not_dropped() {
    let text = QuestionText::structured(
        &Json::parse(r#"{"name":"authored","what":"Ask?","extra":[false,null]}"#).unwrap(),
    )
    .unwrap();
    let yes =
        crate::core::Meaning::structured(&Json::parse(r#"{"what":"yes","extra":false}"#).unwrap())
            .unwrap();
    let no = crate::core::Meaning::structured(&Json::Object(Vec::new())).unwrap();
    let plan = plan(vec![Question::Decide {
        text,
        yes: Some(yes),
        no: Some(no),
    }]);
    let parts = parts(&plan).unwrap();
    assert_eq!(
        parts.questions[0],
        r#"{"type":"predicate","instructions":"{\"name\":\"authored\",\"what\":\"Ask?\",\"extra\":[false,null]}\nTrue means: {\"what\":\"yes\",\"extra\":false}\nFalse means: {}"}"#
    );
    assert!(!parts.questions[0].contains("\"name\":\"q"));
}
#[test]
fn persistent_questions_and_answers_exclude_only_transport_names() {
    let plan = plan(vec![predicate("A?"), predicate("B?")]);
    let parts = parts(&plan).unwrap();
    let together = join(
        &parts.state,
        &parts.model,
        parts.questions.iter().map(String::as_str),
    );
    let alone = join(
        &parts.state,
        &parts.model,
        std::iter::once(parts.questions[1].as_str()),
    );
    assert!(
        std::str::from_utf8(&together)
            .unwrap()
            .contains("\"name\":\"q2\"")
    );
    assert!(
        std::str::from_utf8(&alone)
            .unwrap()
            .contains("\"name\":\"q1\"")
    );
    let wire = br#"{"model":"fixed","answers":[{"type":"predicate","name":"q1","probability":0.1},{"type":"predicate","name":"q2","probability":0.9}]}"#;
    let split = split(plan.questions(), wire).unwrap();
    assert_eq!(
        split.answers[1].as_ref().unwrap(),
        r#"{"type":"predicate","probability":0.9}"#
    );
    assert_eq!(
        stored(&plan.questions()[1], split.answers[1].as_ref().unwrap())
            .unwrap()
            .yes(),
        Some(0.9)
    );
    assert!(
        stored(
            &plan.questions()[1],
            r#"{"type":"predicate","name":"q2","probability":0.9}"#
        )
        .is_err()
    );
}
#[test]
fn malformed_envelopes_fail_whole_and_bad_positions_preserve_independent_answers() {
    let questions = vec![predicate("A?"), predicate("B?")];
    for body in [
        r#"{"model":"fixed","answers":[{"type":"predicate","name":"q1","probability":0.9},{"type":"predicate","name":"q1","probability":0.2}]}"#,
        r#"{"model":"fixed","answers":[{"type":"predicate","name":"q1","probability":0.9,"probability":0.1}]}"#,
        r#"{"model":"fixed","answers":[{"type":"predicate","name":"q1","probability":0.9},{"type":"predicate","name":"q2","probability":0.2},{"type":"predicate","name":"q3","probability":0.2}]}"#,
    ] {
        assert!(split(&questions, body.as_bytes()).is_err());
    }
    for answer in [
        r#"{"type":"refusal","name":"q2","message":"private"}"#,
        r#"{"type":"predicate","name":"wrong","probability":0.9}"#,
        r#"{"type":"predicate","name":"q2","probability":1.1}"#,
        r#"{"type":"predicate","name":"q2","probability":"private"}"#,
    ] {
        let body = format!(
            r#"{{"model":"fixed","answers":[{{"type":"predicate","name":"q1","probability":0.9}},{answer}],"usage":{{"input_tokens":17,"output_tokens":0}}}}"#
        );
        let decoded = split(&questions, body.as_bytes()).unwrap();
        assert!(decoded.answers[0].is_ok());
        assert!(decoded.answers[1].is_err());
        assert_eq!(decoded.usage.unwrap().input_tokens(), Some(17));
        assert!(!format!("{:?}", decoded.answers[1]).contains("private"));
    }
    let missing = split(
        &questions,
        br#"{"model":"fixed","answers":[{"type":"predicate","name":"q1","probability":0.9}]}"#,
    )
    .unwrap();
    assert!(matches!(
        missing.answers[1],
        Err(DecodeError::MissingAnswer(1))
    ));
}
#[test]
fn choices_require_string_values_exact_members_confidence_and_strict_totals() {
    let question = Question::Choose {
        text: QuestionText::new("Pick?").unwrap(),
        options: Labels::options(vec!["true".into(), "false".into()]).unwrap(),
    };
    let valid = r#"{"type":"choice","choice":"false","probabilities":[{"value":"false","probability":0.8},{"value":"true","probability":0.2}],"confidence":0.4}"#;
    let decoded = stored(&question, valid).unwrap();
    assert_eq!(decoded.named().unwrap(), [("true", 0.2), ("false", 0.8)]);
    for bad in [
        valid.replace("\"false\"", "false"),
        valid.replace("0.8", "0.79"),
        valid.replace("\"confidence\":0.4", "\"confidence\":null"),
        valid.replace(
            "\"true\",\"probability\":0.2",
            "\"false\",\"probability\":0.2",
        ),
    ] {
        assert!(stored(&question, &bad).is_err());
    }
    assert_eq!(ApiType::Decisions.request_id_header(), "x-request-id");
}

#[test]
fn packed_byte_limits_count_local_names_and_escaped_structured_input_exactly() {
    let plan = Plan::authored(
        Evidence::structured(
            Json::parse(r#"{"key":"quoted \"text\"","values":[false,null]}"#).unwrap(),
        )
        .unwrap(),
        ModelName::new("fixed").unwrap(),
        vec![predicate("A?"), predicate("B?")],
    )
    .unwrap();
    let asks = crate::core::pack::asks_for(
        ApiType::Decisions,
        &crate::core::Url::new("http://127.0.0.1:9/v1/decisions").unwrap(),
        &plan,
    )
    .unwrap();
    let state = &asks[0].state;
    let model = crate::core::pack::model_json("fixed").unwrap();
    let mut measured = state.base_bytes(&model);
    for (index, ask) in asks.iter().enumerate() {
        measured += ApiType::Decisions.added_bytes(index, ask.question.len());
    }
    let body = state.body(&model, asks.iter().map(|ask| &*ask.question));
    assert_eq!(measured, body.len());
    let parsed = Json::parse(std::str::from_utf8(&body).unwrap()).unwrap();
    assert_eq!(parsed.member("input").unwrap().as_str(), Some(state.json()));
}

#[test]
fn explicit_names_choose_api_type_and_credential_host_guard_runs_without_key_access() {
    use crate::core::backend::named::{self, Named};
    let choice =
        named::choose(&[(Some("openai"), Some("https://api.typesafe.ai/v1"))], &[]).unwrap();
    let backend = choice.backend(None, "unused").unwrap();
    assert_eq!(backend.api_type(), ApiType::Decisions);
    assert!(matches!(
        choice.guard(&backend),
        Err(crate::core::BackendError::KeyElsewhere {
            owner: "openai",
            other: "typesafe",
            ..
        })
    ));
    for base in ["https://API%2EOPENAI.COM./v1", "https://api.openai.com/v1"] {
        let entries =
            [Named::new("custom", base, "TYPESAFE_API_KEY", "fixed").with_path("decisions")];
        let choice = named::choose(&[(Some("custom"), None)], &entries).unwrap();
        let backend = choice.backend(None, "unused").unwrap();
        assert_eq!(backend.api_type(), ApiType::Primary);
        assert!(matches!(
            choice.guard(&backend),
            Err(crate::core::BackendError::KeyElsewhere {
                owner: "typesafe",
                other: "openai",
                ..
            })
        ));
    }
    let choice = named::choose(&[(None, Some("https://api.openai.com/v1"))], &[]).unwrap();
    assert_eq!(
        choice.backend(None, "fixed").unwrap().api_type(),
        ApiType::Primary
    );
    let choice = named::choose(&[(Some("openai"), Some("http://127.0.0.1:9/v1"))], &[]).unwrap();
    assert!(
        choice
            .guard(&choice.backend(None, "fixed").unwrap())
            .is_ok()
    );
}
