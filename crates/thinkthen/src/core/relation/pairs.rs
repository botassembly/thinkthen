//! One yes/no question per allowed relation pair, shared by `recognize` and `relate`.

use super::{
    RelationEdge, RelationEntityView, RelationPlanError, RelationRule, entity_id, push_edge,
    state_evidence,
};
use crate::core::{Answer, Evidence, Question, QuestionText};
use std::collections::HashSet;

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
pub(super) fn admits(side: &str, kind: &str) -> bool {
    side == "*" || side == kind
}

/// Keep the first mention of each admitted name and kind, in input order.
pub(super) fn asked_names<E: RelationEntityView>(
    names: &[E],
    rules: &[RelationRule],
) -> Vec<usize> {
    let mut seen = HashSet::new();
    names
        .iter()
        .enumerate()
        .filter_map(|(place, name)| {
            let eligible = rules
                .iter()
                .any(|rule| admits(&rule.source, name.kind()) || admits(&rule.target, name.kind()));
            (eligible && seen.insert((name.name(), name.kind()))).then_some(place)
        })
        .collect()
}

/// The size of the full plan before any question or pair is materialized.
/// `None` for questions means checked arithmetic overflowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PairCount {
    pub(crate) admitted: usize,
    pub(crate) questions: Option<usize>,
}

pub(crate) fn count_pairs<E: RelationEntityView>(names: &[E], rules: &[RelationRule]) -> PairCount {
    let asked = asked_names(names, rules);
    let mut questions = Some(0_usize);
    for rule in rules {
        let mut source = 0_usize;
        let mut target = 0_usize;
        let mut both = 0_usize;
        for place in &asked {
            let kind = names.get(*place).map_or("", |name| name.kind());
            let from = admits(&rule.source, kind);
            let to = admits(&rule.target, kind);
            source += usize::from(from);
            target += usize::from(to);
            both += usize::from(from && to);
        }
        let rule_questions = source.checked_mul(target).and_then(|total| {
            let total = total.checked_sub(both)?;
            if rule.either {
                let repeats = both.checked_mul(both.saturating_sub(1))?.checked_div(2)?;
                total.checked_sub(repeats)
            } else {
                Some(total)
            }
        });
        questions =
            questions.and_then(|total| rule_questions.and_then(|more| total.checked_add(more)));
    }
    PairCount {
        admitted: asked.len(),
        questions,
    }
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
    let asked = asked_names(names, rules);
    let mut questions = Vec::new();
    let mut pairs = Vec::new();
    for (rule_place, rule) in rules.iter().enumerate() {
        for (question, pair) in rule_pairs(names, &asked, (rule_place, rule), lead)? {
            questions.push(question);
            pairs.push(pair);
        }
    }
    if questions.is_empty() {
        return Ok(None);
    }
    Ok(Some(PairPlan {
        evidence: kept_evidence(text, names, &asked)?,
        questions,
        pairs,
    }))
}

/// One rule's pair questions over the asked names, in source then target order.
pub(super) fn rule_pairs<E: RelationEntityView>(
    names: &[E],
    asked: &[usize],
    (rule_place, rule): (usize, &RelationRule),
    lead: Lead,
) -> Result<Vec<(Question, Pair)>, RelationPlanError> {
    let kind = |at: usize| names.get(at).map_or("", |name| name.kind());
    let ordered = asked
        .iter()
        .enumerate()
        .flat_map(|left| asked.iter().enumerate().map(move |right| (left, right)));
    let mut planned = Vec::new();
    for ((left, source), (right, target)) in ordered {
        if !allowed(rule, (left, kind(*source)), (right, kind(*target))) {
            continue;
        }
        let question = Question::Decide {
            text: QuestionText::new(pair_words(rule, left, right, lead))
                .map_err(|_| RelationPlanError)?,
            yes: None,
            no: None,
        };
        let pair = Pair {
            rule: rule_place,
            source: *source,
            target: *target,
        };
        planned.push((question, pair));
    }
    Ok(planned)
}

/// The shared state of the asked names, which carry ids `i1` onward.
pub(super) fn kept_evidence<E: RelationEntityView>(
    text: Option<&str>,
    names: &[E],
    asked: &[usize],
) -> Result<Evidence, RelationPlanError> {
    let kept: Vec<E> = asked
        .iter()
        .filter_map(|at| names.get(*at).cloned())
        .collect();
    state_evidence(text, &kept)
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

#[cfg(test)]
mod count_tests {
    use super::{Lead, PairCount, RelationRule, count_pairs, plan_pairs};
    use crate::core::relation::RelationEntity;

    #[test]
    fn counted_questions_match_the_small_real_plans() {
        let names = [
            ("Ada", "person"),
            ("Ada", "person"),
            ("Ada", "place"),
            ("Grace", "person"),
            ("Rome", "place"),
            ("Other", "irrelevant"),
        ]
        .map(|(name, kind)| RelationEntity::new(name, kind).expect("entity"));
        let check = |rules: Vec<RelationRule>| {
            let counted = count_pairs(&names, &rules);
            let planned = plan_pairs(None, &names, &rules, Lead::Known).expect("plan");
            assert_eq!(
                counted.questions,
                Some(planned.map_or(0, |plan| plan.questions.len()))
            );
        };
        for (source, target) in [
            ("person", "person"),
            ("person", "place"),
            ("*", "place"),
            ("*", "*"),
        ] {
            for either in [false, true] {
                let rule = RelationRule {
                    name: "near".into(),
                    source: source.into(),
                    target: target.into(),
                    reads: "is near".into(),
                    either,
                    single: false,
                };
                check(vec![rule.clone()]);
                check(vec![rule.clone(), rule]);
            }
        }
    }

    #[test]
    fn count_preserves_zero_work_with_many_admitted_names() {
        let names = (0..256)
            .map(|place| RelationEntity::new(&format!("P{place}"), "person").expect("entity"))
            .collect::<Vec<_>>();
        let rule = RelationRule {
            name: "works".into(),
            source: "person".into(),
            target: "place".into(),
            reads: "works in".into(),
            either: false,
            single: false,
        };
        assert_eq!(
            count_pairs(&names, std::slice::from_ref(&rule)),
            PairCount {
                admitted: 256,
                questions: Some(0)
            }
        );
        assert_eq!(
            count_pairs(&names, &[]),
            PairCount {
                admitted: 0,
                questions: Some(0)
            }
        );
        let irrelevant = (0..256)
            .map(|place| RelationEntity::new(&format!("O{place}"), "other").expect("entity"))
            .collect::<Vec<_>>();
        assert_eq!(
            count_pairs(&irrelevant, &[rule]),
            PairCount {
                admitted: 0,
                questions: Some(0)
            }
        );
    }

    #[test]
    fn wildcard_pair_count_crosses_the_bound_between_63_and_64_names() {
        let names = (0..64)
            .map(|place| RelationEntity::new(&format!("P{place}"), "person").expect("entity"))
            .collect::<Vec<_>>();
        let rule = RelationRule {
            name: "near".into(),
            source: "*".into(),
            target: "*".into(),
            reads: "near".into(),
            either: false,
            single: false,
        };
        assert_eq!(
            count_pairs(&names[..63], std::slice::from_ref(&rule)).questions,
            Some(3_906)
        );
        assert_eq!(
            count_pairs(&names, std::slice::from_ref(&rule)).questions,
            Some(4_032)
        );
        assert_eq!(
            count_pairs(&names, &[rule.clone(), rule]).questions,
            Some(8_064)
        );
    }
}
