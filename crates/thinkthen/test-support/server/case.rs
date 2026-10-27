//! One canned rule set answers both backend wire formats.
use super::{Loopback, Reply};
use serde_json::{Map, Value, json};
impl Loopback {
    /// Serve shared conformance rules through either backend's wire format.
    /// Semantic rules contain probability or distribution; existing answer rules remain supported.
    /// # Errors
    /// Returns the listener's bind error.
    pub fn case(case: &Value) -> std::io::Result<Self> {
        let rules = field(case, "replies")
            .as_array()
            .cloned()
            .unwrap_or_default();
        Self::start(move |request| reply(&rules, request))
    }
}

pub(crate) fn reply(rules: &[Value], request: &super::Request) -> Reply {
    let body = request.json().unwrap_or_default();
    let openai = field(&body, "questions").is_array();
    let questions: Vec<(String, Value)> =
        if let Some(questions) = field(&body, "questions").as_array() {
            questions
                .iter()
                .enumerate()
                .map(|(i, q)| (i.to_string(), normalize(q)))
                .collect()
        } else {
            field(&body, "questions")
                .as_object()
                .into_iter()
                .flatten()
                .map(|(k, q)| (k.clone(), q.clone()))
                .collect()
        };
    if let Some(rule) = rules.iter().find(|rule| {
        rule.get("status").is_some() && questions.iter().any(|(_, q)| matches(rule, q))
    }) {
        let status = field(rule, "status")
            .as_u64()
            .and_then(|n| u16::try_from(n).ok())
            .unwrap_or(500);
        return Reply::json(status, field(rule, "body"));
    }
    let answers: Vec<_> = questions
        .iter()
        .map(|(key, q)| {
            let rule = rules.iter().find(|rule| {
                (rule.get("answer").is_some()
                    || rule.get("probability").is_some()
                    || rule.get("distribution").is_some())
                    && matches(rule, q)
            });
            (
                key.clone(),
                rule.map_or(Value::Null, |rule| answer(q, rule, openai)),
            )
        })
        .collect();
    let answers = if openai {
        Value::Array(answers.into_iter().map(|(_, v)| v).collect())
    } else {
        Value::Object(answers.into_iter().collect())
    };
    Reply::json(200, &json!({"model":body.get("model"),"answers":answers}))
}
fn normalize(question: &Value) -> Value {
    let mut normalized = question.clone();
    let criteria: Map<String, Value> = field(question, "choices")
        .as_array()
        .into_iter()
        .flatten()
        .map(|choice| {
            (
                field(choice, "value")
                    .as_str()
                    .map_or_else(|| field(choice, "value").to_string(), str::to_owned),
                field(choice, "description").clone(),
            )
        })
        .collect();
    if !criteria.is_empty()
        && let Some(fields) = normalized.as_object_mut()
    {
        fields.insert("criteria".to_owned(), Value::Object(criteria));
    }
    normalized
}
fn matches(rule: &Value, question: &Value) -> bool {
    field(rule, "contains").as_str().is_none_or(|text| {
        field(question, "instructions")
            .as_str()
            .is_some_and(|words| words.contains(text))
    }) && field(rule, "criterion")
        .as_str()
        .is_none_or(|name| field(question, "criteria").get(name).is_some())
}
fn answer(question: &Value, rule: &Value, openai: bool) -> Value {
    if !openai && rule.get("answer").is_some() {
        return field(rule, "answer").clone();
    }
    let raw = field(rule, "answer");
    let probability = rule.get("probability").or_else(|| raw.get("noul"));
    if let Some(probability) = probability {
        return if openai {
            json!({"type":"predicate","probability":probability})
        } else {
            json!({"type":"noul","noul":probability})
        };
    }
    let distribution = rule
        .get("distribution")
        .or_else(|| raw.get("probabilities"));
    let Some(distribution) = distribution.and_then(Value::as_object) else {
        return raw.clone();
    };
    let score = field(question, "type") == "score";
    if !openai {
        let probabilities = if score {
            field(question, "criteria")
                .as_array()
                .into_iter()
                .flatten()
                .enumerate()
                .map(|(i, label)| {
                    (
                        i.to_string(),
                        distribution
                            .get(label.as_str().unwrap_or_default())
                            .cloned()
                            .unwrap_or(Value::Null),
                    )
                })
                .collect()
        } else {
            distribution.clone()
        };
        return json!({"type":if score{"score"}else{"choice"},"probabilities":probabilities});
    }
    let probabilities: Vec<Value> = if score {
        field(question, "levels")
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(i, level)| {
                let label = field(level, "label").as_str().unwrap_or_default();
                let probability = distribution
                    .get(label)
                    .or_else(|| distribution.get(&i.to_string()))
                    .unwrap_or(&Value::Null);
                json!({"label":label,"probability":probability})
            })
            .collect()
    } else {
        field(question, "choices")
            .as_array()
            .into_iter()
            .flatten()
            .map(|choice| {
                let value = &field(choice, "value");
                let key = value
                    .as_str()
                    .map_or_else(|| value.to_string(), str::to_owned);
                json!({"value":value,"probability":distribution.get(&key)})
            })
            .collect()
    };
    json!({"type":if score{"score"}else{"choice"},"probabilities":probabilities})
}

fn field<'a>(value: &'a Value, name: &str) -> &'a Value {
    value.get(name).unwrap_or(&Value::Null)
}
