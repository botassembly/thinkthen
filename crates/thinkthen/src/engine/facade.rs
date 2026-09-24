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
    Answer, AnswerOutcome, Backend, BackendProfile, Evidence, Find, FindAnswer, Outcome, Plan,
    Question, Threshold, Value,
};
use crate::engine::annotate_schedule;
use crate::engine::error::Error;
use crate::engine::http::Client;
use crate::engine::prepared_request::PreparedRequests;
use crate::engine::recorder::Recorder;
use crate::engine::request::{self, Transport};
use crate::engine::schedule;
use crate::engine::usage::{Counters, Counts};
use crate::engine::{Cancel, Width};

pub(crate) use crate::engine::annotate_schedule::{
    InputPort as GroupPort, Outcome as GroupOutcome, Prepared,
};
pub(crate) use crate::engine::http::Key;
pub(crate) use crate::engine::prepared_request::{Answered, PreparedChunk as Chunk};
pub(crate) use crate::engine::schedule::{Completed, Input, InputPort, Outcome as RunOutcome};
pub(crate) use recognize::{Recognized, TokenInput};
pub(crate) use relate::{Execution, Logical, Method, PreparedRelation, relations};

mod recognize;
mod relate;

/// The folders replies are replayed from and recorded to.
#[derive(Debug, Default)]
pub(crate) struct Storage {
    pub(crate) record: Option<PathBuf>,
    pub(crate) replay: Option<PathBuf>,
    pub(crate) private_default: bool,
    pub(crate) cache_answers: bool,
}

/// Every engine setting, resolved at the host edge before the engine exists.
pub(crate) struct Settings {
    pub(crate) backend: Backend,
    pub(crate) profile: Option<BackendProfile>,
    pub(crate) timeout: Duration,
    pub(crate) max_retries: u32,
    pub(crate) retry_wait: Duration,
    /// `None` follows the process width and never selects one.
    pub(crate) width: Option<Width>,
    pub(crate) storage: Storage,
    /// Read only when a live attempt is about to go out.
    pub(crate) key: fn() -> Result<Key, Error>,
    pub(crate) usage: Arc<Counters>,
}

/// One immutable engine: its settings and the retained state behind [`Engine::state`].
///
/// It holds no resident thread. Ticket 0096 guards `state`.
pub(crate) struct Engine {
    backend: Backend,
    profile: Option<BackendProfile>,
    max_retries: u32,
    retry_wait: Duration,
    key: fn() -> Result<Key, Error>,
    width: usize,
    state: State,
}

/// The retained pool and its width gate, the recorder and cache coordinator,
/// and the process counters.
struct State {
    client: Client,
    recorder: Recorder,
    usage: Arc<Counters>,
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
    /// Check the local settings, register an explicit width, and build the pool.
    ///
    /// Nothing here reads the key, opens a folder for writing, counts, or connects.
    pub(crate) fn new(settings: Settings) -> Result<Self, Error> {
        let storage = settings.storage;
        let recorder = Recorder::of_private(
            storage.record.as_deref(),
            storage.replay.as_deref(),
            storage.private_default,
            storage.cache_answers,
        )?;
        let width = crate::engine::process_width()
            .select(settings.width)
            .map_err(Error::WidthActive)?
            .get();
        Ok(Self {
            state: State {
                client: Client::new(settings.timeout, settings.backend.is_secure()),
                recorder,
                usage: settings.usage,
            },
            backend: settings.backend,
            profile: settings.profile,
            max_retries: settings.max_retries,
            retry_wait: settings.retry_wait,
            key: settings.key,
            width,
        })
    }

    /// The one door to the retained state.
    const fn state(&self) -> &State {
        &self.state
    }

    /// The address and model every request of this engine names.
    pub(crate) const fn backend(&self) -> &Backend {
        &self.backend
    }

    /// Whether a folder the caller named, rather than the private default, is in use.
    pub(crate) const fn recording(&self) -> bool {
        self.state().recorder.reported()
    }

    /// The process counters, cumulative since the process began.
    #[allow(
        dead_code,
        reason = "the command reads durable totals; ticket 0086 exposes this snapshot"
    )]
    pub(crate) fn usage(&self) -> Counts {
        self.state().usage.snapshot()
    }

    /// Prepare every request one plan needs, split under the backend limits.
    pub(crate) fn split(&self, plan: &Plan) -> Result<Vec<Chunk>, Error> {
        split(&self.backend, self.profile.as_ref(), plan)
    }

    /// Ask one question of one evidence and read the answer under the rule.
    pub(crate) fn judge(
        &self,
        question: &Question,
        threshold: Option<Threshold>,
        evidence: Evidence,
        cancel: &Cancel,
    ) -> Result<Judgment, Error> {
        let plan = Plan::new(
            evidence,
            self.backend.model().clone(),
            vec![question.clone()],
        )
        .map_err(|_| Error::Defect("a plan of one question asks nothing"))?;
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

    /// Send chunks prepared earlier, in order, and hand each reply on.
    pub(crate) fn ask_chunks<E: From<Error>>(
        &self,
        chunks: Vec<Chunk>,
        cancel: &Cancel,
        mut each: impl FnMut(Answered) -> Result<(), E>,
    ) -> Result<(), E> {
        for chunk in chunks {
            each(request::ask_sent(
                &self.backend,
                &chunk.plan,
                chunk.request,
                &self.state().recorder,
                cancel,
                self.transport(),
                || (self.key)().map_err(E::from),
            )?)?;
        }
        Ok(())
    }

    /// Answer framed inputs over this engine's width and emit them in input order.
    ///
    /// The host starts the reader, so a reader blocked on its own input
    /// never holds the call open. Every engine worker has joined on return.
    pub(crate) fn records<T, R, E>(
        &self,
        held: bool,
        cancel: &Cancel,
        start_reader: impl FnOnce(Receiver<()>, InputPort<T, R, E>),
        answer: &(impl Fn(&T) -> Result<Completed<R>, E> + Sync),
        emit: impl FnMut(R) -> Result<bool, E>,
    ) -> Result<RunOutcome<E>, E>
    where
        T: Send + 'static,
        R: Send,
        E: From<Error> + Send,
    {
        schedule::run_cancelled(
            self.width,
            held,
            cancel,
            start_reader,
            answer,
            emit,
            |message| E::from(Error::Defect(message)),
            E::from,
        )
    }

    /// Answer each input's question groups over this engine's width, in input order.
    #[expect(
        clippy::too_many_arguments,
        reason = "the grouped scheduler keeps each typed host callback explicit"
    )]
    pub(crate) fn groups<T, S, A, W, G, R, E>(
        &self,
        streams: bool,
        cancel: &Cancel,
        start_reader: impl FnOnce(Receiver<()>, GroupPort<T, G, E>),
        prepare: impl Fn(T) -> Result<Prepared<S, A, W>, E>,
        answer: &(impl Fn(W) -> Result<G, E> + Sync),
        accept: impl Fn(&mut A, G) -> Result<(), E>,
        finish: impl Fn(S, A) -> Result<Completed<R>, E>,
        emit: impl FnMut(R) -> Result<bool, E>,
    ) -> Result<GroupOutcome<E>, E>
    where
        T: Send + 'static,
        W: Send,
        G: Send,
        R: Send,
        E: From<Error> + Send,
    {
        annotate_schedule::run(
            self.width,
            streams,
            cancel,
            start_reader,
            prepare,
            answer,
            accept,
            finish,
            emit,
            |message| E::from(Error::Defect(message)),
            E::from,
        )
    }

    fn ask(&self, plan: &Plan, cancel: &Cancel) -> Result<Answered, Error> {
        request::ask_profile(
            &self.backend,
            plan,
            self.profile.as_ref(),
            &self.state().recorder,
            cancel,
            self.transport(),
            self.key,
        )
    }

    fn transport(&self) -> Transport<'_> {
        let state = self.state();
        Transport {
            client: &state.client,
            max_retries: self.max_retries,
            retry_wait: self.retry_wait,
            usage: &state.usage,
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
    Ok(PreparedRequests::with_profile(backend, plan, profile)?.into_chunks())
}

/// The one answer a one-question reply carries.
fn only_answer(answered: &Answered) -> Result<Answer, Error> {
    match answered.reply.outcomes() {
        [AnswerOutcome::Answered(answer)] => Ok(answer.clone()),
        _ => Err(Error::Defect("the adapter answered no question")),
    }
}
