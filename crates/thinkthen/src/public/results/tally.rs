//! Explicit facts across completed calls, shared by clones and threads.

use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Instant;

use super::{Call, Facts};
use crate::public::Error;

#[derive(Default)]
struct State {
    first: Option<Instant>,
    last: Option<Instant>,
    records: u64,
    requests_sent: u64,
    cache_answers: u64,
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    missing_usage: bool,
    model: Option<String>,
    mixed_models: bool,
}

/// One caller-owned sum of completed call facts. Clones share its state.
/// Its seconds span the first call start through the last call finish, even
/// when calls overlap; they are not the sum of call durations.
#[derive(Clone, Default)]
pub struct Tally(Arc<Mutex<State>>);

impl fmt::Debug for Tally {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Tally").finish_non_exhaustive()
    }
}

/// A started call's wall-time boundary. Finish it with the facts of a complete
/// or partially failed call; an error before a call starts has no facts.
#[derive(Debug)]
pub struct TallyStart<'a> {
    tally: &'a Tally,
    started: Instant,
}

impl Tally {
    /// Start an empty, explicit tally.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Capture the start before a call or a stream morsel begins.
    #[must_use]
    pub fn start(&self) -> TallyStart<'_> {
        TallyStart {
            tally: self,
            started: Instant::now(),
        }
    }

    /// Run one eager call and include its final facts, including partial
    /// failure facts. A validation error before a call starts adds nothing.
    ///
    /// # Errors
    /// Returns the call error or an accounting overflow.
    pub fn run<T>(&self, call: impl FnOnce() -> Result<Call<T>, Error>) -> Result<Call<T>, Error> {
        let started = self.start();
        let result = call();
        match &result {
            Ok(call) => started.finish(call.facts())?,
            Err(error) => {
                if let Some(facts) = error.facts() {
                    started.finish(facts)?;
                }
            }
        }
        result
    }

    /// A snapshot of calls recorded so far. Token usage remains absent when
    /// any included call lacked reported usage.
    #[must_use]
    pub fn facts(&self) -> Facts {
        let state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        Facts {
            records: state.records,
            requests_sent: state.requests_sent,
            cache_answers: state.cache_answers,
            input_tokens: (!state.missing_usage)
                .then_some(state.input_tokens)
                .flatten(),
            output_tokens: (!state.missing_usage)
                .then_some(state.output_tokens)
                .flatten(),
            seconds: match (state.first, state.last) {
                (Some(first), Some(last)) => last.duration_since(first).as_secs_f64(),
                _ => 0.0,
            },
            model: (!state.mixed_models).then(|| state.model.clone()).flatten(),
        }
    }
}

impl TallyStart<'_> {
    /// Include one finished call's facts. This consumes the start so a call
    /// cannot be counted twice.
    ///
    /// # Errors
    /// Returns [`Error::Defect`] if the count would overflow.
    pub fn finish(self, facts: &Facts) -> Result<(), Error> {
        let ended = Instant::now();
        let mut state = self.tally.0.lock().unwrap_or_else(PoisonError::into_inner);
        let records = state
            .records
            .checked_add(facts.records)
            .ok_or_else(|| Error::defect("tally record count overflowed"))?;
        let requests = state
            .requests_sent
            .checked_add(facts.requests_sent)
            .ok_or_else(|| Error::defect("tally request count overflowed"))?;
        let cached = state
            .cache_answers
            .checked_add(facts.cache_answers)
            .ok_or_else(|| Error::defect("tally cache count overflowed"))?;
        let input = match (state.input_tokens, facts.input_tokens) {
            (Some(a), Some(b)) => Some(
                a.checked_add(b)
                    .ok_or_else(|| Error::defect("tally token count overflowed"))?,
            ),
            (None, Some(b)) => Some(b),
            _ => None,
        };
        let output = match (state.output_tokens, facts.output_tokens) {
            (Some(a), Some(b)) => Some(
                a.checked_add(b)
                    .ok_or_else(|| Error::defect("tally token count overflowed"))?,
            ),
            (None, Some(b)) => Some(b),
            _ => None,
        };
        state.first = Some(
            state
                .first
                .map_or(self.started, |first| first.min(self.started)),
        );
        state.last = Some(state.last.map_or(ended, |last| last.max(ended)));
        state.records = records;
        state.requests_sent = requests;
        state.cache_answers = cached;
        state.input_tokens = input;
        state.output_tokens = output;
        state.missing_usage |= facts.input_tokens.is_none() || facts.output_tokens.is_none();
        match (&state.model, &facts.model) {
            (None, Some(model)) if !state.mixed_models => state.model = Some(model.clone()),
            (Some(previous), Some(model)) if previous != model => state.mixed_models = true,
            (_, None) => state.mixed_models = true,
            _ => {}
        }
        Ok(())
    }
}
