//! Checked native plan totals retain the existing public presentation contract.
use super::{PlanEstimate, from_summary_with_note};

// Retaining unrounded counters does not change the existing equality contract.
impl PartialEq for PlanEstimate {
    fn eq(&self, other: &Self) -> bool {
        self.records == other.records
            && self.requests == other.requests
            && self.estimated_bytes == other.estimated_bytes
            && self.largest_request_bytes == other.largest_request_bytes
            && self.largest_request_estimated_input_tokens
                == other.largest_request_estimated_input_tokens
            && self.token_estimate_method == other.token_estimate_method
            && self.token_estimate_note == other.token_estimate_note
            && self.estimated_input_tokens == other.estimated_input_tokens
            && self.upper_bound == other.upper_bound
            && self.first_body == other.first_body
            && self.dropped_detail == other.dropped_detail
            && self.question_requests == other.question_requests
            && self.details == other.details
    }
}
impl Eq for PlanEstimate {}

impl PlanEstimate {
    /// Combine ordered preview totals with native unrounded input accounting.
    /// Sums and maxima retain their existing meanings, including request bounds.
    /// The first nonempty body survives in order; all function-specific detail,
    /// group-count and prepared-request views are cleared. Description loss is
    /// retained when either constituent reports it. This opens and sends nothing.
    /// # Errors
    /// Refuses differing accounting modes or adapter caveats and checked overflow.
    pub fn combine(&self, other: &Self) -> Result<Self, super::Error> {
        if self.token_estimate_note != other.token_estimate_note {
            return Err(super::Error::usage("plans use different input accounting"));
        }
        let summary = self
            .summary
            .combine(&other.summary)
            .map_err(|error| super::Error::usage(error.to_string()))?;
        let mut combined = from_summary_with_note(summary, self.token_estimate_note)?;
        combined.first_body = self.first_body.clone().or_else(|| other.first_body.clone());
        combined.dropped_detail = self.dropped_detail || other.dropped_detail;
        Ok(combined)
    }
}
