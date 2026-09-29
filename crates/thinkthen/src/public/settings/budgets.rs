//! Process admission settings on an engine builder.

use super::EngineBuilder;

impl EngineBuilder {
    /// Limit this engine's live attempts against the shared process count.
    /// `None` leaves this engine unbounded while its sends still count for
    /// other engines. Zero refuses every live attempt.
    #[must_use]
    pub fn max_requests_total(mut self, value: Option<u64>) -> Self {
        self.max_requests_total = value;
        self
    }

    /// Limit estimated input admitted for live final encoded bodies in this process.
    /// The estimate is ceil(body bytes × 908 / 1000), not reported tokens or billing.
    /// An unset engine still contributes to the retained count.
    #[must_use]
    pub fn max_estimated_input_tokens_total(mut self, value: Option<u64>) -> Self {
        self.max_estimated_input_tokens_total = value;
        self
    }
}
