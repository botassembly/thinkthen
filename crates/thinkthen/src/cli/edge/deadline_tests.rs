//! A spent deadline on every command request path sends nothing.
//!
//! The command exposes no deadline, so these tests give the in-process
//! environment one and count each boundary: key lookups, request accounting,
//! and connections on the loopback listener.

use std::fs;
use std::io::{Cursor, ErrorKind};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

use clap::Parser as _;

use super::Environment;
use crate::cli::args::{Cli, Command};
use crate::cli::failure::{Failure, report};
use crate::engine::usage::{self, Counters, month_now};
use crate::engine::{Cancel, Deadline};

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
    let totals = folder.join("usage");
    let environment = Environment {
        cancel: Cancel::default().with_deadline(Deadline::after(Duration::ZERO)),
        usage: std::sync::Arc::new(Counters::new(Some(totals.clone()))),
        ..Environment::default()
    };
    let input = Cursor::new(input.to_vec());
    let mut output = Vec::new();

    let result = match &cli.command {
        Some(Command::Decide(held)) => {
            crate::cli::judge::decide(held, &environment, input, &mut output)
        }
        Some(Command::Find(held)) => {
            let admitted = crate::cli::request::admit(cli.command.as_ref().expect("command"))
                .expect("native header")
                .expect("find request");
            crate::cli::find::run(held, &environment, admitted, input, &mut output)
        }
        Some(Command::Annotate(held)) => {
            crate::cli::annotate::run(held, &environment, input, &mut output)
        }
        Some(Command::Recognize(held)) => {
            crate::cli::recognize::run(held, &environment, input, &mut output)
        }
        Some(Command::Relate(held)) => {
            crate::cli::relate::run(held, &environment, input, &mut output)
        }
        _ => panic!("no runner arm for {label}"),
    };

    assert_eq!(environment.cancel.keys(), 0, "{label} key lookups");
    let counted = usage::read(&totals, &month_now()).expect("usage totals");
    assert_eq!(counted.total.requests_sent, 0, "{label} requests counted");
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
