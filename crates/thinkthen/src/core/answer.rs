//! The answer a backend gave, in thinkthen's own words.

use std::fmt;

use serde::{Serialize, Serializer};

use crate::core::probability::Probability;
use crate::core::text::Withheld;
use crate::core::threshold::{Outcome, Threshold};

/// The decimal place a weighted score is rounded at, to drop summation noise.
const ROUNDING: f64 = 1e12;

/// The odds of every label in user order, replacing backend key order.
#[derive(Clone, PartialEq)]
pub(crate) struct Distribution {
    entries: Vec<(String, Probability)>,
    total: f64,
}

/// Why a set of probabilities is not a complete distribution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum DistributionError {
    /// The members do not total one within the permitted floating-point error.
    Total {
        /// The sum computed from the reported members.
        total: f64,
        /// The number of members in that sum.
        members: usize,
        /// The current accepted distance from one.
        tolerance: f64,
    },
}

impl Distribution {
    /// Take one probability per label, in label order.
    #[allow(dead_code, reason = "generic default for later adapters")]
    pub(crate) fn new(entries: Vec<(String, Probability)>) -> Result<Self, DistributionError> {
        let tolerance = entries.len() as f64 * f64::EPSILON;
        Self::with_tolerance(entries, tolerance)
    }

    /// Take one probability per label under an adapter's evidenced tolerance.
    pub(crate) fn with_tolerance(
        entries: Vec<(String, Probability)>,
        tolerance: f64,
    ) -> Result<Self, DistributionError> {
        let total = entries.iter().map(|(_, value)| value.as_f64()).sum::<f64>();
        if (total - 1.0).abs() > tolerance {
            return Err(DistributionError::Total {
                total,
                members: entries.len(),
                tolerance,
            });
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

    /// The weighted position, rounded at twelve decimals to remove sum noise.
    /// A score runs from 0 to 9, so this drops no meaningful value.
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

impl fmt::Debug for Distribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        labelled_odds(formatter, "Distribution", &self.entries)
    }
}

/// Independent yes probabilities for tag labels, in user order.
#[derive(Clone, PartialEq)]
struct TagProbabilities(Vec<(String, Probability)>);

impl fmt::Debug for TagProbabilities {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        labelled_odds(formatter, "TagProbabilities", &self.0)
    }
}

/// A label may come from a record, so `Debug` shows the labels' total length
/// and every probability.
fn labelled_odds(
    formatter: &mut fmt::Formatter<'_>,
    name: &str,
    entries: &[(String, Probability)],
) -> fmt::Result {
    let odds: Vec<f64> = entries.iter().map(|(_, odds)| odds.as_f64()).collect();
    formatter
        .debug_struct(name)
        .field(
            "labels",
            &Withheld(entries.iter().map(|(label, _)| label.len()).sum()),
        )
        .field("probabilities", &odds)
        .finish()
}

impl Serialize for TagProbabilities {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(
            self.0
                .iter()
                .map(|(label, probability)| (label, probability)),
        )
    }
}

/// The label that led, which `Debug` withholds because a record may name it.
#[derive(Clone, PartialEq, Serialize)]
#[serde(transparent)]
struct Label(String);

impl fmt::Debug for Label {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Withheld(self.0.len()).fmt(formatter)
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
        pick: Label,
        probabilities: Distribution,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<Probability>,
    },
    /// Independent probabilities that each tag applies.
    Tag { probabilities: TagProbabilities },
    /// A place on named levels, with the odds of every level.
    Score {
        level: Label,
        probabilities: Distribution,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<Probability>,
    },
}

/// What the backend said, carrying no vendor field name.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub(crate) struct Answer(Shape);

/// The bare value one judgment prints on standard output.
#[derive(Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum Value {
    /// `decide`: `true`, `false`, or `null`.
    YesNo(Option<bool>),
    /// `choose`: the winning label, or `null` when the answer is unresolved.
    Choice(Option<String>),
    /// `tag`: every label whose yes probability reached the cut.
    Tag(Vec<String>),
    /// `score`: the weighted position on the levels.
    Score(f64),
}

/// A label may come from a record, so `Debug` shows each label's length.
impl fmt::Debug for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let withheld = |label: &String| Withheld(label.len());
        match self {
            Self::YesNo(value) => formatter.debug_tuple("YesNo").field(value).finish(),
            Self::Choice(label) => formatter
                .debug_tuple("Choice")
                .field(&label.as_ref().map(withheld))
                .finish(),
            Self::Tag(labels) => formatter
                .debug_tuple("Tag")
                .field(&labels.iter().map(withheld).collect::<Vec<_>>())
                .finish(),
            Self::Score(value) => formatter.debug_tuple("Score").field(value).finish(),
        }
    }
}

impl Value {
    /// The label `--raw` prints, or `None` when there is no label to print.
    #[must_use]
    pub(crate) fn label(&self) -> Option<&str> {
        match self {
            Self::Choice(label) => label.as_deref(),
            Self::YesNo(_) | Self::Tag(_) | Self::Score(_) => None,
        }
    }
}

#[cfg(test)]
#[path = "answer_distribution_tests.rs"]
mod distribution_tests;

impl Answer {
    /// Read a choice distribution in the order its options were sent.
    #[must_use]
    pub(crate) fn choice_probabilities(&self) -> Option<Vec<(&str, f64)>> {
        match &self.0 {
            Shape::Choice { probabilities, .. } => Some(
                probabilities
                    .entries
                    .iter()
                    .map(|(label, probability)| (label.as_str(), probability.as_f64()))
                    .collect(),
            ),
            _ => None,
        }
    }

    /// Read a choice answer's backend confidence when one was reported.
    #[must_use]
    pub(crate) fn choice_confidence(&self) -> Option<f64> {
        match &self.0 {
            Shape::Choice { confidence, .. } => confidence.map(Probability::as_f64),
            _ => None,
        }
    }

    /// Take a probability as the answer to a yes/no question.
    pub(crate) const fn new_yes_no(probability: Probability) -> Self {
        Self(Shape::YesNo { probability })
    }

    /// Take the odds of every option, or `None` when none carries odds.
    pub(crate) fn new_choice(
        probabilities: Distribution,
        confidence: Option<Probability>,
    ) -> Option<Self> {
        let (pick, _) = probabilities.leader()?;
        let pick = Label(pick.to_owned());
        Some(Self(Shape::Choice {
            pick,
            probabilities,
            confidence,
        }))
    }

    /// Take one independent yes probability per tag label.
    pub(crate) fn new_tag(probabilities: Vec<(String, Probability)>) -> Self {
        Self(Shape::Tag {
            probabilities: TagProbabilities(probabilities),
        })
    }

    /// Take the odds of every level as the answer to a placement.
    ///
    /// Returns `None` when no level carries odds, which no plan asks for.
    pub(crate) fn new_score(
        probabilities: Distribution,
        confidence: Option<Probability>,
    ) -> Option<Self> {
        let (level, _) = probabilities.leader()?;
        let level = Label(level.to_owned());
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
    pub(crate) fn read(&self, threshold: Option<Threshold>) -> (Value, Outcome) {
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
                    (Value::Choice(Some(pick.0.clone())), Outcome::Yes)
                } else {
                    (Value::Choice(None), Outcome::Unresolved)
                }
            }
            Shape::Tag { probabilities } => {
                let threshold = threshold.unwrap_or_default();
                let selected = probabilities
                    .0
                    .iter()
                    .filter(|(_, probability)| threshold.judge(*probability) == Outcome::Yes)
                    .map(|(label, _)| label.clone())
                    .collect();
                (Value::Tag(selected), Outcome::Yes)
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
    pub(crate) fn yes(&self) -> Option<f64> {
        match &self.0 {
            Shape::YesNo { probability } => Some(probability.as_f64()),
            Shape::Choice { .. } | Shape::Tag { .. } | Shape::Score { .. } => None,
        }
    }

    /// Every label's probability in declared order, or `None` for a yes/no
    /// answer.
    #[must_use]
    pub(crate) fn named(&self) -> Option<Vec<(&str, f64)>> {
        let entries = match &self.0 {
            Shape::YesNo { .. } => return None,
            Shape::Choice { probabilities, .. } | Shape::Score { probabilities, .. } => {
                &probabilities.entries
            }
            Shape::Tag { probabilities } => &probabilities.0,
        };
        Some(
            entries
                .iter()
                .map(|(label, probability)| (label.as_str(), probability.as_f64()))
                .collect(),
        )
    }

    /// The level a placement leads with, or `None` for any other answer.
    #[must_use]
    pub(crate) fn level(&self) -> Option<&str> {
        match &self.0 {
            Shape::Score { level, .. } => Some(&level.0),
            Shape::YesNo { .. } | Shape::Choice { .. } | Shape::Tag { .. } => None,
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
            Shape::Tag { .. } => None,
        }
    }

    /// The label that led, before any threshold, or `None` for a yes/no answer.
    #[cfg(test)]
    pub(crate) fn leader(&self) -> Option<&str> {
        match &self.0 {
            Shape::YesNo { .. } => None,
            Shape::Choice { pick: label, .. } | Shape::Score { level: label, .. } => Some(&label.0),
            Shape::Tag { .. } => None,
        }
    }

    /// The backend's own confidence, when the backend reported one.
    pub(crate) fn confidence(&self) -> Option<Probability> {
        match &self.0 {
            Shape::YesNo { .. } => None,
            Shape::Choice { confidence, .. } | Shape::Score { confidence, .. } => *confidence,
            Shape::Tag { .. } => None,
        }
    }
}

#[cfg(test)]
#[path = "answer_tests.rs"]
mod tests;
