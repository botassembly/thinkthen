//! The compiled annotate command packs every group of every record together.

#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    reason = "a failed fixture setup or a missing field should stop the boundary test"
)]

use serde_json::{Value, json};

use super::set;
use crate::harness::{Canned, Listener, spawn};
use crate::support::keys;

mod progress;
mod splits;
mod tiers;

/// A reply that answers every question of the request yes.
fn all_yes(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).expect("request JSON");
    let count = request["questions"]
        .as_object()
        .map_or(0, serde_json::Map::len);
    let answers = (1..=count)
        .map(|at| (format!("q{at}"), json!({"type":"noul","noul":0.9})))
        .collect::<serde_json::Map<_, _>>();
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}

fn rows(stdout: &[u8]) -> Vec<Value> {
    stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("row JSON"))
        .collect()
}

#[test]
fn two_selected_groups_pack_two_rows_into_one_request_and_name_question_keys() {
    let file = set(
        "batch-two-groups",
        r#"{"version":1,"questions":{"left_answer":{"decide":"Left?","on":"/left"},"right_answer":{"decide":"Right?","on":"/right"}}}"#,
    );
    let listener = Listener::answering(|_| {
        Canned::ok(
            r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1},"q3":{"type":"noul","noul":0.9},"q4":{"type":"noul","noul":0.1}},"usage":{"input_tokens":8,"output_tokens":4}}"#,
        )
    })
    .expect("listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--batch",
            "2",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"{\"left\":\"one\",\"right\":\"alpha\"}\n{\"left\":\"two\",\"right\":\"beta\"}\n",
    )
    .expect("annotate stream");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body = concat!(
        r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"#,
        r#""q1":{"type":"noul","instructions":"The text is \"one\". Left?"},"#,
        r#""q2":{"type":"noul","instructions":"The text is \"alpha\". Right?"},"#,
        r#""q3":{"type":"noul","instructions":"The text is \"two\". Left?"},"#,
        r#""q4":{"type":"noul","instructions":"The text is \"beta\". Right?"}}}"#,
    );
    assert_eq!(String::from_utf8_lossy(&requests[0].body), body);
    let asked = keys(listener.url(), body.as_bytes());
    let rows = rows(&output.stdout);
    assert_eq!(rows.len(), 2);
    for (position, row) in rows.iter().enumerate() {
        assert_eq!(row["answers"]["left_answer"]["value"], true);
        assert_eq!(row["answers"]["right_answer"]["value"], false);
        assert_eq!(
            row["meta"]["requests"],
            json!(asked[position * 2..position * 2 + 2])
        );
        assert_eq!(
            row["meta"]["usage"],
            json!({"input_tokens":4,"output_tokens":2})
        );
        assert!(row["meta"].get("batches").is_none(), "{row}");
    }
}

#[test]
fn a_profile_splits_packed_records_at_its_question_count() {
    let file = set(
        "batch-profile-groups",
        r#"{"version":1,"questions":{"first":{"decide":"First?","on":"/left"},"second":{"decide":"Second?","on":"/left"},"middle":{"decide":"Middle?","on":"/left"},"third":{"decide":"Third?","on":"/right"}}}"#,
    );
    let profile = set(
        "batch-profile-two-questions",
        r#"{"schema":"thinkthen.backend-profile/1","name":"two-questions","max_questions":2}"#,
    );
    let listener = Listener::answering(all_yes).expect("listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--jsonl",
            "--batch",
            "2",
            "--profile",
            &profile.to_string_lossy(),
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"{\"left\":\"one\",\"right\":\"alpha\"}\n{\"left\":\"two\",\"right\":\"beta\"}\n",
    )
    .expect("annotate stream");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 4);
    for request in &requests {
        let body: Value = serde_json::from_slice(&request.body).expect("request JSON");
        assert_eq!(
            body["questions"].as_object().map(serde_json::Map::len),
            Some(2),
            "{body}"
        );
    }
    let mut asked: Vec<Value> = requests
        .iter()
        .flat_map(|request| keys(listener.url(), &request.body))
        .map(Value::from)
        .collect();
    let rows = rows(&output.stdout);
    assert_eq!(rows.len(), 2);
    let mut named = Vec::new();
    for row in &rows {
        let requests = row["meta"]["requests"].as_array().expect("request keys");
        assert_eq!(requests.len(), 4);
        named.extend(requests.iter().cloned());
        assert!(row["meta"].get("batches").is_none(), "{row}");
        assert_eq!(row["value"]["first"], true);
        assert_eq!(row["value"]["second"], true);
        assert_eq!(row["value"]["middle"], true);
        assert_eq!(row["value"]["third"], true);
    }
    asked.sort_by_key(ToString::to_string);
    named.sort_by_key(ToString::to_string);
    assert_eq!(asked, named);
}

#[test]
fn structured_question_text_keeps_singleton_request_bytes_at_max() {
    let file = set(
        "batch-structured-singleton",
        r#"{"version":1,"questions":{"ready":{"decide":["Ready?"]}}}"#,
    );
    let listener = Listener::answering(|_| {
        Canned::ok(r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9}}}"#)
    })
    .expect("listener");
    let run = |batch: &str| {
        spawn(
            &[
                "annotate",
                &file.to_string_lossy(),
                "--lines",
                "--batch",
                batch,
                "--details",
                "--no-cache",
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &[("THINKTHEN_API_KEY", "key")],
            b"one\ntwo\n",
        )
        .expect("annotate stream")
    };
    let singleton = run("1");
    let maximum = run("max");
    assert_eq!(
        singleton.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&singleton.stderr)
    );
    assert_eq!(
        maximum.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&maximum.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 4);
    let mut first = vec![requests[0].body.clone(), requests[1].body.clone()];
    let mut second = vec![requests[2].body.clone(), requests[3].body.clone()];
    first.sort();
    second.sort();
    assert_eq!(first, second);
    assert_eq!(singleton.stdout, maximum.stdout);
}

#[test]
fn partial_questions_keep_good_siblings_and_exit_six() {
    let file = set(
        "batch-partial",
        r#"{"version":1,"questions":{"good":{"decide":"Good?"},"bad":{"decide":"Bad?"}}}"#,
    );
    let response = r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"x":1.0}},"q3":{"type":"noul","noul":0.9},"q4":{"type":"choice","probabilities":{"x":1.0}}}}"#;
    let listener = Listener::serving(vec![Canned::ok(response)]).expect("listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--lines",
            "--batch",
            "2",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\ntwo\n",
    )
    .expect("annotate stream");
    assert_eq!(
        output.status.code(),
        Some(6),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(listener.requests().len(), 1);
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("row JSON"))
        .collect();
    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert_eq!(row["value"]["good"], true);
        assert_eq!(row["value"]["bad"]["failed"]["kind"], "backend");
        assert_eq!(row["meta"]["failed_questions"], 1);
        assert!(row["answers"]["bad"].get("value").is_none());
    }
}

#[test]
fn a_question_cap_keeps_records_whole_and_rows_in_order() {
    let file = set(
        "batch-unequal-close",
        r#"{"version":1,"questions":{"a":{"decide":"A?","on":"/wide"},"b":{"decide":"B?","on":"/wide"},"c":{"decide":"C?","on":"/wide"},"d":{"decide":"D?","on":"/wide"},"single":{"decide":"Single?","on":"/narrow"}}}"#,
    );
    let profile = set(
        "batch-unequal-profile",
        r#"{"schema":"thinkthen.backend-profile/1","name":"eight","max_questions":8}"#,
    );
    for jobs in [1, 3] {
        let listener = Listener::answering(all_yes).expect("listener");
        let input = (1..=5)
            .map(|at| format!("{{\"wide\":\"wide {at}\",\"narrow\":\"narrow {at}\"}}\n"))
            .collect::<String>();
        let output = spawn(
            &[
                "annotate",
                &file.to_string_lossy(),
                "--jsonl",
                "--batch",
                "max",
                "--profile",
                &profile.to_string_lossy(),
                "--jobs",
                &jobs.to_string(),
                "--no-cache",
                "--details",
                "--url",
                listener.base(),
                "--model",
                "local-1",
            ],
            &[("THINKTHEN_API_KEY", "key")],
            input.as_bytes(),
        )
        .expect("annotate stream");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let rows = rows(&output.stdout);
        assert_eq!(rows.len(), 5);
        for (at, row) in rows.iter().enumerate() {
            assert_eq!(row["input"]["wide"], format!("wide {}", at + 1));
            assert_eq!(row["value"]["single"], true);
            assert_eq!(row["meta"]["requests"].as_array().map(Vec::len), Some(5));
        }
        // Two records would pass eight questions, so each record goes whole.
        let requests = listener.requests();
        assert_eq!(requests.len(), 5);
        let counts: Vec<usize> = requests
            .iter()
            .map(|request| {
                let body: Value = serde_json::from_slice(&request.body).expect("request JSON");
                body["questions"]
                    .as_object()
                    .map_or(0, serde_json::Map::len)
            })
            .collect();
        assert_eq!(counts, [5; 5]);
    }
}

#[test]
fn tag_labels_cross_q9_without_using_wire_offsets_as_logical_places() {
    let file = set(
        "batch-tag-width",
        r#"{"version":1,"questions":{"topics":{"tag":"Topics?","labels":["a","b","c","d","e"]}}}"#,
    );
    let answers = (1..=10)
        .map(|at| {
            (
                format!("q{at}"),
                json!({"type":"noul","noul":if at == 1 || at == 10 {0.9} else {0.1}}),
            )
        })
        .collect::<serde_json::Map<_, _>>();
    let listener = Listener::serving(vec![Canned::ok(
        &json!({"model":"local-1","answers":answers}).to_string(),
    )])
    .expect("listener");
    let output = spawn(
        &[
            "annotate",
            &file.to_string_lossy(),
            "--lines",
            "--batch",
            "2",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "local-1",
        ],
        &[("THINKTHEN_API_KEY", "key")],
        b"one\ntwo\n",
    )
    .expect("annotate stream");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let requests = listener.requests();
    assert_eq!(requests.len(), 1);
    let body: Value = serde_json::from_slice(&requests[0].body).expect("request JSON");
    assert_eq!(
        body["questions"].as_object().map(serde_json::Map::len),
        Some(10)
    );
    assert!(body["questions"].get("q9").is_some());
    assert!(body["questions"].get("q10").is_some());
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("row JSON"))
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["value"]["topics"], json!(["a"]));
    assert_eq!(rows[1]["value"]["topics"], json!(["e"]));
}
