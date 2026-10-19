//! Saved recognition exchanges shared by library and command behavior tests.
use conformance_backend::Canned;
use serde_json::{Value, json};

#[cfg(test)]
pub(crate) fn response(body: &[u8]) -> Canned {
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
