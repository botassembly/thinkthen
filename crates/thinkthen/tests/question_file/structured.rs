//! Structured question text and descriptions, through the compiled command.

#![allow(
    clippy::expect_used,
    reason = "a malformed plan line should stop the boundary test"
)]

use crate::harness::{printed, refused, written};

#[test]
fn each_verb_carries_a_structured_question_text_into_the_plan() {
    let cases = [
        (
            "decide",
            r#"{"decide":{"ask":"Does this ask for a refund?","lang":"en"}}"#,
            r#""instructions":{"ask":"Does this ask for a refund?","lang":"en"}"#,
        ),
        (
            "choose",
            r#"{"choose":["Ask which team."],"options":["billing","other"]}"#,
            r#""instructions":["Ask which team."]"#,
        ),
        (
            "score",
            r#"{"score":{"ask":"How much disruption?"},"levels":["low","high"]}"#,
            r#""instructions":{"ask":"How much disruption?"}"#,
        ),
    ];
    for (index, (verb, file, wanted)) in cases.into_iter().enumerate() {
        let written = written(&format!("structured-text-{index}"), file);
        let plan = printed(&[verb, &written, "--dry-run"]);
        assert!(plan.contains(wanted), "{file}: {plan}");
    }
}

#[test]
fn a_structured_tag_question_text_makes_array_instructions() {
    let written = written(
        "structured-tag-text",
        r#"{"tag":["Which topics?"],"labels":["billing","urgent"]}"#,
    );
    let plan = printed(&["tag", &written, "--dry-run"]);
    for label in ["billing", "urgent"] {
        let wanted = format!(r#""instructions":[["Which topics?"],{{"label":"{label}"}}]"#);
        assert!(plan.contains(&wanted), "{label}: {plan}");
    }
}

#[test]
fn one_structured_tag_description_expands_every_label_into_an_array() {
    let written = written(
        "structured-tag-null",
        r#"{"tag":"Which topics?","labels":{"billing":{"what":"Money"},"urgent":"Urgent."}}"#,
    );
    let plan = printed(&["tag", &written, "--dry-run"]);
    let billing =
        r#""instructions":["Which topics?",{"label":"billing","description":{"what":"Money"}}]"#;
    let urgent = r#""instructions":["Which topics?",{"label":"urgent","description":"Urgent."}]"#;
    assert!(plan.contains(billing), "{plan}");
    assert!(plan.contains(urgent), "{plan}");
}

#[test]
fn a_null_tag_description_is_omitted_from_the_array_and_the_criteria() {
    let written = written(
        "tag",
        r#"{"tag":["Which topics?"],"labels":{"billing":null,"urgent":"Urgent."}}"#,
    );
    let plan = printed(&["tag", &written, "--dry-run"]);
    let billing = r#""instructions":[["Which topics?"],{"label":"billing"}]"#;
    assert!(plan.contains(billing), "{plan}");
}

#[test]
fn true_and_false_carry_structure_and_null_into_the_criteria() {
    let written = written(
        "structured-decide-criteria",
        r#"{"decide":"Does this ask for a refund?","true":{"means":"Money back."},"false":null}"#,
    );
    let plan = printed(&["decide", &written, "--dry-run"]);
    assert!(
        plan.contains(r#""criteria":{"true":{"means":"Money back."},"false":null}"#),
        "{plan}"
    );
}

#[test]
fn described_options_and_a_described_score_map_reach_the_plan() {
    let choose = written(
        "structured-choose-map",
        r#"{"choose":"Which team?","options":{"billing":{"what":"Money and invoices."},"other":null}}"#,
    );
    let plan = printed(&["choose", &choose, "--dry-run"]);
    assert!(
        plan.contains(r#""criteria":{"billing":{"what":"Money and invoices."},"other":null}"#),
        "{plan}"
    );

    let score = written(
        "structured-score-map",
        r#"{"score":"How much?","levels":{"low":{"what":"Little disruption."},"high":null}}"#,
    );
    let plan = printed(&["score", &score, "--dry-run"]);
    assert!(
        plan.contains(r#""criteria":[{"what":"Little disruption."},null]"#),
        "{plan}"
    );
}

#[test]
fn a_score_map_of_nulls_sends_nulls_where_a_list_sends_names() {
    let listed = written(
        "structured-score-list",
        r#"{"score":"How much?","levels":["low","high"]}"#,
    );
    let mapped = written(
        "structured-score-nulls",
        r#"{"score":"How much?","levels":{"low":null,"high":null}}"#,
    );
    let listed_plan = printed(&["score", &listed, "--dry-run"]);
    let mapped_plan = printed(&["score", &mapped, "--dry-run"]);
    assert!(
        listed_plan.contains(r#""criteria":["low","high"]"#),
        "{listed_plan}"
    );
    assert!(
        mapped_plan.contains(r#""criteria":[null,null]"#),
        "{mapped_plan}"
    );
}

#[test]
fn the_file_refuses_non_text_question_and_description_slots() {
    let cases = [
        (
            "decide",
            r#"{"decide":null}"#,
            "`decide` in the question file is text, an object, or a list",
        ),
        (
            "decide",
            r#"{"decide":7}"#,
            "`decide` in the question file is text, an object, or a list",
        ),
        (
            "decide",
            r#"{"decide":true}"#,
            "`decide` in the question file is text, an object, or a list",
        ),
        (
            "decide",
            r#"{"decide":"x","true":7}"#,
            "`true` in the question file is text, an object, a list, or null",
        ),
        (
            "decide",
            r#"{"decide":"x","false":true}"#,
            "`false` in the question file is text, an object, a list, or null",
        ),
        (
            "choose",
            r#"{"choose":"x","options":{"a":3,"b":"y"}}"#,
            "`options` in the question file is a list of labels, or a map from each label to its description",
        ),
        (
            "choose",
            r#"{"choose":"x","options":{"a":true,"b":"y"}}"#,
            "`options` in the question file is a list of labels, or a map from each label to its description",
        ),
        (
            "tag",
            r#"{"tag":"x","labels":{"a":3,"b":"y"}}"#,
            "`labels` in the question file is a list of labels, or a map from each label to its description",
        ),
        (
            "score",
            r#"{"score":"x","levels":["low",{"what":"high"}]}"#,
            "`levels` in the question file is a list of levels, lowest first, or a map from each level to its description",
        ),
        (
            "score",
            r#"{"score":"x","levels":{"low":"   ","high":"ok"}}"#,
            "the question file's `levels`: a description is text, not white space",
        ),
        (
            "decide",
            r#"{"decide":{"a":{"a":1,"a":2}}}"#,
            "the question file is not JSON this tool reads: a JSON record holds each member name once, and one name arrived twice",
        ),
        (
            "decide",
            r#"{"decide":{"a":1e999}}"#,
            "the question file is not JSON this tool reads: a JSON number is finite, so `NaN` and `Infinity` are refused",
        ),
    ];
    for (index, (verb, file, wanted)) in cases.into_iter().enumerate() {
        let written = written(&format!("structured-refused-{index}"), file);
        let (stderr, code) = refused(&[verb, &written, "--dry-run"]);
        assert_eq!(stderr, format!("thinkthen: {wanted}\n"), "{file}");
        assert_eq!(code, Some(5), "{file}");
    }
}

#[test]
fn a_typed_boundary_replaces_the_files_structured_criterion() {
    let file = written(
        "structured-boundary",
        r#"{"decide":"Does this ask for a refund?","true":{"means":"Money back."}}"#,
    );
    let plan = printed(&["decide", &file, "--dry-run", "--true", "Money back."]);
    assert!(
        plan.contains(r#""criteria":{"true":"Money back."}"#),
        "{plan}"
    );
    assert!(plan.contains(r#""true":"command line""#), "{plan}");
}
