//! A whole input's preview, counted from the request bodies the real planner prepared.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::core::backend::InputAccounting;
use thiserror::Error;

/// No estimate may wrap into a smaller apparent cost.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("the planned input is too large to count")]
pub(crate) struct PlanTooLarge;

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum PlanCombineError {
    #[error("plans use different input accounting")]
    Accounting,
    #[error(transparent)]
    TooLarge(#[from] PlanTooLarge),
}

/// One pure full-input summary. Callers add validated records and the exact
/// request bodies from the production batcher or splitter in input order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PlanSummary {
    records: usize,
    requests: usize,
    estimated_bytes: usize,
    token_bytes: usize,
    largest_token_bytes: usize,
    accounting: InputAccounting,
    largest_request_bytes: usize,
    first_body: Option<Vec<u8>>,
    upper_bound: bool,
}

impl PlanSummary {
    /// Combine native counters before rounding, with no encoding or input access.
    pub(crate) fn combine(&self, other: &Self) -> Result<Self, PlanCombineError> {
        if self.accounting != other.accounting {
            return Err(PlanCombineError::Accounting);
        }
        let add = |left: usize, right: usize| left.checked_add(right).ok_or(PlanTooLarge);
        Ok(Self {
            records: add(self.records, other.records)?,
            requests: add(self.requests, other.requests)?,
            estimated_bytes: add(self.estimated_bytes, other.estimated_bytes)?,
            token_bytes: add(self.token_bytes, other.token_bytes)?,
            largest_token_bytes: self.largest_token_bytes.max(other.largest_token_bytes),
            accounting: self.accounting,
            largest_request_bytes: self.largest_request_bytes.max(other.largest_request_bytes),
            first_body: self.first_body.clone().or_else(|| other.first_body.clone()),
            upper_bound: self.upper_bound || other.upper_bound,
        })
    }
    /// Public plans already retain their explicit first body; hidden counters do not.
    pub(crate) fn without_body(mut self) -> Self {
        self.first_body = None;
        self
    }

    pub(crate) const TOKEN_ESTIMATE_METHOD: &str = "encoded-body-bytes-908-v1";
    /// Version-one measured high rate, shared by preview and per-attempt admission.
    /// Callers choose whether to round a whole planned sum or each final body.
    pub(crate) fn estimated_input_high(bytes: u64) -> Option<u64> {
        bytes
            .checked_mul(908)?
            .checked_add(999)
            .map(|scaled| scaled / 1000)
    }

    pub(crate) fn new(upper_bound: bool) -> Self {
        Self {
            upper_bound,
            ..Self::default()
        }
    }

    pub(crate) const fn with_accounting(mut self, accounting: InputAccounting) -> Self {
        self.accounting = accounting;
        self
    }

    /// Count the final packed state once per question for routes with that
    /// observed accounting. This is a measured estimate, not a billed bound.
    pub(crate) fn accounting_bytes(
        body: &[u8],
        accounting: InputAccounting,
    ) -> Result<usize, PlanTooLarge> {
        if accounting == InputAccounting::EncodedBody {
            return Ok(body.len());
        }
        #[derive(Deserialize)]
        struct Packed<'a> {
            #[serde(borrow)]
            state: &'a RawValue,
            questions: BTreeMap<String, serde::de::IgnoredAny>,
        }
        let packed: Packed<'_> = serde_json::from_slice(body).map_err(|_| PlanTooLarge)?;
        packed
            .state
            .get()
            .len()
            .checked_mul(packed.questions.len().saturating_sub(1))
            .and_then(|extra| body.len().checked_add(extra))
            .ok_or(PlanTooLarge)
    }

    pub(crate) fn request_input_high(
        body: &[u8],
        accounting: InputAccounting,
    ) -> Result<u64, PlanTooLarge> {
        let bytes = Self::accounting_bytes(body, accounting)?;
        u64::try_from(bytes)
            .ok()
            .and_then(Self::estimated_input_high)
            .ok_or(PlanTooLarge)
    }

    pub(crate) fn record(&mut self) -> Result<(), PlanTooLarge> {
        self.records = self.records.checked_add(1).ok_or(PlanTooLarge)?;
        Ok(())
    }

    pub(crate) fn records_added(&mut self, count: usize) -> Result<(), PlanTooLarge> {
        self.records = self.records.checked_add(count).ok_or(PlanTooLarge)?;
        Ok(())
    }

    pub(crate) fn request(&mut self, body: &[u8]) -> Result<(), PlanTooLarge> {
        let token_bytes = Self::accounting_bytes(body, self.accounting)?;
        self.token_bytes = self
            .token_bytes
            .checked_add(token_bytes)
            .ok_or(PlanTooLarge)?;
        self.largest_token_bytes = self.largest_token_bytes.max(token_bytes);
        let requests = self.requests.checked_add(1).ok_or(PlanTooLarge)?;
        let bytes = self
            .estimated_bytes
            .checked_add(body.len())
            .ok_or(PlanTooLarge)?;
        if self.first_body.is_none() {
            self.first_body = Some(body.to_vec());
        }
        self.largest_request_bytes = self.largest_request_bytes.max(body.len());
        self.requests = requests;
        self.estimated_bytes = bytes;
        Ok(())
    }

    /// Staged work may need this many later requests after answers arrive;
    /// no body exists yet, so bytes count only the prepared requests.
    pub(crate) fn possible_requests(&mut self, count: usize) -> Result<(), PlanTooLarge> {
        self.requests = self.requests.checked_add(count).ok_or(PlanTooLarge)?;
        self.upper_bound = true;
        Ok(())
    }

    /// Every initial request consumes an admitted wire-question occurrence.
    /// The uninterrupted packed preview still owns the byte and token estimates.
    pub(crate) fn bound_requests(&mut self, occurrences: usize) {
        self.upper_bound |= occurrences > self.requests;
        self.requests = occurrences;
    }

    pub(crate) fn first_body(&self) -> Option<&[u8]> {
        self.first_body.as_deref()
    }

    /// Empirical whole-token band; provider billing can exceed these measured byte rates.
    pub(crate) fn estimated_input_tokens(&self) -> Result<(usize, usize), PlanTooLarge> {
        let lower = self.token_bytes.checked_mul(516).ok_or(PlanTooLarge)? / 1000;
        let upper = u64::try_from(self.token_bytes)
            .ok()
            .and_then(Self::estimated_input_high)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or(PlanTooLarge)?;
        Ok((lower, upper))
    }

    pub(crate) fn counts(&self) -> Result<PlanCounts, PlanTooLarge> {
        let (lower, upper) = self.estimated_input_tokens()?;
        Ok(PlanCounts {
            records: self.records,
            requests: self.requests,
            estimated_bytes: self.estimated_bytes,
            largest_request_bytes: self.largest_request_bytes,
            largest_request_estimated_input_tokens: Self::estimated_input_high(
                self.largest_token_bytes as u64,
            )
            .ok_or(PlanTooLarge)?,
            token_estimate_method: Self::TOKEN_ESTIMATE_METHOD,
            estimated_input_tokens: TokenBand { lower, upper },
            upper_bound: self.upper_bound,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct PlanCounts {
    pub(crate) records: usize,
    pub(crate) requests: usize,
    pub(crate) estimated_bytes: usize,
    pub(crate) largest_request_bytes: usize,
    pub(crate) largest_request_estimated_input_tokens: u64,
    pub(crate) token_estimate_method: &'static str,
    pub(crate) estimated_input_tokens: TokenBand,
    pub(crate) upper_bound: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct TokenBand {
    pub(crate) lower: usize,
    pub(crate) upper: usize,
}

#[cfg(test)]
mod tests {
    //! Stays: conservative token and byte arithmetic needs an independent exact-body accounting vector.
    use super::PlanSummary;

    #[test]
    fn combination_rounds_once_and_rejects_overflow_or_mixed_accounting() {
        use super::{InputAccounting, PlanCombineError};
        let mut first = PlanSummary::new(false);
        first.record().unwrap();
        first.request(b"123456").unwrap();
        let combined = first.combine(&first).unwrap();
        assert_eq!(first.estimated_input_tokens().unwrap(), (3, 6));
        assert_eq!(combined.estimated_input_tokens().unwrap(), (6, 11));
        assert_eq!(combined.counts().unwrap().records, 2);
        assert_eq!(combined.first_body(), Some(&b"123456"[..]));
        let other = PlanSummary::new(false).with_accounting(InputAccounting::RepeatedState);
        assert_eq!(first.combine(&other), Err(PlanCombineError::Accounting));
        for field in 0..4 {
            let mut limit = PlanSummary::new(false);
            match field {
                0 => limit.records = usize::MAX,
                1 => limit.requests = usize::MAX,
                2 => limit.estimated_bytes = usize::MAX,
                _ => limit.token_bytes = usize::MAX,
            }
            assert!(matches!(
                limit.combine(&first),
                Err(PlanCombineError::TooLarge(_))
            ));
        }
    }

    #[test]
    fn one_independent_body_pins_bytes_and_conservative_band() {
        let body = br#"{"state":"Refund me please.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}"#;
        assert_eq!(body.len(), 120);
        let mut summary = PlanSummary::new(false);
        summary.record().expect("one record");
        summary.request(body).expect("one request");
        assert_eq!(
            (summary.records, summary.requests, summary.estimated_bytes),
            (1, 1, 120)
        );
        assert_eq!(summary.first_body(), Some(body.as_slice()));
        assert_eq!(summary.estimated_input_tokens(), Ok((61, 109)));
        assert!(!summary.upper_bound);
    }
}
