//! The one private completion layer over the production engine.
//!
//! The command calls the engine only here, and ticket 0086 wraps these same
//! calls. Each call hands typed core values in and gets typed values or one
//! structured [`Error`] back. The core parses, plans, and interprets; the
//! lower engine modules prepare, split, schedule, send, record, count, and
//! stop. This module only orders those owners.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::core::adapters::built_in;
use crate::core::recording::{Digest, Exchange as Recorded};
use crate::core::{
    Answer, AnswerOutcome, Backend, BackendProfile, Find, FindAnswer, ModelName, Outcome, Plan,
    Reply, Value,
};
#[cfg(test)]
use crate::core::{BatchError, Evidence, Question, Threshold, quoted_plan};
use crate::engine::error::Error;
use crate::engine::http::{Client, Exchange};
use crate::engine::process::Guarded;
use crate::engine::usage::{Counters, Counts};
use crate::engine::{Cancel, Width};

pub(crate) use crate::engine::http::{Key, Roots};
pub(crate) use crate::engine::roots::Error as RootsError;
pub(crate) use annotate::{Annotation, GroupAnswer, QuestionAnswer, assemble};
pub(crate) use each::{Asks, Bound, Request};
pub(crate) use recognize::{MAX_TEXT_BYTES, Probabilities, Recognized, step_one};
pub(crate) use relate::{Execution, Logical, PreparedRelations, relations};

mod annotate;
mod each;
mod finish;
#[cfg(test)]
#[cfg(feature = "cli")]
mod fork_tests;
mod recognize;
mod relate;

/// The folders replies are replayed from and recorded to.
#[derive(Clone, Debug, Default)]
pub(crate) struct Storage {
    pub(crate) record: Option<PathBuf>,
    pub(crate) replay: Option<PathBuf>,
    pub(crate) private_default: bool,
    pub(crate) cache_answers: bool,
    pub(crate) refresh_cache: bool,
}

/// Where a live attempt's key comes from: the command's variable, read at
/// send time, or the value a library caller captured.
pub(crate) type KeyReader = Arc<dyn Fn() -> Result<Key, Error> + Send + Sync>;

/// Every engine setting, resolved at the host edge before the engine exists.
pub(crate) struct Settings {
    pub(crate) backend: Backend,
    pub(crate) profile: Option<BackendProfile>,
    pub(crate) timeout: Duration,
    pub(crate) max_retries: u32,
    pub(crate) retry_wait: Duration,
    /// `None` follows the process width and never selects one.
    pub(crate) width: Option<Width>,
    /// `THINKTHEN_REQUESTS_PER_MINUTE`, which outranks the backend's own rate.
    pub(crate) per_minute: Option<std::num::NonZeroU32>,
    pub(crate) storage: Storage,
    /// Read only when a live attempt is about to go out.
    pub(crate) key: KeyReader,
    pub(crate) usage: Arc<Counters>,
}

/// One immutable engine: its settings and the retained state behind [`Engine::state`].
///
/// It holds no resident thread. Every setting here is plain data a forked
/// child may read. Everything that can hold a lock lives in `state`. A clone
/// shares that state.
#[derive(Clone)]
pub(crate) struct Engine {
    backend: Backend,
    profile: Option<BackendProfile>,
    timeout: Duration,
    max_retries: u32,
    retry_wait: Duration,
    key: KeyReader,
    /// The explicit width or `None`, applied again in each process.
    width: Option<Width>,
    per_minute: Option<std::num::NonZeroU32>,
    storage: Storage,
    roots: Option<Roots>,
    usage_path: Option<PathBuf>,
    /// The request and estimated input limits this engine selects against
    /// the process totals, which its state holds.
    send_budget: Option<(Option<u64>, Option<u64>)>,
    state: Arc<Guarded<State>>,
}

/// The retained pool and its width gate, the process counters, and the
/// width this process's calls follow.
#[derive(Debug)]
pub(super) struct State {
    pub(super) client: Client,
    pub(super) usage: Arc<Counters>,
    pub(super) width: usize,
    /// The process request and estimated input totals.
    total: crate::engine::budget::SendBudget,
    /// The replay folder's fixture, read once for this process.
    pub(super) replayed: Option<Arc<crate::engine::store::Replayed>>,
}

/// The transport settings one call's sends share.
pub(crate) struct Transport<'a> {
    pub(crate) client: &'a Client,
    pub(crate) max_retries: u32,
    pub(crate) retry_wait: Duration,
    pub(crate) usage: &'a Counters,
    pub(crate) send_budget: Option<crate::engine::send_budget::ProcessBudget>,
}

/// One answered request: its decoded reply, whether the store answered it,
/// its identity, and its HTTP attempts.
#[derive(Clone)]
pub(crate) struct Answered {
    pub(crate) reply: Reply,
    pub(crate) replayed: bool,
    pub(crate) request: Digest,
    pub(crate) requests_sent: u64,
}

/// One typed judgment and the metadata its result carries.
pub(crate) struct Judgment {
    pub(crate) answer: Answer,
    pub(crate) value: Value,
    pub(crate) outcome: Outcome,
    pub(crate) answered: Answered,
}

/// The selected unit, every candidate's probability, and the request metadata.
pub(crate) struct Found {
    pub(crate) selection: FindAnswer,
    pub(crate) answered: Answered,
}

impl Engine {
    /// Select this engine's limit at each live transport reservation.
    pub(crate) fn with_process_budget(
        mut self,
        limit: Option<u64>,
        estimated_limit: Option<u64>,
    ) -> Self {
        self.send_budget = Some((limit, estimated_limit));
        self
    }

    /// Check the local settings, register an explicit width, and build the pool.
    ///
    /// Nothing here reads the key, opens a folder for writing, counts, or connects.
    pub(crate) fn new(settings: Settings) -> Result<Self, Error> {
        Self::built_by(settings, std::process::id())
    }

    /// The same engine with parsed replacement trust roots.
    pub(crate) fn with_roots(settings: Settings, roots: Option<Roots>) -> Result<Self, Error> {
        match roots {
            Some(roots) => Self::built_with_roots(settings, std::process::id(), Some(roots)),
            None => Self::new(settings),
        }
    }

    /// Build this engine's state as process `pid`, which then owns it.
    fn built_by(settings: Settings, pid: u32) -> Result<Self, Error> {
        Self::built_with_roots(settings, pid, None)
    }

    fn built_with_roots(settings: Settings, pid: u32, roots: Option<Roots>) -> Result<Self, Error> {
        let engine = Self {
            usage_path: settings.usage.path().map(PathBuf::from),
            backend: settings.backend,
            profile: settings.profile,
            timeout: settings.timeout,
            max_retries: settings.max_retries,
            retry_wait: settings.retry_wait,
            key: settings.key,
            width: settings.width,
            per_minute: settings.per_minute,
            storage: settings.storage,
            roots,
            send_budget: None,
            state: Arc::new(Guarded::empty()),
        };
        let folders = [&engine.storage.record, &engine.storage.replay];
        if let [Some(recorded), Some(replayed)] = folders
            && recorded != replayed
        {
            return Err(Error::Defect(
                "record and replay folders differ below the command edge",
            ));
        }
        if folders
            .into_iter()
            .flatten()
            .any(|folder| std::fs::metadata(folder).is_ok_and(|metadata| metadata.is_file()))
        {
            return Err(Error::RecordingPathIsFile);
        }
        let (usage, cancel) = (settings.usage, Cancel::default());
        engine
            .state
            .current(pid, crate::engine::limits::rebuild_wait(&cancel), || {
                engine.fresh(pid, usage, &cancel)
            })?;
        Ok(engine)
    }

    /// State built from the immutable settings alone, as process `pid`.
    fn fresh(&self, pid: u32, usage: Arc<Counters>, cancel: &Cancel) -> Result<State, Error> {
        let storage = &self.storage;
        let limits = crate::engine::limits::of(pid, cancel)?;
        let widths = &limits.widths;
        let width = widths.select(self.width).map_err(Error::WidthActive)?.get();
        let replayed = match (&storage.record, &storage.replay) {
            (None, Some(folder)) => crate::engine::store::Replayed::of(folder)?,
            _ => None,
        };
        let secure = self.backend.is_secure();
        let client = match self.roots.as_ref() {
            Some(roots) => Client::with_roots(self.timeout, secure, widths, Some(roots)),
            None => Client::new(self.timeout, secure, widths),
        };
        Ok(State {
            client: client.paced(crate::engine::backoff::interval(
                self.per_minute.or(self.backend.per_minute()),
            )),
            usage,
            width,
            total: limits.total.clone(),
            replayed,
        })
    }

    /// The one door to the retained state. It compares this process with the
    /// state's owner first, and a forked child gets fresh state and fresh
    /// counters before anything inherited is touched.
    pub(super) fn state(&self, cancel: &Cancel) -> Result<Arc<State>, Error> {
        let pid = std::process::id();
        self.state
            .current(pid, crate::engine::limits::rebuild_wait(cancel), || {
                let usage = Arc::new(Counters::new(self.usage_path.clone()));
                self.fresh(pid, usage, cancel)
            })
    }

    /// The same engine asking another model. It shares this engine's state:
    /// the pool, the counters, and the width.
    pub(crate) fn with_model(&self, model: ModelName) -> Result<Self, Error> {
        let backend = Backend::resolve(Some(self.backend.url().as_str()), None, model.as_str())
            .map_err(|_| Error::Defect("a resolved address was refused again"))?
            .with_request_size(self.backend.ceiling())
            .with_descriptions(self.backend.descriptions())
            .with_per_minute(self.backend.per_minute());
        Ok(Self {
            backend,
            ..self.clone()
        })
    }

    /// The address and model every request of this engine names.
    pub(crate) const fn backend(&self) -> &Backend {
        &self.backend
    }

    /// The enforceable profile the batch planner must retain on a split.
    pub(crate) fn profile(&self) -> Option<&BackendProfile> {
        self.profile.as_ref()
    }

    /// The key a live request carries. With the variable unset, a backend
    /// proven to be this machine takes an empty key, which sends no
    /// authorization header, so a local server that checks none needs no
    /// pretend secret. Every other address still refuses.
    pub(super) fn key(&self) -> Result<Key, Error> {
        match (self.key)() {
            Err(Error::NoKey(_)) if self.backend.is_loopback() => Ok(Key::new(String::new())),
            read => read,
        }
    }

    /// The folders this engine's calls replay from and record to.
    pub(super) const fn storage(&self) -> &Storage {
        &self.storage
    }

    /// The width this process's calls follow.
    pub(crate) fn width(&self, cancel: &Cancel) -> Result<usize, Error> {
        Ok(self.state(cancel)?.width)
    }

    /// Whether a folder the caller named, rather than the private default, is in use.
    pub(crate) const fn recording(&self) -> bool {
        (self.storage.record.is_some() || self.storage.replay.is_some())
            && !self.storage.private_default
    }

    /// The process counters, cumulative since the process began, or since
    /// this process was forked.
    #[allow(
        dead_code,
        reason = "the command reads durable totals; ticket 0086 exposes this snapshot"
    )]
    pub(crate) fn usage(&self) -> Result<Counts, Error> {
        Ok(self.state(&Cancel::default())?.usage.snapshot())
    }

    /// Ask one question of one evidence and read the answer under the rule.
    /// Only tests call it; the public calls go through `ask_all`.
    #[cfg(test)]
    pub(crate) fn judge(
        &self,
        question: &Question,
        threshold: Option<Threshold>,
        evidence: Evidence,
        cancel: &Cancel,
    ) -> Result<Judgment, Error> {
        let plan = quoted_plan(
            self.backend.asked(),
            evidence,
            None,
            vec![question.clone()],
            self.profile.as_ref(),
        )
        .map_err(|error| match error {
            BatchError::Profile(limit) => Error::ProfileLimit(limit),
            _ => Error::Defect("a plan of one question could not be quoted"),
        })?;
        let mut asks = Asks::default();
        asks.add(&self.backend, &plan)?;
        let mut answered = None;
        self.ask_each(&asks, Bound::WHOLE, cancel, |_, one| {
            answered = Some(one);
            Ok(())
        })?;
        let answered = answered.ok_or(Error::Defect("a question had no answer"))?;
        let answer = only_answer(&answered)?;
        let (value, outcome) = answer.read(threshold);
        Ok(Judgment {
            answer,
            value,
            outcome,
            answered,
        })
    }

    /// Ask one aggregate question over a bounded set and select one unit.
    pub(crate) fn find(&self, find: &Find, cancel: &Cancel) -> Result<Found, Error> {
        let mut asks = Asks::default();
        asks.add(&self.backend, find.plan())?;
        let mut answered = None;
        self.ask_each(&asks, Bound::WHOLE, cancel, |_, one| {
            answered = Some(one);
            Ok(())
        })?;
        let answered = answered.ok_or(Error::Defect("a find question had no answer"))?;
        let selection = find
            .select(&only_answer(&answered)?)
            .map_err(|_| Error::Defect("a find choice could not be mapped"))?;
        Ok(Found {
            selection,
            answered,
        })
    }

    /// Send one plan as one request with no store, as `check` does, which
    /// reads and writes no cache (`specification/check.md`). The send runs
    /// on an engine worker, so a host signal never lands in its socket read.
    pub(crate) fn send_plan(&self, plan: &Plan, cancel: &Cancel) -> Result<Reply, Error> {
        let body = built_in::encode(plan)
            .map_err(|_| Error::Defect("a request could not be written as JSON"))?;
        let state = self.state(cancel)?;
        let transport = self.transport(&state);
        let digest = Recorded::new(self.backend.url(), &body).digest();
        let cancel = cancel
            .with_process_budget(transport.send_budget.clone())
            .with_attempt_digest(digest.as_str());
        if let Some(stop) = cancel.stop() {
            return Err(stop);
        }
        cancel.key_lookup();
        let key = self.key()?;
        let exchange = Exchange {
            url: self.backend.url().as_str(),
            body: &body,
            key: &key,
            max_retries: transport.max_retries,
            retry_wait: transport.retry_wait,
        };
        let answered = crate::engine::workers::on_worker(&cancel, || {
            transport
                .client
                .post_marked_with_retry(&exchange, &cancel, transport.usage, |_| (), || ())
        })?;
        let decoded = built_in::decode_observed(plan, &answered.body);
        transport.usage.live_reply(decoded.usage);
        cancel.live_reply(decoded.usage);
        let reply = decoded.reply.map_err(Error::from)?;
        transport.usage.answered_by(reply.model());
        cancel.answered_by(reply.model().as_str());
        Ok(reply)
    }

    pub(super) fn transport<'a>(&self, state: &'a State) -> Transport<'a> {
        Transport {
            client: &state.client,
            max_retries: self.max_retries,
            retry_wait: self.retry_wait,
            usage: &state.usage,
            send_budget: self.send_budget.map(|(requests, estimated)| {
                crate::engine::send_budget::ProcessBudget {
                    budget: state.total.clone(),
                    requests,
                    estimated,
                }
            }),
        }
    }
}

/// The one answer a one-question reply carries.
fn only_answer(answered: &Answered) -> Result<Answer, Error> {
    match answered.reply.outcomes() {
        [AnswerOutcome::Answered(answer)] => Ok(answer.clone()),
        _ => Err(Error::Defect("the adapter answered no question")),
    }
}
