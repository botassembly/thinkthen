//! What `find` returns: every candidate with its probability, and the one selected.

use std::fmt;

use serde::Serialize;

use super::withheld_debug;
use crate::engine::facade;
use crate::public::engine::Evidence;
use crate::public::error::Error;

/// One `find` unit, or the synthetic `none`, and its probability.
#[derive(Clone, PartialEq)]
pub struct Candidate<T> {
    input: Option<T>,
    probability: f64,
}

withheld_debug!(Candidate<T> { probability });

impl<T> Candidate<T> {
    /// The unit as given, or `None` for the synthetic `none`.
    #[must_use]
    pub fn input(&self) -> Option<&T> {
        self.input.as_ref()
    }

    /// Its probability.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.probability
    }

    /// True for the synthetic `none` candidate.
    #[must_use]
    pub fn is_none(&self) -> bool {
        self.input.is_none()
    }
}

/// Every `find` candidate in input order, the `none` candidate last when
/// the question offers it, and the one selected.
#[derive(Clone, PartialEq)]
pub struct Found<T> {
    candidates: Vec<Candidate<T>>,
    selected: Option<usize>,
}

withheld_debug!(Found<T> { selected });

impl<T> Found<T> {
    pub(crate) fn map<U>(self, mut map: impl FnMut(T) -> U) -> Found<U> {
        Found {
            selected: self.selected,
            candidates: self
                .candidates
                .into_iter()
                .map(|candidate| Candidate {
                    input: candidate.input.map(&mut map),
                    probability: candidate.probability,
                })
                .collect(),
        }
    }
    /// Pair each unit, and the `none` candidate last when asked, with its probability.
    pub(crate) fn new(units: Vec<T>, none: bool, found: &facade::Found) -> Result<Self, Error> {
        let probabilities = found.selection.probabilities();
        if probabilities.len() != units.len() + usize::from(none) {
            return Err(Error::defect("a find answer did not cover its units"));
        }
        let candidates = units
            .into_iter()
            .map(Some)
            .chain(none.then_some(None))
            .zip(probabilities)
            .map(|(input, (_, probability))| Candidate {
                input,
                probability: *probability,
            })
            .collect();
        Ok(Self {
            candidates,
            selected: found.selection.selected(),
        })
    }

    /// The selected unit, or `None` when nothing was selected.
    #[must_use]
    pub fn selected(&self) -> Option<&T> {
        self.candidates.get(self.selected?)?.input()
    }

    /// Every candidate, in input order, with the `none` candidate last when
    /// the question offers it.
    #[must_use]
    pub fn candidates(&self) -> &[Candidate<T>] {
        &self.candidates
    }

    /// The selected unit.
    #[must_use]
    pub fn into_selected(self) -> Option<T> {
        self.candidates.into_iter().nth(self.selected?)?.input
    }
}

impl<T: Evidence> Found<T> {
    /// The selected unit's place, text and probability, or `None` when
    /// nothing was selected. The C door writes this as its `find` value.
    #[must_use]
    pub fn picked(&self) -> Option<Picked<'_>> {
        let index = self.selected?;
        let candidate = self.candidates.get(index)?;
        Some(Picked {
            index,
            unit: candidate.input()?.evidence(),
            probability: candidate.probability,
        })
    }
}

/// The selected `find` unit: its zero-based input place, its text and its
/// probability, as `{"index", "unit", "probability"}`.
#[derive(Clone, Copy, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "foundUnit"))]
pub struct Picked<'a> {
    index: usize,
    unit: &'a str,
    probability: f64,
}

impl fmt::Debug for Picked<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Picked")
            .field("index", &self.index)
            .field("probability", &self.probability)
            .finish_non_exhaustive()
    }
}
