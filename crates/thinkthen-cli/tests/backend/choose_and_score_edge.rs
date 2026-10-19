//! The two new verbs at the binary's own edge: usage, the plan, and the help.

use crate::harness::{exits_because, stderr_is};
use std::io;
use std::process::Output;

/// The teams a routing question picks between.
const TEAMS: [&str; 4] = ["billing", "shipping", "account", "other"];

/// The levels a placement question uses, lowest first.
const LEVELS: [&str; 3] = ["none", "workaround", "blocked"];

/// Run the binary with no environment at all, and feed it one ticket.
fn run(arguments: &[&str]) -> io::Result<Output> {
    crate::harness::spawn(arguments, &[], b"The renewal charge bounced last night.")
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
    let output = verb("choose", &TEAMS, &["--plan"]).expect("the compiled binary runs");

    assert_eq!(
        plan_request(&output),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"Each question quotes the text it asks about.","#,
            r#""model":"jev-1.13.0","questions":{"q1":{"type":"choice","#,
            r#""instructions":"The text is \"The renewal charge bounced last night.\". Which team owns this request?","#,
            r#""criteria":{"billing":null,"shipping":null,"account":null,"other":null}}}}}"#,
            "\n",
        )
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_placement_plans_the_levels_as_an_ordered_list() {
    let output = verb("score", &LEVELS, &["--plan"]).expect("the compiled binary runs");

    assert_eq!(
        plan_request(&output),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"Each question quotes the text it asks about.","#,
            r#""model":"jev-1.13.0","questions":{"q1":{"type":"score","#,
            r#""instructions":"The text is \"The renewal charge bounced last night.\". Which team owns this request?","#,
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
        let output = verb(name, &labels, &["--plan"]).expect("the compiled binary runs");

        exits_because(&output, 0, || format!("{name} with {}", labels.len()));
    }
}

#[test]
fn a_band_on_a_pick_is_a_usage_error_and_a_cut_is_not() {
    let refused = verb("choose", &TEAMS, &["--plan", "--threshold", "0.1:0.9"])
        .expect("the compiled binary runs");
    assert_eq!(refused.status.code(), Some(2));
    let message = crate::harness::said(&refused);
    assert!(message.contains("single cut"), "{message}");

    let taken = verb("choose", &TEAMS, &["--plan", "--threshold", "0.8"])
        .expect("the compiled binary runs");
    assert_eq!(taken.status.code(), Some(0));
}

#[test]
fn a_rule_on_a_placement_is_a_usage_error_because_score_takes_none() {
    for (arguments, expected) in [
        (
            &["--quiet"][..],
            "thinkthen: `score` has no answer exit code, so --quiet would discard its result\n",
        ),
        (
            &["--raw"][..],
            "thinkthen: `score` prints a JSON number; `choose --raw` prints a bare label\n",
        ),
    ] {
        let output = verb("score", &LEVELS, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        stderr_is(&output, expected);
    }

    // `--threshold` is taken and then refused by the tool, because clap
    // answered it with a tip about `--record` and a usage line that read as
    // if `--record` were required.
    for arguments in [&["--threshold", "0.8"][..], &["--threshold", "0.1:0.9"][..]] {
        let output = verb("score", &LEVELS, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        let message = crate::harness::said(&output);
        assert_eq!(
            message,
            "thinkthen: --threshold: `score` takes no rule, so cut on the number with `jq -e`\n"
        );
    }
}

#[test]
fn every_option_that_is_not_built_yet_is_a_usage_error() {
    // The record options arrived with ticket 0012 and `--options` with ticket
    // 0013. `--top` and `--none` belong to verbs no ticket has built, so the
    // parser still refuses them by name.
    let cases: [&[&str]; 2] = [&["--top", "3"], &["--none"]];

    for arguments in cases {
        let output = verb("choose", &TEAMS, arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        let message = crate::harness::said(&output);
        assert!(message.contains("unexpected argument"), "{message}");
    }
}

/// The request line of a plan; the records line after it is pinned in `spec/decide.md`.
fn plan_request(output: &Output) -> String {
    let stdout = crate::harness::printed(output);
    stdout
        .split_inclusive('\n')
        .next()
        .unwrap_or_default()
        .to_owned()
}
