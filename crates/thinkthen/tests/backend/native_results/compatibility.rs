//! Compare the preserved legacy judgment fields while checking added identities.
use serde_json::Value;
#[cfg(test)]
pub(crate) fn judgment(mut row: Value) -> Value {
    assert_eq!(row["schema"], "thinkthen.result/2");
    let id = row.as_object_mut().unwrap().remove("answer_id").unwrap();
    assert!(thinkthen::AnswerId::new(id.as_str().unwrap()).is_ok());
    let meta = row["meta"].as_object_mut().unwrap();
    assert!(meta["origin"].is_string() || meta["origin"].is_null());
    assert!(meta["question_sources"].is_array());
    assert!(meta["observations"].is_array());
    for field in [
        "origin",
        "answered_by",
        "question_sources",
        "observations",
        "attempts",
    ] {
        meta.remove(field);
    }
    if let Some(members) = row.get_mut("answers").and_then(Value::as_object_mut) {
        for member in members.values_mut() {
            let member = member.as_object_mut().unwrap();
            if let Some(id) = member.remove("answer_id") {
                assert!(thinkthen::AnswerId::new(id.as_str().unwrap()).is_ok());
            } else {
                let id = member.remove("failure_id").unwrap();
                assert!(thinkthen::FailureId::new(id.as_str().unwrap()).is_ok());
            }
        }
    }
    if let Some(members) = row["answer"]
        .get_mut("questions")
        .and_then(Value::as_array_mut)
    {
        for member in members {
            let member = member.as_object_mut().unwrap();
            if let Some(id) = member.remove("answer_id") {
                assert!(thinkthen::AnswerId::new(id.as_str().unwrap()).is_ok());
                assert!(member.remove("answer").is_some());
            } else {
                let id = member.remove("failure_id").unwrap();
                assert!(thinkthen::FailureId::new(id.as_str().unwrap()).is_ok());
            }
        }
    }
    row
}
