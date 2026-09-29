//! Process-only facts that never enter the count-only usage file.

use std::sync::PoisonError;

use crate::core::{ModelName, Prices, Usage};

use super::{Counters, Counts};

#[derive(Debug)]
pub(super) struct State {
    records: u64,
    live_with_usage: u64,
    live_without_usage: u64,
    model: Option<ModelName>,
    mixed_models: bool,
    pub(super) token_sum_valid: bool,
    prices: Option<Prices>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            records: 0,
            live_with_usage: 0,
            live_without_usage: 0,
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
    pub(crate) usage_known: bool,
    pub(crate) estimated_cost_usd: Option<String>,
    pub(crate) model: Option<String>,
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
    pub(crate) fn live_reply(&self, usage: Option<Usage>) {
        if let Some(usage) = usage {
            self.tokens(usage);
        }
        let mut queue = self
            .shared
            .queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if usage.is_some() {
            queue.facts.live_with_usage = queue.facts.live_with_usage.saturating_add(1);
        } else {
            queue.facts.live_without_usage = queue.facts.live_without_usage.saturating_add(1);
        }
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
        let usage_known = !poisoned
            && queue.facts.token_sum_valid
            && queue.facts.live_with_usage > 0
            && queue.facts.live_without_usage == 0;
        let cost_complete = !poisoned
            && queue.facts.token_sum_valid
            && queue.facts.live_without_usage == 0
            && queue.facts.live_with_usage == queue.totals.requests_sent;
        Snapshot {
            counts: queue.totals,
            records: queue.facts.records,
            usage_known,
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
