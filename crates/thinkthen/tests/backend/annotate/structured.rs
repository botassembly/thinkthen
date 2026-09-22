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
    let common = |profile: &PathBuf, listener: &Listener| {
        vec![
            "annotate".to_owned(),
            set.to_string_lossy().into_owned(),
            "--profile".to_owned(),
            profile.to_string_lossy().into_owned(),
            "--url".to_owned(),
            listener.base().to_owned(),
            "--model".to_owned(),
            "local-1".to_owned(),
        ]
    };

    let exact = profile("annotate-exact", body.len());
    let listener = Listener::serving(vec![Canned::ok(answer)]).expect("a listener");
    let sent = spawn(
        &common(&exact, &listener)
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"body":"The invoice failed."}"#,
    )
    .expect("the command runs");
    assert_eq!(
        sent.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&sent.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(String::from_utf8_lossy(&requests[0].body), body);

    let under = profile("annotate-under", body.len() - 1);
    let refused_listener = Listener::serving(vec![]).expect("a listener");
    let refused = spawn(
        &common(&under, &refused_listener)
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        br#"{"body":"The invoice failed."}"#,
    )
    .expect("the command runs");
    assert_eq!(refused.status.code(), Some(2));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        format!(
            "thinkthen: profile annotate-under allows at most {} request bytes; this request has {}\n",
            body.len() - 1,
            body.len()
        )
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
    let listener = Listener::serving(vec![Canned::ok(&yes("0.9")), Canned::ok(&yes("0.1"))])
        .expect("a listener");
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
    assert_eq!(bodies.len(), 2);
    assert!(bodies[0].contains(r#""state":{"a":1}"#), "{}", bodies[0]);
    assert!(
        bodies[1].contains(r#""state":"The invoice failed.""#),
        "{}",
        bodies[1]
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
        body.contains(r#""state":{"body":"The invoice failed.","meta":{"a":1}}"#),
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
