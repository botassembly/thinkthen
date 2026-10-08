//! One call's receipts, shared by its workers and separate from process usage.

use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};
use std::{fmt, sync::MutexGuard};

use super::Cancel;
use crate::core::{CallId, ReportedSum, ReportedUsage, Surface, Usage};

#[derive(Clone)]
pub(crate) struct CallFacts(Arc<Mutex<State>>);

impl fmt::Debug for CallFacts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CallFacts(<withheld>)")
    }
}

struct State {
    attempts: Option<Vec<crate::core::AttemptObservation>>,
    invocation: super::invocation::Context,
    started: Instant,
    elapsed: Option<Duration>,
    records: u64,
    requests_sent: u64,
    largest_request_bytes: usize,
    cache_answers: u64,
    tokens: Option<Usage>,
    reported: ReportedSum,
    live_replies: u64,
    missing_usage: bool,
    held_model_mismatch: bool,
    token_sum_valid: bool,
    model: Option<String>,
    mixed_models: bool,
}

#[derive(Clone)]
pub(crate) struct Snapshot {
    pub(crate) attempts: Option<Vec<crate::core::AttemptObservation>>,
    pub(crate) call_id: Option<CallId>,
    pub(crate) elapsed: Duration,
    pub(crate) records: u64,
    pub(crate) requests_sent: u64,
    pub(crate) largest_request_bytes: usize,
    pub(crate) cache_answers: u64,
    pub(crate) tokens: Option<Usage>,
    pub(crate) reported: Option<ReportedUsage>,
    pub(crate) cost_complete: bool,
    pub(crate) held_model_mismatch: bool,
    pub(crate) model: Option<String>,
}

impl CallFacts {
    pub(crate) fn capture_attempts(&self, requested: bool) {
        self.state().attempts = requested.then(Vec::new);
    }

    pub(crate) fn wants_attempts(&self) -> bool {
        self.state().attempts.is_some()
    }

    pub(crate) fn attempt(&self, event: &crate::core::AttemptObservation) {
        if let Some(attempts) = &mut self.state().attempts {
            attempts.push(event.clone());
        }
    }
    pub(crate) fn start(surface: Surface) -> Result<Self, super::error::Error> {
        let facts = Self::new();
        let context = super::invocation::Context::new(surface);
        context.get()?;
        facts.state().invocation = context;
        Ok(facts)
    }

    pub(crate) fn new() -> Self {
        Self(Arc::new(Mutex::new(State {
            attempts: None,
            invocation: super::invocation::Context::default(),
            started: Instant::now(),
            elapsed: None,
            records: 0,
            requests_sent: 0,
            largest_request_bytes: 0,
            cache_answers: 0,
            tokens: None,
            reported: ReportedSum::default(),
            live_replies: 0,
            missing_usage: false,
            held_model_mismatch: false,
            token_sum_valid: true,
            model: None,
            mixed_models: false,
        })))
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Called after the final stop and budget checks, after the process mark.
    pub(crate) fn sent(&self, bytes: usize) {
        let mut state = self.state();
        state.requests_sent += 1;
        state.largest_request_bytes = state.largest_request_bytes.max(bytes);
    }

    /// Count only a cache answer that the process counters also count.
    pub(crate) fn cache_answer(&self) {
        self.state().cache_answers += 1;
    }

    /// A received response counts even if its logical answer cannot be decoded.
    pub(crate) fn live_reply(&self, reported: Option<ReportedUsage>) {
        let mut state = self.state();
        state.live_replies += 1;
        state.reported.add(reported);
        let usage = reported.and_then(ReportedUsage::complete);
        match (state.tokens, usage) {
            (_, None) => state.missing_usage = true,
            (Some(previous), Some(next)) => {
                state.tokens = previous.checked_plus(next);
                state.token_sum_valid &= state.tokens.is_some();
                state.missing_usage |= state.tokens.is_none();
            }
            (None, Some(next)) if !state.missing_usage => state.tokens = Some(next),
            (None, Some(_)) => {}
        }
    }

    pub(crate) fn answered_by(&self, model: &str) {
        let mut state = self.state();
        if state
            .model
            .as_deref()
            .is_some_and(|previous| previous != model)
        {
            state.mixed_models = true;
        } else if state.model.is_none() {
            state.model = Some(model.to_owned());
        }
    }

    pub(crate) fn finished_records(&self, records: usize) {
        self.state().records += u64::try_from(records).unwrap_or(u64::MAX);
    }

    pub(crate) fn finish(&self) {
        let mut state = self.state();
        if state.elapsed.is_none() {
            state.elapsed = Some(state.started.elapsed());
        }
    }

    pub(crate) fn snapshot(&self) -> Snapshot {
        let state = self.state();
        let attempts = state.attempts.clone().map(|mut attempts| {
            attempts.sort_by_key(crate::core::AttemptObservation::ordinal);
            attempts
        });
        Snapshot {
            attempts,
            call_id: state.invocation.call_id(),
            held_model_mismatch: state.held_model_mismatch,
            elapsed: state.elapsed.unwrap_or_else(|| state.started.elapsed()),
            records: state.records,
            requests_sent: state.requests_sent,
            largest_request_bytes: state.largest_request_bytes,
            cache_answers: state.cache_answers,
            reported: state.reported.total().ok().flatten(),
            tokens: (!state.missing_usage && state.live_replies > 0)
                .then_some(state.tokens)
                .flatten(),
            cost_complete: state.token_sum_valid
                && !state.missing_usage
                && state.live_replies == state.requests_sent,
            model: (!state.mixed_models).then(|| state.model.clone()).flatten(),
        }
    }
}

impl Cancel<'_> {
    pub(crate) fn held_model_mismatch(&self) {
        if let Some(facts) = &self.facts {
            facts.state().held_model_mismatch = true;
        }
    }

    /// Capture command detail attempts under the already established invocation.
    pub(crate) fn with_captured_attempts(&self, requested: bool) -> Self {
        if !requested {
            return self.clone();
        }
        let facts = self.facts.clone().unwrap_or_else(CallFacts::new);
        facts.state().invocation = self.invocation.clone();
        facts.capture_attempts(true);
        self.with_facts(facts)
    }

    pub(crate) fn with_facts(&self, facts: CallFacts) -> Self {
        let invocation = facts.state().invocation.clone();
        Self {
            invocation,
            facts: Some(facts),
            ..self.clone()
        }
    }

    pub(crate) fn sent(&self, bytes: usize) {
        self.sent_any
            .store(true, std::sync::atomic::Ordering::Release);
        if let Some(facts) = &self.facts {
            facts.sent(bytes);
        }
    }

    pub(crate) fn cache_answer(&self) {
        if let Some(facts) = &self.facts {
            facts.cache_answer();
        }
    }

    pub(crate) fn live_reply(&self, reported: Option<ReportedUsage>) {
        if let Some(facts) = &self.facts {
            facts.live_reply(reported);
        }
    }

    pub(crate) fn answered_by(&self, model: &str) {
        if let Some(facts) = &self.facts {
            facts.answered_by(model);
        }
    }

    pub(crate) fn finished_records(&self, records: usize) {
        if let Some(facts) = &self.facts {
            facts.finished_records(records);
        }
    }
}
