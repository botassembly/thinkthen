//! Pure relation planning, state, and edge assembly shared by recognition and relate.

use serde::Serialize;
use thiserror::Error;

use crate::core::json::Json;
use crate::core::recognize::RecognizedName;
use crate::core::{Description, Evidence, Labels, Question, QuestionText, Withheld};

const MAX_CHOICE_OPTIONS: usize = 255;

#[derive(Clone, Eq, PartialEq, Serialize)]
pub(crate) struct RelationEntity {
    name: String,
    kind: String,
}

/// Entity names and kinds are evidence, so `Debug` withholds them.
impl std::fmt::Debug for RelationEntity {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("RelationEntity")
            .field(&Withheld(self.name.len() + self.kind.len()))
            .finish()
    }
}

impl RelationEntity {
    pub(crate) fn new(name: &str, kind: &str) -> Result<Self, RelationEntityError> {
        if name.trim().is_empty() || kind.trim().is_empty() {
            return Err(RelationEntityError);
        }
        Ok(Self {
            name: name.to_owned(),
            kind: kind.to_owned(),
        })
    }
}

pub(crate) trait RelationEntityView: Clone {
    fn name(&self) -> &str;
    fn kind(&self) -> &str;
}

impl RelationEntityView for RelationEntity {
    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> &str {
        &self.kind
    }
}

impl RelationEntityView for RecognizedName {
    fn name(&self) -> &str {
        &self.text
    }

    fn kind(&self) -> &str {
        &self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("a relation entity has a nonblank name and kind")]
pub(crate) struct RelationEntityError;

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
    pub(crate) relation: RelationRule,
    pub(crate) questions: Vec<Question>,
    pub(crate) mappings: Vec<QuestionMap>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct RelationEdge<E> {
    pub(crate) relation: String,
    pub(crate) source: E,
    pub(crate) target: E,
    pub(crate) probability: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum QuestionMap {
    /// One choice asked by `asker`, with each option's label and edge ends.
    Choice {
        asker: usize,
        options: Vec<(String, usize, usize)>,
    },
    Pair {
        source: usize,
        target: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("a relation question could not be built")]
pub(crate) struct RelationPlanError;

pub(crate) fn plan<E: RelationEntityView>(
    entities: &[E],
    rule: &RelationRule,
) -> Result<Vec<RelationPlan>, RelationPlanError> {
    concrete_relations(entities, rule)
        .into_iter()
        .map(|relation| plan_concrete(entities, relation))
        .collect()
}

fn concrete_relations<E: RelationEntityView>(
    entities: &[E],
    rule: &RelationRule,
) -> Vec<RelationRule> {
    let kinds = admitted_kinds(entities);
    let sources = expanded_side(&rule.source, &kinds);
    let targets = expanded_side(&rule.target, &kinds);
    let mut relations: Vec<RelationRule> = Vec::new();
    for source in sources {
        for target in &targets {
            if rule.either
                && relations
                    .iter()
                    .any(|held| held.source == *target && held.target == source)
            {
                continue;
            }
            let mut concrete = rule.clone();
            concrete.source.clone_from(&source);
            concrete.target.clone_from(target);
            relations.push(concrete);
        }
    }
    relations
}

fn admitted_kinds<E: RelationEntityView>(entities: &[E]) -> Vec<String> {
    let mut kinds = Vec::new();
    for entity in entities {
        if !kinds.iter().any(|kind| kind == entity.kind()) {
            kinds.push(entity.kind().to_owned());
        }
    }
    kinds
}

fn expanded_side(side: &str, kinds: &[String]) -> Vec<String> {
    if side == "*" {
        kinds.to_vec()
    } else {
        vec![side.to_owned()]
    }
}

fn plan_concrete<E: RelationEntityView>(
    entities: &[E],
    relation: RelationRule,
) -> Result<RelationPlan, RelationPlanError> {
    if relation.source == relation.target {
        return pair_plan(entities, relation);
    }
    let sources = matching(entities, &relation.source);
    let targets = matching(entities, &relation.target);
    if sources.len().min(targets.len()).saturating_add(1) > MAX_CHOICE_OPTIONS {
        return pair_plan(entities, relation);
    }
    choice_plan(entities, relation, sources, targets)
}

fn matching<E: RelationEntityView>(entities: &[E], kind: &str) -> Vec<usize> {
    entities
        .iter()
        .enumerate()
        .filter(|(_, entity)| entity.kind() == kind)
        .map(|(place, _)| place)
        .collect()
}

fn choice_plan<E: RelationEntityView>(
    entities: &[E],
    relation: RelationRule,
    sources: Vec<usize>,
    targets: Vec<usize>,
) -> Result<RelationPlan, RelationPlanError> {
    let reversed = sources.len() < targets.len();
    let (asking, options, option_kind) = if reversed {
        (&targets, &sources, relation.source.as_str())
    } else {
        (&sources, &targets, relation.target.as_str())
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
            let label = entity_id(*option);
            labels.push((
                label.clone(),
                Some(Description::text(reference(entity, *option))),
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
            Some(Description::text(format!("No listed {option_kind}."))),
        ));
        // The blank stands where the options go: after the relation words when
        // the source asks, and before them when the target asks.
        let asking = reference(asking_entity, *asker);
        let reads = &relation.reads;
        let clause = if reversed {
            format!("___ {reads} {asking}")
        } else {
            format!("{asking} {reads} ___")
        };
        let text = format!(
            "Which listed {option_kind} fills the blank: {clause}? Choose none if no listed {option_kind} does."
        );
        questions.push(Question::Choose {
            text: QuestionText::new(text).map_err(|_| RelationPlanError)?,
            options: Labels::described(labels).map_err(|_| RelationPlanError)?,
        });
        mappings.push(QuestionMap::Choice {
            asker: *asker,
            options: map,
        });
    }
    Ok(RelationPlan {
        relation,
        questions,
        mappings,
    })
}

fn pair_plan<E: RelationEntityView>(
    entities: &[E],
    relation: RelationRule,
) -> Result<RelationPlan, RelationPlanError> {
    let sources = matching(entities, &relation.source);
    let targets = matching(entities, &relation.target);
    let mut questions = Vec::new();
    let mut mappings = Vec::new();
    for source in sources {
        for target in targets.iter().copied() {
            if source == target
                || (relation.either && relation.source == relation.target && source > target)
            {
                continue;
            }
            let (source, target) = normalized_pair(source, target, relation.either);
            let direction = if relation.either { "between" } else { "from" };
            let joining = if relation.either { "and" } else { "to" };
            let text = format!(
                "Does the relation hold {direction} {} {joining} {}?",
                entity_id(source),
                entity_id(target)
            );
            questions.push(Question::Decide {
                text: QuestionText::new(text).map_err(|_| RelationPlanError)?,
                yes: None,
                no: None,
            });
            mappings.push(QuestionMap::Pair { source, target });
        }
    }
    Ok(RelationPlan {
        relation,
        questions,
        mappings,
    })
}

pub(crate) fn plan_pairs<E: RelationEntityView>(
    entities: &[E],
    relation: &RelationRule,
) -> Result<RelationPlan, RelationPlanError> {
    pair_plan(entities, relation.clone())
}

pub(crate) fn relation_evidence<E: RelationEntityView>(
    source: Option<&str>,
    entities: &[E],
    relation: &RelationRule,
) -> Result<Evidence, RelationPlanError> {
    let state = RelationState {
        evidence: source,
        entities: entities
            .iter()
            .enumerate()
            .map(|(place, entity)| {
                let entity = RelationEntity::new(entity.name(), entity.kind())?;
                Ok(StateEntity {
                    id: entity_id(place),
                    name: entity.name,
                    kind: entity.kind,
                })
            })
            .collect::<Result<Vec<_>, RelationEntityError>>()
            .map_err(|_| RelationPlanError)?,
        relation,
    };
    let text = serde_json::to_string(&state).map_err(|_| RelationPlanError)?;
    let value = Json::parse(&text).map_err(|_| RelationPlanError)?;
    Evidence::structured(value).map_err(|_| RelationPlanError)
}

#[derive(Serialize)]
struct RelationState<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<&'a str>,
    entities: Vec<StateEntity>,
    relation: &'a RelationRule,
}

#[derive(Serialize)]
struct StateEntity {
    id: String,
    name: String,
    kind: String,
}

pub(crate) fn assemble_edges<E: RelationEntityView>(
    entities: &[E],
    relation: &RelationRule,
    mappings: &[QuestionMap],
    answers: &[crate::core::Answer],
    threshold: f64,
) -> Vec<RelationEdge<E>> {
    let mut edges = Vec::new();
    for (mapping, answer) in mappings.iter().zip(answers) {
        match mapping {
            QuestionMap::Choice { options, .. } => {
                let probabilities = answer.choice_probabilities().unwrap_or_default();
                push_choice_edges(
                    &mut edges,
                    entities,
                    relation,
                    options,
                    &probabilities,
                    threshold,
                );
            }
            QuestionMap::Pair { source, target } => {
                if let Some(probability) =
                    answer.yes().filter(|value| reaches_cut(*value, threshold))
                {
                    push_edge(
                        &mut edges,
                        entities,
                        relation,
                        *source,
                        *target,
                        probability,
                    );
                }
            }
        }
    }
    edges
}

/// The one relation cut: a probability at or above the threshold makes an edge.
pub(crate) fn reaches_cut(probability: f64, threshold: f64) -> bool {
    probability >= threshold
}

fn push_choice_edges<E: RelationEntityView>(
    edges: &mut Vec<RelationEdge<E>>,
    entities: &[E],
    relation: &RelationRule,
    options: &[(String, usize, usize)],
    probabilities: &[(&str, f64)],
    threshold: f64,
) {
    for (label, source, target) in options {
        let probability = probabilities
            .iter()
            .find_map(|(held, probability)| (*held == label).then_some(*probability));
        let Some(probability) = probability.filter(|value| reaches_cut(*value, threshold)) else {
            continue;
        };
        push_edge(edges, entities, relation, *source, *target, probability);
    }
}

fn push_edge<E: RelationEntityView>(
    edges: &mut Vec<RelationEdge<E>>,
    entities: &[E],
    relation: &RelationRule,
    source: usize,
    target: usize,
    probability: f64,
) {
    let (source, target) = normalized_pair(source, target, relation.either);
    if source == target {
        return;
    }
    let (Some(source), Some(target)) = (entities.get(source), entities.get(target)) else {
        return;
    };
    edges.push(RelationEdge {
        relation: relation.name.clone(),
        source: source.clone(),
        target: target.clone(),
        probability,
    });
}

fn normalized_pair(source: usize, target: usize, either: bool) -> (usize, usize) {
    if either && source > target {
        (target, source)
    } else {
        (source, target)
    }
}

fn entity_id(place: usize) -> String {
    format!("i{}", place + 1)
}

fn reference<E: RelationEntityView>(entity: &E, place: usize) -> String {
    format!(
        "Item {} ({} \"{}\")",
        place + 1,
        entity.kind(),
        entity.name()
    )
}

#[path = "relation/stated.rs"]
mod stated;
pub(crate) use stated::{plan_stated, stated_edges};

#[cfg(test)]
#[rustfmt::skip]
#[path = "relation/experiment_239.rs"]
mod experiment_239;

#[cfg(test)]
#[path = "relation/boundary_tests.rs"]
mod boundary_tests;

#[cfg(test)]
#[path = "relation/tests.rs"]
mod tests;
