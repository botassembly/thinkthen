//! Process-only facts that never enter the count-only usage file.

use std::sync::PoisonError;

use crate::core::{ModelName, Usage};

use super::{Counters, Counts};

#[derive(Debug, Default)]
pub(super) struct State {
    records: u64,
    live_with_usage: u64,
    live_without_usage: u64,
    model: Option<ModelName>,
    mixed_models: bool,
}

/// One command's process counters and reply presence facts.
pub(crate) struct Snapshot {
    pub(crate) counts: Counts,
    pub(crate) records: u64,
    pub(crate) usage_known: bool,
    pub(crate) model: Option<String>,
}

impl Counters {
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
        let queue = self
            .shared
            .queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        Snapshot {
            counts: queue.totals,
            records: queue.facts.records,
            usage_known: queue.facts.live_with_usage > 0 && queue.facts.live_without_usage == 0,
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
