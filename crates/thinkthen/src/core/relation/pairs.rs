//! One yes/no question per allowed relation pair, shared by `recognize` and `relate`.

use super::{
    RelationEdge, RelationEntityView, RelationPlanError, RelationRule, entity_id, push_edge,
    state_evidence,
};
use crate::core::{Answer, Evidence, Question, QuestionText};

/// The evidence and ordered pair questions shared by one entity set.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PairPlan {
    pub(crate) evidence: Evidence,
    pub(crate) questions: Vec<Question>,
    pub(crate) pairs: Vec<Pair>,
}

/// One asked pair: the rule's place and the two names' places.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Pair {
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
fn allowed(rule: &RelationRule, (left, from): (usize, &str), (right, to): (usize, &str)) -> bool {
    let forward = admits(&rule.source, from) && admits(&rule.target, to);
    if rule.either {
        left < right && (forward || (admits(&rule.source, to) && admits(&rule.target, from)))
    } else {
        left != right && forward
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Lead {
    Stated,
    Known,
}

/// The question about the asked names at `left` and `right`.
fn pair_words(rule: &RelationRule, left: usize, right: usize, lead: Lead) -> String {
    let (one, two, reads) = (entity_id(left), entity_id(right), &rule.reads);
    let also = if rule.either {
        format!(", or that {two} {reads} {one}")
    } else {
        String::new()
    };
    let prefix = match lead {
        Lead::Stated => "Does the text itself state that",
        Lead::Known => "Is it true that",
    };
    format!("{prefix} {one} {reads} {two}{also}?")
}

/// Plan allowed pairs, or `None` when none are allowed. Equal names and kinds
/// are asked once, as the first, preserving recognition identity.
pub(crate) fn plan_pairs<E: RelationEntityView>(
    text: Option<&str>,
    names: &[E],
    rules: &[RelationRule],
    lead: Lead,
) -> Result<Option<PairPlan>, RelationPlanError> {
    let mut asked: Vec<usize> = Vec::new();
    for (place, name) in names.iter().enumerate() {
        let repeated = asked.iter().any(|held| {
            names
                .get(*held)
                .is_some_and(|other| other.name() == name.name() && other.kind() == name.kind())
        });
        let named = rules
            .iter()
            .any(|rule| admits(&rule.source, name.kind()) || admits(&rule.target, name.kind()));
        if !repeated && named {
            asked.push(place);
        }
    }
    let kind = |at: usize| names.get(at).map_or("", |name| name.kind());
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
                text: QuestionText::new(pair_words(rule, left, right, lead))
                    .map_err(|_| RelationPlanError)?,
                yes: None,
                no: None,
            });
            pairs.push(Pair {
                rule: rule_place,
                source: *source,
                target: *target,
            });
        }
    }
    if questions.is_empty() {
        return Ok(None);
    }
    let kept: Vec<E> = asked
        .iter()
        .filter_map(|at| names.get(*at).cloned())
        .collect();
    Ok(Some(PairPlan {
        evidence: state_evidence(text, &kept)?,
        questions,
        pairs,
    }))
}

/// The edges whose yes probability reaches `cut`, in question order.
pub(crate) fn pair_edges<E: RelationEntityView>(
    names: &[E],
    rules: &[RelationRule],
    pairs: &[Pair],
    answers: &[Answer],
    cut: f64,
) -> Vec<RelationEdge<E>> {
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
