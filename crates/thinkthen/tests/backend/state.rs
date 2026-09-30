//! The value a request quotes, decided by what a pointer selects.
//!
//! By ADR 0111 section 1, every request sends the fixed sentence as its
//! state and quotes the selected value's compact JSON at the head of each
//! question. A string quotes as a JSON string, a number, `true`, `false`, and
//! `null` as their JSON spelling, and an object or a list as that JSON value.
//! Two or more pointers quote one object keyed by each pointer's last part.

#![allow(
    clippy::expect_used,
    reason = "a failed fixture setup should stop the boundary test"
)]

use std::fs;
use std::path::PathBuf;

use crate::harness::{Canned, Listener, spawn};
use crate::support::plant_recording;

/// The response the listener gives to the one question a case asks.
const ANSWERED: &str = concat!(
    r#"{"model":"jev-1.13.0","answers":{"q1":{"type":"noul","noul":0.92}},"#,
    r#""usage":{"input_tokens":88,"output_tokens":12}}"#,
);

/// One record carrying one member of every JSON kind.
const RECORD: &str = concat!(
    r#"{"id":"T-91","body":"Payouts failed.","count":3,"ok":false,"#,
    r#""none":null,"meta":{"a":1},"items":[1,"x"],"empty":{},"bare":[]}"#,
);

/// The state every request sends without a context.
const FIXED: &str = r#""state":"Each question quotes the text it asks about.""#;

/// Run `decide` against one base over the bytes on standard input.
fn decide(base: &str, arguments: &[&str], input: &str) -> std::process::Output {
    let asked = [
        "decide",
        "Does this report a payment failure?",
        "--url",
        base,
        "--model",
        "local-1",
    ];
    spawn(
        &[&asked[..], arguments].concat(),
        &[("THINKTHEN_API_KEY", "sk-test-value")],
        input.as_bytes(),
    )
    .expect("the compiled binary runs")
}

/// The one body the listener read, as text.
fn sent(listener: &Listener) -> String {
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    String::from_utf8_lossy(&requests.first().expect("one request").body).into_owned()
}

/// A listener that answers one request.
fn answering() -> Listener {
    Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener")
}

#[test]
fn a_pointer_to_an_object_or_a_list_sends_the_json_value_it_named() {
    for (field, quote) in [
        ("/meta", r#""instructions":"The text is {\"a\":1}. Does"#),
        ("/items", r#""instructions":"The text is [1,\"x\"]. Does"#),
        ("/empty", r#""instructions":"The text is {}. Does"#),
        ("/bare", r#""instructions":"The text is []. Does"#),
    ] {
        let listener = answering();
        let output = decide(
            listener.base(),
            &["--jsonl", "--field", field],
            &format!("{RECORD}\n"),
        );
        assert_eq!(output.status.code(), Some(0), "{field}");
        let body = sent(&listener);
        assert!(body.contains(FIXED) && body.contains(quote), "{field}: {body}");
    }
}

#[test]
fn several_pointers_send_one_object_in_the_order_the_pointers_were_given() {
    let listener = answering();
    let output = decide(
        listener.base(),
        &[
            "--jsonl", "--field", "/body", "--field", "/id", "--field", "/meta",
        ],
        &format!("{RECORD}\n"),
    );
    assert_eq!(output.status.code(), Some(0));
    let body = sent(&listener);
    assert!(
        body.contains(concat!(
            r#""instructions":"The text is {\"body\":\"Payouts failed.\",\"id\":\"T-91\","#,
            r#"\"meta\":{\"a\":1}}. Does"#
        )),
        "{body}"
    );
}

#[test]
fn a_string_quotes_as_a_string_and_a_scalar_as_its_json_spelling() {
    for (field, quote) in [
        ("/body", r#""instructions":"The text is \"Payouts failed.\". Does"#),
        ("/count", r#""instructions":"The text is 3. Does"#),
        ("/ok", r#""instructions":"The text is false. Does"#),
        ("/none", r#""instructions":"The text is null. Does"#),
    ] {
        let listener = answering();
        let output = decide(
            listener.base(),
            &["--jsonl", "--field", field],
            &format!("{RECORD}\n"),
        );
        assert_eq!(output.status.code(), Some(0), "{field}");
        let body = sent(&listener);
        assert!(body.contains(FIXED) && body.contains(quote), "{field}: {body}");
    }
}

/// `RECORD` quoted as a JSON value inside an instruction string.
const QUOTED_RECORD: &str = concat!(
    r#""instructions":"The text is {\"id\":\"T-91\",\"body\":\"Payouts failed.\",\"count\":3,"#,
    r#"\"ok\":false,\"none\":null,\"meta\":{\"a\":1},\"items\":[1,\"x\"],\"empty\":{},"#,
    r#"\"bare\":[]}. Does"#,
);

#[test]
fn the_root_pointer_selects_the_whole_record_as_the_value_it_is() {
    let listener = answering();
    let output = decide(
        listener.base(),
        &["--jsonl", "--field", ""],
        &format!("{RECORD}\n"),
    );
    assert_eq!(output.status.code(), Some(0));
    let body = sent(&listener);
    assert!(body.contains(FIXED) && body.contains(QUOTED_RECORD), "{body}");
}

#[test]
fn no_pointer_quotes_the_whole_record_as_the_value_it_is() {
    let table = concat!(
        r#""instructions":"The text is {\"id\":\"T-91\",\"body\":\"Payouts failed.\"}. "#,
        "Does"
    );
    let cases: [(&[&str], &str, &str); 4] = [
        (&["--jsonl"], &format!("{RECORD}\n"), QUOTED_RECORD),
        (&["--csv"], "id,body\nT-91,Payouts failed.\n", table),
        (&["--tsv"], "id\tbody\nT-91\tPayouts failed.\n", table),
        (
            &["--lines"],
            "Payouts failed.\n",
            r#""instructions":"The text is \"Payouts failed.\". Does"#,
        ),
    ];
    for (framing, input, quote) in cases {
        let listener = answering();
        let output = decide(listener.base(), framing, input);
        assert_eq!(output.status.code(), Some(0), "{framing:?}");
        let body = sent(&listener);
        assert!(body.contains(FIXED) && body.contains(quote), "{framing:?}: {body}");
    }
}

#[test]
fn several_table_columns_send_one_object_in_the_order_the_pointers_named() {
    let cases: [(&str, &str); 2] = [
        ("--csv", "id,body\nT-91,Payouts failed.\n"),
        ("--tsv", "id\tbody\nT-91\tPayouts failed.\n"),
    ];
    for (framing, input) in cases {
        let listener = answering();
        let output = decide(
            listener.base(),
            &[framing, "--field", "/body", "--field", "/id"],
            input,
        );
        assert_eq!(output.status.code(), Some(0), "{framing}");
        let body = sent(&listener);
        assert!(
            body.contains(concat!(
                r#""instructions":"The text is {\"body\":\"Payouts failed.\",\"id\":\"T-91\"}. "#,
                "Does"
            )),
            "{framing}: {body}"
        );
    }
}

#[test]
fn a_blank_selected_string_is_refused_before_any_request() {
    let listener = Listener::serving(vec![]).expect("a loopback listener");
    let output = decide(
        listener.base(),
        &["--jsonl", "--field", "/body"],
        "{\"body\":\"  \"}\n",
    );
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        concat!(
            "thinkthen: the evidence is empty or blank\n",
            "thinkthen: stopped at record 1; 0 records finished\n",
        )
    );
    assert_eq!(listener.connections(), 0);
    assert!(listener.requests().is_empty());
}

#[test]
fn an_evidence_limit_counts_the_fixed_state_and_not_the_quoted_record() {
    // The record rides in the question, so the state is the fixed sentence.
    let state = "Each question quotes the text it asks about.";
    let input = "{\"meta\":{\"a\":1}}\n";
    let profile_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("state-limits");
    let _created = fs::create_dir_all(&profile_dir);
    let profile = |name: &str, bytes: usize| {
        let path = profile_dir.join(format!("{name}.json"));
        let _written = fs::write(
            &path,
            format!(
                r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}","max_evidence_bytes":{bytes}}}"#
            ),
        );
        path
    };
    for (name, limit, code) in [
        ("state-exact", state.len(), 0),
        ("state-under", state.len() - 1, 2),
    ] {
        let limited = profile(name, limit);
        let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
        let output = decide(
            listener.base(),
            &[
                "--jsonl",
                "--field",
                "/meta",
                "--profile",
                &limited.to_string_lossy(),
            ],
            input,
        );
        assert_eq!(output.status.code(), Some(code), "{name}");
        if code == 0 {
            assert_eq!(listener.requests().len(), 1, "{name}");
        } else {
            assert_eq!(listener.connections(), 0, "{name}");
            assert!(listener.requests().is_empty(), "{name}");
            assert_eq!(
                String::from_utf8_lossy(&output.stderr),
                format!(
                    "thinkthen: profile {name} allows at most {} evidence bytes; this request has {}\n\
                     thinkthen: stopped at record 1; 0 records finished\n",
                    limit,
                    state.len()
                )
            );
        }
    }
}

#[test]
fn a_request_limit_counts_the_complete_body_with_a_structured_state() {
    // The exact body the run sends, so the profile lands on and one under it.
    let body = concat!(
        r#"{"state":"Each question quotes the text it asks about.","model":"local-1","#,
        r#""questions":{"q1":{"type":"noul","#,
        r#""instructions":"The text is {\"a\":1}. Does this report a payment failure?"}}}"#,
    );
    let profile_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("state-request-limits");
    let _created = fs::create_dir_all(&profile_dir);
    let profile = |name: &str, bytes: usize| {
        let path = profile_dir.join(format!("{name}.json"));
        let _written = fs::write(
            &path,
            format!(
                r#"{{"schema":"thinkthen.backend-profile/1","name":"{name}","max_request_bytes":{bytes}}}"#
            ),
        );
        path
    };
    let record = format!("{{\"meta\":{}}}\n", r#"{"a":1}"#);
    for (name, limit, code) in [
        ("request-exact", body.len(), 0),
        ("request-under", body.len() - 1, 2),
    ] {
        let limited = profile(name, limit);
        let listener = Listener::serving(vec![Canned::ok(ANSWERED)]).expect("a loopback listener");
        let output = decide(
            listener.base(),
            &[
                "--jsonl",
                "--field",
                "/meta",
                "--profile",
                &limited.to_string_lossy(),
            ],
            &record,
        );
        assert_eq!(output.status.code(), Some(code), "{name}");
        if code == 0 {
            let requests = listener.requests();
            assert_eq!(requests.len(), 1, "{name}");
            assert_eq!(
                String::from_utf8_lossy(&requests.first().expect("one request").body),
                body,
                "{name}"
            );
        } else {
            assert_eq!(listener.connections(), 0, "{name}");
            assert!(listener.requests().is_empty(), "{name}");
        }
    }
}

#[test]
fn an_unquoted_entry_answers_no_run_and_the_quoted_entry_answers_its_own() {
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("state-recording");
    let url = "http://127.0.0.1:9/v1/systemone";
    // The bytes a version before ADR 0111 sent for one named text field: the
    // record was the state. A run today quotes it, so the old entry cannot
    // answer it.
    let old = concat!(
        r#"{"state":"Payouts failed.","model":"local-1","#,
        r#""questions":{"q1":{"type":"noul","instructions":"Does this report a payment failure?"}}}"#,
    );
    plant_recording(&folder, url, old.as_bytes(), ANSWERED).expect("a recorded exchange");
    let output = decide(
        "http://127.0.0.1:9/v1",
        &[
            "--jsonl",
            "--field",
            "/body",
            "--replay",
            &folder.to_string_lossy(),
        ],
        &format!("{RECORD}\n"),
    );
    assert_eq!(output.status.code(), Some(5));
    let said = String::from_utf8_lossy(&output.stderr);
    assert!(
        said.contains("the replay folder holds no entry named"),
        "{said}"
    );

    // The entry in the quoted form answers the same run.
    let folder = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("state-recording-text");
    let quoted = concat!(
        r#"{"state":"Each question quotes the text it asks about.","model":"local-1","#,
        r#""questions":{"q1":{"type":"noul","#,
        r#""instructions":"The text is \"Payouts failed.\". Does this report a payment failure?"}}}"#,
    );
    plant_recording(&folder, url, quoted.as_bytes(), ANSWERED).expect("a recorded exchange");
    let output = decide(
        "http://127.0.0.1:9/v1",
        &[
            "--jsonl",
            "--field",
            "/body",
            "--replay",
            &folder.to_string_lossy(),
        ],
        &format!("{RECORD}\n"),
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
