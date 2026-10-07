//! Explicit facts across completed calls, shared by clones and threads.

use std::fmt;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Instant;

use super::{Call, Counters, Facts};
use crate::public::Error;

#[derive(Default)]
struct State {
    first: Option<Instant>,
    last: Option<Instant>,
    records: u64,
    requests_sent: u64,
    cache_answers: u64,
    // Checked known totals; missing reports affect snapshots, not accumulation.
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    missing_input: bool,
    missing_output: bool,
    missing_priced_facts: bool,
    model: Option<String>,
    mixed_models: bool,
    cost_micro_usd: u64,
    missing_cost: bool,
    held_model_mismatch: bool,
}

/// One caller-owned sum of completed call facts. Clones share its state.
/// Its seconds span the first call start through the last call finish, even
/// when calls overlap; they are not the sum of call durations. Its estimated
/// cost adds each call's rounded six-decimal estimate. That sum can differ
/// from one rounding of the whole work by up to n/2 micro-dollars for n
/// calls. The cost is absent when any call lacked one and when no call was
/// recorded. Its model ignores calls that got no reply, as the command does.
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
            Ok(call) => started
                .finish(call.facts())
                .map_err(|error| error.with_facts(call.facts().clone()))?,
            Err(error) => {
                if let Some(facts) = error.facts() {
                    started
                        .finish(facts)
                        .map_err(|error| error.with_facts(facts.clone()))?;
                }
            }
        }
        result
    }

    /// A snapshot of calls recorded so far. Token usage remains absent when
    /// any included call lacked that reported dimension.
    #[must_use]
    pub fn facts(&self) -> Facts {
        let state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        state.snapshot()
    }
}

impl Tally {
    /// Snapshot checked raw totals, rounding once under this engine's prices.
    /// Live calls need complete priced facts; priced zero-send calls add zero.
    /// Missing reported tokens stay absent even when current cost is known.
    #[must_use]
    pub fn facts_with_engine(&self, engine: &crate::public::Engine) -> Facts {
        let state = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let mut facts = state.snapshot();
        let totals = if state.missing_priced_facts {
            None
        } else {
            Some((
                state.input_tokens.unwrap_or(0),
                state.output_tokens.unwrap_or(0),
            ))
        };
        facts.estimated_cost_usd =
            totals.and_then(|(input, output)| engine.estimate_reported_cost(input, output));
        facts
    }
}
impl State {
    fn snapshot(&self) -> Facts {
        let state = self;
        Facts {
            held_model_mismatch: state.held_model_mismatch,
            attempts: None,
            call_id: None,
            records: state.records,
            requests_sent: state.requests_sent,
            cache_answers: state.cache_answers,
            input_tokens: (!state.missing_input)
                .then_some(state.input_tokens)
                .flatten(),
            output_tokens: (!state.missing_output)
                .then_some(state.output_tokens)
                .flatten(),
            estimated_cost_usd: (state.first.is_some() && !state.missing_cost).then(|| {
                let micro = state.cost_micro_usd;
                format!("{}.{:06}", micro / 1_000_000, micro % 1_000_000)
            }),
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
            (previous, None) => previous,
        };
        let output = match (state.output_tokens, facts.output_tokens) {
            (Some(a), Some(b)) => Some(
                a.checked_add(b)
                    .ok_or_else(|| Error::defect("tally token count overflowed"))?,
            ),
            (None, Some(b)) => Some(b),
            (previous, None) => previous,
        };
        state.first = Some(
            state
                .first
                .map_or(self.started, |first| first.min(self.started)),
        );
        state.last = Some(state.last.map_or(ended, |last| last.max(ended)));
        state.held_model_mismatch |= facts.held_model_mismatch;
        state.records = records;
        state.requests_sent = requests;
        state.cache_answers = cached;
        state.input_tokens = input;
        state.output_tokens = output;
        state.missing_input |= facts.input_tokens.is_none();
        state.missing_output |= facts.output_tokens.is_none();
        state.missing_priced_facts |= facts.estimated_cost_usd.is_none()
            || (facts.requests_sent > 0
                && (facts.input_tokens.is_none() || facts.output_tokens.is_none()));
        match facts
            .estimated_cost_usd
            .as_deref()
            .and_then(micro_usd)
            .and_then(|cost| state.cost_micro_usd.checked_add(cost))
        {
            Some(total) => state.cost_micro_usd = total,
            None => state.missing_cost = true,
        }
        if facts.requests_sent == 0 && facts.cache_answers == 0 {
            return Ok(());
        }
        match (&state.model, &facts.model) {
            (None, Some(model)) if !state.mixed_models => state.model = Some(model.clone()),
            (Some(previous), Some(model)) if previous != model => state.mixed_models = true,
            (_, None) => state.mixed_models = true,
            _ => {}
        }
        Ok(())
    }
}

/// A call's estimate, which the crate writes with six decimal places.
fn micro_usd(cost: &str) -> Option<u64> {
    let (whole, fraction) = cost.split_once('.')?;
    if fraction.len() != 6 {
        return None;
    }
    whole
        .parse::<u64>()
        .ok()?
        .checked_mul(1_000_000)?
        .checked_add(fraction.parse().ok()?)
}

impl std::ops::Add for Counters {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            requests_sent: self.requests_sent.saturating_add(other.requests_sent),
            retries: self.retries.saturating_add(other.retries),
            cache_answers: self.cache_answers.saturating_add(other.cache_answers),
            input_tokens: self.input_tokens.saturating_add(other.input_tokens),
            output_tokens: self.output_tokens.saturating_add(other.output_tokens),
        }
    }
}

impl std::iter::Sum for Counters {
    fn sum<I: Iterator<Item = Self>>(counts: I) -> Self {
        counts.fold(Self::ZERO, std::ops::Add::add)
    }
}

#[cfg(test)]
mod tests {
    use super::{Facts, Tally};

    fn facts(requests_sent: u64, cache_answers: u64, model: Option<&str>) -> Facts {
        Facts {
            held_model_mismatch: false,
            attempts: None,
            call_id: None,
            cache_answers,
            estimated_cost_usd: None,
            input_tokens: None,
            model: model.map(str::to_owned),
            output_tokens: None,
            records: 1,
            requests_sent,
            seconds: 0.0,
        }
    }

    #[test]
    fn a_call_without_a_reply_leaves_the_model_alone() {
        let cases = [
            (vec![facts(1, 0, Some("m")), facts(0, 0, None)], Some("m")),
            (vec![facts(0, 0, None), facts(0, 1, Some("m"))], Some("m")),
            (vec![facts(1, 0, Some("m")), facts(1, 0, None)], None),
            (vec![facts(1, 0, Some("m")), facts(0, 1, Some("n"))], None),
        ];
        for (calls, model) in cases {
            let tally = Tally::new();
            for call in &calls {
                assert!(tally.start().finish(call).is_ok());
            }
            assert_eq!(tally.facts().model(), model);
        }
    }

    #[test]
    fn token_overflow_leaves_the_snapshot_unchanged_and_a_new_scope_clean() {
        let tally = Tally::new();
        let mut first = facts(1, 0, Some("m"));
        first.input_tokens = Some(u64::MAX);
        first.output_tokens = Some(2);
        first.estimated_cost_usd = Some("0.000000".into());
        tally.start().finish(&first).unwrap();
        let mut next = facts(1, 0, Some("m"));
        next.input_tokens = Some(1);
        next.output_tokens = Some(3);
        next.estimated_cost_usd = Some("0.000000".into());
        let before = serde_json::to_value(tally.facts()).unwrap();
        let error = tally.start().finish(&next).unwrap_err();
        assert!(matches!(error, crate::public::Error::Defect { .. }));
        assert_eq!(serde_json::to_value(tally.facts()).unwrap(), before);
        let fresh = Tally::new();
        fresh.start().finish(&next).unwrap();
        assert_eq!(fresh.facts().input_tokens(), Some(1));
        assert_eq!(fresh.facts().output_tokens(), Some(3));
        assert_eq!(fresh.facts().requests_sent(), 1);
    }
}
