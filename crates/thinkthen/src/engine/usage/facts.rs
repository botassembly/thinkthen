//! Process-only facts that never enter the count-only usage file.

use std::sync::PoisonError;

use crate::core::{ModelName, Prices, ReportedSum, ReportedUsage};

use super::{Counters, Counts};

#[derive(Debug)]
pub(super) struct State {
    records: u64,
    live_replies: u64,
    reported: ReportedSum,
    model: Option<ModelName>,
    mixed_models: bool,
    pub(super) token_sum_valid: bool,
    prices: Option<Prices>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            records: 0,
            live_replies: 0,
            reported: ReportedSum::default(),
            model: None,
            mixed_models: false,
            token_sum_valid: true,
            prices: None,
        }
    }
}

/// One command's process counters and reply presence facts.
pub(crate) struct Snapshot {
    pub(crate) counts: Counts,
    pub(crate) records: u64,
    pub(crate) reported: Option<ReportedUsage>,
    pub(crate) estimated_cost_usd: Option<String>,
    pub(crate) model: Option<String>,
    pub(crate) http_time: Option<std::time::Duration>,
}

impl Counters {
    pub(crate) fn set_prices(&self, prices: Option<Prices>) {
        self.shared
            .queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .facts
            .prices = prices;
    }
    /// Count an accepted command row, including one filtered from output.
    pub(crate) fn record_done(&self) {
        let mut queue = self
            .shared
            .queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        queue.facts.records = queue.facts.records.saturating_add(1);
    }

    /// Record whether a live reply supplied usage. Never invent zero tokens.
    pub(crate) fn live_reply(&self, usage: Option<ReportedUsage>) {
        if let Some(usage) = usage {
            // A missing dimension contributes no counter delta. Its absence
            // remains in ReportedSum, so these totals never become reply facts.
            self.add(Counts {
                input_tokens: usage.input_tokens().unwrap_or(0),
                output_tokens: usage.output_tokens().unwrap_or(0),
                ..Counts::default()
            });
        }
        let mut queue = self
            .shared
            .queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        queue.facts.live_replies = queue.facts.live_replies.saturating_add(1);
        queue.facts.reported.add(usage);
    }

    /// Remember the answer's reported model, including replayed answers.
    pub(crate) fn answered_by(&self, model: &ModelName) {
        let mut queue = self
            .shared
            .queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        match &queue.facts.model {
            None => queue.facts.model = Some(model.clone()),
            Some(first) if first != model => queue.facts.mixed_models = true,
            Some(_) => {}
        }
    }

    /// Read one consistent snapshot for the final command line.
    pub(crate) fn run_snapshot(&self) -> Snapshot {
        let lock = self.shared.queue.lock();
        let poisoned = lock.is_err();
        let queue = lock.unwrap_or_else(PoisonError::into_inner);
        let reported = (!poisoned)
            .then(|| queue.facts.reported.total().ok().flatten())
            .flatten();
        let cost_complete = !poisoned
            && queue.facts.token_sum_valid
            && queue.facts.live_replies == queue.totals.requests_sent
            && (queue.facts.live_replies == 0
                || reported.and_then(ReportedUsage::complete).is_some());
        Snapshot {
            http_time: self.shared.http.total(),
            counts: queue.totals,
            records: queue.facts.records,
            reported,
            estimated_cost_usd: queue.facts.prices.and_then(|prices| {
                cost_complete.then(|| {
                    prices.estimate(queue.totals.input_tokens, queue.totals.output_tokens)
                })?
            }),
            model: (!queue.facts.mixed_models)
                .then(|| {
                    queue
                        .facts
                        .model
                        .as_ref()
                        .map(ModelName::as_str)
                        .map(str::to_owned)
                })
                .flatten(),
        }
    }
}
