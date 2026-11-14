//! Named guide outcomes, independent of the wording of an example.
use super::{Loopback, Reply, case};
use serde_json::{Value, json};

impl Loopback {
    /// Serve a designated guide outcome through either backend wire format.
    ///
    /// The canonical form has one predicate, one choice and one scale. Targeted
    /// scenarios select those primitive shapes, never question or record text.
    /// # Errors
    /// Returns invalid input for an unknown name, or the listener's bind error.
    pub fn guide_scenario(name: &str) -> std::io::Result<Self> {
        let (target, outcome) = match name {
            "yes" => (None, "yes"),
            "relations" => (None, "relations"),
            "low-score" => (Some("score"), "low"),
            "no" => (None, "no"),
            "unsure" => (None, "unsure"),
            "failed" => (None, "failed"),
            "refund-failed" => (Some("predicate"), "failed"),
            "topic-failed" => (Some("choice"), "failed"),
            "urgency-failed" => (Some("score"), "failed"),
            "refund-unsure" => (Some("predicate"), "unsure"),
            "topic-unsure" => (Some("choice"), "unsure"),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "unknown guide scenario",
                ));
            }
        };
        Self::start(move |request| {
            if target.is_none() && outcome == "failed" {
                return Reply::json(503, &json!({"error":"unavailable"}));
            }
            case::reply_with(request, |question, openai| {
                let shape = match question["type"].as_str() {
                    Some("noul" | "predicate") => "predicate",
                    Some("score") => "score",
                    _ => "choice",
                };
                let selected = if target.is_none_or(|target| target == shape) {
                    outcome
                } else {
                    "yes"
                };
                if selected == "failed" {
                    return Value::Null;
                }
                if shape == "predicate" {
                    let probability = match selected {
                        "no" => 0.05,
                        "unsure" => 0.5,
                        _ => 0.95,
                    };
                    return case::answer(question, &json!({"probability": probability}), openai);
                }
                let labels: Vec<String> = if shape == "score" {
                    question[if openai { "levels" } else { "criteria" }]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|v| {
                            if openai {
                                v["label"].as_str()
                            } else {
                                v.as_str()
                            }
                        })
                        .map(str::to_owned)
                        .collect()
                } else {
                    question["criteria"]
                        .as_object()
                        .into_iter()
                        .flatten()
                        .map(|(key, _)| key.clone())
                        .collect()
                };
                let structural = (selected == "relations")
                    .then(|| structural_winner(question, &labels))
                    .flatten();
                let winner = if let Some(winner) = structural.as_ref() {
                    labels.iter().find(|label| *label == winner)
                } else if selected == "low" {
                    labels.first()
                } else if selected == "no" && labels.iter().any(|v| v == "none") {
                    labels.iter().find(|v| *v == "none")
                } else if shape == "score"
                    || (labels.iter().any(|v| v == "none") && selected != "no")
                {
                    labels.iter().rfind(|v| *v != "none")
                } else if selected == "no" {
                    labels.last()
                } else {
                    labels.first()
                };
                let distribution: serde_json::Map<String, Value> = labels
                    .iter()
                    .map(|label| {
                        let p = if selected == "unsure" {
                            1.0 / labels.len() as f64
                        } else if Some(label) == winner {
                            1.0
                        } else {
                            0.0
                        };
                        (label.clone(), json!(p))
                    })
                    .collect();
                case::answer(question, &json!({"distribution": distribution}), openai)
            })
        })
    }
}

/// A structural fixture, not a language model: recognize a leading title-cased
/// word and subsequent multiword title-cased spans. Kinds depend on span width.
/// Read only the JSON-quoted evidence and generated marker/label protocol, never
/// the question prose, record spelling, or request arrival order.
fn structural_winner(question: &Value, labels: &[String]) -> Option<String> {
    let quoted = question["instructions"]
        .as_str()?
        .strip_prefix("The text is ")?;
    let text = serde_json::Deserializer::from_str(quoted)
        .into_iter::<String>()
        .next()?
        .ok()?;
    let (before, rest) = text.split_once("[[")?;
    let (marked, after) = rest.split_once("]]")?;
    if labels.iter().any(|label| label == "BEGIN") {
        let previous = before
            .split_whitespace()
            .next_back()
            .is_some_and(title_word);
        let next = after
            .split_whitespace()
            .next()
            .map(|word| word.trim_end_matches(|ch: char| !ch.is_alphabetic()))
            .is_some_and(title_word);
        let label = if !title_word(marked) {
            "OUT"
        } else if previous && next {
            "INSIDE"
        } else if previous {
            "END"
        } else if next {
            "BEGIN"
        } else if before.is_empty() {
            "SINGLE"
        } else {
            "OUT"
        };
        return Some(label.to_owned());
    }
    if labels.iter().any(|label| label == "none of these") {
        return Some(
            if marked.split_whitespace().count() > 1 {
                "organization"
            } else {
                "person"
            }
            .to_owned(),
        );
    }
    None
}

fn title_word(text: &str) -> bool {
    text.chars().next().is_some_and(char::is_uppercase) && text.chars().all(char::is_alphabetic)
}
