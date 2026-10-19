//! Compare complete current results while validating run-specific identities.
use serde_json::Value;

/// Keep every field; normalize only native identities and measured durations.
#[cfg(test)]
pub(crate) fn judgment(mut row: Value) -> Value {
    assert_eq!(row["schema"], "thinkthen.result/2");
    identity(&mut row, "answer_id");
    let meta = &mut row["meta"];
    assert!(meta["origin"].is_string() || meta["origin"].is_null());
    observations(meta);
    for attempt in meta["attempts"].as_array_mut().unwrap() {
        identity(attempt, "sdk_request_id");
        assert!(attempt["wall_ms"].as_u64().is_some());
        attempt["wall_ms"] = 0.into();
    }
    if let Some(members) = row.get_mut("answers").and_then(Value::as_object_mut) {
        for member in members.values_mut() {
            member_facts(member);
        }
    }
    if let Some(members) = row
        .get_mut("answer")
        .and_then(|answer| answer.get_mut("questions"))
        .and_then(Value::as_array_mut)
    {
        for member in members {
            member_facts(member);
        }
    }
    row
}

/// Scheduling changes send order and global attempt ordinals, but no send facts.
#[cfg(test)]
pub(crate) fn scheduled_judgment(row: Value) -> Value {
    let mut row = judgment(row);
    let attempts = row["meta"]["attempts"].as_array_mut().unwrap();
    attempts.sort_by_key(|attempt| attempt["request_sha256"].as_str().unwrap().to_owned());
    for (index, attempt) in attempts.iter_mut().enumerate() {
        assert!(attempt["ordinal"].as_u64().unwrap() > 0);
        attempt["ordinal"] = (index + 1).into();
    }
    row
}

#[cfg(test)]
fn member_facts(member: &mut Value) {
    if let Some(usage) = member.get("usage") {
        assert!(usage.is_object());
    }
    assert!(member["question"].is_object());
    assert!(member["threshold"].is_number() || member["threshold"].is_null());
    observations(member);
    if member.get("answer_id").is_some() {
        identity(member, "answer_id");
        assert!(member["answer"].is_object());
    } else {
        identity(member, "failure_id");
    }
}

#[cfg(test)]
fn identity(value: &mut Value, field: &str) {
    let id = value[field].as_str().unwrap();
    let valid = match field {
        "answer_id" => thinkthen::AnswerId::new(id).is_ok(),
        "failure_id" => thinkthen::FailureId::new(id).is_ok(),
        "observation_id" => thinkthen::ObservationId::new(id).is_ok(),
        "sdk_request_id" => thinkthen::SdkRequestId::new(id).is_ok(),
        _ => panic!("unknown native identity"),
    };
    assert!(valid, "invalid {field}");
    value[field] = Value::String("0".repeat(64));
}

#[cfg(test)]
fn observations(value: &mut Value) {
    for source in value["question_sources"].as_array().unwrap() {
        assert!(matches!(
            source["origin"].as_str().unwrap(),
            "live" | "cache" | "replay" | "proxy" | "memory"
        ));
        assert!(!source["answered_by"].as_str().unwrap().is_empty());
        if let Some(size) = source.get("batch_size") {
            assert!(size.as_u64().unwrap() > 0);
        }
    }
    for observation in value["observations"].as_array_mut().unwrap() {
        let field = match (
            observation.get("observation_id"),
            observation.get("failure_id"),
        ) {
            (Some(_), None) => "observation_id",
            (None, Some(_)) => "failure_id",
            _ => panic!("observation needs exactly one native identity"),
        };
        identity(observation, field);
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
