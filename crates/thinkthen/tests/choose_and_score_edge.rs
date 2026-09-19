//! The two new verbs at the binary's own edge: usage, the plan, and the help.

use std::io::{self, Write};
use std::process::{Command, Output, Stdio};

/// The teams a routing question picks between.
const TEAMS: [&str; 4] = ["billing", "shipping", "account", "other"];

/// The levels a placement question uses, lowest first.
const LEVELS: [&str; 3] = ["none", "workaround", "blocked"];

/// Run the binary with no environment at all, and feed it one ticket.
fn run(arguments: &[&str]) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(b"The renewal charge bounced last night.");
    drop(input);
    child.wait_with_output()
}

/// Run one verb over the labels a case names, plus whatever else it names.
fn verb(name: &str, labels: &[&str], arguments: &[&str]) -> io::Result<Output> {
    run(&[&[name, "Which team owns this request?"], labels, arguments].concat())
}

/// A list of the given length, so a case can sit on each edge of the range.
fn many(count: usize) -> Vec<String> {
    (0..count).map(|place| format!("option{place}")).collect()
}

/// The same list as borrowed arguments.
fn listed(values: &[String]) -> Vec<&str> {
    values.iter().map(String::as_str).collect()
}

#[test]
fn a_pick_plans_the_options_as_criteria_and_names_the_key_variable() {
    let output = verb("choose", &TEAMS, &["--dry-run"]).expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-latest","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"The renewal charge bounced last night.","#,
            r#""model":"jev-latest","questions":{"q1":{"type":"choice","#,
            r#""instructions":"Which team owns this request?","#,
            r#""criteria":{"billing":null,"shipping":null,"account":null,"other":null}}}}}"#,
            "\n",
        )
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_placement_plans_the_levels_as_an_ordered_list() {
    let output = verb("score", &LEVELS, &["--dry-run"]).expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-latest","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"The renewal charge bounced last night.","#,
            r#""model":"jev-latest","questions":{"q1":{"type":"score","#,
            r#""instructions":"Which team owns this request?","#,
            r#""criteria":["none","workaround","blocked"]}}}}"#,
            "\n",
        )
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_list_at_each_edge_of_the_range_is_taken() {
    let full_options = many(255);
    let full_levels = many(10);
    let cases: [(&str, Vec<&str>); 4] = [
        ("choose", vec!["billing", "other"]),
        ("choose", listed(&full_options)),
        ("score", vec!["none", "blocked"]),
        ("score", listed(&full_levels)),
    ];

    for (name, labels) in cases {
        let output = verb(name, &labels, &["--dry-run"]).expect("the compiled binary runs");

        assert_eq!(
            output.status.code(),
            Some(0),
            "{name} with {}",
            labels.len()
        );
    }
}

#[test]
fn a_band_on_a_pick_is_a_usage_error_and_a_cut_is_not() {
    let refused = verb("choose", &TEAMS, &["--dry-run", "--threshold", "0.1:0.9"])
        .expect("the compiled binary runs");
    assert_eq!(refused.status.code(), Some(2));
    let message = String::from_utf8_lossy(&refused.stderr);
    assert!(message.contains("single cut"), "{message}");

    let taken = verb("choose", &TEAMS, &["--dry-run", "--threshold", "0.8"])
        .expect("the compiled binary runs");
    assert_eq!(taken.status.code(), Some(0));
}

#[test]
fn a_rule_on_a_placement_is_a_usage_error_because_score_takes_none() {
    for arguments in [&["--quiet"][..], &["--raw"][..]] {
        let output = verb("score", &LEVELS, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("unexpected argument"), "{message}");
    }

    // `--threshold` is taken and then refused by the tool, because clap
    // answered it with a tip about `--record` and a usage line that read as
    // if `--record` were required.
    for arguments in [&["--threshold", "0.8"][..], &["--threshold", "0.1:0.9"][..]] {
        let output = verb("score", &LEVELS, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            message,
            "thinkthen: --threshold: `score` takes no rule, so cut on the number with `jq -e`\n"
        );
    }
}

#[test]
fn two_views_of_one_answer_are_a_usage_error() {
    let cases: [&[&str]; 3] = [
        &["--quiet", "--details"],
        &["--raw", "--details"],
        &["--raw", "--quiet"],
    ];

    for arguments in cases {
        let output = verb("choose", &TEAMS, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        assert!(!output.stderr.is_empty(), "{arguments:?}");
    }
}

#[test]
fn every_option_out_of_scope_for_one_document_is_a_usage_error() {
    let cases: [&[&str]; 5] = [
        &["--lines"],
        &["--jsonl"],
        &["--field", "/body"],
        &["--options", "/codes"],
        &["--input", "ticket.txt"],
    ];

    for arguments in cases {
        let output = verb("choose", &TEAMS, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("unexpected argument"), "{message}");
    }
}

#[test]
fn the_help_of_each_verb_carries_the_advice_its_page_names() {
    let long = |name: &str| {
        let output = run(&[name, "--help"]).expect("the compiled binary runs");
        String::from_utf8_lossy(&output.stdout).into_owned()
    };

    let choose = long("choose");
    for said in [
        "not_stated",
        "contradicted",
        "supported",
        "case $rc",
        "Exit 0 is a label and exit 3 is unresolved.",
        "never exits 1",
    ] {
        assert!(choose.contains(said), "{said} is missing from {choose}");
    }

    let score = long("score");
    for said in ["jq -e", "18%", "46%", "choose", "--details"] {
        assert!(score.contains(said), "{said} is missing from {score}");
    }

    let short = |name: &str| {
        let output = run(&[name, "-h"]).expect("the compiled binary runs");
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    for name in ["choose", "score"] {
        let short = short(name);
        for hidden in ["--url", "--model", "--record", "--timeout"] {
            assert!(
                !short.contains(hidden),
                "{hidden} is in the short {name} help"
            );
        }
    }
    assert!(short("choose").contains("--raw"));
    assert!(!short("score").contains("--threshold"));
}

#[test]
fn a_question_that_is_blank_and_evidence_that_is_blank_are_both_refused() {
    let blank =
        run(&["choose", "  ", "billing", "other", "--dry-run"]).expect("the compiled binary runs");
    assert_eq!(blank.status.code(), Some(2));

    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    let empty = command
        .env_clear()
        .args([
            "score",
            "How much disruption?",
            "none",
            "blocked",
            "--dry-run",
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("the compiled binary runs");
    assert_eq!(empty.status.code(), Some(2));
}
