//! The conformance backend's generic and fault arms, crossed by the compiled command.

use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Output;
use std::thread;
use std::time::{Duration, Instant};

use conformance_backend::Backend;

use crate::harness::spawn;

const KEY: [(&str, &str); 1] = [("THINKTHEN_API_KEY", "sk-loopback-arms")];

/// A question set with one yes-or-no question and one choice after it.
const SET: &str = r#"{"version":1,"questions":{"ready":{"decide":"Is it ready?"},"kind":{"choose":"Which kind?","options":["bug","other"]}}}"#;

/// Run the command with its arguments, a base under the backend, and the input.
fn run(backend: &Backend, path: &str, arguments: &[&str], input: &str) -> io::Result<Output> {
    let base = format!("{}{path}", backend.origin());
    let tail = ["--url", base.as_str(), "--no-cache"];
    spawn(&[arguments, &tail[..]].concat(), &KEY, input.as_bytes())
}

/// Write one file into a folder of its own and return its path.
fn file(name: &str, text: &str) -> io::Result<PathBuf> {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("loopback-arms");
    fs::create_dir_all(&folder)?;
    let path = folder.join(name);
    fs::write(&path, text)?;
    Ok(path)
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn an_unknown_body_on_the_case_arm_earns_the_drift_status_and_never_a_generic_answer() {
    let backend = Backend::start().expect("backend");
    let output = run(
        &backend,
        "/case/01-decide-yes-captured/v1",
        &["decide", "Is this a body no case holds?"],
        "unknown",
    )
    .expect("the command runs");
    assert_eq!(text(&output.stdout), "");
    assert_eq!(
        text(&output.stderr),
        "thinkthen: the backend answered with status 500: the backend failed after the allowed attempts; try again later or change --max-retries\n"
    );
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        backend.count(),
        4,
        "the default three retries follow the first 500"
    );
}

#[test]
fn each_wire_fault_arm_yields_its_kind_and_sentence() {
    let arms = [
        (
            "reset",
            1,
            "thinkthen: the backend closed the connection before a reply and may have received the request; it was not sent again\n",
        ),
        (
            "429",
            4,
            "thinkthen: the backend answered with status 429: the backend's rate limit was reached after the allowed attempts; try again later or change --max-retries\n",
        ),
        (
            "503",
            4,
            "thinkthen: the backend answered with status 503: the backend failed after the allowed attempts; try again later or change --max-retries\n",
        ),
        (
            "refuse",
            1,
            "thinkthen: the backend answered with status 422: the backend refused the request as malformed or too large\n",
        ),
    ];
    for (arm, requests, sentence) in arms {
        let backend = Backend::start().expect("backend");
        let path = format!("/arm/{arm}/v1");
        let output = run(&backend, &path, &["decide", "Is it?"], "hi").expect("the command runs");
        assert_eq!(text(&output.stderr), sentence, "{arm}");
        assert_eq!(output.status.code(), Some(4), "{arm} is a backend failure");
        assert_eq!(text(&output.stdout), "", "{arm}");
        assert_eq!(backend.count(), requests, "{arm}");
    }
}

#[test]
fn each_malformed_arm_fails_the_last_question_with_its_cause() {
    let set = file("malformed.json", SET).expect("question set");
    let set = set.to_str().expect("a UTF-8 path");
    for cause in [
        "missing_answer",
        "wrong_kind",
        "missing_probability",
        "invalid_probability",
        "invalid_distribution",
        "unexpected_probability",
    ] {
        let backend = Backend::start().expect("backend");
        let path = format!("/arm/malformed/{cause}/v1");
        let output = run(&backend, &path, &["annotate", set], "hi").expect("the command runs");
        assert_eq!(
            text(&output.stdout),
            format!(
                "{{\"ready\":true,\"kind\":{{\"failed\":{{\"kind\":\"backend\",\"cause\":\"{cause}\"}}}}}}\n"
            ),
        );
        assert_eq!(
            output.status.code(),
            Some(6),
            "{cause} is a partial failure"
        );
        assert_eq!(backend.count(), 1, "{cause}");
    }
}

#[test]
fn a_held_reply_answers_only_after_its_release() {
    let backend = Backend::start().expect("backend");
    let base = format!("{}/arm/held/v1", backend.origin());
    let command = thread::spawn(move || {
        spawn(
            &["decide", "Is it?", "--url", base.as_str(), "--no-cache"],
            &KEY,
            b"hi",
        )
    });
    let deadline = Instant::now() + Duration::from_secs(10);
    while backend.count() == 0 {
        assert!(Instant::now() < deadline, "the held request never arrived");
        thread::sleep(Duration::from_millis(10));
    }
    thread::sleep(Duration::from_millis(300));
    assert!(
        !command.is_finished(),
        "the held reply went before its release"
    );
    backend.release();
    let output = command
        .join()
        .expect("command thread")
        .expect("the command runs");
    assert_eq!(text(&output.stdout), "true\n");
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn the_generic_arm_answers_every_verb() {
    let set = file("generic.json", SET).expect("question set");
    let set = set.to_str().expect("a UTF-8 path");
    let recognize = file(
        "recognize.json",
        r#"{"version":1,"recognize":{"kinds":{"person":"A person's name."}},"threshold":0.5}"#,
    )
    .expect("recognize question");
    let recognize = format!("@{}", recognize.display());
    let relate = file(
        "relate.json",
        r#"{"version":1,"relate":{"relations":[{"name":"same_as","source":"alert","target":"alert","either":true}]},"threshold":0.5}"#,
    )
    .expect("relate question");
    let relate = format!("@{}", relate.display());
    let entities =
        r#"[{"name":"Checkout fails.","kind":"alert"},{"name":"Cards fail.","kind":"alert"}]"#;
    let runs: [(&[&str], &str, &str); 10] = [
        (&["decide", "Is it?"], "hi", "true\n"),
        (&["choose", "Which?", "a", "b", "c"], "hi", "\"a\"\n"),
        (&["tag", "Which?", "a", "b"], "hi", "[\"a\",\"b\"]\n"),
        (
            &["score", "How much?", "low", "mid", "high"],
            "hi",
            "0.15\n",
        ),
        (&["filter", "Is it?", "--lines"], "one\ntwo\n", "one\ntwo\n"),
        (&["rank", "Is it?", "--lines"], "one\ntwo\n", "one\ntwo\n"),
        (&["find", "Which?"], "one\ntwo\n", "one\n"),
        (
            &["annotate", set],
            "hi",
            "{\"ready\":true,\"kind\":\"bug\"}\n",
        ),
        (
            &["recognize", recognize.as_str()],
            "Maria Chen arrived.",
            concat!(
                "{\"entities\":[{\"text\":\"Maria Chen\",\"start\":0,\"end\":10,\"length\":10,\"kind\":\"person\",\"strength\":0.6736},",
                "{\"text\":\"arrived.\",\"start\":11,\"end\":19,\"length\":8,\"kind\":\"person\",\"strength\":0.6736}]}\n"
            ),
        ),
        (
            &["relate", relate.as_str()],
            entities,
            "{\"relation\":\"same_as\",\"source\":{\"name\":\"Checkout fails.\",\"kind\":\"alert\"},\"target\":{\"name\":\"Cards fail.\",\"kind\":\"alert\"},\"probability\":0.9,\"either\":true}\n",
        ),
    ];
    let backend = Backend::start().expect("backend");
    for (arguments, input, printed) in runs {
        let output = run(&backend, "/generic/v1", arguments, input).expect("the command runs");
        assert_eq!(text(&output.stdout), printed, "{arguments:?}");
        assert_eq!(output.status.code(), Some(0), "{arguments:?}");
    }
}
