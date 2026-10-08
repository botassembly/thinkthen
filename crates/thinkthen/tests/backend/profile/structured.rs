//! Structured tag descriptions and expanded tags against a profile's limits.

use super::*;

const STRUCTURED_TAG_BODY: &str = concat!(
    r#"{"state":"Refund me please.","model":"local-1","questions":{"q1":{"type":"noul","#,
    r#""instructions":[["Which topics?"],{"label":"billing"}]},"#,
    r#""q2":{"type":"noul","instructions":[["Which topics?"],{"label":"urgent"}]}}}"#,
);
const STRUCTURED_TAG_ANSWERS: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92},"#,
    r#""q2":{"type":"noul","noul":0.08}},"#,
    r#""usage":{"input_tokens":3,"output_tokens":1}}"#,
);

fn structured_tag(prefix: &str) -> (String, PathBuf, PathBuf) {
    let question = file(
        &format!("{prefix}structured-tag"),
        r#"{"tag":["Which topics?"],"labels":["billing","urgent"]}"#,
    );
    let limited =
        |name: &str, bytes: usize| profile(name, &format!(r#""max_request_bytes":{bytes}"#));
    let [edge, under] = ["edge", "under"].map(|end| format!("{prefix}{end}-structured"));
    let exact = limited(&edge, STRUCTURED_TAG_BODY.len());
    let under = limited(&under, STRUCTURED_TAG_BODY.len() - 1);
    (format!("@{}", question.to_string_lossy()), exact, under)
}

/// Preview bytes retain packing at the edge and splitting one byte under.
/// Both plans bound requests by the two admitted wire questions.
#[test]
fn a_structured_dry_run_counts_its_complete_body_at_the_edge() {
    let (question, exact, under) = structured_tag("plan-");
    let base = ["tag", question.as_str(), "--model", "local-1"];
    let preview = |profile: &Path| {
        let planned = spawn(
            &[
                &base[..],
                &["--plan", "--profile", &profile.to_string_lossy()],
            ]
            .concat(),
            &[],
            b"Refund me please.",
        )
        .expect("command");
        assert_eq!(
            planned.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&planned.stderr)
        );
        let mut lines = planned.stdout.split(|byte| *byte == b'\n');
        let document: serde_json::Value =
            serde_json::from_slice(lines.next().expect("a preview line")).expect("preview");
        let counts: serde_json::Value =
            serde_json::from_slice(lines.next().expect("a counts line")).expect("counts");
        (document["request"].clone(), counts)
    };
    let (packed, packed_counts) = preview(&exact);
    assert_eq!(
        packed,
        serde_json::from_str::<serde_json::Value>(STRUCTURED_TAG_BODY).expect("independent body")
    );
    assert_eq!(
        packed_counts,
        serde_json::json!({"records":1,"requests":2,"estimated_bytes":212,
            "largest_request_bytes":212,"largest_request_estimated_input_tokens":193,
            "token_estimate_method":"encoded-body-bytes-908-v1",
            "estimated_input_tokens":{"lower":109,"upper":193},"upper_bound":true})
    );
    let (split, split_counts) = preview(&under);
    assert_eq!(
        split,
        serde_json::json!({"state":"Refund me please.","model":"local-1",
            "questions":{"q1":{"type":"noul",
                "instructions":[["Which topics?"],{"label":"billing"}]}}})
    );
    assert_eq!(
        split_counts,
        serde_json::json!({"records":1,"requests":2,"estimated_bytes":273,
            "largest_request_bytes":137,"largest_request_estimated_input_tokens":125,
            "token_estimate_method":"encoded-body-bytes-908-v1",
            "estimated_input_tokens":{"lower":140,"upper":248},"upper_bound":false})
    );
}

#[test]
fn a_structured_request_counts_its_complete_body_at_the_edge() {
    let (question, exact, under) = structured_tag("");
    let base = ["tag", question.as_str(), "--model", "local-1"];
    let send = |profile: &Path, listener: &Listener| {
        spawn(
            &[
                &base[..],
                &[
                    "--profile",
                    &profile.to_string_lossy(),
                    "--url",
                    listener.base(),
                ],
            ]
            .concat(),
            &[("THINKTHEN_API_KEY", "secret-key")],
            b"Refund me please.",
        )
        .expect("command")
    };

    let listener = Listener::answering(|_| Canned::ok(STRUCTURED_TAG_ANSWERS)).expect("listener");
    let sent = send(&exact, &listener);
    assert_eq!(
        sent.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&sent.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&requests[0].body),
        STRUCTURED_TAG_BODY
    );

    let split_listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let split = send(&under, &split_listener);
    assert_eq!(
        split.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&split.stderr)
    );
    let bodies: Vec<serde_json::Value> = split_listener
        .requests()
        .iter()
        .map(|request| serde_json::from_slice(&request.body).expect("a body"))
        .collect();
    assert_eq!(bodies.len(), 2);
    for body in &bodies {
        assert_eq!(body["state"], "Refund me please.");
        assert_eq!(
            body["questions"].as_object().map(serde_json::Map::len),
            Some(1)
        );
    }
    assert_eq!(
        String::from_utf8_lossy(&split.stdout),
        "[\"billing\",\"urgent\"]\n"
    );
}

/// A tag's labels count as wire questions: a one-question profile sends each
/// label alone. A limit that one question passes alone sends nothing.
#[test]
fn expanded_tags_count_as_wire_questions_and_over_limit_sends_nothing() {
    let one = profile("one-question", r#""max_questions":1"#);
    let tiny = profile("tiny-request", r#""max_request_bytes":10"#);
    let run = |profile: &Path, listener: &Listener| {
        spawn(
            &[
                "tag",
                "Which topics?",
                "billing",
                "urgent",
                "--profile",
                &profile.to_string_lossy(),
                "--url",
                listener.base(),
            ],
            &[("THINKTHEN_API_KEY", "secret-key")],
            b"private evidence",
        )
        .expect("command")
    };
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = run(&one, &listener);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.requests().len(), 2);

    let refused = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = run(&tiny, &refused);
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(refused.connections(), 0);
    let said = String::from_utf8_lossy(&output.stderr);
    assert!(
        said.starts_with(
            "thinkthen: profile tiny-request allows at most 10 request bytes; this request has "
        ),
        "{said}"
    );
    assert!(!said.contains("private"));
    assert!(!said.contains("secret-key"));
}
