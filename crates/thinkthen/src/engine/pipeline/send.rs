//! The send stage: one closed request through the key, the pacer, the send
//! budget and the transport, split into one answer per question. A request
//! of two or more questions refused as too large is halved once.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use crate::core::AttemptObservation;
use crate::core::adapters::built_in;
use crate::core::pack::{self, Ask, Split, State};
use crate::core::recording::Exchange as Recorded;
use crate::engine::error::Error;
use crate::engine::facade::{Engine, Key};
use crate::engine::http::Exchange;
use crate::engine::request::Transport;
use crate::engine::store;
use crate::engine::{AttemptSink, Cancel};

/// One closed request: its questions with the label of the input each was
/// packed for, and its body.
pub(super) struct Job {
    pub(super) state: State,
    pub(super) asks: Vec<(Ask, usize)>,
    pub(super) body: Vec<u8>,
}

/// One request's outcome, for a halved request one per half.
pub(super) struct Done {
    pub(super) asks: Vec<(Ask, usize)>,
    pub(super) result: Result<Split, Error>,
    pub(super) requests_sent: u64,
    pub(super) attempts: Arc<[AttemptObservation]>,
    pub(super) taken_at: i64,
}

/// What every worker of one call shares.
pub(super) struct Sender<'a> {
    engine: &'a Engine,
    transport: Transport<'a>,
    model: String,
    packing: super::Packing,
    key: Mutex<Option<Arc<Key>>>,
}

impl<'a> Sender<'a> {
    pub(super) fn new(
        engine: &'a Engine,
        state: &'a crate::engine::facade::State,
        model: String,
        packing: super::Packing,
    ) -> Self {
        Self {
            engine,
            transport: engine.transport(state),
            model,
            packing,
            key: Mutex::new(None),
        }
    }

    /// Send one request, halving it once on a size refusal. The parent
    /// stores nothing, and its attempts count with the first half.
    pub(super) fn send(&self, job: Job, cancel: &Cancel) -> Vec<Done> {
        let whole = self.one(&job.body, job.asks, cancel);
        let halves =
            matches!(&whole.result, Err(error) if error.too_large()) && whole.asks.len() > 1;
        if !halves {
            return vec![whole];
        }
        let Done {
            mut asks,
            requests_sent,
            attempts,
            ..
        } = whole;
        let second = asks.split_off(asks.len().div_ceil(2));
        let mut done = Vec::with_capacity(2);
        for (place, half) in [asks, second].into_iter().enumerate() {
            // A refused first half fails the second too, so nothing is paid
            // for inputs behind a failure the run stops at. A host that takes
            // rows past a failure still sends it, unless the budget refused.
            let failed = done
                .first()
                .and_then(|first: &Done| first.result.as_ref().err().cloned())
                .filter(|error| !self.packing.continues || error.spent());
            let mut answered = match cancel.stop().or(failed) {
                Some(stop) => refused(half, stop),
                None => {
                    let body = built_in::join(
                        job.state.json(),
                        &self.model,
                        half.iter().map(|(ask, _)| &*ask.question),
                    );
                    self.one(&body, half, cancel)
                }
            };
            // Both halves show the refused parent's attempt; the first
            // half's count carries it, by ADR 0111 section 7.
            if place == 0 {
                answered.requests_sent += requests_sent;
            }
            answered.attempts = attempts
                .iter()
                .chain(answered.attempts.iter())
                .cloned()
                .collect();
            done.push(answered);
        }
        done
    }

    /// One request's attempts, from the first live key read to its split.
    fn one(&self, body: &[u8], asks: Vec<(Ask, usize)>, cancel: &Cancel) -> Done {
        let budgeted = cancel.with_process_budget(self.transport.send_budget.clone());
        // A zero limit refuses before the key is read, so a bad key never
        // outranks a spent budget.
        if budgeted.has_zero_send_limit()
            && let Err(denied) = budgeted.reserve_send(None, body.len())
        {
            return refused(asks, denied);
        }
        let key = match self.key(cancel) {
            Ok(key) => key,
            Err(error) => return refused(asks, error),
        };
        let url = self.engine.backend().url();
        let digest = Recorded::new(url, body).digest();
        let events = Arc::new(Mutex::new(Vec::new()));
        let observed = if self.packing.detailed {
            budgeted.with_attempt_sink(collector(&events))
        } else {
            budgeted
        }
        .with_attempt_digest(digest.as_str());
        let sent = AtomicU64::new(0);
        let exchange = Exchange {
            url: url.as_str(),
            body,
            key: &key,
            max_retries: self.transport.max_retries,
            retry_wait: self.transport.retry_wait,
        };
        let answer = self.transport.client.post_marked_with_retry(
            &exchange,
            &observed,
            self.transport.usage,
            |_| (),
            || {
                sent.fetch_add(1, Ordering::Relaxed);
            },
        );
        let taken_at = store::now();
        let result = answer.and_then(|http| {
            let decoders: Vec<_> = asks.iter().map(|(ask, _)| ask.decoder.clone()).collect();
            match pack::split(&decoders, &http.body) {
                Ok(split) => {
                    self.transport.usage.live_reply(split.usage);
                    cancel.live_reply(split.usage);
                    self.transport.usage.answered_by(&split.model);
                    cancel.answered_by(split.model.as_str());
                    Ok(split)
                }
                Err((error, usage)) => {
                    self.transport.usage.live_reply(usage);
                    cancel.live_reply(usage);
                    Err(Error::from(error))
                }
            }
        });
        let mut attempts = events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        attempts.sort_by_key(AttemptObservation::ordinal);
        attempts.dedup_by_key(|event| event.ordinal());
        Done {
            asks,
            result,
            requests_sent: sent.load(Ordering::Relaxed),
            attempts: attempts.into(),
            taken_at,
        }
    }

    /// The key, read once, when the call's first request is about to go.
    fn key(&self, cancel: &Cancel) -> Result<Arc<Key>, Error> {
        let mut held = self.key.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(key) = held.as_ref() {
            return Ok(Arc::clone(key));
        }
        if let Some(stop) = cancel.stop() {
            return Err(stop);
        }
        cancel.key_lookup();
        let key = Arc::new(self.engine.key()?);
        *held = Some(Arc::clone(&key));
        Ok(key)
    }
}

/// A sink that keeps each attempt's observation for the rows' details.
fn collector(events: &Arc<Mutex<Vec<AttemptObservation>>>) -> AttemptSink {
    let events = Arc::clone(events);
    AttemptSink::new(move |event| {
        events
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(event);
    })
}

fn refused(asks: Vec<(Ask, usize)>, error: Error) -> Done {
    Done {
        asks,
        result: Err(error),
        requests_sent: 0,
        attempts: Arc::from([]),
        taken_at: 0,
    }
}
