//! A probability from 0 to 1.

use serde::{Serialize, Serializer};

/// A finite probability from 0 to 1, inclusive.
///
/// Serialization rounds to two decimal places; [`Self::get`] retains full precision.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, schemars::JsonSchema)]
#[schemars(transparent)]
pub struct Probability(#[schemars(range(min = 0, max = 1))] f64);

impl Probability {
    /// The probability `value`, or `None` when it is not finite or lies outside 0 to 1.
    #[must_use]
    pub const fn new(value: f64) -> Option<Self> {
        if value.is_finite() && value >= 0.0 && value <= 1.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// One half, the default cut.
    pub(crate) const HALF: Self = Self(0.5);

    /// The probability as a number.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl Serialize for Probability {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_f64((self.0 * 100.0).round() / 100.0)
    }
}
