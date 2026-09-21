//! A probability: one finite number from zero to one.

use serde::Serialize;
use thiserror::Error;

/// Why a number is not a probability.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum ProbabilityError {
    /// The number is not-a-number or an infinity.
    #[error("a probability is a finite number")]
    NotFinite,
    /// The number is below zero or above one.
    #[error("a probability is from zero to one")]
    OutOfRange,
}

/// How likely the backend judged the condition to hold.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(into = "f64")]
pub(crate) struct Probability(f64);

impl Probability {
    /// Take a finite number from zero to one as a probability.
    ///
    /// # Errors
    ///
    /// Returns [`ProbabilityError`] when the number is not finite or falls
    /// outside zero to one.
    pub(crate) fn new(value: f64) -> Result<Self, ProbabilityError> {
        if !value.is_finite() {
            return Err(ProbabilityError::NotFinite);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(ProbabilityError::OutOfRange);
        }
        Ok(Self(value))
    }

    /// Read the probability back as a number.
    #[must_use]
    pub(crate) const fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<Probability> for f64 {
    fn from(probability: Probability) -> Self {
        probability.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Probability, ProbabilityError};

    #[test]
    fn new_accepts_a_finite_number_from_zero_to_one() {
        let cases = [0.0, 0.1, 0.5, 0.9, 0.92, 1.0];
        for case in cases {
            let probability = Probability::new(case).expect("inside the range");
            assert!(
                (probability.as_f64() - case).abs() < f64::EPSILON,
                "probability {case}"
            );
        }
    }

    #[test]
    fn new_refuses_what_is_not_a_probability() {
        let cases = [
            (f64::NAN, ProbabilityError::NotFinite),
            (f64::INFINITY, ProbabilityError::NotFinite),
            (f64::NEG_INFINITY, ProbabilityError::NotFinite),
            (-0.000_001, ProbabilityError::OutOfRange),
            (-1.0, ProbabilityError::OutOfRange),
            (1.000_001, ProbabilityError::OutOfRange),
        ];
        for (case, expected) in cases {
            assert_eq!(Probability::new(case), Err(expected), "probability {case}");
        }
    }
}
