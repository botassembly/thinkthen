//! The rule that turns a probability into yes, no or not sure.

use std::str::FromStr;

use crate::core::error::{Error, ErrorKind};
use crate::core::probability::Probability;

/// What a threshold makes of a probability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The probability reaches the yes side.
    Yes,
    /// The probability falls on the no side.
    No,
    /// The probability sits inside a band.
    Unsure,
}

/// A single cut, or a band with a not sure middle.
///
/// A cut `T` says yes at `p >= T`. A band `LOW:HIGH` says yes at `p >= HIGH`,
/// no below `LOW`, and not sure from `LOW` up to `HIGH`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Threshold(Rule);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rule {
    Cut(Probability),
    Band(Probability, Probability),
}

impl Threshold {
    /// Require a single cut for a question that cannot return not sure.
    ///
    /// # Errors
    ///
    /// `usage.threshold` when this threshold is a band.
    pub fn require_cut(self) -> Result<Self, Error> {
        match self.0 {
            Rule::Cut(_) => Ok(self),
            Rule::Band(..) => Err(Error::new(
                ErrorKind::Threshold,
                "this function requires a single threshold cut",
            )),
        }
    }

    /// What this threshold makes of `p`.
    #[must_use]
    pub fn verdict(self, p: Probability) -> Verdict {
        match self.0 {
            Rule::Cut(cut) if p >= cut => Verdict::Yes,
            Rule::Cut(_) => Verdict::No,
            Rule::Band(_, high) if p >= high => Verdict::Yes,
            Rule::Band(low, _) if p < low => Verdict::No,
            Rule::Band(..) => Verdict::Unsure,
        }
    }
}

impl Default for Threshold {
    fn default() -> Self {
        Self(Rule::Cut(Probability::HALF))
    }
}

impl FromStr for Threshold {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Error> {
        let refused = || {
            Error::new(
                ErrorKind::Threshold,
                format!(
                    "the threshold `{text}` must be a cut above 0 and at most 1, \
                     or a band LOW:HIGH with 0 < LOW < HIGH <= 1"
                ),
            )
        };
        // A bound of 0 leaves no probability below it, so it could never say no.
        let number = |part: &str| {
            part.parse::<f64>()
                .ok()
                .and_then(Probability::new)
                .filter(|bound| bound.get() > 0.0)
                .ok_or_else(refused)
        };
        let rule = match text.split_once(':') {
            None => Rule::Cut(number(text)?),
            Some((low, high)) => {
                let (low, high) = (number(low)?, number(high)?);
                if low >= high {
                    return Err(refused());
                }
                Rule::Band(low, high)
            }
        };
        Ok(Self(rule))
    }
}

impl serde::Serialize for Threshold {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Rule::Cut(cut) => serializer.serialize_f64(cut.get()),
            Rule::Band(low, high) => {
                serde::Serialize::serialize(&(low.get(), high.get()), serializer)
            }
        }
    }
}

impl std::fmt::Display for Threshold {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Rule::Cut(cut) => write!(formatter, "{}", cut.get()),
            Rule::Band(low, high) => write!(formatter, "{}:{}", low.get(), high.get()),
        }
    }
}
