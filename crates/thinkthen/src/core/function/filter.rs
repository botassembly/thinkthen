//! `filter`: keep only records whose yes or no answer is yes.

use crate::core::{Answer, Decide, Outcome};

/// A filter asks the same yes or no question as `decide`.
pub type Filter = Decide;

/// Whether a record's outcome passes the filter.
#[must_use]
pub fn passes(outcome: &Outcome) -> bool {
    matches!(
        outcome,
        Outcome::Answered {
            answer: Answer::Bool(true),
            ..
        }
    )
}
