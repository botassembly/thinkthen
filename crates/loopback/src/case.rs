//! Canned System One replies shared by all conformance drivers.

use crate::{Loopback, Reply};
use serde_json::{Map, Value, json};

impl Loopback {
    /// Serve a conformance case's canned backend rules.
    /// Rules match instruction substrings and optional criterion names, in order.
    /// A status rule fails a whole matching request; answer rules match each question.
    ///
    /// # Errors
    /// Returns the loopback listener's bind error.
    pub fn case(case: &Value) -> std::io::Result<Self> {
        let rules = case["replies"].as_array().cloned().unwrap_or_default();
        Self::start(move |request| {
            let body = request.json().unwrap_or_default();
            let questions = body.get("questions").and_then(Value::as_object);
            if let Some(rule) = rules.iter().find(|rule| {
                rule.get("status").is_some()
                    && questions
                        .into_iter()
                        .flatten()
                        .any(|(_, q)| matches(rule, q))
            }) {
                let status = rule["status"]
                    .as_u64()
                    .and_then(|n| u16::try_from(n).ok())
                    .unwrap_or(500);
                return Reply::json(status, &rule["body"]);
            }
            let answers: Map<String, Value> = questions
                .into_iter()
                .flatten()
                .map(|(key, q)| {
                    let answer = rules
                        .iter()
                        .find(|rule| rule.get("answer").is_some() && matches(rule, q))
                        .map_or(Value::Null, |rule| rule["answer"].clone());
                    (key.clone(), answer)
                })
                .collect();
            Reply::json(200, &json!({"model":body.get("model"), "answers":answers}))
        })
    }
}

fn matches(rule: &Value, question: &Value) -> bool {
    rule["contains"].as_str().is_none_or(|text| {
        question["instructions"]
            .as_str()
            .is_some_and(|words| words.contains(text))
    }) && rule["criterion"]
        .as_str()
        .is_none_or(|name| question["criteria"].get(name).is_some())
}
