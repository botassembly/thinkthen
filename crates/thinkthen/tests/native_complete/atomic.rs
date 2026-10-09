use super::*;

#[test]
fn complete_decision_identity_is_independent_of_retrieval_call_and_default_equivalent_reading() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":887}}"#)).unwrap();
    let folder = folder();
    let build = || {
        Engine::builder()
            .base_url(listener.base())
            .unwrap()
            .model("fixed")
            .unwrap()
            .api_key("complete-private")
            .unwrap()
            .max_retries(0)
    };
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let live = engine
        .decide_complete_with(&question, "Refund me.", CallOptions::new().attempts(true))
        .unwrap();
    let result = live.value();
    assert_eq!(result.value(), Answer::Yes);
    assert_eq!(result.identity().origin(), Some(Origin::Live));
    assert_eq!(result.identity().answered_by(), Some("fixed"));
    assert_eq!(
        result.identity().question_sources()[0].batch_size(),
        Some(1)
    );
    let [Observation::Answered { observation_id }] = result.identity().observations() else {
        panic!("one accepted observation");
    };
    let expected = framed(&[
        "decide",
        r#"{"record":0}"#,
        &format!(r#"[{{"observation_id":"{observation_id}"}}]"#),
        r#"{"question":{"verb":"decide","text":"Refund?"},"threshold":0.5}"#,
        "[]",
    ]);
    assert_eq!(result.answer_id().as_str(), expected);
    assert_decision_document(result);
    schema::call(&live, "completeDecide");
    let equivalent = Question::decide("Refund?").unwrap().cut_at(0.50).unwrap();
    let cache = engine
        .decide_complete_with(&equivalent, "Refund me.", CallOptions::new().attempts(true))
        .unwrap();
    assert_eq!(cache.value().answer_id(), result.answer_id());
    assert_eq!(cache.value().identity().origin(), Some(Origin::Cache));
    assert_ne!(cache.facts().call_id(), live.facts().call_id());
    let cache_json: Value = serde_json::from_str(&cache.value().to_json().unwrap()).unwrap();
    assert_eq!(cache_json["meta"]["attempts"], json!([]));
    let stricter = Question::decide("Refund?").unwrap().cut_at(0.8).unwrap();
    let changed = engine
        .decide_complete_with(&stricter, "Refund me.", CallOptions::new())
        .unwrap();
    assert_eq!(changed.value().value(), Answer::No);
    assert_ne!(changed.value().answer_id(), result.answer_id());
    assert_eq!(
        changed.value().identity().observations(),
        result.identity().observations()
    );
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let held = replay
        .decide_complete_with(&question, "Refund me.", CallOptions::new())
        .unwrap();
    assert_eq!(held.value().answer_id(), result.answer_id());
    assert_eq!(held.value().identity().origin(), Some(Origin::Replay));
    assert_eq!(listener.count(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
        json!({"state":"Each question quotes the text it asks about.","model":"fixed","questions":{"q1":{"type":"noul","instructions":"The text is \"Refund me.\". Refund?"}}})
    );
    let legacy = engine
        .details(&question, "Refund me.")
        .unwrap()
        .value()
        .to_json();
    assert!(legacy.contains("thinkthen.result/1"));
    assert!(!legacy.contains("answer_id"));
    assert!(!format!("{result:?} {live:?}").contains("complete-private"));
    drop(replay);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn concrete_complete_choice_tag_and_score_keep_every_declared_probability_and_successful_null() {
    let listener = Listener::answering(|request| {
        let request: Value = serde_json::from_slice(request).unwrap();
        let questions = request["questions"].as_object().unwrap();
        let mut answers = serde_json::Map::new();
        for (name, question) in questions {
            let answer = match question["type"].as_str().unwrap() {
                "choice" => {
                    json!({"type":"choice","probabilities":{"a":0.5,"b":0.5},"confidence":0.4})
                }
                "score" => json!({"type":"score","probabilities":{"0":0.25,"1":0.75}}),
                "noul" => json!({"type":"noul","noul":if name=="q1" {0.9} else {0.1}}),
                _ => unreachable!(),
            };
            answers.insert(name.clone(), answer);
        }
        Canned::ok(&json!({"model":"fixed","answers":answers}).to_string())
    })
    .unwrap();
    let engine = engine(&listener);
    let choice = Question::choose_labels("Which?")
        .unwrap()
        .label("a", None)
        .unwrap()
        .label("b", None)
        .unwrap()
        .build()
        .unwrap();
    let chosen = engine
        .choose_complete_with(&choice, "Text.", CallOptions::new())
        .unwrap();
    schema::call(&chosen, "completeChoose");
    assert_eq!(chosen.value().value(), None);
    assert_eq!(chosen.value().confidence(), Some(0.4));
    let document: Value = serde_json::from_str(&chosen.value().to_json().unwrap()).unwrap();
    assert_eq!(document["value"], Value::Null);
    assert_eq!(
        document["answer"]["probabilities"],
        json!({"a":0.5,"b":0.5})
    );
    assert!(document["answer_id"].is_string());
    let tags = Question::tag_labels("Topics?")
        .unwrap()
        .label("a", None)
        .unwrap()
        .label("b", None)
        .unwrap()
        .build()
        .unwrap();
    let tagged = engine
        .tag_complete_with(&tags, "Text.", CallOptions::new())
        .unwrap();
    schema::call(&tagged, "completeTag");
    assert_eq!(tagged.value().value(), ["a"]);
    let document: Value = serde_json::from_str(&tagged.value().to_json().unwrap()).unwrap();
    assert_eq!(document["value"], json!(["a"]));
    assert_eq!(
        document["answer"]["probabilities"],
        json!({"a":0.9,"b":0.1})
    );
    assert_eq!(tagged.value().identity().observations().len(), 2);
    assert!(
        tagged
            .value()
            .identity()
            .question_sources()
            .iter()
            .all(|source| source.batch_size() == Some(2))
    );
    let score = Question::score("Grade?")
        .unwrap()
        .level("low", None)
        .unwrap()
        .level("high", None)
        .unwrap()
        .build()
        .unwrap();
    let graded = engine
        .score_complete_with(&score, "Text.", CallOptions::new())
        .unwrap();
    schema::call(&graded, "completeScore");
    assert_eq!(graded.value().value(), 0.75);
    assert_eq!(graded.value().confidence(), None);
    assert_eq!(listener.count(), 3);
    assert_eq!(listener.questions(), 4);
    let refused = engine
        .score_complete_with(&choice, "Text.", CallOptions::new())
        .unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 3);
}

#[cfg(test)]
fn assert_decision_document(result: &CompleteDecision) {
    let document: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert_eq!(document["schema"], "thinkthen.result/2");
    assert_eq!(document["value"], true);
    assert_eq!(document["meta"]["usage"], json!({"input_tokens":887}));
    assert!(document["meta"]["attempts"][0]["sdk_request_id"].is_string());
}
