//! Structured question-set entries through the compiled `annotate` command.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup should stop the boundary test"
)]

use std::fs;
use std::path::PathBuf;

use crate::harness::{Canned, Listener, spawn};

fn questions(name: &str, text: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("annotate-structured");
    let _created = fs::create_dir_all(&folder);
    let path = folder.join(format!("{name}.json"));
    let _written = fs::write(&path, text);
    path
}

#[test]
fn annotate_carries_a_structured_tag_entry_beside_a_plain_one_and_answers_both() {
    let answer = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.81},"#,
        r#""q2":{"type":"noul","noul":0.92},"#,
        r#""q3":{"type":"noul","noul":0.12}}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(answer)]).expect("a listener");
    let file = questions(
        "structured-tag",
        concat!(
            r#"{"version":1,"questions":{"risky":{"decide":"Is this risky?"},"#,
            r#""topics":{"tag":["Which topics?"],"#,
            r#""labels":{"billing":{"what":"Money and invoices."},"urgent":null}}}}"#,
        ),
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"body":"The invoice failed."}"#,
    )
    .expect("the command runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"body\":\"The invoice failed.\",\"risky\":true,\"topics\":[\"billing\"]}\n"
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body = String::from_utf8_lossy(&requests[0].body);
    assert!(
        body.contains(r#""q1":{"type":"noul","instructions":"Is this risky?"}"#),
        "{body}"
    );
    assert!(
        body.contains(
            r#""instructions":[["Which topics?"],{"label":"billing","description":{"what":"Money and invoices."}}]"#
        ),
        "{body}"
    );
    assert!(
        body.contains(r#""criteria":{"true":{"what":"Money and invoices."}}"#),
        "{body}"
    );
    assert!(
        body.contains(r#""instructions":[["Which topics?"],{"label":"urgent"}]"#),
        "{body}"
    );
}

#[test]
fn a_structured_annotate_counts_the_whole_body_and_over_limit_sends_nothing() {
    let body = concat!(
        r#"{"state":"{\"body\":\"The invoice failed.\"}","model":"local-1","questions":{"#,
        r#""q1":{"type":"noul","instructions":[["Which topics?"],{"label":"billing","description":{"what":"Money"}}],"criteria":{"true":{"what":"Money"}}},"#,
        r#""q2":{"type":"noul","instructions":[["Which topics?"],{"label":"urgent"}]}}}"#,
    );
    let answer = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92},"#,
        r#""q2":{"type":"noul","noul":0.12}}}"#,
    );
    let set = questions(
        "structured-limit",
        concat!(
            r#"{"version":1,"questions":{"topics":{"tag":["Which topics?"],"#,
            r#""labels":{"billing":{"what":"Money"},"urgent":null}}}}"#,
        ),
    );
    let profile = |name: &str, limit: usize| {
        questions(
            name,
            &format!(
                r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}","max_request_bytes":{limit}}}"#
            ),
        )
    };
    let run = |profile: &PathBuf, listener: &Listener| {
        let (set, profile) = (set.to_string_lossy(), profile.to_string_lossy());
        spawn(
            &[
                "annotate",
                &set,
                "--profile",
                &profile,
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            br#"{"body":"The invoice failed."}"#,
        )
        .expect("the command runs")
    };

    let exact = profile("annotate-exact", body.len());
    let listener = Listener::serving(vec![Canned::ok(answer)]).expect("a listener");
    let sent = run(&exact, &listener);
    assert_eq!(
        sent.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&sent.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(String::from_utf8_lossy(&requests[0].body), body);

    // One byte under, each label goes in its own request with the state
    // repeated, by ADR 0111 section 4.
    let under = profile("annotate-under", body.len() - 1);
    let one = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.92}}}"#;
    let split_listener = Listener::answering(move |_| Canned::ok(one)).expect("a listener");
    let split = run(&under, &split_listener);
    assert_eq!(
        split.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&split.stderr)
    );
    let split_bodies = split_listener.requests();
    assert_eq!(split_bodies.len(), 2);
    for request in &split_bodies {
        assert!(
            String::from_utf8_lossy(&request.body)
                .starts_with(r#"{"state":"{\"body\":\"The invoice failed.\"}","#)
        );
    }

    // A limit one question passes alone sends nothing.
    let tiny = profile("annotate-tiny", 50);
    let refused_listener = Listener::serving(vec![]).expect("a listener");
    let refused = run(&tiny, &refused_listener);
    assert_eq!(refused.status.code(), Some(2));
    let said = String::from_utf8_lossy(&refused.stderr);
    assert!(
        said.starts_with(
            "thinkthen: profile annotate-tiny allows at most 50 request bytes; this request has "
        ),
        "{said}"
    );
    assert!(refused_listener.requests().is_empty());
}

/// A one-question yes/no answer with the named share.
fn yes(noul: &str) -> String {
    format!(
        r#"{{"model":"local-1","answers":{{"q1":{{"type":"noul","noul":{noul}}}}},"usage":{{"input_tokens":3,"output_tokens":1}}}}"#
    )
}

#[test]
fn a_per_question_on_sends_the_json_value_the_pointer_names() {
    let both = concat!(
        r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"#,
        r#""q2":{"type":"noul","noul":0.1}}}"#,
    );
    let listener = Listener::serving(vec![Canned::ok(both)]).expect("a listener");
    let file = questions(
        "on-selects-json",
        concat!(
            r#"{"version":1,"questions":{"is_meta":{"decide":"Meta?","on":"/meta"},"#,
            r#""is_body":{"decide":"Body?","on":"/body"}}}"#,
        ),
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jobs",
            "1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"meta":{"a":1},"body":"The invoice failed."}"#,
    )
    .expect("the command runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        r#"{"meta":{"a":1},"body":"The invoice failed.","is_meta":true,"is_body":false}"#
            .to_owned()
            + "\n"
    );
    let bodies: Vec<String> = listener
        .requests()
        .iter()
        .map(|request| String::from_utf8_lossy(&request.body).into_owned())
        .collect();
    // Both questions share the fixed state, so they ride one request.
    assert_eq!(bodies.len(), 1);
    assert!(
        bodies[0].contains(r#""instructions":"The text is {\"a\":1}. Meta?""#),
        "{}",
        bodies[0]
    );
    assert!(
        bodies[0].contains(r#""instructions":"The text is \"The invoice failed.\". "#),
        "{}",
        bodies[0]
    );
}

#[test]
fn several_command_fields_and_a_root_on_send_the_object_the_fields_built() {
    let listener = Listener::serving(vec![Canned::ok(&yes("0.9"))]).expect("a listener");
    let file = questions(
        "fields-root",
        r#"{"version":1,"questions":{"risky":{"decide":"Is this risky?"}}}"#,
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
            "--jsonl",
            "--field",
            "/body",
            "--field",
            "/meta",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"meta":{"a":1},"body":"The invoice failed.","extra":"kept"}"#,
    )
    .expect("the command runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body = String::from_utf8_lossy(&requests[0].body);
    assert!(
        body.contains(concat!(
            r#""instructions":"The text is {\"body\":\"The invoice failed.\","#,
            r#"\"meta\":{\"a\":1}}. Is this risky?""#
        )),
        "{body}"
    );
}

#[test]
fn an_invalid_nested_description_fails_the_set_before_any_request() {
    let listener = Listener::serving(Vec::new()).expect("a listener");
    let file = questions(
        "bad-nested-description",
        concat!(
            r#"{"version":1,"questions":{"topics":{"tag":"Which topics?","#,
            r#""labels":{"billing":true,"urgent":"Urgent."}}}}"#,
        ),
    );
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"body":"The invoice failed."}"#,
    )
    .expect("the command runs");
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: `questions.topics.labels` is a list of labels, or a map from each label to its description\n"
    );
    assert!(output.stdout.is_empty());
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
}
