//! Complete annotation keeps declared readings and original JSON through replay.
use super::*;
const SET: &str = r#"{"version":1,"questions":{"first":{"decide":"First?","on":"/body","name":"first-contract","wording_version":2,"item_schema":{"type":"string"}},"second":{"decide":"Second?","on":"/body"}}}"#;
const INPUT: &[u8] = "{\"body\":\"Text.\",\"n\":1,\"lexical\":\"Keep Café.\"}\n{\"body\":\"Text.\",\"n\":1,\"lexical\":\"Keep Café.\"}\n".as_bytes();
#[cfg(test)]
fn expected() -> Value {
    json!({"model":"fixed","state":"Each question quotes the text it asks about.","questions":{
        "q1":{"type":"noul","instructions":"The text is \"Text.\". First?"},
        "q2":{"type":"noul","instructions":"The text is \"Text.\". Second?"}
    }})
}
#[cfg(test)]
fn run(listener: &Listener, set: &std::path::Path, flags: &[&str]) -> std::process::Output {
    let arguments = [
        &[
            "annotate",
            set.to_str().unwrap(),
            "--jsonl",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "fixed",
            "--max-retries",
            "0",
        ],
        flags,
    ]
    .concat();
    spawn(
        &arguments,
        &[("THINKTHEN_API_KEY", "annotation-private")],
        INPUT,
    )
    .unwrap()
}
#[cfg(test)]
fn rows(output: &std::process::Output) -> Vec<Value> {
    withheld(output, "annotation-private");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
#[test]
fn complete_annotation_replays_coalesced_observations_and_retains_declared_readings_and_lexical_originals()
 {
    let root = crate::input_sources::folder("native-complete-annotation").unwrap();
    let set = root.join("questions.json");
    std::fs::write(&set, SET).unwrap();
    let recording = root.join("recording");
    let listener = Listener::answering(|_|Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul","noul":0.1}},"usage":{"input_tokens":887}}"#)).unwrap();
    let live = run(
        &listener,
        &set,
        &["--batch", "max", "--record", recording.to_str().unwrap()],
    );
    assert_eq!(
        live.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&live.stderr)
    );
    assert!(
        String::from_utf8_lossy(&live.stdout)
            .contains("\"input\":{\"body\":\"Text.\",\"n\":1,\"lexical\":\"Keep Café.\"}")
    );
    let live = rows(&live);
    assert_eq!(live.len(), 2);
    assert_ne!(live[0]["answer_id"], live[1]["answer_id"]);
    assert_eq!(
        live[0]["meta"]["observations"],
        live[1]["meta"]["observations"]
    );
    assert_eq!(
        live[0]["answers"]["first"]["question"],
        json!({"verb":"decide","text":"First?","name":"first-contract","wording_version":2,"item_schema":{"type":"string"}})
    );
    assert_eq!(live[0]["value"], json!({"first":true,"second":false}));
    assert_eq!(live[0]["meta"]["usage"], json!({"input_tokens":887}));
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        expected()
    );
    assert_eq!((listener.count(), listener.questions()), (1, 2));
    let replayed = run(
        &listener,
        &set,
        &["--batch", "1", "--replay", recording.to_str().unwrap()],
    );
    assert_eq!(replayed.status.code(), Some(0));
    let replayed = rows(&replayed);
    for (live, held) in live.iter().zip(&replayed) {
        assert_eq!(held["answer_id"], live["answer_id"]);
        assert_eq!(
            held["answers"],
            compatibility::replayed_members(live["answers"].clone())
        );
        assert_eq!(held["meta"]["origin"], "replay");
        assert_eq!(held["meta"]["requests_sent"], 0);
        assert_eq!(held["meta"]["attempts"], json!([]));
        assert!(
            held["meta"]["question_sources"]
                .as_array()
                .unwrap()
                .iter()
                .all(|s| s["batch_size"] == 2)
        );
    }
    assert_eq!(listener.count(), 1);
    validate(
        &live
            .into_iter()
            .chain(replayed)
            .map(|row| ("completeAnnotation".into(), row))
            .collect::<Vec<_>>(),
    );
}
#[test]
fn complete_annotation_failure_has_actual_failure_identity_without_an_answer_or_reported_output() {
    let root = crate::input_sources::folder("native-complete-annotation-failure").unwrap();
    let set = root.join("questions.json");
    std::fs::write(&set, SET).unwrap();
    let listener = Listener::answering(|_|Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul"}},"usage":{"input_tokens":887}}"#)).unwrap();
    let output = run(&listener, &set, &["--batch", "max"]);
    assert_eq!(output.status.code(), Some(6));
    assert!(output.stderr.is_empty());
    let rows = rows(&output);
    for row in &rows {
        assert_eq!(
            row["value"]["second"],
            json!({"failed":{"kind":"backend","cause":"missing_probability"}})
        );
        assert!(
            thinkthen::FailureId::new(row["answers"]["second"]["failure_id"].as_str().unwrap())
                .is_ok()
        );
        assert!(row["answers"]["second"].get("answer_id").is_none());
        assert!(row["answers"]["second"].get("value").is_none());
        assert_eq!(row["meta"]["usage"], json!({"input_tokens":887}));
    }
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        expected()
    );
    assert_eq!(listener.count(), 1);
    validate(
        &rows
            .into_iter()
            .map(|row| ("completeAnnotation".into(), row))
            .collect::<Vec<_>>(),
    );
}
