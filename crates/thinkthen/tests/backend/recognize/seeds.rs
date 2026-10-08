//! Unconfirmed proposals reach classification without changing boundary answers.
use super::*;
use serde_json::json;

fn judge(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let mut answers = serde_json::Map::new();
    for (id, q) in request["questions"].as_object().unwrap() {
        let labels = q["criteria"].as_object().unwrap();
        let pick = if labels.contains_key("BEGIN") {
            "OUT"
        } else if labels.contains_key("ENTITY") {
            "ENTITY"
        } else if labels.contains_key("person") {
            "person"
        } else {
            labels.keys().next().unwrap()
        };
        let probabilities: serde_json::Map<_, _> = labels
            .keys()
            .map(|label| (label.clone(), json!(if label == pick { 1.0 } else { 0.0 })))
            .collect();
        answers.insert(
            id.clone(),
            json!({"type":"choice","choice":pick,"probabilities":probabilities}),
        );
    }
    Canned::ok(&json!({"model":"local-1","answers":answers}).to_string())
}
const FLAGS: &[&str] = &[
    "--jsonl",
    "--field",
    "/body",
    "--seed-spans-field",
    "/seeds",
];

#[test]
fn missed_single_piece_is_classified_but_retains_the_boundary_strength_cut() {
    let listener = Listener::answering(judge).unwrap();
    let input = format!("{}\n", json!({"body":"Ada","seeds":[{"start":0,"end":3}]}));
    let output = run(
        &listener,
        &[FLAGS, &["--details"]].concat(),
        input.as_bytes(),
    );
    let detail: Value = serde_json::from_str(&stdout(&output)).unwrap();
    assert_eq!(listener.count(), 2);
    assert_eq!(detail["answer"]["names"][0]["kinds"]["ENTITY"], 1.0);
    assert_eq!(detail["value"]["entities"], json!([]));
    let baseline = plan_json(&run(&listener, &["--plan"], b"Ada"));
    let seeded = plan_json(&run(
        &listener,
        &[FLAGS, &["--plan"]].concat(),
        input.as_bytes(),
    ));
    assert_eq!(baseline["requests"], seeded["requests"]);
    assert_eq!(listener.count(), 2);
}

#[test]
fn invalid_later_seed_admission_sends_nothing_and_withholds_input() {
    let listener = Listener::answering(judge).unwrap();
    for seeds in [
        Value::Null,
        json!([{"start":1,"end":3}]),
        json!([{"start":0,"end":9}]),
        json!([{"start":0,"end":3,"kind":null}]),
        json!([{"start":0,"end":3,"kind":"secret-kind"}]),
        json!([{"start":-1,"end":3}]),
    ] {
        let input = format!(
            "{}\n{}\n",
            json!({"body":"Ada","seeds":[]}),
            json!({"body":"Bob","seeds":seeds})
        );
        let output = run(&listener, FLAGS, input.as_bytes());
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-kind"));
    }
    assert_eq!(listener.count(), 0);
}

#[test]
fn overlapping_untyped_seeds_have_two_question_bounds_each() {
    let listener = Listener::answering(judge).unwrap();
    let input = format!(
        "{}\n",
        json!({"body":"Ada Bob Cara","seeds":[{"start":0,"end":12},{"start":0,"end":3},{"start":4,"end":12},{"start":4,"end":7}]})
    );
    let plan = plan_json(&run(
        &listener,
        &[FLAGS, &["--plan"]].concat(),
        input.as_bytes(),
    ));
    assert_eq!(plan["name_requests_upper_bound"], 11);
    assert_eq!(listener.count(), 0);
}
