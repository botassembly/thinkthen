//! Bounded cumulative accounting for one explicit client and its clones.
use crate::{Code, Error};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Condvar, Mutex, PoisonError};

/// Provider-reported tokens, with unavailable finished attempts kept explicit.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Sum of known token counts; absent after arithmetic overflow.
    pub reported: Option<u64>,
    /// Finished attempts without an admitted count for this dimension.
    pub unknown_attempts: u64,
}
impl Default for TokenUsage {
    fn default() -> Self {
        Self::empty()
    }
}
impl TokenUsage {
    fn empty() -> Self {
        Self {
            reported: Some(0),
            unknown_attempts: 0,
        }
    }
    fn add(&mut self, value: Option<u64>) {
        match value {
            Some(value) => self.reported = self.reported.and_then(|sum| sum.checked_add(value)),
            None => self.unknown_attempts += 1,
        }
    }
}
/// An independent copy of cumulative client accounting, never a complete invoice.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Usage {
    /// Actual transport send attempts, including retries.
    pub attempts: u64,
    /// Fully received response bodies within the transport's bounds.
    pub responses: u64,
    /// Attempts made because a prior HTTP status was retriable.
    pub status_retries: u64,
    /// Attempts made because a connection was refused or stale.
    pub connection_retries: u64,
    /// Encoded body bytes passed to send, including retries.
    pub request_body_bytes: u64,
    /// Calls whose engine workers have not finished, including paused input.
    pub active_calls: u64,
    /// Attempts not yet finished; their tokens are still unresolved.
    pub active_attempts: u64,
    /// Distinct materialized asks served from the local TTL cache.
    #[serde(default)]
    pub cache_hits: u64,
    /// Distinct materialized asks served from an immutable recording snapshot.
    #[serde(default)]
    pub replay_hits: u64,
    /// Observed input tokens.
    pub input_tokens: TokenUsage,
    /// Observed output tokens.
    pub output_tokens: TokenUsage,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Account {
    usage: Arc<Mutex<Usage>>,
    lifecycle: Arc<Lifecycle>,
    limit: Option<u64>,
}
#[derive(Debug, Default)]
struct Lifecycle {
    calls: Mutex<u64>,
    settled: Condvar,
}
#[derive(Debug)]
pub(crate) struct ActiveCall(Arc<Lifecycle>);
impl Drop for ActiveCall {
    fn drop(&mut self) {
        let mut calls = self.0.calls.lock().unwrap_or_else(PoisonError::into_inner);
        *calls -= 1;
        self.0.settled.notify_all();
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum Reason {
    Initial,
    Status,
    Connection,
}
impl Account {
    pub(crate) fn new(limit: Option<u64>) -> Self {
        Self {
            limit,
            ..Self::default()
        }
    }
    pub(crate) const fn limit(&self) -> Option<u64> {
        self.limit
    }

    pub(crate) fn open_call(&self) -> Result<ActiveCall, Error> {
        let mut calls = self
            .lifecycle
            .calls
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        *calls = calls.checked_add(1).ok_or_else(|| {
            Error::from_contract(
                Code::LocalAccounting,
                "client active-call accounting exceeded its supported range",
            )
        })?;
        Ok(ActiveCall(self.lifecycle.clone()))
    }
    pub(crate) fn settle(&self) {
        let calls = self
            .lifecycle
            .calls
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        drop(
            self.lifecycle
                .settled
                .wait_while(calls, |calls| *calls != 0)
                .unwrap_or_else(PoisonError::into_inner),
        );
    }

    pub(crate) fn snapshot(&self) -> Usage {
        let calls = self
            .lifecycle
            .calls
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let mut usage = *self.usage.lock().unwrap_or_else(PoisonError::into_inner);
        usage.active_calls = *calls;
        usage
    }
    pub(crate) fn reused(&self, count: usize, replay: bool) -> Result<(), Error> {
        let mut usage = self.usage.lock().unwrap_or_else(PoisonError::into_inner);
        let counter = if replay {
            &mut usage.replay_hits
        } else {
            &mut usage.cache_hits
        };
        *counter = counter.checked_add(count as u64).ok_or_else(|| {
            Error::from_contract(
                Code::LocalAccounting,
                "client reuse accounting exceeded its supported range",
            )
        })?;
        Ok(())
    }
    pub(crate) fn begin(&self, bytes: usize, reason: Reason) -> Result<Attempt, Error> {
        let mut usage = self.usage.lock().unwrap_or_else(PoisonError::into_inner);
        if self.limit.is_some_and(|limit| usage.attempts >= limit) {
            return Err(Error::from_contract(
                Code::LocalRequestLimit,
                "the client request allowance is exhausted; work is incomplete and no further request was sent",
            ));
        }
        let mut next = *usage;
        let overflow = || {
            Error::from_contract(
                Code::LocalAccounting,
                "client usage accounting exceeded its supported range; no further request was sent",
            )
        };
        next.attempts = next.attempts.checked_add(1).ok_or_else(overflow)?;
        next.request_body_bytes = next
            .request_body_bytes
            .checked_add(u64::try_from(bytes).map_err(|_| overflow())?)
            .ok_or_else(overflow)?;
        next.active_attempts = next.active_attempts.checked_add(1).ok_or_else(overflow)?;
        match reason {
            Reason::Initial => {}
            Reason::Status => {
                next.status_retries = next.status_retries.checked_add(1).ok_or_else(overflow)?
            }
            Reason::Connection => {
                next.connection_retries = next
                    .connection_retries
                    .checked_add(1)
                    .ok_or_else(overflow)?
            }
        }
        *usage = next;
        Ok(Attempt {
            account: self.clone(),
            tokens: (None, None),
            response: false,
        })
    }
}
#[derive(Debug)]
pub(crate) struct Attempt {
    account: Account,
    tokens: (Option<u64>, Option<u64>),
    response: bool,
}
impl Attempt {
    pub(crate) fn response(&mut self, body: &[u8], complete: bool) {
        self.response = complete;
        // Both supported decisions protocols report these fields. Parse optional
        // metadata independently of answers and independently per dimension.
        #[derive(Deserialize)]
        struct Envelope {
            usage: Option<Box<serde_json::value::RawValue>>,
        }
        #[derive(Deserialize)]
        struct Tokens {
            input_tokens: Option<Box<serde_json::value::RawValue>>,
            output_tokens: Option<Box<serde_json::value::RawValue>>,
        }
        if let Some(tokens) = serde_json::from_slice::<Envelope>(body)
            .ok()
            .and_then(|envelope| envelope.usage)
            .and_then(|raw| serde_json::from_str::<Tokens>(raw.get()).ok())
        {
            let count = |raw: Option<Box<serde_json::value::RawValue>>| {
                raw.and_then(|raw| serde_json::from_str::<u64>(raw.get()).ok())
            };
            self.tokens = (count(tokens.input_tokens), count(tokens.output_tokens));
        }
    }
}
impl Drop for Attempt {
    fn drop(&mut self) {
        let mut usage = self
            .account
            .usage
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        usage.active_attempts -= 1;
        usage.responses += u64::from(self.response);
        usage.input_tokens.add(self.tokens.0);
        usage.output_tokens.add(self.tokens.1);
    }
}
