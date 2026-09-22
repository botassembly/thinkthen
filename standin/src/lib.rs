//! The stand-in engine: the contract, implemented today.
//!
//! Experiment 211 proved the shape this crate carries — a blocking `ureq`
//! client, scoped threads, no async runtime anywhere, one process-wide
//! width gate, a process-ID check behind no lock, a wait the calling
//! thread can leave, and an error that says whether a second try could
//! help. This crate is that engine moved into the repository and grown to
//! the whole contract: the eight verbs, `details`, and the counters, over
//! the real `thinkthen-core` wire shapes.
//!
//! Two fork preconditions, stated because "by construction" alone cannot
//! defend them: the settings are read when an engine value is built
//! (`ResolvedConfig::resolve`), so build the engine before any fork; and a
//! forked child keeps the inherited pool's file descriptors for its
//! lifetime, because the pid-check rebuild leaks the retired pool rather
//! than tearing it down — the descriptors close when the child exits, and
//! nothing else closes them.
//!
//! What is stand-in here, named so nothing mistakes it for product: the
//! session has no disk cache (`cache_answers` stays zero and the cache
//! settings are carried but unspent), `tokens` counts only what replies
//! report, and `find` judges each unit alone and takes the best, because
//! the relative one-request form belongs to the real engine. The null
//! backend answers every verb in-process with the conformance file's own
//! numbers, so one set of cases runs against it and against recordings.
//!
//! Environment, read when an engine value is built and used only for what
//! a [`EngineConfig`] leaves unset: `ENGINE_NULL` for the in-process
//! backend, `ENGINE_BASE_URL` for the stub on the wire (the contract's
//! `THINKTHEN_BASE_URL` wins over it), `ENGINE_TIMEOUT_SECS` (30),
//! `ENGINE_MAX_RETRIES` (2), `ENGINE_WIDTH` (4), and
//! `ENGINE_SYNTHETIC_PARTIAL` (unset) which arms the annotate partial-
//! failure fixture for tests alone. No key is read and none is sent.

use std::ptr;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use thinkthen_contract::{
    Annotated, AnnotatedRecord, Answer, Cancel, Cause, Details, Edge, EngineConfig, Error, Failed,
    FailureKind, Found, Judgment, Options, Question, QuestionKind, QuestionSet, Ranked, Recognize,
    Recognized, Relate, Scored, Settings, Usage,
};
use thinkthen_core::adapters::built_in;
use thinkthen_core::{Backend, Evidence, ModelName, Plan, Reply, Value};

/// The recognize and relate replay over the recorded cases.
mod replay;

/// Re-exported so a surface names one crate and reaches the whole shape.
pub use thinkthen_contract::Engine;

/// How long one wait in the tick loop holds before it comes up for air.
///
/// The bulk wait checks the cancel token and runs the poll callback on this
/// tick. Fifty milliseconds keeps a stop gesture answered inside a tenth of
/// a second while costing an idle batch two wakeups a second.
const TICK: Duration = Duration::from_millis(50);

/// The statuses a backend is asked again after, as `backends.md` lists.
const RETRIED: [u16; 6] = [429, 500, 502, 503, 504, 529];

/// The longest a `Retry-After` header may move the wait, as `backends.md`
/// caps it.
const MAX_RETRY_WAIT: Duration = Duration::from_secs(60);

/// The most of one response body an attempt reads before it gives up.
const MAX_RESPONSE_BYTES: u64 = 1024 * 1024;

/// The settled knobs one engine value runs with, resolved from its
/// [`EngineConfig`] first and the environment second.
#[derive(Debug)]
struct ResolvedConfig {
    null: bool,
    base: String,
    timeout: Duration,
    max_retries: u32,
    width: usize,
    synthetic_partial: bool,
}

impl ResolvedConfig {
    /// Resolve one engine value's knobs: every config field set wins, the
    /// environment answers for what it leaves unset, and the built-in
    /// defaults answer last.
    fn resolve(config: &EngineConfig) -> Self {
        let env = |name: &str| std::env::var(name).ok().filter(|value| !value.trim().is_empty());
        let env_number = |name: &str| env(name).and_then(|value| value.parse::<u64>().ok());
        Self {
            null: env("ENGINE_NULL").is_some(),
            base: config
                .address
                .clone()
                .or_else(|| env("THINKTHEN_BASE_URL"))
                .or_else(|| env("ENGINE_BASE_URL"))
                .unwrap_or_else(|| "http://127.0.0.1:8091/v1".into()),
            timeout: config
                .timeout
                .unwrap_or_else(|| Duration::from_secs(env_number("ENGINE_TIMEOUT_SECS").unwrap_or(30))),
            max_retries: config
                .max_retries
                .unwrap_or_else(|| env_number("ENGINE_MAX_RETRIES").unwrap_or(2) as u32),
            width: config
                .width
                .or_else(|| env_number("ENGINE_WIDTH").map(|value| value as usize))
                .unwrap_or(4),
            synthetic_partial: env("ENGINE_SYNTHETIC_PARTIAL").is_some(),
        }
    }

    /// The knobs as the environment alone reads them, for the helpers that
    /// name a request without an engine value.
    fn from_env() -> Self {
        Self::resolve(&EngineConfig::from_env())
    }
}

/// The engine every surface binds until the real one lands.
///
/// Built from the environment or from an [`EngineConfig`], carrying the
/// resolved knobs. Holds no thread between calls.
#[derive(Clone, Debug)]
pub struct BlockingEngine {
    model: Option<String>,
    max_requests: Option<u64>,
    config: Arc<ResolvedConfig>,
}

impl BlockingEngine {
    /// Build an engine from the environment, as `Engine::from_env` does on
    /// the Rust surface.
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_settings(Settings::from_env())
    }

    /// Build an engine from settled settings.
    #[must_use]
    pub fn from_settings(settings: Settings) -> Self {
        Self {
            model: settings.model.clone(),
            max_requests: settings.max_requests,
            config: Arc::new(ResolvedConfig::resolve(&settings)),
        }
    }

    /// The core's threshold for a contract question, when it holds one.
    /// Refuse a bulk call that outnumbers the request limit, before any
    /// request leaves.
    fn limit(&self, records: usize) -> Result<(), Error> {
        match self.max_requests {
            Some(limit) if records as u64 > limit => Err(Error::usage(format!(
                "the request limit is {limit} and {records} records came"
            ))),
            _ => Ok(()),
        }
    }

    /// One judgment of one question of one evidence, any verb, with the
    /// sends that produced it.
    fn ask(
        &self,
        question: &Question,
        evidence: &str,
        options: &Options<'_>,
    ) -> Result<(thinkthen_core::Answer, u32, String), Error> {
        Error::guard(options)?;
        let text = Evidence::new(evidence).map_err(|error| Error::usage(error.to_string()))?;
        let model_name = match &self.model {
            Some(named) => named.as_str(),
            None => question.model(),
        };
        let model = ModelName::new(model_name).map_err(|error| Error::usage(error.to_string()))?;
        let plan = Plan::new(text, model, vec![question.core().clone()])
            .map_err(|error| Error::usage(error.to_string()))?;
        let settings = &self.config;
        let (body, sends) = if settings.null {
            // The null backend sends nothing, so it counts one a call, the
            // way a send would.
            REQUESTS.fetch_add(1, Ordering::Relaxed);
            (null_reply(question, evidence)?, 1)
        } else {
            let inner = state(options, &self.config)?;
            let _ticket = inner.gate.enter(options)?;
            let url = endpoint_of(settings, model_name)?;
            let wire =
                built_in::encode(&plan).map_err(|error| Error::backend_not_retryable(error.to_string()))?;
            post(&inner.agent, &url, wire, options, &self.config)?
        };
        let reply: Reply = built_in::decode(&plan, &body)
            .map_err(|error| Error::backend_not_retryable(error.to_string()))?;
        let answer = reply
            .answers()
            .first()
            .ok_or_else(|| Error::backend_not_retryable("the reply carries no answer"))?;
        let _ = &question.threshold();
        Ok((answer.clone(), sends, model_name.to_owned()))
    }

    /// The bulk spine: ask one question of every record at the engine's
    /// width, keeping every judgment in input order, with the tick wait.
    fn batch(
        &self,
        question: &Question,
        records: &[&str],
        options: Options<'_>,
        mut poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Judgment>, Error> {
        Error::guard(&options)?;
        self.limit(records.len())?;
        if records.is_empty() {
            return Ok(Vec::new());
        }
        let width = self.config.width.max(1);
        let threads = width.min(records.len());
        let (task_tx, task_rx) = mpsc::sync_channel::<(usize, &str)>(threads);
        let task_rx = Arc::new(Mutex::new(task_rx));
        let (done_tx, done_rx) = mpsc::channel::<(usize, Result<Judgment, Error>)>();
        let stop = Arc::new(AtomicBool::new(false));
        let mut answers: Vec<Option<Judgment>> = Vec::with_capacity(records.len());
        answers.resize_with(records.len(), || None);
        let mut first_failure: Option<Error> = None;

        thread::scope(|scope| {
            for worker in 0..threads {
                let task_rx = Arc::clone(&task_rx);
                let done_tx = done_tx.clone();
                let stop = Arc::clone(&stop);
                let question = question.clone();
                let options = options;
                let born = thread::Builder::new()
                    .name(format!("ttb-worker-{worker}"))
                    .spawn_scoped(scope, move || loop {
                        let taken = lock(&task_rx).recv();
                        match taken {
                            Ok((place, record)) => {
                                if stop.load(Ordering::Relaxed) {
                                    continue;
                                }
                                let answer = self
                                    .ask(&question, record, &options)
                                    .and_then(|(answer, _, _)| {
                                        let probability = answer.yes().ok_or_else(|| {
                                            Error::backend_not_retryable(
                                                "the answer carries no probability",
                                            )
                                        })?;
                                        Ok(Judgment {
                                            probability,
                                            answer: outcome_of(answer.read(question.threshold())),
                                        })
                                    })
                                    .map_err(|error| {
                                        stop.store(true, Ordering::Relaxed);
                                        error
                                    });
                                if done_tx.send((place, answer)).is_err() {
                                    break;
                                }
                            }
                            Err(_) => break,
                        }
                    })
                    .expect("a worker thread starts");
                drop(born);
            }

            // The original sender dies here, so the wait below can see the
            // far end go quiet once every worker has left: after a cancel
            // the workers discard what is left, and the only end of the
            // wait is the disconnect, not a count.
            drop(done_tx);

            let feed_stop = Arc::clone(&stop);
            let feed = thread::Builder::new()
                .name("ttb-feed".to_owned())
                .spawn_scoped(scope, move || {
                    for (place, record) in records.iter().enumerate() {
                        if feed_stop.load(Ordering::Relaxed) {
                            break;
                        }
                        if task_tx.send((place, record)).is_err() {
                            break;
                        }
                    }
                })
                .expect("the feed thread starts");
            drop(feed);

            let mut received = 0_usize;
            // The poll must run on a busy channel too, or a fast backend
            // starves it and a Ctrl-C waits for the whole batch (reproduced
            // at 8.48 s on a three-million-record null batch). Gate the tick
            // by elapsed time, not by the wait having idled.
            let mut last_poll = Instant::now();
            let mut maybe_tick = |poll: &mut Option<&mut dyn FnMut()>| {
                if last_poll.elapsed() >= TICK {
                    if let Some(poll) = poll.as_mut() {
                        poll();
                    }
                    if Error::guard(&options).is_err() {
                        stop.store(true, Ordering::Relaxed);
                    }
                    last_poll = Instant::now();
                }
            };
            while received < records.len() {
                match done_rx.recv_timeout(TICK) {
                    Ok((place, Ok(judgment))) => {
                        answers[place] = Some(judgment);
                        received += 1;
                        maybe_tick(&mut poll);
                    }
                    Ok((_, Err(error))) => {
                        if first_failure.is_none() {
                            first_failure = Some(error);
                        }
                        received += 1;
                        maybe_tick(&mut poll);
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        maybe_tick(&mut poll);
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });

        if options.cancel_token().is_some_and(Cancel::is_cancelled) {
            return Err(Error::cancelled());
        }
        if options.passed() {
            return Err(Error::deadline(options.seconds()));
        }
        if let Some(error) = first_failure {
            return Err(error);
        }
        answers
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| Error::defect("a worker stopped before its record was answered"))
    }
}

/// The connector a surface names today: it builds stand-in engines, and
/// pointing a surface at the real engine's connector is the one line the
/// merge changes.
///
/// ```
/// let connector = thinkthen_standin::StandinConnector;
/// let engine = thinkthen_contract::Connector::connect(
///     &connector,
///     &thinkthen_contract::EngineConfig::from_env(),
/// )?;
/// # Ok::<(), thinkthen_contract::Error>(())
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct StandinConnector;

impl thinkthen_contract::Connector for StandinConnector {
    /// Build an engine value: every config field set wins over the
    /// environment, and the environment answers for what it leaves unset.
    ///
    /// # Errors
    ///
    /// Never, on the stand-in: resolution has no failure path. The real
    /// engine's connector returns the usage or local kind for a config it
    /// cannot honor.
    fn connect(&self, config: &EngineConfig) -> Result<Arc<dyn Engine>, Error> {
        Ok(Arc::new(BlockingEngine::from_settings(config.clone())))
    }
}

impl Engine for BlockingEngine {
    fn decide_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Answer, Error> {
        let (answer, _, _) = self.ask(question, evidence, &options)?;
        answer
            .yes()
            .ok_or_else(|| Error::backend_not_retryable("the answer carries no probability"))?;
        Ok(outcome_of(answer.read(question.threshold())))
    }

    fn decide_many_opts(
        &self,
        question: &Question,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Judgment>, Error> {
        self.batch(question, records, options, poll)
    }

    fn choose_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Option<String>, Error> {
        let (answer, _, _) = self.ask(question, evidence, &options)?;
        match answer.read(question.threshold()).0 {
            Value::Choice(pick) => Ok(pick),
            _ => Err(Error::defect("a choose reply carried no choice")),
        }
    }

    fn score_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Scored, Error> {
        let (answer, _, _) = self.ask(question, evidence, &options)?;
        let value = answer.read(question.threshold()).0;
        let number = match value {
            Value::Score(number) => number,
            _ => return Err(Error::defect("a score reply carried no position")),
        };
        let (nearest, _) = score_level_and_probability(&answer)?;
        Ok(Scored { value: number, nearest })
    }

    fn tag_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Vec<String>, Error> {
        let (answer, _, _) = self.ask(question, evidence, &options)?;
        match answer.read(question.threshold()).0 {
            Value::Tag(held) => Ok(held),
            _ => Err(Error::defect("a tag reply carried no labels")),
        }
    }

    fn filter_opts(
        &self,
        question: &Question,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<usize>, Error> {
        if matches!(question.threshold(), Some(held) if !held.is_cut()) {
            return Err(Error::usage("filter takes a cut, and a band came"));
        }
        let judgments = self.batch(question, records, options, poll)?;
        Ok(judgments
            .iter()
            .enumerate()
            .filter(|(_, judgment)| judgment.answer == Answer::Yes)
            .map(|(place, _)| place)
            .collect())
    }

    fn rank_opts(
        &self,
        question: &Question,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Ranked>, Error> {
        if question.threshold_named() {
            return Err(Error::usage("rank takes no threshold"));
        }
        self.limit(records.len())?;
        let judgments = self.batch(question, records, options, poll)?;
        let mut ranked: Vec<Ranked> = judgments
            .iter()
            .enumerate()
            .map(|(index, judgment)| Ranked { index, probability: judgment.probability })
            .collect();
        ranked.sort_by(|one, two| {
            two.probability
                .partial_cmp(&one.probability)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(ranked)
    }

    fn find_opts(
        &self,
        question: &Question,
        units: &[&str],
        options: Options<'_>,
    ) -> Result<Found, Error> {
        if units.len() < 2 || units.len() > 255 {
            return Err(Error::usage(format!(
                "find takes 2 to 255 units, and {} came",
                units.len()
            )));
        }
        // Stand-in behavior, named in the crate docs: each unit is judged
        // alone and the best wins, because the relative one-request form
        // belongs to the real engine.
        let judgments = self.batch(question, units, options, None)?;
        let mut best = 0_usize;
        for (place, judgment) in judgments.iter().enumerate() {
            if judgment.probability > judgments[best].probability {
                best = place;
            }
        }
        Ok(Found { index: Some(best), probability: judgments[best].probability })
    }

    fn annotate_opts(
        &self,
        set: &QuestionSet,
        records: &[&str],
        options: Options<'_>,
        mut poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<AnnotatedRecord>, Error> {
        self.limit(records.len().max(set.questions().len()))?;
        let mut rows = Vec::with_capacity(records.len());
        for record in records {
            Error::guard(&options)?;
            let mut fields = Vec::with_capacity(set.questions().len());
            for (name, question) in set.names().iter().zip(set.questions()) {
                if let Some(poll) = poll.as_mut() {
                    poll();
                }
                // The one synthesized partial failure (no recording carries
                // a failed logical question): when the test-only
                // `ENGINE_SYNTHETIC_PARTIAL` opt-in is set, exactly this
                // record's last name-order question returns the ruled marker
                // and its neighbours answer normally. The conformance case
                // `74-annotate-preserves-good-answers` pins the shape when
                // the opt-in is set, and DIVERGENCES.md marks it synthesized.
                // Unset, which is every production process, this record
                // answers like any other.
                if self.config.synthetic_partial
                    && record == &SYNTHETIC_PARTIAL_RECORD
                    && Some(name) == set.names().last()
                {
                    fields.push((
                        name.clone(),
                        Annotated::Failed(Failed {
                            kind: FailureKind::Backend,
                            cause: Cause::MissingAnswer,
                        }),
                    ));
                    continue;
                }
                let field = match question.kind() {
                    QuestionKind::Decide => {
                        let (answer, _, _) = self.ask(question, record, &options)?;
                        answer.yes().ok_or_else(|| {
                            Error::backend_not_retryable("the answer carries no probability")
                        })?;
                        Annotated::Decision(outcome_of(answer.read(question.threshold())))
                    }
                    QuestionKind::Choose => {
                        Annotated::Choice(self.choose_opts(question, record, options)?)
                    }
                    QuestionKind::Score => {
                        Annotated::Score(self.score_opts(question, record, options)?)
                    }
                    QuestionKind::Tag => Annotated::Tags(self.tag_opts(question, record, options)?),
                };
                fields.push((name.clone(), field));
            }
            rows.push(fields);
        }
        Ok(rows)
    }

    fn details_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Details, Error> {
        let (answer, sends, model) = self.ask(question, evidence, &options)?;
        let (probability, nearest) = if question.kind() == QuestionKind::Score {
            // A score carries no yes side: the trail's number is the
            // nearest level's own probability, and the level's name rides
            // in `nearest` per ADR 0017 pick 6.
            let (nearest, leader) = score_level_and_probability(&answer)?;
            (leader, Some(nearest))
        } else {
            let probability = answer
                .yes()
                .ok_or_else(|| Error::backend_not_retryable("the answer carries no probability"))?;
            (probability, None)
        };
        Ok(Details {
            probability,
            answer: outcome_of(answer.read(question.threshold())),
            nearest,
            model,
            digest: question.digest(),
            sends,
            requests: vec![request_digest(question, self.model.as_deref(), evidence)?],
            failed_questions: 0,
        })
    }

    fn recognize_opts(
        &self,
        ask: &Recognize,
        text: &str,
        options: Options<'_>,
    ) -> Result<Recognized, Error> {
        Error::guard(&options)?;
        replay::recognize(ask, text)
    }

    fn relate_opts(
        &self,
        ask: &Relate,
        records: &[&str],
        options: Options<'_>,
    ) -> Result<Vec<Edge>, Error> {
        Error::guard(&options)?;
        thinkthen_contract::guard_relate_records(records.len())?;
        replay::relate(ask, records)
    }

    fn usage(&self) -> Usage {
        Usage {
            requests: REQUESTS.load(Ordering::Relaxed),
            cache_answers: 0,
            tokens: TOKENS.load(Ordering::Relaxed),
        }
    }
}

/// Carry the core's read outcome into the contract's public answer.
fn outcome_of((_, outcome): (Value, thinkthen_core::Outcome)) -> Answer {
    match outcome {
        thinkthen_core::Outcome::Yes => Answer::Yes,
        thinkthen_core::Outcome::No => Answer::No,
        thinkthen_core::Outcome::Unresolved => Answer::Unsure,
    }
}

/// A score answer's nearest level and that level's own probability, read
/// from the answer's serialized form. The core keeps no public accessor
/// for the level yet, so both the `score` and `details` paths read it
/// here, once.
fn score_level_and_probability(
    answer: &thinkthen_core::Answer,
) -> Result<(String, f64), Error> {
    let held = serde_json::to_value(answer).map_err(|error| Error::defect(error.to_string()))?;
    let level = held
        .get("level")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    let probability = held
        .get("probabilities")
        .and_then(serde_json::Value::as_object)
        .and_then(|map| {
            map.values().filter_map(serde_json::Value::as_f64).reduce(f64::max)
        });
    match (level, probability) {
        (Some(level), Some(probability)) => Ok((level, probability)),
        _ => Err(Error::defect("a score reply named no level")),
    }
}

/// What left the machine this session: the counted sends and the vendor's
/// reported tokens.
static REQUESTS: AtomicU64 = AtomicU64::new(0);
static TOKENS: AtomicU64 = AtomicU64::new(0);

/// Zero the counters.
pub fn reset_usage() {
    REQUESTS.store(0, Ordering::Relaxed);
    TOKENS.store(0, Ordering::Relaxed);
}

/// The one synthesized partial-failure record, armed only by tests.
///
/// No recording carries a failed logical question, so the stand-in answers
/// the ruled marker for exactly this record (its last name-order question)
/// and nothing else — but only when the test-only `ENGINE_SYNTHETIC_PARTIAL`
/// opt-in is set. Unset, the default in every production process, the
/// record answers like any other input, so no caller ever meets a fake
/// failure. `74-annotate-preserves-good-answers` pins the marker's shape
/// and `conformance/DIVERGENCES.md` marks it synthesized.
pub const SYNTHETIC_PARTIAL_RECORD: &str = "order 4471: charged twice, please refund";

/// The recording digest the request this call would make is filed under.
///
/// The production rule (0053): the adapter name, the resolved URL, and the
/// exact request bytes through `recording::Exchange::digest`. The encoder
/// is deterministic, so the digest is computed on the null backend too; it
/// names the request the plan would make, and nothing is sent.
///
/// `model` is the engine's own model setting, exactly as [`BlockingEngine::ask`]
/// resolves it, so the digest names the request the engine would send.
///
/// # Errors
///
/// The usage kind for a blank evidence text, an unresolvable model, or an
/// unbuildable plan; the backend kind when the plan does not encode.
pub fn request_digest(
    question: &Question,
    model: Option<&str>,
    evidence: &str,
) -> Result<String, Error> {
    let text = Evidence::new(evidence).map_err(|error| Error::usage(error.to_string()))?;
    let settings = ResolvedConfig::from_env();
    let model_name = model.unwrap_or(question.model());
    let model = ModelName::new(model_name).map_err(|error| Error::usage(error.to_string()))?;
    let plan = Plan::new(text, model, vec![question.core().clone()])
        .map_err(|error| Error::usage(error.to_string()))?;
    let wire = built_in::encode(&plan)
        .map_err(|error| Error::backend_not_retryable(error.to_string()))?;
    let backend = Backend::resolve(None, Some(&settings.base), model_name)
        .map_err(|error| Error::usage(error.to_string()))?;
    Ok(thinkthen_core::recording::Exchange::new(backend.url(), &wire)
        .digest()
        .as_str()
        .to_owned())
}

/// The null backend's reply for any verb, in the adapter's own shapes.
///
/// The rules are the conformance file's numbers, so one set of cases runs
/// against the null backend and against recordings: `refund` in the judged
/// member or evidence lifts the probability, `maybe` holds the middle, and
/// anything else falls. A `score` question answers with the fixed
/// distributions its evidence class picks.
fn null_reply(question: &Question, evidence: &str) -> Result<Vec<u8>, Error> {
    if evidence.contains("malformed") {
        return Err(Error::backend_not_retryable(
            "HTTP 422: the backend refused the request as malformed or too large",
        ));
    }
    let single = |judged: &str| {
        if judged.contains("refund") {
            0.97
        } else if judged.contains("maybe") {
            0.55
        } else {
            0.03
        }
    };
    let answer = match question.kind() {
        QuestionKind::Decide => serde_json::json!({
            "type": "noul", "noul": single(evidence)
        }),
        QuestionKind::Choose => {
            // Raw weights by keyword, then normalized to total one, because
            // an answer distribution totals one and the core checks it.
            let raw: Vec<f64> = question
                .members()
                .iter()
                .map(|option| {
                    if option.contains("refund") {
                        0.62
                    } else if option.contains("maybe") {
                        0.31
                    } else {
                        0.07
                    }
                })
                .collect();
            let total: f64 = raw.iter().sum();
            let odds: Vec<(String, f64)> = question
                .members()
                .iter()
                .zip(raw.iter())
                .map(|(option, weight)| (option.clone(), weight / total))
                .collect();
            let winner = odds
                .iter()
                .max_by(|one, two| {
                    one.1.partial_cmp(&two.1).unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(option, _)| option.clone())
                .unwrap_or_default();
            let probabilities: serde_json::Map<String, serde_json::Value> = odds
                .into_iter()
                .map(|(option, probability)| (option, serde_json::Value::from(probability)))
                .collect();
            serde_json::json!({
                "type": "choice", "choice": winner,
                "probabilities": probabilities
            })
        }
        QuestionKind::Tag => {
            // The adapter expands a tag question into one noul question a
            // label, so the null reply holds one noul answer a label,
            // keyed q1..qN in the question's own order.
            let mut answers = serde_json::Map::new();
            for (place, label) in question.members().iter().enumerate() {
                let probability = if label.contains("refund") {
                    0.72
                } else if label.contains("maybe") {
                    0.55
                } else {
                    0.03
                };
                answers.insert(format!("q{}", place + 1), serde_json::json!({
                    "type": "noul", "noul": probability
                }));
            }
            return Ok(serde_json::json!({
                "model": question.model(),
                "answers": answers
            })
            .to_string()
            .into_bytes());
        }
        QuestionKind::Score => {
            let odds = if evidence.contains("refund") {
                vec![("0".to_owned(), 0.05), ("1".to_owned(), 0.20), ("2".to_owned(), 0.75)]
            } else if evidence.contains("maybe") {
                vec![("0".to_owned(), 0.20), ("1".to_owned(), 0.55), ("2".to_owned(), 0.25)]
            } else {
                vec![("0".to_owned(), 0.34), ("1".to_owned(), 0.33), ("2".to_owned(), 0.33)]
            };
            let probabilities: serde_json::Map<String, serde_json::Value> = odds
                .into_iter()
                .map(|(level, probability)| (level, serde_json::Value::from(probability)))
                .collect();
            let legend: serde_json::Map<String, serde_json::Value> = question
                .members()
                .iter()
                .enumerate()
                .map(|(place, level)| (place.to_string(), serde_json::Value::from(level.clone())))
                .collect();
            serde_json::json!({
                "type": "score", "score": 1.0, "legend": legend,
                "probabilities": probabilities
            })
        }
    };
    Ok(serde_json::json!({
        "model": question.model(),
        "answers": { "q1": answer }
    })
    .to_string()
    .into_bytes())
}

/// Where one request goes, with the endpoint path already on the base.
fn endpoint_of(settings: &ResolvedConfig, model: &str) -> Result<String, Error> {
    let backend = Backend::resolve(None, Some(&settings.base), model)
        .map_err(|error| Error::usage(error.to_string()))?;
    Ok(backend.url().as_str().to_owned())
}

/// One POST, retried as `backends.md` says, counting every send.
fn post(
    agent: &ureq::Agent,
    url: &str,
    body: Vec<u8>,
    options: &Options<'_>,
    settings: &ResolvedConfig,
) -> Result<(Vec<u8>, u32), Error> {
    let limit = settings.max_retries;
    let mut waited = Duration::from_secs(1);
    let mut attempt = 0_u32;
    let mut sends = 0_u32;
    loop {
        Error::guard(options)?;
        let budget = match options.remaining() {
            Some(left) => settings.timeout.min(left),
            None => settings.timeout,
        };
        let request = agent
            .post(url)
            .header("content-type", "application/json")
            .config()
            .timeout_per_call(Some(budget))
            .build();
        // The counter counts what left the process. A send that left
        // counts, and a retry that is sent counts again because the vendor
        // bills each one. A connection refused before anything left counts
        // nothing; a dead pooled connection is retried inside one send() by
        // the client, so it counts once, because nothing was sent twice.
        let sent = request.send(&body);
        let mut response = match sent {
            Ok(response) => {
                REQUESTS.fetch_add(1, Ordering::Relaxed);
                sends += 1;
                response
            }
            Err(error) => {
                if left_the_machine(&error) {
                    REQUESTS.fetch_add(1, Ordering::Relaxed);
                    sends += 1;
                }
                if options.passed() {
                    return Err(Error::deadline(options.seconds()));
                }
                if attempt < limit {
                    attempt += 1;
                    let nap = match options.remaining() {
                        Some(left) => waited.min(left),
                        None => waited,
                    };
                    sleep_checked(nap, options)?;
                    waited = waited.saturating_mul(2);
                    continue;
                }
                return Err(classify_transport(&error));
            }
        };
        let status = response.status().as_u16();
        if RETRIED.contains(&status) && attempt < limit && !options.passed() {
            let wait = retry_wait(&response).unwrap_or(waited).min(MAX_RETRY_WAIT);
            attempt += 1;
            let nap = match options.remaining() {
                Some(left) => wait.min(left),
                None => wait,
            };
            sleep_checked(nap, options)?;
            waited = waited.saturating_mul(2);
            continue;
        }
        if RETRIED.contains(&status) {
            return Err(Error::backend_retryable(format!("HTTP {status}: {}", phrase(status))));
        }
        if !(200..300).contains(&status) {
            return Err(Error::backend_not_retryable(format!("HTTP {status}: {}", phrase(status))));
        }
        let body = response
            .body_mut()
            .with_config()
            .limit(MAX_RESPONSE_BYTES)
            .read_to_vec()
            .map_err(|error| Error::backend_not_retryable(error.to_string()))?;
        count_tokens(&body);
        return Ok((body, sends));
    }
}

/// Carry the usage a reply reported into the counter, when it holds one.
fn count_tokens(body: &[u8]) {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(body) else {
        return;
    };
    let usage = value.get("usage").filter(|held| held.is_object());
    if let Some(usage) = usage {
        let input = usage.get("input_tokens").and_then(serde_json::Value::as_u64).unwrap_or(0);
        let output = usage.get("output_tokens").and_then(serde_json::Value::as_u64).unwrap_or(0);
        TOKENS.fetch_add(input + output, Ordering::Relaxed);
    }
}

/// A transport failure, split by whether a second try could help.
/// Whether a failed send left the process: a refusal, a failed connect,
/// or a name that did not resolve never reached the wire, so the counter
/// stays put for them. Every other transport failure may have left, and
/// counts.
fn left_the_machine(error: &ureq::Error) -> bool {
    match error {
        ureq::Error::Io(io) => io.kind() != std::io::ErrorKind::ConnectionRefused,
        ureq::Error::ConnectionFailed
        | ureq::Error::HostNotFound
        | ureq::Error::BadUri(_) => false,
        _ => true,
    }
}

fn classify_transport(error: &ureq::Error) -> Error {
    let refused = match error {
        ureq::Error::Io(io) => io.kind() == std::io::ErrorKind::ConnectionRefused,
        _ => false,
    };
    if refused {
        return Error::backend_not_retryable(format!("{error}: the address refused the connection"));
    }
    Error::backend_retryable(error.to_string())
}

/// The wait a response's headers name, milliseconds first, seconds second.
fn retry_wait(response: &ureq::http::Response<ureq::Body>) -> Option<Duration> {
    let header = |name: &str| {
        response
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
    };
    if let Some(milliseconds) = header("retry-after-ms") {
        return Some(Duration::from_millis(milliseconds));
    }
    header("retry-after").map(Duration::from_secs)
}

/// How long one slice of a retry wait holds before the token and the
/// deadline are checked again.
const SLEEP_SLICE: Duration = Duration::from_millis(100);

/// Wait out a retry backoff in slices, hearing a cancel and a spent
/// deadline inside the wait.
///
/// A backoff may run to sixty seconds (`MAX_RETRY_WAIT`); a plain sleep
/// would ignore a stop gesture for that whole time. The wait ends early
/// with the cancelled kind when the token is set, and with the deadline
/// kind when the budget passes.
fn sleep_checked(nap: Duration, options: &Options<'_>) -> Result<(), Error> {
    let end = Instant::now() + nap;
    loop {
        Error::guard(options)?;
        let left = end.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Ok(());
        }
        thread::sleep(left.min(SLEEP_SLICE));
    }
}

/// The fixed phrase that follows a status, as `backends.md` sets each one.
const fn phrase(status: u16) -> &'static str {
    match status {
        401 => "the key was refused",
        402 => "the account has no credit",
        403 => "the key may not use this model or address",
        404 => "nothing answers at this address",
        422 => "the backend refused the request as malformed or too large",
        429 => "the backend's rate limit was reached",
        _ => "the backend failed",
    }
}

/// The process state: one pool and one width gate, stamped with the pid
/// and the transport shape it was built for.
struct Inner {
    pid: u32,
    base: String,
    width: usize,
    timeout: Duration,
    agent: ureq::Agent,
    gate: Gate,
}

/// The counts the width gate keeps, behind its own lock.
struct GateCounts {
    busy: usize,
    limit: usize,
}

/// The process-wide width gate: a counting semaphore on a mutex and
/// condvar.
struct Gate {
    counts: Mutex<GateCounts>,
    signal: Condvar,
}

/// One permit taken from the gate, given back on drop.
struct Ticket<'a> {
    counts: &'a Mutex<GateCounts>,
    signal: &'a Condvar,
}

impl Drop for Ticket<'_> {
    /// Give the permit back and wake one waiter.
    fn drop(&mut self) {
        let mut counts = lock(&self.counts);
        counts.busy -= 1;
        self.signal.notify_one();
    }
}

impl Gate {
    /// Take a permit, waiting on the tick when the width is full.
    ///
    /// # Errors
    ///
    /// Returns the cancelled kind when the token is set, and the deadline's
    /// own kind when the budget is gone.
    fn enter(&self, options: &Options<'_>) -> Result<Ticket<'_>, Error> {
        let mut counts = lock(&self.counts);
        loop {
            Error::guard(options)?;
            if counts.busy < counts.limit {
                counts.busy += 1;
                return Ok(Ticket { counts: &self.counts, signal: &self.signal });
            }
            let waited = match self.signal.wait_timeout(counts, TICK) {
                Ok(pair) => pair,
                Err(poisoned) => poisoned.into_inner(),
            };
            counts = waited.0;
        }
    }
}

/// The live state, as a leaked box holding the `Arc`, behind an atomic
/// pointer that is exchanged atomically and never unwound.
///
/// A fork during a batch leaves locks held by threads that do not exist in
/// the child, so the child's rebuild path must take no lock any request
/// path can hold. The read is one atomic load and an `Arc` clone; the
/// rebuild is a fresh construction and one compare-and-swap, and the box an
/// exchange retires is leaked on purpose.
static STATE: AtomicPtr<Arc<Inner>> = AtomicPtr::new(ptr::null_mut());

/// Lock a mutex without ever panicking on another thread's failure.
fn lock<T>(held: &Mutex<T>) -> MutexGuard<'_, T> {
    held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Build fresh state for this pid and transport shape.
fn build_inner(pid: u32, settings: &ResolvedConfig) -> Result<Inner, Error> {
    let width = settings.width.max(1);
    let secure = Backend::resolve(None, Some(&settings.base), "jev-latest")
        .map(|backend| backend.is_secure())
        .unwrap_or(false);
    // The pool must hold what the gate lets through: `ureq` pools 10 idle
    // connections overall and 3 per host by default, so a wide engine would
    // open a connection for nearly every request.
    let mut builder = ureq::Agent::config_builder()
        .timeout_global(Some(settings.timeout))
        .http_status_as_error(false)
        .max_redirects(0)
        .max_idle_connections(width)
        .max_idle_connections_per_host(width);
    if !secure {
        builder = builder.proxy(None);
    }
    Ok(Inner {
        pid,
        base: settings.base.clone(),
        width,
        timeout: settings.timeout,
        agent: builder.build().into(),
        gate: Gate {
            counts: Mutex::new(GateCounts { busy: 0, limit: width }),
            signal: Condvar::new(),
        },
    })
}

/// The state for this process and transport shape, rebuilt after a fork
/// and when an engine value with a different address, width, or timeout
/// arrives.
///
/// # Errors
///
/// Returns the options' own failure when the call is already stopped, and
/// an error of the backend kind when the pool cannot be built.
fn state(options: &Options<'_>, settings: &ResolvedConfig) -> Result<Arc<Inner>, Error> {
    let now = std::process::id();
    loop {
        let current = STATE.load(Ordering::Acquire);
        if !current.is_null() {
            // SAFETY: a stored pointer is never freed. Retirement leaks the
            // old value on purpose, so this read stays valid forever.
            let inner = unsafe { (*current).clone() };
            if inner.pid == now
                && inner.base == settings.base
                && inner.width == settings.width.max(1)
                && inner.timeout == settings.timeout
            {
                return Ok(inner);
            }
        }
        Error::guard(options)?;
        let fresh = Arc::new(build_inner(now, settings)?);
        let boxed = Box::into_raw(Box::new(Arc::clone(&fresh)));
        if STATE
            .compare_exchange(current, boxed, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            // SAFETY: the box this exchange retired is deliberately leaked:
            // dropping it could take a lock a vanished thread still holds.
            return Ok(fresh);
        }
        // SAFETY: `boxed` was never published, so this is the only handle.
        drop(unsafe { Box::from_raw(boxed) });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex as StdMutex;
    use std::sync::MutexGuard as StdMutexGuard;

    use thinkthen_contract::{
        Annotated, Answer, Engine, ErrorKind, Options, Question, QuestionSet,
    };

    use super::BlockingEngine;

    const CUT: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":0.9}"#;
    const BAND: &str = r#"{"decide":"Does the writer ask for a refund?","threshold":"0.2:0.8"}"#;

    /// Set the null backend once, serialized against the other null tests.
    fn null() -> StdMutexGuard<'static, ()> {
        static SEAT: StdMutex<()> = StdMutex::new(());
        let seat = SEAT.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        // Sound in this binary: every engine call runs after the seat is
        // held, so no other thread reads the environment while it is set.
        unsafe { std::env::set_var("ENGINE_NULL", "1") };
        unsafe { std::env::remove_var("ENGINE_SYNTHETIC_PARTIAL") };
        seat
    }

    /// The null backend with the test-only partial-failure fixture armed,
    /// serialized the same way.
    fn synthetic() -> StdMutexGuard<'static, ()> {
        let seat = null();
        // Sound in this binary: the seat is held, so no engine call runs
        // between this set and the engine value that reads it.
        unsafe { std::env::set_var("ENGINE_SYNTHETIC_PARTIAL", "1") };
        seat
    }

    /// Every verb answers on the null backend with the conformance
    /// numbers, and the rules hold at the verbs.
    #[test]
    fn every_verb_answers_on_null() {
        let _seat = null();
        let tt = BlockingEngine::from_env();

        let yes = tt.decide(&Question::from_json(CUT).expect("parses"), "i want a refund");
        assert_eq!(yes.expect("answers"), Answer::Yes);

        let band = Question::from_json(BAND).expect("parses");
        let middle = tt.decide(&band, "maybe later");
        assert_eq!(middle.expect("answers"), Answer::Unsure);

        let choose = Question::choose("Which team owns this?", &[
            "the refund desk",
            "the maybe desk",
            "anywhere else",
        ])
        .expect("builds")
        .build()
        .expect("builds");
        let pick = tt.choose(&choose, "please route this ticket");
        assert_eq!(pick.expect("answers").as_deref(), Some("the refund desk"));

        let score = Question::score("How strong is the refund claim?", &["low", "mid", "high"])
            .expect("builds");
        let scored = tt.score(&score, "maybe later").expect("answers");
        assert!((scored.value - 1.05).abs() < 1e-9);
        assert_eq!(scored.nearest, "mid");

        let tag = Question::tag("Does this line show the word?", &[
            "the refund word",
            "the maybe word",
            "nothing",
        ])
        .expect("builds");
        let held = tt.tag(&tag, "a plain line");
        assert_eq!(held.expect("answers"), ["the refund word", "the maybe word"]);

        let set = QuestionSet::from_json(
            r#"{"version":1,"questions":{
                "spam":{"decide":"Spam?","threshold":0.5},
                "kind":{"choose":"Which kind?","options":["bug","feature"]}}}"#,
        )
        .expect("parses");
        let rows = tt.annotate(&set, &["maybe later"], None).expect("answers");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0].0, "kind");
        // `bug` and `feature` carry no keyword, so the null backend ties
        // them at one half each, and a tied choice reads as unsure — the
        // core's own rule, which the contract keeps.
        assert!(matches!(&rows[0][0].1, Annotated::Choice(None)));

        let trail = tt.details(&Question::from_json(CUT).expect("parses"), "i want a refund");
        let trail = trail.expect("answers");
        assert_eq!(trail.sends, 1);
        assert_eq!(trail.digest.len(), 64);
        assert_eq!(trail.nearest, None, "no level on a decide question");

        // Pick 6: a score question's details carry the nearest level's
        // name and its own probability; the decide-only answer member
        // reads as the core's own resolved outcome.
        let score_trail = tt.details(&score, "maybe later").expect("answers");
        assert_eq!(score_trail.nearest.as_deref(), Some("mid"));
        assert!((score_trail.probability - 0.55).abs() < 1e-9);
        assert_eq!(score_trail.answer, Answer::Yes);
    }

    /// The verb rules refuse what they must, and the counters count sends.
    #[test]
    fn the_rules_hold() {
        let _seat = null();
        let tt = BlockingEngine::from_env();
        super::reset_usage();

        let band = Question::from_json(BAND).expect("parses");
        let refused = tt.filter(&band, &["a", "b"], None);
        assert_eq!(refused.unwrap_err().kind, ErrorKind::Usage);

        let cut_with_threshold = Question::from_json(CUT).expect("parses");
        let ranked = tt.rank(&cut_with_threshold, &["a"], None);
        assert_eq!(ranked.unwrap_err().kind, ErrorKind::Usage);

        let one = tt.find(&Question::from_json(CUT).expect("parses"), &["only"]);
        assert_eq!(one.unwrap_err().kind, ErrorKind::Usage);

        let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#)
            .expect("parses");
        let records = ["refund now", "good morning", "refund again"];
        let kept = tt.filter(&question, &records, None).expect("answers");
        assert_eq!(kept, [0, 2]);

        let order = tt.rank(&Question::from_json(r#"{"decide":"Refund?"}"#).expect("parses"), &records, None);
        let order = order.expect("answers");
        assert_eq!(order[0].index, 0);
        assert_eq!(order[2].index, 1);

        let found = tt.find(&Question::from_json(r#"{"decide":"Refund?"}"#).expect("parses"), &records);
        let found = found.expect("answers");
        assert_eq!(found.index, Some(0));
        assert!(tt.usage().requests >= 3);
    }

    /// A batch answers in order, a set token stops it, and the deadline is
    /// its own kind.
    #[test]
    fn the_batch_answers_and_stops() {
        let _seat = null();
        let tt = BlockingEngine::from_env();
        let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#)
            .expect("parses");
        let records = ["refund now", "good morning", "maybe so", "refund again", "bye"];
        let all = tt.decide_many(&question, &records, None).expect("answers");
        assert_eq!(
            all.iter().map(|judgment| judgment.probability).collect::<Vec<_>>(),
            [0.97, 0.03, 0.55, 0.97, 0.03]
        );

        let token = thinkthen_contract::Cancel::new();
        token.cancel();
        let stopped = tt.decide_many_opts(
            &question,
            &records,
            thinkthen_contract::Options::new().cancel(&token),
            None,
        );
        assert_eq!(stopped.unwrap_err().kind, ErrorKind::Cancelled);

        let spent = Options::new().deadline_in(std::time::Duration::from_millis(1));
        std::thread::sleep(std::time::Duration::from_millis(3));
        let late = tt.decide_many_opts(&question, &records, spent, None);
        assert_eq!(late.unwrap_err().kind, ErrorKind::Deadline);
    }

    /// The poll runs while answers flow: a fast backend must not starve
    /// the interrupt. Reproduced before the fix: SIGINT one second into a
    /// three-million-record null batch raised at 8.48 s, after the batch.
    #[test]
    fn the_poll_runs_on_a_busy_channel() {
        let _seat = null();
        let tt = BlockingEngine::from_env();
        let question = Question::from_json(CUT).expect("parses");
        let owned: Vec<String> = (0..1_000_000).map(|i| format!("refund {i}")).collect();
        let records: Vec<&str> = owned.iter().map(String::as_str).collect();
        let token = thinkthen_contract::Cancel::new();
        let calls = std::sync::atomic::AtomicUsize::new(0);
        let mut poll = || {
            calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            token.cancel();
        };
        let options = Options::new().cancel(&token);
        let stopped = tt.decide_many_opts(&question, &records, options, Some(&mut poll));
        assert_eq!(stopped.unwrap_err().kind, ErrorKind::Cancelled);
        assert!(calls.load(std::sync::atomic::Ordering::Relaxed) >= 1);
    }

    /// The conformance file, embedded so the pins below read the same bytes
    /// the validator and every surface read.
    const CONFORMANCE: &str = include_str!("../../conformance/conformance.json");

    fn case(id: &str) -> serde_json::Value {
        let file: serde_json::Value =
            serde_json::from_str(CONFORMANCE).expect("the conformance file parses");
        file["cases"]
            .as_array()
            .expect("cases is an array")
            .iter()
            .find(|held| held["id"] == id)
            .unwrap_or_else(|| panic!("case {id} exists"))
            .clone()
    }

    /// 0053: the requests list is the production digest of the logical
    /// request, computed the way `--record` names the file, and the
    /// conformance case pins the same value.
    #[test]
    fn the_requests_digest_follows_the_production_rule() {
        let _seat = null();
        let tt = BlockingEngine::from_env();
        let question = Question::from_json(CUT).expect("parses");
        let evidence = "I want my money back";
        let details = tt.details(&question, evidence).expect("details");
        assert_eq!(details.requests.len(), 1, "one logical request, one element");
        assert_eq!(details.failed_questions, 0, "no failed question in a details result");
        let pinned = case("73-details-carries-requests")["expect"]["details"]["requests"][0]
            .as_str()
            .expect("the case pins the digest")
            .to_owned();
        assert_eq!(details.requests[0], pinned, "the engine and the case agree");
        let direct = super::request_digest(&question, None, evidence).expect("digests");
        assert_eq!(direct, pinned, "the helper and the engine agree");
        assert_eq!(details.requests[0].len(), 64, "64 hex figures");
        assert!(details.requests[0].chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }

    /// 0054: one logical question may fail while its neighbour answers, the
    /// marker is the ruled JSON, and the count is one. The fixture is armed
    /// by its test-only opt-in, the way `74-annotate-preserves-good-answers`
    /// arms it.
    #[test]
    fn the_partial_failure_marker_is_the_ruled_shape() {
        let _seat = synthetic();
        let tt = BlockingEngine::from_env();
        let set = QuestionSet::from_json(
            r#"{"version":1,"questions":{
                "kind":{"decide":"Is this a refund request?","threshold":0.5},
                "topic":{"decide":"Is this a billing problem?","threshold":0.5}}}"#,
        )
        .expect("parses");
        let rows = tt
            .annotate(&set, &[super::SYNTHETIC_PARTIAL_RECORD], None)
            .expect("answers");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0].0, "kind", "name order: kind, then topic");
        assert_eq!(rows[0][0].1, Annotated::Decision(Answer::Yes), "the neighbour answers");
        assert_eq!(rows[0][1].0, "topic");
        assert_eq!(
            rows[0][1].1,
            Annotated::Failed(thinkthen_contract::Failed {
                kind: thinkthen_contract::FailureKind::Backend,
                cause: thinkthen_contract::Cause::MissingAnswer,
            }),
            "the ruled marker"
        );
        let json = serde_json::to_string(&rows[0]).expect("clean");
        assert!(
            json.contains(r#"{"failed":{"kind":"backend","cause":"missing_answer"}}"#),
            "the marker's JSON is the ruled one: {json}"
        );
        assert_eq!(thinkthen_contract::failed_questions(&rows), 1, "one failed logical question");
    }

    /// Finding 7: with the test-only opt-in unset, the fixture text answers
    /// like any other input — no caller ever meets a fake failure.
    #[test]
    fn the_fixture_text_answers_on_the_default_path() {
        let _seat = null();
        let tt = BlockingEngine::from_env();
        let set = QuestionSet::from_json(
            r#"{"version":1,"questions":{
                "kind":{"decide":"Is this a refund request?","threshold":0.5},
                "topic":{"decide":"Is this a billing problem?","threshold":0.5}}}"#,
        )
        .expect("parses");
        let rows = tt
            .annotate(&set, &[super::SYNTHETIC_PARTIAL_RECORD], None)
            .expect("answers");
        assert_eq!(rows.len(), 1);
        for (name, value) in &rows[0] {
            assert!(
                !matches!(value, Annotated::Failed(_)),
                "{name} answered, not failed, on the default path"
            );
        }
        assert_eq!(thinkthen_contract::failed_questions(&rows), 0, "no failed logical question");
        assert_eq!(rows[0][0].1, Annotated::Decision(Answer::Yes), "the fixture text is a refund");
    }

    /// The ruled record row serializes as `{"input","value"}` and carries
    /// the bare answer in the command's own JSON idiom.
    #[test]
    fn the_record_row_is_the_ruled_shape() {
        let _seat = null();
        let tt = BlockingEngine::from_env();
        let question = Question::from_json(CUT).expect("parses");
        let records = ["i want a refund now", "good morning"];
        let judgments = tt.decide_many(&question, &records, None).expect("bulk");
        let rows: Vec<thinkthen_contract::Row<Option<bool>>> = records
            .iter()
            .zip(&judgments)
            .map(|(input, judgment)| thinkthen_contract::Row {
                input: (*input).to_owned(),
                value: judgment.value(),
            })
            .collect();
        let json = thinkthen_contract::rows_json(&rows);
        assert_eq!(
            json,
            r#"[{"input":"i want a refund now","value":true},{"input":"good morning","value":false}]"#,
            "the ruled row list"
        );
    }

    /// The request limit refuses a bulk call before its first request.
    #[test]
    fn the_limit_refuses_first() {
        let _seat = null();
        let mut settings = thinkthen_contract::Settings::default();
        settings.max_requests = Some(2);
        let tt = BlockingEngine::from_settings(settings);
        let question = Question::from_json(r#"{"decide":"Refund?","threshold":0.5}"#)
            .expect("parses");
        super::reset_usage();
        let refused = tt.decide_many(&question, &["a", "b", "c"], None);
        assert_eq!(refused.unwrap_err().kind, ErrorKind::Usage);
        assert_eq!(super::REQUESTS.load(std::sync::atomic::Ordering::Relaxed), 0);
    }
}
