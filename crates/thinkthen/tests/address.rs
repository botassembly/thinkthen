//! The two variables at the edge: where a request goes, and the key it carries.

mod harness;

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use harness::{Canned, Listener};

/// The response the listener gives to the one question the command asks.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":312,"output_tokens":48}}"#,
);

/// The evidence every case on this page judges.
const EVIDENCE: &[u8] = b"Refund me please.";

/// The address the tool posts to when nothing names another one.
const BUILT_IN: &str = "https://api.typesafe.ai/v1/systemone";

/// Run `decide` over the evidence, with no environment but what the case names.
fn decide(arguments: &[&str], environment: &[(&str, &str)]) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_thinkthen"));
    command
        .env_clear()
        .env("THINKTHEN_TEST_RETRY_WAIT_MS", "1")
        .args(["decide", "asks for a refund"])
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
    let _ = input.write_all(EVIDENCE);
    drop(input);
    child.wait_with_output()
}

/// A folder this test owns, removed and remade so each run starts empty.
fn folder(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _absent = fs::remove_dir_all(&path);
    path
}

#[test]
fn the_option_outranks_the_variable_and_the_variable_outranks_the_default() {
    let chosen = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let ignored = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output = decide(
        &[
            "--url",
            chosen.base(),
            "--adapter",
            "systemone",
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_BASE_URL", ignored.base())],
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    assert_eq!(chosen.requests().len(), 1, "the option names the address");
    assert!(ignored.requests().is_empty(), "the variable was outranked");

    let named = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
    let output = decide(
        &[],
        &[
            ("THINKTHEN_BASE_URL", named.base()),
            ("THINKTHEN_API_KEY", "sk-test-value"),
        ],
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let requests = named.requests();
    let request = requests.first().expect("the variable names the address");
    assert_eq!(request.line, "POST /v1/systemone HTTP/1.1");

    let output = decide(&["--dry-run"], &[]).expect("the compiled binary runs");
    let printed = String::from_utf8_lossy(&output.stdout);
    assert!(
        printed.contains(&format!(r#""url":"{BUILT_IN}""#)),
        "{printed}"
    );
}

#[test]
fn a_base_reaches_the_same_path_with_a_trailing_slash_and_without_one() {
    for base in ["", "/", "//"] {
        let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
        let given = format!("{}{base}", listener.base());

        let output = decide(
            &[
                "--url",
                &given,
                "--adapter",
                "systemone",
                "--model",
                "local-1",
            ],
            &[],
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{given}");
        let requests = listener.requests();
        let request = requests.first().expect("one request reached the listener");
        assert_eq!(request.line, "POST /v1/systemone HTTP/1.1", "{given}");
    }
}

#[test]
fn a_base_that_is_not_an_http_address_is_a_usage_error_that_shows_no_address() {
    for bad in [
        "ftp://127.0.0.1/v1",
        "127.0.0.1:8080/v1",
        "file:///tmp/v1",
        "http:/127.0.0.1/v1",
        "https//127.0.0.1/v1",
        "/v1",
        "https://someone:sk-in-the-address@127.0.0.1/v1",
    ] {
        let output = decide(&[], &[("THINKTHEN_BASE_URL", bad)]).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(2), "{bad}");
        assert!(output.stdout.is_empty(), "{bad}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(
            message.starts_with("thinkthen: a base address"),
            "{message}"
        );
        assert!(!message.contains("127.0.0.1"), "{message}");
        assert!(!message.contains("sk-in-the-address"), "{message}");
    }
}

#[test]
fn a_base_is_read_past_its_surrounding_space_and_past_the_case_of_its_scheme() {
    for shape in ["  {base}  ", "\t{base}\n", "{upper}"] {
        let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
        let upper = listener.base().replacen("http://", "HTTP://", 1);
        let given = shape
            .replace("{base}", listener.base())
            .replace("{upper}", &upper);

        let output = decide(
            &[],
            &[("THINKTHEN_BASE_URL", &given), ("THINKTHEN_API_KEY", "sk")],
        )
        .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{given:?}");
        let requests = listener.requests();
        let request = requests.first().expect("one request reached the listener");
        assert_eq!(request.line, "POST /v1/systemone HTTP/1.1", "{given:?}");
    }
}

#[test]
fn a_variable_that_holds_nothing_counts_as_absent() {
    for empty in ["", " "] {
        let output = decide(&["--dry-run"], &[("THINKTHEN_BASE_URL", empty)])
            .expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(0), "{empty:?}");
        let printed = String::from_utf8_lossy(&output.stdout);
        assert!(
            printed.contains(&format!(r#""url":"{BUILT_IN}""#)),
            "{printed}"
        );
    }
}

#[test]
fn the_key_comes_from_thinkthen_api_key_and_reaches_nothing_but_the_header() {
    let secret = "sk-never-printed";
    let folder = folder("key");
    let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");

    let output = decide(
        &["--details", "--record", &folder.to_string_lossy()],
        &[
            ("THINKTHEN_BASE_URL", listener.base()),
            ("THINKTHEN_API_KEY", secret),
        ],
    )
    .expect("the compiled binary runs");

    assert_eq!(output.status.code(), Some(0));
    let requests = listener.requests();
    let request = requests.first().expect("one request reached the listener");
    assert_eq!(
        request.header("authorization"),
        Some("Bearer sk-never-printed")
    );

    let printed = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!printed.contains(secret), "{printed}");

    let mut entries = 0;
    for entry in fs::read_dir(&folder).expect("the recording folder is there") {
        let path = entry.expect("an entry").path();
        let written = fs::read_to_string(&path).expect("an entry is text");
        for shown in [secret, "authorization", "Authorization", "Bearer"] {
            assert!(!written.contains(shown), "{written}");
        }
        entries += 1;
    }
    assert_eq!(entries, 1, "the run recorded its one exchange");

    let plan =
        decide(&["--dry-run"], &[("THINKTHEN_API_KEY", secret)]).expect("the compiled binary runs");
    let printed = String::from_utf8_lossy(&plan.stdout);
    assert!(
        printed.contains(r#""key_env":"THINKTHEN_API_KEY""#),
        "{printed}"
    );
    assert!(!printed.contains(secret), "{printed}");
}

#[test]
fn a_key_that_is_unset_or_empty_is_exit_four_and_names_the_variable_it_read() {
    let listener = Listener::serving(Vec::new()).expect("a loopback listener");
    let unset: &[(&str, &str)] = &[("THINKTHEN_BASE_URL", listener.base())];
    let empty: &[(&str, &str)] = &[
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", ""),
    ];
    let blank: &[(&str, &str)] = &[
        ("THINKTHEN_BASE_URL", listener.base()),
        ("THINKTHEN_API_KEY", "   "),
    ];

    for environment in [unset, empty, blank] {
        let output = decide(&[], environment).expect("the compiled binary runs");

        assert_eq!(output.status.code(), Some(4), "{environment:?}");
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains("THINKTHEN_API_KEY"), "{message}");
        assert!(output.stdout.is_empty(), "{environment:?}");
    }

    assert!(listener.requests().is_empty(), "no key, no request");
}
