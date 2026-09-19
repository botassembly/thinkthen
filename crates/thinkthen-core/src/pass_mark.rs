//! A pass mark: the symmetric probability a judgment must reach to be accepted.

use serde::Serialize;
use thiserror::Error;

/// Why a number is not a pass mark.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PassMarkError {
    /// The number is not-a-number or an infinity.
    #[error("a pass mark is a finite number")]
    NotFinite,
    /// The number is one half or below, so a coin would pass it.
    #[error("a pass mark is above one half")]
    NotAboveOneHalf,
    /// The number is above one, so no probability could reach it.
    #[error("a pass mark is at most one")]
    AboveOne,
}

/// The probability a judgment must reach before local policy accepts it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(into = "f64")]
pub struct PassMark(f64);

impl PassMark {
    /// Take a number above one half and at most one as a pass mark.
    ///
    /// # Errors
    ///
    /// Returns [`PassMarkError`] when the number is not finite, is one half or
    /// below, or is above one.
    pub fn new(value: f64) -> Result<Self, PassMarkError> {
        if !value.is_finite() {
            return Err(PassMarkError::NotFinite);
        }
        if value <= 0.5 {
            return Err(PassMarkError::NotAboveOneHalf);
        }
        if value > 1.0 {
            return Err(PassMarkError::AboveOne);
        }
        Ok(Self(value))
    }

    /// Read the pass mark back as a number.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0
    }
}

impl From<PassMark> for f64 {
    fn from(mark: PassMark) -> Self {
        mark.0
    }
}

#[cfg(test)]
mod tests {
    use super::{PassMark, PassMarkError};

    #[test]
    fn new_accepts_a_number_above_one_half_and_at_most_one() {
        let cases = [0.500_001, 0.6, 0.9, 0.99, 1.0];
        for case in cases {
            let mark = PassMark::new(case).expect("inside the range");
            assert!((mark.as_f64() - case).abs() < f64::EPSILON, "mark {case}");
        }
    }

    #[test]
    fn new_refuses_what_is_not_a_pass_mark() {
        let cases = [
            (f64::NAN, PassMarkError::NotFinite),
            (f64::INFINITY, PassMarkError::NotFinite),
            (f64::NEG_INFINITY, PassMarkError::NotFinite),
            (0.5, PassMarkError::NotAboveOneHalf),
            (0.0, PassMarkError::NotAboveOneHalf),
            (-1.0, PassMarkError::NotAboveOneHalf),
            (1.000_001, PassMarkError::AboveOne),
        ];
        for (case, expected) in cases {
            assert_eq!(PassMark::new(case), Err(expected), "mark {case}");
        }
    }
}
