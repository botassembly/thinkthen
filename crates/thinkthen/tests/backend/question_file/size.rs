//! The 1 MiB cap on a question file or question set, on every verb that reads one.
//!
//! Each file is a valid question padded with trailing spaces, so one byte is
//! the whole difference between the file that runs and the file refused.

use std::io::ErrorKind;
use std::net::TcpListener;

use super::harness::{run, run_with, written};

/// The cap in bytes: 1 MiB, the libraries' question-file cap.
const LIMIT: usize = 1_048_576;

/// Each verb that reads `@FILE`, a valid file for it, and evidence it accepts.
const VERBS: [(&str, &str, &str); 9] = [
    ("decide", r#"{"decide":"Is it?"}"#, "hello"),
    ("filter", r#"{"decide":"Is it?"}"#, "hello"),
    ("rank", r#"{"decide":"Is it?"}"#, "hello"),
    (
        "choose",
        r#"{"choose":"Which?","options":["a","b"]}"#,
        "hello",
    ),
    ("tag", r#"{"tag":"Which?","labels":["a","b"]}"#, "hello"),
    (
        "score",
        r#"{"score":"How?","levels":["low","high"]}"#,
        "hello",
    ),
    (
        "annotate",
        r#"{"version":1,"questions":{"q":{"decide":"Is it?"}}}"#,
        "hello",
    ),
    ("recognize", r#"{"version":1,"recognize":{}}"#, "hello"),
    (
        "relate",
        r#"{"version":1,"relate":{"relations":[{"name":"r","source":"a","target":"b"}]}}"#,
        r#"[{"name":"Ann","kind":"a"},{"name":"Acme","kind":"b"}]"#,
    ),
];

/// `text` padded with spaces to `size` bytes.
fn padded(text: &str, size: usize) -> String {
    format!("{text}{}", " ".repeat(size - text.len()))
}

#[test]
fn a_question_file_of_one_mib_runs_and_one_byte_more_is_refused_before_any_request() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a loopback listener");
    listener.set_nonblocking(true).expect("nonblocking");
    let url = format!("http://{}/v1", listener.local_addr().expect("an address"));
    for (verb, text, evidence) in VERBS {
        let fits = written(&format!("size-{verb}-fits"), &padded(text, LIMIT));
        let output = run(&[verb, &fits, "--plan", "--url", &url], evidence.as_bytes())
            .expect("the compiled binary runs");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{verb}: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let over = written(&format!("size-{verb}-over"), &padded(text, LIMIT + 1));
        let mut files = vec![over.as_str()];
        // Windows has no endless file like `/dev/zero`.
        if cfg!(unix) {
            files.push("@/dev/zero");
        }
        for file in files {
            let output = run_with(
                &[verb, file, "--url", &url, "--no-cache"],
                evidence.as_bytes(),
                "sk-test",
            )
            .expect("the compiled binary runs");
            assert_eq!(output.status.code(), Some(5), "{verb} {file}");
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                "thinkthen: the question file is too large\n",
                "{verb} {file}"
            );
            assert!(output.stdout.is_empty(), "{verb} {file}");
        }
    }
    let sent = listener.accept().map(|_| ());
    assert!(
        matches!(&sent, Err(error) if error.kind() == ErrorKind::WouldBlock),
        "no command reached the backend: {sent:?}"
    );
}

/// Ticket 0345: `annotate` looks at `--input` to name a swapped question set
/// under the same cap, so a broken set beside `--input /dev/zero` fails at once.
#[test]
fn a_broken_question_set_beside_an_endless_input_fails_without_reading_it_all() {
    let broken = written("size-annotate-broken", r#"{"version":1,"questions":{}}"#);
    let output = run(
        &["annotate", &broken, "--input", "/dev/zero", "--plan"],
        b"",
    )
    .expect("the compiled binary runs");
    assert_eq!(
        (
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ),
        (
            Some(5),
            "thinkthen: `questions` holds at least one named question\n".into()
        )
    );
    assert!(output.stdout.is_empty());
}
