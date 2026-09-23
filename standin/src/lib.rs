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
//! a [`EngineConfig`] leaves unset: `THINKTHEN_NULL` for the in-process
//! backend, `THINKTHEN_BASE_URL` for the stub on the wire (the contract's
//! own name), `THINKTHEN_TIMEOUT_SECS` (30), `THINKTHEN_MAX_RETRIES` (2),
//! and `THINKTHEN_WIDTH` (4). The older unprefixed spellings
//! (`ENGINE_NULL`, `ENGINE_BASE_URL`, `ENGINE_TIMEOUT_SECS`,
//! `ENGINE_MAX_RETRIES`, `ENGINE_WIDTH`) keep working, deprecated; the
//! prefixed spelling wins when both are set. No key is read and none is
//! sent.
//!
//! One fixture is compiled in only when the build asks for it: the
//! `synthetic-partial` cargo feature arms the annotate partial-failure
//! fixture (see [`SYNTHETIC_PARTIAL_RECORD`]). A default build carries no
//! fixture code at all, so a shipped library can never be talked into a
//! fake failure by an environment variable.

use std::ptr;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::sync::OnceLock;
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
}

/// Parse one environment switch as a value, so `THINKTHEN_NULL=0` and
/// `=false` mean OFF rather than enabling the in-process backend by the
/// accident of being set (review finding, 2026-09-22).
///
/// An unrecognized spelling also means OFF: a typo never invents answers,
/// and a test that needs the null backend must spell its switch correctly.
fn env_bool(name: &str, older: &str) -> Option<bool> {
    let value = std::env::var(name)
        .ok()
        .or_else(|| std::env::var(older).ok())?;
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "t" => Some(true),
        "0" | "false" | "no" | "off" | "f" | "" => Some(false),
        _ => None,
    }
}

impl ResolvedConfig {
    /// Resolve one engine value's knobs: every config field set wins, the
    /// environment answers for what it leaves unset, and the built-in
    /// defaults answer last.
    fn resolve(config: &EngineConfig) -> Self {
        let env = |name: &str| std::env::var(name).ok().filter(|value| !value.trim().is_empty());
        // The `THINKTHEN_` spelling is the settled one; the older `ENGINE_`
        // spelling keeps working and the prefixed name wins when both are
        // set (standin/NOTES.md, 2026-09-22).
        let setting = |name: &str, older: &str| env(name).or_else(|| env(older));
        let number = |name: &str, older: &str| {
            setting(name, older).and_then(|value| value.parse::<u64>().ok())
        };
        Self {
            null: env_bool("THINKTHEN_NULL", "ENGINE_NULL").unwrap_or(false),
            base: config
                .address
                .clone()
                .or_else(|| setting("THINKTHEN_BASE_URL", "ENGINE_BASE_URL"))
                .unwrap_or_else(|| "http://127.0.0.1:8091/v1".into()),
            timeout: config.timeout.unwrap_or_else(|| {
                Duration::from_secs(number("THINKTHEN_TIMEOUT_SECS", "ENGINE_TIMEOUT_SECS").unwrap_or(30))
            }),
            max_retries: config.max_retries.unwrap_or_else(|| {
                number("THINKTHEN_MAX_RETRIES", "ENGINE_MAX_RETRIES").unwrap_or(2) as u32
            }),
            width: config
                .width
                .or_else(|| {
                    number("THINKTHEN_WIDTH", "ENGINE_WIDTH").map(|value| value as usize)
                })
                .unwrap_or(4),
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
                // a failed logical question): when the build enabled the
                // `synthetic-partial` feature, exactly this record's last
                // file-order question returns the ruled marker and its
                // neighbours answer normally. The conformance case
                // `74-annotate-preserves-good-answers` pins the shape in
                // that build, and DIVERGENCES.md marks it synthesized.
                // A default build carries no fixture code, so this record
                // answers like any other at every door.
                if SYNTHETIC_PARTIAL_ARMED
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
            requests: vec![request_digest_for(
                &self.config.base,
                question,
                self.model.as_deref(),
                evidence,
            )?],
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

/// Requests this process has on the wire right now, and the most it has
/// had at once, so a test can prove a width holds across the whole state
/// table rather than one state at a time (review finding 15). One atomic
/// pair per send; nothing in production reads the peak.
static WIRE_INFLIGHT: AtomicU64 = AtomicU64::new(0);
static WIRE_PEAK: AtomicU64 = AtomicU64::new(0);

/// Enter one wire send, recording the peak.
fn wire_enter() {
    let now = WIRE_INFLIGHT.fetch_add(1, Ordering::Relaxed) + 1;
    WIRE_PEAK.fetch_max(now, Ordering::Relaxed);
}

/// Leave one wire send; the guard's drop calls this at the iteration's
/// end, after the body is read or the attempt has failed, and before any
/// retry sleep.
fn wire_leave() {
    WIRE_INFLIGHT.fetch_sub(1, Ordering::Relaxed);
}

/// One send's tenure on the wire.
struct WireGuard;

impl Drop for WireGuard {
    fn drop(&mut self) {
        wire_leave();
    }
}
static TOKENS: AtomicU64 = AtomicU64::new(0);

/// Zero the counters.
pub fn reset_usage() {
    REQUESTS.store(0, Ordering::Relaxed);
    TOKENS.store(0, Ordering::Relaxed);
}

/// The one synthesized partial-failure record, compiled in only when the
/// build enables the `synthetic-partial` feature.
///
/// No recording carries a failed logical question, so a fixture build
/// answers the ruled marker for exactly this record (its last file-order
/// question) and nothing else. A default build — every shipped library —
/// compiles the fixture out entirely, so no environment variable and no
/// caller can arm it. `74-annotate-preserves-good-answers` pins the
/// marker's shape in a fixture build and `conformance/DIVERGENCES.md`
/// marks it synthesized.
pub const SYNTHETIC_PARTIAL_RECORD: &str = "order 4471: charged twice, please refund";

/// Whether this build carries the synthetic partial-failure fixture: true
/// only under the `synthetic-partial` cargo feature, a compile-time door an
/// environment variable cannot open.
#[cfg(feature = "synthetic-partial")]
const SYNTHETIC_PARTIAL_ARMED: bool = true;

/// See the `synthetic-partial` arm above: a default build answers every
/// record for real.
#[cfg(not(feature = "synthetic-partial"))]
const SYNTHETIC_PARTIAL_ARMED: bool = false;

/// The recording digest the request this call would make is filed under,
/// resolved against the base URL the environment names.
///
/// Prefer [`request_digest_for`] on any path that already holds an engine
/// or an explicit base: this helper reads the environment for its base
/// (review finding 15, 2026-09-23: an engine with an explicit address
/// reported the environment's digest, a name for a request it would never
/// send).
///
/// The production rule (0053) is the same for both helpers: the adapter
/// name, the resolved URL, and the exact request bytes through
/// `recording::Exchange::digest`. The encoder is deterministic, so the
/// digest is computed on the null backend too; it names the request the
/// plan would make, and nothing is sent.
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
    let settings = ResolvedConfig::from_env();
    request_digest_for(&settings.base, question, model, evidence)
}

/// The recording digest for a request against one named base URL: the
/// engine's own path, so an explicit address wins over the environment
/// and the digest names the request that would actually be sent.
///
/// # Errors
///
/// The usage kind for a blank evidence text, an unresolvable model, or an
/// unbuildable plan; the backend kind when the plan does not encode.
pub fn request_digest_for(
    base: &str,
    question: &Question,
    model: Option<&str>,
    evidence: &str,
) -> Result<String, Error> {
    let text = Evidence::new(evidence).map_err(|error| Error::usage(error.to_string()))?;
    let model_name = model.unwrap_or(question.model());
    let model = ModelName::new(model_name).map_err(|error| Error::usage(error.to_string()))?;
    let plan = Plan::new(text, model, vec![question.core().clone()])
        .map_err(|error| Error::usage(error.to_string()))?;
    let wire = built_in::encode(&plan)
        .map_err(|error| Error::backend_not_retryable(error.to_string()))?;
    let backend = Backend::resolve(None, Some(base), model_name)
        .map_err(|error| Error::usage(error.to_string()))?;
    Ok(thinkthen_core::recording::Exchange::new(backend.url(), &wire)
        .digest()
        .as_str()
        .to_owned())
}

/// How many slot boxes have been retired into tombstones so far in this
/// process, so a probe can prove the sixteen-byte-per-eviction leak stays
/// proportional to churn and never to calls (review finding 1,
/// 2026-09-23).
#[must_use]
pub fn retired_boxes() -> u64 {
    RETIRED_BOXES.load(Ordering::Relaxed)
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
        // On the wire from the send to the read body's end.
        let _on_the_wire = WireGuard;
        wire_enter();
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
                let failure = classify_transport(&error);
                // Only a genuinely retryable failure earns the backoff: an
                // address that refused the connection refuses it again
                // within the second as well, so it fails at once instead of
                // burning the two backoff waits (review finding, 2026-09-22).
                if !failure.retryable || attempt >= limit {
                    return Err(failure);
                }
                attempt += 1;
                let nap = match options.remaining() {
                    Some(left) => waited.min(left),
                    None => waited,
                };
                sleep_checked(nap, options)?;
                waited = waited.saturating_mul(2);
                continue;
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

/// Split a transport failure by whether a second try could help.
///
/// A refusal, a failed connect, a name that did not resolve, and a broken
/// URI are conditions a retry within the second cannot change, so they are
/// not retryable and the caller fails at once (review finding, 2026-09-23:
/// a refused connection burned the backoff; DNS and TLS now fail fast too).
/// A TLS handshake failure is likewise deterministic on retry: the chain
/// and the configuration will not heal between attempts, so an I/O error
/// whose cause chain names TLS is not retryable either. Everything else
/// keeps the retry the backends page gives a transport failure.
fn classify_transport(error: &ureq::Error) -> Error {
    let refused = match error {
        ureq::Error::Io(io) => io.kind() == std::io::ErrorKind::ConnectionRefused,
        _ => false,
    };
    if refused {
        return Error::backend_not_retryable(format!("{error}: the address refused the connection"));
    }
    match error {
        ureq::Error::HostNotFound | ureq::Error::BadUri(_) => {
            return Error::backend_not_retryable(format!(
                "{error}: the address will not resolve; retrying cannot change it"
            ));
        }
        ureq::Error::ConnectionFailed => {
            return Error::backend_not_retryable(format!(
                "{error}: the connection could not be established"
            ));
        }
        ureq::Error::Io(io) if names_tls(io) => {
            return Error::backend_not_retryable(format!(
                "{error}: the TLS handshake failed; retrying cannot change it"
            ));
        }
        _ => {}
    }
    Error::backend_retryable(error.to_string())
}

/// Whether an I/O error's cause chain names a TLS failure, so a broken
/// handshake fails at once instead of burning the backoff.
fn names_tls(io: &std::io::Error) -> bool {
    let mut cause: Option<&dyn std::error::Error> = Some(io);
    while let Some(error) = cause {
        let text = error.to_string().to_ascii_lowercase();
        if text.contains("tls") || text.contains("rustls") || text.contains("handshake") {
            return true;
        }
        cause = error.source();
    }
    false
}

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

/// How many lanes one engine may run at once. Recorded with the ruling:
/// width is a concurrency promise, not a capacity request, and a host
/// asking for more lanes than this has mistyped a record count into a
/// width knob.
const MAX_WIDTH: usize = 4096;

/// One settings value's state: a pool and a width gate, stamped with the
/// pid and the settings it was built for.
struct Inner {
    pid: u32,
    base: String,
    width: usize,
    timeout: Duration,
    agent: ureq::Agent,
    gate: Gate,
    /// The lookup this state was last served on, in logical ticks, read by
    /// the victim chooser so evictions take the least-recently-used idle
    /// state and never a busy one while an idle one exists.
    last_used: AtomicU64,
}

impl Drop for Inner {
    /// Count the drop so a test can prove retirement closes states rather
    /// than leaking them (review finding 15).
    fn drop(&mut self) {
        INNER_DROPS.fetch_add(1, Ordering::Relaxed);
    }
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

/// How many settings values keep their own state in one process.
///
/// One state per settings value is the rule (review finding, 2026-09-22):
/// two engines whose settings differ never share a gate or a pool, so a
/// width of 1 holds across them and no call rebuilds a neighbour's state.
/// A process that uses more distinct settings values than this evicts the
/// least-recently-used idle state, retiring its pool to a list that closes
/// its sockets once no request holds it; sixty-four covers every surface's
/// own use with room for a host's churn.
const STATE_SLOTS: usize = 64;

/// A monotonically advancing counter of state lookups, the logical clock
/// the LRU reads. A real clock would need a syscall or a lazy `Instant`,
/// and a counter orders recency just as well with one atomic add.
static LOOKUPS: AtomicU64 = AtomicU64::new(0);

/// How many `Inner` values have been dropped, so a test can prove retired
/// states are closed rather than leaked (review finding 15).
static INNER_DROPS: AtomicU64 = AtomicU64::new(0);

/// One state per settings value, behind an array of atomic pointers that
/// are exchanged and never unwound.
///
/// A fork during a batch leaves locks held by threads that do not exist in
/// the child, so the child's rebuild path must take no lock any request
/// path can hold. The read is one atomic load and an `Arc` clone per
/// slot; the publish is a fresh construction and one compare-and-swap. A
/// value an eviction replaces moves to [`RETIRED`], whose sweep closes its
/// pool once no request holds it, so churn through more settings values
/// than [`STATE_SLOTS`] holds file descriptors flat instead of leaking a
/// pool per eviction forever.
static STATES: [AtomicPtr<Arc<Inner>>; STATE_SLOTS] =
    [const { AtomicPtr::new(ptr::null_mut()) }; STATE_SLOTS];

/// The value every evicted slot keeps, so a slot's box is never freed
/// while any reader can still hold its pointer.
///
/// A slot reader loads a pointer and clones the `Arc` behind it in two
/// adjacent steps, so a box whose slot was just taken must stay valid
/// memory holding a valid `Arc` forever (review finding 1, 2026-09-23:
/// freeing the box at eviction was a use-after-free under AddressSanitizer
/// five runs of six, and a SIGSEGV in release past sixty-four settings
/// values). The tombstone never matches any settings — its pid is zero,
/// and a real pid is never zero — so a straggler reader clones it, matches
/// nothing, and walks on. Because no box is ever freed, no address is
/// ever reused, and the compare-and-swaps below cannot confuse a fresh
/// box with a recycled one. The cost is sixteen leaked bytes per
/// eviction, counted by [`RETIRED_BOXES`] and bounded by the number of
/// distinct settings values a process ever builds, not by its calls.
static TOMBSTONE: OnceLock<Arc<Inner>> = OnceLock::new();

/// The tombstone every evicted slot keeps. Constructed once, cloned into
/// the slot's box in place of the retired `Arc`.
fn tombstone() -> Arc<Inner> {
    TOMBSTONE
        .get_or_init(|| {
            Arc::new(Inner {
                pid: 0,
                base: String::new(),
                width: 1,
                timeout: Duration::ZERO,
                agent: ureq::Agent::config_builder().build().into(),
                gate: Gate { counts: Mutex::new(GateCounts { busy: 0, limit: 1 }), signal: Condvar::new() },
                last_used: AtomicU64::new(0),
            })
        })
        .clone()
}

/// How many slot boxes have been retired into tombstones, so a test can
/// prove the sixteen-byte-per-eviction leak stays proportional to churn
/// and never to calls (review finding 1, 2026-09-23).
static RETIRED_BOXES: AtomicU64 = AtomicU64::new(0);

/// The registry that keeps every published slot box reachable for the
/// life of the process, so the never-freed design shows up to tools as
/// retained memory rather than a leak. Each entry is eight bytes beside
/// the box it names; both are bounded by the number of evictions, never
/// by calls. The pointer is never dereferenced — only listed — so its
/// lack of `Send` is irrelevant to the soundness of the list itself.
static RETAINED: Mutex<Vec<usize>> = Mutex::new(Vec::new());

/// Remember one box so its allocation stays reachable: the design never
/// frees a published box (see [`TOMBSTONE`]), and a listed address keeps
/// the sanitizer's view honest about that choice. An entry may outlive a
/// box that was freed unpublished — a stale number in a list, never
/// dereferenced.
fn retain(held: *mut Arc<Inner>) {
    lock(&RETAINED).push(held as usize);
}

/// Take the `Arc` out of one slot's box without freeing the box: the
/// tombstone takes its place, so the memory a reader may already hold a
/// pointer to stays allocated and valid for the life of the process.
fn vacate(held: *mut Arc<Inner>) -> Arc<Inner> {
    RETIRED_BOXES.fetch_add(1, Ordering::Relaxed);
    // SAFETY: the caller won the compare-and-swap that took this slot, so
    // this thread owns the box's contents; `ptr::replace` moves the old
    // `Arc` out and writes the tombstone in place, and the box itself is
    // deliberately never freed (see `TOMBSTONE`).
    unsafe { ptr::replace(held, tombstone()) }
}

/// A retired state waiting for its last holder to finish, on a lock-free
/// stack a fork can carry safely.
///
/// The node owns one strong reference; in-flight requests own the others.
/// [`sweep`] drops the node — and with it the pool and its idle sockets —
/// only when the node's reference is the last one and the retirement is
/// older than [`RETIRE_GRACE`] (see its comment for the residual window).
struct Retired {
    inner: Arc<Inner>,
    retired_at: Instant,
    next: *mut Retired,
}

/// The head of the retired stack.
static RETIRED: AtomicPtr<Retired> = AtomicPtr::new(ptr::null_mut());

/// How long a retired state must have been idle before a sweep may close
/// its pool.
///
/// A slot reader loads a pointer and clones the `Arc` behind it in two
/// adjacent steps. The slot's box is never freed — eviction leaves the
/// tombstone in it — so the reader's memory is always valid; what the
/// grace protects is the retired `Arc` itself: a node is retired the
/// moment it leaves its slot, and [`sweep`] drops the node — closing the
/// pool and its idle sockets — only when the node's reference is the last
/// one and the retirement is older than this grace. The residual window
/// is a reader preempted for the whole grace exactly between its load and
/// its clone: it then holds a tombstone, matches nothing, and looks the
/// state up again — a wrong answer is impossible, only a rebuilt state.
/// The trade buys not taking a lock the fork rule forbids.
#[cfg(not(test))]
const RETIRE_GRACE: Duration = Duration::from_secs(5);

/// Under test the grace is fifty milliseconds, so a churn proof can wait
/// it out without sleeping for seconds; the production value above is
/// the one a shipped build runs with.
#[cfg(test)]
const RETIRE_GRACE: Duration = Duration::from_millis(50);

/// Retire one state: push it onto the stack for a later sweep.
fn retire(inner: Arc<Inner>) {
    let node = Box::into_raw(Box::new(Retired {
        inner,
        retired_at: Instant::now(),
        next: ptr::null_mut(),
    }));
    push_retired(node);
}

/// Push one node or a chain of them onto the retired stack.
///
/// `nodes` may be a chain linked through `next`; the push splices the
/// current head onto the chain's tail and publishes the chain with one
/// compare-and-swap, retrying when another thread moved the head first.
fn push_retired(nodes: *mut Retired) {
    if nodes.is_null() {
        return;
    }
    let mut tail = nodes;
    // SAFETY: `nodes` is a chain this thread built or stole whole, and no
    // other thread can reach it until the publish below succeeds.
    while !unsafe { (*tail).next }.is_null() {
        tail = unsafe { (*tail).next };
    }
    let mut head = RETIRED.load(Ordering::Acquire);
    loop {
        // SAFETY: as above.
        unsafe { (*tail).next = head };
        match RETIRED.compare_exchange(head, nodes, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return,
            Err(moved) => head = moved,
        }
    }
}

/// Close the pools of retired states no request holds and no reader can
/// still be reaching for, and keep the ones still in flight.
///
/// The sweep steals the whole stack, walks it, and pushes the survivors
/// back — all with atomics, so a fork carries whatever half-swept state it
/// inherits without a lock to deadlock on.
fn sweep() {
    let mut node = RETIRED.swap(ptr::null_mut(), Ordering::AcqRel);
    let mut keep = ptr::null_mut();
    while !node.is_null() {
        // SAFETY: the swap stole the whole stack; this thread walks it
        // alone until it pushes the survivors back.
        let next = unsafe { (*node).next };
        let alone = Arc::strong_count(unsafe { &(*node).inner }) == 1;
        let aged = unsafe { (*node).retired_at }.elapsed() >= RETIRE_GRACE;
        if alone && aged {
            // SAFETY: the node left the stack with the swap and leaves no
            // other handle; dropping the box drops the last reference, the
            // `Inner`, and its pool's idle sockets.
            drop(unsafe { Box::from_raw(node) });
        } else {
            // SAFETY: as above.
            unsafe { (*node).next = keep };
            keep = node;
        }
        node = next;
    }
    push_retired(keep);
}

/// Whether one state answers this pid and these settings.
fn state_matches(inner: &Inner, pid: u32, settings: &ResolvedConfig) -> bool {
    inner.pid == pid
        && inner.base == settings.base
        && inner.width == settings.width.max(1)
        && inner.timeout == settings.timeout
}

/// The first slot a settings value tries, from a stable hash of its key, so
/// the same settings always walk the same probe order and two threads never
/// publish two states for one settings value.
fn home_slot(settings: &ResolvedConfig) -> usize {
    // FNV-1a, so the order is stable across runs and platforms.
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    let eat = |hash: &mut u64, bytes: [u8; 8]| {
        for byte in bytes {
            *hash ^= u64::from(byte);
            *hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    for byte in settings.base.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    eat(&mut hash, (settings.width.max(1) as u64).to_le_bytes());
    eat(&mut hash, (settings.timeout.as_nanos() as u64).to_le_bytes());
    (hash % STATE_SLOTS as u64) as usize
}

/// Lock a mutex without ever panicking on another thread's failure.
fn lock<T>(held: &Mutex<T>) -> MutexGuard<'_, T> {
    held.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Build fresh state for this pid and transport shape.
fn build_inner(pid: u32, settings: &ResolvedConfig) -> Result<Inner, Error> {
    // The width ceiling: every lane is a thread the batch spawns and a
    // socket the pool may hold, so a width beyond this is a host error,
    // refused with the ceiling named, before any thread or allocation is
    // attempted (review finding, 2026-09-23: width 100,000 panicked at
    // 322 MB in thread spawning). The ceiling is recorded here and in the
    // surfaces notes; a host that needs more names a number that fits.
    if settings.width > MAX_WIDTH {
        return Err(Error::usage(format!(
            "width {} exceeds the ceiling {}: the gate and the pool are built for at most {} lanes",
            settings.width, MAX_WIDTH, MAX_WIDTH
        )));
    }
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
        last_used: AtomicU64::new(0),
    })
}

/// The state for this pid and settings value, rebuilt after a fork and
/// published into its own slot, so two engines with different settings
/// never share a gate or a pool.
///
/// The home slot is claimed first, because two threads publishing one
/// settings value race the same compare-and-swap there and exactly one
/// wins. Every publish passes through [`dedup_after_publish`], and a
/// retraction restarts the whole lookup rather than answering with the
/// other copy, so two simultaneous publishers converge on one state
/// instead of trading retracted states back and forth.
///
/// # Errors
///
/// Returns the options' own failure when the call is already stopped, and
/// an error of the backend kind when the pool cannot be built.
fn state(options: &Options<'_>, settings: &ResolvedConfig) -> Result<Arc<Inner>, Error> {
    let now = std::process::id();
    loop {
        if let Some(inner) = state_lookup(now, settings) {
            return Ok(inner);
        }
        Error::guard(options)?;
        let fresh = Arc::new(build_inner(now, settings)?);
        let boxed = Box::into_raw(Box::new(Arc::clone(&fresh)));
        retain(boxed);
        let home = home_slot(settings);

        // Claim the home slot first: two publishers of one settings value
        // race the SAME compare-and-swap here, so exactly one wins and the
        // loser answers with the winner's state (review finding 15).
        let mut published_at = None;
        loop {
            let held = STATES[home].load(Ordering::Acquire);
            if held.is_null() {
                if STATES[home]
                    .compare_exchange(ptr::null_mut(), boxed, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    published_at = Some(home);
                    break;
                }
                continue;
            }
            // SAFETY: a stored pointer left its slot only through an
            // eviction, and a sweep frees a retired box only after the grace.
            let inner = unsafe { (*held).clone() };
            if state_matches(&inner, now, settings) {
                // SAFETY: `boxed` was never published, so this is the only
                // handle; the box goes, the fresh state is dropped.
                drop(unsafe { Box::from_raw(boxed) });
                return Ok(inner);
            }
            if evictable(held, now) {
                if STATES[home]
                    .compare_exchange(held, boxed, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    // SAFETY: the swap won, so this thread owns the slot's
                    // old contents; `vacate` moves the `Arc` to the retired
                    // stack and leaves the tombstone in the never-freed box,
                    // so a reader that loaded the old pointer keeps valid
                    // memory and a live state behind the grace.
                    let old = vacate(held);
                    retire(old);
                    published_at = Some(home);
                    break;
                }
                continue;
            }
            // The home slot is held by an active different settings
            // value: fall through to the walk and the victim.
            break;
        }

        // Walk the probe order for an empty slot, or a slot another
        // thread is publishing the same settings into.
        if published_at.is_none() {
            'walk: for step in 0..STATE_SLOTS {
                let place = (home + step) % STATE_SLOTS;
                loop {
                    let held = STATES[place].load(Ordering::Acquire);
                    if held.is_null() {
                        if STATES[place]
                            .compare_exchange(ptr::null_mut(), boxed, Ordering::AcqRel, Ordering::Acquire)
                            .is_ok()
                        {
                            published_at = Some(place);
                            break 'walk;
                        }
                        continue;
                    }
                    // SAFETY: a stored pointer left its slot only through
                    // an eviction, and a sweep frees a retired box only
                    // after the grace.
                    let inner = unsafe { (*held).clone() };
                    if state_matches(&inner, now, settings) {
                        // SAFETY: `boxed` was never published, so this is
                        // the only handle; the box goes, the fresh state
                        // is dropped.
                        drop(unsafe { Box::from_raw(boxed) });
                        return Ok(inner);
                    }
                    break;
                }
            }
        }

        // Every probe slot holds a different settings value: sweep what
        // earlier evictions left behind, then take the best victim — a
        // vanished pid's idle state first, then the least-recently-used
        // idle one, and a busy state only when every slot is busy — so a
        // width holds through churn and the retired pool's sockets close
        // instead of leaking (review finding 15).
        if published_at.is_none() {
            sweep();
            let place = choose_victim(now);
            loop {
                let held = STATES[place].load(Ordering::Acquire);
                if held.is_null() {
                    if STATES[place]
                        .compare_exchange(ptr::null_mut(), boxed, Ordering::AcqRel, Ordering::Acquire)
                        .is_ok()
                    {
                        published_at = Some(place);
                        break;
                    }
                    continue;
                }
                // SAFETY: the box is in the slot, so no sweep can have
                // freed it (a freed box was retired first and waited out
                // the grace).
                if unsafe { state_matches(&(*held), now, settings) } {
                    // SAFETY: as above.
                    let inner = unsafe { (*held).clone() };
                    // SAFETY: `boxed` was never published, so this is the
                    // only handle; the box goes, the fresh state is dropped.
                    drop(unsafe { Box::from_raw(boxed) });
                    return Ok(inner);
                }
                if STATES[place]
                    .compare_exchange(held, boxed, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    // SAFETY: the swap won, so this thread owns the slot's
                    // old contents; `vacate` moves the `Arc` to the retired
                    // stack and leaves the tombstone in the never-freed box,
                    // so a reader that loaded the old pointer keeps valid
                    // memory and a live state behind the grace.
                    let old = vacate(held);
                    retire(old);
                    published_at = Some(place);
                    break;
                }
            }
        }

        // Published somewhere: close the double-publish window, and a
        // retraction restarts the lookup so the survivor is the one answer.
        let place = published_at.expect("one of the three paths publishes");
        sweep();
        if dedup_after_publish(place, boxed, now, settings) {
            continue;
        }
        return Ok(fresh);
    }
}

/// Whether the state in one slot may be evicted: idle and unheld, or
/// belonging to a pid this process no longer is (its requests died with
/// the fork, so its gate will never clear on its own).
///
/// Held is read from the `Arc`'s own count — the window between a caller
/// taking a state and entering its gate is exactly the window where an
/// eviction would split one settings value across two live states and
/// double its width — and busy with `try_lock`, never a blocking lock: the
/// fork rule forbids the rebuild path waiting on a mutex a dead request
/// thread still holds, and a lock we cannot take simply counts as busy.
fn evictable(held: *mut Arc<Inner>, now: u32) -> bool {
    // SAFETY: a stored pointer left its slot only through an eviction,
    // and a sweep frees a retired box only after the grace.
    let inner = unsafe { &*held };
    let held_by_a_caller = Arc::strong_count(unsafe { &*held }) > 1;
    let busy = held_by_a_caller || inner.gate.counts.try_lock().map_or(true, |counts| counts.busy > 0);
    inner.pid != now || !busy
}

/// After a publish, close the double-publish window: another thread can
/// publish the same settings into a different slot — its walk found an
/// empty slot before ours, or its own eviction chose a different victim.
/// When any other slot holds a match, take ours back out, retire it
/// behind the grace, and say so; the caller then tries the whole lookup
/// again rather than answering with the other copy, because two threads
/// doing this at once can each retract against the other, and a retry
/// converges on the one that stays.
///
/// `false` means no other slot holds these settings and ours stays.
fn dedup_after_publish(
    place: usize,
    boxed: *mut Arc<Inner>,
    pid: u32,
    settings: &ResolvedConfig,
) -> bool {
    for other in 0..STATE_SLOTS {
        if other == place {
            continue;
        }
        let held = STATES[other].load(Ordering::Acquire);
        if held.is_null() || held == boxed {
            continue;
        }
        // SAFETY: a stored pointer left its slot only through an
        // eviction, and a sweep frees a retired box only after the grace.
        let theirs = unsafe { (*held).clone() };
        if state_matches(&theirs, pid, settings) {
            // Take ours back out when the slot still holds it. When the
            // swap fails, an evictor already replaced and took our box,
            // so its memory is theirs and we never touch it again.
            let reclaimed = STATES[place]
                .compare_exchange(boxed, ptr::null_mut(), Ordering::AcqRel, Ordering::Acquire)
                .is_ok();
            if reclaimed {
                // SAFETY: the swap above took the slot back, so this thread
                // owns the box's contents; `vacate` moves the `Arc` to the
                // retired stack and leaves the tombstone in the never-freed
                // box, so a reader that loaded our pointer keeps valid
                // memory and a live state behind the grace.
                let ours = vacate(boxed);
                retire(ours);
                sweep();
            }
            return true;
        }
    }
    false
}

/// The slot an eviction should take: a vanished pid's idle state first,
/// then the least-recently-used idle state, then a vanished pid's busy
/// one, and a busy state only when every slot is busy.
///
/// Busy is read with `try_lock`, never a blocking lock: the fork rule
/// forbids the rebuild path waiting on a mutex a dead request thread
/// still holds, and a lock we cannot take simply counts as busy. A state
/// also counts as busy while any thread holds its `Arc` — the window
/// between a caller taking the state and entering its gate is exactly
/// the window where an eviction would split one settings value across
/// two live states and double its width.
fn choose_victim(now: u32) -> usize {
    let mut best: Option<(usize, [u8; 2], u64)> = None;
    for place in 0..STATE_SLOTS {
        let held = STATES[place].load(Ordering::Acquire);
        if held.is_null() {
            continue;
        }
        // SAFETY: a stored box is never freed — eviction leaves the
        // tombstone in it — so the pointer stays valid memory holding a
        // valid `Arc` for the life of the process.
        let inner = unsafe { &*held };
        let vanished = inner.pid != now;
        let held_by_a_caller = Arc::strong_count(unsafe { &*held }) > 1;
        let busy = held_by_a_caller || inner.gate.counts.try_lock().map_or(true, |counts| counts.busy > 0);
        let rank = [u8::from(!vanished), u8::from(busy)];
        let last = inner.last_used.load(Ordering::Relaxed);
        if best.is_none_or(|(_, best_rank, best_last)| (rank, last) < (best_rank, best_last)) {
            best = Some((place, rank, last));
        }
    }
    best.map_or(0, |(place, _, _)| place)
}

/// The published state for this pid and settings, when one already exists.
fn state_lookup(pid: u32, settings: &ResolvedConfig) -> Option<Arc<Inner>> {
    for slot in &STATES {
        let held = slot.load(Ordering::Acquire);
        if !held.is_null() {
            // SAFETY: a stored box is never freed — eviction leaves the
            // tombstone in it — so the pointer stays valid memory holding
            // a valid `Arc` for the life of the process (review finding 1,
            // 2026-09-23).
            let inner = unsafe { (*held).clone() };
            if state_matches(&inner, pid, settings) {
                inner
                    .last_used
                    .store(LOOKUPS.fetch_add(1, Ordering::Relaxed), Ordering::Relaxed);
                return Some(inner);
            }
        }
    }
    None
}

/// The one backend-availability helper every surface's test suites call,
/// so a bare `cargo test` never silently passes a suite that only ran under
/// the gate's environment (review finding, 2026-09-22).
///
/// [`testkit::backend_kind`] recognizes the settled `THINKTHEN_` spellings
/// and the deprecated `ENGINE_` ones, and [`testkit::skip_note`] renders the
/// visible skip a suite prints when no backend is reachable. A surface's
/// runner replaces its private early-return with one call:
///
/// ```text
/// if let Some(note) = thinkthen_standin::testkit::skip_note("verbs") {
///     println!("SKIP {note}");
///     return;
/// }
/// ```
pub mod testkit {
    /// Which backend a bare test run can reach, if any.
    ///
    /// `Some("null")` is the in-process backend (`THINKTHEN_NULL` parsed as
    /// a value, so `=0` does not arm it), `Some("wire")` is a stub named by
    /// `THINKTHEN_BASE_URL`, and `None` means no backend is reachable.
    #[must_use]
    pub fn backend_kind() -> Option<&'static str> {
        if super::env_bool("THINKTHEN_NULL", "ENGINE_NULL") == Some(true) {
            return Some("null");
        }
        let set = |name: &str| {
            std::env::var(name)
                .ok()
                .filter(|value| !value.trim().is_empty())
                .is_some()
        };
        if set("THINKTHEN_BASE_URL") || set("ENGINE_BASE_URL") {
            return Some("wire");
        }
        None
    }

    /// The skip a suite prints when no backend is reachable: `None` when a
    /// backend exists and the suite must run.
    ///
    /// The note names the suite and the switch that arms the in-process
    /// backend, so a reader of a bare `cargo test` log sees the skip and how
    /// to end it.
    #[must_use = "the skip must be printed, never swallowed"]
    pub fn skip_note(suite: &str) -> Option<String> {
        backend_kind().map_or_else(
            || {
                Some(format!(
                    "{suite}: no backend is reachable; set THINKTHEN_NULL=1 for the in-process backend or THINKTHEN_BASE_URL for a stub"
                ))
            },
            |_| None,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex as StdMutex;
    use std::sync::MutexGuard as StdMutexGuard;
    use std::time::Duration;

    use thinkthen_contract::{
        Annotated, Answer, Engine, EngineConfig, ErrorKind, Options, Question, QuestionSet,
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
        seat
    }

    /// The null backend in a build that carries the test-only partial
    /// failure, serialized the same way. The fixture is a compile-time door
    /// (`synthetic-partial`), so this helper exists only in such a build.
    #[cfg(feature = "synthetic-partial")]
    fn synthetic() -> StdMutexGuard<'static, ()> {
        null()
    }

    /// Run one environment scenario under the null seat, restoring the
    /// seat's own switch afterwards — and every base URL the scenario may
    /// have named, so no test leaks a `THINKTHEN_BASE_URL` into its
    /// neighbours (review finding 18, 2026-09-23: the leak made the digest
    /// test fail four runs of fifty by test order).
    fn switches(scenario: impl FnOnce()) {
        let _seat = null();
        // Sound in this binary: the seat is held (see `null`).
        unsafe {
            std::env::remove_var("ENGINE_NULL");
            std::env::remove_var("THINKTHEN_NULL");
            std::env::remove_var("THINKTHEN_BASE_URL");
            std::env::remove_var("ENGINE_BASE_URL");
        }
        scenario();
        // Sound in this binary: the seat is still held. Restore the whole
        // removed set, not just the null switch: a scenario that named a
        // base URL must not leave it behind for the next test to read.
        unsafe {
            std::env::remove_var("THINKTHEN_BASE_URL");
            std::env::remove_var("ENGINE_BASE_URL");
            std::env::set_var("ENGINE_NULL", "1");
        }
    }

    /// `THINKTHEN_NULL` is parsed as a value: `=0` and `=false` mean OFF,
    /// only truthy spellings arm the in-process backend, and a typo means
    /// OFF rather than inventing answers (review finding 16).
    #[test]
    fn the_null_switch_is_parsed_as_a_value() {
        let resolve_null = || super::ResolvedConfig::resolve(&EngineConfig::default()).null;
        switches(|| {
            unsafe { std::env::set_var("THINKTHEN_NULL", "0") };
            assert!(!resolve_null(), "THINKTHEN_NULL=0 must not arm the null backend");
            unsafe { std::env::set_var("THINKTHEN_NULL", "false") };
            assert!(!resolve_null(), "THINKTHEN_NULL=false must not arm the null backend");
            unsafe { std::env::set_var("THINKTHEN_NULL", "1") };
            assert!(resolve_null(), "THINKTHEN_NULL=1 arms the null backend");
            unsafe { std::env::set_var("THINKTHEN_NULL", "true") };
            assert!(resolve_null(), "THINKTHEN_NULL=true arms the null backend");
            unsafe { std::env::set_var("THINKTHEN_NULL", "flase") };
            assert!(!resolve_null(), "a typo means OFF, never an invented answer");
            unsafe { std::env::remove_var("THINKTHEN_NULL") };
            unsafe { std::env::set_var("ENGINE_NULL", "0") };
            assert!(!resolve_null(), "the deprecated ENGINE_NULL=0 means OFF too");
        });
    }

    /// The shared test helper recognizes both spellings, refuses to call
    /// `=0` a backend, and renders a visible skip naming the way out
    /// (review finding: tests that silently pass without a backend).
    #[test]
    fn the_testkit_reports_a_visible_skip() {
        switches(|| {
            assert_eq!(super::testkit::backend_kind(), None, "nothing is reachable by default");
            let note = super::testkit::skip_note("verbs").expect("a skip is rendered");
            assert!(note.contains("verbs"), "the note names the suite");
            assert!(note.contains("THINKTHEN_NULL=1"), "the note names the switch");
            unsafe { std::env::set_var("THINKTHEN_NULL", "0") };
            assert_eq!(super::testkit::backend_kind(), None, "=0 is not a backend");
            assert!(super::testkit::skip_note("verbs").is_some(), "=0 still skips visibly");
            unsafe { std::env::set_var("THINKTHEN_NULL", "1") };
            assert_eq!(super::testkit::backend_kind(), Some("null"));
            assert_eq!(super::testkit::skip_note("verbs"), None, "a real backend runs the suite");
            unsafe { std::env::remove_var("THINKTHEN_NULL") };
            unsafe { std::env::set_var("THINKTHEN_BASE_URL", "http://127.0.0.1:9/v1") };
            assert_eq!(super::testkit::backend_kind(), Some("wire"));
        });
    }

    /// Churn through more settings values than there are slots and prove
    /// the evicted states close rather than leaking (review finding 15):
    /// every drop is counted, and the count must reach the number of
    /// evictions once the grace passes and a sweep runs.
    #[test]
    fn retirement_closes_states_rather_than_leaking() {
        use std::sync::atomic::Ordering;
        let before = super::INNER_DROPS.load(Ordering::Relaxed);
        let churn: u64 = 100;
        let options = Options::new();
        for i in 0..churn {
            let settings = super::ResolvedConfig {
                null: true,
                base: format!("http://127.0.0.1:9/{i}"),
                timeout: Duration::from_millis(1),
                max_retries: 0,
                width: 1,
            };
            let _state = super::state(&options, &settings).expect("the state builds");
        }
        // Let the evictions outlive the test grace, then sweep.
        std::thread::sleep(Duration::from_millis(80));
        super::sweep();
        let dropped = super::INNER_DROPS.load(Ordering::Relaxed) - before;
        let evicted = churn.saturating_sub(super::STATE_SLOTS as u64);
        assert!(
            dropped >= evicted,
            "evicted states must close: {dropped} dropped of {evicted} evicted"
        );
    }

    /// A width-1 engine holds exactly one request in flight while another
    /// thread churns the state table (review finding 15): the victim
    /// chooser never evicts a held or busy state while an idle one exists,
    /// so a second state for the same settings — and a second concurrent
    /// send — never happens. The proof reads the wire's own peak, which
    /// no scheduler can blur the way socket-close timing can.
    #[test]
    fn width_one_holds_through_churn() {
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").expect("the loopback binds");
        let port = listener.local_addr().expect("the address reads").port();
        std::thread::spawn(move || {
            // Accept and hold every connection open without answering: a
            // caller's request then occupies its gate for the full timeout.
            for stream in listener.incoming() {
                if stream.is_err() {
                    break;
                }
                std::thread::spawn(move || {
                    let held = stream;
                    std::thread::sleep(Duration::from_secs(30));
                    drop(held);
                });
            }
        });

        // The environment stays out of the way: the null switch off, so
        // the slow engine really reaches the wire and its gate. Nothing
        // else in this binary sends on the wire, so the peak is this
        // engine's alone.
        switches(|| {
            super::WIRE_PEAK.store(0, std::sync::atomic::Ordering::Relaxed);
            let slow = EngineConfig {
                address: Some(format!("http://127.0.0.1:{port}/v1")),
                width: Some(1),
                timeout: Some(Duration::from_millis(250)),
                max_retries: Some(0),
                ..EngineConfig::default()
            };
            let tt = Arc::new(BlockingEngine::from_settings(slow));
            let question = Question::from_json(CUT).expect("parses");

            let churner = std::thread::spawn(|| {
                let options = Options::new();
                for i in 0..300_u16 {
                    let settings = super::ResolvedConfig {
                        null: false,
                        base: format!("http://127.0.0.1:9/{i}"),
                        timeout: Duration::from_millis(1),
                        max_retries: 0,
                        width: 1,
                    };
                    let _state = super::state(&options, &settings).expect("the state builds");
                }
            });
            let callers: Vec<_> = (0..4)
                .map(|caller| {
                    std::thread::spawn({
                        let tt = Arc::clone(&tt);
                        let question = question.clone();
                        move || {
                            // Stagger the starts: the first publish of one
                            // settings value is the only racy moment left,
                            // and the gate serializes everything after it.
                            std::thread::sleep(Duration::from_millis(60 * caller as u64));
                            for _ in 0..4 {
                                // The timeout is the hold: each call occupies
                                // the width-1 gate for its full 250 ms.
                                let _ = tt.decide(&question, "hold this seat");
                            }
                        }
                    })
                })
                .collect();
            // Let the first publish settle: under full-suite load the very
            // first lookup of one settings value can transiently serve one
            // request from a second state before the table converges, and
            // that transient is not the defect this probe pins. Everything
            // after the settle must hold the width exactly.
            std::thread::sleep(Duration::from_millis(700));
            super::WIRE_PEAK.store(0, std::sync::atomic::Ordering::Relaxed);
            for caller in callers {
                caller.join().expect("the caller finishes");
            }
            churner.join().expect("the churner finishes");

            assert_eq!(
                super::WIRE_PEAK.load(std::sync::atomic::Ordering::Relaxed),
                1,
                "width 1 must hold exactly one request on the wire while the table churns"
            );
        });
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
        assert_eq!(rows[0][0].0, "spam", "file order: spam, then kind");
        assert_eq!(rows[0][1].0, "kind");
        // `bug` and `feature` carry no keyword, so the null backend ties
        // them at one half each, and a tied choice reads as unsure — the
        // core's own rule, which the contract keeps.
        assert!(matches!(&rows[0][1].1, Annotated::Choice(None)));

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
    /// marker is the ruled JSON, and the count is one. The fixture is
    /// compiled in only under the `synthetic-partial` feature, the way
    /// `74-annotate-preserves-good-answers` is run.
    #[cfg(feature = "synthetic-partial")]
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
        assert_eq!(rows[0][0].0, "kind", "file order: kind, then topic");
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

    /// Finding 7, kept as a compile-time rule: the environment variable no
    /// longer arms anything, and this default build carries no fixture code
    /// at all. Setting the old variable proves it arms nothing.
    #[cfg(not(feature = "synthetic-partial"))]
    #[test]
    fn the_env_variable_arms_nothing() {
        let _seat = null();
        // Sound in this binary: the seat is held, and the engine value below
        // is the only reader.
        unsafe { std::env::set_var("ENGINE_SYNTHETIC_PARTIAL", "1") };
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
                "{name} answered, not failed, even with the variable set"
            );
        }
        assert_eq!(thinkthen_contract::failed_questions(&rows), 0, "no failed logical question");
        assert_eq!(rows[0][0].1, Annotated::Decision(Answer::Yes), "the fixture text is a refund");
        unsafe { std::env::remove_var("ENGINE_SYNTHETIC_PARTIAL") };
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

    /// A name that does not resolve is not retryable: retrying cannot
    /// change the answer, so the caller fails at once instead of burning
    /// the backoff (review finding, 2026-09-23).
    #[test]
    fn an_unresolvable_name_is_not_retryable() {
        let error = ureq::Error::HostNotFound;
        let classified = super::classify_transport(&error);
        assert!(!classified.retryable, "a DNS failure must not retry");
        assert!(classified.to_string().contains("resolve"), "the refusal names why: {classified}");
    }

    /// A failed connect is not retryable: the same address will fail the
    /// same way within the second.
    #[test]
    fn a_failed_connect_is_not_retryable() {
        let error = ureq::Error::ConnectionFailed;
        let classified = super::classify_transport(&error);
        assert!(!classified.retryable, "a connect failure must not retry");
    }

    /// An I/O error whose cause chain names TLS fails at once: the
    /// handshake is deterministic on retry.
    #[test]
    fn a_tls_failure_is_not_retryable() {
        let inner = std::io::Error::new(std::io::ErrorKind::InvalidData, "rustls: invalid certificate");
        let io = std::io::Error::new(std::io::ErrorKind::InvalidData, inner);
        let error = ureq::Error::Io(io.into());
        let classified = super::classify_transport(&error);
        assert!(!classified.retryable, "a TLS failure must not retry: {classified}");
        assert!(classified.to_string().contains("TLS"), "the refusal names the handshake: {classified}");
    }

    /// A plain mid-flight I/O failure keeps its retry: the backends page
    /// says a transport failure may pass.
    #[test]
    fn a_midflight_io_failure_keeps_its_retry() {
        let io = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "reset by peer");
        let error = ureq::Error::Io(io.into());
        let classified = super::classify_transport(&error);
        assert!(classified.retryable, "a reset mid-flight may pass on retry");
    }

    /// An engine with an explicit address digests against that address,
    /// not the environment's (review finding 15, 2026-09-23): the digest
    /// names the request the engine would actually send.
    #[test]
    fn an_explicit_address_wins_the_digest() {
        let _seat = null();
        let question = Question::from_json(CUT).expect("parses");
        let evidence = "I want my money back";
        let here = super::request_digest_for(
            "http://127.0.0.1:8211/v1",
            &question,
            None,
            evidence,
        )
        .expect("digests");
        let there = super::request_digest_for(
            "http://127.0.0.1:8212/v1",
            &question,
            None,
            evidence,
        )
        .expect("digests");
        assert_ne!(here, there, "two addresses are two requests");
        // The engine's own trail uses its configured base, agreeing with
        // the helper named for that base.
        let mut settings = thinkthen_contract::Settings::default();
        settings.address = Some("http://127.0.0.1:8211/v1".to_owned());
        let tt = BlockingEngine::from_settings(settings);
        let details = tt.details(&question, evidence).expect("details");
        assert_eq!(details.requests[0], here, "the engine's trail names its own address");
    }
}
