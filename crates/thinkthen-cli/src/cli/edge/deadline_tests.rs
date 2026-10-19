//! Stays: spent deadlines must stop direct and record paths before any send.
//! A spent deadline on every command request path sends nothing.
//!
//! The command exposes no deadline, so these tests give the in-process
//! request one and count each boundary: request accounting,
//! and connections on the loopback listener.

use std::fs;
use std::io::{Cursor, ErrorKind};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser as _;

use super::Environment;
use crate::cli::args::{Cli, Command};
use crate::cli::failure::{Failure, report};
#[path = "../../../../thinkthen/src/test_deadline/child.rs"]
mod child;
use child::{ChildEnvironment as _, Folder};

const CHILD_HOME: &str = "THINKTHEN_TEST_COMMAND_DEADLINE_HOME";

/// Reuse the shared cleared-environment fixture for native captured usage paths.
fn isolated(name: &str) -> bool {
    if std::env::var_os(CHILD_HOME).is_some() { return true; }
    let home = scratch(name);
    let path = format!("cli::edge::deadline_tests::{name}");
    let mut command = std::process::Command::new(std::env::current_exe().expect("test binary"));
    command.args(["--exact", &path, "--nocapture", "--test-threads=1"])
        .clear_environment().isolated_home(&home).env(CHILD_HOME, &home);
    let output = child::run::output(&mut command).expect("isolated deadline test");
    fs::remove_dir_all(home).expect("child home removed");
    assert!(output.status.success(), "{}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stdout).contains("1 passed"), "the child ran its test");
    false
}

/// A label, the command's arguments, and its standard input.
type Case<'a> = (&'a str, Vec<&'a str>, &'a [u8]);

const DEADLINE: &str = "thinkthen: defect: an unavailable deadline reached the command\n";

fn scratch(label: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "thinkthen-deadline-command-{label}-{}",
        std::process::id()
    ));
    let _absent = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).expect("scratch folder");
    path
}

/// Run one command with a spent deadline and return what it reported.
fn spent(label: &str, arguments: &[&str], input: &[u8]) -> (ExitCode, String) {
    let folder = scratch(label);
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let base = format!("http://{}/v1", listener.local_addr().expect("address"));
    let mut line = vec!["thinkthen"];
    line.extend_from_slice(arguments);
    line.extend_from_slice(&["--url", &base, "--model", "local-1", "--no-cache"]);
    let cli = Cli::try_parse_from(line).expect("command line");
    let environment = Environment::read().expect("isolated captured configuration");
    let input = Cursor::new(input.to_vec());
    let mut output = Vec::new();

    let result = match &cli.command {
        Some(Command::Decide(held)) => crate::cli::judge::decide(
            held,
            &environment,
            admitted(cli.command.as_ref().expect("command")),
            input,
            &mut output,
        ),
        Some(Command::Find(held)) => {
            let admitted = admitted(cli.command.as_ref().expect("command"));
            crate::cli::find::run(held, &environment, admitted, input, &mut output)
        }
        Some(Command::Annotate(held)) => crate::cli::annotate::run(
            held,
            &environment,
            admitted(cli.command.as_ref().expect("command")),
            input,
            &mut output,
        ),
        Some(Command::Recognize(held)) => crate::cli::recognize::run(
            held,
            &environment,
            admitted(cli.command.as_ref().expect("command")),
            input,
            &mut output,
        ),
        Some(Command::Relate(held)) => crate::cli::relate::run(
            held,
            &environment,
            admitted(cli.command.as_ref().expect("command")),
            input,
            &mut output,
        ),
        _ => panic!("no runner arm for {label}"),
    };

    assert_eq!(environment.snapshot().counts.requests_sent(), 0, "{label} requests counted");
    assert!(environment.finish_usage().is_none(), "{label} persisted usage safely");
    let home = std::env::var_os(CHILD_HOME).expect("isolated child home");
    let totals = crate::read_usage(&Folder::Usage.under(std::path::Path::new(&home)), &crate::current_usage_month())
        .expect("persisted usage totals");
    assert_eq!(totals.total.requests_sent(), 0, "{label} persisted requests counted");
    assert!(
        matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock),
        "{label} opened a connection"
    );
    assert!(output.is_empty(), "{label} printed a result");
    let failure: Failure = result.expect_err(label);
    let mut written = Vec::new();
    let code = report(&failure, &mut written);
    fs::remove_dir_all(folder).expect("scratch removed");
    (code, String::from_utf8(written).expect("diagnostic text"))
}

/// Use the public deadline control on the actual admitted command header.
/// Private key-reader admission counts remain owned by the library tests.
fn admitted(command: &Command) -> crate::AdmittedRequest {
    let admitted = crate::cli::request::admit(command)
        .expect("native header").expect("judging request");
    let mut request = admitted.request().clone();
    request.call.arguments.options.deadline_ms = Some(0);
    request.admit().expect("spent deadline is an admitted control")
}

fn questions(label: &str) -> String {
    file(
        label,
        "questions.json",
        r#"{"version":1,"questions":{"risky":{"decide":"Is this risky?"},"kind":{"choose":"What kind?","options":["bug","other"]}}}"#,
    )
}

fn remove(file: &str) {
    let folder = std::path::Path::new(file).parent().expect("fixture folder");
    fs::remove_dir_all(folder).expect("fixture removed");
}

fn file(label: &str, name: &str, text: &str) -> String {
    let path = scratch(label).join(name);
    fs::write(&path, text).expect("fixture file");
    path.display().to_string()
}

#[test]
fn a_spent_deadline_on_every_direct_path_sends_nothing() {
    if !isolated("a_spent_deadline_on_every_direct_path_sends_nothing") { return; }
    let annotate = questions("annotate-questions");
    let split = file(
        "split-profile",
        "profile.json",
        r#"{"schema":"thinkthen.backend-profile/1","name":"one","max_questions":1}"#,
    );
    let entities = br#"[{"name":"gateway","kind":"service"},{"name":"billing","kind":"service"}]"#;
    let cases: [Case<'_>; 7] = [
        ("decide", vec!["decide", "Is it accepted?"], b"evidence"),
        (
            "find",
            vec!["find", "Which unit answers?"],
            b"first\nsecond\n",
        ),
        ("annotate", vec!["annotate", &annotate], b"evidence"),
        ("recognize", vec!["recognize"], b"Ada works at Acme"),
        (
            "split",
            vec!["recognize", "--profile", &split],
            b"Ada works at Acme",
        ),
        ("relate", vec!["relate", "calls=service:service"], entities),
        (
            "relate-split",
            vec!["relate", "calls=service:service", "--profile", &split],
            entities,
        ),
    ];
    for (label, arguments, input) in cases {
        let (code, written) = spent(label, &arguments, input);
        assert_eq!(code, ExitCode::from(70), "{label}: {written}");
        assert_eq!(written, DEADLINE, "{label}");
    }
    remove(&annotate);
    remove(&split);
}

#[test]
fn a_spent_deadline_over_records_stops_before_the_first_request() {
    if !isolated("a_spent_deadline_over_records_stops_before_the_first_request") { return; }
    let stopped = format!("{DEADLINE}thinkthen: stopped at record 1; 0 records finished\n");
    let annotate = questions("annotate-records-questions");
    let cases: [Case<'_>; 4] = [
        (
            "records",
            vec!["decide", "Is it accepted?", "--jsonl", "--field", "/body"],
            b"{\"body\":\"one\"}\n{\"body\":\"two\"}\n",
        ),
        (
            "empty-records",
            vec!["decide", "Is it accepted?", "--jsonl"],
            b"",
        ),
        (
            "annotate-records",
            vec!["annotate", &annotate, "--jsonl", "--field", "/body"],
            b"{\"body\":\"one\"}\n",
        ),
        (
            "empty-annotate",
            vec!["annotate", &annotate, "--jsonl"],
            b"",
        ),
    ];
    for (label, arguments, input) in cases {
        let (code, written) = spent(label, &arguments, input);
        assert_eq!(code, ExitCode::from(70), "{label}: {written}");
        assert_eq!(written, stopped, "{label}");
    }
    remove(&annotate);
}
