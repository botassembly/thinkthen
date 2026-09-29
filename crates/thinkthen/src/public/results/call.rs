//! One eager result and the count-only facts of the call that produced it.

use std::fmt;

use crate::core::Prices;
use crate::engine::call_facts::Snapshot;
use crate::public::error::Error;

/// Count-only facts fixed when one call and all of its workers finish.
#[derive(Clone)]
pub struct Facts {
    pub(super) records: u64,
    pub(super) requests_sent: u64,
    pub(super) cache_answers: u64,
    pub(super) input_tokens: Option<u64>,
    pub(super) output_tokens: Option<u64>,
    pub(super) estimated_cost_usd: Option<String>,
    pub(super) seconds: f64,
    pub(super) model: Option<String>,
}

impl fmt::Debug for Facts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Facts")
            .field("records", &self.records)
            .field("requests_sent", &self.requests_sent)
            .field("cache_answers", &self.cache_answers)
            .field("input_tokens", &self.input_tokens)
            .field("output_tokens", &self.output_tokens)
            .field("estimated_cost_usd", &self.estimated_cost_usd.is_some())
            .field("seconds", &self.seconds)
            .field("model", &self.model.is_some())
            .finish()
    }
}

impl Facts {
    pub(crate) fn of(snapshot: Snapshot, prices: Option<Prices>) -> Self {
        let tokens = snapshot.tokens.map(crate::core::Usage::token_counts);
        let estimated_cost_usd = prices.and_then(|prices| {
            if !snapshot.cost_complete {
                return None;
            }
            let (input, output) = tokens.unwrap_or((0, 0));
            prices.estimate(input, output)
        });
        Self {
            records: snapshot.records,
            requests_sent: snapshot.requests_sent,
            cache_answers: snapshot.cache_answers,
            input_tokens: tokens.map(|(input, _)| input),
            output_tokens: tokens.map(|(_, output)| output),
            estimated_cost_usd,
            seconds: snapshot.elapsed.as_secs_f64(),
            model: snapshot.model,
        }
    }

    /// Ordered input records finished before a stop, including filtered rows.
    #[must_use]
    pub const fn records(&self) -> u64 {
        self.records
    }

    /// Live transport attempts, including retries and failed attempts.
    #[must_use]
    pub const fn requests_sent(&self) -> u64 {
        self.requests_sent
    }

    /// Answers obtained from the answer cache without a live send.
    #[must_use]
    pub const fn cache_answers(&self) -> u64 {
        self.cache_answers
    }

    /// Backend-reported input tokens, absent when a live reply omitted usage.
    #[must_use]
    pub const fn input_tokens(&self) -> Option<u64> {
        self.input_tokens
    }

    /// Backend-reported output tokens, absent when a live reply omitted usage.
    #[must_use]
    pub const fn output_tokens(&self) -> Option<u64> {
        self.output_tokens
    }

    /// Caller-priced estimate from complete reported usage, if configured.
    #[must_use]
    pub fn estimated_cost_usd(&self) -> Option<&str> {
        self.estimated_cost_usd.as_deref()
    }

    /// Wall seconds from call start through worker join and final stop check.
    #[must_use]
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    /// The validated model a reply reported, when one was received.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }
}

/// One eager value with its call-scoped facts.
pub struct Call<T> {
    value: T,
    facts: Facts,
}

impl<T> fmt::Debug for Call<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Call")
            .field("facts", &self.facts)
            .finish_non_exhaustive()
    }
}

impl<T> Call<T> {
    pub(crate) const fn new(value: T, facts: Facts) -> Self {
        Self { value, facts }
    }

    /// Borrow the value without losing its facts.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Take the value when an adapter retains an older outward shape.
    #[must_use]
    pub fn into_value(self) -> T {
        self.value
    }

    /// The immutable facts of this completed call.
    #[must_use]
    pub const fn facts(&self) -> &Facts {
        &self.facts
    }

    pub(crate) fn map<U>(self, map: impl FnOnce(T) -> U) -> Call<U> {
        Call::new(map(self.value), self.facts)
    }

    pub(crate) fn try_map<U>(
        self,
        map: impl FnOnce(T) -> Result<U, Error>,
    ) -> Result<Call<U>, Error> {
        match map(self.value) {
            Ok(value) => Ok(Call::new(value, self.facts)),
            Err(error) => Err(error.with_facts(self.facts)),
        }
    }
}
