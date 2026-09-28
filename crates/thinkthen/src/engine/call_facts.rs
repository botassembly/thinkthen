//! One call's receipts, shared by its workers and separate from process usage.

use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};
use std::{fmt, sync::MutexGuard};

use super::Cancel;
use crate::core::Usage;

#[derive(Clone)]
pub(crate) struct CallFacts(Arc<Mutex<State>>);

impl fmt::Debug for CallFacts {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CallFacts(<withheld>)")
    }
}

struct State {
    started: Instant,
    elapsed: Option<Duration>,
    records: u64,
    requests_sent: u64,
    cache_answers: u64,
    tokens: Option<Usage>,
    live_replies: u64,
    missing_usage: bool,
    model: Option<String>,
}

#[derive(Clone)]
pub(crate) struct Snapshot {
    pub(crate) elapsed: Duration,
    pub(crate) records: u64,
    pub(crate) requests_sent: u64,
    pub(crate) cache_answers: u64,
    pub(crate) tokens: Option<Usage>,
    pub(crate) model: Option<String>,
}

impl CallFacts {
    pub(crate) fn new() -> Self {
        Self(Arc::new(Mutex::new(State {
            started: Instant::now(),
            elapsed: None,
            records: 0,
            requests_sent: 0,
            cache_answers: 0,
            tokens: None,
            live_replies: 0,
            missing_usage: false,
            model: None,
        })))
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Called after the final stop and budget checks, after the process mark.
    pub(crate) fn sent(&self) {
        self.state().requests_sent += 1;
    }

    /// Count only a cache answer that the process counters also count.
    pub(crate) fn cache_answer(&self) {
        self.state().cache_answers += 1;
    }

    /// A received response counts even if its logical answer cannot be decoded.
    pub(crate) fn live_reply(&self, usage: Option<Usage>) {
        let mut state = self.state();
        state.live_replies += 1;
        match (state.tokens, usage) {
            (_, None) => state.missing_usage = true,
            (Some(previous), Some(next)) => {
                state.tokens = previous.checked_plus(next);
                state.missing_usage |= state.tokens.is_none();
            }
            (None, Some(next)) if !state.missing_usage => state.tokens = Some(next),
            (None, Some(_)) => {}
        }
    }

    pub(crate) fn answered_by(&self, model: &str) {
        self.state().model.get_or_insert_with(|| model.to_owned());
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
        Snapshot {
            elapsed: state.elapsed.unwrap_or_else(|| state.started.elapsed()),
            records: state.records,
            requests_sent: state.requests_sent,
            cache_answers: state.cache_answers,
            tokens: (!state.missing_usage && state.live_replies > 0)
                .then_some(state.tokens)
                .flatten(),
            model: state.model.clone(),
        }
    }
}

impl Cancel<'_> {
    pub(crate) fn with_facts(&self, facts: CallFacts) -> Self {
        Self {
            facts: Some(facts),
            ..self.clone()
        }
    }

    pub(crate) fn sent(&self) {
        if let Some(facts) = &self.facts {
            facts.sent();
        }
    }

    pub(crate) fn cache_answer(&self) {
        if let Some(facts) = &self.facts {
            facts.cache_answer();
        }
    }

    pub(crate) fn live_reply(&self, usage: Option<Usage>) {
        if let Some(facts) = &self.facts {
            facts.live_reply(usage);
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
