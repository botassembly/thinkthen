//! A backend's too-large refusal splits one batch once, including on replay.

use super::{answering, decide, details, folder, lines, places, rows, text};
use crate::harness::{Canned, Listener};
use serde_json::json;

const SPLIT_ORDINALS: [[Option<u64>; 2]; 5] = [
    [Some(1), Some(2)],
    [Some(1), Some(2)],
    [Some(1), Some(2)],
    [Some(1), Some(3)],
    [Some(1), Some(3)],
];

fn sent(listener: &Listener) -> Vec<usize> {
    listener
        .requests()
        .iter()
        .map(|request| places(&request.body).len())
        .collect()
}

#[test]
fn a_too_large_batch_halves_once_and_counts_the_refused_request() {
    let named = r#"{"detail":{"error_type":"max_tokens_exceeded"}}"#;
    for (status, body, split) in [
        (413, "{}", true),
        (400, named, true),
        (400, "{}", false),
        (422, "{}", false),
    ] {
        let listener = Listener::answering(move |request| {
            if places(request).len() == 5 {
                Canned::status(status, body)
            } else {
                answering(request)
            }
        })
        .expect("a loopback listener");
        let output = decide(
            listener.base(),
            &[
                "--batch",
                "5",
                "--jobs",
                "1",
                "--max-retries",
                "0",
                "--no-cache",
                "--details",
            ],
            &[],
            &lines(1..=5),
        );
        if split {
            assert_eq!(
                output.status.code(),
                Some(0),
                "{status}: {}",
                text(&output.stderr)
            );
            assert!(output.stderr.is_empty(), "{status}");
            assert_eq!(sent(&listener), [5, 3, 2], "{status}");
            let printed = details(&output);
            assert_eq!(printed.len(), 5, "{status}");
            let expected = [(3, 1, 2), (3, 2, 2), (3, 3, 2), (2, 1, 1), (2, 2, 1)];
            for (row, (records, position, requests_sent)) in printed.iter().zip(expected) {
                assert_eq!(
                    row["meta"]["batch"],
                    json!({"setting":5,"records":records,"position":position,"closed":"size",
                        "usage":{"input_tokens":88,"output_tokens":12},
                        "requests_sent":requests_sent,"split":true}),
                    "{status}"
                );
            }
            for (place, row) in printed.iter().enumerate() {
                let ordinals: Vec<_> = row["meta"]["attempts"]
                    .as_array()
                    .expect("live split attempts")
                    .iter()
                    .map(|event| event["ordinal"].as_u64())
                    .collect();
                assert_eq!(ordinals, SPLIT_ORDINALS[place]);
                assert_eq!(row["meta"]["attempts"][0]["outcome"], "status");
                assert_eq!(row["meta"]["attempts"][0]["status"], status);
                assert_ne!(
                    row["meta"]["attempts"][0]["request_sha256"],
                    row["meta"]["attempts"][1]["request_sha256"]
                );
            }
            let attempts: Vec<_> = printed
                .iter()
                .map(|row| row["meta"]["requests_sent"].as_u64())
                .collect();
            assert_eq!(
                attempts,
                [Some(1), Some(1), Some(0), Some(1), Some(0)],
                "{status}"
            );
            assert!(
                printed[..3]
                    .iter()
                    .all(|row| row["meta"]["requests"][0]["digest"]
                        == printed[0]["meta"]["requests"][0]["digest"])
            );
        } else {
            assert_eq!(output.status.code(), Some(4), "{status}");
            assert_eq!(sent(&listener), [5], "{status}");
            assert!(output.stdout.is_empty(), "{status}");
        }
    }
}

#[test]
fn a_refused_second_half_keeps_first_rows_and_does_not_split_again() {
    let listener = Listener::answering(|request| {
        if places(request).len() == 5 || places(request).first().is_some_and(|&(_, at)| at == 4) {
            Canned::status(413, "{}")
        } else {
            answering(request)
        }
    })
    .expect("a loopback listener");
    let output = decide(
        listener.base(),
        &[
            "--batch",
            "5",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--no-cache",
        ],
        &[],
        &lines(1..=5),
    );
    assert_eq!(sent(&listener), [5, 3, 2]);
    assert_eq!(text(&output.stdout), rows(1..=3));
    assert_eq!(output.status.code(), Some(4));
    assert_eq!(
        text(&output.stderr),
        "thinkthen: stopped at record 4; the request for records 4 to 5 failed: the backend answered with status 413: the backend refused the request as too large; shorten the text, or set a lower --max-request-bytes or max_request_bytes with --profile; 3 records finished\n"
    );
}

#[test]
fn a_refused_first_half_sends_no_second_half() {
    let listener = Listener::answering(|request| {
        if places(request).len() >= 3 {
            Canned::status(413, "{}")
        } else {
            answering(request)
        }
    })
    .expect("a loopback listener");
    let output = decide(
        listener.base(),
        &[
            "--batch",
            "5",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--no-cache",
        ],
        &[],
        &lines(1..=5),
    );
    assert_eq!(sent(&listener), [5, 3]);
    assert!(output.stdout.is_empty());
    assert_eq!(output.status.code(), Some(4));
    assert!(text(&output.stderr).contains("the request for records 1 to 3 failed"));
}

#[test]
fn a_split_recording_replays_byte_for_byte() {
    let listener = Listener::answering(|request| {
        if places(request).len() == 5 {
            Canned::status(413, "{}")
        } else {
            answering(request)
        }
    })
    .expect("a loopback listener");
    let directory = folder("split-recording");
    let input = lines(1..=5);
    let recorded = decide(
        listener.base(),
        &["--batch", "5", "--record", &directory],
        &[],
        &input,
    );
    assert_eq!(
        recorded.status.code(),
        Some(0),
        "{}",
        text(&recorded.stderr)
    );
    assert_eq!(listener.count(), 3);
    let detailed_replay = decide(
        listener.base(),
        &["--batch", "5", "--replay", &directory, "--details"],
        &[],
        &input,
    );
    assert_eq!(detailed_replay.status.code(), Some(0));
    assert!(
        details(&detailed_replay)
            .iter()
            .all(|row| row["meta"].get("attempts").is_none())
    );
    assert_eq!(listener.count(), 3, "replay creates no live attempt");
    let replayed = decide(
        listener.base(),
        &["--batch", "5", "--replay", &directory],
        &[],
        &input,
    );
    assert_eq!(
        replayed.status.code(),
        Some(0),
        "{}",
        text(&replayed.stderr)
    );
    assert_eq!(replayed.stdout, recorded.stdout);
    assert_eq!(replayed.stderr, recorded.stderr);
    assert_eq!(listener.count(), 3, "replay sends no request");
}

#[test]
fn a_cached_first_half_counts_only_its_replayed_records() {
    let listener = Listener::answering(|request| {
        let places = places(request);
        if places.len() == 5 || places.first().is_some_and(|&(_, at)| at == 4) {
            Canned::status(413, "{}")
        } else {
            answering(request)
        }
    })
    .expect("a loopback listener");
    let cache = folder("mixed-halves");
    let first = decide(
        listener.base(),
        &["--batch", "3", "--cache", &cache],
        &[],
        &lines(1..=3),
    );
    assert_eq!(first.status.code(), Some(0), "{}", text(&first.stderr));
    let mixed = decide(
        listener.base(),
        &[
            "--batch",
            "5",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--cache",
            &cache,
        ],
        &[],
        &lines(1..=5),
    );
    assert_eq!(text(&mixed.stdout), rows(1..=3));
    assert_eq!(mixed.status.code(), Some(4));
    assert!(
        text(&mixed.stderr).contains("3 records finished, 3 records from a recording"),
        "{}",
        text(&mixed.stderr)
    );
    assert_eq!(sent(&listener), [3, 5, 2]);

    let replay = decide(
        listener.base(),
        &["--batch", "5", "--replay", &cache],
        &[],
        &lines(1..=5),
    );
    assert_eq!(replay.status.code(), Some(5));
    assert_eq!(text(&replay.stdout), rows(1..=3));
    assert!(
        text(&replay.stderr).contains("3 records finished, 3 records from a recording"),
        "{}",
        text(&replay.stderr)
    );
    assert_eq!(listener.count(), 3, "replay sends no request");
}

#[test]
fn repeated_members_split_by_record_position() {
    let listener = Listener::answering(|request| {
        if places(request).len() == 4 {
            Canned::status(413, "{}")
        } else {
            answering(request)
        }
    })
    .expect("a loopback listener");
    let output = decide(
        listener.base(),
        &["--batch", "5", "--no-cache"],
        &[],
        "line 1\nline 1\nline 2\nline 3\nline 4\n",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let bodies = listener.requests();
    assert_eq!(bodies.len(), 3);
    assert_eq!(
        bodies
            .iter()
            .map(|body| places(&body.body).len())
            .collect::<Vec<_>>(),
        [4, 2, 2]
    );
    assert_eq!(text(&output.stdout).lines().count(), 5);
}

#[test]
fn estimated_total_charges_the_refused_parent_and_refuses_a_split_child() {
    let answer = |request: &[u8]| {
        if places(request).len() == 5 {
            Canned::status(400, r#"{"detail":{"error_type":"max_tokens_exceeded"}}"#)
        } else {
            answering(request)
        }
    };
    let baseline = Listener::answering(answer).expect("baseline listener");
    let ordinary = decide(
        baseline.base(),
        &[
            "--batch",
            "5",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--no-cache",
        ],
        &[],
        &lines(1..=5),
    );
    assert_eq!(ordinary.status.code(), Some(0));
    let bodies = baseline.requests();
    assert_eq!(
        bodies
            .iter()
            .map(|r| places(&r.body).len())
            .collect::<Vec<_>>(),
        [5, 3, 2]
    );
    let parent = bodies.first().expect("refused parent").body.len() as u64;
    let charge = (parent * 908).div_ceil(1000);

    let limited = Listener::answering(answer).expect("limited listener");
    let limit = charge.to_string();
    let stopped = decide(
        limited.base(),
        &[
            "--batch",
            "5",
            "--jobs",
            "1",
            "--max-retries",
            "0",
            "--no-cache",
            "--max-estimated-input-tokens-total",
            &limit,
        ],
        &[],
        &lines(1..=5),
    );
    assert_eq!(stopped.status.code(), Some(2), "{}", text(&stopped.stderr));
    assert!(text(&stopped.stderr).contains(&format!(
        "max_estimated_input_tokens_total={limit} (encoded-body-bytes-908-v1) would be exceeded before another request in this call"
    )));
    assert_eq!(limited.count(), 1, "the denied split child never arrived");
    assert_eq!(
        limited.requests().first().expect("parent body").body.len() as u64,
        parent
    );
}
