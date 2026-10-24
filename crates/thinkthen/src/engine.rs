//! The engine: it packs records into requests, sends them on a pool of
//! workers and hands rows back in input order.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::sync::mpsc::{Receiver, sync_channel};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::thread;

use crate::core::{
    Annotate, Chunk, Decide, Function, NextRound, Packer, Record, Request, RequestAnswers, Row,
};

use crate::backend::Backend;
use crate::batch::Batch;
use crate::transport;
use crate::{Cancel, Error, Outcome, RetryPolicy};

/// How many requests may be open at once.
const WORKERS: usize = 4;

enum Split {
    Allowed,
    Spent,
}

/// Runs questions against one backend.
#[derive(Clone)]
pub struct Engine {
    backend: Backend,
    batch: Batch,
    agent: Arc<OnceLock<ureq::Agent>>,
    policy: RetryPolicy,
    pub(crate) cancel: Cancel,
    operation: Option<&'static str>,
}

impl Engine {
    /// An engine that sends to `backend` at the default batch.
    #[must_use]
    pub fn new(backend: Backend) -> Self {
        let policy = RetryPolicy::default();
        Self {
            policy,
            cancel: Cancel::default(),
            operation: None,
            backend,
            batch: Batch::default(),
            agent: Arc::new(OnceLock::new()),
        }
    }

    /// This engine with `batch` records to a request.
    #[must_use]
    pub fn batch(self, batch: Batch) -> Self {
        Self { batch, ..self }
    }

    /// Set retry limits and the timeout for each attempt.
    #[must_use]
    pub fn retry_policy(self, policy: RetryPolicy) -> Self {
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
            agent: Arc::new(OnceLock::new()),
            ..self
        }
    }

    /// Answer every named question for each record in input order.
    ///
    /// # Errors
    /// Returns the first failed row's error, or `local.cancelled` on cancellation.
    pub fn annotate(
        &self,
        question: &Annotate,
        records: impl IntoIterator<Item = Record>,
        sink: impl FnMut(Row),
    ) -> Result<(), Error> {
        self.call(question, records, sink)
    }

    /// Ask any typed question about each record, returning rows in input order.
    /// All rows reach the sink before the terminal result is returned.
    ///
    /// # Errors
    /// Returns the first failed row's error, or `local.cancelled` on cancellation.
    #[expect(
        clippy::expect_used,
        reason = "the scope owns both ends of each channel until every worker is done"
    )]
    pub fn call(
        &self,
        question: &impl Function,
        records: impl IntoIterator<Item = Record>,
        mut sink: impl FnMut(Row),
    ) -> Result<(), Error> {
        let source = crate::core::Source {
            question_identity: self
                .operation
                .map_or_else(|| question.identity(), |name| question.identity_as(name)),
            backend: self.backend.name().to_owned(),
            model: self.backend.model().to_owned(),
        };
        let identity = source.question_identity.clone();
        let mut result = Ok(());
        let sink = |row: Row| {
            if result.is_ok()
                && let Outcome::Failed { error } = row.outcome()
            {
                result = Err(error.clone());
            }
            sink(row.with_source(source.clone()));
        };
        let url = self.backend.url();
        let mut packer = Packer::new(
            question,
            self.backend.wire_adapter(),
            &url,
            self.backend.model(),
            self.batch.get(),
            self.backend.limits(),
        )
        .identity(identity);
        let (jobs, inbox) = sync_channel(WORKERS);
        let (done, results) = sync_channel::<(usize, Vec<Row>)>(WORKERS);
        let inbox = Mutex::new(inbox);
        thread::scope(|scope| {
            for _ in 0..WORKERS {
                let (done, inbox, url) = (done.clone(), &inbox, &url);
                scope.spawn(move || {
                    while let Some((place, chunk)) = next_job(inbox) {
                        let rows = self.answer(chunk, url);
                        done.send((place, rows))
                            .expect("the scope still reads results");
                    }
                });
            }
            drop(done);
            let mut window = Window {
                rows: VecDeque::new(),
                first: 0,
                sink,
            };
            // The window holds at most one request per worker, so the rows
            // waiting for an earlier request never pass workers times batch.
            // Room comes before the next record is read, so a caller who
            // stops the records stops the sending.
            let make_room = |window: &mut Window<_>| {
                while window.rows.len() == WORKERS {
                    let (place, rows) = results.recv().expect("a worker answers each request");
                    window.fill(place, rows);
                }
            };
            let send = |chunk, window: &mut Window<_>| {
                window.rows.push_back(None);
                jobs.send((window.first + window.rows.len() - 1, chunk))
                    .expect("the workers' inbox lives as long as the scope");
            };
            let mut records = records.into_iter();
            loop {
                make_room(&mut window);
                if self.cancel.is_cancelled() {
                    break;
                }
                let Some(record) = records.next() else {
                    break;
                };
                if let Some(chunk) = packer.push(record) {
                    send(chunk, &mut window);
                }
            }
            if let Some(chunk) = packer.finish() {
                send(chunk, &mut window);
            }
            drop(jobs);
            for (place, rows) in results {
                window.fill(place, rows);
            }
        });
        self.cancel.complete(result)
    }

    /// Keep the original values whose records answer yes, in input order.
    /// Each pair holds the record to judge and the value to return unchanged.
    ///
    /// # Errors
    /// Returns the first failed row's error after emitting all passing records,
    /// or `local.cancelled` on cancellation.
    pub fn filter<T>(
        &self,
        question: &Decide,
        records: impl IntoIterator<Item = (Record, T)>,
        mut sink: impl FnMut(T),
    ) -> Result<(), Error> {
        self.filter_rows(question, records, |original, row| {
            if crate::core::passes(row.outcome()) {
                sink(original);
            }
        })
    }

    /// Judge each record and hand back its original value and row, including
    /// no, unsure and failed outcomes, in input order.
    ///
    /// # Errors
    /// Returns the first failed row's error, or `local.cancelled` on cancellation.
    #[expect(clippy::expect_used, reason = "each row belongs to one queued record")]
    pub fn filter_rows<T>(
        &self,
        question: &Decide,
        records: impl IntoIterator<Item = (Record, T)>,
        mut sink: impl FnMut(T, Row),
    ) -> Result<(), Error> {
        // The decide window bounds how far the iterator runs ahead of its sink.
        let originals = RefCell::new(VecDeque::new());
        let records = records.into_iter().map(|(record, original)| {
            originals.borrow_mut().push_back(original);
            record
        });
        let engine = Self {
            operation: Some("filter"),
            ..self.clone()
        };
        engine.call(question, records, |row| {
            let original = originals
                .borrow_mut()
                .pop_front()
                .expect("one original per row");
            sink(original, row);
        })
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
            self.agent
                .get_or_init(|| agent(self.policy, self.cancel.clone())),
            remote,
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

fn agent(policy: RetryPolicy, cancel: Cancel) -> ureq::Agent {
    let config = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .max_redirects(0)
        .timeout_global(Some(policy.attempt_timeout))
        .build();
    transport::agent(config, cancel)
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
struct Window<F> {
    rows: VecDeque<Option<Vec<Row>>>,
    first: usize,
    sink: F,
}

impl<F: FnMut(Row)> Window<F> {
    /// Hold the rows of request `place`, then hand on every finished request at the front.
    fn fill(&mut self, place: usize, rows: Vec<Row>) {
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
