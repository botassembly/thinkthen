//! Read-only recording metadata comes from admitted execution values.

use super::*;
use crate::{Answer, Outcome};
use serde_json::json;

/// An admitted function's wording and fixed answer domain, without its threshold.
#[derive(Debug, Clone)]
pub struct QuestionDefinition {
    function: &'static str,
    domain: Value,
    wording: Value,
    members: BTreeMap<String, Self>,
}

impl QuestionDefinition {
    /// The existing function name.
    pub const fn function(&self) -> &'static str {
        self.function
    }

    /// Fixed answer labels and ordered scale levels; no wording or threshold.
    pub const fn domain(&self) -> &Value {
        &self.domain
    }

    /// Exact admitted wording, descriptions and presentation order.
    pub const fn wording(&self) -> &Value {
        &self.wording
    }

    /// Check a curated answer against the same typed answer used by execution.
    /// `evidence` is the materialized selected text; this method applies no pointer.
    ///
    /// # Errors
    /// Returns the shared question error for a different shape or answer domain.
    pub fn validate_answer(&self, answer: &Answer, evidence: &str) -> Result<(), Error> {
        let labels = |key: &str, value: &str| {
            self.domain
                .get(key)
                .and_then(Value::as_array)
                .is_some_and(|values| values.iter().any(|label| label.as_str() == Some(value)))
        };
        let valid = match (self.function, answer) {
            ("decide" | "filter", Answer::Bool(_)) => true,
            ("choose", Answer::Label(label)) => labels("options", label),
            ("tag", Answer::Labels(values)) => values.iter().enumerate().all(|(i, label)| {
                labels("labels", label) && !values.iter().take(i).any(|other| other == label)
            }),
            ("score", Answer::Number(value)) => value.is_finite() && (0.0..=1.0).contains(value),
            ("rank", Answer::Position(value)) => *value > 0,
            ("find", Answer::Found(found)) => found
                .as_ref()
                .is_none_or(|v| crate::Find::admits_found(evidence, v)),
            ("recognize", Answer::Names(names)) => {
                names
                    .windows(2)
                    .all(|pair| matches!(pair, [left, right] if left.end <= right.start))
                    && names.iter().all(|name| {
                        span(evidence, &name.text, name.start, name.end)
                            && labels("kinds", &name.kind)
                    })
            }
            ("relate", Answer::Relations(relations)) => {
                relations.iter().enumerate().all(|(position, relation)| {
                    let distinct = relation.source.text != relation.target.text
                        || relation.source.kind != relation.target.kind;
                    let repeated = relations.iter().take(position).any(|prior| {
                        prior.relation == relation.relation
                            && prior.source.text == relation.source.text
                            && prior.source.kind == relation.source.kind
                            && prior.target.text == relation.target.text
                            && prior.target.kind == relation.target.kind
                    });
                    distinct
                        && (!relation.either || relation.source.start < relation.target.start)
                        && (relation.source.end <= relation.target.start
                            || relation.target.end <= relation.source.start)
                        && !repeated
                        && span(
                            evidence,
                            &relation.source.text,
                            relation.source.start,
                            relation.source.end,
                        )
                        && span(
                            evidence,
                            &relation.target.text,
                            relation.target.start,
                            relation.target.end,
                        )
                        && self
                            .domain
                            .get("rules")
                            .and_then(Value::as_array)
                            .is_some_and(|rules| {
                                rules.iter().any(|rule| {
                                    rule.get("name").and_then(Value::as_str)
                                        == Some(&relation.relation)
                                        && ((rule.get("source").and_then(Value::as_str)
                                            == Some(&relation.source.kind)
                                            && rule.get("target").and_then(Value::as_str)
                                                == Some(&relation.target.kind))
                                            || (relation.either
                                                && rule.get("source").and_then(Value::as_str)
                                                    == Some(&relation.target.kind)
                                                && rule.get("target").and_then(Value::as_str)
                                                    == Some(&relation.source.kind)))
                                        && rule.get("either").and_then(Value::as_bool)
                                            == Some(relation.either)
                                })
                            })
                })
            }
            ("annotate", Answer::Annotations(entries)) => {
                entries.len() == self.members.len()
                    && self.members.iter().all(|(name, definition)| {
                        matches!(entries.get(name), Some(Outcome::Answered { answer, .. })
                        if definition.validate_answer(answer, evidence).is_ok())
                    })
            }
            _ => false,
        };
        if valid {
            Ok(())
        } else {
            Err(invalid(
                "answer does not match the admitted function domain",
            ))
        }
    }
}

impl SdkCall {
    /// Capture recording metadata without changing engine execution identity.
    ///
    /// # Errors
    /// Returns a question error if admitted metadata cannot be serialized.
    pub fn definition(&self) -> Result<QuestionDefinition, Error> {
        match &self.question {
            Question::Ordinary(function) => definition(function.name(), function),
            Question::Rank(crate::Rank::Decide(q)) => {
                definition("rank", &ToolboxFunction::Decide(q.clone()))
            }
            Question::Rank(crate::Rank::Score(q)) => {
                definition("rank", &ToolboxFunction::Score(q.clone()))
            }
            Question::Annotate(question) => {
                let members = question
                    .definitions()
                    .map(|(name, function)| {
                        Ok((name.to_owned(), definition(function.name(), function)?))
                    })
                    .collect::<Result<BTreeMap<_, _>, Error>>()?;
                Ok(QuestionDefinition {
                    function: "annotate",
                    domain: Value::Object(
                        members
                            .iter()
                            .map(|(name, d)| (name.clone(), json!([d.function, d.domain])))
                            .collect(),
                    ),
                    wording: Value::Object(
                        members
                            .iter()
                            .map(|(name, d)| (name.clone(), d.wording.clone()))
                            .collect(),
                    ),
                    members,
                })
            }
        }
    }
}

fn definition(
    function: &'static str,
    question: &ToolboxFunction,
) -> Result<QuestionDefinition, Error> {
    let mut wording =
        serde_json::to_value(question.definition()).map_err(|e| invalid(e.to_string()))?;
    if let Some(object) = wording.as_object_mut() {
        object.remove("threshold");
    }
    let mut domain = serde_json::Map::new();
    for key in ["options", "labels", "levels", "kinds", "rules"] {
        if let Some(value) = wording.get(key) {
            let mut value = value.clone();
            if let Some(values) = value.as_array_mut() {
                for value in values.iter_mut() {
                    if key == "options" || key == "kinds" {
                        *value = value.get("label").cloned().unwrap_or(Value::Null);
                    } else if key == "rules"
                        && let Some(rule) = value.as_object_mut()
                    {
                        rule.remove("reads");
                    }
                }
                if key != "levels" {
                    values.sort_by_key(Value::to_string);
                }
                if key == "kinds" && values.is_empty() {
                    values.push(json!("ENTITY"));
                }
            }
            domain.insert(key.to_owned(), value);
        }
    }
    Ok(QuestionDefinition {
        function,
        domain: Value::Object(domain),
        wording,
        members: BTreeMap::new(),
    })
}

fn span(evidence: &str, text: &str, start: usize, end: usize) -> bool {
    start < end
        && end <= evidence.chars().count()
        && evidence
            .chars()
            .skip(start)
            .take(end - start)
            .eq(text.chars())
}
