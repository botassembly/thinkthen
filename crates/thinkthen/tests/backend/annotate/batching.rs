//! The compiled annotate command packs each selected group independently.

use serde_json::{Value, json};

use super::set;
use crate::harness::{Canned, Listener, spawn};
use crate::support::digest;

mod progress;
mod splits;
mod tiers;

#[test]
fn two_selected_groups_pack_two_rows_and_align_request_metadata() {
    let file = set(
        "batch-two-groups",
        r#"{"version":1,"questions":{"left_answer":{"decide":"Left?","on":"/left"},"right_answer":{"decide":"Right?","on":"/right"}}}"#,
    );
    let listener = Listener::answering(|_| {
        Canned::ok(
            r#"{"model":"local-1","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}},"usage":{"input_tokens":5,"output_tokens":3}}"#,
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
    assert_eq!(requests.len(), 2);
    let bodies: Vec<Value> = requests
        .iter()
        .map(|request| serde_json::from_slice(&request.body).expect("request JSON"))
        .collect();
    assert!(bodies.contains(&json!({"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"The text is \"one\". Left?"},"q2":{"type":"noul","instructions":"The text is \"two\". Left?"}}})));
    assert!(bodies.contains(&json!({"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Right?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Right?"}}})));
    let left = r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"The text is \"one\". Left?"},"q2":{"type":"noul","instructions":"The text is \"two\". Left?"}}}"#;
    let right = r#"{"state":"Each question quotes the text it asks about.","model":"local-1","questions":{"q1":{"type":"noul","instructions":"The text is \"alpha\". Right?"},"q2":{"type":"noul","instructions":"The text is \"beta\". Right?"}}}"#;
    assert!(
        requests
            .iter()
            .any(|request| request.body == left.as_bytes())
    );
    assert!(
        requests
            .iter()
            .any(|request| request.body == right.as_bytes())
    );
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("row JSON"))
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["answers"]["left_answer"]["value"], true);
    assert_eq!(rows[1]["answers"]["right_answer"]["value"], false);
    for (position, row) in rows.iter().enumerate() {
        assert_eq!(row["meta"]["requests"].as_array().map(Vec::len), Some(2));
        assert_eq!(row["meta"]["batches"].as_array().map(Vec::len), Some(2));
        assert_eq!(row["meta"]["batches"][0]["group"], 1);
        assert_eq!(row["meta"]["batches"][1]["group"], 2);
        assert_eq!(row["meta"]["batches"][0]["position"], position + 1);
        assert_eq!(row["meta"]["batches"][0]["records"], 2);
        assert_eq!(
            row["meta"]["batches"][0]["request"],
            row["meta"]["requests"][0]
        );
        assert_eq!(
            row["meta"]["requests"],
            json!([
                digest(listener.url(), left.as_bytes()),
                digest(listener.url(), right.as_bytes())
            ])
        );
    }
}

#[test]
fn a_profile_splits_one_group_while_another_group_batches() {
    let file = set(
        "batch-profile-groups",
        r#"{"version":1,"questions":{"first":{"decide":"First?","on":"/left"},"second":{"decide":"Second?","on":"/left"},"middle":{"decide":"Middle?","on":"/left"},"third":{"decide":"Third?","on":"/right"}}}"#,
    );
    let profile = set(
        "batch-profile-two-questions",
        r#"{"schema":"thinkthen.backend-profile/1","name":"two-questions","max_questions":2}"#,
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
    let rows: Vec<Value> = output
        .stdout
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).expect("row JSON"))
        .collect();
    assert_eq!(rows.len(), 2);
    for row in &rows {
        assert_eq!(row["meta"]["batches"].as_array().map(Vec::len), Some(3));
        assert_eq!(row["meta"]["batches"][0]["group"], 1);
        assert_eq!(row["meta"]["batches"][1]["group"], 1);
        assert_eq!(row["meta"]["batches"][2]["group"], 2);
        assert_eq!(row["value"]["first"], true);
        assert_eq!(row["value"]["second"], true);
        assert_eq!(row["value"]["middle"], true);
        assert_eq!(row["value"]["third"], true);
    }
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
fn unequal_group_close_points_keep_the_oldest_row_ahead() {
    let file = set(
        "batch-unequal-close",
        r#"{"version":1,"questions":{"a":{"decide":"A?","on":"/wide"},"b":{"decide":"B?","on":"/wide"},"c":{"decide":"C?","on":"/wide"},"d":{"decide":"D?","on":"/wide"},"single":{"decide":"Single?","on":"/narrow"}}}"#,
    );
    let profile = set(
        "batch-unequal-profile",
        r#"{"schema":"thinkthen.backend-profile/1","name":"eight","max_questions":8}"#,
    );
    for jobs in [1, 3] {
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
        let rows: Vec<Value> = output
            .stdout
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice(line).expect("row JSON"))
            .collect();
        assert_eq!(rows.len(), 5);
        for (at, row) in rows.iter().enumerate() {
            assert_eq!(row["input"]["wide"], format!("wide {}", at + 1));
            assert_eq!(row["value"]["single"], true);
            assert_eq!(row["meta"]["batches"].as_array().map(Vec::len), Some(2));
        }
        let requests = listener.requests();
        assert_eq!(requests.len(), 4);
        let counts: Vec<usize> = requests
            .iter()
            .map(|request| {
                let body: Value = serde_json::from_slice(&request.body).expect("request JSON");
                body["questions"]
                    .as_object()
                    .map_or(0, serde_json::Map::len)
            })
            .collect();
        if jobs == 1 {
            assert_eq!(counts, [8, 5, 8, 4]);
        } else {
            let mut sorted = counts;
            sorted.sort();
            assert_eq!(sorted, [4, 5, 8, 8]);
        }
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
