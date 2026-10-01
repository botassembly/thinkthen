//! What `rank` returns: every record with its input place and probability
//! of yes, most likely first.

use std::fmt;

use serde::Serialize;

use super::withheld_debug;
use crate::public::engine::Evidence;

/// One ranked record, its zero-based input place and its probability of yes.
#[derive(Clone, PartialEq)]
pub struct Ranked<T> {
    index: usize,
    input: T,
    probability: f64,
}

withheld_debug!(Ranked<T> { index, probability });

impl<T> Ranked<T> {
    pub(crate) const fn new(index: usize, input: T, probability: f64) -> Self {
        Self {
            index,
            input,
            probability,
        }
    }

    /// The record's zero-based place in the input.
    #[must_use]
    pub fn index(&self) -> usize {
        self.index
    }

    /// The record as given.
    #[must_use]
    pub fn input(&self) -> &T {
        &self.input
    }

    /// Its probability of yes.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.probability
    }

    /// The record.
    #[must_use]
    pub fn into_input(self) -> T {
        self.input
    }
}

impl<T: Evidence> Ranked<T> {
    /// The record's place, text and probability. The C door writes one of
    /// these for each record as its `rank` value.
    #[must_use]
    pub fn row(&self) -> RankedRow<'_> {
        RankedRow {
            index: self.index,
            record: self.input.evidence(),
            probability: self.probability,
        }
    }
}

/// One ranked record: its zero-based input place, its text and its
/// probability of yes, as `{"index", "record", "probability"}`.
#[derive(Clone, Copy, PartialEq, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "rankedRecord"))]
pub struct RankedRow<'a> {
    index: usize,
    record: &'a str,
    probability: f64,
}

impl fmt::Debug for RankedRow<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RankedRow")
            .field("index", &self.index)
            .field("probability", &self.probability)
            .finish_non_exhaustive()
    }
}
