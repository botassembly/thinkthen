//! The engine: it packs records into requests, sends them on a pool of
//! workers and hands rows back in input order.

use std::collections::VecDeque;
use std::fmt;
use std::sync::mpsc::{Receiver, sync_channel};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::thread;

use crate::core::{
    Chunk, Function, NextRound, Outcome, Packer, Record, Request, RequestAnswers, Row,
};

use crate::backend::Backend;
use crate::batch::Batch;
use crate::transport;
use crate::{Cancel, Error, RetryPolicy};
mod contract;
mod input_report;
mod limits;
pub(crate) use contract::RecordEvent;
use limits::{REORDER_WINDOW, WORKERS};

enum Split {
    Allowed,
    Spent,
}

/// Runs questions against one backend.
#[non_exhaustive]
#[derive(Clone)]
pub struct Engine {
    backend: Backend,
    batch: Batch,
    agent: Arc<OnceLock<ureq::Agent>>,
    policy: RetryPolicy,
    pub(crate) cancel: Cancel,
    cancel_attached: bool,
}

impl Engine {
    /// Read the resolved route and policy without retaining or exposing credentials.
    #[must_use]
    pub fn resolved_settings(&self) -> crate::Settings {
        crate::Settings::resolved(
            self.backend.backend_name(),
            self.backend.model(),
            self.backend.base_url(),
            self.batch,
            self.policy.into(),
        )
    }
    /// An engine that sends to `backend` at the default batch.
    #[must_use]
    pub(crate) fn new(backend: Backend) -> Self {
        let policy = RetryPolicy::default();
        Self {
            policy,
            cancel: Cancel::default(),
            cancel_attached: false,
            backend,
            batch: Batch::default(),
            agent: Arc::new(OnceLock::new()),
        }
    }

    /// This engine with `batch` records to a request.
    #[must_use]
    pub(crate) fn batch(self, batch: Batch) -> Self {
        Self { batch, ..self }
    }

    /// Set retry limits and the timeout for each attempt.
    #[must_use]
    pub(crate) fn retry_policy(self, policy: RetryPolicy) -> Self {
        Self {
            policy,
            agent: Arc::new(OnceLock::new()),
            ..self
        }
    }

    /// Attach a shared cancellation signal to this engine's calls.
    #[must_use]
    pub fn cancel(self, cancel: Cancel) -> Self {
        Self {
            cancel,
            cancel_attached: true,
            ..self
        }
    }

    pub(crate) fn for_call(&self) -> Self {
        let cancel = if self.cancel_attached {
            self.cancel.child()
        } else {
            Cancel::default()
        };
        self.clone().cancel(cancel)
    }
    pub(crate) fn backend(&self) -> &Backend {
        &self.backend
    }
    pub(crate) fn call_contract(
        &self,
        question: &impl Function,
        records: impl Iterator<Item = RecordEvent<Record>> + Send,
        sink: impl FnMut(Result<Row, Error>),
        identity: String,
    ) -> Result<(), Error> {
        self.call_parallel(question, records, sink, identity)
    }
    fn answer<F: Function>(&self, chunk: Chunk<'_, F>, url: &str) -> Vec<Row> {
        let replies = chunk
            .requests()
            .into_iter()
            .flat_map(|request| self.send(request, url, Split::Allowed))
            .collect();
        chunk
            .advance(replies, url, self.batch.get())
            .into_iter()
            .flat_map(|round| match round {
                NextRound::Ready(rows) => rows,
                NextRound::Pending(chunk) => self.answer(chunk, url),
            })
            .collect()
    }

    fn send(&self, request: Request<'_>, url: &str, split: Split) -> Vec<RequestAnswers> {
        let Some(remote) = self.backend.remote() else {
            return vec![if self.cancel.is_cancelled() {
                request.answered(Err(transport::Refused::cancelled().error))
            } else {
                request.stub_answer()
            }];
        };
        let reply = transport::post(
            self.agent.get_or_init(|| agent(self.policy)),
            remote,
            self.backend.backend_name(),
            url,
            &request.body(),
            self.policy,
            &self.cancel,
        );
        match reply {
            Err(refused) if refused.too_large && matches!(split, Split::Allowed) => {
                if let Some((left, right)) = request.split() {
                    let mut answers = self.send(left, url, Split::Spent);
                    answers.extend(self.send(right, url, Split::Spent));
                    answers
                } else {
                    vec![request.answered(Err(refused.error))]
                }
            }
            reply => vec![request.answered(reply.map_err(|refused| refused.error))],
        }
    }
}

fn agent(policy: RetryPolicy) -> ureq::Agent {
    let config = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .max_redirects(0)
        .max_idle_connections(WORKERS)
        .max_idle_connections_per_host(WORKERS)
        .timeout_global(Some(policy.attempt_timeout))
        .build();
    transport::agent(config)
}

/// The next request to send, or `None` once no more will come.
fn next_job<'a, F: Function>(
    inbox: &Mutex<Receiver<(usize, Chunk<'a, F>)>>,
) -> Option<(usize, Chunk<'a, F>)> {
    inbox
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .recv()
        .ok()
}

/// Requests sent and not yet handed on, in input order, and the sink they go to.
struct Window<F, T = Row> {
    rows: VecDeque<Option<Vec<T>>>,
    first: usize,
    sink: F,
}

impl<T, F: FnMut(T)> Window<F, T> {
    /// Hold the rows of request `place`, then hand on every finished request at the front.
    fn fill(&mut self, place: usize, rows: Vec<T>) {
        if let Some(slot) = self.rows.get_mut(place - self.first) {
            *slot = Some(rows);
        }
        while let Some(rows) = self.rows.front_mut().and_then(Option::take) {
            self.rows.pop_front();
            self.first += 1;
            rows.into_iter().for_each(&mut self.sink);
        }
    }
}

impl fmt::Debug for Engine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Engine")
            .field("backend", &self.backend)
            .field("batch", &self.batch)
            .finish_non_exhaustive()
    }
}
