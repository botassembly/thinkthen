//! Cache/2 metadata and original fixture/1 validation are distinct identities.
use super::*;
use serde_json::value::RawValue;
use std::collections::BTreeMap;

type Members = BTreeMap<String, Box<RawValue>>;

#[test]
fn fixture_questions_use_v2_result_keys_and_original_v1_rows_still_replay_without_writes() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../../specification/fixtures/batching/portable-records.json"
    ))
    .unwrap();
    let portable: Vec<String> = corpus["texts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|text| text.as_str().unwrap().to_owned())
        .collect();
    let fixture_questions: Vec<String> = [
        include_str!("../../../../specification/fixtures/batching/portable-1.request.json"),
        include_str!("../../../../specification/fixtures/batching/portable-2.request.json"),
        include_str!("../../../../specification/fixtures/batching/portable-3.request.json"),
    ]
    .into_iter()
    .flat_map(|body| {
        let body: Members = serde_json::from_str(body).unwrap();
        let questions: Members = serde_json::from_str(body["questions"].get()).unwrap();
        questions
            .into_values()
            .map(|question| question.get().to_owned())
            .collect::<Vec<_>>()
    })
    .collect();
    let cases: Value =
        serde_json::from_str(include_str!("../../../../conformance/cases.json")).unwrap();
    let first = &cases["cases"][0];
    let exchange = &first["exchanges"][0];
    let body: Members = serde_json::from_str(exchange["request"].as_str().unwrap()).unwrap();
    let questions: Members = serde_json::from_str(body["questions"].get()).unwrap();
    for (question, inputs, expected_questions) in [
        (
            corpus["question"].as_str().unwrap(),
            portable,
            fixture_questions,
        ),
        (
            first["question"]["decide"].as_str().unwrap(),
            vec![exchange["evidence"].as_str().unwrap().to_owned()],
            vec![questions["q1"].get().to_owned()],
        ),
    ] {
        check_keys(question, &inputs, &expected_questions);
    }
}

#[cfg(test)]
fn check_keys(question: &str, inputs: &[String], expected_questions: &[String]) {
    let listener = Listener::answering(|body| {
        let body: Value = serde_json::from_slice(body).unwrap();
        let answers = body["questions"]
            .as_object()
            .unwrap()
            .keys()
            .map(|name| (name.clone(), json!({"type":"noul","noul":0.9})))
            .collect::<serde_json::Map<_, _>>();
        Canned::ok(&json!({"model":"jev-1.13.0","answers":answers}).to_string())
    })
    .unwrap();
    let place = folder();
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("jev-1.13.0")
            .unwrap()
            .api_key("native-store-private")
            .unwrap()
            .max_retries(0)
    };
    let engine = build().no_cache().build().unwrap();
    let question = Question::decide(question).unwrap().cut();
    let rows = engine
        .details_many_with(
            &question,
            inputs.iter().map(String::as_str),
            CallOptions::new().batch(BatchSetting::Max),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let sent = listener.requests();
    assert_eq!(sent.len(), 1);
    let parts: Members = serde_json::from_slice(&sent[0].body).unwrap();
    let questions: Members = serde_json::from_str(parts["questions"].get()).unwrap();
    let url = format!("{}/systemone", listener.base());
    let (model, state) = (parts["model"].get(), parts["state"].get());
    let state_digest = sha256(state);
    let mut fixture = vec![json!({"sha256":state_digest,"state":state}).to_string()];
    let mut expected_keys = Vec::new();
    for (at, (row, expected)) in rows.iter().zip(expected_questions).enumerate() {
        let actual = questions[&format!("q{}", at + 1)].get();
        assert_eq!(actual, expected, "fixture question bytes stay unchanged");
        let original_key = sha256(&["systemone", &url, model, state, actual].join("\n"));
        let v2 = framed(
            "thinkthen.question-key/2",
            &["systemone", &url, model, model, state, actual],
        );
        assert_ne!(v2, original_key);
        assert_eq!(row.value().requests(), std::slice::from_ref(&v2));
        expected_keys.push(v2);
        fixture.push(
            json!({"key":original_key,"url":url,"model":"jev-1.13.0",
            "state":state_digest,"question":actual,"answer":ANSWER,
            "answered_by":"jev-1.13.0","input_tokens":null,"output_tokens":null,
            "taken_at":0,"origin":"converted"})
            .to_string(),
        );
    }
    let fixture = fixture.join("\n") + "\n";
    let path = place.join("thinkthen.jsonl");
    std::fs::write(&path, &fixture).unwrap();
    let replay = build().replay(&place).unwrap().build().unwrap();
    let rows = replay
        .details_many_with(
            &question,
            inputs.iter().map(String::as_str),
            CallOptions::new().batch(BatchSetting::Max),
        )
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(rows.len(), inputs.len());
    for (row, key) in rows.iter().zip(expected_keys) {
        assert_eq!(row.value().requests(), &[key]);
        assert_eq!(row.value().question_sources()[0].origin(), Origin::Replay);
        assert_eq!(row.value().requests_sent(), 0);
    }
    assert_eq!(listener.count(), 1);
    assert_eq!(std::fs::read_to_string(path).unwrap(), fixture);
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(place).unwrap();
}

#[cfg(test)]
fn sha256(text: &str) -> String {
    Sha256::digest(text)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
