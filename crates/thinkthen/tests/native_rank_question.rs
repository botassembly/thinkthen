//! Public rank-specific admission retains the shared described question grammar.

use thinkthen::{Description, LoadedQuestion, Question, QuestionKind};

#[test]
fn described_rank_matches_the_saved_decide_criteria_and_model() {
    let built = Question::rank_described(
        "Relevant?",
        Some(Description::from_json(r#"{"means":"direct","examples":["yes"]}"#).unwrap()),
        Some(Description::text("irrelevant").unwrap()),
    )
    .unwrap()
    .with_model("literal-model")
    .unwrap();
    let saved = Question::rank_from_json(
        r#"{"decide":"Relevant?","true":{"means":"direct","examples":["yes"]},"false":"irrelevant","model":"literal-model"}"#,
    ).unwrap();
    assert_eq!(saved, built);
    assert_eq!(saved.kind(), QuestionKind::Rank);
}

#[test]
fn saved_score_rank_keeps_ordered_levels_and_descriptions() {
    let json = r#"{"score":"Relevant?","levels":{"low":{"meaning":"unrelated"},"medium":{"meaning":"related"},"high":{"meaning":"direct"}},"model":"literal-model","batch":2}"#;
    let ranked = Question::rank_from_json(json).unwrap();
    let LoadedQuestion::Question(scored) = Question::from_json(json).unwrap() else {
        panic!("a score has no band");
    };
    assert_eq!(ranked, scored);
    assert_eq!(ranked.kind(), QuestionKind::Score);
}

#[test]
fn rank_rejects_bands_score_thresholds_and_other_functions() {
    for json in [
        r#"{"decide":"Relevant?","threshold":"0.2:0.8"}"#,
        r#"{"score":"Relevant?","levels":["low","high"],"threshold":0.5}"#,
        r#"{"score":"Relevant?","levels":["low","high"],"true":"meaning"}"#,
        r#"{"choose":"Relevant?","options":["a","b"]}"#,
    ] {
        assert!(Question::rank_from_json(json).is_err(), "{json}");
    }
}
