//! Ticket 0143: one relate's requests go out together under `--jobs`.

use std::fs;
use std::path::PathBuf;
use std::process::Output;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{answered, run};
use crate::harness::{Canned, Gathering, Listener};

/// The relation whose request carries this body.
fn relation(body: &[u8]) -> String {
    let request: serde_json::Value = serde_json::from_slice(body).expect("request");
    request["questions"]["q1"]["instructions"]
        .as_str()
        .expect("words")
        .split_whitespace()
        .nth(5)
        .expect("relation word")
        .to_owned()
}

fn said(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn relate_requests_reach_the_throttle_and_no_further() {
    let gathering = Gathering::new(3);
    let listener = Listener::answering(move |body| {
        gathering.hold();
        answered(body)
    })
    .expect("listener");
    let names = (0..36).map(|n| format!("name {n}\n")).collect::<String>();
    let options = ["r", "--lines", "--jobs", "3"];
    let output = run(&listener, &options, names.as_bytes());

    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    assert_eq!(listener.requests().len(), 4);
    assert_eq!(listener.peak(), 3);
}

/// A profile that splits twelve pair questions into three requests.
fn four_a_request() -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-at-once");
    fs::create_dir_all(&folder).expect("profile folder");
    let path = folder.join("four.json");
    let text = r#"{"schema":"thinkthen.backend-profile/1","name":"four","max_questions":4}"#;
    fs::write(&path, text).expect("profile");
    path
}

/// Two rules split into three requests each, over one listener whose earlier
/// requests answer later. The digests name the address, so both runs share it.
fn reversed(listener: &Listener, jobs: &str, details: bool) -> Output {
    let profile = four_a_request();
    let mut options = vec![
        "knows",
        "likes",
        "--lines",
        "--profile",
        profile.to_str().expect("path"),
        "--jobs",
        jobs,
    ];
    if details {
        options.push("--details");
    }
    run(listener, &options, b"ada\nbob\ncy\ndee\n")
}

#[test]
fn split_relations_print_what_one_job_prints() {
    for details in [false, true] {
        let arrived = AtomicU64::new(0);
        let listener = Listener::answering(move |body| {
            let place = arrived.fetch_add(1, Ordering::SeqCst) % 6;
            answered(body).after(40 * (6 - place))
        })
        .expect("listener");
        let one = reversed(&listener, "1", details);
        let four = reversed(&listener, "4", details);
        assert_eq!(one.status.code(), Some(0), "{}", said(&one));
        assert!(!one.stdout.is_empty());
        if details {
            let judgment = |output: &Output| {
                crate::native_results::compatibility::judgment(
                    serde_json::from_slice(&output.stdout).unwrap(),
                )
            };
            assert_eq!(judgment(&four), judgment(&one));
        } else {
            assert_eq!(four.stdout, one.stdout);
        }
        assert_eq!((four.status.code(), said(&four)), (Some(0), String::new()));
        assert_eq!(listener.requests().len(), 12);
        assert!(listener.peak() > 1, "peak {}", listener.peak());
    }
}

/// One run of six one-request rules where `fail` scripts each relation's reply.
fn failing(jobs: &str, fail: fn(&str) -> Canned) -> (Output, usize) {
    let listener = Listener::answering(move |body| fail(&relation(body))).expect("listener");
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("relate-at-once");
    fs::create_dir_all(&folder).expect("profile folder");
    let profile = folder.join("two.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"two","max_questions":2}"#,
    )
    .expect("profile");
    let options = [
        "alpha",
        "bravo",
        "charlie",
        "delta",
        "echo",
        "foxtrot",
        "--lines",
        "--profile",
        profile.to_str().expect("path"),
        "--max-retries",
        "0",
        "--jobs",
        jobs,
    ];
    let output = run(&listener, &options, b"ada\nbob\n");
    (output, listener.requests().len())
}

fn ok() -> Canned {
    Canned::ok(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#,
    )
}

fn first_fails_at_once(name: &str) -> Canned {
    match name {
        "alpha" => Canned::status(500, "{}"),
        "bravo" => ok().after(300),
        _ => ok(),
    }
}

fn second_fails_first(name: &str) -> Canned {
    match name {
        "alpha" => Canned::status(500, "{}").after(100),
        "bravo" => Canned::status(503, "{}"),
        _ => ok(),
    }
}

/// Bravo answers first, from another model, so the closure fails once
/// alpha answers. Charlie went out when bravo's reply freed a worker.
fn a_later_model_differs(name: &str) -> Canned {
    match name {
        "alpha" => ok().after(300),
        "bravo" => Canned::ok(
            r#"{"model":"other-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.9}}}"#,
        ),
        _ => ok().after(600),
    }
}

#[test]
fn a_failed_chunk_stops_the_run_as_one_job_does() {
    let (one, sent_one) = failing("1", first_fails_at_once);
    let (two, sent_two) = failing("2", first_fails_at_once);
    assert_ne!(one.status.code(), Some(0));
    assert!(said(&one).contains("500"), "{}", said(&one));
    assert_eq!(
        (two.status.code(), said(&two), &two.stdout),
        (one.status.code(), said(&one), &one.stdout)
    );
    assert_eq!((sent_one, sent_two), (1, 2));

    let (one, _) = failing("1", second_fails_first);
    let (two, sent_two) = failing("2", second_fails_first);
    assert!(said(&one).contains("500"), "{}", said(&one));
    assert_eq!(
        (two.status.code(), said(&two), &two.stdout),
        (one.status.code(), said(&one), &one.stdout)
    );
    assert_eq!(sent_two, 2);

    let (one, sent_one) = failing("1", a_later_model_differs);
    let (two, sent_two) = failing("2", a_later_model_differs);
    assert_eq!(one.status.code(), Some(4), "{}", said(&one));
    assert_eq!(
        (two.status.code(), said(&two), &two.stdout),
        (one.status.code(), said(&one), &one.stdout)
    );
    assert_eq!((sent_one, sent_two), (2, 3));
}
