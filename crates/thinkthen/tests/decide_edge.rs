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
    let output = decide(&["--plan", "--url", CLOSED, "--model", "local-1"])
        .expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"url":"http://127.0.0.1:1/v1/systemone","model":"local-1","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"Refund me please.","model":"local-1","#,
            r#""questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}"#,
            "\n",
            r#"{"records":1,"requests":1,"estimated_bytes":117,"estimated_input_tokens":{"lower":60,"upper":107},"upper_bound":false}"#,
            "\n",
        )
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

/// The plan document, pinned field by field in the order it prints them.
#[test]
fn the_plan_holds_four_fields_and_names_the_key_variable_without_reading_it() {
    let output = decide(&["--plan"]).expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"Refund me please.","model":"jev-1.13.0","#,
            r#""questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}"#,
            "\n",
            r#"{"records":1,"requests":1,"estimated_bytes":120,"estimated_input_tokens":{"lower":61,"upper":109},"upper_bound":false}"#,
            "\n",
        )
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_five_backend_environment_variables_are_gone_and_change_no_plan() {
    let output = run(
        &["decide", "asks for a refund", "--plan"],
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
    let output = decide(&["--plan", "--model", "jev-1.13.0"]).expect("the compiled binary runs");

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
        &["--dry-run"],
        &["--backend", "jev"],
        &["--threshold", "0.9", "--status"],
        &["--plan", "--min-prob", "0.9"],
    ];

    for arguments in cases {
        let output = decide(arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
    }

    // The old grammar named the verb after the command, and it is gone too.
    let output = run(
        &["decide", "if", "asks for a refund", "--plan"],
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
    let plan = decide(&["--plan"]).expect("the compiled binary runs");
    let plan = String::from_utf8_lossy(&plan.stdout).into_owned();

    for view in ["--quiet", "--details"] {
        let output = decide(&["--plan", view]).expect("the compiled binary runs");

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
        &["decide", "--plan", "--", "--asks for a refund"],
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
        let output = run(&["decide", "asks for a refund", "--plan"], &[], evidence)
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
    let output = run(&["decide", "  ", "--plan"], &[], b"Refund me please.")
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

#[path = "decide_edge/help.rs"]
mod help;

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

#[test]
fn estimated_input_zero_refuses_the_compiled_command_before_transport() {
    let output = run(
        &[
            "decide",
            "asks for a refund",
            "--url",
            CLOSED,
            "--no-cache",
            "--max-estimated-input-tokens-total",
            "0",
        ],
        &[("THINKTHEN_API_KEY", "sk-loopback-test")],
        b"Refund me please.",
    )
    .expect("compiled command");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen usage: max_estimated_input_tokens_total=0 (encoded-body-bytes-908-v1) would be exceeded before this call's first request\n"
    );
}
