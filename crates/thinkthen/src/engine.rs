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
#[cfg(test)]
mod contract_tests;
mod input_report;
mod limits;
mod reuse;
pub(crate) use contract::RecordEvent;
use limits::{REORDER_WINDOW, WORKERS};

enum BatchStage {
    First,
    Later,
}

enum Split {
    Allowed,
    Spent,
}

/// Runs questions against one backend.
#[non_exhaustive]
#[derive(Clone)]
pub struct Engine {
    backend: Backend,
    pub(crate) store: Option<Arc<crate::response_store::Store>>,
    pub(crate) account: crate::usage::Account,
    batch: Batch,
    agent: Arc<OnceLock<transport::Agent>>,
    policy: RetryPolicy,
    pub(crate) cancel: Cancel,
    pub(crate) name: Option<String>,
}

impl Engine {
    /// Copy cumulative usage for this client and its clones, including unresolved attempts.
    #[must_use]
    pub fn usage(&self) -> crate::Usage {
        self.account.snapshot()
    }

    /// Wait for all calls opened so far to settle, then copy usage.
    /// Call only after stopping owned producers and preventing new calls.
    /// This does not cancel calls and is not an ordinary usage getter.
    #[must_use]
    pub fn settled_usage(&self) -> crate::Usage {
        self.account.settle();
        self.usage()
    }
    /// Read the resolved route and policy without retaining or exposing credentials.
    #[must_use]
    pub fn resolved_settings(&self) -> crate::Settings {
        crate::Settings::resolved(
            self.backend.backend_name(),
            self.backend.model(),
            self.backend.base_url(),
            self.batch,
            self.policy.into(),
            self.account.limit(),
            self.store.as_ref().map(|store| store.config().clone()),
        )
    }
    /// An engine that sends to `backend` at the default batch.
    #[must_use]
    pub(crate) fn new(backend: Backend) -> Self {
        let policy = RetryPolicy::default();
        Self {
            policy,
            store: None,
            account: crate::usage::Account::default(),
            cancel: Cancel::default(),
            backend,
            batch: Batch::default(),
            agent: Arc::new(OnceLock::new()),
            name: None,
        }
    }

    /// Read `THINKTHEN_BACKEND`, the selected backend's key variable (such as
    /// `THINKTHEN_API_KEY`) and `THINKTHEN_BATCH` from the environment, as the
    /// command does. With no backend set, the default backend is used.
    ///
    /// ```no_run
    /// let engine = thinkthen::Engine::from_env()?;
    /// # Ok::<(), thinkthen::Error>(())
    /// ```
    /// # Errors
    /// Returns `usage.backend` for an unknown backend name, or the shared URL,
    /// credential or batch error.
    pub fn from_env() -> Result<Self, Error> {
        crate::Settings::default()
            .environment_request_limit(std::env::var("THINKTHEN_MAX_REQUESTS").ok())?
            .engine(|name| std::env::var(name).ok())
    }

    /// Give this engine's typed calls an execution name, which every row's
    /// [`Source::name`](crate::results::Source::name) reports.
    /// # Errors
    /// Returns `usage.name` for a blank name or one containing U+0000.
    pub fn named(self, name: impl Into<String>) -> Result<Self, Error> {
        Ok(Self {
            name: Some(crate::contract_request::admit_name(name.into())?),
            ..self
        })
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
        Self { cancel, ..self }
    }

    /// This engine with a signal of the call's own: cancelling the call leaves the engine's
    /// signal alone, and the engine's signal reaches the call.
    pub(crate) fn for_call(&self) -> Self {
        self.clone().cancel(self.cancel.child())
    }
    pub(crate) fn backend(&self) -> &Backend {
        &self.backend
    }
    pub(crate) fn batch_size(&self) -> usize {
        self.batch.get().get()
    }
    fn answer<F: Function>(
        &self,
        chunk: Chunk<'_, F>,
        url: &str,
        stage: BatchStage,
    ) -> Result<Vec<Row>, Error> {
        let replies = chunk
            .requests()
            .into_iter()
            .map(|request| self.send(request, url, Split::Allowed))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        if matches!(stage, BatchStage::First)
            && replies.iter().all(|reply| reply.failure().is_some())
            && let Some(error) = replies
                .iter()
                .filter_map(RequestAnswers::failure)
                .find(|error| {
                    error
                        .http_status()
                        .is_some_and(|status| (400..500).contains(&status))
                })
        {
            return Err(error.clone());
        }
        chunk
            .advance(replies, url, self.batch.get())
            .into_iter()
            .map(|round| match round {
                NextRound::Ready(rows) => Ok(rows),
                NextRound::Pending(chunk) => self.answer(chunk, url, BatchStage::Later),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|rounds| rounds.into_iter().flatten().collect())
    }

    fn send_live(
        &self,
        request: Request<'_>,
        url: &str,
        split: Split,
    ) -> Result<Vec<RequestAnswers>, Error> {
        if self.store.as_ref().is_some_and(|store| store.replay()) {
            return Err(crate::response_store::error(crate::Code::LocalReplayMiss));
        }
        let Some(remote) = self.backend.remote() else {
            return Ok(vec![if self.cancel.is_cancelled() {
                request.answered(Err(transport::Refused::cancelled().error))
            } else {
                request.stub_answer()
            }]);
        };
        let reply = transport::post(
            self.agent.get_or_init(|| agent(self.policy)),
            remote,
            self.backend.backend_name(),
            url,
            &request.body(),
            self.policy,
            &self.cancel,
            &self.account,
            self.store.as_ref().map(|store| store.body_limit()),
        );
        match reply {
            Err(refused)
                if matches!(
                    refused.error.code(),
                    crate::Code::LocalAccounting
                        | crate::Code::LocalRequestLimit
                        | crate::Code::LocalStoreQuota
                ) =>
            {
                Err(refused.error)
            }
            Err(refused) if matches!(refused.error.http_status(), Some(401 | 403)) => {
                Err(refused.error)
            }
            Err(refused) if refused.too_large && matches!(split, Split::Allowed) => {
                if let Some((left, right)) = request.split() {
                    let mut answers = self.send_live(left, url, Split::Spent)?;
                    answers.extend(self.send_live(right, url, Split::Spent)?);
                    Ok(answers)
                } else {
                    Ok(vec![request.answered(Err(refused.error))])
                }
            }
            Ok(body) if self.store.is_some() => self.retain(request, url, body),
            reply => Ok(vec![
                request.answered(reply.map_err(|refused| refused.error)),
            ]),
        }
    }
}

fn agent(policy: RetryPolicy) -> transport::Agent {
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
    #[expect(
        clippy::expect_used,
        reason = "a request is queued before its answer; a panic here ends the call as a fault"
    )]
    fn fill(&mut self, place: usize, rows: Vec<T>) {
        let slot = self
            .rows
            .get_mut(place - self.first)
            .expect("an answered request is in the window");
        *slot = Some(rows);
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
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}
