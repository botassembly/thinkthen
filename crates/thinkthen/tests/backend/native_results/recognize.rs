//! Shared stage fixtures retain physical spans and logical occurrence IDs through replay.
use super::*;
#[test]
fn recognition_details_replay_original_stages_and_counts_across_source_coordinate_changes() {
    let root = crate::input_sources::folder("complete-command-recognize").unwrap();
    let question = root.join("question.json");
    std::fs::write(&question, r#"{"version":1,"recognize":{"kinds":{"person":null,"organization":null}},"name":"names","wording_version":2,"item_schema":{"type":"string"}}"#).unwrap();
    let input = root.join("text.txt");
    std::fs::write(&input, "Ada met Acme.\n\nAda met Acme.\n").unwrap();
    let recording = root.join("recording");
    let listener = Listener::answering(crate::recognize::automatic).unwrap();
    let reference = format!("@{}", question.display());
    let base = [
        "recognize",
        &reference,
        "--lines",
        "--details",
        "--url",
        listener.base(),
        "--model",
        "local-1",
        "--jobs",
        "1",
        "--max-retries",
        "0",
    ];
    let live = spawn(
        &[
            &base[..],
            &[
                "--unit",
                "line",
                "--input",
                input.to_str().unwrap(),
                "--cache",
                recording.to_str().unwrap(),
            ],
        ]
        .concat(),
        &[("THINKTHEN_API_KEY", "recognition-private")],
        b"ignored",
    )
    .unwrap();
    assert_eq!(
        live.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&live.stderr)
    );
    withheld(&live, "recognition-private");
    let live: Vec<Value> = String::from_utf8_lossy(&live.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    live_sources(&live, &input);
    let requests = listener.requests();
    assert_eq!(requests.len(), 2);
    // The established fixture contract independently pins these complete stage bodies.
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[0].body).unwrap()["state"],
        "Ada met Acme."
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[0].body).unwrap()["questions"]
            .as_object()
            .unwrap()
            .len(),
        4
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[1].body).unwrap()["questions"]
            .as_object()
            .unwrap()
            .len(),
        3
    );
    let held = spawn(
        &[&base[..], &["--replay", recording.to_str().unwrap()]].concat(),
        &[],
        b"Ada met Acme.\nAda met Acme.\n",
    )
    .unwrap();
    assert_eq!(held.status.code(), Some(0));
    let held = rows(&held);
    held_sources(&live, &held);
    assert_eq!(listener.count(), 2);
    validate(
        &live
            .into_iter()
            .chain(held)
            .map(|row| ("completeRecognition".into(), row))
            .collect::<Vec<_>>(),
    );
}

#[cfg(test)]
fn live_sources(live: &[Value], input: &std::path::Path) {
    assert_eq!(live.len(), 2);
    assert_ne!(live[0]["answer_id"], live[1]["answer_id"]);
    for (row, physical) in live.iter().zip([1, 3]) {
        assert_eq!(row["question"]["name"], "names");
        assert_eq!(row["question"]["wording_version"], 2);
        assert_eq!(row["value"]["entities"][0]["file"], input.to_str().unwrap());
        assert_eq!(row["value"]["entities"][0]["first_line"], physical);
        assert_eq!(row["value"]["entities"][1]["last_line"], physical);
        assert_eq!(row["value"]["entities"][1]["start"], 8);
        assert_eq!(row["value"]["entities"][1]["end"], 12);
    }
    assert_eq!(
        live[0]["meta"]["question_sources"]
            .as_array()
            .unwrap()
            .len(),
        7
    );
    assert_eq!(live[0]["meta"]["attempts"].as_array().unwrap().len(), 2);
    assert!(live[1]["meta"]["attempts"].as_array().unwrap().is_empty());
}

#[cfg(test)]
fn held_sources(live: &[Value], held: &[Value]) {
    for (live, held) in live.iter().zip(held) {
        assert_eq!(held["answer_id"], live["answer_id"]);
        assert_eq!(held["answer"], live["answer"]);
        assert_eq!(held["meta"]["origin"], "replay");
        assert_eq!(held["meta"]["attempts"], json!([]));
        assert!(held["value"]["entities"][0].get("file").is_none());
        assert_eq!(held["meta"]["question_sources"][0]["batch_size"], 4);
        assert_eq!(held["meta"]["question_sources"][4]["batch_size"], 3);
    }
}

#[cfg(test)]
fn rows(output: &std::process::Output) -> Vec<Value> {
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
