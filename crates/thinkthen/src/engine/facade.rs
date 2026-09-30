//! The one private completion layer over the production engine.
//!
//! The command calls the engine only here, and ticket 0086 wraps these same
//! calls. Each call hands typed core values in and gets typed values or one
//! structured [`Error`] back. The core parses, plans, and interprets; the
//! lower engine modules prepare, split, schedule, send, record, count, and
//! stop. This module only orders those owners.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::core::{
    Answer, AnswerOutcome, Backend, BackendProfile, Find, FindAnswer, ModelName, Outcome, Plan,
    Value,
};
#[cfg(test)]
use crate::core::{BatchError, Evidence, Question, Threshold, quoted_plan};
use crate::engine::error::Error;
use crate::engine::http::Client;
use crate::engine::prepared_request::PreparedRequests;
use crate::engine::process::Guarded;
use crate::engine::recorder::Recorder;
use crate::engine::request::{self, Transport};
use crate::engine::schedule;
use crate::engine::usage::{Counters, Counts};
use crate::engine::{Cancel, Width};

pub(crate) use crate::engine::http::{Key, Roots};
pub(crate) use crate::engine::prepared_request::{Answered, PreparedChunk as Chunk};
pub(crate) use crate::engine::roots::Error as RootsError;
pub(crate) use crate::engine::schedule::{
    Completed, Input, InputPort, Outcome as RunOutcome, RecordFlow,
};
pub(crate) use annotate::{Annotation, GroupAnswer, QuestionAnswer, assemble};
pub(crate) use recognize::{MAX_TEXT_BYTES, Probabilities, Recognized, step_one};
pub(crate) use relate::{Execution, Logical, PreparedRelations, relations};

mod annotate;
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
    /// `THINKTHEN_REQUESTS_PER_MINUTE`, or `None` for the address's default.
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
    recording: bool,
    send_budget: Option<crate::engine::send_budget::ProcessBudget>,
    state: Arc<Guarded<State>>,
}

/// The retained pool and its width gate, the recorder and cache coordinator,
/// the process counters, and the width this process's calls follow.
#[derive(Debug)]
pub(super) struct State {
    pub(super) client: Client,
    recorder: Recorder,
    pub(super) usage: Arc<Counters>,
    pub(super) width: usize,
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
        self.send_budget = Some(crate::engine::send_budget::ProcessBudget {
            budget: crate::engine::budget::process_budget(),
            requests: limit,
            estimated: estimated_limit,
        });
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
        let mut engine = Self {
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
            recording: false,
            send_budget: None,
            state: Arc::new(Guarded::empty()),
        };
        let (usage, cancel) = (settings.usage, Cancel::default());
        let state = engine
            .state
            .current(pid, crate::engine::rebuild_wait(&cancel), || {
                engine.fresh(pid, usage, &cancel)
            })?;
        engine.recording = state.recorder.reported();
        Ok(engine)
    }

    /// State built from the immutable settings alone, as process `pid`.
    fn fresh(&self, pid: u32, usage: Arc<Counters>, cancel: &Cancel) -> Result<State, Error> {
        let storage = &self.storage;
        let recorder = Recorder::of_private(
            storage.record.as_deref(),
            storage.replay.as_deref(),
            storage.private_default,
            storage.cache_answers,
        )?
        .with_refresh(storage.refresh_cache);
        let widths = crate::engine::process_width_of(pid, cancel)?;
        let width = widths.select(self.width).map_err(Error::WidthActive)?.get();
        let secure = self.backend.is_secure();
        let client = match self.roots.as_ref() {
            Some(roots) => Client::with_roots(self.timeout, secure, widths, Some(roots)),
            None => Client::new(self.timeout, secure, widths),
        };
        Ok(State {
            client: client.paced(crate::engine::backoff::interval(self.per_minute, secure)),
            recorder,
            usage,
            width,
        })
    }

    /// The one door to the retained state. It compares this process with the
    /// state's owner first, and a forked child gets fresh state and fresh
    /// counters before anything inherited is touched.
    pub(super) fn state(&self, cancel: &Cancel) -> Result<Arc<State>, Error> {
        let pid = std::process::id();
        self.state
            .current(pid, crate::engine::rebuild_wait(cancel), || {
                let usage = Arc::new(Counters::new(self.usage_path.clone()));
                self.fresh(pid, usage, cancel)
            })
    }

    /// The same engine asking another model. It shares this engine's state:
    /// the pool, the recorder, the counters, and the width.
    pub(crate) fn with_model(&self, model: ModelName) -> Result<Self, Error> {
        let backend = Backend::resolve(Some(self.backend.url().as_str()), None, model.as_str())
            .map_err(|_| Error::Defect("a resolved address was refused again"))?
            .with_request_size(self.backend.ceiling());
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

    /// Whether a folder the caller named, rather than the private default, is in use.
    pub(crate) const fn recording(&self) -> bool {
        self.recording
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

    /// Prepare every request one plan needs, split under the backend limits.
    pub(crate) fn split(&self, plan: &Plan) -> Result<Vec<Chunk>, Error> {
        split(&self.backend, self.profile.as_ref(), plan)
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
            self.backend.model().clone(),
            evidence,
            None,
            vec![question.clone()],
            self.profile.as_ref(),
        )
        .map_err(|error| match error {
            BatchError::Profile(limit) => Error::ProfileLimit(limit),
            _ => Error::Defect("a plan of one question could not be quoted"),
        })?;
        let answered = self.ask(&plan, cancel)?;
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
        let answered = self.ask(find.plan(), cancel)?;
        let selection = find
            .select(&only_answer(&answered)?)
            .map_err(|_| Error::Defect("a find choice could not be mapped"))?;
        Ok(Found {
            selection,
            answered,
        })
    }

    /// Send chunks prepared earlier and hand each reply on in chunk order.
    ///
    /// Up to the engine's width go out at once. The first failure in chunk
    /// order returns, as a send one at a time would return it.
    pub(crate) fn ask_chunks<E: From<Error>>(
        &self,
        chunks: Vec<Chunk>,
        cancel: &Cancel,
        mut each: impl FnMut(Answered) -> Result<(), E>,
    ) -> Result<(), E> {
        self.ask_chunks_with_plan(chunks, cancel, |_, answered| each(answered))
    }

    /// Retain each already prepared plan beside its ordered reply for a
    /// caller that must name the actual logical questions it answered.
    pub(crate) fn ask_chunks_with_plan<E: From<Error>>(
        &self,
        chunks: Vec<Chunk>,
        cancel: &Cancel,
        mut each: impl FnMut(&Plan, Answered) -> Result<(), E>,
    ) -> Result<(), E> {
        let state = self.state(cancel)?;
        let send = |chunk: Chunk| {
            let answered = request::ask_sent(
                &self.backend,
                &chunk.plan,
                chunk.request,
                &state.recorder,
                cancel,
                self.transport(&state),
                || self.key(),
            )?;
            Ok::<_, Error>((chunk.plan, answered))
        };
        let jobs = state.width.min(chunks.len());
        if jobs < 2 {
            for chunk in chunks {
                let (plan, answered) = send(chunk)?;
                each(&plan, answered)?;
            }
            return Ok(());
        }
        crate::engine::workers::ordered(jobs, chunks, cancel, &send, |(plan, answered)| {
            each(&plan, answered)
        })
    }

    /// Answer framed inputs over this engine's width and emit them in input order.
    ///
    /// The host starts the reader, so a reader blocked on its own input
    /// never holds the call open. Every engine worker has joined on return.
    pub(crate) fn records<T, R, E>(
        &self,
        flow: RecordFlow,
        cancel: &Cancel,
        start_reader: impl FnOnce(Receiver<()>, InputPort<T, R, E>),
        answer: &(impl Fn(&T) -> Result<Completed<R, E>, E> + Sync),
        emit: impl FnMut(R) -> Result<bool, E>,
    ) -> Result<RunOutcome<E>, E>
    where
        T: Send + 'static,
        R: Send,
        E: From<Error> + Send,
    {
        let width = self.state(cancel)?.width;
        schedule::run_cancelled(
            width,
            flow,
            cancel,
            start_reader,
            answer,
            emit,
            |message| E::from(Error::Defect(message)),
            E::from,
        )
    }

    fn ask(&self, plan: &Plan, cancel: &Cancel) -> Result<Answered, Error> {
        let state = self.state(cancel)?;
        request::ask_profile(
            &self.backend,
            plan,
            self.profile.as_ref(),
            &state.recorder,
            cancel,
            self.transport(&state),
            || self.key(),
        )
    }

    pub(super) fn transport<'a>(&self, state: &'a State) -> Transport<'a> {
        Transport {
            client: &state.client,
            max_retries: self.max_retries,
            retry_wait: self.retry_wait,
            usage: &state.usage,
            send_budget: self.send_budget.clone(),
        }
    }
}

/// Prepare every request one plan needs, split under the backend limits.
///
/// A plan shows its requests before any engine exists, so this needs none.
pub(crate) fn split(
    backend: &Backend,
    profile: Option<&BackendProfile>,
    plan: &Plan,
) -> Result<Vec<Chunk>, Error> {
    Ok(PreparedRequests::with_profile(backend, plan, profile, None)?.into_chunks())
}

/// The one answer a one-question reply carries.
fn only_answer(answered: &Answered) -> Result<Answer, Error> {
    match answered.reply.outcomes() {
        [AnswerOutcome::Answered(answer)] => Ok(answer.clone()),
        _ => Err(Error::Defect("the adapter answered no question")),
    }
}
