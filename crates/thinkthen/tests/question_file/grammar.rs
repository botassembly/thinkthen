//! Every refusal the question-file grammar and the command line make.

use crate::harness::{REFUND, refused, written};

/// Each refusal the question-file grammar makes: a name, a verb, a file, a code.
pub(crate) const REFUSALS: [(&str, &str, &str, i32); 21] = [
    (
        "two-verbs",
        "decide",
        r#"{"decide":"a","choose":"b","options":["x","y"]}"#,
        5,
    ),
    ("no-verb", "decide", r#"{"text":"a"}"#, 5),
    ("unknown-key", "decide", r#"{"decide":"a","nope":1}"#, 5),
    ("version-key", "decide", r#"{"version":1,"decide":"a"}"#, 5),
    (
        "key-not-for-verb",
        "choose",
        r#"{"choose":"a","options":["x","y"],"true":"yes"}"#,
        5,
    ),
    (
        "band-on-choose",
        "choose",
        r#"{"choose":"a","options":["x","y"],"threshold":"0.2:0.8"}"#,
        5,
    ),
    (
        "cut-of-zero",
        "decide",
        r#"{"decide":"a","threshold":0}"#,
        5,
    ),
    (
        "cut-above-one",
        "decide",
        r#"{"decide":"a","threshold":1.5}"#,
        5,
    ),
    (
        "low-above-high",
        "decide",
        r#"{"decide":"a","threshold":"0.9:0.2"}"#,
        5,
    ),
    (
        "one-option",
        "choose",
        r#"{"choose":"a","options":["x"]}"#,
        5,
    ),
    (
        "repeated-option",
        "choose",
        r#"{"choose":"a","options":["x","x"]}"#,
        5,
    ),
    (
        "blank-option",
        "choose",
        r#"{"choose":"a","options":["x","  "]}"#,
        5,
    ),
    (
        "option-that-is-not-text",
        "choose",
        r#"{"choose":"a","options":["x",3]}"#,
        5,
    ),
    (
        "control-character",
        "choose",
        r#"{"choose":"a","options":["x","y\u0007"]}"#,
        5,
    ),
    ("one-level", "score", r#"{"score":"a","levels":["x"]}"#, 5),
    (
        "eleven-levels",
        "score",
        r#"{"score":"a","levels":["1","2","3","4","5","6","7","8","9","10","11"]}"#,
        5,
    ),
    ("not-json", "decide", "nope", 5),
    ("not-an-object", "decide", r#"["a"]"#, 5),
    ("wrong-shape", "decide", r#"{"decide":"a","true":3}"#, 5),
    ("blank-question", "decide", r#"{"decide":"   "}"#, 5),
    ("bad-pointer", "decide", r#"{"decide":"a","on":"body"}"#, 5),
];
/// The sentence each refusal above prints, in the same order.
const SAID: [&str; 21] = [
    "a question file holds one question, and this one holds `decide` and `choose`",
    "a question file holds one of `decide`, `choose`, `tag`, or `score`",
    "a question file takes no key `nope`",
    "a question file takes no key `version`; `version` belongs in a question set, a recognize file, or a relate file",
    "a `choose` question file takes no key `true`",
    "the question file's `threshold`: `choose` takes a single cut and never a band",
    "the question file's `threshold`: a single cut is above zero and at most one",
    "the question file's `threshold`: a single cut is above zero and at most one",
    "the question file's `threshold`: a band's low side is below its high side",
    "the question file's `options`: `choose` takes 2 to 255 options",
    "the question file's `options`: a list holds each option once",
    "the question file's `options`: an option is text, not white space",
    "`options` in the question file is a list of labels, or a map from each label to its description",
    "the question file's `options`: an option is one line of printable text",
    "the question file's `levels`: `score` takes 2 to 10 levels, lowest first",
    "the question file's `levels`: `score` takes 2 to 10 levels, lowest first",
    "the question file is not valid JSON: the JSON at line 1 column 2 is not one",
    "a question file is one JSON object",
    "`true` in the question file is text, an object, a list, or null",
    "the question file's `decide`: a question is text, not white space",
    "the question file's `on` `body`: a pointer is RFC 6901, so it is empty or begins with `/`",
];

#[test]
fn every_refusal_the_grammar_makes_names_its_key_and_its_exit_code() {
    for ((name, verb, text, code), sentence) in REFUSALS.into_iter().zip(SAID) {
        let file = written(name, text);
        let (message, status) = refused(&[verb, &file, "--dry-run"]);
        assert_eq!(message, format!("thinkthen: {sentence}\n"), "{name}");
        assert_eq!(status, Some(code), "{name}");
    }
}

#[test]
fn an_unknown_local_key_is_escaped_and_stays_on_one_line() {
    let file = written(
        "unknown-control-key",
        r#"{"decide":"Does this pass?","line\nbreak":true}"#,
    );
    assert_eq!(
        refused(&["decide", &file, "--dry-run"]),
        (
            r#"thinkthen: a question file takes no key `line\nbreak`
"#
            .to_owned(),
            Some(5)
        )
    );
}

#[test]
fn a_file_with_too_many_options_is_refused_by_the_same_sentence() {
    let names: Vec<String> = (0..256).map(|number| format!(r#""o{number}""#)).collect();
    let file = written(
        "too-many",
        &format!(r#"{{"choose":"a","options":[{}]}}"#, names.join(",")),
    );
    let (message, status) = refused(&["choose", &file, "--dry-run"]);
    assert_eq!(
        message,
        "thinkthen: the question file's `options`: `choose` takes 2 to 255 options\n"
    );
    assert_eq!(status, Some(5));
}

#[test]
fn the_command_must_name_the_verb_the_file_holds() {
    let file = written("mismatch", REFUND);
    let (message, status) = refused(&["choose", &file, "billing", "other", "--dry-run"]);
    assert_eq!(
        message,
        "thinkthen: the command is `choose` and the question file holds a `decide` question\n"
    );
    assert_eq!(status, Some(2));
}

#[test]
fn a_score_file_refuses_a_rule_and_a_level_that_holds_a_control_character() {
    let rule = written(
        "score-rule",
        r#"{"score":"a","levels":["x","y"],"threshold":0.5}"#,
    );
    assert_eq!(
        refused(&["score", &rule, "--dry-run"]),
        (
            "thinkthen: a `score` question file takes no key `threshold`\n".to_owned(),
            Some(5)
        )
    );

    let typed = refused(&["score", "how much", "low", "high", "--threshold", "0.5"]);
    assert_eq!(
        typed,
        (
            concat!(
                "thinkthen: --threshold: `score` takes no rule, ",
                "so cut on the number with `jq -e`\n",
            )
            .to_owned(),
            Some(2)
        )
    );

    let level = written(
        "score-control",
        &format!(r#"{{"score":"a","levels":["x","y{}u0007"]}}"#, '\\'),
    );
    assert_eq!(
        refused(&["score", &level, "--dry-run"]),
        (
            concat!(
                "thinkthen: the question file's `levels`: ",
                "a level is one line of printable text\n",
            )
            .to_owned(),
            Some(5)
        )
    );
}

#[test]
fn a_file_that_cannot_be_opened_is_a_local_failure() {
    let (message, status) = refused(&["decide", "@no-such-question-file.json", "--dry-run"]);
    assert!(
        message.starts_with("thinkthen: the question file could not be opened:"),
        "{message}"
    );
    assert_eq!(status, Some(5));
}

#[test]
fn a_list_of_options_and_a_described_option_have_no_order_between_them() {
    let (message, status) = refused(&[
        "choose",
        "Which team?",
        "billing",
        "other",
        "--option",
        "sales=New business.",
        "--dry-run",
    ]);
    assert_eq!(
        message,
        concat!(
            "thinkthen: --option and a list of options have no order between them, ",
            "so one run takes one\n",
        )
    );
    assert_eq!(status, Some(2));
}

#[test]
fn an_option_without_an_equals_sign_is_a_usage_error() {
    let (message, status) = refused(&["choose", "Which team?", "--option", "sales", "--dry-run"]);
    assert_eq!(
        message,
        "thinkthen: --option is LABEL=DESCRIPTION, and this one holds no `=`\n"
    );
    assert_eq!(status, Some(2));
}

#[test]
fn a_verb_refuses_the_option_another_verb_takes() {
    let (message, status) = refused(&[
        "choose",
        "Which team?",
        "billing",
        "other",
        "--true",
        "Money back.",
    ]);
    assert_eq!(
        message,
        concat!(
            "error: unexpected argument '--true' found\n\n",
            "  tip: to pass '--true' as a value, use '-- --true'\n\n",
            "Usage: thinkthen choose <QUESTION> <OPTIONS>...\n\n",
            "For more information, try '--help'.\n",
        )
    );
    assert_eq!(status, Some(2));
}

#[test]
fn a_band_typed_on_a_pick_is_refused_by_the_option_that_carried_it() {
    let (message, status) = refused(&[
        "choose",
        "Which team?",
        "billing",
        "other",
        "--threshold",
        "0.2:0.8",
    ]);
    assert_eq!(
        message,
        "thinkthen: --threshold: `choose` takes a single cut and never a band\n"
    );
    assert_eq!(status, Some(2));
}

#[test]
fn a_verb_with_no_list_in_either_home_names_neither_home() {
    for (name, verb, text, sentence) in [
        (
            "no-options",
            "choose",
            r#"{"choose":"Which team?"}"#,
            "`choose` takes 2 to 255 options",
        ),
        (
            "no-levels",
            "score",
            r#"{"score":"How much?"}"#,
            "`score` takes 2 to 10 levels, lowest first",
        ),
    ] {
        let file = written(name, text);
        let (message, status) = refused(&[verb, &file, "--dry-run"]);
        assert_eq!(message, format!("thinkthen: {sentence}\n"), "{name}");
        assert_eq!(status, Some(2), "{name}");
    }
}
