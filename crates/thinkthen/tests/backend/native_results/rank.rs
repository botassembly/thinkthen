//! Final rank positions and set identities are stable through top selection/replay.
use super::*;
#[cfg(test)]
fn rows(output: &std::process::Output) -> Vec<Value> {
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
#[test]
fn rank_details_have_numeric_stable_positions_full_answers_and_zero_send_replay() {
    let root = crate::input_sources::folder("complete-command-rank").unwrap();
    let recording = root.join("recording");
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.2},"q2":{"type":"noul","noul":0.8},"q3":{"type":"noul","noul":0.8}},"usage":{"input_tokens":887}}"#)).unwrap();
    let run = |mode, batch, top: &[&str]| {
        spawn(
            &[
                &[
                    "rank",
                    "Good?",
                    "--lines",
                    "--details",
                    "--batch",
                    batch,
                    "--no-cache",
                    "--url",
                    listener.base(),
                    "--model",
                    "fixed",
                    mode,
                    recording.to_str().unwrap(),
                ][..],
                top,
            ]
            .concat(),
            &[("THINKTHEN_API_KEY", "rank-private")],
            b"a\nb\nc\n",
        )
        .unwrap()
    };
    let live_output = run("--record", "max", &[]);
    withheld(&live_output, "rank-private");
    let live = rows(&live_output);
    assert_eq!(
        live.iter()
            .map(|row| row["input"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["b", "c", "a"]
    );
    assert_eq!(
        live.iter()
            .map(|row| row["value"].as_u64().unwrap())
            .collect::<Vec<_>>(),
        [1, 2, 3]
    );
    for row in &live {
        assert_eq!(row["schema"], "thinkthen.result/2");
        assert_eq!(row["threshold"], Value::Null);
        assert_eq!(row["meta"]["question_sources"][0]["batch_size"], 3);
        assert!(row["meta"]["usage"].get("output_tokens").is_none());
    }
    assert_eq!(
        live[0]["answer"],
        json!({"kind":"yes_no","probability":0.8})
    );
    let replay = rows(&run("--replay", "1", &["--top", "2"]));
    for (actual, original) in replay.iter().zip(&live) {
        assert_eq!(actual["answer_id"], original["answer_id"]);
        assert_eq!(actual["value"], original["value"]);
        assert_eq!(actual["meta"]["attempts"], json!([]));
    }
    assert_eq!(replay.len(), 2);
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"model":"fixed","state":"Each question quotes the text it asks about.","questions":{
            "q1":{"type":"noul","instructions":"The text is \"a\". Good?"},
            "q2":{"type":"noul","instructions":"The text is \"b\". Good?"},
            "q3":{"type":"noul","instructions":"The text is \"c\". Good?"}
        }})
    );
    validate(
        &live
            .into_iter()
            .chain(replay)
            .map(|row| ("completeRank".into(), row))
            .collect::<Vec<_>>(),
    );
}
#[test]
fn set_rank_top_preserves_all_member_observations_and_positions_outside_member_top() {
    let root = crate::input_sources::folder("complete-command-set-rank").unwrap();
    let question = root.join("questions.json");
    std::fs::write(&question, r#"{"version":1,"questions":{"first":{"decide":"First?","name":"alpha","wording_version":2},"second":{"decide":"Second?","name":"beta","wording_version":3}}}"#).unwrap();
    let reference = format!("@{}", question.display());
    let recording = root.join("recording");
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7},"q2":{"type":"noul","noul":1},"q3":{"type":"noul","noul":0.6},"q4":{"type":"noul","noul":0.98},"q5":{"type":"noul","noul":0.5},"q6":{"type":"noul","noul":0.99}},"usage":{"input_tokens":887}}"#)).unwrap();
    let run = |mode, batch, top: &[&str]| {
        spawn(
            &[
                &[
                    "rank",
                    &reference,
                    "--lines",
                    "--details",
                    "--batch",
                    batch,
                    "--no-cache",
                    "--url",
                    listener.base(),
                    "--model",
                    "fixed",
                    mode,
                    recording.to_str().unwrap(),
                ][..],
                top,
            ]
            .concat(),
            &[],
            b"a\nb\nc\n",
        )
        .unwrap()
    };
    let live = rows(&run("--record", "max", &[]));
    assert_eq!(
        live.iter()
            .map(|row| (&row["input"], &row["question_name"], &row["value"]))
            .collect::<Vec<_>>(),
        vec![
            (&json!("a"), &json!("first"), &json!(1)),
            (&json!("b"), &json!("first"), &json!(2)),
            (&json!("c"), &json!("second"), &json!(3))
        ]
    );
    for row in &live {
        assert_eq!(row["meta"]["observations"].as_array().unwrap().len(), 2);
        assert_eq!(row["meta"]["question_sources"].as_array().unwrap().len(), 2);
        assert!(row["meta"]["question_sha256"].is_string());
        assert!(row["meta"].get("questions_sha256").is_none());
        assert_eq!(row["meta"]["attempts"].as_array().unwrap().len(), 1);
        assert!(row["meta"]["usage"].get("output_tokens").is_none());
    }
    assert_eq!(live[1]["question"]["name"], "alpha");
    assert_eq!(live[2]["question"]["wording_version"], 3);
    let replay = rows(&run("--replay", "1", &["--top", "2"]));
    assert_eq!(replay.len(), 2);
    for (actual, original) in replay.iter().zip(&live) {
        assert_eq!(actual["answer_id"], original["answer_id"]);
        assert_eq!(
            actual["meta"]["observations"],
            original["meta"]["observations"]
        );
        assert_eq!(actual["meta"]["question_sources"][1]["batch_size"], 6);
    }
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"model":"fixed","state":"Each question quotes the text it asks about.","questions":{
            "q1":{"type":"noul","instructions":"The text is \"a\". First?"},"q2":{"type":"noul","instructions":"The text is \"a\". Second?"},
            "q3":{"type":"noul","instructions":"The text is \"b\". First?"},"q4":{"type":"noul","instructions":"The text is \"b\". Second?"},
            "q5":{"type":"noul","instructions":"The text is \"c\". First?"},"q6":{"type":"noul","instructions":"The text is \"c\". Second?"}
        }})
    );
    validate(
        &live
            .into_iter()
            .chain(replay)
            .map(|row| ("completeRank".into(), row))
            .collect::<Vec<_>>(),
    );
}
