//! The answer a backend gave, in thinkthen's own words.

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::probability::Probability;
use crate::threshold::{Outcome, Threshold};

/// The decimal place a weighted score is rounded at, to drop summation noise.
const ROUNDING: f64 = 1e12;

/// The odds of every label, in the order the labels were sent.
///
/// The backend may answer in any key order. The order here is the user's own,
/// so a reader of a result sees the list they typed.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Distribution {
    entries: Vec<(String, Probability)>,
    total: f64,
}

/// Why a set of probabilities is not a complete distribution.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum DistributionError {
    /// The members do not total one within the permitted floating-point error.
    #[error("probabilities must total one within member count times f64::EPSILON")]
    Total,
}

impl Distribution {
    /// Take one probability per label, in label order.
    pub(crate) fn new(entries: Vec<(String, Probability)>) -> Result<Self, DistributionError> {
        let total = entries
            .iter()
            .map(|(_, probability)| probability.as_f64())
            .sum::<f64>();
        let tolerance = entries.len() as f64 * f64::EPSILON;
        if (total - 1.0).abs() > tolerance {
            return Err(DistributionError::Total);
        }
        Ok(Self { entries, total })
    }

    /// Read the labels back, in the order they were sent.
    #[cfg(test)]
    pub(crate) fn labels(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|(label, _)| label.as_str())
    }

    /// Read the probabilities back, in the same order.
    pub(crate) fn probabilities(&self) -> impl Iterator<Item = Probability> {
        self.entries.iter().map(|(_, probability)| *probability)
    }

    /// The first label with the highest probability, which is the one that led.
    fn leader(&self) -> Option<(&str, Probability)> {
        self.entries
            .iter()
            .fold(None, |best, (label, probability)| match best {
                Some((_, highest)) if highest.as_f64() >= probability.as_f64() => best,
                _ => Some((label.as_str(), *probability)),
            })
    }

    /// True when two labels share the highest probability exactly.
    fn tied(&self) -> bool {
        self.leader().is_some_and(|(_, highest)| {
            self.probabilities()
                .filter(|probability| probability.as_f64() == highest.as_f64())
                .count()
                > 1
        })
    }

    /// The probability-weighted position on the labels, lowest first.
    ///
    /// Adding ten fractions leaves a remainder in the last bits that no reader
    /// of a score means, so the sum is rounded at the twelfth decimal. A score
    /// runs from 0 to 9, so the rounding drops that remainder and nothing else.
    fn position(&self) -> f64 {
        let weighted: f64 = self
            .probabilities()
            .enumerate()
            .map(|(place, probability)| {
                let place: u32 = place.try_into().unwrap_or(u32::MAX);
                f64::from(place) * probability.as_f64()
            })
            .sum();
        let normalized = weighted / self.total;
        (normalized * ROUNDING).round() / ROUNDING
    }
}

impl Serialize for Distribution {
    /// Write one JSON object with the labels as its keys, in label order.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(
            self.entries
                .iter()
                .map(|(label, probability)| (label, probability)),
        )
    }
}

/// The three shapes of answer, each carrying what its backend reported.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Shape {
    /// A probability that the condition holds.
    YesNo { probability: Probability },
    /// One label picked from a list, with the odds of every option.
    Choice {
        pick: String,
        probabilities: Distribution,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<Probability>,
    },
    /// A place on named levels, with the odds of every level.
    Score {
        level: String,
        probabilities: Distribution,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<Probability>,
    },
}

/// What the backend said, carrying no vendor field name.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Answer(Shape);

/// The bare value one judgment prints on standard output.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Value {
    /// `decide`: `true`, `false`, or `null`.
    YesNo(Option<bool>),
    /// `choose`: the winning label, or `null` when the answer is unresolved.
    Choice(Option<String>),
    /// `score`: the weighted position on the levels.
    Score(f64),
}

impl Value {
    /// The label `--raw` prints, or `None` when there is no label to print.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        match self {
            Self::Choice(label) => label.as_deref(),
            Self::YesNo(_) | Self::Score(_) => None,
        }
    }
}

#[cfg(test)]
#[path = "answer_distribution_tests.rs"]
mod distribution_tests;

impl Answer {
    /// Take a probability as the answer to a yes/no question.
    pub(crate) const fn new_yes_no(probability: Probability) -> Self {
        Self(Shape::YesNo { probability })
    }

    /// Take the odds of every option as the answer to a pick.
    ///
    /// Returns `None` when no option carries odds, which no plan asks for.
    pub(crate) fn new_choice(
        probabilities: Distribution,
        confidence: Option<Probability>,
    ) -> Option<Self> {
        let (pick, _) = probabilities.leader()?;
        let pick = pick.to_owned();
        Some(Self(Shape::Choice {
            pick,
            probabilities,
            confidence,
        }))
    }

    /// Take the odds of every level as the answer to a placement.
    ///
    /// Returns `None` when no level carries odds, which no plan asks for.
    pub(crate) fn new_score(
        probabilities: Distribution,
        confidence: Option<Probability>,
    ) -> Option<Self> {
        let (level, _) = probabilities.leader()?;
        let level = level.to_owned();
        Some(Self(Shape::Score {
            level,
            probabilities,
            confidence,
        }))
    }

    /// Read this answer under the rule, as the bare value and the outcome.
    ///
    /// A pick is unresolved when the winning option falls under the cut, and
    /// when the top two options tie exactly. An exact tie is unresolved with or
    /// without a threshold, because the order the user typed is no evidence.
    #[must_use]
    pub fn read(&self, threshold: Option<Threshold>) -> (Value, Outcome) {
        match &self.0 {
            Shape::YesNo { probability } => {
                let outcome = threshold.unwrap_or_default().judge(*probability);
                (Value::YesNo(outcome.value()), outcome)
            }
            Shape::Choice {
                pick,
                probabilities,
                ..
            } => {
                let cleared = probabilities.leader().is_some_and(|(_, highest)| {
                    threshold.is_none_or(|rule| rule.judge(highest) == Outcome::Yes)
                });
                if cleared && !probabilities.tied() {
                    (Value::Choice(Some(pick.clone())), Outcome::Yes)
                } else {
                    (Value::Choice(None), Outcome::Unresolved)
                }
            }
            Shape::Score { probabilities, .. } => {
                (Value::Score(probabilities.position()), Outcome::Yes)
            }
        }
    }

    /// The probability that the answer to a yes/no question is yes.
    ///
    /// `rank` sorts on it. A pick and a placement carry none, because neither
    /// answers a yes/no question and neither is a thing `rank` orders.
    #[must_use]
    pub fn yes(&self) -> Option<f64> {
        match &self.0 {
            Shape::YesNo { probability } => Some(probability.as_f64()),
            Shape::Choice { .. } | Shape::Score { .. } => None,
        }
    }

    /// The odds of every label, or `None` for a yes/no answer.
    #[cfg(test)]
    pub(crate) fn distribution(&self) -> Option<&Distribution> {
        match &self.0 {
            Shape::YesNo { .. } => None,
            Shape::Choice { probabilities, .. } | Shape::Score { probabilities, .. } => {
                Some(probabilities)
            }
        }
    }

    /// The label that led, before any threshold, or `None` for a yes/no answer.
    #[cfg(test)]
    pub(crate) fn leader(&self) -> Option<&str> {
        match &self.0 {
            Shape::YesNo { .. } => None,
            Shape::Choice { pick: label, .. } | Shape::Score { level: label, .. } => Some(label),
        }
    }

    /// The backend's own confidence, when the backend reported one.
    #[cfg(test)]
    pub(crate) fn confidence(&self) -> Option<Probability> {
        match &self.0 {
            Shape::YesNo { .. } => None,
            Shape::Choice { confidence, .. } | Shape::Score { confidence, .. } => *confidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Answer, Distribution, Value};
    use crate::probability::Probability;
    use crate::threshold::{Outcome, Threshold};
    use proptest::collection::vec;
    use proptest::prelude::Strategy;
    use proptest::{prop_assert, prop_assert_eq, proptest};

    /// One case's distribution, as a label and a probability per entry.
    type Odds<'a> = &'a [(&'a str, f64)];

    /// One reading case: the odds, the rule, and what the verb then prints.
    type Reading<'a> = (Odds<'a>, Option<Threshold>, Value, Outcome);

    fn odds(entries: &[(&str, f64)]) -> Distribution {
        Distribution::new(
            entries
                .iter()
                .map(|(label, value)| {
                    (
                        (*label).to_owned(),
                        Probability::new(*value).expect("a probability"),
                    )
                })
                .collect(),
        )
        .expect("a distribution")
    }

    fn choice(entries: &[(&str, f64)]) -> Answer {
        Answer::new_choice(odds(entries), None).expect("one option carries odds")
    }

    fn rule(text: &str) -> Threshold {
        text.parse().expect("a threshold")
    }

    #[test]
    fn a_pick_clears_the_cut_falls_under_it_or_ties_for_first() {
        let split = &[("bug", 0.5), ("feature", 0.5), ("other", 0.0)];
        let clear = &[("bug", 0.94), ("feature", 0.04), ("other", 0.02)];
        let cases: [Reading<'_>; 6] = [
            (
                clear,
                None,
                Value::Choice(Some("bug".to_owned())),
                Outcome::Yes,
            ),
            (
                clear,
                Some(rule("0.8")),
                Value::Choice(Some("bug".to_owned())),
                Outcome::Yes,
            ),
            (
                clear,
                Some(rule("0.94")),
                Value::Choice(Some("bug".to_owned())),
                Outcome::Yes,
            ),
            (
                clear,
                Some(rule("0.99")),
                Value::Choice(None),
                Outcome::Unresolved,
            ),
            (split, None, Value::Choice(None), Outcome::Unresolved),
            (
                split,
                Some(rule("0.1")),
                Value::Choice(None),
                Outcome::Unresolved,
            ),
        ];
        for (entries, threshold, value, outcome) in cases {
            assert_eq!(
                choice(entries).read(threshold),
                (value, outcome),
                "{threshold:?}"
            );
        }
    }

    #[test]
    fn the_leader_is_the_first_of_a_tie_and_a_tie_is_still_unresolved() {
        let answer = choice(&[("bug", 0.5), ("feature", 0.5)]);
        assert_eq!(answer.leader(), Some("bug"));
        assert_eq!(answer.read(None).0, Value::Choice(None));
    }

    #[test]
    fn a_score_is_the_weighted_position_on_the_levels_it_was_given() {
        let cases: [(Odds<'_>, f64); 4] = [
            (&[("low", 0.05), ("mid", 0.30), ("high", 0.65)], 1.6),
            (&[("low", 0.0), ("mid", 1.0), ("high", 0.0)], 1.0),
            (&[("low", 0.5), ("mid", 0.0), ("high", 0.5)], 1.0),
            (&[("low", 1.0), ("mid", 0.0)], 0.0),
        ];
        for (entries, expected) in cases {
            let answer = Answer::new_score(odds(entries), None).expect("one level carries odds");
            let (value, outcome) = answer.read(None);
            let Value::Score(number) = value else {
                panic!("a score prints a number");
            };
            assert!(
                (number - expected).abs() < 1e-12,
                "{number} is not {expected}"
            );
            assert_eq!(outcome, Outcome::Yes);
        }
    }

    #[test]
    fn an_answer_serializes_in_the_shape_the_specification_prints() {
        let answer = Answer::new_choice(
            odds(&[("bug", 0.94), ("feature", 0.04), ("other", 0.02)]),
            Some(Probability::new(0.91).expect("a probability")),
        )
        .expect("one option carries odds");
        assert_eq!(
            serde_json::to_string(&answer).expect("an answer serializes"),
            concat!(
                r#"{"kind":"choice","pick":"bug","#,
                r#""probabilities":{"bug":0.94,"feature":0.04,"other":0.02},"#,
                r#""confidence":0.91}"#,
            )
        );

        let answer = Answer::new_score(odds(&[("low", 0.4), ("high", 0.6)]), None)
            .expect("one level carries odds");
        assert_eq!(
            serde_json::to_string(&answer).expect("an answer serializes"),
            r#"{"kind":"score","level":"high","probabilities":{"low":0.4,"high":0.6}}"#
        );
    }

    #[test]
    fn only_a_yes_no_answer_carries_the_probability_that_rank_orders_by() {
        let probability = Probability::new(0.82).expect("a probability");
        assert_eq!(Answer::new_yes_no(probability).yes(), Some(0.82));
        assert_eq!(choice(&[("bug", 0.94), ("feature", 0.06)]).yes(), None);
        let placed = Answer::new_score(odds(&[("low", 0.4), ("high", 0.6)]), None)
            .expect("one level carries odds");
        assert_eq!(placed.yes(), None);
    }

    #[test]
    fn only_a_pick_carries_a_label_for_the_raw_view() {
        assert_eq!(Value::Choice(Some("bug".to_owned())).label(), Some("bug"));
        assert_eq!(Value::Choice(None).label(), None);
        assert_eq!(Value::YesNo(Some(true)).label(), None);
        assert_eq!(Value::Score(1.6).label(), None);
    }

    /// A label and a probability, so a generated distribution is a real one.
    fn entries() -> impl Strategy<Value = Vec<(String, f64)>> {
        vec((1_usize..8, 0.0_f64..=1.0), 2..8).prop_map(|drawn| {
            let total: f64 = drawn.iter().map(|(_, value)| *value).sum();
            drawn
                .into_iter()
                .enumerate()
                .map(|(place, (width, value))| {
                    let value = normalized(place, value, total);
                    (format!("{place}{}", "x".repeat(width)), value)
                })
                .collect()
        })
    }

    fn normalized(place: usize, value: f64, total: f64) -> f64 {
        if total == 0.0 {
            return f64::from(place == 0);
        }
        value / total
    }

    proptest! {
        /// The bare value of `choose` is one of the options sent, or `null`.
        #[test]
        fn a_pick_is_always_an_option_that_was_sent_or_nothing(
            drawn in entries(),
            mark in 0.001_f64..=1.0,
            cut in proptest::bool::ANY,
        ) {
            let listed: Vec<(&str, f64)> = drawn
                .iter()
                .map(|(label, value)| (label.as_str(), *value))
                .collect();
            let answer = choice(&listed);
            let threshold = cut.then(|| Threshold::cut(mark).expect("a cut"));
            let (value, outcome) = answer.read(threshold);
            let Value::Choice(label) = value else {
                panic!("a pick prints a label or nothing");
            };
            match label {
                Some(label) => {
                    prop_assert!(drawn.iter().any(|(sent, _)| *sent == label));
                    prop_assert_eq!(outcome, Outcome::Yes);
                }
                None => prop_assert_eq!(outcome, Outcome::Unresolved),
            }
        }
    }
}
