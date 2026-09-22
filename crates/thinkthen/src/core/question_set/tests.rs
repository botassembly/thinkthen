use super::{QuestionSet, QuestionSetError};
use proptest::prelude::*;

#[test]
fn pinned_set_has_the_contract_digest() {
    let set = QuestionSet::parse(
        r#"{"version":1,"questions":{"refund":{"decide":"Does this ask for a refund?"}}}"#,
    )
    .expect("valid set");
    assert_eq!(
        set.sha256().expect("digest"),
        "4318689ccd64c08b788ea48c5f72b8dca279cf3d482173ed280f3fe243158b62"
    );
}

#[test]
fn grammar_errors_name_the_full_path() {
    let cases = [
        (
            r#"{"version":1,"questions":{}}"#,
            "`questions` holds at least",
        ),
        (
            r#"{"version":1,"questions":{"Bad":{"decide":"x"}}}"#,
            "questions.Bad",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":"x","model":"jev-latest"}}}"#,
            "questions.ok.model",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":"x","extra":1}}}"#,
            "questions.ok.extra",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":"x","on":"not-a-pointer"}}}"#,
            "questions.ok.on",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"choose":"x","options":true}}}"#,
            "questions.ok.options",
        ),
        (
            r#"{"version":1,"threshold":true,"questions":{"ok":{"decide":"x"}}}"#,
            "`threshold`",
        ),
        (r#"{"version":1,"questions":{"ok":true}}"#, "`questions.ok`"),
        (
            r#"{"version":1,"questions":{"ok":{"threshold":0.5}}}"#,
            "`questions.ok`",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":"x","choose":"y"}}}"#,
            "`questions.ok`",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"score":"x","threshold":0.5,"levels":["low","high"]}}}"#,
            "questions.ok.threshold",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":" "}}}"#,
            "questions.ok.decide",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"score":"x","levels":["only"]}}}"#,
            "questions.ok.levels",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":"x","on":["/a/id","/b/id"]}}}"#,
            "questions.ok.on",
        ),
    ];
    for (text, wanted) in cases {
        let error = QuestionSet::parse(text).expect_err("refused");
        assert!(error.to_string().contains(wanted), "{error}");
    }
    assert_eq!(QuestionSet::parse("[]"), Err(QuestionSetError::NotObject));
    for (text, path) in [
        (
            r#"{"version":1,"version":1,"questions":{"ok":{"decide":"x"}}}"#,
            "version",
        ),
        (
            r#"{"version":1,"questions":{"ok":{"decide":"x","decide":"y"}}}"#,
            "questions.ok.decide",
        ),
    ] {
        assert_eq!(
            QuestionSet::parse(text),
            Err(QuestionSetError::Duplicate(path.to_owned()))
        );
    }
    let path = "questions.key-marker.evidence-marker";
    let error = QuestionSetError::Duplicate(path.to_owned());
    assert!(error.to_string().contains(path));
    assert!(!format!("{error:?}").contains("key-marker"));
    assert!(!format!("{error:?}").contains("evidence-marker"));
}

#[test]
fn a_missing_questions_wrapper_precedes_unknown_top_level_keys() {
    assert_eq!(
        QuestionSet::parse(r#"{"version":1,"unresolved":{"decide":"x"}}"#),
        Err(QuestionSetError::MissingQuestions)
    );
    assert_eq!(
        QuestionSetError::MissingQuestions.to_string(),
        "the question set is missing its `questions` object"
    );
}

#[test]
fn groups_keep_first_question_order() {
    let set = QuestionSet::parse(r#"{"version":1,"questions":{"a":{"decide":"a","on":"/x"},"b":{"score":"b","levels":["low","high"],"on":"/y"},"c":{"decide":"c","on":"/x"}}}"#).expect("valid set");
    assert_eq!(set.groups(), vec![vec![0, 2], vec![1]]);
}

#[test]
fn absent_and_explicit_root_are_one_behavior_and_one_group() {
    let absent = QuestionSet::parse(
        r#"{"version":1,"questions":{"a":{"decide":"a"},"b":{"decide":"b","on":""}}}"#,
    )
    .expect("valid set");
    assert_eq!(absent.groups(), vec![vec![0, 1]]);
    let explicit = QuestionSet::parse(
        r#"{"version":1,"questions":{"a":{"decide":"a","on":""},"b":{"decide":"b"}}}"#,
    )
    .expect("valid set");
    assert_eq!(
        absent.sha256().expect("digest"),
        explicit.sha256().expect("digest")
    );
}

proptest! {
    #[test]
    fn whitespace_and_an_explicit_default_keep_one_resolved_digest(
        name in "[a-z][a-z0-9_]{0,15}",
        question in "[A-Za-z][A-Za-z0-9 ?]{0,40}",
    ) {
        let compact = format!(r#"{{"version":1,"questions":{{"{name}":{{"decide":"{question}"}}}}}}"#);
        let spaced = format!(r#"{{ "version" : 1, "questions" : {{ "{name}" : {{ "decide" : "{question}", "threshold" : 0.5 }} }} }}"#);
        let first = QuestionSet::parse(&compact).expect("generated set parses");
        let second = QuestionSet::parse(&spaced).expect("generated set parses");
        prop_assert_eq!(first.sha256().expect("digest"), second.sha256().expect("digest"));
    }

    #[test]
    fn generated_question_grammar_round_trips_through_resolved_json(
        name in "[a-z][a-z0-9_]{0,15}",
        question in "[A-Za-z][A-Za-z0-9 ?]{0,40}",
        kind in 0_u8..4,
    ) {
        let held = match kind {
            0 => format!(r#"{{"decide":"{question}","threshold":"0.2:0.8"}}"#),
            1 => format!(r#"{{"choose":"{question}","options":["first","second"],"threshold":0.7}}"#),
            2 => format!(r#"{{"tag":"{question}","labels":{{"first":null,"second":"Second label."}},"threshold":0.7}}"#),
            _ => format!(r#"{{"score":"{question}","levels":["low","high"]}}"#),
        };
        let compact = format!(r#"{{"version":1,"questions":{{"{name}":{held}}}}}"#);
        let first = QuestionSet::parse(&compact).expect("generated set parses");
        let resolved = first.resolved_json().expect("resolved set renders");
        let second = QuestionSet::parse(&resolved).expect("resolved set parses again");
        prop_assert_eq!(&first, &second);
        prop_assert_eq!(first.sha256().expect("digest"), second.sha256().expect("digest"));
    }
}
