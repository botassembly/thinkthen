//! Pure relation planning, state, and edge assembly shared by recognition and relate.

use serde::Serialize;
use thiserror::Error;

use crate::core::json::Json;
use crate::core::recognize::RecognizedName;
use crate::core::{Evidence, Withheld};

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

#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct RelationEdge<E> {
    pub(crate) relation: String,
    pub(crate) source: E,
    pub(crate) target: E,
    pub(crate) probability: f64,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("a relation question could not be built")]
pub(crate) struct RelationPlanError;

/// The state every pair request carries: the text when there is one and
/// each admitted entity by id.
fn state_evidence<E: RelationEntityView>(
    source: Option<&str>,
    entities: &[E],
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
}

#[derive(Serialize)]
struct StateEntity {
    id: String,
    name: String,
    kind: String,
}

/// The one relation cut: a probability at or above the threshold makes an edge.
pub(crate) fn reaches_cut(probability: f64, threshold: f64) -> bool {
    probability >= threshold
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

#[path = "relation/pairs.rs"]
mod pairs;
pub(crate) use pairs::{Lead, Pair, PairPlan, count_pairs, pair_edges, plan_pairs};

#[cfg(test)]
#[path = "relation/tests.rs"]
mod tests;
