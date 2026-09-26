//! What calibration and the coverage curve read: each answer's confidence in
//! the answer it gave as run, and how right that answer was.

use serde::Serialize;

use crate::core::measure::answer::{Answer, Rule, Said, Verb};
use crate::core::measure::key::{Outcome, Want, outcome};
use crate::core::measure::rows::Graded;
use crate::core::measure::{MeasureError, Pair, python_sum};

/// One point of the curve: the answers whose confidence reaches the cut, and their summed right.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Step {
    pub(crate) cut: f64,
    pub(crate) kept: usize,
    pub(crate) right: f64,
}

/// The share a tie earns: one over the tied options when the answer said tied and the key is one of them, else 0.
pub(crate) fn tie_share(answer: &Answer, want: &Want) -> f64 {
    answer
        .top
        .as_ref()
        .filter(|top| {
            matches!(answer.said(Rule::AsRun), Ok(Said::Tied))
                && top.holders.iter().any(|held| held == want.text())
        })
        .map_or(0.0, |top| 1.0 / top.holders.len() as f64)
}

/// One answer's pair: its confidence in the answer it gave, and 1, 0, or a tie's
/// share. `rank` and `score` give no answer to pair.
///
/// # Errors
///
/// Returns [`MeasureError`] when the answer cannot be read as run.
fn pair(answer: &Answer, want: &Want) -> Result<Option<Pair>, MeasureError> {
    if matches!(answer.verb, Verb::Rank | Verb::Score) {
        return Ok(None);
    }
    let said = answer.said(Rule::AsRun)?;
    let right = match outcome(&said, want) {
        Outcome::Right => 1.0,
        Outcome::Tied => tie_share(answer, want),
        Outcome::Wrong | Outcome::Unresolved => 0.0,
    };
    Ok(answer
        .confidence()
        .map(|p| match (answer.verb.yes_no(), said) {
            (true, Said::No) => (1.0 - p, right),
            (true, Said::Unresolved) => (p.max(1.0 - p), right),
            _ => (p, right),
        }))
}

/// The pairs of every labeled answer that pairs, in order.
///
/// # Errors
///
/// Returns [`MeasureError`] when an answer cannot be read as run.
pub(crate) fn pairs(items: &[Graded<'_>]) -> Result<Vec<Pair>, MeasureError> {
    items
        .iter()
        .filter_map(|(answer, want)| pair(answer, want).transpose())
        .collect()
}

/// The curve at every distinct confidence, from the highest down.
pub(crate) fn curve(pairs: &[Pair]) -> Vec<Step> {
    let mut cuts: Vec<f64> = pairs.iter().map(|pair| pair.0).collect();
    cuts.sort_by(|a, b| b.total_cmp(a));
    cuts.dedup();
    cuts.into_iter()
        .map(|cut| {
            let kept: Vec<&Pair> = pairs.iter().filter(|pair| pair.0 >= cut).collect();
            Step {
                cut,
                kept: kept.len(),
                right: python_sum(kept.iter().map(|pair| pair.1)),
            }
        })
        .collect()
}
