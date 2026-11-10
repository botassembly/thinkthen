//! Report arithmetic keeps absent denominators and excluded observations visible.
use super::Manifest;
use serde::Serialize;

/// A defined measurement or the reason it cannot be calculated.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Metric {
    /// A measurement with a nonzero, eligible denominator.
    Value(f64),
    /// No measurement is justified by these observations.
    Unavailable {
        /// Why this measurement is absent.
        reason: &'static str,
    },
}
impl Metric {
    fn ratio(numerator: usize, denominator: usize) -> Self {
        if denominator == 0 {
            Self::Unavailable {
                reason: "zero denominator",
            }
        } else {
            Self::Value(numerator as f64 / denominator as f64)
        }
    }
}
/// Dispositions use the complete manifest inventory, not only successful rows.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Counts {
    /// Cases declared in the manifest.
    pub expected_cases: usize,
    /// Expected cases without a saved result.
    pub missing_results: usize,
    /// Cases carrying a gold label, including absent results.
    pub labeled_cases: usize,
    /// Labeled answered results.
    pub answered_labeled: usize,
    /// Correct labeled answered results.
    pub correct: usize,
    /// Incorrect labeled answered results.
    pub wrong: usize,
    /// Labeled unsure results.
    pub unsure_labeled: usize,
    /// Labeled failed results.
    pub failed_labeled: usize,
    /// Saved results without a gold label.
    pub unlabeled_results: usize,
    /// Answered results without a gold label.
    pub unlabeled_answered: usize,
    /// Unsure results without a gold label.
    pub unlabeled_unsure: usize,
    /// Failed results without a gold label.
    pub unlabeled_failed: usize,
    /// Gold find-none cases, including cases without results.
    pub gold_none: usize,
    /// Answered none on a gold-none case.
    pub correct_none: usize,
    /// Answered none on a gold-span case.
    pub false_none: usize,
    /// Labeled results without a usable probability.
    pub excluded_missing_probability: usize,
    /// Unsure choose/find results without their winning selection.
    pub excluded_unknown_winner: usize,
}
/// One equal-width probability bin; the last bin includes one.
#[derive(Debug, Clone, Serialize)]
pub struct Bin {
    /// Inclusive lower bound.
    pub lower: f64,
    /// Upper bound, exclusive except for the final bin.
    pub upper: f64,
    /// Number of eligible labeled observations.
    pub count: usize,
    /// Mean backend probability.
    pub mean_probability: Metric,
    /// Observed yes rate for decide/filter; winner correctness otherwise.
    pub observed_rate: Metric,
    #[serde(skip)]
    probability_sum: f64,
    #[serde(skip)]
    observed: usize,
}
/// Offline audit of one explicitly identified question and route group.
#[derive(Debug, Clone, Serialize)]
pub struct AuditReport {
    /// Report format version.
    pub version: u32,
    /// Original metadata; a model alias is not promoted into a revision.
    pub manifest: Manifest,
    /// Complete case dispositions.
    pub counts: Counts,
    /// Correct divided by answered labeled results.
    pub answered_accuracy: Metric,
    /// Answered labeled results divided by all labeled cases.
    pub coverage: Metric,
    /// Unsure labeled results divided by all labeled cases.
    pub abstention: Metric,
    /// Binary tied-rank AUC; only decide/filter with both classes.
    pub auc: Metric,
    /// Equal-width bins of raw backend probabilities.
    pub bins: Vec<Bin>,
}
impl AuditReport {
    pub(super) fn new(manifest: Manifest, bins: usize) -> Self {
        Self {
            version: 1,
            manifest,
            counts: Counts::default(),
            answered_accuracy: Metric::ratio(0, 0),
            coverage: Metric::ratio(0, 0),
            abstention: Metric::ratio(0, 0),
            auc: Metric::ratio(0, 0),
            bins: (0..bins)
                .map(|index| Bin {
                    lower: index as f64 / bins as f64,
                    upper: (index + 1) as f64 / bins as f64,
                    count: 0,
                    mean_probability: Metric::ratio(0, 0),
                    observed_rate: Metric::ratio(0, 0),
                    probability_sum: 0.0,
                    observed: 0,
                })
                .collect(),
        }
    }
    pub(super) fn add_probability(&mut self, probability: f64, observed: bool) {
        let last = self.bins.len().saturating_sub(1);
        let index = ((probability * self.bins.len() as f64).floor() as usize).min(last);
        if let Some(bin) = self.bins.get_mut(index) {
            bin.count += 1;
            bin.probability_sum += probability;
            bin.observed += usize::from(observed);
        }
    }
    pub(super) fn finish(&mut self, binary: Vec<(f64, bool)>) {
        let counts = &self.counts;
        self.answered_accuracy = Metric::ratio(counts.correct, counts.answered_labeled);
        self.coverage = Metric::ratio(counts.answered_labeled, counts.labeled_cases);
        self.abstention = Metric::ratio(counts.unsure_labeled, counts.labeled_cases);
        self.auc = auc(binary);
        for bin in &mut self.bins {
            bin.mean_probability = if bin.count == 0 {
                Metric::ratio(0, 0)
            } else {
                Metric::Value(bin.probability_sum / bin.count as f64)
            };
            bin.observed_rate = Metric::ratio(bin.observed, bin.count);
        }
    }
}
fn auc(mut pairs: Vec<(f64, bool)>) -> Metric {
    pairs.sort_by(|left, right| left.0.total_cmp(&right.0));
    let positives = pairs.iter().filter(|(_, positive)| *positive).count();
    let negatives = pairs.len() - positives;
    if positives == 0 || negatives == 0 {
        return Metric::Unavailable {
            reason: "binary AUC requires decide/filter probabilities and both gold classes",
        };
    }
    let mut wins = 0.0;
    let mut lower_negatives = 0;
    let mut remaining = pairs.as_slice();
    while let Some((probability, _)) = remaining.first() {
        let tied = remaining
            .iter()
            .take_while(|pair| pair.0 == *probability)
            .count();
        let (group, rest) = remaining.split_at(tied);
        let positive = group.iter().filter(|(_, positive)| *positive).count();
        let negative = tied - positive;
        wins += positive as f64 * (lower_negatives as f64 + negative as f64 / 2.0);
        lower_negatives += negative;
        remaining = rest;
    }
    Metric::Value(wins / (positives as f64 * negatives as f64))
}
