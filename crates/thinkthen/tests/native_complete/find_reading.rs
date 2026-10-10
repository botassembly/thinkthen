use super::*;
use thinkthen::QuestionKind;

#[test]
fn saved_find_reading_uses_actual_ordered_units_and_preserves_model_profile_and_full_answer() {
    let listener = Listener::answering(|_| Canned::ok(r#"{"model":"actual-model","answers":{"q1":{"type":"choice","probabilities":{"u001":0.2,"u002":0.8}}},"usage":{"input_tokens":887}}"#)).unwrap();
    let folder = folder();
    let path = folder.join("find.json");
    std::fs::write(&path, r#"{"find":{"z":"Which unit?","a":["Read both."]},"on":"","model":"saved-model","profile":"saved-profile"}"#).unwrap();
    let question = Question::load_find(&path).unwrap();
    assert_eq!(question.kind(), QuestionKind::Find);
    let engine = engine(&listener);
    let call = engine
        .find_complete_with(&question, ["first", "second"], CallOptions::new())
        .unwrap();
    let result = call.value();
    assert_eq!(result.selected(), Some(&"second"));
    assert_eq!(result.raw_pick(), "u002");
    assert_eq!(result.question().profile(), Some("saved-profile"));
    assert_eq!(
        result.question().text().to_json().unwrap(),
        r#"{"z":"Which unit?","a":["Read both."]}"#
    );
    assert_eq!(result.meta().identity().answered_by(), Some("actual-model"));
    assert_eq!(result.meta().usage().unwrap().input_tokens(), Some(887));
    assert_eq!(result.meta().usage().unwrap().output_tokens(), None);
    assert_eq!(listener.count(), 1);
    assert_eq!(
        std::str::from_utf8(&listener.requests()[0].body).unwrap(),
        r#"{"state":"[{\"id\":\"u001\",\"evidence\":\"first\"},{\"id\":\"u002\",\"evidence\":\"second\"}]","model":"saved-model","questions":{"q1":{"type":"choice","instructions":{"z":"Which unit?","a":["Read both."]},"criteria":{"u001":null,"u002":null}}}}"#
    );
    let doc: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
    assert_eq!(doc["question"]["profile"], "saved-profile");
    assert_eq!(doc["value"], "second");
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn saved_find_refuses_authored_cut_candidates_and_local_invalid_files_before_sending() {
    let listener = Listener::answering(|_| Canned::ok("unused")).unwrap();
    for text in [
        r#"{"find":"Which?","threshold":0.8}"#,
        r#"{"find":"Which?","options":["a","b"]}"#,
        r#"{"find":"Which?","batch":1}"#,
        r#"{"find":"Which?","on":"/body"}"#,
        r#"{"find":"Which?","decide":"Wrong?"}"#,
        r#"{"find":"One?","find":"Two?"}"#,
    ] {
        let error = Question::find_from_json(text).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Usage);
    }
    let folder = folder();
    let path = folder.join("invalid.json");
    std::fs::write(&path, r#"{"find":"private-question","threshold":0.8}"#).unwrap();
    let error = Question::load_find(&path).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Local);
    assert!(!format!("{error} {error:?}").contains("private-question"));
    assert_eq!(listener.count(), 0);
    std::fs::remove_dir_all(folder).unwrap();
}

#[test]
fn single_unit_with_none_selects_the_original_or_none_and_reuses_cached_answers() {
    for (probabilities, selected) in [
        (r#"{"u001":0.9,"none":0.1}"#, true),
        (r#"{"u001":0.1,"none":0.9}"#, false),
    ] {
        let response = format!(
            r#"{{"model":"fixed","answers":{{"q1":{{"type":"choice","probabilities":{probabilities}}}}}}}"#
        );
        let listener = Listener::answering(move |_| Canned::ok(&response)).unwrap();
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
        let question = Question::find("Which?").unwrap().offering_none().unwrap();
        question.admit_find_units([Ok("Original.")]).unwrap();
        let original = r#"{"body":"Original.","private":1}"#;
        let records = || {
            let reading = thinkthen::RecordReading::new(&["/body"], None, None).unwrap();
            let mut record = reading
                .compose(thinkthen::RawRecord::json(original).unwrap())
                .unwrap();
            record.original = record.original.with_location(
                thinkthen::SourceLocation::new("unit.jsonl".into(), Some(42), Some(42)).unwrap(),
            );
            [Ok(record)]
        };
        let call = engine
            .try_find_records_complete_with(&question, records(), CallOptions::new())
            .unwrap();
        let result = call.value();
        assert_eq!(result.selected().is_some(), selected);
        if selected {
            let record = result.selected().unwrap();
            assert_eq!(
                record.original().content().unwrap().to_json().unwrap(),
                original
            );
            assert_eq!(record.location().unwrap().first_line(), Some(42));
            assert_eq!(record.location().unwrap().file(), "unit.jsonl");
        }
        assert_eq!(result.candidates().len(), 2);
        assert!(result.candidates()[0].input().is_some());
        assert!(result.candidates()[1].is_none());
        let document: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
        assert_eq!(
            document["answer"]["probabilities"],
            serde_json::from_str::<Value>(probabilities).unwrap()
        );
        let cached = engine
            .try_find_records_complete_with(&question, records(), CallOptions::new())
            .unwrap();
        assert_eq!(cached.value().answer_id(), result.answer_id());
        assert_eq!(cached.value().identity().origin(), Some(Origin::Cache));
        assert_eq!(cached.facts().requests_sent(), 0);
        assert_eq!(listener.count(), 1);
        assert_eq!(
            serde_json::from_slice::<Value>(&listener.requests()[0].body).unwrap(),
            json!({"model":"fixed","state":r#"[{"id":"u001","evidence":"Original."}]"#,"questions":{"q1":{"type":"choice","instructions":"Which?","criteria":{"u001":null,"none":null}}}})
        );
        drop(engine);
        std::fs::remove_dir_all(folder).unwrap();
    }
}
