//! The five counts one month file holds. A file without `retries` reads as zero.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Counts {
    pub(super) schema: UsageSchema,
    pub(crate) requests_sent: u64,
    #[serde(default)]
    pub(crate) retries: u64,
    pub(crate) input_tokens: u64,
    pub(crate) output_tokens: u64,
    pub(crate) cache_answers: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) enum UsageSchema {
    #[serde(rename = "thinkthen.usage/1")]
    One,
}

impl Default for Counts {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Counts {
    pub(super) const ZERO: Self = Self {
        schema: UsageSchema::One,
        requests_sent: 0,
        retries: 0,
        input_tokens: 0,
        output_tokens: 0,
        cache_answers: 0,
    };

    pub(super) fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self {
            schema: UsageSchema::One,
            requests_sent: self.requests_sent.checked_add(other.requests_sent)?,
            retries: self.retries.checked_add(other.retries)?,
            input_tokens: self.input_tokens.checked_add(other.input_tokens)?,
            output_tokens: self.output_tokens.checked_add(other.output_tokens)?,
            cache_answers: self.cache_answers.checked_add(other.cache_answers)?,
        })
    }
}
