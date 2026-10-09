//! The compiled `tag` command against a loopback backend.

use std::fs;
use std::path::PathBuf;

use crate::harness::{Canned, Listener, spawn};

const ANSWER: &str = concat!(
    r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.91},"#,
    r#""q2":{"type":"noul","noul":0.22}},"#,
    r#""usage":{"input_tokens":21,"output_tokens":4}}"#,
);

fn file(name: &str, text: &str) -> PathBuf {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("tag");
    let _created = fs::create_dir_all(&folder);
    let path = folder.join(format!("{name}.json"));
    let _written = fs::write(&path, text);
    path
}

#[test]
fn described_labels_make_one_exact_request_and_one_complete_result() {
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let output = spawn(
        &[
            "tag",
            "Which topics?",
            "--label",
            "billing=The item concerns a charge.",
            "--label",
            "urgent=The item needs prompt attention.",
            "--details",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The invoice failed.",
    )
    .expect("tag runs");

    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    let line = String::from_utf8_lossy(&output.stdout);
    assert!(line.contains(r#""value":["billing"]"#), "{line}");
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("tag details");
    assert_eq!(
        row["question"],
        serde_json::json!({"verb":"tag","text":"Which topics?","labels":["billing","urgent"],"label_details":[{"name":"billing","description":"The item concerns a charge."},{"name":"urgent","description":"The item needs prompt attention."}]})
    );
    assert!(
        line.contains(r#""answer":{"kind":"tag","probabilities":{"billing":0.91,"urgent":0.22}}"#),
        "{line}"
    );
    assert!(line.contains(r#""threshold":0.5"#), "{line}");
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&requests[0].body),
        concat!(
            r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"The text is \"The invoice failed.\". Which topics?\n\nDetermine whether the label \"billing\" applies to this item.","criteria":{"true":"The item concerns a charge."}},"#,
            r#""q2":{"type":"noul","instructions":"The text is \"The invoice failed.\". Which topics?\n\nDetermine whether the label \"urgent\" applies to this item.","criteria":{"true":"The item needs prompt attention."}}}}"#,
        )
    );
}

#[test]
fn an_empty_tag_list_is_a_successful_answer() {
    let answer = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.2}}}"#;
    let listener = Listener::serving(vec![Canned::ok(answer)]).expect("listener");
    let output = spawn(
        &[
            "tag",
            "Which topics?",
            "billing",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"No charge is mentioned.",
    )
    .expect("tag runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "[]\n");
}

#[test]
fn typed_labels_replace_the_question_files_whole_list() {
    let question = file(
        "replacement",
        r#"{"tag":"Which topics?","labels":{"old":"Old label."}}"#,
    );
    let output = spawn(
        &[
            "tag",
            &format!("@{}", question.to_string_lossy()),
            "new",
            "--plan",
        ],
        &[],
        b"evidence",
    )
    .expect("tag plans");
    assert_eq!(output.status.code(), Some(0));
    let plan = String::from_utf8_lossy(&output.stdout);
    assert!(plan.contains(r#"label \"new\" applies"#), "{plan}");
    assert!(!plan.contains("Old label"), "{plan}");
}

#[test]
fn invalid_tag_surfaces_stop_before_any_request() {
    let listener = Listener::serving(Vec::new()).expect("listener");
    let base = ["--url", listener.base(), "--model", "local-1"];
    let cases: Vec<Vec<&str>> = vec![
        [&["tag", "Which topics?"][..], &base].concat(),
        [&["tag", "Which topics?", "a", "a"][..], &base].concat(),
        [&["tag", "Which topics?", ""][..], &base].concat(),
        [&["tag", "Which topics?", "line\nbreak"][..], &base].concat(),
        [&["tag", "Which topics?", "a", "--label", "b=B"][..], &base].concat(),
        [
            &["tag", "Which topics?", "a", "--threshold", "0.2:0.8"][..],
            &base,
        ]
        .concat(),
        [&["tag", "Which topics?", "a", "--raw"][..], &base].concat(),
        [&["tag", "Which topics?", "a", "--quiet"][..], &base].concat(),
    ];
    for arguments in cases {
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            b"evidence",
        )
        .expect("tag refuses");
        assert_eq!(output.status.code(), Some(2), "{arguments:?}");
        assert!(output.stdout.is_empty(), "{arguments:?}");
    }
    assert!(listener.requests().is_empty());
}

#[test]
fn too_many_labels_and_a_malformed_file_stop_before_any_request() {
    let listener = Listener::serving(Vec::new()).expect("listener");
    let mut owned = vec!["tag".to_owned(), "Which topics?".to_owned()];
    owned.extend((0..21).map(|place| format!("tag{place}")));
    owned.extend([
        "--url".to_owned(),
        listener.base().to_owned(),
        "--model".to_owned(),
        "local-1".to_owned(),
    ]);
    let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
    let too_many = spawn(&borrowed, &[], b"evidence").expect("tag refuses");
    assert_eq!(too_many.status.code(), Some(2));

    let bad = file("malformed", r#"{"tag":"Which topics?","labels":7}"#);
    let malformed = spawn(
        &[
            "tag",
            &format!("@{}", bad.to_string_lossy()),
            "--url",
            listener.base(),
        ],
        &[],
        b"evidence",
    )
    .expect("tag refuses");
    assert_eq!(malformed.status.code(), Some(5));
    assert!(listener.requests().is_empty());
}

#[test]
fn tag_keeps_record_order_at_each_supported_job_count() {
    for jobs in [1, 4, 32] {
        let jobs = jobs.to_string();
        let listener = Listener::serving(vec![
            Canned::ok(ANSWER),
            Canned::ok(ANSWER),
            Canned::ok(ANSWER),
        ])
        .expect("listener");
        let output = spawn(
            &[
                "tag",
                "Which topics?",
                "billing",
                "urgent",
                "--lines",
                "--jobs",
                &jobs,
                "--batch",
                "1",
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &[("THINKTHEN_API_KEY", "sk-test-value")],
            b"first\nsecond\nthird\n",
        )
        .expect("tag runs");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            concat!(
                r#"{"input":"first","value":["billing"]}"#,
                "\n",
                r#"{"input":"second","value":["billing"]}"#,
                "\n",
                r#"{"input":"third","value":["billing"]}"#,
                "\n",
            )
        );
        assert_eq!(listener.requests().len(), 3);
    }
}

#[test]
fn a_described_tag_file_sends_the_same_bytes_as_typed_labels() {
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let question = file(
        "described",
        r#"{"tag":"Which topics?","labels":{"billing":"The item concerns a charge.","urgent":"The item needs prompt attention."}}"#,
    );
    let output = spawn(
        &[
            "tag",
            &format!("@{}", question.to_string_lossy()),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The invoice failed.",
    )
    .expect("tag runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&requests[0].body),
        concat!(
            r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"The text is \"The invoice failed.\". Which topics?\n\nDetermine whether the label \"billing\" applies to this item.","criteria":{"true":"The item concerns a charge."}},"#,
            r#""q2":{"type":"noul","instructions":"The text is \"The invoice failed.\". Which topics?\n\nDetermine whether the label \"urgent\" applies to this item.","criteria":{"true":"The item needs prompt attention."}}}}"#,
        )
    );
}

#[test]
fn one_structured_description_expands_every_label_and_sends_exact_bytes() {
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let question = file(
        "structured",
        r#"{"tag":"Which topics?","labels":{"billing":{"what":"Money and invoices."},"urgent":"The item needs prompt attention."}}"#,
    );
    let output = spawn(
        &[
            "tag",
            &format!("@{}", question.to_string_lossy()),
            "--details",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The invoice failed.",
    )
    .expect("tag runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let line = String::from_utf8_lossy(&output.stdout);
    assert!(line.contains(r#""value":["billing"]"#), "{line}");
    let row: serde_json::Value = serde_json::from_slice(&output.stdout).expect("tag details");
    assert_eq!(
        row["question"],
        serde_json::json!({"verb":"tag","text":"Which topics?","labels":["billing","urgent"],"label_details":[{"name":"billing","description":{"what":"Money and invoices."}},{"name":"urgent","description":"The item needs prompt attention."}]})
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&requests[0].body),
        concat!(
            r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","#,
            r#""instructions":["The text is \"The invoice failed.\". Which topics?",{"label":"billing","description":{"what":"Money and invoices."}}],"#,
            r#""criteria":{"true":{"what":"Money and invoices."}}},"#,
            r#""q2":{"type":"noul","#,
            r#""instructions":["The text is \"The invoice failed.\". Which topics?",{"label":"urgent","description":"The item needs prompt attention."}],"#,
            r#""criteria":{"true":"The item needs prompt attention."}}}}"#,
        )
    );
}

#[test]
fn a_structured_tag_text_carries_labels_without_criteria() {
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let question = file(
        "structured-text",
        r#"{"tag":["Which topics?","Ask which apply."],"labels":["billing","urgent"]}"#,
    );
    let output = spawn(
        &[
            "tag",
            &format!("@{}", question.to_string_lossy()),
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"The invoice failed.",
    )
    .expect("tag runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        String::from_utf8_lossy(&requests[0].body),
        concat!(
            r#"{"state":"The invoice failed.","model":"local-1","questions":{"q1":{"type":"noul","#,
            r#""instructions":[["Which topics?","Ask which apply."],{"label":"billing"}]},"#,
            r#""q2":{"type":"noul","#,
            r#""instructions":[["Which topics?","Ask which apply."],{"label":"urgent"}]}}}"#,
        )
    );
}

#[test]
fn an_invalid_tag_description_sends_no_request() {
    let listener = Listener::serving(Vec::new()).expect("listener");
    let bad = file("bad-description", r#"{"tag":"x","labels":{"a":3,"b":"y"}}"#);
    let output = spawn(
        &[
            "tag",
            &format!("@{}", bad.to_string_lossy()),
            "--url",
            listener.base(),
        ],
        &[],
        b"evidence",
    )
    .expect("tag refuses");
    assert_eq!(output.status.code(), Some(5));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "thinkthen: `labels` in the question file is a list of labels, or a map from each label to its description\n"
    );
    assert!(listener.requests().is_empty());
}

#[test]
fn a_tag_recording_replays_without_a_request() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("tag-replay");
    let folder_text = folder.to_string_lossy();
    let _removed = fs::remove_dir_all(&folder);
    let listener = Listener::serving(vec![Canned::ok(ANSWER)]).expect("listener");
    let common = [
        "tag",
        "Which topics?",
        "billing",
        "urgent",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--cache",
        &folder_text,
    ];
    let recorded = spawn(
        &common,
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        b"invoice",
    )
    .expect("tag records");
    assert_eq!(recorded.status.code(), Some(0));
    let replayed = spawn(&common, &[], b"invoice").expect("tag replays");
    assert_eq!(replayed.status.code(), Some(0));
    assert_eq!(replayed.stdout, recorded.stdout);
    assert_eq!(listener.requests().len(), 1);
}

mod matrix;
