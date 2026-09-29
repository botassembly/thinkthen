//! The compiled binary at its own edge: usage, the plan, and standard input.
#![cfg(feature = "cli")]

use std::io::{self, Write};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "../src/test_deadline/wait.rs"]
mod wait;

const DEFAULT_MODEL: &str = "jev-1.13.0";

/// A port nothing listens on, so a connection would be refused at once.
const CLOSED: &str = "http://127.0.0.1:1/v1";

/// Run the binary with no environment but what the case names, and feed it bytes.
fn run(arguments: &[&str], environment: &[(&str, &str)], evidence: &[u8]) -> io::Result<Output> {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let home = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "decide-edge-home-{}-{}",
        std::process::id(),
        RUNS.fetch_add(1, Ordering::Relaxed)
    ));
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .env("HOME", home)
        .args(arguments)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in environment {
        command.env(name, value);
    }
    let mut child = command.spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("no pipe to standard input"))?;
    let _ = input.write_all(evidence);
    drop(input);
    wait::finish(child, &arguments.join(" "))
}

/// Run `decide` over one line of evidence.
fn decide(arguments: &[&str]) -> io::Result<Output> {
    run(
        &[&["decide", "asks for a refund"], arguments].concat(),
        &[],
        b"Refund me please.",
    )
}

/// `--url` names a base and takes no companion option, and the key rule holds.
#[test]
fn a_url_alone_names_the_base_and_the_key_variable_does_not_change() {
    let output = decide(&["--dry-run", "--url", CLOSED, "--model", "local-1"])
        .expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"url":"http://127.0.0.1:1/v1/systemone","model":"local-1","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"Refund me please.","model":"local-1","#,
            r#""questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}"#,
            "\n",
        )
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

/// The plan document, pinned field by field in the order it prints them.
#[test]
fn the_plan_holds_four_fields_and_names_the_key_variable_without_reading_it() {
    let output = decide(&["--dry-run"]).expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"Refund me please.","model":"jev-1.13.0","#,
            r#""questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}"#,
            "\n",
        )
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_five_backend_environment_variables_are_gone_and_change_no_plan() {
    let output = run(
        &["decide", "asks for a refund", "--dry-run"],
        &[
            ("THINKTHEN_BACKEND", "nowhere"),
            ("THINKTHEN_URL", CLOSED),
            ("THINKTHEN_ADAPTER", "systemone"),
            ("THINKTHEN_MODEL", "from-the-environment"),
            ("THINKTHEN_KEY_ENV", "FROM_THE_ENVIRONMENT"),
        ],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");

    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(r#""url":"https://api.typesafe.ai/v1/systemone""#),
        "{printed}"
    );
    assert!(printed.contains(r#""model":"jev-1.13.0""#), "{printed}");
    assert!(
        printed.contains(r#""key_env":"THINKTHEN_API_KEY""#),
        "{printed}"
    );
    assert!(!printed.contains("from-the-environment"), "{printed}");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn a_model_flag_replaces_the_default_model() {
    let output = decide(&["--dry-run", "--model", "jev-1.13.0"]).expect("the compiled binary runs");

    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(printed.contains(r#""model":"jev-1.13.0""#), "{printed}");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn each_way_of_naming_no_backend_and_no_threshold_is_a_usage_error() {
    let cases: [(&[&str], &str); 7] = [
        (&["--url", ""], "a blank base"),
        (&["--url", "   "], "a base of white space"),
        (&["--url", "ftp://127.0.0.1/v1"], "a base of another scheme"),
        (&["--model", ""], "a blank model"),
        (&["--model", " "], "a model of white space"),
        (&["--threshold", "90"], "a percent"),
        (&["--threshold", "0.9:0.1"], "a reversed band"),
    ];

    for (arguments, what) in cases {
        let output = decide(arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{what}");
        assert!(output.stdout.is_empty(), "{what}");
        assert!(!output.stderr.is_empty(), "{what}");
    }
}

#[test]
fn every_option_the_old_surface_carried_is_a_usage_error() {
    let cases: [&[&str]; 6] = [
        &["--status"],
        &["--min-prob", "0.9"],
        &["--plan"],
        &["--backend", "jev"],
        &["--threshold", "0.9", "--status"],
        &["--dry-run", "--min-prob", "0.9"],
    ];

    for arguments in cases {
        let output = decide(arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
    }

    // The old grammar named the verb after the command, and it is gone too.
    let output = run(
        &["decide", "if", "asks for a refund", "--dry-run"],
        &[],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn every_remaining_option_the_configuration_surface_carried_is_a_usage_error() {
    let cases: [&[&str]; 5] = [
        &["--adapter", "systemone"],
        &["--adapter", "chat-logprobs"],
        &["--key-env", "LOCAL_KEY"],
        &["--config", "site.json"],
        &["--url", CLOSED, "--adapter", "systemone", "--model", "m"],
    ];

    for arguments in cases {
        let output = decide(arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("unexpected argument"), "{message}");
    }
}

#[test]
fn a_dry_run_prints_the_plan_whichever_view_option_stands_beside_it() {
    let plan = decide(&["--dry-run"]).expect("the compiled binary runs");
    let plan = String::from_utf8_lossy(&plan.stdout).into_owned();

    for view in ["--quiet", "--details"] {
        let output = decide(&["--dry-run", view]).expect("the compiled binary runs");

        assert_eq!(String::from_utf8_lossy(&output.stdout), plan, "{view}");
        assert_eq!(output.status.code(), Some(0), "{view}");
    }
}

#[test]
fn quiet_beside_details_is_a_usage_error() {
    let output = decide(&["--quiet", "--details"]).expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("--quiet"), "{message}");
    assert!(output.stdout.is_empty());
}

#[test]
fn an_option_may_sit_before_the_question_and_a_dash_ends_the_options() {
    let output = run(
        &["decide", "--dry-run", "--", "--asks for a refund"],
        &[],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");

    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(r#""instructions":"--asks for a refund""#),
        "{printed}"
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn evidence_that_is_blank_or_not_utf_eight_stops_the_command() {
    let cases: [(&[u8], i32); 4] = [
        (b"", 2),
        (b"   \n\t ", 2),
        (b"\xff\xfe not text", 5),
        (b"Refund me please.\xc3\x28", 5),
    ];

    for (evidence, code) in cases {
        let output = run(&["decide", "asks for a refund", "--dry-run"], &[], evidence)
            .expect("the compiled binary runs");

        assert_eq!(
            output.status.code(),
            Some(code),
            "{}",
            String::from_utf8_lossy(evidence)
        );
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn a_blank_question_is_a_usage_error() {
    let output = run(&["decide", "  ", "--dry-run"], &[], b"Refund me please.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
}

/// The key a secrecy case sets, which no channel may ever repeat.
const SECRET: &str = "sk-never-printed";

/// The evidence a secrecy case sends, which no channel may ever repeat.
const PRIVATE: &str = "The customer wrote something private.";

/// Every verb asked over the private evidence, once refused and once failing.
///
/// A bad address is refused before a request. A closed port fails after one.
/// The two together walk the usage path and the backend path of each verb.
fn secrecy_cases() -> [(&'static str, Vec<&'static str>); 6] {
    let refused = "ftp://127.0.0.1/v1";
    [
        (
            "decide",
            vec!["decide", "asks for a refund", "--url", refused],
        ),
        (
            "decide",
            vec!["decide", "asks for a refund", "--url", CLOSED],
        ),
        (
            "choose",
            vec![
                "choose",
                "which team owns this",
                "billing",
                "other",
                "--url",
                refused,
            ],
        ),
        (
            "choose",
            vec![
                "choose",
                "which team owns this",
                "billing",
                "other",
                "--url",
                CLOSED,
            ],
        ),
        (
            "score",
            vec![
                "score",
                "how much disruption",
                "none",
                "blocked",
                "--url",
                refused,
            ],
        ),
        (
            "score",
            vec![
                "score",
                "how much disruption",
                "none",
                "blocked",
                "--url",
                CLOSED,
            ],
        ),
    ]
}

#[test]
fn no_diagnostic_of_any_verb_ever_carries_the_key_or_the_evidence() {
    for (verb, arguments) in secrecy_cases() {
        let output = run(
            &arguments,
            &[
                ("THINKTHEN_API_KEY", SECRET),
                ("THINKTHEN_TEST_RETRY_WAIT_MS", "1"),
            ],
            PRIVATE.as_bytes(),
        )
        .expect("the compiled binary runs");

        for channel in [&output.stderr, &output.stdout] {
            let said = String::from_utf8_lossy(channel);
            assert!(!said.contains(SECRET), "{verb}: {said}");
            assert!(!said.contains(PRIVATE), "{verb}: {said}");
        }
        assert!(
            matches!(output.status.code(), Some(2 | 4)),
            "{verb}: {:?}",
            output.status.code()
        );
    }
}

#[test]
fn the_short_help_shows_the_everyday_options_and_the_long_help_adds_the_rest() {
    let short = run(&["decide", "-h"], &[], b"").expect("the compiled binary runs");
    let long = run(&["decide", "--help"], &[], b"").expect("the compiled binary runs");

    let short = String::from_utf8_lossy(&short.stdout);
    for option in [
        "--threshold",
        "--quiet",
        "--details",
        "--dry-run",
        "--profile",
    ] {
        assert!(short.contains(option), "{option} is missing from {short}");
    }
    assert!(short.contains("--url"), "--url is missing from {short}");
    for option in ["--model", "--record", "--timeout", "--jobs"] {
        assert!(!short.contains(option), "{option} is in the short help");
    }

    let long = String::from_utf8_lossy(&long.stdout);
    let examples = long.split("The answer is a bare").next().unwrap_or("");
    assert!(
        examples.contains("printf 'Refund me please.' | thinkthen decide")
            && examples.contains("--threshold 0.1:0.9 --details"),
        "{long}"
    );
    for option in [
        "--threshold",
        "--quiet",
        "--details",
        "--dry-run",
        "--url",
        "--profile",
        "--model",
        "--record",
        "--replay",
        "--timeout",
        "--max-retries",
    ] {
        assert!(long.contains(option), "{option} is missing from {long}");
    }
    for gone in ["--adapter", "--key-env", "--config"] {
        assert!(!long.contains(gone), "{gone} is still in {long}");
    }
    assert!(
        long.contains(&format!("[default: {DEFAULT_MODEL}]")),
        "the long help does not name the default model {DEFAULT_MODEL}: {long}"
    );
    assert!(!long.contains("THINKTHEN_TEST_RETRY_WAIT_MS"), "{long}");
    assert!(!long.contains("THINKTHEN_TEST_SIGINT_ACK"), "{long}");
    assert!(long.contains("set -e"), "the help warns about set -e");
    assert!(
        long.contains("no or not sure"),
        "the help names both nonzero answers"
    );
    assert!(
        long.contains("It defaults to 0.5"),
        "the help does not name the threshold default: {long}"
    );
    assert!(
        long.contains("[default: 4]"),
        "the help does not name the jobs default: {long}"
    );
    for cache_rule in [
        "cached by default in the platform cache folder",
        "Entries contain the judged text",
        "overriding THINKTHEN_CACHE and the platform default",
        "An explicit recording folder suppresses the platform default cache",
    ] {
        assert!(long.contains(cache_rule), "{cache_rule}: {long}");
    }
}

#[test]
fn filter_help_names_default_batching() {
    let filter = run(&["filter", "--help"], &[], b"").expect("the compiled binary runs");
    let filter = String::from_utf8_lossy(&filter.stdout);
    assert!(filter.contains("Records share requests by default; --batch 1"));
    assert!(!filter.contains("one paid request for every record"));
}

#[test]
fn the_long_help_says_a_transport_failure_is_never_sent_again() {
    for command in ["decide", "find"] {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(
            help.contains("\n          How many times a retried status is sent again. A transport failure is never sent again\n"),
            "{command}: {help}"
        );
    }
}

#[test]
fn record_capable_help_pins_run_exit_behavior() {
    const RECORD_EXIT: &str = "A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers.";
    const SHORT: &str = "Answer one yes or no question about a text. A record run exits 0 when it completes without a partial or whole-run failure. The printed values carry the individual answers\n\nUsage:";
    const WHOLE_SET: &str = "A run that answers some relation questions and fails others prints what it has and exits 6. A run whose relation questions all fail prints nothing and exits 4.";
    const TEACHING: &str = "The exit code is 0 for yes, 1 for no, 3 for not sure, and any other code when the run is broken or interrupted.";
    let long = |command: &str| {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        String::from_utf8_lossy(&output.stdout).into_owned()
    };
    let output = run(&["decide", "-h"], &[], b"").expect("the compiled binary runs");
    let short = String::from_utf8_lossy(&output.stdout);
    assert!(short.starts_with(SHORT), "decide short help: {short}");

    for command in [
        "decide",
        "filter",
        "rank",
        "choose",
        "score",
        "tag",
        "annotate",
        "recognize",
    ] {
        let help = long(command);
        assert_eq!(help.matches(RECORD_EXIT).count(), 1, "{command}: {help}");
    }
    let relate = long("relate");
    assert!(!relate.contains("A record run"), "relate: {relate}");
    assert_eq!(relate.matches(WHOLE_SET).count(), 1, "relate: {relate}");
    let decide = long("decide");
    assert_eq!(decide.matches(TEACHING).count(), 1, "decide: {decide}");
    assert!(!decide.contains("3 for unresolved"), "decide: {decide}");
}

#[test]
fn shared_help_defers_order_and_document_rules_to_each_command() {
    for command in ["decide", "filter", "rank"] {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        let help = String::from_utf8_lossy(&output.stdout);
        assert!(
            help.contains("Output follows the order the command defines."),
            "{command}: {help}"
        );
        assert!(
            help.contains("On a command that accepts a single text"),
            "{command}: {help}"
        );
    }
    let output = run(&["rank", "--help"], &[], b"").expect("the compiled binary runs");
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("most likely yes first"), "{help}");
    assert!(help.contains("An exact tie keeps input order."), "{help}");
}

#[test]
fn record_framing_help_names_each_commands_output_rule() {
    const VALUE: &str = "Value verbs keep `input` beside `value`.";
    const RECORD: &str = "Record-returning verbs return records.";
    const ANNOTATE: &str = "`annotate` enriches object records.";

    for command in ["decide", "annotate", "filter"] {
        let output = run(&[command, "--help"], &[], b"").expect("the compiled binary runs");
        let help = String::from_utf8_lossy(&output.stdout);
        for rule in [VALUE, RECORD, ANNOTATE] {
            assert!(help.contains(rule), "{command}: {rule}\n{help}");
        }
    }
}

#[test]
fn an_unknown_word_is_a_usage_error_and_never_an_instruction() {
    for arguments in [
        &["decide", "a", "b"][..],
        &["think", "about", "it"][..],
        &["decide"][..],
    ] {
        let output = run(arguments, &[], b"Refund me please.").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
    }
}

#[test]
fn decide_names_choose_as_the_home_of_raw_labels() {
    let output =
        run(&["decide", "q", "--raw"], &[], b"evidence").expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: `decide` prints JSON; `choose --raw` prints a bare label\n"
    );
}
