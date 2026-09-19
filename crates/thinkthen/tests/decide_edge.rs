//! The compiled binary at its own edge: usage, the plan, and standard input.

use std::io::{self, Write};
use std::process::{Command, Output, Stdio};

/// A port nothing listens on, so a connection would be refused at once.
const CLOSED: &str = "http://127.0.0.1:1/v1";

/// Run the binary with no environment but what the case names, and feed it bytes.
fn run(arguments: &[&str], environment: &[(&str, &str)], evidence: &[u8]) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
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
    child.wait_with_output()
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
            r#"{"url":"https://api.typesafe.ai/v1/systemone","model":"jev-latest","#,
            r#""key_env":"THINKTHEN_API_KEY","#,
            r#""request":{"state":"Refund me please.","model":"jev-latest","#,
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
    assert!(printed.contains(r#""model":"jev-latest""#), "{printed}");
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
fn every_option_the_configuration_surface_carried_is_a_usage_error() {
    let cases: [&[&str]; 8] = [
        &["--profile", "jev"],
        &["--profile", "site"],
        &["--adapter", "systemone"],
        &["--adapter", "chat-logprobs"],
        &["--key-env", "LOCAL_KEY"],
        &["--config", "site.json"],
        &["--url", CLOSED, "--adapter", "systemone", "--model", "m"],
        &["--dry-run", "--profile", "jev"],
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

#[test]
fn no_diagnostic_ever_carries_the_key_or_the_evidence() {
    let secret = "sk-never-printed";
    let evidence = "The customer wrote something private.";
    let output = run(
        &["decide", "asks for a refund", "--url", "ftp://127.0.0.1/v1"],
        &[("THINKTHEN_API_KEY", secret)],
        evidence.as_bytes(),
    )
    .expect("the compiled binary runs");

    let message = String::from_utf8_lossy(&output.stderr);
    assert!(!message.contains(secret), "{message}");
    assert!(!message.contains(evidence), "{message}");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn the_short_help_shows_the_everyday_options_and_the_long_help_adds_the_rest() {
    let short = run(&["decide", "-h"], &[], b"").expect("the compiled binary runs");
    let long = run(&["decide", "--help"], &[], b"").expect("the compiled binary runs");

    let short = String::from_utf8_lossy(&short.stdout);
    for option in ["--threshold", "--quiet", "--details", "--dry-run"] {
        assert!(short.contains(option), "{option} is missing from {short}");
    }
    for option in ["--url", "--model", "--record", "--timeout"] {
        assert!(!short.contains(option), "{option} is in the short help");
    }

    let long = String::from_utf8_lossy(&long.stdout);
    for option in [
        "--threshold",
        "--quiet",
        "--details",
        "--dry-run",
        "--url",
        "--model",
        "--record",
        "--replay",
        "--timeout",
        "--max-retries",
    ] {
        assert!(long.contains(option), "{option} is missing from {long}");
    }
    for gone in ["--profile", "--adapter", "--key-env", "--config"] {
        assert!(!long.contains(gone), "{gone} is still in {long}");
    }
    assert!(!long.contains("THINKTHEN_TEST_RETRY_WAIT_MS"), "{long}");
    assert!(long.contains("set -e"), "the help warns about set -e");
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
