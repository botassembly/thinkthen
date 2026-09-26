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

/// True for a verb whose answers pair: `rank` gives no answer, and `score` reads levels.
pub(crate) const fn pairs_verb(verb: Verb) -> bool {
    matches!(verb, Verb::Decide | Verb::Tag | Verb::Choose | Verb::Find)
}

/// The share a tie earns: one over the tied options when the key is one of them, else 0.
pub(crate) fn tie_share(answer: &Answer, want: &Want) -> f64 {
    answer
        .top
        .as_ref()
        .filter(|top| top.tied && top.holders.iter().any(|held| held == want.text()))
        .map_or(0.0, |top| 1.0 / top.holders.len() as f64)
}

/// One answer's pair: its confidence in the answer it gave, and 1, 0, or a tie's share.
///
/// # Errors
///
/// Returns [`MeasureError`] when the answer cannot be read as run.
fn pair(answer: &Answer, want: &Want) -> Result<Option<Pair>, MeasureError> {
    let said = answer.said(Rule::AsRun)?;
    let right = match outcome(&said, want) {
        Outcome::Right => 1.0,
        Outcome::Tied => tie_share(answer, want),
        Outcome::Wrong | Outcome::Unresolved => 0.0,
    };
    Ok(match answer.verb {
        Verb::Decide | Verb::Tag => answer.probability.map(|p| {
            let p = p.as_f64();
            let confidence = match said {
                Said::Yes => p,
                Said::No => 1.0 - p,
                _ => p.max(1.0 - p),
            };
            (confidence, right)
        }),
        Verb::Choose | Verb::Find => answer.top.as_ref().map(|top| (top.top, right)),
        Verb::Rank | Verb::Score => None,
    })
}

/// The pairs of every labeled answer whose verb pairs, in order.
///
/// # Errors
///
/// Returns [`MeasureError`] when an answer cannot be read as run.
pub(crate) fn pairs(items: &[Graded<'_>]) -> Result<Vec<Pair>, MeasureError> {
    let mut out = Vec::new();
    for (answer, want) in items {
        if pairs_verb(answer.verb)
            && let Some(pair) = pair(answer, want)?
        {
            out.push(pair);
        }
    }
    Ok(out)
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
