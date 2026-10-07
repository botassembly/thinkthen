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
