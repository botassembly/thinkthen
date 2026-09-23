//! Pure relation planning and edge assembly shared by recognition and relate.

use crate::core::recognize::RecognizedName;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct RelationRule {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) reads: String,
    pub(crate) either: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RelationPlan {
    pub(crate) questions: Vec<crate::core::Question>,
    pub(crate) mappings: Vec<QuestionMap>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct RelationEdge {
    pub(crate) relation: String,
    pub(crate) source: RecognizedName,
    pub(crate) target: RecognizedName,
    pub(crate) probability: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum QuestionMap {
    Choice(Vec<(String, usize, usize)>),
    Pair { source: usize, target: usize },
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("a relation question could not be built")]
pub(crate) struct RelationPlanError;

pub(crate) fn plan(
    entities: &[RecognizedName],
    rule: &RelationRule,
) -> Result<RelationPlan, RelationPlanError> {
    if rule.source == rule.target {
        return pair_plan(entities, rule);
    }
    let sources = matching(entities, &rule.source);
    let targets = matching(entities, &rule.target);
    if sources.len().min(targets.len()).saturating_add(1) > 255 {
        return pair_plan(entities, rule);
    }
    choice_plan(entities, rule, sources, targets)
}

fn matching(entities: &[RecognizedName], kind: &str) -> Vec<usize> {
    entities
        .iter()
        .enumerate()
        .filter(|(_, entity)| kind == "*" || entity.kind == kind)
        .map(|(place, _)| place)
        .collect()
}

fn choice_plan(
    entities: &[RecognizedName],
    rule: &RelationRule,
    sources: Vec<usize>,
    targets: Vec<usize>,
) -> Result<RelationPlan, RelationPlanError> {
    let reversed = sources.len() < targets.len();
    let (asking, options, option_kind) = if reversed {
        (&targets, &sources, rule.source.as_str())
    } else {
        (&sources, &targets, rule.target.as_str())
    };
    let mut questions = Vec::new();
    let mut mappings = Vec::new();
    for asker in asking {
        let Some(asking_entity) = entities.get(*asker) else {
            return Err(RelationPlanError);
        };
        let mut labels = Vec::new();
        let mut map = Vec::new();
        for option in options.iter().filter(|option| *option != asker) {
            let Some(entity) = entities.get(*option) else {
                return Err(RelationPlanError);
            };
            let label = format!("i{}", option + 1);
            labels.push((
                label.clone(),
                Some(crate::core::Description::text(reference(entity, *option))),
            ));
            let (source, target) = if reversed {
                (*option, *asker)
            } else {
                (*asker, *option)
            };
            map.push((label, source, target));
        }
        if labels.is_empty() {
            continue;
        }
        labels.push((
            "none".to_owned(),
            Some(crate::core::Description::text(format!(
                "No listed {option_kind}."
            ))),
        ));
        let text = format!(
            "Which listed {option_kind} fills the blank: {} {} ___? Choose none if no listed {option_kind} does.",
            reference(asking_entity, *asker),
            rule.reads
        );
        questions.push(crate::core::Question::Choose {
            text: crate::core::QuestionText::new(text).map_err(|_| RelationPlanError)?,
            options: crate::core::Labels::described(labels).map_err(|_| RelationPlanError)?,
        });
        mappings.push(QuestionMap::Choice(map));
    }
    Ok(RelationPlan {
        questions,
        mappings,
    })
}

fn pair_plan(
    entities: &[RecognizedName],
    rule: &RelationRule,
) -> Result<RelationPlan, RelationPlanError> {
    let sources = matching(entities, &rule.source);
    let targets = matching(entities, &rule.target);
    let mut questions = Vec::new();
    let mut mappings = Vec::new();
    for source in sources {
        for target in targets.iter().copied() {
            if source == target || (rule.either && source > target) {
                continue;
            }
            let (Some(left), Some(right)) = (entities.get(source), entities.get(target)) else {
                return Err(RelationPlanError);
            };
            let text = format!(
                "Does this hold: {} {} {}?",
                reference(left, source),
                rule.reads,
                reference(right, target)
            );
            questions.push(crate::core::Question::Decide {
                text: crate::core::QuestionText::new(text).map_err(|_| RelationPlanError)?,
                yes: None,
                no: None,
            });
            mappings.push(QuestionMap::Pair { source, target });
        }
    }
    Ok(RelationPlan {
        questions,
        mappings,
    })
}

pub(crate) fn plan_pairs(
    entities: &[RecognizedName],
    rule: &RelationRule,
) -> Result<RelationPlan, RelationPlanError> {
    pair_plan(entities, rule)
}

pub(crate) fn assemble_edges(
    entities: &[RecognizedName],
    rule: &RelationRule,
    mappings: &[QuestionMap],
    answers: &[crate::core::Answer],
    threshold: f64,
) -> Vec<RelationEdge> {
    let mut edges = Vec::new();
    for (mapping, answer) in mappings.iter().zip(answers) {
        match mapping {
            QuestionMap::Choice(options) => {
                let probabilities = answer.choice_probabilities().unwrap_or_default();
                push_choice_edges(
                    &mut edges,
                    entities,
                    rule,
                    options,
                    &probabilities,
                    threshold,
                );
            }
            QuestionMap::Pair { source, target } => {
                if let Some(probability) = answer.yes().filter(|value| *value >= threshold) {
                    push_edge(&mut edges, entities, rule, *source, *target, probability);
                }
            }
        }
    }
    edges
}

fn push_choice_edges(
    edges: &mut Vec<RelationEdge>,
    entities: &[RecognizedName],
    rule: &RelationRule,
    options: &[(String, usize, usize)],
    probabilities: &[(&str, f64)],
    threshold: f64,
) {
    for (label, source, target) in options {
        let probability = probabilities
            .iter()
            .find_map(|(held, probability)| (*held == label).then_some(*probability));
        if let Some(probability) = probability.filter(|value| *value >= threshold) {
            push_edge(edges, entities, rule, *source, *target, probability);
        }
    }
}

fn push_edge(
    edges: &mut Vec<RelationEdge>,
    entities: &[RecognizedName],
    rule: &RelationRule,
    source: usize,
    target: usize,
    probability: f64,
) {
    let (Some(source), Some(target)) = (entities.get(source), entities.get(target)) else {
        return;
    };
    edges.push(RelationEdge {
        relation: rule.name.clone(),
        source: source.clone(),
        target: target.clone(),
        probability,
    });
}

fn reference(entity: &RecognizedName, place: usize) -> String {
    format!("Item {} ({} \"{}\")", place + 1, entity.kind, entity.name)
}

#[cfg(test)]
#[rustfmt::skip]
#[path = "relation/experiment_239.rs"]
mod experiment_239;

#[cfg(test)]
mod tests {
    use super::{RelationRule, plan};
    use crate::core::recognize::RecognizedName;

    fn entity(name: &str, kind: &str, start: usize) -> RecognizedName {
        RecognizedName {
            name: name.to_owned(),
            kind: kind.to_owned(),
            start,
            end: start + name.chars().count(),
            strength: 1.0,
        }
    }

    fn rule(source: &str, target: &str, either: bool) -> RelationRule {
        RelationRule {
            name: "linked_to".to_owned(),
            source: source.to_owned(),
            target: target.to_owned(),
            either,
            reads: "is linked to".to_owned(),
        }
    }

    #[test]
    fn cross_kind_choice_asks_the_larger_side_with_smaller_side_options() {
        let entities = [
            entity("Ada", "person", 0),
            entity("Grace", "person", 4),
            entity("Acme", "organization", 10),
        ];
        let planned = plan(&entities, &rule("person", "organization", false)).expect("plan");
        assert_eq!(planned.questions.len(), 2);
        for question in planned.questions {
            let crate::core::Question::Choose { options, .. } = question else {
                panic!("cross-kind planner did not choose");
            };
            assert_eq!(options.count(), 2);
        }
    }

    #[test]
    fn same_kind_h_asks_unordered_or_ordered_pairs_and_never_self_pairs() {
        let entities = [
            entity("A", "person", 0),
            entity("B", "person", 2),
            entity("C", "person", 4),
        ];
        assert_eq!(
            plan(&entities, &rule("person", "person", true))
                .expect("plan")
                .questions
                .len(),
            3
        );
        assert_eq!(
            plan(&entities, &rule("person", "person", false))
                .expect("plan")
                .questions
                .len(),
            6
        );
    }

    #[test]
    fn explicit_wildcards_match_kinds_without_relating_a_name_to_itself() {
        let entities = [entity("A", "person", 0), entity("B", "place", 2)];
        assert_eq!(
            plan(&entities, &rule("*", "*", true))
                .expect("plan")
                .questions
                .len(),
            1
        );
        assert_eq!(
            plan(&entities, &rule("*", "*", false))
                .expect("plan")
                .questions
                .len(),
            2
        );
    }

    #[test]
    fn two_hundred_fifty_six_choice_options_make_every_relation_question_a_pair() {
        let mut entities = (0..256)
            .map(|place| entity(&format!("P{place}"), "person", place))
            .collect::<Vec<_>>();
        entities.extend(
            (0..255).map(|place| entity(&format!("O{place}"), "organization", place + 256)),
        );
        let planned = plan(&entities, &rule("person", "organization", false)).expect("pair plan");
        assert_eq!(planned.questions.len(), 256 * 255);
        assert!(
            planned
                .questions
                .iter()
                .all(|question| matches!(question, crate::core::Question::Decide { .. }))
        );
        assert!(
            planned
                .mappings
                .iter()
                .all(|mapping| matches!(mapping, super::QuestionMap::Pair { .. }))
        );
    }
}
