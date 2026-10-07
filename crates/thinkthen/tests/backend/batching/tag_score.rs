//! Tag's wire expansion and score's ordered levels through compiled record commands.

use serde_json::{Value, json};

use super::{KEY, details, folder, text};
use crate::harness::{Canned, Listener, spawn};
use std::fs;

fn run(
    base: &str,
    verb: &str,
    labels: &[&str],
    extra: &[&str],
    input: &str,
) -> std::process::Output {
    spawn(
        &[
            &[verb, "Which?"],
            labels,
            &[
                "--lines",
                "--details",
                "--no-cache",
                "--url",
                base,
                "--model",
                "local-1",
            ],
            extra,
        ]
        .concat(),
        &[KEY],
        input.as_bytes(),
    )
    .expect("compiled command")
}

#[test]
fn tag_expands_wire_questions_but_reads_one_logical_outcome_per_record() {
    let answer = json!({"model":"local-1","answers":{
        "q1":{"type":"noul","noul":0.9},
        "q2":{"type":"noul","noul":0.1},
        "q3":{"type":"noul","noul":0.1},
        "q4":{"type":"noul","noul":0.9}
    },"usage":{"input_tokens":40,"output_tokens":8}})
    .to_string();
    let listener = Listener::serving(vec![Canned::ok(&answer)]).expect("listener");
    let output = run(
        listener.base(),
        "tag",
        &["billing", "urgent"],
        &["--batch", "2"],
        "first\nsecond\n",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let request: Value = serde_json::from_slice(&requests[0].body).expect("request");
    assert_eq!(
        request["state"],
        json!("Each question quotes the text it asks about.")
    );
    assert_eq!(
        text(&requests[0].body),
        r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"The text is \"first\". Which?\n\nDetermine whether the label \"billing\" applies to this item."},"q2":{"type":"noul","instructions":"The text is \"first\". Which?\n\nDetermine whether the label \"urgent\" applies to this item."},"q3":{"type":"noul","instructions":"The text is \"second\". Which?\n\nDetermine whether the label \"billing\" applies to this item."},"q4":{"type":"noul","instructions":"The text is \"second\". Which?\n\nDetermine whether the label \"urgent\" applies to this item."}}}"#
    );
    let questions = request["questions"].as_object().expect("wire questions");
    assert_eq!(questions.len(), 4);
    assert!(
        questions["q1"]["instructions"]
            .as_str()
            .unwrap_or_default()
            .contains("billing")
    );
    assert!(
        questions["q2"]["instructions"]
            .as_str()
            .unwrap_or_default()
            .contains("urgent")
    );
    assert!(
        questions["q3"]["instructions"]
            .as_str()
            .unwrap_or_default()
            .contains("second")
    );
    let rows = details(&output);
    assert_eq!(rows.len(), 2);
    assert!(
        rows.iter()
            .all(|row| row["meta"]["attempts"][0]["ordinal"] == 1)
    );
    assert_eq!(rows[0]["value"], json!(["billing"]));
    assert_eq!(rows[1]["value"], json!(["urgent"]));
}

#[test]
fn score_uses_ordered_levels_and_shares_an_equal_record() {
    let answer = json!({"model":"local-1","answers":{
        "q1":{"type":"score","score":0.2,"confidence":0.9,"legend":{"0":"low","1":"high"},"probabilities":{"0":0.8,"1":0.2}},
        "q2":{"type":"score","score":0.8,"confidence":0.9,"legend":{"0":"low","1":"high"},"probabilities":{"0":0.2,"1":0.8}}
    }}).to_string();
    let listener = Listener::serving(vec![Canned::ok(&answer)]).expect("listener");
    let output = run(
        listener.base(),
        "score",
        &["low", "high"],
        &["--batch", "3"],
        "first\nsecond\nfirst\n",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let request: Value = serde_json::from_slice(&requests[0].body).expect("request");
    assert_eq!(
        request["state"],
        json!("Each question quotes the text it asks about.")
    );
    assert_eq!(
        request["questions"].as_object().map(serde_json::Map::len),
        Some(2)
    );
    assert_eq!(
        request["questions"]["q1"]["criteria"],
        json!(["low", "high"])
    );
    let rows = details(&output);
    assert_eq!(rows.len(), 3);
    assert!(
        rows.iter()
            .all(|row| row["meta"]["attempts"][0]["ordinal"] == 1)
    );
    assert_eq!(rows[0]["value"], rows[2]["value"]);
    assert_ne!(rows[0]["value"], rows[1]["value"]);
}

#[test]
fn batch_one_preserves_a_tag_documents_request_bytes() {
    let answer = json!({"model":"local-1","answers":{
        "q1":{"type":"noul","noul":0.9},
        "q2":{"type":"noul","noul":0.1}
    }})
    .to_string();
    let listener = Listener::answering(move |_| Canned::ok(&answer)).expect("listener");
    let fixed = [
        "tag",
        "Which?",
        "billing",
        "urgent",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    let document = spawn(&fixed, &[KEY], b"first").expect("document");
    let one = run(
        listener.base(),
        "tag",
        &["billing", "urgent"],
        &["--batch", "1"],
        "first\n",
    );
    assert_eq!(
        document.status.code(),
        Some(0),
        "{}",
        text(&document.stderr)
    );
    assert_eq!(one.status.code(), Some(0), "{}", text(&one.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, requests[1].body);
    assert_eq!(details(&one)[0]["value"], json!(["billing"]));
}

#[test]
fn batch_one_preserves_a_score_documents_request_bytes() {
    let answer = json!({"model":"local-1","answers":{
        "q1":{"type":"score","score":0.2,"confidence":0.9,
            "legend":{"0":"low","1":"high"},"probabilities":{"0":0.8,"1":0.2}}
    }})
    .to_string();
    let listener = Listener::answering(move |_| Canned::ok(&answer)).expect("listener");
    let fixed = [
        "score",
        "Which?",
        "low",
        "high",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "local-1",
    ];
    let document = spawn(&fixed, &[KEY], b"first").expect("document");
    let one = run(
        listener.base(),
        "score",
        &["low", "high"],
        &["--batch", "1"],
        "first\n",
    );
    assert_eq!(
        document.status.code(),
        Some(0),
        "{}",
        text(&document.stderr)
    );
    assert_eq!(one.status.code(), Some(0), "{}", text(&one.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].body, requests[1].body);
    assert_eq!(details(&one)[0]["value"], json!(0.2));
}

#[test]
fn missing_one_tag_label_stops_after_a_complete_row_while_no_labels_is_an_answer() {
    let partial = json!({"model":"local-1","answers":{
        "q1":{"type":"noul","noul":0.1},
        "q2":{"type":"noul","noul":0.1},
        "q3":{"type":"noul","noul":0.9}
    }})
    .to_string();
    let listener = Listener::serving(vec![Canned::ok(&partial)]).expect("listener");
    let output = run(
        listener.base(),
        "tag",
        &["billing", "urgent"],
        &["--batch", "2"],
        "first\nsecond\n",
    );
    assert_eq!(output.status.code(), Some(4));
    let rows = details(&output);
    assert_eq!(rows.len(), 1, "{}", text(&output.stderr));
    assert_eq!(rows[0]["value"], json!([]));
    assert!(text(&output.stderr).contains("record 2"));
    assert_eq!(listener.requests().len(), 1);
}

#[test]
fn tag_wire_name_growth_closes_before_the_tenth_question() {
    let input = "one\ntwo\nthree\nfour\nfive\n";
    let plan = spawn(
        &[
            "tag",
            "Which?",
            "billing",
            "urgent",
            "--lines",
            "--plan",
            "--batch",
            "5",
            "--no-cache",
            "--model",
            "local-1",
        ],
        &[],
        input.as_bytes(),
    )
    .expect("offline plan");
    assert_eq!(plan.status.code(), Some(0), "{}", text(&plan.stderr));
    let lines: Vec<_> = plan.stdout.split(|byte| *byte == b'\n').collect();
    assert_eq!(lines.len(), 3, "plan needs exactly two JSON lines");
    assert!(lines[2].is_empty(), "plan needs a final newline");
    let plan: Value = serde_json::from_slice(lines[0]).expect("prepared request");
    let counts: Value = serde_json::from_slice(lines[1]).expect("whole-input counts");
    assert_eq!(
        counts,
        json!({"records":5,"requests":10,"estimated_bytes":1402,
            "estimated_input_tokens":{"lower":723,"upper":1274},"upper_bound":true})
    );
    let full = serde_json::to_vec(&plan["request"]).expect("body size");
    assert_eq!(
        plan["request"]["questions"]
            .as_object()
            .map(serde_json::Map::len),
        Some(10)
    );
    let maximum = (full.len() - 1).to_string();
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let answers: serde_json::Map<String, Value> = request["questions"]
            .as_object()
            .expect("questions")
            .keys()
            .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let output = run(
        listener.base(),
        "tag",
        &["billing", "urgent"],
        &[
            "--batch",
            "5",
            "--jobs",
            "1",
            "--max-request-bytes",
            &maximum,
        ],
        input,
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    let wires: Vec<usize> = requests
        .iter()
        .map(|request| {
            let body: Value = serde_json::from_slice(&request.body).expect("request");
            body["questions"]
                .as_object()
                .map_or(0, serde_json::Map::len)
        })
        .collect();
    assert_eq!(wires, [8, 2]);
    assert_eq!(details(&output).len(), 5);
}

/// Answers every wire question of one request with a no.
fn noes(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request");
    let answers: serde_json::Map<String, Value> = request["questions"]
        .as_object()
        .expect("questions")
        .keys()
        .map(|key| (key.clone(), json!({"type":"noul","noul":0.1})))
        .collect();
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}

/// One record whose questions pass a limit together splits across requests,
/// one question each, by ADR 0111 section 4, and its row reads them all.
#[test]
fn a_tag_record_over_a_limit_splits_its_labels_across_requests() {
    let directory = folder("tag-profile-wire-limit");
    fs::create_dir_all(&directory).expect("profile directory");
    let profile = format!("{directory}/profile.json");
    fs::write(
        &profile,
        r#"{"schema":"thinkthen.backend-profile/1","name":"tag-wire","max_questions":1}"#,
    )
    .expect("profile");
    for limit in [
        ["--profile", profile.as_str()],
        ["--max-request-bytes", "10"],
    ] {
        let listener = Listener::answering(noes).expect("listener");
        let output = run(
            listener.base(),
            "tag",
            &["billing", "urgent"],
            &limit,
            "one\n",
        );
        assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
        assert_eq!(details(&output)[0]["value"], json!([]));
        let sizes: Vec<usize> = listener
            .requests()
            .iter()
            .map(|request| {
                let body: Value = serde_json::from_slice(&request.body).expect("request");
                body["questions"]
                    .as_object()
                    .map_or(0, serde_json::Map::len)
            })
            .collect();
        assert_eq!(sizes, [1, 1], "{limit:?}");
    }
}

#[test]
fn a_tag_batch_halves_once_and_counts_the_refused_request() {
    let listener = Listener::answering(|body| {
        let request: Value = serde_json::from_slice(body).expect("request");
        let questions = request["questions"].as_object().expect("questions");
        if questions.len() == 6 {
            return Canned::status(413, "{}");
        }
        let answers: serde_json::Map<String, Value> = questions
            .keys()
            .map(|key| (key.clone(), json!({"type":"noul","noul":0.9})))
            .collect();
        Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
    })
    .expect("listener");
    let output = run(
        listener.base(),
        "tag",
        &["billing", "urgent"],
        &["--batch", "3", "--max-retries", "0", "--facts"],
        "one\ntwo\nthree\n",
    );
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let requests = listener.requests();
    assert_eq!(requests.len(), 3);
    let wires: Vec<usize> = requests
        .iter()
        .map(|request| {
            let body: Value = serde_json::from_slice(&request.body).expect("request");
            body["questions"]
                .as_object()
                .map_or(0, serde_json::Map::len)
        })
        .collect();
    // The asks halve, so a record's two labels may ride different halves.
    assert_eq!(wires, [6, 3, 3]);
    let rows = details(&output);
    assert_eq!(rows.len(), 3);
    assert!(
        rows.iter()
            .all(|row| row["value"] == json!(["billing", "urgent"]))
    );
    assert_eq!(rows[0]["meta"]["attempts"][0]["status"], 413);
    let facts: Value = serde_json::from_slice(&output.stderr).expect("facts");
    assert_eq!(facts["requests_sent"], 3);
    assert_eq!(facts["records"], 3);
}

#[test]
fn described_labels_and_levels_keep_their_wire_order() {
    let tag_answer = json!({"model":"local-1","answers":{
        "q1":{"type":"noul","noul":0.9},
        "q2":{"type":"noul","noul":0.1},
        "q3":{"type":"noul","noul":0.1},
        "q4":{"type":"noul","noul":0.9}
    }})
    .to_string();
    let tag_listener = Listener::serving(vec![Canned::ok(&tag_answer)]).expect("listener");
    let tag = run(
        tag_listener.base(),
        "tag",
        &[],
        &[
            "--label",
            "billing=money",
            "--label",
            "urgent=time",
            "--batch",
            "2",
        ],
        "one\ntwo\n",
    );
    assert_eq!(tag.status.code(), Some(0), "{}", text(&tag.stderr));
    let sent = tag_listener.requests();
    assert_eq!(sent.len(), 1);
    let body: Value = serde_json::from_slice(&sent[0].body).expect("request");
    assert_eq!(
        body["questions"].as_object().map(serde_json::Map::len),
        Some(4)
    );
    assert_eq!(body["questions"]["q1"]["criteria"]["true"], "money");
    assert_eq!(body["questions"]["q2"]["criteria"]["true"], "time");
    let rows = details(&tag);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["value"], json!(["billing"]));
    assert_eq!(rows[1]["value"], json!(["urgent"]));

    let directory = folder("described-score");
    fs::create_dir_all(&directory).expect("question directory");
    let file = format!("{directory}/question.json");
    fs::write(
        &file,
        r#"{"score":"Which?","levels":{"low":"small","high":"large"},"batch":2}"#,
    )
    .expect("question file");
    let score_answer = json!({"model":"local-1","answers":{
        "q1":{"type":"score","score":0.2,"confidence":0.9,"legend":{"0":"low","1":"high"},"probabilities":{"0":0.8,"1":0.2}},
        "q2":{"type":"score","score":0.8,"confidence":0.9,"legend":{"0":"low","1":"high"},"probabilities":{"0":0.2,"1":0.8}}
    }}).to_string();
    let score_listener = Listener::serving(vec![Canned::ok(&score_answer)]).expect("listener");
    let argument = format!("@{file}");
    let score = spawn(
        &[
            "score",
            &argument,
            "--lines",
            "--details",
            "--no-cache",
            "--url",
            score_listener.base(),
            "--model",
            "local-1",
        ],
        &[KEY],
        b"one\ntwo\n",
    )
    .expect("score stream");
    assert_eq!(score.status.code(), Some(0), "{}", text(&score.stderr));
    let sent = score_listener.requests();
    assert_eq!(sent.len(), 1);
    let body: Value = serde_json::from_slice(&sent[0].body).expect("request");
    assert_eq!(
        body["questions"]["q1"]["criteria"],
        json!(["small", "large"])
    );
    assert_eq!(
        body["questions"]["q2"]["criteria"],
        json!(["small", "large"])
    );
    assert_eq!(details(&score).len(), 2);
}

#[path = "tag_score/context.rs"]
mod context;
