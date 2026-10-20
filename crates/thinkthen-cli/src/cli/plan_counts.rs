//! Command count-line presentation over the native public preview.
use serde::Serialize;
use thinkthen::PlanEstimate;

#[derive(Debug, Serialize)]
pub(crate) struct Counts {
    pub(crate) records: usize,
    pub(crate) requests: usize,
    estimated_bytes: usize,
    largest_request_bytes: usize,
    largest_request_estimated_input_tokens: u64,
    token_estimate_method: &'static str,
    estimated_input_tokens: TokenBand,
    pub(crate) upper_bound: bool,
}
#[derive(Debug, Serialize)]
struct TokenBand {
    lower: usize,
    upper: usize,
}
impl From<&PlanEstimate> for Counts {
    fn from(plan: &PlanEstimate) -> Self {
        let (lower, upper) = plan.estimated_input_tokens();
        Self {
            records: plan.records(),
            requests: plan.requests(),
            estimated_bytes: plan.estimated_bytes(),
            largest_request_bytes: plan.largest_request_bytes(),
            largest_request_estimated_input_tokens: plan.largest_request_estimated_input_tokens(),
            token_estimate_method: plan.token_estimate_method(),
            estimated_input_tokens: TokenBand { lower, upper },
            upper_bound: plan.upper_bound(),
        }
    }
}
