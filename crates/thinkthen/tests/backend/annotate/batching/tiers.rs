//! Top-level annotate batch tiers and the set identity.

use super::{Canned, Listener, Value, json, set, spawn};

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "the tier case checks each source and its no-send refusal in one listener"
)]
fn top_level_batch_uses_flag_environment_and_file_tiers() {
    let file = set(
        "batch-tier-set",
        r#"{"version":1,"batch":2,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let count = request["questions"]
            .as_object()
            .map_or(0, serde_json::Map::len);
        let answers = (1..=count)
            .map(|at| (format!("q{at}"), json!({"type":"noul","noul":0.9})))
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let run = |extra: &[&str], environment: &[(&str, &str)]| {
        let base = [
            "annotate",
            &file.to_string_lossy(),
            "--lines",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ];
        spawn(
            &[&base[..], extra].concat(),
            environment,
            b"one\ntwo\nthree\n",
        )
        .expect("annotate stream")
    };
    let file_result = run(&[], &[("THINKTHEN_API_KEY", "key")]);
    let environment_result = run(
        &[],
        &[("THINKTHEN_API_KEY", "key"), ("THINKTHEN_BATCH", "1")],
    );
    let flag_result = run(
        &["--batch", "3"],
        &[("THINKTHEN_API_KEY", "key"), ("THINKTHEN_BATCH", "1")],
    );
    for result in [&file_result, &environment_result, &flag_result] {
        assert_eq!(
            result.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout.iter().filter(|&&byte| byte == b'\n').count(),
            3
        );
    }
    assert_eq!(listener.requests().len(), 2 + 3 + 1);

    let invalid = set(
        "batch-tier-invalid",
        r#"{"version":1,"batch":0,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let refused = spawn(
        &[
            "annotate",
            &invalid.to_string_lossy(),
            "--lines",
            "--url",
            listener.base(),
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\n",
    )
    .expect("invalid batch");
    assert_eq!(refused.status.code(), Some(5));
    assert_eq!(listener.requests().len(), 0);
    let overruled = spawn(
        &[
            "annotate",
            &invalid.to_string_lossy(),
            "--lines",
            "--batch",
            "1",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\n",
    )
    .expect("flag overrides file");
    assert_eq!(overruled.status.code(), Some(0));
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn top_level_batch_does_not_change_the_resolved_set_digest() {
    let plain = set(
        "batch-digest-plain",
        r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let saved = set(
        "batch-digest-saved",
        r#"{"version":1,"batch":2,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    let run = |file: &std::path::Path| {
        spawn(
            &[
                "annotate",
                &file.to_string_lossy(),
                "--lines",
                "--batch",
                "1",
                "--details",
                "--no-cache",
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &[("THINKTHEN_API_KEY", "key")],
            b"one\n",
        )
        .expect("annotate stream")
    };
    let before = run(&plain);
    let after = run(&saved);
    assert_eq!(before.status.code(), Some(0));
    assert_eq!(after.status.code(), Some(0));
    let before: Value = serde_json::from_slice(&before.stdout).expect("plain row");
    let after: Value = serde_json::from_slice(&after.stdout).expect("saved row");
    assert_eq!(
        before["meta"]["questions_sha256"],
        after["meta"]["questions_sha256"]
    );
    assert_eq!(before["meta"]["requests"], after["meta"]["requests"]);
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, requests[1].body);
}

#[test]
fn request_size_flag_overrides_the_environment_close_limit() {
    let file = set(
        "batch-request-size-tier",
        r#"{"version":1,"questions":{"ready":{"decide":"Ready?"}}}"#,
    );
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request JSON");
        let count = request["questions"]
            .as_object()
            .map_or(0, serde_json::Map::len);
        let answers = (1..=count)
            .map(|at| (format!("q{at}"), json!({"type":"noul","noul":0.9})))
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let base = [
        "annotate",
        &file.to_string_lossy(),
        "--lines",
        "--batch",
        "3",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    let env = [
        ("THINKTHEN_API_KEY", "key"),
        ("THINKTHEN_MAX_REQUEST_BYTES", "1"),
    ];
    let tight = spawn(&base, &env, b"one\ntwo\nthree\n").expect("tight request size");
    assert_eq!(
        tight.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&tight.stderr)
    );
    assert_eq!(listener.requests().len(), 3);
    let override_size = spawn(
        &[&base[..], &["--max-request-bytes", "96000"]].concat(),
        &env,
        b"one\ntwo\nthree\n",
    )
    .expect("flag request size");
    assert_eq!(
        override_size.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&override_size.stderr)
    );
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn annotate_plan_bounds_wire_occurrences_and_keeps_packed_group_counts() {
    let file = set(
        "repeated-plan-groups",
        r#"{"version":1,"questions":{"left_answer":{"decide":"Left?","on":"/left"},"right_answer":{"decide":"Right?","on":"/right"}}}"#,
    );
    let listener =
        Listener::answering(|_| Canned::status(500, "should not send")).expect("listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--batch",
            "2",
            "--plan",
            "--url",
            listener.base(),
        ],
        &[],
        b"{\"left\":\"same\",\"right\":\"same\"}\n{\"left\":\"same\",\"right\":\"same\"}\n",
    )
    .expect("annotate plan");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let lines: Vec<Value> = String::from_utf8(output.stdout)
        .expect("UTF-8")
        .lines()
        .map(|line| serde_json::from_str(line).expect("plan JSON"))
        .collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0]["request_count"], 1);
    assert_eq!(lines[0]["group_requests"], json!([1, 1]));
    assert_eq!(
        lines[0]["request"]["questions"]
            .as_object()
            .expect("questions")
            .len(),
        4
    );
    assert_eq!(lines[1]["records"], 2);
    assert_eq!(lines[1]["requests"], 4);
    assert_eq!(lines[1]["upper_bound"], true);
    assert_eq!(listener.count(), 0);
}
