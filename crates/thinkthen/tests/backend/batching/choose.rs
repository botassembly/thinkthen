//! A compiled choose stream keeps each row's complete options on the wire.

use std::fs;

use serde_json::{Map, Value, json};

use super::{KEY, details, folder, text};
use crate::harness::{Canned, Listener, spawn};

fn chosen_odds(label: &str, chosen: &str) -> Value {
    json!(if label == chosen { 0.9 } else { 0.1 })
}

#[test]
fn equal_selected_records_share_only_an_equal_complete_question() {
    let answer = json!({
        "model": "jev-latest",
        "answers": {
            "q1": {"type":"choice", "choice":"billing", "confidence":0.9,
                "probabilities":{"billing":0.9,"other":0.1}},
            "q2": {"type":"choice", "choice":"shipping", "confidence":0.8,
                "probabilities":{"shipping":0.8,"other":0.2}}
        },
        "usage":{"input_tokens":90,"output_tokens":12}
    });
    let listener = Listener::serving(vec![Canned::ok(&answer.to_string())]).expect("listener");
    let input = concat!(
        "{\"note\":\"same\",\"codes\":{\"billing\":\"Money\",\"other\":\"Fallback\"}}\n",
        "{\"note\":\"same\",\"codes\":{\"shipping\":\"Parcels\",\"other\":\"Fallback\"}}\n",
        "{\"note\":\"same\",\"codes\":{\"billing\":\"Money\",\"other\":\"Fallback\"}}\n",
    );
    let output = spawn(
        &[
            "choose",
            "Which team?",
            "--jsonl",
            "--field",
            "/note",
            "--options",
            "/codes",
            "--batch",
            "3",
            "--details",
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "jev-latest",
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("choose stream");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let wire = text(&requests[0].body);
    assert_eq!(
        wire,
        r#"{"state":"Each question quotes the text it asks about.","model":"jev-latest","questions":{"q1":{"type":"choice","instructions":"The text is \"same\". Which team?","criteria":{"billing":"Money","other":"Fallback"}},"q2":{"type":"choice","instructions":"The text is \"same\". Which team?","criteria":{"shipping":"Parcels","other":"Fallback"}}}}"#
    );
    let rows = details(&output);
    assert_eq!(rows.len(), 3);
    assert!(
        rows.iter()
            .all(|row| row["meta"]["attempts"][0]["ordinal"] == 1)
    );
    assert_eq!(rows[0]["value"], "billing");
    assert_eq!(rows[1]["value"], "shipping");
    assert_eq!(rows[2]["value"], "billing");
    assert_eq!(
        rows[0]["meta"]["question_sha256"],
        "47804fef270a59562f9d8beaf53069ff88488d07986eb4fee37795a71bdfa00f"
    );
    assert_eq!(
        rows[1]["meta"]["question_sha256"],
        "aeee14703d1ed4929f8c95db71eb68aa5fd3479b84ff0f311e6934c5f8e2e9ee"
    );
    assert_eq!(
        rows[0]["meta"]["question_sha256"],
        rows[2]["meta"]["question_sha256"]
    );
    assert_ne!(
        rows[0]["meta"]["question_sha256"],
        rows[1]["meta"]["question_sha256"]
    );
    let facts: Value = serde_json::from_slice(&output.stderr).expect("one final facts line");
    assert_eq!(facts["records"], 3);
    assert_eq!(facts["requests_sent"], 1);
}

#[test]
fn batch_one_reuses_the_one_document_body_and_request_identity() {
    let answer = json!({"model":"jev-latest","answers":{"q1":{
        "type":"choice","choice":"billing","confidence":0.9,
        "probabilities":{"billing":0.9,"other":0.1}
    }}})
    .to_string();
    let listener = Listener::answering(move |_| Canned::ok(&answer)).expect("listener");
    let base = [
        "choose",
        "Which team?",
        "billing",
        "other",
        "--details",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "jev-latest",
    ];
    let old = spawn(&base, &[KEY], b"same").expect("one document");
    let batched = spawn(
        &[&base[..], &["--lines", "--batch", "1"]].concat(),
        &[KEY],
        b"same\n",
    )
    .expect("batch one");
    assert_eq!(old.status.code(), Some(0), "{}", text(&old.stderr));
    assert_eq!(batched.status.code(), Some(0), "{}", text(&batched.stderr));
    let sent = listener.requests();
    assert_eq!(sent.len(), 2);
    assert_eq!(sent[0].body, sent[1].body);
    let one = details(&old);
    let rows = details(&batched);
    assert_eq!(one.len(), 1);
    assert_eq!(rows.len(), 1);
    assert_eq!(one[0]["meta"]["requests"], rows[0]["meta"]["requests"]);
    assert_eq!(rows[0]["value"], "billing");
}

#[test]
fn a_choose_file_warns_when_its_tuned_batch_is_overridden() {
    let place = folder("choose-tuned-batch");
    fs::create_dir_all(&place).expect("question folder");
    let file = format!("{place}/question.json");
    fs::write(
        &file,
        r#"{"choose":"Which team?","options":["billing","other"],"threshold":0.5,"batch":1}"#,
    )
    .expect("choose question file");
    let answer = json!({"model":"jev-latest","answers":{
        "q1":{"type":"choice","choice":"billing","confidence":0.9,
              "probabilities":{"billing":0.9,"other":0.1}},
        "q2":{"type":"choice","choice":"billing","confidence":0.9,
              "probabilities":{"billing":0.9,"other":0.1}}
    }})
    .to_string();
    let listener = Listener::serving(vec![Canned::ok(&answer)]).expect("listener");
    let output = spawn(
        &[
            "choose",
            &format!("@{file}"),
            "--lines",
            "--batch",
            "2",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "jev-latest",
        ],
        &[KEY],
        b"first\nsecond\n",
    )
    .expect("choose stream");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert_eq!(listener.requests().len(), 1);
    assert_eq!(
        text(&output.stderr),
        "thinkthen: warning: threshold tuned at batch 1 is running at batch 2\n"
    );
    let rows = details(&output);
    assert_eq!(rows.len(), 2);
    for row in rows {
        assert_eq!(
            row["meta"]["batch_warning"],
            json!({"tuned_for":1,"running":2})
        );
    }
}

#[test]
fn context_changes_choose_request_identity_and_names_exact_file_bytes() {
    let place = folder("choose-context");
    fs::create_dir_all(&place).expect("context folder");
    let file = format!("{place}/catalog.txt");
    fs::write(&file, b"Shared catalog\n").expect("context file");
    let answer = json!({"model":"jev-latest","answers":{"q1":{
        "type":"choice","choice":"billing","confidence":0.9,
        "probabilities":{"billing":0.9,"other":0.1}
    }}})
    .to_string();
    let listener = Listener::answering(move |_| Canned::ok(&answer)).expect("listener");
    let base = [
        "choose",
        "Which team?",
        "billing",
        "other",
        "--lines",
        "--batch",
        "1",
        "--details",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "jev-latest",
    ];
    let plain = spawn(&base, &[KEY], b"same\n").expect("plain choose");
    let shared = spawn(
        &[&base[..], &["--context", &file]].concat(),
        &[KEY],
        b"same\n",
    )
    .expect("context choose");
    assert_eq!(plain.status.code(), Some(0), "{}", text(&plain.stderr));
    assert_eq!(shared.status.code(), Some(0), "{}", text(&shared.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let shared_body: Value = serde_json::from_slice(&requests[1].body).expect("request body");
    assert_eq!(shared_body["state"], "Shared catalog\n");
    let one = details(&plain);
    let rows = details(&shared);
    assert_eq!(one.len(), 1);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0]["meta"]["context_sha256"],
        "6a927e8e39950cbd7f94d077964a6cab628cecce248f5f5bdb25567007ff1ac3"
    );
    assert_eq!(rows[0]["meta"]["batch"]["setting"], 1);
    assert_eq!(
        one[0]["meta"]["question_sha256"],
        rows[0]["meta"]["question_sha256"]
    );
    assert_ne!(one[0]["meta"]["requests"], rows[0]["meta"]["requests"]);
}

#[test]
fn a_refused_choose_batch_keeps_each_halfs_original_options() {
    let listener = Listener::answering(|request| {
        let body: Value = serde_json::from_slice(request).unwrap_or(Value::Null);
        let Some(questions) = body["questions"].as_object() else {
            return Canned::status(400, "{}");
        };
        if questions.len() == 4 {
            return Canned::status(413, "{}");
        }
        let answers: Map<String, Value> = questions
            .iter()
            .filter_map(|(name, question)| {
                let labels = question["criteria"].as_object()?;
                let chosen = labels.keys().find(|label| label.as_str() != "other")?;
                let odds: Map<String, Value> = labels
                    .keys()
                    .map(|label| (label.clone(), chosen_odds(label, chosen)))
                    .collect();
                Some((
                    name.clone(),
                    json!({"type":"choice","choice":chosen,
                    "confidence":0.9,"probabilities":odds}),
                ))
            })
            .collect();
        Canned::ok(&json!({"model":"jev-latest","answers":answers}).to_string())
    })
    .expect("listener");
    let input = (1..=4)
        .map(|at| format!("{{\"note\":\"row {at}\",\"codes\":[\"a{at}\",\"other\"]}}\n"))
        .collect::<String>();
    let output = spawn(
        &[
            "choose",
            "Which team?",
            "--jsonl",
            "--field",
            "/note",
            "--options",
            "/codes",
            "--batch",
            "4",
            "--details",
            "--no-cache",
            "--max-retries",
            "0",
            "--url",
            listener.base(),
            "--model",
            "jev-latest",
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("choose stream");
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    let sizes: Vec<_> = requests
        .iter()
        .map(|request| {
            let body: Value = serde_json::from_slice(&request.body).expect("request body");
            body["questions"].as_object().map(Map::len).unwrap_or(0)
        })
        .collect();
    assert_eq!(sizes, [4, 2, 2]);
    let rows = details(&output);
    assert_eq!(rows.len(), 4);
    for (at, row) in rows.iter().enumerate() {
        assert_eq!(row["value"], format!("a{}", at + 1));
        assert_eq!(row["meta"]["batch"]["split"], true);
    }
}

#[test]
fn a_profile_refuses_later_large_options_after_the_closed_prefix() {
    let response = json!({"model":"jev-latest","answers":{"q1":{
        "type":"choice","choice":"a1","confidence":0.9,
        "probabilities":{"a1":0.9,"other":0.1}
    }}});
    let response = response.to_string();
    let listener = Listener::answering(move |_| Canned::ok(&response)).expect("listener");
    let place = folder("choose-profile-limit");
    fs::create_dir_all(&place).expect("profile folder");
    let profile = format!("{place}/limit.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"small","max_request_bytes":500}"#,
    )
    .expect("profile");
    let large: Vec<String> = (1..=100).map(|at| format!("option_{at:03}")).collect();
    let input = format!(
        "{{\"note\":\"first\",\"codes\":[\"a1\",\"other\"]}}\n{}\n",
        json!({"note":"second","codes":large})
    );
    let output = spawn(
        &[
            "choose",
            "Which team?",
            "--jsonl",
            "--field",
            "/note",
            "--options",
            "/codes",
            "--profile",
            &profile,
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "jev-latest",
        ],
        &[KEY],
        input.as_bytes(),
    )
    .expect("choose stream");
    assert_eq!(listener.count(), 1, "{}", text(&output.stderr));
    let rows = details(&output);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["value"], "a1");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        text(&output.stderr),
        "thinkthen: profile small allows at most 500 request bytes; this request has 1980\nthinkthen: stopped at record 2; 1 record finished\n"
    );
}

#[test]
fn a_tie_is_a_valid_null_but_a_missing_answer_stops_after_the_prefix() {
    let tied = json!({"model":"jev-latest","answers":{
        "q1":{"type":"choice","choice":"billing","confidence":0.5,
            "probabilities":{"billing":0.5,"other":0.5}},
        "q2":{"type":"choice","choice":"billing","confidence":0.9,
            "probabilities":{"billing":0.9,"other":0.1}}
    }});
    let partial = json!({"model":"jev-latest","answers":{
        "q1":{"type":"choice","choice":"billing","confidence":0.9,
            "probabilities":{"billing":0.9,"other":0.1}}
    }});
    for (name, response, expected, values) in [
        ("tie", tied, 0, vec![Value::Null, json!("billing")]),
        ("missing", partial, 4, vec![json!("billing")]),
    ] {
        let listener =
            Listener::serving(vec![Canned::ok(&response.to_string())]).expect("listener");
        let output = spawn(
            &[
                "choose",
                "Which team?",
                "billing",
                "other",
                "--lines",
                "--batch",
                "2",
                "--details",
                "--no-cache",
                "--url",
                listener.base(),
                "--model",
                "jev-latest",
            ],
            &[KEY],
            b"one\ntwo\n",
        )
        .expect("choose stream");
        assert_eq!(
            output.status.code(),
            Some(expected),
            "{name}: {}",
            text(&output.stderr)
        );
        assert_eq!(listener.requests().len(), 1, "{name}");
        let rows = details(&output);
        assert_eq!(rows.len(), values.len(), "{name}");
        for (row, value) in rows.iter().zip(values) {
            assert_eq!(row["value"], value, "{name}");
        }
    }
}
