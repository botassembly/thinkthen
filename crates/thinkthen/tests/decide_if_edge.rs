//! The compiled binary at its own edge: usage, the plan, and standard input.

use std::io::{self, Write};
use std::process::{Command, Output, Stdio};

/// A port nothing listens on, so a connection would be refused at once.
const CLOSED: &str = "http://127.0.0.1:1/v1/systemone";

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

/// Run `decide if` over one line of evidence.
fn decide(arguments: &[&str]) -> io::Result<Output> {
    run(
        &[&["decide", "if", "asks for a refund"], arguments].concat(),
        &[],
        b"Refund me please.",
    )
}

#[test]
fn the_plan_prints_six_fields_and_opens_no_connection() {
    let output = decide(&[
        "--plan",
        "--url",
        CLOSED,
        "--adapter",
        "systemone",
        "--model",
        "local-1",
    ])
    .expect("the compiled binary runs");

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        concat!(
            r#"{"backend":null,"url":"http://127.0.0.1:1/v1/systemone","adapter":"systemone","#,
            r#""model":"local-1","key_env":null,"#,
            r#""request":{"state":"Refund me please.","model":"local-1","#,
            r#""questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}}"#,
            "\n",
        )
    );
    assert!(output.stderr.is_empty());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_plan_of_a_named_profile_names_its_key_variable_and_needs_no_key() {
    let output = decide(&["--plan"]).expect("the compiled binary runs");

    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.starts_with(concat!(
            r#"{"backend":"jev","url":"https://api.typesafe.ai/v1/systemone","#,
            r#""adapter":"systemone","model":"jev-latest","key_env":"TYPESAFE_API_KEY","#,
        )),
        "{printed}"
    );
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn environment_variables_name_a_backend_and_a_flag_beats_them() {
    let output = run(
        &["decide", "if", "asks for a refund", "--plan"],
        &[
            ("THINKTHEN_URL", CLOSED),
            ("THINKTHEN_ADAPTER", "systemone"),
            ("THINKTHEN_MODEL", "from-the-environment"),
        ],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(r#""model":"from-the-environment""#),
        "{printed}"
    );

    let output = run(
        &[
            "decide",
            "if",
            "asks for a refund",
            "--plan",
            "--model",
            "from-the-flag",
        ],
        &[
            ("THINKTHEN_URL", CLOSED),
            ("THINKTHEN_ADAPTER", "systemone"),
            ("THINKTHEN_MODEL", "from-the-environment"),
        ],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(printed.contains(r#""model":"from-the-flag""#), "{printed}");
}

#[test]
fn a_profile_named_only_by_the_environment_yields_to_an_ad_hoc_backend() {
    let ad_hoc = &[
        "decide",
        "if",
        "asks for a refund",
        "--plan",
        "--url",
        CLOSED,
        "--adapter",
        "systemone",
        "--model",
        "local-1",
    ];

    let output = run(
        ad_hoc,
        &[("THINKTHEN_BACKEND", "jev")],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(printed.starts_with(r#"{"backend":null,"#), "{printed}");
    assert_eq!(output.status.code(), Some(0));

    let output = run(
        &[ad_hoc, &["--backend", "jev"][..]].concat(),
        &[],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(2));

    let output = run(
        &["decide", "if", "asks for a refund", "--plan"],
        &[
            ("THINKTHEN_BACKEND", "jev"),
            ("THINKTHEN_URL", CLOSED),
            ("THINKTHEN_ADAPTER", "systemone"),
            ("THINKTHEN_MODEL", "local-1"),
        ],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn an_empty_environment_variable_is_unset_and_white_space_is_still_refused() {
    let output = run(
        &["decide", "if", "asks for a refund", "--plan"],
        &[("THINKTHEN_BACKEND", ""), ("THINKTHEN_MODEL", "")],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(printed.contains(r#""backend":"jev""#), "{printed}");
    assert!(printed.contains(r#""model":"jev-latest""#), "{printed}");
    assert_eq!(output.status.code(), Some(0));

    let output = run(
        &["decide", "if", "asks for a refund", "--plan"],
        &[("THINKTHEN_MODEL", " ")],
        b"Refund me please.",
    )
    .expect("the compiled binary runs");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn each_way_of_naming_no_backend_and_no_mark_is_a_usage_error() {
    let cases: [(&[&str], &str); 8] = [
        (&["--url", CLOSED], "a url with no adapter and no model"),
        (
            &["--url", CLOSED, "--adapter", "systemone"],
            "a url with no model",
        ),
        (&["--adapter", "systemone"], "an adapter with no url"),
        (&["--backend", "nowhere"], "a profile that does not exist"),
        (
            &["--url", CLOSED, "--adapter", "rest", "--model", "m"],
            "an adapter that does not exist",
        ),
        (
            &[
                "--backend",
                "jev",
                "--url",
                CLOSED,
                "--adapter",
                "systemone",
                "--model",
                "m",
            ],
            "a named profile beside a url",
        ),
        (&["--model", ""], "a blank model"),
        (&["--min-prob", "0.5"], "a pass mark a coin would pass"),
    ];

    for (arguments, what) in cases {
        let output = decide(arguments).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{what}");
        assert!(output.stdout.is_empty(), "{what}");
        assert!(!output.stderr.is_empty(), "{what}");
    }
}

#[test]
fn the_status_flag_without_a_pass_mark_is_a_usage_error() {
    let output = decide(&["--status"]).expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
    let message = String::from_utf8_lossy(&output.stderr);
    assert!(message.contains("--min-prob"), "{message}");
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
        let output = run(
            &["decide", "if", "asks for a refund", "--plan"],
            &[],
            evidence,
        )
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
fn a_blank_condition_is_a_usage_error() {
    let output = run(&["decide", "if", "  ", "--plan"], &[], b"Refund me please.")
        .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn no_diagnostic_ever_carries_the_key_or_the_evidence() {
    let secret = "sk-never-printed";
    let evidence = "The customer wrote something private.";
    let output = run(
        &[
            "decide",
            "if",
            "asks for a refund",
            "--key-env",
            "LOCAL_KEY",
        ],
        &[("LOCAL_KEY", secret), ("THINKTHEN_BACKEND", "nowhere")],
        evidence.as_bytes(),
    )
    .expect("the compiled binary runs");

    let message = String::from_utf8_lossy(&output.stderr);
    assert!(!message.contains(secret), "{message}");
    assert!(!message.contains(evidence), "{message}");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn the_help_for_decide_if_names_the_condition_and_every_option() {
    let output = run(&["decide", "if", "--help"], &[], b"").expect("the compiled binary runs");

    let help = String::from_utf8_lossy(&output.stdout);
    for option in [
        "--min-prob",
        "--status",
        "--plan",
        "--backend",
        "--url",
        "--adapter",
        "--model",
        "--key-env",
        "--record",
        "--replay",
        "--timeout",
        "--max-retries",
    ] {
        assert!(help.contains(option), "{option} is missing from {help}");
    }
    assert!(!help.contains("THINKTHEN_TEST_RETRY_WAIT_MS"), "{help}");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn an_unknown_word_is_a_usage_error_and_never_an_instruction() {
    for arguments in [
        &["decide", "sideways", "something"][..],
        &["decide", "if", "a", "b"][..],
        &["think", "about", "it"][..],
    ] {
        let output = run(arguments, &[], b"Refund me please.").expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
    }
}
