//! A context is shared evidence on the wire and named separately in each row.

use std::fs;

use serde_json::{Value, json};

use super::{KEY, details, folder, text};
use crate::harness::{Canned, Listener, spawn};

#[test]
fn a_context_rides_the_batch_and_names_its_raw_bytes() {
    let place = folder("context-wire");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/catalog.txt");
    fs::write(
        &file,
        b"Come Together appears on Abbey Road.\nBecause appears on Abbey Road.\n",
    )
    .expect("context file");
    let listener = Listener::answering(super::answering).expect("listener");
    let output = spawn(
        &[
            "decide",
            "It appears on the album Abbey Road.",
            "--lines",
            "--details",
            "--no-cache",
            "--model",
            "jev-latest",
            "--context",
            &file,
            "--url",
            listener.base(),
        ],
        &[KEY],
        b"Come Together\nHelp!\n",
    )
    .expect("command");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert!(output.stderr.is_empty());
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let expected = include_bytes!(
        "../../../../../specification/fixtures/systemone/batch-context.request.json"
    );
    assert_eq!(sent[0].body, expected[..expected.len() - 1]);
    let rows = details(&output);
    assert_eq!(rows.len(), 2);
    for row in rows {
        assert_eq!(
            row["meta"]["context_sha256"],
            "d3c9ec5006fd22bc389a26bbe49c06abbde5b1c8bd459cf4430f21b264aa2730"
        );
        assert_eq!(row["meta"]["batch"]["records"], 2);
    }
}

#[test]
fn a_context_is_in_batch_one_and_the_dry_run_plan() {
    let place = folder("context-branches");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/context.txt");
    fs::write(&file, b"Shared evidence\n").expect("context file");
    let listener = Listener::answering(super::answering).expect("listener");
    let base = ["decide", super::QUESTION, "--lines", "--context", &file];
    let at_one = spawn(
        &[
            &base[..],
            &[
                "--batch",
                "1",
                "--details",
                "--no-cache",
                "--url",
                listener.base(),
            ],
        ]
        .concat(),
        &[KEY],
        b"line 1\nline 2\n",
    )
    .expect("batch one");
    assert_eq!(at_one.status.code(), Some(0), "{}", text(&at_one.stderr));
    let sent = listener.requests();
    assert_eq!(sent.len(), 2);
    for request in &sent {
        let body: Value = serde_json::from_slice(&request.body).expect("request body");
        assert_eq!(body["state"], "Shared evidence\n");
        assert_eq!(
            body["questions"]
                .as_object()
                .map_or(0, serde_json::Map::len),
            1
        );
    }
    let rows = details(&at_one);
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row["meta"]["batch"]["setting"] == 1));

    let planned = spawn(
        &[&base[..], &["--dry-run", "--url", listener.base()]].concat(),
        &[],
        b"line 1\nline 2\n",
    )
    .expect("plan");
    assert_eq!(planned.status.code(), Some(0), "{}", text(&planned.stderr));
    let plan: Value = serde_json::from_slice(&planned.stdout).expect("plan JSON");
    assert_eq!(plan["request"]["state"], "Shared evidence\n");
    assert_eq!(listener.count(), 2, "dry-run sent no new request");
}

#[test]
fn filter_and_rank_plan_the_same_shared_context() {
    let place = folder("context-other-verbs");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/context.txt");
    fs::write(&file, b"Shared evidence\n").expect("context file");
    for verb in ["filter", "rank"] {
        let output = spawn(
            &[
                verb,
                super::QUESTION,
                "--lines",
                "--context",
                &file,
                "--dry-run",
            ],
            &[],
            b"line 1\nline 2\n",
        )
        .expect("plan");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{verb}: {}",
            text(&output.stderr)
        );
        let plan: Value = serde_json::from_slice(&output.stdout).expect("plan JSON");
        assert_eq!(plan["request"]["state"], "Shared evidence\n", "{verb}");
    }
}

#[test]
fn a_refused_context_batch_keeps_the_context_in_both_halves() {
    let place = folder("context-halves");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/context.txt");
    fs::write(&file, b"Shared evidence\n").expect("context file");
    let base = ["decide", super::QUESTION, "--lines", "--context", &file];
    let split = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        if request["questions"]
            .as_object()
            .map_or(0, serde_json::Map::len)
            == 4
        {
            Canned::status(413, "{}")
        } else {
            super::answering(body)
        }
    })
    .expect("split listener");
    let output = spawn(
        &[
            &base[..],
            &[
                "--batch",
                "4",
                "--details",
                "--max-retries",
                "0",
                "--no-cache",
                "--url",
                split.base(),
            ],
        ]
        .concat(),
        &[KEY],
        super::lines(1..=4).as_bytes(),
    )
    .expect("split run");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let sent = split.requests();
    assert_eq!(sent.len(), 3);
    let sizes: Vec<_> = sent
        .iter()
        .map(|request| {
            let body: Value = serde_json::from_slice(&request.body).expect("request");
            assert_eq!(body["state"], "Shared evidence\n");
            body["questions"]
                .as_object()
                .map_or(0, serde_json::Map::len)
        })
        .collect();
    assert_eq!(sizes, [4, 2, 2]);
    let rows = details(&output);
    assert_eq!(rows.len(), 4);
    assert!(
        rows.iter()
            .all(|row| row["meta"]["batch"]["split"] == json!(true))
    );
}

#[test]
fn a_late_context_overflow_keeps_earlier_rows_and_sends_no_third_record() {
    let place = folder("context-late-limit");
    fs::create_dir_all(&place).expect("scratch folder");
    let file = format!("{place}/context.txt");
    fs::write(&file, b"Shared evidence\n").expect("context file");
    let listener = Listener::answering(super::answering).expect("listener");
    let long = format!("record 3 marker {}", "x".repeat(600));
    let input = format!("line 1\nline 2\n{long}\n");
    let output = spawn(
        &[
            "decide",
            super::QUESTION,
            "--lines",
            "--context",
            &file,
            "--max-request-bytes",
            "500",
            "--jobs",
            "1",
            "--no-cache",
            "--url",
            listener.base(),
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("context run");
    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    let sent = listener.requests();
    assert_eq!(sent.len(), 1, "earlier records sent together");
    let body: Value = serde_json::from_slice(&sent[0].body).expect("request body");
    assert_eq!(
        body["questions"]
            .as_object()
            .map_or(0, serde_json::Map::len),
        2
    );
    assert_eq!(body["state"], "Shared evidence\n");
    assert_eq!(text(&output.stdout), super::rows(1..=2));
    assert!(!text(&output.stderr).contains("record 3 marker"));
    assert_eq!(
        text(&output.stderr),
        "thinkthen: --context: this record and the context make a request of 754 bytes, over the request size of 500 bytes; raise --max-request-bytes, or shorten the context or the record\nthinkthen: stopped at record 3; 2 records finished\n"
    );
}

#[test]
fn a_context_refuses_invalid_file_and_question_shapes_before_a_send() {
    let place = folder("context-input-refusals");
    fs::create_dir_all(&place).expect("scratch folder");
    let valid = format!("{place}/valid.txt");
    let empty = format!("{place}/empty.txt");
    let invalid = format!("{place}/invalid.txt");
    let missing = format!("{place}/missing.txt");
    let structured = format!("{place}/structured.json");
    fs::write(&valid, b"Shared evidence\n").expect("valid context");
    fs::write(&empty, b" \n").expect("empty context");
    fs::write(&invalid, b"\xff").expect("invalid context");
    fs::write(&structured, r#"{"decide":{"rule":"names a place"}}"#).expect("structured question");
    let question = format!("@{structured}");
    let cases = [
        (
            "document",
            super::QUESTION,
            valid.as_str(),
            false,
            2,
            "thinkthen: --context shares one text across the records of a stream, and a single text is one record\n",
        ),
        (
            "missing",
            super::QUESTION,
            missing.as_str(),
            true,
            5,
            "thinkthen: --context could not be opened: No such file or directory (os error 2)\n",
        ),
        (
            "utf8",
            super::QUESTION,
            invalid.as_str(),
            true,
            5,
            "thinkthen: --context is not UTF-8 text\n",
        ),
        (
            "empty",
            super::QUESTION,
            empty.as_str(),
            true,
            2,
            "thinkthen: --context names an empty file\n",
        ),
        (
            "json question",
            question.as_str(),
            valid.as_str(),
            true,
            2,
            "thinkthen: --context needs a question written as text; a question written as JSON cannot quote a record\n",
        ),
    ];
    for (name, question, context, streams, code, stderr) in cases {
        let listener = Listener::answering(super::answering).expect("listener");
        let mut arguments = vec![
            "decide",
            question,
            "--context",
            context,
            "--url",
            listener.base(),
        ];
        if streams {
            arguments.push("--lines");
        }
        let output = spawn(&arguments, &[KEY], b"line 1\n").expect("command");
        assert_eq!(output.status.code(), Some(code), "{name}");
        assert!(output.stdout.is_empty(), "{name}");
        assert_eq!(text(&output.stderr), stderr, "{name}");
        assert_eq!(listener.count(), 0, "{name}");
    }
}

#[test]
fn a_context_size_refusal_names_the_binding_limit_and_never_sends_that_record() {
    let place = folder("context-size-refusals");
    fs::create_dir_all(&place).expect("scratch folder");
    let context = format!("{place}/context.txt");
    let profile = format!("{place}/profile.json");
    let small_context = format!("{place}/small-context.json");
    let small_request = format!("{place}/small-request.json");
    fs::write(&context, b"Shared evidence\n").expect("context file");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"small","max_request_bytes":500}"#,
    )
    .expect("profile");
    fs::write(
        &small_context,
        r#"{"schema":"thinkthen.backend-profile/1","name":"context","max_evidence_bytes":10}"#,
    )
    .expect("context profile");
    fs::write(
        &small_request,
        r#"{"schema":"thinkthen.backend-profile/1","name":"request","max_request_bytes":50}"#,
    )
    .expect("request profile");
    let long = format!("record 1 marker {}\n", "x".repeat(600));
    let cases = [
        (
            "question only",
            b"line 1\n".as_slice(),
            vec!["--max-request-bytes", "50"],
            "thinkthen: --context: the context and the question make a request of 65 bytes, over the request size of 50 bytes; raise --max-request-bytes or shorten the context\n",
        ),
        (
            "record",
            long.as_bytes(),
            vec!["--max-request-bytes", "500"],
            "thinkthen: --context: this record and the context make a request of 754 bytes, over the request size of 500 bytes; raise --max-request-bytes, or shorten the context or the record\nthinkthen: stopped at record 1; 0 records finished\n",
        ),
        (
            "profile",
            long.as_bytes(),
            vec!["--max-request-bytes", "96000", "--profile", &profile],
            "thinkthen: --context: profile small allows at most 500 request bytes; this record and the context make 754\nthinkthen: stopped at record 1; 0 records finished\n",
        ),
        (
            "context profile",
            b"line 1\n".as_slice(),
            vec!["--profile", &small_context],
            "thinkthen: --context: profile context allows at most 10 evidence bytes; the context's request has 16\n",
        ),
        (
            "request profile",
            b"line 1\n".as_slice(),
            vec!["--profile", &small_request],
            "thinkthen: --context: profile request allows at most 50 request bytes; the context's request has 65\n",
        ),
    ];
    for (name, input, extra, stderr) in cases {
        let listener = Listener::answering(super::answering).expect("listener");
        let fixed = [
            "decide",
            super::QUESTION,
            "--lines",
            "--context",
            &context,
            "--no-cache",
            "--url",
            listener.base(),
        ];
        let output = spawn(&[&fixed[..], &extra].concat(), &[KEY], input).expect("command");
        assert_eq!(
            output.status.code(),
            Some(2),
            "{name}: {}",
            text(&output.stderr)
        );
        assert!(output.stdout.is_empty(), "{name}");
        assert_eq!(listener.count(), 0, "{name}");
        assert!(!text(&output.stderr).contains("record 1 marker"), "{name}");
        assert_eq!(text(&output.stderr), stderr, "{name}");
    }
}

#[test]
fn a_changed_context_keeps_the_question_digest_but_misses_the_recording() {
    let place = folder("context-digest");
    fs::create_dir_all(&place).expect("scratch folder");
    let first = format!("{place}/first.txt");
    let second = format!("{place}/second.txt");
    let recording = format!("{place}/recording");
    fs::write(&first, b"First shared evidence\n").expect("first context");
    fs::write(&second, b"Second shared evidence\n").expect("second context");
    let listener = Listener::answering(super::answering).expect("listener");
    let fixed = [
        "decide",
        super::QUESTION,
        "--lines",
        "--details",
        "--url",
        listener.base(),
    ];
    let input = b"line 1\nline 2\n";
    let recorded = spawn(
        &[&fixed[..], &["--context", &first, "--record", &recording]].concat(),
        &[KEY],
        input,
    )
    .expect("recorded run");
    assert_eq!(
        recorded.status.code(),
        Some(0),
        "{}",
        text(&recorded.stderr)
    );
    let changed = spawn(
        &[&fixed[..], &["--context", &second, "--no-cache"]].concat(),
        &[KEY],
        input,
    )
    .expect("changed context");
    assert_eq!(changed.status.code(), Some(0), "{}", text(&changed.stderr));
    let (recorded, changed) = (details(&recorded), details(&changed));
    assert_eq!((recorded.len(), changed.len()), (2, 2));
    assert_eq!(
        recorded[0]["meta"]["question_sha256"],
        changed[0]["meta"]["question_sha256"]
    );
    assert_ne!(
        recorded[0]["meta"]["context_sha256"],
        changed[0]["meta"]["context_sha256"]
    );
    assert_ne!(
        recorded[0]["meta"]["requests"],
        changed[0]["meta"]["requests"]
    );
    let sent_before_replay = listener.count();
    assert_eq!(sent_before_replay, 2);
    let replay = spawn(
        &[&fixed[..], &["--context", &second, "--replay", &recording]].concat(),
        &[],
        input,
    )
    .expect("replay under another context");
    assert_eq!(replay.status.code(), Some(5), "{}", text(&replay.stderr));
    assert!(replay.stdout.is_empty());
    assert_eq!(
        listener.count(),
        sent_before_replay,
        "a replay miss sends nothing"
    );
}
