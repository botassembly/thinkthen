//! Result/2 command details use actual typed answers and independent exchanges.
use crate::harness::{Canned, Listener, spawn};
use serde_json::{Value, json};
use std::io::Write;
use std::process::Stdio;

#[cfg(test)]
fn validate(rows: &[(String, Value)]) {
    let script = r#"
import json,sys
from jsonschema import Draft202012Validator
case=json.load(sys.stdin)
for name,row in case['rows']:
    Draft202012Validator({'$ref':'#/$defs/'+name,'$defs':case['schema']['$defs']}).validate(row)
"#;
    let mut child = crate::child::command("python3", &[])
        .args(["-c", script])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let document = json!({"schema":serde_json::from_str::<Value>(thinkthen::complete_call_schema()).unwrap(),"rows":rows});
    child
        .stdin
        .take()
        .unwrap()
        .write_all(document.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
#[cfg(test)]
fn withheld(output: &std::process::Output, sentinel: &str) {
    assert!(!String::from_utf8_lossy(&output.stdout).contains(sentinel));
    assert!(!String::from_utf8_lossy(&output.stderr).contains(sentinel));
}
#[cfg(test)]
fn partial_facts(output: &std::process::Output) {
    let facts: Value = serde_json::from_slice(&output.stderr).unwrap();
    assert_eq!(facts["input_tokens"], 887);
    assert!(facts.get("output_tokens").is_none());
    assert!(facts.get("estimated_cost_usd").is_none());
}
#[cfg(test)]
fn complete_sources(row: &Value, count: usize) {
    assert_eq!(
        row["meta"]["question_sources"].as_array().unwrap().len(),
        count
    );
    assert!(row["meta"]["attempts"][0]["sdk_request_id"].is_string());
}
#[test]
fn atomic_command_details_keep_actual_partial_usage_probabilities_and_independent_wire_questions() {
    let fixtures = [
        (
            "decide",
            "Good?",
            vec![],
            r#"{"q1":{"type":"noul","noul":0.1}}"#,
            1,
            json!(false),
            "completeDecide",
            json!({"q1":{"type":"noul","instructions":"The text is \"Text.\". Good?"}}),
        ),
        (
            "choose",
            "Which?",
            vec!["a", "b"],
            r#"{"q1":{"type":"choice","probabilities":{"a":0.5,"b":0.5},"confidence":0.4}}"#,
            3,
            Value::Null,
            "completeChoose",
            json!({"q1":{"type":"choice","instructions":"The text is \"Text.\". Which?","criteria":{"a":null,"b":null}}}),
        ),
        (
            "tag",
            "Topics?",
            vec!["a", "b"],
            r#"{"q1":{"type":"noul","noul":0.1},"q2":{"type":"noul","noul":0.2}}"#,
            0,
            json!([]),
            "completeTag",
            json!({"q1":{"type":"noul","instructions":"The text is \"Text.\". Topics?\n\nDetermine whether the label \"a\" applies to this item."},"q2":{"type":"noul","instructions":"The text is \"Text.\". Topics?\n\nDetermine whether the label \"b\" applies to this item."}}),
        ),
        (
            "score",
            "Grade?",
            vec!["low", "high"],
            r#"{"q1":{"type":"score","probabilities":{"0":0.25,"1":0.75}}}"#,
            0,
            json!(0.75),
            "completeScore",
            json!({"q1":{"type":"score","instructions":"The text is \"Text.\". Grade?","criteria":["low","high"]}}),
        ),
    ];
    let mut rows = Vec::new();
    for (verb, question, labels, answers, code, value, definition, expected) in fixtures {
        let response =
            format!(r#"{{"model":"fixed","answers":{answers},"usage":{{"input_tokens":887}}}}"#);
        let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
        let mut arguments = vec![verb, question];
        arguments.extend(labels);
        arguments.extend([
            "--details",
            "--facts",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "fixed",
            "--max-retries",
            "0",
        ]);
        let output = spawn(
            &arguments,
            &[("THINKTHEN_API_KEY", "native-cli-private")],
            b"Text.",
        )
        .unwrap();
        assert_eq!(
            output.status.code(),
            Some(code),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        withheld(&output, "native-cli-private");
        let row: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(row["schema"], "thinkthen.result/2");
        assert_eq!(row["value"], value);
        assert_eq!(row["meta"]["origin"], "live");
        assert_eq!(row["meta"]["usage"], json!({"input_tokens":887}));
        partial_facts(&output);
        complete_sources(&row, if verb == "tag" { 2 } else { 1 });
        assert_eq!(
            serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
            json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":expected})
        );
        assert_eq!(listener.count(), 1);
        rows.push((definition.into(), row));
    }
    validate(&rows);
}
#[test]
fn filtered_duplicate_occurrences_keep_original_observed_batch_counts_and_replay_identity() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}},"usage":{"input_tokens":9}}"#)).unwrap();
    let root = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("native-filter-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let run = |mode, batch| {
        spawn(
            &[
                "filter",
                "Keep?",
                "--lines",
                "--details",
                "--batch",
                batch,
                "--url",
                listener.base(),
                "--model",
                "fixed",
                "--no-cache",
                mode,
                root.to_str().unwrap(),
                "--max-retries",
                "0",
            ],
            &[("THINKTHEN_API_KEY", "native-filter-private")],
            b"Same.\nSame.\nOther.\n",
        )
        .unwrap()
    };
    let live = run("--record", "max");
    let replay = run("--replay", "1");
    assert_eq!(
        live.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&live.stderr)
    );
    assert_eq!(
        replay.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&replay.stderr)
    );
    let decode = |output: &std::process::Output| {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| serde_json::from_str::<Value>(line).unwrap())
            .collect::<Vec<_>>()
    };
    withheld(&live, "native-filter-private");
    withheld(&replay, "native-filter-private");
    let rows = decode(&live);
    let held = decode(&replay);
    assert_eq!(rows.len(), 2);
    assert_eq!(held.len(), 2);
    assert_eq!(rows[0]["input"], "Same.");
    assert_ne!(rows[0]["answer_id"], rows[1]["answer_id"]);
    assert_eq!(
        rows[0]["meta"]["observations"],
        rows[1]["meta"]["observations"]
    );
    for (row, cached) in rows.iter().zip(&held) {
        assert_eq!(row["answer_id"], cached["answer_id"]);
        assert_eq!(cached["meta"]["origin"], "replay");
        assert_eq!(cached["meta"]["question_sources"][0]["batch_size"], 2);
        assert_eq!(cached["meta"]["requests_sent"], 0);
    }
    assert_eq!(listener.count(), 1);
    assert_eq!(listener.questions(), 2);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Same.\". Keep?"},"q2":{"type":"noul","instructions":"The text is \"Other.\". Keep?"}}})
    );
    validate(
        &rows
            .into_iter()
            .chain(held)
            .map(|row| ("completeFilter".into(), row))
            .collect::<Vec<_>>(),
    );
    std::fs::remove_dir_all(root).unwrap();
}
