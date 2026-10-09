//! Compare the preserved legacy judgment fields while checking added identities.
use serde_json::Value;
#[cfg(test)]
pub(crate) fn judgment(mut row: Value) -> Value {
    row = judgment_with_member_facts(row);
    if let Some(members) = row.get_mut("answers").and_then(Value::as_object_mut) {
        for member in members.values_mut() {
            legacy_member(member, &["question_sources", "observations", "usage"]);
        }
    }
    if let Some(members) = row
        .get_mut("answer")
        .and_then(|answer| answer.get_mut("questions"))
        .and_then(Value::as_array_mut)
    {
        for member in members {
            legacy_member(
                member,
                &[
                    "question_sources",
                    "observations",
                    "usage",
                    "answer",
                    "question",
                    "threshold",
                ],
            );
        }
    }
    row
}

#[cfg(test)]
fn legacy_member(member: &mut Value, fields: &[&str]) {
    let member = member.as_object_mut().unwrap();
    if let Some(usage) = member.get("usage") {
        assert!(usage.is_object());
    }
    assert!(member["question"].is_object());
    assert!(member["threshold"].is_number() || member["threshold"].is_null());
    for field in fields {
        member.remove(*field);
    }
}

/// Compare all presented member facts except validated run-specific identities.
#[cfg(test)]
pub(crate) fn judgment_with_member_facts(mut row: Value) -> Value {
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
            normalize_member_observations(member);
            if let Some(id) = member.remove("answer_id") {
                assert!(thinkthen::AnswerId::new(id.as_str().unwrap()).is_ok());
            } else {
                let id = member.remove("failure_id").unwrap();
                assert!(thinkthen::FailureId::new(id.as_str().unwrap()).is_ok());
            }
        }
    }
    if let Some(members) = row
        .get_mut("answer")
        .and_then(|answer| answer.get_mut("questions"))
        .and_then(Value::as_array_mut)
    {
        for member in members {
            let member = member.as_object_mut().unwrap();
            normalize_member_observations(member);
            if let Some(id) = member.remove("answer_id") {
                assert!(thinkthen::AnswerId::new(id.as_str().unwrap()).is_ok());
                assert!(member["answer"].is_object());
            } else {
                let id = member.remove("failure_id").unwrap();
                assert!(thinkthen::FailureId::new(id.as_str().unwrap()).is_ok());
            }
        }
    }
    row
}

#[cfg(test)]
fn normalize_member_observations(member: &mut serde_json::Map<String, Value>) {
    for source in member["question_sources"].as_array().unwrap() {
        assert!(matches!(
            source["origin"].as_str().unwrap(),
            "live" | "cache" | "replay" | "proxy" | "memory"
        ));
        assert!(!source["answered_by"].as_str().unwrap().is_empty());
        if let Some(size) = source.get("batch_size") {
            assert!(size.as_u64().unwrap() > 0);
        }
    }
    for observation in member["observations"].as_array_mut().unwrap() {
        let observation = observation.as_object_mut().unwrap();
        let field = match (
            observation.get("observation_id"),
            observation.get("failure_id"),
        ) {
            (Some(id), None) => {
                assert!(thinkthen::ObservationId::new(id.as_str().unwrap()).is_ok());
                "observation_id"
            }
            (None, Some(id)) => {
                assert!(thinkthen::FailureId::new(id.as_str().unwrap()).is_ok());
                "failure_id"
            }
            _ => panic!("member observation needs exactly one native identity"),
        };
        // Keep the observation variant, order and any other facts comparable.
        observation.insert(field.to_owned(), Value::String("0".repeat(64)));
    }
}

/// Retrieval retains member facts and identities while changing the actual origin.
#[cfg(test)]
pub(crate) fn retrieved_members(mut members: Value, origin: &str) -> Value {
    assert!(matches!(origin, "replay" | "cache"));
    let entries: Vec<&mut Value> = match &mut members {
        Value::Object(map) => map.values_mut().collect(),
        Value::Array(array) => array.iter_mut().collect(),
        _ => panic!("expected an annotation map or relation member array"),
    };
    for member in entries {
        for source in member["question_sources"].as_array_mut().unwrap() {
            assert_eq!(source["origin"], "live");
            source["origin"] = origin.into();
        }
    }
    members
}
