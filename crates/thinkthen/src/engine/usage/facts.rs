//! Process-only facts that never enter the count-only usage file.

use std::sync::PoisonError;

use crate::core::{ModelName, ReportedSum, ReportedUsage};

use super::{Counters, Counts};

#[derive(Debug)]
pub(super) struct State {
    largest_request_bytes: usize,
    largest_request_estimated_input_tokens: Option<u64>,
    live_replies: u64,
    reported: ReportedSum,
    model: Option<ModelName>,
    mixed_models: bool,
    pub(super) token_sum_valid: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            largest_request_bytes: 0,
            largest_request_estimated_input_tokens: Some(0),
            live_replies: 0,
            reported: ReportedSum::default(),
            model: None,
            mixed_models: false,
            token_sum_valid: true,
        }
    }
}

impl Counters {
    /// Measure bodies only after their live attempt is marked.
    pub(crate) fn request_body_sent(&self, bytes: usize, tokens: Option<u64>) {
        let mut queue = self
            .shared
            .queue
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        queue.facts.largest_request_bytes = queue.facts.largest_request_bytes.max(bytes);
        queue.facts.largest_request_estimated_input_tokens = queue
            .facts
            .largest_request_estimated_input_tokens
            .zip(tokens)
            .map(|(held, next)| held.max(next));
    }

    pub(crate) fn held_model_mismatch(&self) {
        self.held_model_mismatch
            .store(true, std::sync::atomic::Ordering::Release);
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
}
