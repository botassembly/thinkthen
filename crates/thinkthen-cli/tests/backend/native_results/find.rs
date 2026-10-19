//! Dedicated find details keep synthetic selection, authored fields and accepted sources.
use super::*;
#[test]
fn find_none_replays_the_actual_raw_pick_and_partial_usage_without_a_second_send() {
    let root = crate::input_sources::folder("complete-command-find").unwrap();
    let set = root.join("question.json");
    std::fs::write(&set, r#"{"find":"Which?","name":"unit-choice","wording_version":3,"item_schema":{"type":"string"},"on":"/body"}"#).unwrap();
    let recording = root.join("recording");
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"u001":0.5,"u002":0.0,"none":0.5}}},"usage":{"input_tokens":887}}"#)).unwrap();
    let reference = format!("@{}", set.display());
    let base = [
        "find",
        &reference,
        "--none",
        "--jsonl",
        "--details",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "fixed",
        "--max-retries",
        "0",
    ];
    let input = b"{\"body\":\"First.\",\"private\":1}\n{\"body\":\"Second.\",\"private\":2}\n";
    let live = spawn(
        &[&base[..], &["--record", recording.to_str().unwrap()]].concat(),
        &[("THINKTHEN_API_KEY", "find-private")],
        input,
    )
    .unwrap();
    assert_eq!(live.status.code(), Some(3));
    withheld(&live, "find-private");
    let live: Value = serde_json::from_slice(&live.stdout).unwrap();
    assert_eq!(live["value"], Value::Null);
    assert_eq!(live["answer"]["pick"], "u001");
    assert_eq!(
        live["question"],
        json!({"verb":"find","text":"Which?","none":true,"name":"unit-choice","wording_version":3,"item_schema":{"type":"string"},"on":["/body"]})
    );
    assert_eq!(live["meta"]["usage"], json!({"input_tokens":887}));
    complete_sources(&live, 1);
    let requests = listener.requests();
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[0].body).unwrap(),
        json!({"model":"fixed","state":"[{\"id\":\"u001\",\"evidence\":\"First.\"},{\"id\":\"u002\",\"evidence\":\"Second.\"}]","questions":{"q1":{"type":"choice","instructions":"Which?","criteria":{"u001":null,"u002":null,"none":null}}}})
    );
    assert_eq!(
        requests[0].header("x-thinkthen-request-id"),
        live["meta"]["attempts"][0]["sdk_request_id"].as_str()
    );
    let held = spawn(
        &[&base[..], &["--replay", recording.to_str().unwrap()]].concat(),
        &[],
        input,
    )
    .unwrap();
    assert_eq!(held.status.code(), Some(3));
    let held: Value = serde_json::from_slice(&held.stdout).unwrap();
    assert_eq!(held["answer_id"], live["answer_id"]);
    assert_eq!(held["answer"], live["answer"]);
    assert_eq!(held["meta"]["origin"], "replay");
    assert_eq!(held["meta"]["usage"], live["meta"]["usage"]);
    assert_eq!(held["meta"]["attempts"], json!([]));
    assert_eq!(listener.count(), 1);
    validate(&[("completeFind".into(), live), ("completeFind".into(), held)]);
}

#[test]
fn single_unit_with_none_replays_the_original_source_or_none() {
    for (probabilities, selected) in [
        (r#"{"u001":0.9,"none":0.1}"#, true),
        (r#"{"u001":0.1,"none":0.9}"#, false),
    ] {
        let root = crate::input_sources::folder("single-find-none").unwrap();
        let source = root.join("unit.jsonl");
        let original = r#"{"body":"Original.","private":1}"#;
        std::fs::write(&source, format!("{original}\n")).unwrap();
        let recording = root.join("recording");
        let response = format!(
            r#"{{"model":"fixed","answers":{{"q1":{{"type":"choice","probabilities":{probabilities}}}}}}}"#
        );
        let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
        let base = [
            "find",
            "Which?",
            "--none",
            "--jsonl",
            "--field",
            "/body",
            "--details",
            "--no-cache",
            "--input",
            source.to_str().unwrap(),
            "--url",
            listener.base(),
            "--model",
            "fixed",
            "--max-retries",
            "0",
        ];
        let plan = spawn(&[&base[..], &["--plan"]].concat(), &[], b"").unwrap();
        assert_eq!(plan.status.code(), Some(0));
        assert_eq!(listener.count(), 0);
        let live = spawn(
            &[&base[..], &["--record", recording.to_str().unwrap()]].concat(),
            &[("THINKTHEN_API_KEY", "find-private")],
            b"",
        )
        .unwrap();
        assert_eq!(live.status.code(), Some(if selected { 0 } else { 3 }));
        withheld(&live, "find-private");
        let live: Value = serde_json::from_slice(&live.stdout).unwrap();
        assert_eq!(
            live["value"],
            if selected {
                serde_json::from_str::<Value>(original).unwrap()
            } else {
                Value::Null
            }
        );
        assert_eq!(
            live["answer"]["probabilities"],
            serde_json::from_str::<Value>(probabilities).unwrap()
        );
        if selected {
            assert_eq!(
                live["position"],
                json!({"file":source.to_str().unwrap(),"first":1,"last":1})
            );
        } else {
            assert!(live.get("position").is_none());
        }
        let held = spawn(
            &[&base[..], &["--replay", recording.to_str().unwrap()]].concat(),
            &[],
            b"",
        )
        .unwrap();
        assert_eq!(held.status.code(), Some(if selected { 0 } else { 3 }));
        let held: Value = serde_json::from_slice(&held.stdout).unwrap();
        assert_eq!(held["answer_id"], live["answer_id"]);
        assert_eq!(held["answer"], live["answer"]);
        assert_eq!(held["meta"]["origin"], "replay");
        assert_eq!(listener.count(), 1);
    }
}
