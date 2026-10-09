//! Relation result/2 members and located occurrence expansion share actual observations.
use super::*;
#[test]
fn complete_relation_partial_failure_has_full_success_and_failure_id_with_partial_usage() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"noul"}},"usage":{"input_tokens":887}}"#)).unwrap();
    let output = spawn(
        &[
            "relate",
            "follows=person:person",
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "fixed",
            "--max-retries",
            "0",
        ],
        &[("THINKTHEN_API_KEY", "relation-private")],
        br#"[{"name":"Ada","kind":"person"},{"name":"Grace","kind":"person"}]"#,
    )
    .unwrap();
    assert_eq!(output.status.code(), Some(6));
    withheld(&output, "relation-private");
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        row["answer"]["questions"][0]["answer"],
        json!({"kind":"yes_no","probability":0.9})
    );
    assert!(
        thinkthen::AnswerId::new(row["answer"]["questions"][0]["answer_id"].as_str().unwrap())
            .is_ok()
    );
    let failed = &row["answer"]["questions"][1];
    assert!(thinkthen::FailureId::new(failed["failure_id"].as_str().unwrap()).is_ok());
    assert!(failed.get("answer").is_none() && failed.get("answer_id").is_none());
    assert_eq!(row["meta"]["failed_questions"], 1);
    assert_eq!(row["meta"]["usage"], json!({"input_tokens":887}));
    assert_eq!(listener.count(), 1);
    validate(&[("completeRelation".into(), row)]);
}
#[test]
fn located_relation_duplicates_replay_one_observation_with_actual_filenames_and_coordinates() {
    let root = crate::input_sources::folder("complete-command-relate").unwrap();
    let recording = root.join("recording");
    let input = root.join("names.txt");
    std::fs::write(&input, b"Ada\nBob\nAda\n").unwrap();
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9}},"usage":{"input_tokens":887}}"#)).unwrap();
    let base = [
        "relate",
        "linked",
        "--either",
        "--lines",
        "--unit",
        "line",
        "--details",
        "--no-cache",
        "--url",
        listener.base(),
        "--model",
        "fixed",
        "--max-retries",
        "0",
        "--input",
        input.to_str().unwrap(),
    ];
    let live = spawn(
        &[&base[..], &["--record", recording.to_str().unwrap()]].concat(),
        &[("THINKTHEN_API_KEY", "relation-private")],
        b"Ada\nBob\nAda\n",
    )
    .unwrap();
    assert_eq!(
        live.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&live.stderr)
    );
    withheld(&live, "relation-private");
    let live: Value = serde_json::from_slice(&live.stdout).unwrap();
    assert_eq!(live["value"].as_array().unwrap().len(), 2);
    assert_eq!(
        live["value"][0]["source"],
        json!({"name":"Ada","kind":"*","record":"Ada","file":input.to_str().unwrap(),"first_line":1,"last_line":1})
    );
    assert_eq!(live["value"][1]["source"]["first_line"], 3);
    assert_eq!(live["value"][0]["target"]["first_line"], 2);
    assert_eq!(live["meta"]["usage"], json!({"input_tokens":887}));
    complete_sources(&live, 1);
    let request = serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap();
    assert_eq!(
        request,
        json!({"model":"fixed","state":{"entities":[{"id":"i1","name":"Ada","kind":"*"},{"id":"i2","name":"Bob","kind":"*"}]},"questions":{"q1":{"type":"noul","instructions":"Is it true that i1 linked i2, or that i2 linked i1?"}}})
    );
    let held = spawn(
        &[&base[..], &["--replay", recording.to_str().unwrap()]].concat(),
        &[],
        b"Ada\nBob\nAda\n",
    )
    .unwrap();
    assert_eq!(held.status.code(), Some(0));
    let held: Value = serde_json::from_slice(&held.stdout).unwrap();
    assert_eq!(held["answer_id"], live["answer_id"]);
    assert_eq!(held["value"], live["value"]);
    let mut expected_answer = live["answer"].clone();
    expected_answer["questions"] =
        compatibility::replayed_members(expected_answer["questions"].clone());
    assert_eq!(held["answer"], expected_answer);
    assert_eq!(held["meta"]["attempts"], json!([]));
    assert_eq!(held["meta"]["question_sources"][0]["batch_size"], 1);
    assert_eq!(listener.count(), 1);
    validate(&[
        ("completeRelation".into(), live),
        ("completeRelation".into(), held),
    ]);
}
#[test]
fn a_lone_menu_has_no_observations_answered_model_usage_or_sends() {
    let root = crate::input_sources::folder("complete-command-relate-empty").unwrap();
    let question = root.join("question.json");
    std::fs::write(&question, r#"{"version":1,"relate":{"relations":[{"name":"mentors","source":"*","target":"*","single":true}]},"name":"lone","wording_version":4}"#).unwrap();
    let listener = Listener::answering(|_| panic!("empty relation sent")).unwrap();
    let output = spawn(
        &[
            "relate",
            &format!("@{}", question.display()),
            "--details",
            "--no-cache",
            "--url",
            listener.base(),
            "--model",
            "fixed",
        ],
        &[],
        br#"[{"name":"Ada","kind":"person"}]"#,
    )
    .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let row: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(row["value"], json!([]));
    assert_eq!(row["meta"]["origin"], Value::Null);
    assert_eq!(row["meta"]["cached"], false);
    assert_eq!(row["meta"]["observations"], json!([]));
    assert_eq!(row["meta"]["requests_sent"], 0);
    assert!(row["meta"].get("answered_by").is_none() && row["meta"].get("usage").is_none());
    assert_eq!(row["question"]["name"], "lone");
    assert_eq!(row["question"]["wording_version"], 4);
    assert_eq!(listener.count(), 0);
    validate(&[("completeRelation".into(), row)]);
}
