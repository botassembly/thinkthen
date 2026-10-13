use super::*;

#[test]
fn complete_decision_identity_is_independent_of_retrieval_call_and_default_equivalent_reading() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.7}},"usage":{"input_tokens":887}}"#)).unwrap();
    let folder = folder();
    let build = || crate::api::fixed(listener.base(), "complete-private").max_retries(0);
    let engine = build().cache_at(&folder).unwrap().build().unwrap();
    let question = Question::decide("Refund?").unwrap().cut();
    let decide = |engine: &Engine, question: &Question, options| {
        crate::api::decide(engine, question, "Refund me.", options)
    };
    let live = decide(&engine, &question, CallOptions::new().attempts(true)).unwrap();
    let result = crate::api::decision(&live).unwrap();
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
    let cache = decide(&engine, &equivalent, CallOptions::new().attempts(true)).unwrap();
    let cache_result = crate::api::decision(&cache).unwrap();
    assert_eq!(cache_result.answer_id(), result.answer_id());
    assert_eq!(cache_result.identity().origin(), Some(Origin::Cache));
    assert_ne!(cache.facts().call_id(), live.facts().call_id());
    let cache_json: Value = serde_json::from_str(&cache_result.to_json().unwrap()).unwrap();
    assert_eq!(cache_json["meta"]["attempts"], json!([]));
    let stricter = Question::decide("Refund?").unwrap().cut_at(0.8).unwrap();
    let changed = decide(&engine, &stricter, CallOptions::new()).unwrap();
    let changed_result = crate::api::decision(&changed).unwrap();
    assert_eq!(changed_result.value(), Answer::No);
    assert_ne!(changed_result.answer_id(), result.answer_id());
    assert_eq!(
        changed_result.identity().observations(),
        result.identity().observations()
    );
    let replay = build().replay(&folder).unwrap().build().unwrap();
    let held = decide(&replay, &question, CallOptions::new()).unwrap();
    let held_result = crate::api::decision(&held).unwrap();
    assert_eq!(held_result.answer_id(), result.answer_id());
    assert_eq!(held_result.identity().origin(), Some(Origin::Replay));
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
    let choice = ["a", "b"]
        .into_iter()
        .try_fold(
            Question::choose_labels("Which?").unwrap(),
            |builder, label| builder.label(label, None),
        )
        .unwrap()
        .build()
        .unwrap();
    let chosen = crate::api::request(
        &engine,
        thinkthen::RequestFunction::Choose,
        choice.clone().into(),
        crate::api::text("Text."),
        thinkthen::RequestEnvironment::default(),
    )
    .unwrap();
    let result = crate::api::choices(&chosen).unwrap()[0].result();
    schema::call(&chosen, "completeChoose");
    assert_eq!(result.value(), None);
    assert_eq!(result.confidence(), Some(0.4));
    let document: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert_eq!(document["value"], Value::Null);
    assert_eq!(
        document["answer"]["probabilities"],
        json!({"a":0.5,"b":0.5})
    );
    assert!(document["answer_id"].is_string());
    let tags = Question::tag_labels("Topics?")
        .unwrap()
        .label("a", Some(thinkthen::Description::text("Topic.").unwrap()))
        .unwrap()
        .label("b", None)
        .unwrap()
        .build()
        .unwrap();
    let tagged = crate::api::request(
        &engine,
        thinkthen::RequestFunction::Tag,
        tags.into(),
        crate::api::text("Text."),
        thinkthen::RequestEnvironment::default(),
    )
    .unwrap();
    let tagged_value = crate::api::tags(&tagged).unwrap()[0].result();
    schema::call(&tagged, "completeTag");
    assert_eq!(tagged_value.value(), ["a"]);
    let document: Value = serde_json::from_str(&tagged_value.to_json().unwrap()).unwrap();
    assert_eq!(document["value"], json!(["a"]));
    assert_eq!(
        document["question"]["label_details"],
        json!([
            {"name":"a","description":"Topic."}, {"name":"b"}
        ])
    );
    assert_eq!(
        document["answer"]["probabilities"],
        json!({"a":0.9,"b":0.1})
    );
    assert_eq!(tagged_value.identity().observations().len(), 2);
    assert!(
        tagged_value
            .identity()
            .question_sources()
            .iter()
            .all(|source| source.batch_size() == Some(2))
    );
    typed_score_document(&engine, &described_score(), &choice, &listener);
}

#[cfg(test)]
fn assert_decision_document(result: &CompleteDecision) {
    let document: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert_eq!(document["schema"], "thinkthen.result/2");
    assert_eq!(document["value"], true);
    assert_eq!(document["meta"]["usage"], json!({"input_tokens":887}));
    assert!(document["meta"]["attempts"][0]["sdk_request_id"].is_string());
}

#[cfg(test)]
fn typed_score_document(engine: &Engine, score: &Question, choice: &Question, listener: &Listener) {
    let graded = crate::api::request(
        engine,
        thinkthen::RequestFunction::Score,
        score.clone().into(),
        crate::api::text("Text."),
        thinkthen::RequestEnvironment::default(),
    )
    .unwrap();
    let result = crate::api::scores(&graded).unwrap()[0].result();
    schema::call(&graded, "completeScore");
    assert_eq!(result.value(), 0.75);
    assert_eq!(result.confidence(), None);
    let document = serde_json::to_value(&graded).unwrap();
    assert_eq!(
        document["value"][0]["question"]["levels"],
        json!(["low", "high"])
    );
    assert_eq!(
        document["value"][0]["question"]["label_details"],
        json!([
            {"name":"low","description":"Lower."}, {"name":"high","description":"Higher."}
        ])
    );
    assert_eq!(listener.count(), 3);
    assert_eq!(listener.questions(), 4);
    let refused = crate::api::request(
        engine,
        thinkthen::RequestFunction::Score,
        choice.clone().into(),
        crate::api::text("Text."),
        thinkthen::RequestEnvironment::default(),
    )
    .unwrap_err();
    assert_eq!(refused.kind(), ErrorKind::Usage);
    assert_eq!(listener.count(), 3);
}

#[cfg(test)]
fn described_score() -> Question {
    Question::score("Grade?")
        .unwrap()
        .level("low", Some(thinkthen::Description::text("Lower.").unwrap()))
        .unwrap()
        .level(
            "high",
            Some(thinkthen::Description::text("Higher.").unwrap()),
        )
        .unwrap()
        .build()
        .unwrap()
}
