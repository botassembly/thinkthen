//! Negative threshold routing before Clap, over every command home.

use crate::harness::{Canned, Listener, spawn};

const QUESTION: &str = "Does this report a payment failure?";
const KEY: &str = "private-key-marker";
const EVIDENCE: &str = "private-evidence-marker";

fn homes() -> [(&'static [&'static str], &'static str); 7] {
    [
        (&["decide", QUESTION], "decide"),
        (&["choose", QUESTION, "late", "lost"], "choose"),
        (&["tag", QUESTION, "late"], "tag"),
        (
            &["filter", QUESTION, "--jsonl", "--field", "/body"],
            "filter",
        ),
        (&["rank", QUESTION, "--jsonl", "--field", "/body"], "rank"),
        (&["score", QUESTION, "none", "some"], "score"),
        (&["annotate", "/no/question-set"], "annotate"),
    ]
}

fn expected(name: &str, value: &str) -> &'static str {
    match name {
        "rank" => concat!(
            "thinkthen: --threshold: `rank` orders and never selects, ",
            "so put a cut in `filter --threshold`\n"
        ),
        "score" => concat!(
            "thinkthen: --threshold: `score` takes no rule, ",
            "so cut on the number with `jq -e`\n"
        ),
        "annotate" => concat!(
            "thinkthen: --threshold belongs to each question in the question set; ",
            "`annotate` takes no command-level threshold\n"
        ),
        _ if value == "-inf" => "thinkthen: --threshold: a threshold is a finite number\n",
        _ => "thinkthen: --threshold: a single cut is above zero and at most one\n",
    }
}

#[test]
fn negative_float_spellings_match_the_equals_form_in_all_seven_homes() {
    for (prefix, name) in homes() {
        for value in ["-0.5", "-.5", "-1e-1", "-inf"] {
            let listener = Listener::answering(|_| Canned::ok("{}")).expect("a listener");
            let common = ["--url", listener.base()];
            let spaced = [prefix, &["--threshold", value], &common].concat();
            let joined_value = format!("--threshold={value}");
            let joined = [prefix, &[joined_value.as_str()], &common].concat();
            let spaced_output = spawn(&spaced, &[("THINKTHEN_API_KEY", KEY)], EVIDENCE.as_bytes())
                .expect("the compiled binary runs");
            let joined_output = spawn(&joined, &[("THINKTHEN_API_KEY", KEY)], EVIDENCE.as_bytes())
                .expect("the compiled binary runs");

            assert_eq!(spaced_output.status.code(), Some(2), "{name} {value}");
            assert!(spaced_output.stdout.is_empty(), "{name} {value}");
            assert_eq!(spaced_output.stderr, joined_output.stderr, "{name} {value}");
            assert_eq!(spaced_output.stderr, expected(name, value).as_bytes());
            let said = String::from_utf8_lossy(&spaced_output.stderr);
            assert!(!said.contains(KEY), "{name} {value}");
            assert!(!said.contains(EVIDENCE), "{name} {value}");
            assert!(listener.requests().is_empty(), "{name} {value}");
            assert_eq!(listener.connections(), 0, "{name} {value}");
        }
    }
}

#[test]
fn a_threshold_does_not_consume_a_following_known_option() {
    for (prefix, _) in homes() {
        let arguments = [prefix, &["--threshold", "--dry-run"][..]].concat();
        let output = spawn(&arguments, &[], EVIDENCE.as_bytes()).expect("the binary runs");
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("a value is required for"), "{message}");
        assert!(!message.contains("a threshold is"), "{message}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
    }
}

#[test]
fn non_numeric_option_tokens_keep_claps_existing_meaning() {
    for value in ["--bogus", "-word", "-.config", "-infamous", "-nanosecond"] {
        let output = spawn(
            &["decide", QUESTION, "--threshold", value],
            &[],
            EVIDENCE.as_bytes(),
        )
        .expect("the compiled binary runs");
        assert_eq!(output.status.code(), Some(2), "{value}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.starts_with("error:"), "{value}: {message}");
        assert!(!message.contains("a threshold is"), "{value}: {message}");
    }

    let output = spawn(
        &["decide", QUESTION, "--", "--threshold", "-.5"],
        &[],
        EVIDENCE.as_bytes(),
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let message = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        message,
        "thinkthen: the question is one argument and each option takes one value; quote a question of several words, and send evidence on standard input or as `--input FILE`\n"
    );
    assert!(!message.contains("a threshold is"), "{message}");
}
