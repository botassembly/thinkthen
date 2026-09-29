//! Typed call-level controls for the eager Rust Polars column door.

use std::fmt;

use crate::core::{self, Question as CoreQuestion};
use crate::public::question::Kind;
use crate::public::{CallOptions, Description, Error, Question, QuestionKind};

/// Controls one eager Polars column call, leaving the original question owned
/// by the caller. The threshold and meanings use the pure question grammar.
#[derive(Clone, Copy)]
pub struct PolarsCallOptions<'a> {
    call: CallOptions<'a>,
    threshold: Option<&'a str>,
    yes: Option<&'a Description>,
    no: Option<&'a Description>,
    probability: bool,
}

impl fmt::Debug for PolarsCallOptions<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PolarsCallOptions")
            .field("threshold", &self.threshold.is_some())
            .field("true", &self.yes.is_some())
            .field("false", &self.no.is_some())
            .field("probability", &self.probability)
            .finish_non_exhaustive()
    }
}

impl<'a> PolarsCallOptions<'a> {
    /// Start with no call overrides.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            call: CallOptions::new(),
            threshold: None,
            yes: None,
            no: None,
            probability: false,
        }
    }

    /// Carry cancellation, deadline, batch and context controls.
    #[must_use]
    pub const fn call(mut self, value: CallOptions<'a>) -> Self {
        self.call = value;
        self
    }

    /// One cut or a `LOW:HIGH` band under the shared threshold grammar.
    #[must_use]
    pub const fn threshold(mut self, value: &'a str) -> Self {
        self.threshold = Some(value);
        self
    }

    /// What a yes answer means, on decide alone.
    #[must_use]
    pub const fn true_meaning(mut self, value: &'a Description) -> Self {
        self.yes = Some(value);
        self
    }

    /// What a no answer means, on decide alone.
    #[must_use]
    pub const fn false_meaning(mut self, value: &'a Description) -> Self {
        self.no = Some(value);
        self
    }

    /// Include a probability column on decide or choose.
    #[must_use]
    pub const fn probability(mut self, value: bool) -> Self {
        self.probability = value;
        self
    }

    pub(super) fn apply(
        self,
        source: &Question,
    ) -> Result<(Question, CallOptions<'a>, bool), Error> {
        let mut question = source.clone();
        if let Some(text) = self.threshold {
            if question.kind() == QuestionKind::Score {
                return Err(Error::usage("threshold does not belong to score"));
            }
            let rule = text.parse::<core::Threshold>().map_err(Error::refused)?;
            question.threshold = Some(rule);
            if matches!(question.kind, Kind::Decide | Kind::Banded) {
                question.kind = match rule.is_cut() {
                    true => Kind::Decide,
                    false => Kind::Banded,
                };
            }
        }
        if self.yes.is_some() || self.no.is_some() {
            let CoreQuestion::Decide { yes, no, .. } = &mut question.core else {
                return Err(Error::usage("true and false meanings belong to decide"));
            };
            if let Some(value) = self.yes {
                *yes = Some(value.meaning()?);
            }
            if let Some(value) = self.no {
                *no = Some(value.meaning()?);
            }
        }
        Ok((question, self.call, self.probability))
    }
}

impl Default for PolarsCallOptions<'_> {
    fn default() -> Self {
        Self::new()
    }
}
