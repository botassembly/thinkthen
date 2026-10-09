//! Pure name recognition in three steps (ADR 0056): BILOU boundaries, then
//! kinds and edges, then stated relations in `relation`.

use std::ops::Range;

use serde::{Serialize, Serializer};

use crate::core::question::LabelsError;
use crate::core::text::Withheld;
use crate::core::{Description, Question};

mod bilou;
mod categories;
mod examples;
mod pieces;
mod seeds;
mod stage_context;
pub use seeds::RecognitionSeedSpan;
pub(crate) use seeds::{seed_stretches, selected_seeds};
pub use stage_context::RecognitionStageContext;
mod questions;

use bilou::best_of;
pub(crate) use bilou::{SpanOdds, TAGS, TagRow, decode};
pub(crate) use examples::{ExamplesError, example_file, render_examples, selected_examples};
pub use examples::{RecognitionExample, RecognitionExampleEntity, RecognitionExampleText};
pub(crate) use pieces::{Piece, pieces};
pub(crate) use questions::{
    NONE_OF_THESE, edge_label, edge_options, edge_question, evidence, kind_question, name_groups,
    step_one_groups, step_one_questions,
};

/// The kind every name takes when the caller names no kind.
pub(crate) const ENTITY: &str = "ENTITY";

#[derive(Clone, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "entity"))]
pub(crate) struct RecognizedName {
    pub(crate) text: String,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) length: usize,
    pub(crate) kind: String,
    pub(crate) strength: f64,
}

/// A name is evidence, so `Debug` withholds it and keeps the configured kind.
impl std::fmt::Debug for RecognizedName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("RecognizedName")
            .field("text", &Withheld(self.text.len()))
            .field("start", &self.start)
            .field("end", &self.end)
            .field("length", &self.length)
            .field("kind", &self.kind)
            .field("strength", &self.strength)
            .finish()
    }
}

/// What step 2 asks about one found name: the kind question when the run has
/// kinds, and the edge question when the name has two or more stretches.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Asked {
    pub(crate) kind: bool,
    pub(crate) edges: Vec<(usize, usize)>,
}

impl Asked {
    pub(crate) fn edge_question(&self) -> bool {
        self.edges.len() > 1
    }
}

/// One step-2 request's questions, and what each name in it asked.
pub(crate) type StepTwo = (Vec<Question>, Vec<Asked>);

/// The step-2 questions for the found names in `group`, in name order: each
/// name's kind question, then its edge question.
pub(crate) fn step_two_questions(
    text: &str,
    pieces: &[Piece],
    found: &[(usize, usize)],
    group: Range<usize>,
    kinds: &[(String, Option<Description>)],
    spec: Option<&crate::core::RecognizeSpec>,
) -> Result<StepTwo, LabelsError> {
    let mut questions = Vec::new();
    let mut asked = Vec::new();
    for stretch in found.get(group).unwrap_or_default().iter().copied() {
        let mut edges = edge_options(text, pieces, stretch);
        // A blank label would fail after step 1 is paid, so the name keeps its span.
        if edges
            .iter()
            .any(|edge| edge_label(text, pieces, *edge).is_empty())
        {
            edges.truncate(1);
        }
        let held = Asked {
            kind: !kinds.is_empty()
                || spec.is_some_and(|spec| {
                    spec.seed_spans.iter().any(|seed| {
                        pieces
                            .get(stretch.0)
                            .is_some_and(|piece| piece.start == seed.start)
                            && pieces
                                .get(stretch.1)
                                .is_some_and(|piece| piece.end == seed.end)
                    })
                }),
            edges,
        };
        if held.kind {
            let generic = vec![(ENTITY.to_owned(), None)];
            questions.push(kind_question(
                text,
                pieces,
                stretch,
                if kinds.is_empty() { &generic } else { kinds },
                spec,
            )?);
        }
        if held.edge_question() {
            questions.push(edge_question(text, pieces, &held.edges, spec)?);
        }
        asked.push(held);
    }
    Ok((questions, asked))
}

/// Labels and their probabilities, in the order they were asked.
#[derive(Clone, Default, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(
    test,
    schemars(inline, with = "std::collections::BTreeMap<String, f64>")
)]
pub(crate) struct Odds(pub(crate) Vec<(String, f64)>);

/// An edge label is evidence, so `Debug` withholds every label.
impl std::fmt::Debug for Odds {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let held: Vec<(Withheld, f64)> = self
            .0
            .iter()
            .map(|(label, value)| (Withheld(label.len()), *value))
            .collect();
        formatter.debug_tuple("Odds").field(&held).finish()
    }
}

impl Serialize for Odds {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.iter().map(|(label, value)| (label, value)))
    }
}

impl Odds {
    /// The most likely label's place, the earliest on a tie.
    fn leader(&self) -> Option<(usize, &str, f64)> {
        let (place, value) = best_of(self.0.iter().map(|(_, value)| *value).enumerate())?;
        let (label, _) = self.0.get(place)?;
        Some((place, label.as_str(), value))
    }
}

/// One found name's step-2 answers, as `--details` lists them.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "nameOdds"))]
pub(crate) struct NameOdds {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) kinds: Option<Odds>,
    pub(crate) edges: Option<Odds>,
}

/// One piece's tag probabilities, as `--details` lists them.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "pieceOdds"))]
pub(crate) struct PieceOdds {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) tags: Odds,
}

impl PieceOdds {
    pub(crate) fn new(piece: &Piece, row: &TagRow) -> Self {
        Self {
            start: piece.start,
            end: piece.end,
            tags: Odds(
                TAGS.iter()
                    .zip(row)
                    .map(|(tag, value)| ((*tag).to_owned(), *value))
                    .collect(),
            ),
        }
    }
}

/// `strength` at four places: P(kind) times P(span).
pub(crate) fn strength(kind: f64, span: f64) -> f64 {
    let raw = kind * span;
    format!("{raw:.4}").parse().unwrap_or(raw)
}

/// The printed names: declined names go, each name takes its picked stretch
/// and kind, a repeated stretch and kind keeps its stronger name, and a name
/// under `cut` goes. Names come in order of start, then end.
/// Each name's P(span) is its decoded stretch's, before any edge pick.
pub(crate) fn settle(
    text: &str,
    pieces: &[Piece],
    rows: &[TagRow],
    found: &[(usize, usize)],
    answers: &[(Asked, NameOdds)],
    cut: f64,
) -> Vec<RecognizedName> {
    let spans = SpanOdds::new(rows);
    let mut names: Vec<RecognizedName> = Vec::new();
    for ((first, last), (asked, odds)) in found.iter().copied().zip(answers) {
        let (kind, kind_odds) = match &odds.kinds {
            None => (ENTITY, 1.0),
            Some(kinds) => match kinds.leader() {
                Some((_, kind, _)) if kind == NONE_OF_THESE => continue,
                Some((_, kind, value)) => (kind, value),
                None => continue,
            },
        };
        let picked = odds
            .edges
            .as_ref()
            .and_then(Odds::leader)
            .and_then(|(place, _, _)| asked.edges.get(place).copied())
            .unwrap_or((first, last));
        let strength = strength(kind_odds, spans.span(first, last));
        let Some(held) = named(text, pieces, picked, kind, strength) else {
            continue;
        };
        match names.iter_mut().find(|other| {
            (other.start, other.end, &other.kind) == (held.start, held.end, &held.kind)
        }) {
            Some(other) if other.strength < held.strength => *other = held,
            Some(_) => {}
            None => names.push(held),
        }
    }
    names.retain(|name| name.strength >= cut);
    names.sort_by_key(|name| (name.start, name.end));
    names
}

fn named(
    text: &str,
    pieces: &[Piece],
    stretch: (usize, usize),
    kind: &str,
    strength: f64,
) -> Option<RecognizedName> {
    let first = pieces.get(stretch.0)?;
    let last = pieces.get(stretch.1)?;
    Some(RecognizedName {
        text: text.get(first.byte_start..last.byte_end)?.to_owned(),
        start: first.start,
        end: last.end,
        length: last.end - first.start,
        kind: kind.to_owned(),
        strength,
    })
}

#[cfg(test)]
#[path = "recognize/tables.rs"]
mod tables;
