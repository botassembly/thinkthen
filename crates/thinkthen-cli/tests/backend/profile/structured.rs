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

/// Send the structured-profile cases with their shared transport and key.
fn send(
    arguments: &[&str],
    profile: &Path,
    listener: &Listener,
    evidence: &[u8],
) -> std::process::Output {
    spawn(
        &[
            arguments,
            &[
                "--profile",
                &profile.to_string_lossy(),
                "--url",
                listener.base(),
            ],
        ]
        .concat(),
        &[("THINKTHEN_API_KEY", "secret-key")],
        evidence,
    )
    .expect("command")
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
        crate::harness::exits(&planned, 0);
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

    let listener = Listener::answering(|_| Canned::ok(STRUCTURED_TAG_ANSWERS)).expect("listener");
    let sent = send(&base, &exact, &listener, b"Refund me please.");
    crate::harness::exits(&sent, 0);
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&requests[0].body),
        STRUCTURED_TAG_BODY
    );

    let split_listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let split = send(&base, &under, &split_listener, b"Refund me please.");
    crate::harness::exits(&split, 0);
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
        crate::harness::printed(&split),
        "[\"billing\",\"urgent\"]\n"
    );
}

/// A tag's labels count as wire questions: a one-question profile sends each
/// label alone. A limit that one question passes alone sends nothing.
#[test]
fn expanded_tags_count_as_wire_questions_and_over_limit_sends_nothing() {
    let one = profile("one-question", r#""max_questions":1"#);
    let tiny = profile("tiny-request", r#""max_request_bytes":10"#);
    let arguments = ["tag", "Which topics?", "billing", "urgent"];
    let listener = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = send(&arguments, &one, &listener, b"private evidence");
    crate::harness::exits(&output, 0);
    assert_eq!(listener.requests().len(), 2);

    let refused = Listener::answering(|_| Canned::ok(ANSWER)).expect("listener");
    let output = send(&arguments, &tiny, &refused, b"private evidence");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(refused.connections(), 0);
    let said = crate::harness::said(&output);
    assert!(
        said.starts_with(
            "thinkthen: profile tiny-request allows at most 10 request bytes; this request has "
        ),
        "{said}"
    );
    assert!(!said.contains("private"));
    assert!(!said.contains("secret-key"));
}
