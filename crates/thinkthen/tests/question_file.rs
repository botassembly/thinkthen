//! A question read from a file, and every refusal the two homes make.

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

/// Write one question file under the test target directory and name its path.
///
/// Each case owns its file, so a case that rewrites one cannot reach another.
fn written(name: &str, text: &str) -> String {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("question-file");
    let _ = fs::create_dir_all(&folder);
    let path = folder.join(format!("{name}.json"));
    let _ = fs::write(&path, text);
    format!("@{}", path.display())
}

/// Run the binary with no environment over the evidence the case names.
fn run(arguments: &[&str], evidence: &[u8]) -> io::Result<Output> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_thinkthen"))
        .env_clear()
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence);
    drop(input);
    child.wait_with_output()
}

/// What a case reads when the binary never ran, so the failure says which.
const UNRUN: &str = "the compiled binary did not run";

/// Run one command over one line of evidence and read back its standard output.
fn printed(arguments: &[&str]) -> String {
    printed_over(arguments, b"Refund me please.")
}

/// Run one command over the evidence it names, and read back its output.
fn printed_over(arguments: &[&str], evidence: &[u8]) -> String {
    let Ok(output) = run(arguments, evidence) else {
        return UNRUN.to_owned();
    };
    assert_eq!(
        output.status.code(),
        Some(0),
        "{arguments:?} {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Run one command that is refused, and read back its message and its code.
fn refused(arguments: &[&str]) -> (String, Option<i32>) {
    let Ok(output) = run(arguments, b"Refund me please.") else {
        return (UNRUN.to_owned(), None);
    };
    assert!(output.stdout.is_empty(), "{arguments:?}");
    (
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.code(),
    )
}

/// The file every override case starts from.
const REFUND: &str = concat!(
    r#"{"decide":"Does this message ask for a refund?","#,
    r#""true":"The writer asks for money back.","#,
    r#""false":"The writer asks for anything else.","#,
    r#""threshold":"0.2:0.8","model":"jev-1.13.0"}"#,
);

#[test]
fn a_file_holds_the_whole_question_and_the_plan_says_so_setting_by_setting() {
    let file = written("refund", REFUND);

    assert_eq!(
        printed(&["decide", &file, "--dry-run"]),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""from":{"question":"file","true":"file","false":"file","#,
            r#""threshold":"file","on":"default","model":"file"},"#,
            r#""request":{"state":"Refund me please.","model":"jev-1.13.0","#,
            r#""questions":{"q1":{"type":"noul","#,
            r#""instructions":"Does this message ask for a refund?","#,
            r#""criteria":{"true":"The writer asks for money back.","#,
            r#""false":"The writer asks for anything else."}}}}}"#,
            "\n",
        )
    );
}

#[test]
fn a_question_typed_with_no_file_names_no_source_at_all() {
    let plan = printed(&["decide", "Does this message ask for a refund?", "--dry-run"]);
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
        let plan = printed(&[&["decide", &file, "--dry-run"], typed].concat());
        assert!(plan.contains(named), "{typed:?} gives {plan}");
    }
    let over = printed_over(
        &["decide", &file, "--dry-run", "--field", "/body"],
        br#"{"body":"Refund me please."}"#,
    );
    assert!(over.contains(r#""on":"command line""#), "{over}");
    assert!(over.contains(r#""state":"Refund me please.""#), "{over}");

    let plan = printed(&["decide", &file, "--dry-run", "--true", "Money back."]);
    assert!(plan.contains(r#""true":"Money back.""#), "{plan}");
    assert!(plan.contains(r#""false":"The writer asks for anything else.""#));
}

#[test]
fn a_typed_list_replaces_the_files_whole_list_and_never_merges_with_it() {
    let file = written(
        "teams",
        r#"{"choose":"Which team owns this?","options":["billing","shipping","other"]}"#,
    );
    let plan = printed(&["choose", &file, "sales", "support", "--dry-run"]);
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
        "--dry-run",
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
    let plan = printed(&["score", &file, "Low.", "High.", "--dry-run"]);
    assert!(plan.contains(r#""criteria":["Low.","High."]"#), "{plan}");
    assert!(plan.contains(r#""levels":"command line""#), "{plan}");
}

/// Each refusal the question-file grammar makes: a name, a verb, a file, a code.
const REFUSALS: [(&str, &str, &str, i32); 20] = [
    (
        "two-verbs",
        "decide",
        r#"{"decide":"a","choose":"b","options":["x","y"]}"#,
        5,
    ),
    ("no-verb", "decide", r#"{"text":"a"}"#, 5),
    ("unknown-key", "decide", r#"{"decide":"a","nope":1}"#, 5),
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
const SAID: [&str; 20] = [
    "a question file holds one question, and this one holds `decide` and `choose`",
    "a question file holds one of `decide`, `choose`, or `score`",
    "a question file holds no key `nope`",
    "a `choose` question file takes no key `true`",
    "the question file's `threshold`: `choose` takes a single cut and never a band",
    "the question file's `threshold`: a single cut is above zero and at most one",
    "the question file's `threshold`: a single cut is above zero and at most one",
    "the question file's `threshold`: a band's low side is below its high side",
    "the question file's `options`: `choose` takes 2 to 255 options",
    "the question file's `options`: a list holds each option and each level once",
    "the question file's `options`: an option or a level is text, not white space",
    "`options` in the question file is a list of labels, or a map from each label to its description",
    "the question file's `options`: an option or a level is one line of printable text",
    "the question file's `levels`: `score` takes 2 to 10 levels, lowest first",
    "the question file's `levels`: `score` takes 2 to 10 levels, lowest first",
    "the question file is not JSON this tool reads: the record is not valid JSON",
    "a question file is one JSON object",
    "`true` in the question file is text",
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
    assert!(
        message.contains("unexpected argument '--true' found"),
        "{message}"
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

#[test]
fn neither_the_key_nor_the_evidence_reaches_any_message() {
    let file = written("secret-check", r#"{"decide":"a","threshold":0}"#);
    let (message, _) = refused(&["decide", &file, "--dry-run"]);
    assert!(!message.contains("Refund me please."), "{message}");
    assert!(!message.to_lowercase().contains("apikey_"), "{message}");
    assert!(
        !message.to_lowercase().contains("authorization"),
        "{message}"
    );
}
