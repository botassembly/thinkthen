use super::*;
use thinkthen::{Entity, Kind, QuestionSet, Recognize, Relate, RelationRule};

#[test]
fn whole_set_find_context_is_separate_and_every_candidate_retains_its_original() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"choice","probabilities":{"u001":0.2,"u002":0.3,"none":0.5},"confidence":0.6}},"usage":{"input_tokens":887}}"#)).unwrap();
    let engine = engine(&listener);
    let question = Question::find("Where?").unwrap().offering_none().unwrap();
    let found = engine
        .find_complete_with(
            &question,
            ["first", "second"],
            CallOptions::new().context("  context\n").attempts(true),
        )
        .unwrap();
    schema::call(&found, "completeFind");
    assert_eq!(found.value().selected(), None);
    assert_eq!(found.value().raw_pick(), "none");
    assert!(found.value().question().offers_none());
    assert_eq!(found.value().candidates()[0].input(), Some(&"first"));
    assert_eq!(found.value().candidates()[2].probability(), 0.5);
    assert!(found.value().candidates()[2].is_none());
    let row: Value = serde_json::from_str(&found.value().to_json().unwrap()).unwrap();
    assert_eq!(row["schema"], "thinkthen.result/2");
    assert_eq!(row["value"], Value::Null);
    assert_eq!(
        row["answer"]["probabilities"],
        json!({"u001":0.2,"u002":0.3,"none":0.5})
    );
    assert_eq!(row["meta"]["usage"], json!({"input_tokens":887}));
    assert_eq!(row["meta"]["attempts"].as_array().unwrap().len(), 1);
    find_metadata(found.value().meta());
    let request: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(
        request["state"],
        json!({"context":"  context\n","evidence":r#"[{"id":"u001","evidence":"first"},{"id":"u002","evidence":"second"}]"#})
    );
    assert_eq!(listener.questions(), 1);
    let legacy = engine
        .find_with(
            &question,
            ["first", "second"],
            CallOptions::new().context("legacy context"),
        )
        .unwrap();
    assert!(legacy.value().selected().is_none());
    assert_eq!(listener.count(), 2);
}

#[test]
fn native_annotation_keeps_member_null_failure_order_and_actual_batch_sources() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"wrong":1.0}},"q3":{"type":"noul","noul":0.5}},"usage":{"input_tokens":9}}"#)).unwrap();
    let engine = engine(&listener);
    let set = QuestionSet::from_json(r#"{"version":1,"questions":{"good":{"decide":"Good?"},"failed":{"decide":"Fails?"},"unsure":{"decide":"Sure?","threshold":"0.1:0.9"}}}"#).unwrap();
    let rows = engine
        .annotate_complete_with(
            &set,
            ["Original."],
            CallOptions::new().context("Separate.").attempts(true),
        )
        .unwrap();
    schema::call(&rows, "completeAnnotation");
    assert_eq!(rows.value()[0].original(), &"Original.");
    let result = rows.value()[0].result();
    let members = result.members().collect::<Vec<_>>();
    assert_eq!(
        members.iter().map(|m| m.name()).collect::<Vec<_>>(),
        ["good", "failed", "unsure"]
    );
    assert!(members[0].answer_id().is_some());
    assert!(members[1].answer_id().is_none());
    assert!(members[1].failure_id().is_some());
    assert!(members[2].answer_id().is_some());
    assert_eq!(
        members[2].value(),
        Some(thinkthen::Judgment::Decision(Answer::Unsure))
    );
    let row: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert_eq!(row["value"]["good"], true);
    assert_eq!(row["value"]["unsure"], Value::Null);
    assert_eq!(row["meta"]["usage"], json!({"input_tokens":9}));
    assert_eq!(row["meta"]["failed_questions"], 1);
    annotation_member_documents(result, &row);
    assert_eq!(row["answers"]["failed"]["threshold"], json!(0.5));
    annotation_metadata(result);
    assert_eq!(result.identity().observations().len(), 3);
    assert!(
        result
            .identity()
            .question_sources()
            .iter()
            .all(|source| source.batch_size() == Some(3))
    );
    let request: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(request["state"], "Separate.");
    assert_eq!(
        request["questions"]["q1"]["instructions"],
        "The text is \"Original.\". Good?"
    );
    assert_eq!(listener.count(), 1);
}

#[test]
fn complete_relations_keep_full_answers_rejections_failures_and_empty_metadata() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.9},"q2":{"type":"choice","probabilities":{"wrong":1.0}}}}"#)).unwrap();
    let engine = engine(&listener);
    let ask = Relate::builder()
        .relation(RelationRule::one_way("follows", "person", "person").unwrap())
        .unwrap()
        .build()
        .unwrap();
    let result = engine
        .relate_complete_with(
            &ask,
            [
                Entity::new("Ada", "person").unwrap(),
                Entity::new("Grace", "person").unwrap(),
            ],
            CallOptions::new().context("Separate.").attempts(true),
        )
        .unwrap();
    schema::call(&result, "completeRelation");
    assert_eq!(result.value().value().len(), 1);
    let members = result.value().members().collect::<Vec<_>>();
    relation_reading(result.value());
    assert_eq!(members[0].probability(), Some(0.9));
    assert!(members[0].probabilities().is_some());
    assert!(members[0].answer_id().is_some());
    assert!(members[1].failure_id().is_some());
    assert_eq!(members[1].probabilities(), None);
    let document: Value = serde_json::from_str(&result.value().to_json().unwrap()).unwrap();
    assert_eq!(
        document["answer"]["questions"][0]["answer"]["probability"],
        0.9
    );
    assert_eq!(document["meta"]["failed_questions"], 1);
    relation_member_documents(result.value(), &document);
    let body: Value = serde_json::from_slice(&listener.requests()[0].body).unwrap();
    assert_eq!(body["state"]["context"], "Separate.");
    assert!(body["state"]["evidence"].is_object());
    let empty = engine
        .relate_complete_with(&ask, [], CallOptions::new().attempts(true))
        .unwrap();
    schema::call(&empty, "completeRelation");
    let document: Value = serde_json::from_str(&empty.value().to_json().unwrap()).unwrap();
    assert_eq!(document["meta"]["origin"], Value::Null);
    assert_eq!(document["meta"]["cached"], false);
    assert_eq!(document["meta"]["observations"], json!([]));
    assert_eq!(document["meta"]["attempts"], json!([]));
    assert!(document["meta"].get("answered_by").is_none());
    assert!(empty.facts().model().is_none());
    empty_metadata(empty.value().meta());
    assert_eq!(listener.count(), 1);
}

#[cfg(test)]
pub(super) fn recognized_response(body: &[u8]) -> Canned {
    let request: Value = serde_json::from_slice(body).unwrap();
    let mut answers = serde_json::Map::new();
    for (name, question) in request["questions"].as_object().unwrap() {
        if question["type"] == "noul" {
            answers.insert(name.clone(), json!({"type":"noul","noul":0.9}));
            continue;
        }
        let labels = question["criteria"].as_object().unwrap();
        let words = question["instructions"].as_str().unwrap();
        let picked = if labels.contains_key("SINGLE") {
            if words.contains("[[Ada]]") || words.contains("[[Acme]]") {
                "SINGLE"
            } else {
                "OUT"
            }
        } else if labels.contains_key("none of these") {
            if words.contains("[[Acme]]") {
                "organization"
            } else {
                "person"
            }
        } else {
            "Acme"
        };
        assert!(labels.contains_key(picked));
        let probabilities = labels
            .keys()
            .map(|label| (label.clone(), json!(u8::from(label == picked))))
            .collect::<serde_json::Map<_, _>>();
        answers.insert(
            name.clone(),
            json!({"type":"choice","probabilities":probabilities}),
        );
    }
    Canned::ok(&json!({"model":"fixed","answers":answers,"usage":{"input_tokens":887}}).to_string())
}

#[test]
fn all_recognition_stages_keep_context_separate_and_original_spans_with_partial_usage() {
    let listener = Listener::answering(recognized_response).unwrap();
    let folder = folder();
    let engine = Engine::builder()
        .base_url(listener.base())
        .unwrap()
        .model("fixed")
        .unwrap()
        .api_key("complete-private")
        .unwrap()
        .cache_at(&folder)
        .unwrap()
        .max_retries(0)
        .build()
        .unwrap();
    let ask = Recognize::builder()
        .kind(Kind::new("person", None).unwrap())
        .unwrap()
        .kind(Kind::new("organization", None).unwrap())
        .unwrap()
        .relation(RelationRule::one_way("works_for", "person", "organization").unwrap())
        .unwrap()
        .build()
        .unwrap();
    let found = engine
        .recognize_complete_with(
            &ask,
            "Ada met Acme.",
            CallOptions::new()
                .context("Original names only.")
                .attempts(true),
        )
        .unwrap();
    schema::call(&found, "completeRecognition");
    recognition_reading(found.value());
    let entities = found.value().value().entities();
    assert_eq!(
        (entities[0].text(), entities[0].start(), entities[0].end()),
        ("Ada", 0, 3)
    );
    assert_eq!(
        (entities[1].text(), entities[1].start(), entities[1].end()),
        ("Acme", 8, 12)
    );
    assert_eq!(found.value().probabilities().pieces().len(), 4);
    assert_eq!(found.value().probabilities().names().len(), 2);
    assert_eq!(found.value().probabilities().pairs().len(), 1);
    assert_eq!(found.value().identity().observations().len(), 8);
    assert_eq!(
        found.value().reported_usage().unwrap().input_tokens(),
        Some(2661)
    );
    assert_eq!(
        found.value().reported_usage().unwrap().output_tokens(),
        None
    );
    assert_eq!(listener.count(), 3);
    for request in listener.requests() {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(body["state"]["context"], "Original names only.");
        assert!(body["state"].get("evidence").is_some());
    }
    let cached = engine
        .recognize_complete_with(
            &ask,
            "Ada met Acme.",
            CallOptions::new().context("Original names only."),
        )
        .unwrap();
    assert_eq!(cached.value().answer_id(), found.value().answer_id());
    assert_eq!(cached.value().identity().origin(), Some(Origin::Cache));
    let empty = engine
        .recognize_complete_with(&ask, "", CallOptions::new().attempts(true))
        .unwrap();
    schema::call(&empty, "completeRecognition");
    let document: Value = serde_json::from_str(&empty.value().to_json().unwrap()).unwrap();
    assert_eq!(document["meta"]["origin"], Value::Null);
    assert_eq!(document["meta"]["cached"], false);
    assert_eq!(document["meta"]["question_sources"], json!([]));
    assert_eq!(listener.count(), 3);
    drop(engine);
    std::fs::remove_dir_all(folder).unwrap();
}

fn annotation_metadata(result: &thinkthen::CompleteAnnotated) {
    assert_eq!(result.meta().failed_questions(), 1);
    assert!(result.meta().question_sha256().is_none());
    assert!(result.meta().questions_sha256().is_some());
    assert_eq!(
        result.meta().usage().and_then(|usage| usage.input_tokens()),
        Some(9)
    );
}

fn empty_metadata(meta: thinkthen::ResultMetadata<'_>) {
    assert!(!meta.cached());
    assert_eq!(meta.requests_sent(), 0);
    assert_eq!(meta.attempts(), Some(&[][..]));
}

fn recognition_reading(result: &thinkthen::CompleteRecognized) {
    let reading = result.question();
    assert_eq!(
        reading
            .kinds()
            .map(|k| k.name().to_owned())
            .collect::<Vec<_>>(),
        ["person", "organization"]
    );
    assert_eq!(reading.threshold(), thinkthen::ResolvedThreshold::Cut(0.5));
    assert_eq!(
        reading.relation_threshold(),
        thinkthen::ResolvedThreshold::Cut(0.5)
    );
    assert_eq!(
        reading
            .relations()
            .map(|r| (
                r.name().to_owned(),
                r.reads().to_owned(),
                r.either(),
                r.single()
            ))
            .collect::<Vec<_>>(),
        [("works_for".into(), "works for".into(), false, false)]
    );
}
fn relation_reading(result: &thinkthen::CompleteRelated) {
    let reading = result.question();
    assert_eq!(reading.fields(), Some(("/name", "/kind")));
    assert_eq!(reading.threshold(), thinkthen::ResolvedThreshold::Cut(0.5));
    for member in result.members() {
        assert_eq!(member.question().kind(), thinkthen::QuestionKind::Decide);
        assert_eq!(
            member.threshold(),
            Some(thinkthen::ResolvedThreshold::Cut(0.5))
        );
        assert_eq!(member.question_sources().len(), 1);
        assert_eq!(member.observations().len(), 1);
    }
}

fn find_metadata(meta: thinkthen::ResultMetadata<'_>) {
    assert_eq!(
        meta.usage().and_then(|usage| usage.input_tokens()),
        Some(887)
    );
    assert_eq!(meta.usage().and_then(|usage| usage.output_tokens()), None);
    assert_eq!(meta.requests_sent(), 1);
    assert_eq!(meta.attempts().map(<[_]>::len), Some(1));
    assert!(!meta.cached());
    assert!(meta.context_sha256().is_some());
}

#[cfg(test)]
fn annotation_member_documents(result: &thinkthen::CompleteAnnotated, row: &Value) {
    for member in result.members() {
        let document = &row["answers"][member.name()];
        assert_eq!(
            document["question_sources"],
            json!(member.question_sources())
        );
        assert_eq!(document["observations"], json!(member.observations()));
        assert_eq!(document["usage"], json!(member.reported_usage()));
    }
}

#[cfg(test)]
fn relation_member_documents(result: &thinkthen::CompleteRelated, document: &Value) {
    for (member, document) in result
        .members()
        .zip(document["answer"]["questions"].as_array().unwrap())
    {
        assert_eq!(
            document["question_sources"],
            json!(member.question_sources())
        );
        assert_eq!(document["observations"], json!(member.observations()));
        assert_eq!(document.get("usage"), None);
        assert_eq!(document["threshold"], json!(0.5));
        assert_eq!(document["question"]["verb"], "decide");
    }
}

#[test]
fn rank_request_cutoff_keeps_complete_probabilities_positions_and_observations() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"fixed","answers":{"q1":{"type":"noul","noul":0.79},"q2":{"type":"noul","noul":0.8},"q3":{"type":"noul","noul":0.8}},"usage":{"input_tokens":9}}"#)).unwrap();
    let engine = engine(&listener);
    let request = thinkthen::Request::from_json(r#"{"schema":"thinkthen.request/1","call":{"function":"rank","question":{"kind":"text","text":"Best?"},"input":{"kind":"records","items":[{"original":{"kind":"text","text":"below"}},{"original":{"kind":"text","text":"first"}},{"original":{"kind":"text","text":"last"}}]},"options":{"threshold":0.8,"top":3,"batch":"max"}}}"#).unwrap().admit().unwrap();
    let thinkthen::RequestOutcome::Complete(call) = engine
        .execute_request(&request, thinkthen::RequestEnvironment::default())
        .unwrap()
    else {
        panic!("completed rank");
    };
    let document = serde_json::to_value(call.value()).unwrap();
    let rows = document.as_array().unwrap();
    assert_eq!(rows.len(), 2);
    for (at, input) in ["first", "last"].iter().enumerate() {
        assert_eq!(rows[at]["input"], *input);
        assert_eq!(rows[at]["value"], at + 1);
        assert_eq!(rows[at]["answer"]["probability"], 0.8);
        assert!(
            !rows[at]["meta"]["question_sources"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
    assert_eq!(listener.count(), 1);
}
