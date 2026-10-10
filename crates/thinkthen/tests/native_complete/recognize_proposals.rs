//! Proposal facts survive settlement and share ordinary cached stage answers.
use super::*;
use thinkthen::{Kind, RecognitionSeedSpan, Recognize};

#[cfg(test)]
fn response(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    assert_eq!(request["state"], "!! Ada Bob Åda");
    let questions = request["questions"].as_object().unwrap();
    let boundary = questions["q1"]["criteria"].get("SINGLE").is_some();
    assert_eq!(questions.len(), if boundary { 5 } else { 7 });
    let answers: serde_json::Map<String, Value> = questions
        .iter()
        .map(|(name, question)| {
            let at = name[1..].parse::<usize>().unwrap() - 1;
            let probabilities = if boundary {
                let single = [0.9, 0.8, 0.9, 0.6, 0.4][at];
                json!({"BEGIN":0.0,"INSIDE":0.0,"END":0.0,"SINGLE":single,"OUT":1.0-single})
            } else if question["criteria"].get("person").is_some() {
                let kind = [0.6, 0.0, 0.9, 0.0, 0.2, 0.7, 0.9][at];
                json!({"person":kind,"none of these":1.0-kind})
            } else {
                let options = question["criteria"].as_object().unwrap();
                assert!(options.contains_key("!!"));
                Value::Object(
                    options
                        .keys()
                        .map(|label| (label.clone(), json!(if label == "!!" { 1.0 } else { 0.0 })))
                        .collect(),
                )
            };
            (
                name.clone(),
                json!({"type":"choice","probabilities":probabilities}),
            )
        })
        .collect();
    Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
}

#[test]
fn judged_proposals_retain_declines_duplicate_winners_and_under_cut_seed_facts() {
    let listener = Listener::answering(response).unwrap();
    let cache = folder();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("fake")
        .unwrap()
        .cache_at(&cache)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let ask = Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .build()
        .unwrap()
        .with_seed_spans(vec![RecognitionSeedSpan {
            start: 11,
            end: 14,
            kind: None,
        }]);
    let first = engine
        .recognize_complete_with(&ask, "!! Ada Bob Åda", CallOptions::new())
        .unwrap();
    let result = first.value();
    let proposals = result
        .probabilities()
        .judged_proposals()
        .collect::<Vec<_>>();
    assert_eq!(proposals.len(), 5);
    assert_eq!(
        proposals.iter().map(|p| p.range()).collect::<Vec<_>>(),
        [0..1, 1..2, 3..6, 7..10, 11..14]
    );
    assert_span_probabilities(&proposals);
    assert_eq!(
        proposals.iter().map(|p| p.strength()).collect::<Vec<_>>(),
        [Some(0.54), Some(0.72), None, Some(0.42), Some(0.36)]
    );
    assert_eq!(
        proposals.iter().map(|p| p.kept()).collect::<Vec<_>>(),
        [false, true, false, false, false]
    );
    assert_eq!(proposals[0].selected(), Some(0..2));
    assert_eq!(proposals[1].selected(), Some(0..2));
    assert_eq!(proposals[2].selected(), None);
    assert_eq!(proposals[2].kind(), None);
    assert_eq!(proposals[4].kind(), Some("person"));
    let document: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert!(document["answer"]["proposals"][2].get("strength").is_none());
    assert!(document["answer"]["proposals"][2].get("selected").is_none());
    assert_eq!(
        document["value"],
        json!({"entities":[{"text":"!!","start":0,"end":2,"length":2,"kind":"person","strength":0.72}]})
    );
    schema::call(&first, "completeRecognition");
    let replay = engine
        .recognize_complete_with(&ask, "!! Ada Bob Åda", CallOptions::new())
        .unwrap();
    assert_eq!(first.facts().requests_sent(), 2);
    assert_eq!(replay.facts().requests_sent(), 0);
    assert_eq!(result.answer_id(), replay.value().answer_id());
    assert_eq!(
        result.identity().observations(),
        replay.value().identity().observations()
    );
    let cached: Value = serde_json::from_str(&replay.value().to_json().unwrap()).unwrap();
    assert_eq!(document["answer"], cached["answer"]);
    assert_eq!(listener.count(), 2);
    assert_eq!(listener.questions(), 12);
    std::fs::remove_dir_all(cache).unwrap();
}

#[cfg(feature = "cli")]
#[test]
fn cli_details_show_the_same_proposal_facts_without_changing_bare_output_or_replay_keys() {
    use super::child::ChildEnvironment as _;
    let listener = Listener::answering(response).unwrap();
    let home = folder();
    let input = home.join("input.jsonl");
    std::fs::write(
        &input,
        r#"{"text":"!! Ada Bob Åda","seeds":[{"start":11,"end":14}]}"#,
    )
    .unwrap();
    let recording = home.join("recording");
    let mut command = child::command(env!("CARGO_BIN_EXE_thinkthen"), &[]);
    command
        .isolated_home(&home)
        .env("THINKTHEN_API_KEY", "fake")
        .args([
            "recognize",
            "person",
            "--model",
            "fixed",
            "--url",
            listener.base(),
            "--jsonl",
            "--field",
            "/text",
            "--seed-spans-field",
            "/seeds",
            "--input",
        ])
        .arg(&input)
        .arg("--no-cache")
        .arg("--record")
        .arg(&recording)
        .arg("--details");
    let details = child::run::output(&mut command).unwrap();
    assert!(
        details.status.success(),
        "{}",
        String::from_utf8_lossy(&details.stderr)
    );
    let document: Value = serde_json::from_slice(&details.stdout).unwrap();
    assert_eq!(document["answer"]["proposals"].as_array().unwrap().len(), 5);
    assert_eq!(document["answer"]["proposals"][1]["kept"], true);
    assert_eq!(document["answer"]["proposals"][4]["strength"], 0.36);
    assert!(document["answer"]["proposals"][2].get("strength").is_none());
    let before = std::fs::read_dir(&recording)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    let mut replay = child::command(env!("CARGO_BIN_EXE_thinkthen"), &[]);
    replay
        .isolated_home(&home)
        .args([
            "recognize",
            "person",
            "--model",
            "fixed",
            "--url",
            listener.base(),
            "--jsonl",
            "--field",
            "/text",
            "--seed-spans-field",
            "/seeds",
            "--input",
        ])
        .arg(&input)
        .arg("--no-cache")
        .arg("--replay")
        .arg(&recording);
    let bare = child::run::output(&mut replay).unwrap();
    assert!(
        bare.status.success(),
        "{}",
        String::from_utf8_lossy(&bare.stderr)
    );
    let bare: Value = serde_json::from_slice(&bare.stdout).unwrap();
    assert_eq!(bare["value"], document["value"]);
    assert!(bare.get("answer").is_none());
    assert_eq!(listener.count(), 2);
    assert_eq!(listener.questions(), 12);
    assert_eq!(
        before,
        std::fs::read_dir(&recording)
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect::<Vec<_>>()
    );
    std::fs::remove_dir_all(home).unwrap();
}

#[cfg(test)]
fn assert_span_probabilities(proposals: &[thinkthen::RecognitionProposal<'_>]) {
    for (p, expected) in proposals.iter().zip([0.9, 0.8, 0.9, 0.6, 0.4]) {
        assert!((p.span_probability() - expected).abs() < 0.00001);
    }
}
