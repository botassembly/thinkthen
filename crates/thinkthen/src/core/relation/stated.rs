//! Step 3 of `recognize`: one yes/no question per pair a rule allows, about
//! what the text itself states (ADR 0056).

use super::{RelationEdge, RelationPlanError, RelationRule, entity_id, push_edge, state_evidence};
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

/// Whether a rule side, `*` or one kind, admits `kind`.
fn admits(side: &str, kind: &str) -> bool {
    side == "*" || side == kind
}

/// Whether `rule` asks about the pair of asked names at `left` and `right`,
/// each with its kind. An `either` rule asks each unordered pair once.
fn allowed(rule: &RelationRule, left: (usize, &str), right: (usize, &str)) -> bool {
    let ((left, from), (right, to)) = (left, right);
    if rule.either {
        left < right
            && ((admits(&rule.source, from) && admits(&rule.target, to))
                || (admits(&rule.source, to) && admits(&rule.target, from)))
    } else {
        left != right && admits(&rule.source, from) && admits(&rule.target, to)
    }
}

/// The step-3 question about the asked names at `left` and `right`.
fn pair_words(rule: &RelationRule, left: usize, right: usize) -> String {
    let (one, two, reads) = (entity_id(left), entity_id(right), &rule.reads);
    if rule.either {
        format!("Does the text itself state that {one} {reads} {two}, or that {two} {reads} {one}?")
    } else {
        format!("Does the text itself state that {one} {reads} {two}?")
    }
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
        let ordered = asked
            .iter()
            .enumerate()
            .flat_map(|left| asked.iter().enumerate().map(move |right| (left, right)));
        for ((left, source), (right, target)) in ordered {
            if !allowed(rule, (left, kind(*source)), (right, kind(*target))) {
                continue;
            }
            questions.push(Question::Decide {
                text: QuestionText::new(pair_words(rule, left, right))
                    .map_err(|_| RelationPlanError)?,
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
    if questions.is_empty() {
        return Ok(None);
    }
    let kept: Vec<RecognizedName> = asked
        .iter()
        .filter_map(|at| names.get(*at).cloned())
        .collect();
    Ok(Some(StatedPlan {
        evidence: state_evidence(Some(text), &kept, None)?,
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
    let mut edges = Vec::new();
    for (pair, answer) in pairs.iter().zip(answers) {
        let probability = answer.yes().filter(|value| super::reaches_cut(*value, cut));
        if let (Some(probability), Some(rule)) = (probability, rules.get(pair.rule)) {
            push_edge(
                &mut edges,
                names,
                rule,
                pair.source,
                pair.target,
                probability,
            );
        }
    }
    edges
}
