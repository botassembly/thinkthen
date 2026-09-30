//! A single-answer rule asks one `choice` per source, listing every allowed
//! target and `none` (ticket 0342, amending ADR 0057). Every other rule asks
//! its yes/no pairs, and both share one state.

use super::pairs::{admits, asked_names, kept_evidence, rule_pairs};
use super::{
    Lead, Pair, RelationEdge, RelationEntityView, RelationPlanError, RelationRule, entity_id,
    push_edge, reaches_cut,
};
use crate::core::{Answer, Description, Evidence, Labels, Question, QuestionText};

/// The last option of every menu.
const NONE: &str = "none";

/// One asked menu: the rule's place, the source's place, and each
/// candidate's place, in option order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Menu {
    pub(crate) rule: usize,
    pub(crate) source: usize,
    pub(crate) candidates: Vec<usize>,
}

/// What one relate question asks about.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RelateAsk {
    Pair(Pair),
    Menu(Menu),
}

impl RelateAsk {
    pub(crate) const fn rule(&self) -> usize {
        match self {
            Self::Pair(pair) => pair.rule,
            Self::Menu(menu) => menu.rule,
        }
    }
}

/// The evidence, questions and asked pairs or menus of one entity set.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RelatePlan {
    pub(crate) evidence: Evidence,
    pub(crate) questions: Vec<Question>,
    pub(crate) asked: Vec<RelateAsk>,
}

/// Plan every rule in order, or `None` when no rule asks anything.
pub(crate) fn plan_relate<E: RelationEntityView>(
    names: &[E],
    rules: &[RelationRule],
) -> Result<Option<RelatePlan>, RelationPlanError> {
    let asked_places = asked_names(names, rules);
    let mut questions = Vec::new();
    let mut asked = Vec::new();
    for (rule_place, rule) in rules.iter().enumerate() {
        if rule.single {
            for (question, menu) in rule_menus(names, &asked_places, (rule_place, rule))? {
                questions.push(question);
                asked.push(RelateAsk::Menu(menu));
            }
        } else {
            let pairs = rule_pairs(names, &asked_places, (rule_place, rule), Lead::Known)?;
            for (question, pair) in pairs {
                questions.push(question);
                asked.push(RelateAsk::Pair(pair));
            }
        }
    }
    if questions.is_empty() {
        return Ok(None);
    }
    Ok(Some(RelatePlan {
        evidence: kept_evidence(None, names, &asked_places)?,
        questions,
        asked,
    }))
}

/// One menu per asked source with at least one candidate, in input order.
/// The options are each candidate's state id, then `none`, all described.
fn rule_menus<E: RelationEntityView>(
    names: &[E],
    asked: &[usize],
    (rule_place, rule): (usize, &RelationRule),
) -> Result<Vec<(Question, Menu)>, RelationPlanError> {
    let listed = if rule.target == "*" {
        "entity"
    } else {
        rule.target.as_str()
    };
    let mut planned = Vec::new();
    for (left, source) in asked.iter().enumerate() {
        let Some(from) = names.get(*source) else {
            return Err(RelationPlanError);
        };
        if !admits(&rule.source, from.kind()) {
            continue;
        }
        let mut labels = Vec::new();
        let mut candidates = Vec::new();
        for (right, target) in asked.iter().enumerate() {
            let Some(to) = names.get(*target) else {
                return Err(RelationPlanError);
            };
            if right == left || !admits(&rule.target, to.kind()) {
                continue;
            }
            labels.push((entity_id(right), Some(Description::text(item(right, to)))));
            candidates.push(*target);
        }
        if candidates.is_empty() {
            continue;
        }
        labels.push((
            NONE.to_owned(),
            Some(Description::text(format!("No listed {listed}."))),
        ));
        let text = format!(
            "Which listed {listed} fills the blank: {} {} ___? Choose none if no listed {listed} does.",
            item(left, from),
            rule.reads
        );
        let question = Question::Choose {
            text: QuestionText::new(text).map_err(|_| RelationPlanError)?,
            options: Labels::described(labels).map_err(|_| RelationPlanError)?,
        };
        let menu = Menu {
            rule: rule_place,
            source: *source,
            candidates,
        };
        planned.push((question, menu));
    }
    Ok(planned)
}

/// How a menu names the asked name at `place`: its item number, kind and name.
fn item<E: RelationEntityView>(place: usize, entity: &E) -> String {
    format!(
        "Item {} ({} \"{}\")",
        place + 1,
        entity.kind(),
        entity.name()
    )
}

/// A menu's top label: a candidate's place, or `None` when `none` wins or
/// the top two labels tie exactly, with the top probability. A tie is not
/// sure, as `choose` rules it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Pick {
    pub(crate) target: Option<usize>,
    pub(crate) probability: f64,
}

impl Pick {
    /// Read the top label of a menu's answer, or `None` for an answer that
    /// does not hold the menu's options.
    pub(crate) fn of(menu: &Menu, answer: &Answer) -> Option<Self> {
        let named = answer.named()?;
        if named.len() != menu.candidates.len() + 1 {
            return None;
        }
        let mut top: Option<(usize, f64)> = None;
        let mut tied = false;
        for (place, (_, probability)) in named.iter().enumerate() {
            match top {
                Some((_, held)) if *probability < held => {}
                Some((_, held)) if *probability == held => tied = true,
                _ => {
                    top = Some((place, *probability));
                    tied = false;
                }
            }
        }
        let (place, probability) = top?;
        Some(Self {
            target: menu.candidates.get(place).copied().filter(|_| !tied),
            probability,
        })
    }

    /// Whether the pick makes an edge: a target at or above the cut.
    pub(crate) fn accepted(&self, cut: f64) -> bool {
        self.target.is_some() && reaches_cut(self.probability, cut)
    }
}

/// The edge one answered pair or menu makes at `cut`, if any.
pub(crate) fn relate_edge<E: RelationEntityView>(
    names: &[E],
    rules: &[RelationRule],
    asked: &RelateAsk,
    answer: &Answer,
    cut: f64,
) -> Option<RelationEdge<E>> {
    let rule = rules.get(asked.rule())?;
    let (source, target, probability) = match asked {
        RelateAsk::Pair(pair) => (pair.source, pair.target, answer.yes()?),
        RelateAsk::Menu(menu) => {
            let pick = Pick::of(menu, answer).filter(|pick| pick.accepted(cut))?;
            (menu.source, pick.target?, pick.probability)
        }
    };
    if !reaches_cut(probability, cut) {
        return None;
    }
    let mut edges = Vec::new();
    push_edge(&mut edges, names, rule, source, target, probability);
    edges.pop()
}
