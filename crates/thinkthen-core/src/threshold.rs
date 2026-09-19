//! The threshold: one cut, or a band that leaves a middle unresolved.

use std::fmt;
use std::str::FromStr;

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::answer::Answer;

/// Why a value is not a threshold.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ThresholdError {
    /// The text is not a decimal fraction, and not two of them around a colon.
    #[error("a threshold is a decimal fraction, or two of them as LOW:HIGH")]
    NotANumber,
    /// A side is not-a-number or an infinity.
    #[error("a threshold is a finite number")]
    NotFinite,
    /// A single cut sits outside zero to one, as a percent such as 90 does.
    #[error("a single cut is above zero and at most one")]
    CutOutOfRange,
    /// A side of a band sits outside zero to one.
    #[error("a band runs from zero to one")]
    BandOutOfRange,
    /// The low side reaches the high side, so the band holds nothing.
    #[error("a band's low side is below its high side")]
    BandNotRising,
}

/// What the rule made of one answer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    /// The probability reached the mark.
    Yes,
    /// The probability did not reach the mark.
    No,
    /// The probability sits inside the band.
    Unresolved,
}

impl Outcome {
    /// The bare value this outcome prints, which is `null` when unresolved.
    #[must_use]
    pub const fn value(self) -> Option<bool> {
        match self {
            Self::Yes => Some(true),
            Self::No => Some(false),
            Self::Unresolved => None,
        }
    }
}

/// The rule a judgment is read under.
///
/// A single cut answers yes or no and never leaves an answer unresolved. A band
/// answers yes at or above the high side, no at or below the low side, and
/// leaves the middle unresolved. Boundaries are inclusive.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Threshold {
    /// One cut, above zero and at most one.
    Cut(f64),
    /// A band, with the low side below the high side.
    Band {
        /// At or below this the answer is no.
        low: f64,
        /// At or above this the answer is yes.
        high: f64,
    },
}

/// The cut a user who names no threshold is judged under.
const DEFAULT_CUT: f64 = 0.5;

impl Default for Threshold {
    /// Take the cut of one half, which is the rule when the user names none.
    fn default() -> Self {
        Self::Cut(DEFAULT_CUT)
    }
}

impl Threshold {
    /// Take one cut above zero and at most one.
    ///
    /// # Errors
    ///
    /// Returns [`ThresholdError`] when the number is not finite or falls
    /// outside the range the cut allows.
    pub fn cut(value: f64) -> Result<Self, ThresholdError> {
        if !value.is_finite() {
            return Err(ThresholdError::NotFinite);
        }
        if value <= 0.0 || value > 1.0 {
            return Err(ThresholdError::CutOutOfRange);
        }
        Ok(Self::Cut(value))
    }

    /// Take a band whose low side is below its high side.
    ///
    /// # Errors
    ///
    /// Returns [`ThresholdError`] when a side is not finite, when a side falls
    /// outside zero to one, or when the low side reaches the high side.
    pub fn band(low: f64, high: f64) -> Result<Self, ThresholdError> {
        if !low.is_finite() || !high.is_finite() {
            return Err(ThresholdError::NotFinite);
        }
        if !(0.0..=1.0).contains(&low) || !(0.0..=1.0).contains(&high) {
            return Err(ThresholdError::BandOutOfRange);
        }
        if low >= high {
            return Err(ThresholdError::BandNotRising);
        }
        Ok(Self::Band { low, high })
    }

    /// Read one answer under this rule.
    #[must_use]
    pub fn judge(self, answer: Answer) -> Outcome {
        let probability = answer.probability().as_f64();
        match self {
            Self::Cut(mark) => {
                if probability >= mark {
                    Outcome::Yes
                } else {
                    Outcome::No
                }
            }
            Self::Band { low, high } => {
                if probability >= high {
                    Outcome::Yes
                } else if probability <= low {
                    Outcome::No
                } else {
                    Outcome::Unresolved
                }
            }
        }
    }
}

impl FromStr for Threshold {
    type Err = ThresholdError;

    /// Read `T` as a cut and `LOW:HIGH` as a band.
    ///
    /// # Errors
    ///
    /// Returns [`ThresholdError`] when the text is not a decimal fraction or a
    /// pair of them, or when the numbers break the rule.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let number = |side: &str| side.parse::<f64>().map_err(|_| ThresholdError::NotANumber);
        match text.split_once(':') {
            Some((low, high)) => Self::band(number(low)?, number(high)?),
            None => Self::cut(number(text)?),
        }
    }
}

impl fmt::Display for Threshold {
    /// Write the rule the way `--threshold` takes it back.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Cut(mark) => write!(formatter, "{mark}"),
            Self::Band { low, high } => write!(formatter, "{low}:{high}"),
        }
    }
}

impl Serialize for Threshold {
    /// Write a cut as a number and a band as the string `LOW:HIGH`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match *self {
            Self::Cut(mark) => serializer.serialize_f64(mark),
            Self::Band { .. } => serializer.collect_str(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Outcome, Threshold, ThresholdError};
    use crate::answer::Answer;
    use crate::probability::Probability;
    use proptest::strategy::Strategy;
    use proptest::{prop_assert, prop_assert_eq, proptest};

    fn answer(probability: f64) -> Answer {
        Answer::new_yes_no(Probability::new(probability).expect("a probability"))
    }

    fn rule(text: &str) -> Threshold {
        text.parse().expect("a threshold")
    }

    #[test]
    fn the_worked_boundaries_follow_the_specification() {
        let none = Threshold::default();
        let cut = rule("0.9");
        let band = rule("0.1:0.9");
        let cases = [
            (0.0, none, Outcome::No),
            (0.1, none, Outcome::No),
            (0.5, none, Outcome::Yes),
            (0.9, none, Outcome::Yes),
            (1.0, none, Outcome::Yes),
            (0.0, cut, Outcome::No),
            (0.1, cut, Outcome::No),
            (0.5, cut, Outcome::No),
            (0.9, cut, Outcome::Yes),
            (1.0, cut, Outcome::Yes),
            (0.0, band, Outcome::No),
            (0.1, band, Outcome::No),
            (0.5, band, Outcome::Unresolved),
            (0.9, band, Outcome::Yes),
            (1.0, band, Outcome::Yes),
        ];
        for (probability, threshold, expected) in cases {
            assert_eq!(
                threshold.judge(answer(probability)),
                expected,
                "probability {probability} under {threshold:?}"
            );
        }
    }

    #[test]
    fn no_threshold_and_a_cut_of_one_half_name_the_same_rule() {
        assert_eq!(Threshold::default(), rule("0.5"));
    }

    #[test]
    fn an_outcome_carries_the_bare_value_it_prints() {
        assert_eq!(Outcome::Yes.value(), Some(true));
        assert_eq!(Outcome::No.value(), Some(false));
        assert_eq!(Outcome::Unresolved.value(), None);
    }

    #[test]
    fn a_threshold_parses_from_a_cut_and_from_a_band() {
        assert_eq!(rule("1"), Threshold::Cut(1.0));
        assert_eq!(rule("0.9"), Threshold::Cut(0.9));
        assert_eq!(
            rule("0:1"),
            Threshold::Band {
                low: 0.0,
                high: 1.0
            }
        );
    }

    #[test]
    fn what_is_not_a_threshold_names_its_own_cause() {
        let cases = [
            ("90", ThresholdError::CutOutOfRange),
            ("0", ThresholdError::CutOutOfRange),
            ("-0.5", ThresholdError::CutOutOfRange),
            ("1.0001", ThresholdError::CutOutOfRange),
            ("inf", ThresholdError::NotFinite),
            ("NaN", ThresholdError::NotFinite),
            ("0.9:0.1", ThresholdError::BandNotRising),
            ("0.9:0.9", ThresholdError::BandNotRising),
            ("0.1:90", ThresholdError::BandOutOfRange),
            ("-0.1:0.9", ThresholdError::BandOutOfRange),
            (":0.9", ThresholdError::NotANumber),
            ("0.1:", ThresholdError::NotANumber),
            ("", ThresholdError::NotANumber),
            ("half", ThresholdError::NotANumber),
            ("0.1:0.5:0.9", ThresholdError::NotANumber),
            ("50%", ThresholdError::NotANumber),
        ];
        for (text, expected) in cases {
            assert_eq!(text.parse::<Threshold>(), Err(expected), "{text:?}");
        }
    }

    #[test]
    fn a_threshold_prints_as_a_number_or_as_the_band_it_was_given() {
        let rendered = |text: &str| serde_json::to_string(&rule(text)).expect("a threshold");
        assert_eq!(rendered("0.5"), "0.5");
        assert_eq!(rendered("0.9"), "0.9");
        assert_eq!(rendered("0.1:0.9"), r#""0.1:0.9""#);
        assert_eq!(rendered("0:1"), r#""0:1""#);
    }

    fn probabilities() -> impl Strategy<Value = f64> {
        0.0_f64..=1.0
    }

    fn cuts() -> impl Strategy<Value = f64> {
        (0.0_f64..=1.0).prop_filter("a cut is above zero", |value| *value > 0.0)
    }

    proptest! {
        /// Every probability reaches one outcome, and the value the page fixes.
        #[test]
        fn a_cut_answers_yes_at_or_above_the_mark_and_no_below_it(
            probability in probabilities(),
            mark in cuts(),
        ) {
            let threshold = Threshold::cut(mark).expect("a cut");
            let outcome = threshold.judge(answer(probability));
            prop_assert_eq!(outcome == Outcome::Yes, probability >= mark);
            prop_assert_eq!(outcome == Outcome::No, probability < mark);
            prop_assert!(outcome != Outcome::Unresolved);
        }

        #[test]
        fn a_band_answers_yes_above_it_no_below_it_and_nothing_inside_it(
            probability in probabilities(),
            low in probabilities(),
            high in probabilities(),
        ) {
            let Ok(threshold) = Threshold::band(low, high) else {
                prop_assert!(low >= high);
                return Ok(());
            };
            let outcome = threshold.judge(answer(probability));
            prop_assert_eq!(outcome == Outcome::Yes, probability >= high);
            prop_assert_eq!(outcome == Outcome::No, probability <= low);
            prop_assert_eq!(
                outcome == Outcome::Unresolved,
                probability > low && probability < high
            );
        }

        /// A rule that printed into a result works again on the command line.
        #[test]
        fn a_printed_threshold_parses_back_to_the_same_rule(
            low in probabilities(),
            high in probabilities(),
            mark in cuts(),
        ) {
            let cut = Threshold::cut(mark).expect("a cut");
            prop_assert_eq!(cut.to_string().parse::<Threshold>(), Ok(cut));
            if let Ok(band) = Threshold::band(low, high) {
                prop_assert_eq!(band.to_string().parse::<Threshold>(), Ok(band));
            }
        }
    }
}
