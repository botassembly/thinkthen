//! A whole input's preview, counted from the request bodies the real planner prepared.

use serde::Serialize;
use thiserror::Error;

use crate::core::batch::Batch;

/// No estimate may wrap into a smaller apparent cost.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("the planned input is too large to count")]
pub(crate) struct PlanTooLarge;

/// One pure full-input summary. Callers add validated records and the exact
/// request bodies from the production batcher or splitter in input order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PlanSummary {
    records: usize,
    requests: usize,
    estimated_bytes: usize,
    first_body: Option<Vec<u8>>,
    upper_bound: bool,
}

impl PlanSummary {
    pub(crate) fn new(upper_bound: bool) -> Self {
        Self {
            upper_bound,
            ..Self::default()
        }
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
        let requests = self.requests.checked_add(1).ok_or(PlanTooLarge)?;
        let bytes = self
            .estimated_bytes
            .checked_add(body.len())
            .ok_or(PlanTooLarge)?;
        if self.first_body.is_none() {
            self.first_body = Some(body.to_vec());
        }
        self.requests = requests;
        self.estimated_bytes = bytes;
        Ok(())
    }

    pub(crate) fn batch(&mut self, batch: &Batch) -> Result<(), PlanTooLarge> {
        self.request(&batch.body)
    }

    /// Staged work may need this many later requests after answers arrive;
    /// no body exists yet, so bytes count only the prepared requests.
    pub(crate) fn possible_requests(&mut self, count: usize) -> Result<(), PlanTooLarge> {
        self.requests = self.requests.checked_add(count).ok_or(PlanTooLarge)?;
        self.upper_bound = true;
        Ok(())
    }

    pub(crate) fn first_body(&self) -> Option<&[u8]> {
        self.first_body.as_deref()
    }

    /// Conservative whole-token band at the measured 0.516/0.908 rates.
    pub(crate) fn estimated_input_tokens(&self) -> Result<(usize, usize), PlanTooLarge> {
        let lower = self.estimated_bytes.checked_mul(516).ok_or(PlanTooLarge)? / 1000;
        let upper = self
            .estimated_bytes
            .checked_mul(908)
            .and_then(|number| number.checked_add(999))
            .ok_or(PlanTooLarge)?
            / 1000;
        Ok((lower, upper))
    }

    pub(crate) fn counts(&self) -> Result<PlanCounts, PlanTooLarge> {
        let (lower, upper) = self.estimated_input_tokens()?;
        Ok(PlanCounts {
            records: self.records,
            requests: self.requests,
            estimated_bytes: self.estimated_bytes,
            estimated_input_tokens: TokenBand { lower, upper },
            upper_bound: self.upper_bound,
        })
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct PlanCounts {
    records: usize,
    requests: usize,
    estimated_bytes: usize,
    estimated_input_tokens: TokenBand,
    upper_bound: bool,
}

#[derive(Debug, Serialize)]
struct TokenBand {
    lower: usize,
    upper: usize,
}

#[cfg(test)]
mod tests {
    use super::PlanSummary;

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
