//! A file holding the whole question, and every typed setting that replaces one.

use super::harness::{REFUND, printed, printed_over, written};

#[test]
fn a_file_holds_the_whole_question_and_the_plan_says_so_setting_by_setting() {
    let file = written("refund", REFUND);

    assert_eq!(
        printed(&["decide", &file, "--plan"]),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""from":{"question":"file","true":"file","false":"file","#,
            r#""threshold":"file","on":"default","model":"file"},"#,
            r#""request":{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","#,
            r#""questions":{"q1":{"type":"noul","#,
            r#""instructions":"The text is \"Refund me please.\". Does this message ask for a refund?","#,
            r#""criteria":{"true":"The writer asks for money back.","#,
            r#""false":"The writer asks for anything else."}}}}}"#,
            "\n",
            r#"{"records":1,"requests":1,"estimated_bytes":299,"largest_request_bytes":299,"largest_request_estimated_input_tokens":272,"token_estimate_method":"encoded-body-bytes-908-v1","estimated_input_tokens":{"lower":154,"upper":272},"upper_bound":false}"#,
            "\n",
        )
    );
}

#[test]
fn a_question_typed_with_no_file_names_no_source_at_all() {
    let plan = printed(&["decide", "Does this message ask for a refund?", "--plan"]);
    assert!(!plan.contains("\"from\""), "{plan}");
}

#[test]
fn each_typed_setting_replaces_the_files_and_the_plan_names_the_command_line() {
    let file = written("overrides", REFUND);
    let cases: [(&[&str], &str); 4] = [
        (&["--threshold", "0.9"], r#""threshold":"command line""#),
        (&["--true", "Money back."], r#""true":"command line""#),
        (&["--false", "Anything else."], r#""false":"command line""#),
        (&["--model", "local-1"], r#""model":"command line""#),
    ];
    for (typed, named) in cases {
        let plan = printed(&[&["decide", &file, "--plan"], typed].concat());
        assert!(plan.contains(named), "{typed:?} gives {plan}");
    }
    let over = printed_over(
        &["decide", &file, "--plan", "--field", "/body"],
        br#"{"body":"Refund me please."}"#,
    );
    assert!(over.contains(r#""on":"command line""#), "{over}");
    assert!(
        over.contains(r#""instructions":"The text is \"Refund me please.\". Does this message ask for a refund?""#),
        "{over}"
    );

    let plan = printed(&["decide", &file, "--plan", "--true", "Money back."]);
    assert!(plan.contains(r#""true":"Money back.""#), "{plan}");
    assert!(plan.contains(r#""false":"The writer asks for anything else.""#));
}

#[test]
fn a_typed_list_replaces_the_files_whole_list_and_never_merges_with_it() {
    let file = written(
        "teams",
        r#"{"choose":"Which team owns this?","options":["billing","shipping","other"]}"#,
    );
    let plan = printed(&["choose", &file, "sales", "support", "--plan"]);
    assert!(
        plan.contains(r#""criteria":{"sales":null,"support":null}"#),
        "{plan}"
    );
    assert!(!plan.contains("billing"), "{plan}");
    assert!(plan.contains(r#""options":"command line""#), "{plan}");
}

#[test]
fn a_description_typed_beside_a_file_replaces_the_whole_map() {
    let file = written(
        "described",
        r#"{"choose":"Which team owns this?","options":{"billing":"Money.","other":null}}"#,
    );
    let plan = printed(&[
        "choose",
        &file,
        "--option",
        "sales=New business.",
        "--option",
        "support=Everything after the sale.",
        "--plan",
    ]);
    assert!(
        plan.contains(
            r#""criteria":{"sales":"New business.","support":"Everything after the sale."}"#
        ),
        "{plan}"
    );
}

#[test]
fn a_levels_list_typed_beside_a_file_replaces_the_files_levels() {
    let file = written(
        "levels",
        r#"{"score":"How much disruption?","levels":["None.","Some.","Blocked."]}"#,
    );
    let plan = printed(&["score", &file, "Low.", "High.", "--plan"]);
    assert!(plan.contains(r#""criteria":["Low.","High."]"#), "{plan}");
    assert!(plan.contains(r#""levels":"command line""#), "{plan}");
}
