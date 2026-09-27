//! Step 3 of `recognize`: one yes/no question per pair a rule allows, about
//! what the text itself states (ADR 0056).

use serde::Serialize;

use super::{RelationEdge, RelationPlanError, RelationRule, entity_id, reaches_cut};
use crate::core::json::Json;
use crate::core::recognize::RecognizedName;
use crate::core::{Answer, Evidence, Question, QuestionText};

/// The pair questions of one text: the evidence they share and each pair asked.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StatedPlan {
    pub(crate) evidence: Evidence,
    pub(crate) questions: Vec<Question>,
    pub(crate) pairs: Vec<StatedPair>,
}

/// One asked pair: the rule's place and the two names' places.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StatedPair {
    pub(crate) rule: usize,
    pub(crate) source: usize,
    pub(crate) target: usize,
}

#[derive(Serialize)]
struct State<'a> {
    evidence: &'a str,
    entities: Vec<StateEntity<'a>>,
}

#[derive(Serialize)]
struct StateEntity<'a> {
    id: String,
    name: &'a str,
    kind: &'a str,
}

/// Whether a rule side, `*` or one kind, admits `kind`.
fn admits(side: &str, kind: &str) -> bool {
    side == "*" || side == kind
}

/// Plan the pair questions, or `None` when no pair is allowed. Names with
/// equal text and kind are asked once, as the first.
pub(crate) fn plan_stated(
    text: &str,
    names: &[RecognizedName],
    rules: &[RelationRule],
) -> Result<Option<StatedPlan>, RelationPlanError> {
    let mut asked: Vec<usize> = Vec::new();
    for (place, name) in names.iter().enumerate() {
        let repeated = asked.iter().any(|held| {
            names
                .get(*held)
                .is_some_and(|other| other.text == name.text && other.kind == name.kind)
        });
        let named = rules
            .iter()
            .any(|rule| admits(&rule.source, &name.kind) || admits(&rule.target, &name.kind));
        if !repeated && named {
            asked.push(place);
        }
    }
    let kind = |at: usize| names.get(at).map_or("", |name| name.kind.as_str());
    let mut questions = Vec::new();
    let mut pairs = Vec::new();
    for (rule_place, rule) in rules.iter().enumerate() {
        for (left, source) in asked.iter().enumerate() {
            for (right, target) in asked.iter().enumerate() {
                let (from, to) = (kind(*source), kind(*target));
                let allowed = if rule.either {
                    left < right
                        && ((admits(&rule.source, from) && admits(&rule.target, to))
                            || (admits(&rule.source, to) && admits(&rule.target, from)))
                } else {
                    left != right && admits(&rule.source, from) && admits(&rule.target, to)
                };
                if !allowed {
                    continue;
                }
                let (one, two, reads) = (entity_id(left), entity_id(right), &rule.reads);
                let words = if rule.either {
                    format!(
                        "Does the text itself state that {one} {reads} {two}, or that {two} {reads} {one}?"
                    )
                } else {
                    format!("Does the text itself state that {one} {reads} {two}?")
                };
                questions.push(Question::Decide {
                    text: QuestionText::new(words).map_err(|_| RelationPlanError)?,
                    yes: None,
                    no: None,
                });
                pairs.push(StatedPair {
                    rule: rule_place,
                    source: *source,
                    target: *target,
                });
            }
        }
    }
    if questions.is_empty() {
        return Ok(None);
    }
    let state = State {
        evidence: text,
        entities: asked
            .iter()
            .enumerate()
            .filter_map(|(place, at)| {
                let name = names.get(*at)?;
                Some(StateEntity {
                    id: entity_id(place),
                    name: &name.text,
                    kind: &name.kind,
                })
            })
            .collect(),
    };
    let written = serde_json::to_string(&state).map_err(|_| RelationPlanError)?;
    let value = Json::parse(&written).map_err(|_| RelationPlanError)?;
    Ok(Some(StatedPlan {
        evidence: Evidence::structured(value).map_err(|_| RelationPlanError)?,
        questions,
        pairs,
    }))
}

/// The edges whose yes probability reaches `cut`, in question order.
pub(crate) fn stated_edges(
    names: &[RecognizedName],
    rules: &[RelationRule],
    pairs: &[StatedPair],
    answers: &[Answer],
    cut: f64,
) -> Vec<RelationEdge<RecognizedName>> {
    pairs
        .iter()
        .zip(answers)
        .filter_map(|(pair, answer)| {
            let probability = answer.yes().filter(|value| reaches_cut(*value, cut))?;
            Some(RelationEdge {
                relation: rules.get(pair.rule)?.name.clone(),
                source: names.get(pair.source)?.clone(),
                target: names.get(pair.target)?.clone(),
                probability,
            })
        })
        .collect()
}
