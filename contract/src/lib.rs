//! The contract every `thinkthen` surface binds, and both engines meet.
//!
//! ADR 0017 rules the shape: one engine layer under ten surfaces, the same
//! names everywhere, and a move from the stand-in engine to the real engine
//! that changes one dependency. This crate is the written form of that
//! ruling. The stand-in in `standin/` implements the [`Engine`] trait today;
//! the real engine, built in the four merge steps of the ADR, implements it
//! later; a library or a database extension names this crate and never the
//! engine beneath it.
//!
//! What the crate holds:
//!
//! - the [`Engine`] trait: the eight verbs (`decide`, `choose`, `score`,
//!   `tag`, `filter`, `rank`, `find`, `annotate`), `decide_many` as
//!   `decide`'s bulk spelling, `details`, and `usage`;
//! - a [`Question`] built from parts or read from the file grammar, so a
//!   file and keywords give the same value;
//! - the one error shape with six kinds and the retry signal;
//! - the cancel token and the deadline every verb takes beside its
//!   arguments;
//! - the [`Settings`] an engine value carries.
//!
//! Rules every implementation keeps, from ADR 0017:
//!
//! - No failure ever reads as a `No`. A failure raises the host's own error.
//! - A bulk call crosses into the engine once and runs at the engine's
//!   width. `filter` refuses a band, `rank` refuses any threshold, `choose`
//!   takes a cut alone, and `score` and `tag` take none; the conformance
//!   cases carry each rule.
//! - A cancel starts no new request and lets a sent request finish. A
//!   deadline stops the whole call at its instant and returns the deadline's
//!   own kind, which says nothing about the backend's health.
//! - The request limit refuses a bulk call before its first request when the
//!   records outnumber it.
//! - `score` answers the specification's number, the probability-weighted
//!   position from 0 to K−1, and the nearest level's name rides in the
//!   answer beside it.
//! - The counters count sends, so a retried send shows twice and a bill
//!   never meets a tool that shows one; a judgment's `details` carries the
//!   sends that produced it.

use std::path::Path;
use std::str::FromStr;

use serde::Serialize;
use thinkthen_core::question_sha256;
use thinkthen_core::{Question as CoreQuestion, QuestionFile, QuestionFileError, Threshold, Typed, Verb, resolve};

/// The answer to a yes-or-no question, with the public word for "not sure".
///
/// A cut gives `Yes` or `No`. A band gives all three, and `Unsure` is the
/// middle arm. Every host maps `Unsure` to its own empty value: `None` in
/// Python, `null` in TypeScript, `nil` in Ruby, `NA` in R, `NULL` in SQL.
/// The specification keeps "unresolved" in its own grammar and no page mixes
/// the two words.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Answer {
    /// The probability reached the mark.
    Yes,
    /// The probability did not reach the mark.
    No,
    /// The probability sits inside the band.
    Unsure,
}

impl Answer {
    /// The bare value this answer prints: `Some(true)`, `Some(false)`, or
    /// `None` when unsure.
    #[must_use]
    pub const fn value(self) -> Option<bool> {
        match self {
            Self::Yes => Some(true),
            Self::No => Some(false),
            Self::Unsure => None,
        }
    }
}

impl std::fmt::Display for Answer {
    /// The one word the answer goes by.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Unsure => "unsure",
        })
    }
}

/// Why a call failed. No failure ever reads as a `No`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorKind {
    /// The caller asked for something the grammar refuses, or a bulk call
    /// outnumbered the request limit. Nothing was sent.
    Usage,
    /// The wire failed or refused the request.
    Backend,
    /// The call's own deadline passed before it answered. The caller set
    /// this limit, so it says nothing about the backend's health.
    Deadline,
    /// A local file or recording the caller named failed.
    Local,
    /// The caller cancelled the wait. Requests already sent finished.
    Cancelled,
    /// The engine broke its own contract.
    Defect,
}

impl std::fmt::Display for ErrorKind {
    /// The one word the kind goes by.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Usage => "usage",
            Self::Backend => "backend",
            Self::Deadline => "deadline",
            Self::Local => "local",
            Self::Cancelled => "cancelled",
            Self::Defect => "defect",
        })
    }
}

/// One failure, with its kind, a retry signal, and a message that repeats no
/// evidence.
///
/// `retryable` says whether a second try could help, in the tool-search
/// page's sense: a busy or slow backend earns `true`, a refused address or a
/// reply that cannot be trusted earns `false`. A spent deadline keeps `true`
/// with its own meaning: the budget was the caller's, a second try carries a
/// fresh one, and it may answer.
///
/// A deadline's message names the limit and its value — the deadline of
/// 5 s passed — so a reader knows which setting to raise.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("{kind}: {message}")]
pub struct Error {
    /// Which of the six kinds this failure is.
    pub kind: ErrorKind,
    /// Whether a second try could help.
    pub retryable: bool,
    /// What went wrong, in the grammar's own words where it can be.
    pub message: String,
}

impl Error {
    /// A failure the caller caused. Never retryable as asked.
    #[must_use]
    pub fn usage(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Usage, retryable: false, message: message.into() }
    }

    /// A failure a named local file or recording caused.
    #[must_use]
    pub fn local(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Local, retryable: false, message: message.into() }
    }

    /// A failure the engine caused.
    #[must_use]
    pub fn defect(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Defect, retryable: false, message: message.into() }
    }

    /// A backend failure a second try could not fix: a refused address, a
    /// refused request, or a reply that cannot be trusted.
    #[must_use]
    pub fn backend_not_retryable(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Backend, retryable: false, message: message.into() }
    }

    /// A backend failure a second try could help with: busy, slow, or
    /// reset.
    #[must_use]
    pub fn backend_retryable(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Backend, retryable: true, message: message.into() }
    }

    /// A spent deadline, carrying the budget in its message so a reader
    /// knows which setting to raise.
    #[must_use]
    pub fn deadline(seconds: f64) -> Self {
        Self {
            kind: ErrorKind::Deadline,
            retryable: true,
            message: format!("the deadline of {seconds} s passed before the call answered"),
        }
    }

    /// A failure the caller's stop gesture caused.
    #[must_use]
    pub fn cancelled() -> Self {
        Self { kind: ErrorKind::Cancelled, retryable: false, message: "the wait was cancelled".into() }
    }

    /// Stop now when the options say so: a set token first, a passed
    /// deadline second. Engines call this at entry and on every tick.
    ///
    /// # Errors
    ///
    /// Returns the cancelled kind when the token is set, and the deadline's
    /// own kind when the budget is gone.
    pub fn guard(options: &Options<'_>) -> Result<(), Self> {
        if let Some(token) = options.cancel {
            if token.is_cancelled() {
                return Err(Self::cancelled());
            }
        }
        if options.passed() {
            return Err(Self::deadline(options.seconds()));
        }
        Ok(())
    }
}

/// The sentinel a host may send in place of a deadline: the C door's
/// `THINKTHEN_NO_DEADLINE`, meaning the call carries no deadline at all.
///
/// Any other negative value is refused: a host that computed a negative
/// budget meant something the engine cannot honor.
pub const NO_DEADLINE: f64 = -1.0;

/// The largest budget a host may name, in seconds: about 136 years.
///
/// A budget beyond this is refused as too large rather than turned into an
/// instant, because an instant that far out cannot be represented and the
/// arithmetic that builds it would end the host process.
pub const MAX_DEADLINE_SECONDS: f64 = u32::MAX as f64;

/// Convert a host's deadline in floating-point seconds into the budget a
/// call carries.
///
/// `Ok(None)` means no deadline: the caller passed the [`NO_DEADLINE`]
/// sentinel. `Ok(Some(budget))` is the deadline counted from now, and a
/// zero budget is a spent deadline — legal, and the call returns the
/// deadline kind having sent nothing.
///
/// # Errors
///
/// The usage kind for a NaN, a negative value other than the sentinel, or
/// a budget larger than [`MAX_DEADLINE_SECONDS`].
#[must_use = "the budget decides the call"]
pub fn deadline_from_seconds(seconds: f64) -> Result<Option<std::time::Duration>, Error> {
    if seconds == NO_DEADLINE {
        return Ok(None);
    }
    if seconds.is_nan() {
        return Err(Error::usage("the deadline is not a number"));
    }
    if seconds < 0.0 {
        return Err(Error::usage(format!("the deadline of {seconds} s is negative")));
    }
    if seconds > MAX_DEADLINE_SECONDS {
        return Err(Error::usage(format!(
            "the deadline of {seconds} s is larger than the {MAX_DEADLINE_SECONDS} s the engine holds"
        )));
    }
    Ok(Some(std::time::Duration::from_secs_f64(seconds)))
}

/// Convert a host's deadline in floating-point milliseconds, the C door's
/// spelling of [`deadline_from_seconds`].
///
/// The sentinel, the NaN, and the negative rules are the same, with
/// messages naming milliseconds.
///
/// # Errors
///
/// The usage kind for a NaN, a negative value other than the sentinel, or
/// a budget larger than [`MAX_DEADLINE_SECONDS`].
#[must_use = "the budget decides the call"]
pub fn deadline_from_millis(millis: f64) -> Result<Option<std::time::Duration>, Error> {
    if millis == NO_DEADLINE {
        return Ok(None);
    }
    if millis.is_nan() {
        return Err(Error::usage("the deadline is not a number"));
    }
    if millis < 0.0 {
        return Err(Error::usage(format!("the deadline of {millis} ms is negative")));
    }
    if millis > MAX_DEADLINE_SECONDS * 1000.0 {
        return Err(Error::usage(format!(
            "the deadline of {millis} ms is larger than the {MAX_DEADLINE_SECONDS} s the engine holds"
        )));
    }
    Ok(Some(std::time::Duration::from_secs_f64(millis / 1000.0)))
}

/// Run one host-facing boundary, turning a panic into the defect error.
///
/// Every surface that can meet a host process puts its foreign-function
/// body behind this helper: a panic from anywhere beneath the body — the
/// engine, the grammar, a poisoned lock — comes back as one error of the
/// defect kind instead of unwinding into a host that was never built to
/// catch it. Rust's own panic hook still prints the panic to stderr, so the
/// crash is still visible; what the boundary removes is the host process's
/// death.
///
/// The message names the boundary and the panic's own text, so a host log
/// says which door failed.
pub fn catch_panic<T>(boundary: &str, body: impl FnOnce() -> Result<T, Error>) -> Result<T, Error> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(body)) {
        Ok(result) => result,
        Err(payload) => Err(Error::defect(format!(
            "a panic crossed {boundary}: {}",
            panic_text(payload.as_ref())
        ))),
    }
}

/// The text a panic payload carries: the message for a string panic, and a
/// fixed phrase for a payload that is not text.
///
/// The formatter lives here so every surface's boundary spells the same
/// message, and the panic's own words never vanish.
#[must_use]
pub fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(text) = payload.downcast_ref::<&'static str>() {
        (*text).to_owned()
    } else if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else {
        "a payload that is not text".to_owned()
    }
}

/// A cancellation token, set from any thread, checked between requests and
/// on every tick of a wait.
///
/// The promise is exact and no larger: no new request starts after a cancel,
/// and requests already sent finish. A host's stop gesture — Python's
/// signals, TypeScript's `AbortSignal`, PostgreSQL's cancel — sets this
/// token through the poll callback the engine calls on the calling thread.
#[derive(Clone, Default)]
pub struct Cancel(std::sync::Arc<std::sync::atomic::AtomicBool>);

impl Cancel {
    /// A token nothing has set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ask every wait that sees this token to stop.
    pub fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Whether `cancel` has been called.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl std::fmt::Debug for Cancel {
    /// Show only whether the token is set, never the pointer inside.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("Cancel").field("set", &self.is_cancelled()).finish()
    }
}

/// What one call carries beside its question: a cancel token and a deadline.
///
/// A deadline is an instant, so it bounds the whole call: the queue at the
/// width gate, the retries, and one blocking send all count against it. When
/// it passes mid-batch, no new request starts, sent requests finish, and the
/// call returns the deadline's own kind.
///
/// A budget of zero, or any deadline already at or past the current
/// instant, is legal: the deadline is spent before the call starts,
/// nothing is sent, and the call returns the deadline kind with the
/// message naming the budget. No surface refuses a spent deadline.
#[derive(Clone, Copy, Default)]
pub struct Options<'a> {
    cancel: Option<&'a Cancel>,
    deadline: Option<std::time::Instant>,
    seconds: f64,
}

impl<'a> Options<'a> {
    /// Neither a token nor a deadline.
    #[must_use]
    pub const fn new() -> Self {
        Self { cancel: None, deadline: None, seconds: 0.0 }
    }

    /// Carry this token, checked between requests and on every tick.
    #[must_use]
    pub const fn cancel(mut self, token: &'a Cancel) -> Self {
        self.cancel = Some(token);
        self
    }

    /// Carry a token that may be absent, for callers that hold an `Option`.
    #[must_use]
    pub const fn maybe_cancel(mut self, token: Option<&'a Cancel>) -> Self {
        self.cancel = token;
        self
    }

    /// Stop the whole call at this instant.
    #[must_use]
    pub const fn deadline(mut self, at: std::time::Instant) -> Self {
        self.deadline = Some(at);
        self
    }

    /// Stop the whole call after this budget, counted from now.
    ///
    /// A zero budget is spent immediately: the call sends nothing and
    /// returns the deadline kind.
    #[must_use]
    pub fn deadline_in(self, budget: std::time::Duration) -> Self {
        let seconds = budget.as_secs_f64();
        Self { deadline: Some(std::time::Instant::now() + budget), seconds, ..self }
    }

    /// Whether the deadline has passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        matches!(self.deadline, Some(at) if std::time::Instant::now() >= at)
    }

    /// The budget the message names, when one was set.
    #[must_use]
    pub fn seconds(&self) -> f64 {
        self.seconds
    }

    /// The token this call carries, when one was given. Engines read it
    /// after a batch wait to report a cancel that landed mid-flight.
    #[must_use]
    pub const fn cancel_token(&self) -> Option<&Cancel> {
        self.cancel
    }

    /// The time left before the deadline, when one was set.
    #[must_use]
    pub fn remaining(&self) -> Option<std::time::Duration> {
        self.deadline.map(|at| at.saturating_duration_since(std::time::Instant::now()))
    }

    /// Carry a host's deadline in floating-point seconds, checked by
    /// [`deadline_from_seconds`]: `None` and the sentinel both mean no
    /// deadline.
    ///
    /// This is the one conversion every host door calls, so a NaN, a
    /// negative, or an oversized budget comes back as the usage kind
    /// instead of a panic inside the host process.
    ///
    /// # Errors
    ///
    /// The usage kind for a NaN, a negative value other than the sentinel,
    /// or a budget larger than [`MAX_DEADLINE_SECONDS`].
    pub fn with_deadline_seconds(self, seconds: Option<f64>) -> Result<Self, Error> {
        match seconds.map(deadline_from_seconds).transpose()? {
            Some(Some(budget)) => Ok(self.deadline_in(budget)),
            _ => Ok(self),
        }
    }

    /// Carry a host's deadline in floating-point milliseconds, the C
    /// door's spelling of [`Options::with_deadline_seconds`].
    ///
    /// # Errors
    ///
    /// The usage kind for a NaN, a negative value other than the sentinel,
    /// or a budget larger than [`MAX_DEADLINE_SECONDS`].
    pub fn with_deadline_millis(self, millis: Option<f64>) -> Result<Self, Error> {
        match millis.map(deadline_from_millis).transpose()? {
            Some(Some(budget)) => Ok(self.deadline_in(budget)),
            _ => Ok(self),
        }
    }
}

/// One judgment: the probability and what the question's rule made of it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Judgment {
    /// The probability the backend gave the yes side.
    pub probability: f64,
    /// Yes, no, or unsure under the question's rule.
    pub answer: Answer,
}

impl Judgment {
    /// The bare value: `Some(true)`, `Some(false)`, or `None` when unsure.
    #[must_use]
    pub const fn value(&self) -> Option<bool> {
        self.answer.value()
    }
}

/// What `score` returned: the specification's number and the nearest level.
///
/// The number is the probability-weighted position on the named levels,
/// from 0 to K−1: with probabilities 0.20, 0.55, 0.25 over three levels the
/// answer is 1.05. The nearest level is the one with the highest
/// probability, and it rides in the answer beside the number.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Scored {
    /// The position on the levels, from 0 to K−1.
    pub value: f64,
    /// The name of the nearest level.
    pub nearest: String,
}

/// What `details` returned: the judgment plus its audit trail.
///
/// `sends` counts the wire sends that produced this judgment. One is the
/// norm; two means the connection died after the request left and the send
/// repeated, and the caller sees what the bill sees.
///
/// `requests` is the ordered recording digests of the logical requests
/// that produced the judgment, per ticket 0053: always an array, one
/// element for a one-request result, in logical construction order, and a
/// retry adds no element. The digests name the same files `--record` and
/// `--cache` use. No singular form exists.
///
/// `failed_questions` is always present, including zero, per ticket 0054.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Details {
    /// The probability the backend gave the yes side.
    ///
    /// A score question has no yes side: its details read the probability
    /// of the nearest level, and the position `score` returns is that
    /// verb's own answer, not repeated here.
    pub probability: f64,
    /// Yes, no, or unsure under the question's rule.
    ///
    /// A score question reads as `Yes` when it resolved, because the core
    /// reports a score's read as its own `Outcome::Yes` and a score has no
    /// threshold to apply. The answer a caller wants from a score question
    /// is the position, with the nearest level's name in
    /// [`Details::nearest`].
    pub answer: Answer,
    /// The nearest level's name, for a score question; `None` for every
    /// other verb, serialized as null.
    ///
    /// This is ADR 0017 pick 6: the level's name rides in `details` on
    /// every door, so no surface carries a private field for it. The
    /// nearest level is the one with the highest probability among the
    /// question's own levels.
    pub nearest: Option<String>,
    /// The model the request named.
    pub model: String,
    /// The digest of the question with its threshold, 64 hex figures.
    pub digest: String,
    /// The wire sends that produced this judgment.
    pub sends: u32,
    /// The recording digests of the logical requests, in construction
    /// order. One element for one request; a retry adds no element (0053).
    pub requests: Vec<String>,
    /// Failed logical questions in this result; zero here by construction,
    /// because a failed single question is a whole-call error (0054).
    pub failed_questions: u32,
}

/// The process counters `usage` reports.
///
/// `requests` counts sends, not calls, so a retried send counts twice: the
/// bill pays for both. `cache_answers` counts answers a cache gave without
/// a send, and `tokens` carries the usage the replies reported.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
pub struct Usage {
    /// Wire sends since the last reset, retried sends included.
    pub requests: u64,
    /// Answers a cache gave with no send at all.
    pub cache_answers: u64,
    /// Tokens the replies reported, when the engine counts them.
    pub tokens: u64,
}

/// One record of a `rank` answer: where the record placed and its
/// probability.
///
/// Most likely yes first, and ties keep input order.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Ranked {
    /// The record's place in the input.
    pub index: usize,
    /// The probability the backend gave the yes side.
    pub probability: f64,
}

/// What `find` returned: the unit that best answers the question.
///
/// The answer is relative: the units see each other in one request, and a
/// job that needs each unit judged alone uses `filter` or `rank`. `None`
/// means the question's `none` arm won, when the question carries one.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Found {
    /// The winning unit's place in the input, or `None` when nothing fits.
    pub index: Option<usize>,
    /// The probability the backend gave the winner.
    pub probability: f64,
}

/// One field of an `annotate` answer, typed by its question's verb.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Annotated {
    /// A `decide` question: yes, no, or unsure.
    Decision(Answer),
    /// A `choose` question: the winning label, or `None` when unresolved.
    Choice(Option<String>),
    /// A `score` question: the position and the nearest level.
    Score(Scored),
    /// A `tag` question: the labels that held, in the question's order.
    Tags(Vec<String>),
    /// The logical question failed while the reply answered a neighbour.
    ///
    /// Serializes as the ruled marker `{"failed":{"kind":"backend",
    /// "cause":CAUSE}}` (0054). A run that prints one of these markers
    /// completes its input and exits 6; the marker, not a diagnostic,
    /// identifies the failure.
    #[serde(rename = "failed")]
    Failed(Failed),
}

/// Why one logical question failed while its neighbours answered (0054).
///
/// The list is closed: missing answer, wrong answer kind, missing or
/// out-of-range probability, an invalid distribution total, and an
/// unexpected option or level.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Cause {
    /// The response omitted the wire answer.
    MissingAnswer,
    /// The response used a shape the question did not ask for.
    WrongKind,
    /// An option or level has no probability.
    MissingProbability,
    /// A reported probability falls outside zero to one.
    InvalidProbability,
    /// A distribution does not total one within the adapter tolerance.
    InvalidDistribution,
    /// A distribution contains an option or level that was not sent.
    UnexpectedProbability,
}

/// The kind a failed logical question carries; backend, today, always.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    /// The wire failed this logical question while the request succeeded.
    Backend,
}

/// One failed logical question's marker (0054).
///
/// It serializes as `{"kind":"backend","cause":CAUSE}` inside the
/// `failed` object, and `null` still means `not sure` and never failed.
///
/// At the merge this mirrors the core's `BackendFailure`; the build team
/// reconciles the two spellings in one place.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Failed {
    /// The failure's kind; [`FailureKind::Backend`] today.
    pub kind: FailureKind,
    /// The closed cause.
    pub cause: Cause,
}

/// Count the failed logical questions across annotate rows (0054 rule 3).
///
/// One failed field is one failed logical question, so a `tag` whose wire
/// answers failed counts once through its single field.
#[must_use]
pub fn failed_questions(rows: &[AnnotatedRecord]) -> u32 {
    u32::try_from(
        rows.iter()
            .flat_map(|row| row.iter())
            .filter(|(_, field)| matches!(field, Annotated::Failed(_)))
            .count(),
    )
    .unwrap_or(u32::MAX)
}

/// One record's `annotate` answer: one field per question, in the set's
/// name order.
pub type AnnotatedRecord = Vec<(String, Annotated)>;

/// One end of a relation rule: a named kind or any kind.
///
/// A question file writes `"*"` for the any kind and every string surface
/// writes it the same way; the Rust builders take [`Kind::Any`] itself. One
/// parser in the core checks the rule, and no binding checks it again. A
/// named end must be one of the caller's kinds; the check is
/// [`RelationRule::check_kinds`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Any kind.
    Any,
    /// One kind, by the user's own word.
    Named(String),
}

impl Kind {
    /// One kind, by the user's own word.
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self::Named(name.into())
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => formatter.write_str("*"),
            Self::Named(name) => formatter.write_str(name),
        }
    }
}

/// One relation rule: the name, the kind it comes from, the kind it goes
/// to, and whether it reads the same both ways.
///
/// The answer carries the rule's own name. A one-way rule reads from the
/// left of the colon to the right; `either` asks a both-ways relation once
/// per pair. A blank name or a blank named end is a usage error, and an end
/// that names a kind the call did not ask for is a usage error naming that
/// kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationRule {
    /// The relation's own name; the answer carries this word.
    pub name: String,
    /// The kind the relation comes from.
    pub from: Kind,
    /// The kind the relation goes to.
    pub to: Kind,
    /// Ask once per pair, both ways.
    pub either: bool,
}

impl RelationRule {
    /// Build one rule; a blank name or a blank named end is a usage error.
    ///
    /// # Errors
    ///
    /// The usage kind when the name or a named end is blank.
    pub fn new(name: &str, from: Kind, to: Kind) -> Result<Self, Error> {
        if name.trim().is_empty() {
            return Err(Error::usage("a relation rule has a blank name"));
        }
        for end in [&from, &to] {
            if let Kind::Named(kind) = end {
                if kind.trim().is_empty() {
                    return Err(Error::usage(format!(
                        "the relation rule {name} has a missing end"
                    )));
                }
            }
        }
        Ok(Self { name: name.to_owned(), from, to, either: false })
    }

    /// Ask this rule once per pair with both directions as one question.
    #[must_use]
    pub fn either(mut self, yes: bool) -> Self {
        self.either = yes;
        self
    }

    /// Check every named end against the kinds a call carries.
    ///
    /// # Errors
    ///
    /// The usage kind naming the first end that is not among the kinds.
    pub fn check_kinds(&self, kinds: &[String]) -> Result<(), Error> {
        for end in [&self.from, &self.to] {
            if let Kind::Named(kind) = end {
                if !kinds.iter().any(|asked| asked == kind) {
                    return Err(Error::usage(format!(
                        "the relation rule {} names the kind {kind}, which is not among the asked kinds",
                        self.name
                    )));
                }
            }
        }
        Ok(())
    }
}

/// Check a threshold against its fraction range, zero included because
/// `--threshold 0` returns every candidate with its number.
fn check_threshold(what: &str, threshold: f64) -> Result<(), Error> {
    if (0.0..=1.0).contains(&threshold) {
        Ok(())
    } else {
        Err(Error::usage(format!("the {what} {threshold} is outside 0 to 1")))
    }
}

/// The kinds a recognize call asks about when none are named.
fn default_kinds() -> Vec<String> {
    ["person", "organization", "place"].map(str::to_owned).to_vec()
}

/// What `recognize` looks for, and which relations it may find.
///
/// The defaults are the three kinds `person`, `organization`, and `place`,
/// no relation rules, and a 0.5 bar on names and relations. Naming kinds
/// replaces the defaults; the command's own rule is one to twenty kinds.
#[derive(Clone, Debug, PartialEq)]
pub struct Recognize {
    /// The kinds to look for, in the user's own words.
    pub kinds: Vec<String>,
    /// The relation rules that turn relations on.
    pub relations: Vec<RelationRule>,
    /// The bar a name must reach.
    pub threshold: f64,
    /// The bar a relation must reach.
    pub relation_threshold: f64,
}

impl Default for Recognize {
    fn default() -> Self {
        Self::new()
    }
}

impl Recognize {
    /// The three default kinds, no rules, both bars at 0.5.
    #[must_use]
    pub fn new() -> Self {
        Self {
            kinds: default_kinds(),
            relations: Vec::new(),
            threshold: 0.5,
            relation_threshold: 0.5,
        }
    }

    /// Name the kinds to look for; an empty list keeps the defaults.
    #[must_use]
    pub fn kinds<I, S>(mut self, kinds: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let kinds: Vec<String> = kinds.into_iter().map(Into::into).collect();
        if !kinds.is_empty() {
            self.kinds = kinds;
        }
        self
    }

    /// Add one one-way relation rule.
    ///
    /// # Errors
    ///
    /// The usage kind when the rule's name or a named end is blank.
    pub fn relation(mut self, name: &str, from: Kind, to: Kind) -> Result<Self, Error> {
        self.relations.push(RelationRule::new(name, from, to)?);
        Ok(self)
    }

    /// Set the bar a name must reach, 0 to 1.
    ///
    /// # Errors
    ///
    /// The usage kind when the threshold is outside 0 to 1.
    pub fn threshold(mut self, threshold: f64) -> Result<Self, Error> {
        check_threshold("threshold", threshold)?;
        self.threshold = threshold;
        Ok(self)
    }

    /// Set the bar a relation must reach, 0 to 1.
    ///
    /// # Errors
    ///
    /// The usage kind when the threshold is outside 0 to 1.
    pub fn relation_threshold(mut self, threshold: f64) -> Result<Self, Error> {
        check_threshold("relation threshold", threshold)?;
        self.relation_threshold = threshold;
        Ok(self)
    }

    /// Read the question file's `recognize` section: kinds as an object of
    /// descriptions or a list of names, relations with `source`, `target`,
    /// and `either`, and both thresholds. A `reads` phrase is accepted and
    /// carried by no field here; it is the model-facing wording. The old
    /// `from`/`to` spelling is refused with the ruled spelling named.
    ///
    /// # Errors
    ///
    /// The usage kind for a shape the grammar refuses, a blank or missing
    /// end, more than twenty kinds, or a named end outside the kinds.
    pub fn from_json(spec: &str) -> Result<Self, Error> {
        let value: serde_json::Value = serde_json::from_str(spec).map_err(|error| {
            Error::usage(format!("the recognize spec is not JSON: {error}"))
        })?;
        let object = value
            .as_object()
            .ok_or_else(|| Error::usage("the recognize spec is not a JSON object"))?;
        let mut ask = Self::new();
        if let Some(kinds) = object.get("kinds") {
            let names: Vec<String> = match kinds {
                serde_json::Value::Object(map) => map.keys().cloned().collect(),
                serde_json::Value::Array(items) => items
                    .iter()
                    .map(|item| {
                        item.as_str().map(str::to_owned).ok_or_else(|| {
                            Error::usage("a kind in the spec is not a string")
                        })
                    })
                    .collect::<Result<Vec<String>, Error>>()?,
                _ => return Err(Error::usage("the spec's kinds are not an object or a list")),
            };
            if names.len() > 20 {
                return Err(Error::usage(format!(
                    "the spec carries {} kinds, and 20 is the limit",
                    names.len()
                )));
            }
            ask = ask.kinds(names);
        }
        if let Some(relations) = object.get("relations") {
            let entries = relations
                .as_array()
                .ok_or_else(|| Error::usage("the spec's relations are not a list"))?;
            for entry in entries {
                let rule = relation_from_value(entry)?;
                ask.relations.push(rule);
            }
        }
        if let Some(threshold) = object.get("threshold") {
            let threshold = threshold
                .as_f64()
                .ok_or_else(|| Error::usage("the spec's threshold is not a number"))?;
            ask = ask.threshold(threshold)?;
        }
        if let Some(threshold) = object.get("relation_threshold") {
            let threshold = threshold
                .as_f64()
                .ok_or_else(|| Error::usage("the spec's relation_threshold is not a number"))?;
            ask = ask.relation_threshold(threshold)?;
        }
        for rule in &ask.relations {
            rule.check_kinds(&ask.kinds)?;
        }
        Ok(ask)
    }
}

/// Read one relation rule from a JSON value: `source` and `target` required,
/// `*` meaning any kind, `either` optional. The old `from`/`to` spelling is
/// refused with the ruled spelling named: one spelling on every door, and
/// nothing has shipped that an alias would need to keep.
fn relation_from_value(entry: &serde_json::Value) -> Result<RelationRule, Error> {
    let object = entry
        .as_object()
        .ok_or_else(|| Error::usage("a relation rule is not a JSON object"))?;
    let name = object
        .get("name")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| Error::usage("a relation rule has no name"))?;
    if object.contains_key("from") || object.contains_key("to") {
        return Err(Error::usage(format!(
            "the relation rule {name} uses from/to; the ruled spelling is source and target"
        )));
    }
    let end = |key: &str| -> Result<Kind, Error> {
        let raw = object
            .get(key)
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| Error::usage(format!("the relation rule {name} has no {key} end")))?;
        if raw.is_empty() {
            return Err(Error::usage(format!(
                "the relation rule {name} has a missing end"
            )));
        }
        Ok(if raw == "*" { Kind::Any } else { Kind::named(raw) })
    };
    let rule = RelationRule::new(name, end("source")?, end("target")?)?;
    let either = object.get("either").and_then(serde_json::Value::as_bool).unwrap_or(false);
    Ok(rule.either(either))
}

/// What `relate` asks over records: the relation rules, where a record's
/// kind sits, and the bar an edge must reach.
///
/// The recordings behind the stand-in carry no kinds, so a named end is a
/// usage error there; the real engine takes the kind field pointer. The
/// threshold defaults to 0.5.
#[derive(Clone, Debug, PartialEq)]
pub struct Relate {
    /// The rules to ask; a name alone means the any kind to the any kind.
    pub relations: Vec<RelationRule>,
    /// The JSON pointer to each record's kind, when the records carry one.
    pub kind_field: Option<String>,
    /// The bar an edge must reach.
    pub threshold: f64,
}

impl Default for Relate {
    fn default() -> Self {
        Self::new()
    }
}

impl Relate {
    /// No rules yet, no kind field, the bar at 0.5.
    #[must_use]
    pub fn new() -> Self {
        Self { relations: Vec::new(), kind_field: None, threshold: 0.5 }
    }

    /// Add one one-way relation rule.
    ///
    /// # Errors
    ///
    /// The usage kind when the rule's name or a named end is blank.
    pub fn relation(mut self, name: &str, from: Kind, to: Kind) -> Result<Self, Error> {
        self.relations.push(RelationRule::new(name, from, to)?);
        Ok(self)
    }

    /// Add one both-ways rule between the same kind at both ends.
    ///
    /// # Errors
    ///
    /// The usage kind when the rule's name is blank.
    pub fn either(mut self, name: &str, kind: Kind) -> Result<Self, Error> {
        self.relations
            .push(RelationRule::new(name, kind.clone(), kind)?.either(true));
        Ok(self)
    }

    /// Name the JSON pointer where each record keeps its kind.
    #[must_use]
    pub fn kind_field(mut self, pointer: impl Into<String>) -> Self {
        self.kind_field = Some(pointer.into());
        self
    }

    /// Set the bar an edge must reach, 0 to 1.
    ///
    /// # Errors
    ///
    /// The usage kind when the threshold is outside 0 to 1.
    pub fn threshold(mut self, threshold: f64) -> Result<Self, Error> {
        check_threshold("threshold", threshold)?;
        self.threshold = threshold;
        Ok(self)
    }

    /// Read the question file's `relate` section: `relations` entries in
    /// the same shape as `recognize`'s or as bare names, an `either` list of
    /// bare names, an optional `kind_field`, and the threshold.
    ///
    /// # Errors
    ///
    /// The usage kind for a shape the grammar refuses.
    pub fn from_json(spec: &str) -> Result<Self, Error> {
        let value: serde_json::Value = serde_json::from_str(spec).map_err(|error| {
            Error::usage(format!("the relate spec is not JSON: {error}"))
        })?;
        let object = value
            .as_object()
            .ok_or_else(|| Error::usage("the relate spec is not a JSON object"))?;
        let mut ask = Self::new();
        if let Some(relations) = object.get("relations") {
            let entries = relations
                .as_array()
                .ok_or_else(|| Error::usage("the spec's relations are not a list"))?;
            for entry in entries {
                match entry {
                    serde_json::Value::String(name) => ask
                        .relations
                        .push(RelationRule::new(name, Kind::Any, Kind::Any)?),
                    _ => ask.relations.push(relation_from_value(entry)?),
                }
            }
        }
        if let Some(either) = object.get("either") {
            let names = either
                .as_array()
                .ok_or_else(|| Error::usage("the spec's either is not a list"))?;
            for name in names {
                let name = name
                    .as_str()
                    .ok_or_else(|| Error::usage("an either entry is not a string"))?;
                ask.relations
                    .push(RelationRule::new(name, Kind::Any, Kind::Any)?.either(true));
            }
        }
        if let Some(pointer) = object.get("kind_field") {
            let pointer = pointer
                .as_str()
                .ok_or_else(|| Error::usage("the spec's kind_field is not a string"))?;
            ask.kind_field = Some(pointer.to_owned());
        }
        if let Some(threshold) = object.get("threshold") {
            let threshold = threshold
                .as_f64()
                .ok_or_else(|| Error::usage("the spec's threshold is not a number"))?;
            ask = ask.threshold(threshold)?;
        }
        Ok(ask)
    }
}

/// One name `recognize` found: the user's own kind word and where the name
/// sits in the text the user gave.
///
/// `start` and `end` count code points of the text, so `text[start:end]` is
/// the name in a host whose indexing is code points. Every other host
/// converts once: JavaScript to UTF-16 units, Rust and C to bytes. The
/// number field is the interim name for the number on a name: the
/// marketing vocabulary page (the product vocabulary page,
/// "The words for numbers") restricts `confidence` to the literal field Jev
/// returns in detailed output, and this number is computed from several of
/// Jev's numbers, so it may not carry that word; the recognize team's free
/// comparison — whether the lowest Jev probability behind a name filters as
/// well as our computed number — settles the final name.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Entity {
    /// The name's id within the answer; relations point at it.
    pub id: u64,
    /// The name's text, sliced from the original exactly.
    pub text: String,
    /// The user's own kind word; never a code such as `PER`.
    pub kind: String,
    /// The first code point of the name in the original text.
    pub start: usize,
    /// One past the last code point of the name.
    pub end: usize,
    /// The strength of a name: a number we compute — the least of the word
    /// probabilities times the mean of the kind probabilities — defined once
    /// in the manual, with its parts under details, per
    /// `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md`.
    /// Settled by Ian on 2026-09-21: the field is `strength`. The vendor's
    /// own `confidence` passes through under details only, under its own
    /// name, and nothing gates on it.
    pub strength: f64,
}

/// One relation between two names, by entity id.
///
/// The ruled name for the number on a relation is `probability`, and the
/// spelling of the ends is `source` and `target` on every surface, C's
/// returned JSON included, the command's own JSON, and the question file —
/// no door converts anything. The harvest recordings still say `from` and
/// `to`; the stand-in maps them at replay.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Relation {
    /// The rule's own name.
    pub name: String,
    /// The id of the name the relation comes from.
    pub source: u64,
    /// The id of the name the relation goes to.
    pub target: u64,
    /// The probability of the picked relation.
    pub probability: f64,
}

/// What `recognize` returned: the names, and the relations when a rule was
/// given.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Recognized {
    /// The names found, in text order.
    pub entities: Vec<Entity>,
    /// The relations found; empty when no rule was given or none was found.
    pub relations: Vec<Relation>,
}

impl Recognized {
    /// The answer as one JSON string, the shape the C door returns and
    /// every other host parses into its own records.
    #[must_use]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("a recognized answer is JSON-clean")
    }
}

/// One edge `relate` found, by record numbers counted from 1 in input
/// order. `source` is the subject and `target` the object; an `either` edge
/// prints once with the lower number in `source`.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Edge {
    /// The rule's own name.
    pub name: String,
    /// The record the edge comes from, counted from 1.
    pub source: u64,
    /// The record the edge goes to, counted from 1.
    pub target: u64,
    /// The probability of the picked relation.
    pub probability: f64,
    /// The source record's kind, when the rule names one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kind: Option<String>,
    /// The target record's kind, when the rule names one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_kind: Option<String>,
}

/// The edges as one JSON object, `{"edges": [...]}`, the shape the C door
/// returns for `relate`.
#[must_use]
pub fn edges_json(edges: &[Edge]) -> String {
    #[derive(Serialize)]
    struct Edges<'a> {
        edges: &'a [Edge],
    }
    serde_json::to_string(&Edges { edges }).expect("an edge list is JSON-clean")
}

/// One record with its answer: the ruled record-mode row.
///
/// The ruling (`sdlc/planning/go-ahead-for-the-build-team-2026-09-21.md`,
/// item 4) makes `{"input","value"}` rows the default where records flow
/// with answers — the value-printing bulk forms and `filter`. `annotate`
/// keeps its enrichment of objects and does not use this row. The input is
/// the caller's own record, unchanged; the value is the answer its verdict
/// carries.
///
/// The row is a host-side rendering, not an engine return value: the
/// library verbs hand back their own values (judgments, kept records,
/// ranked pairs), and a host that pairs answers with the caller's records
/// renders them in this shape. No surface must emit the row for the
/// engine's sake; the SQL forms get the same shape from their own two
/// columns.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Row<V> {
    /// The caller's own record, unchanged.
    pub input: String,
    /// The record's answer.
    pub value: V,
}

/// The ruled row list as JSON, for a host door: `[{"input","value"}...]`.
#[must_use]
pub fn rows_json<V: Serialize>(rows: &[Row<V>]) -> String {
    serde_json::to_string(rows).expect("a row list is JSON-clean")
}

/// The `find` limit, reused for `relate`: one call takes at most 255
/// records, and the refusal is a usage error naming the limit.
pub const MAX_RELATE_RECORDS: usize = 255;

/// Refuse a relate call that outnumbers its record limit, before anything
/// else happens.
///
/// # Errors
///
/// The usage kind when `count` is past [`MAX_RELATE_RECORDS`].
pub fn guard_relate_records(count: usize) -> Result<(), Error> {
    if count > MAX_RELATE_RECORDS {
        Err(Error::usage(format!(
            "relate takes at most {MAX_RELATE_RECORDS} records and {count} came"
        )))
    } else {
        Ok(())
    }
}

/// Call `relate` through the contract: the record limit is checked here, so
/// every surface that enters through this function inherits it.
///
/// # Errors
///
/// The usage kind past the record limit, then the same kinds as
/// [`Engine::relate_opts`].
pub fn relate_checked(
    engine: &dyn Engine,
    ask: &Relate,
    records: &[&str],
    options: Options<'_>,
) -> Result<Vec<Edge>, Error> {
    guard_relate_records(records.len())?;
    engine.relate_opts(ask, records, options)
}

/// The settled settings an engine value carries, with one spelling each.
///
/// The defaults come from ADR 0017's settings table: the built-in address,
/// no key, the model alias, a width of 4, no request limit, the XDG cache
/// home, and a 100 MB cache cap. A setting the host passes always wins over
/// the environment, and a folder the user names always wins over the cache
/// home.
///
/// [`Settings`] is this struct's older name and stays an alias, so a host
/// written against either name compiles.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EngineConfig {
    /// The backend address, overriding `THINKTHEN_BASE_URL`.
    pub address: Option<String>,
    /// The model, overriding the alias and the question's own name.
    pub model: Option<String>,
    /// How many requests one process has in flight at once. Each
    /// in-flight request holds its own connection: 1,000 records at
    /// width 32 measured 33 pooled connections.
    pub width: Option<usize>,
    /// The request limit: a bulk call with more records is refused before
    /// its first request.
    pub max_requests: Option<u64>,
    /// The cache folder, overriding `THINKTHEN_CACHE` and the XDG cache
    /// home. `None` leaves the engine's own default in place.
    pub cache: Option<std::path::PathBuf>,
    /// The cache cap in bytes, overriding the 100 MB default.
    pub cache_bytes: Option<u64>,
    /// How long one request may wait on the wire, overriding the
    /// connector's own default.
    pub timeout: Option<std::time::Duration>,
    /// How many times one failed request is sent again, overriding the
    /// connector's own default.
    pub max_retries: Option<u32>,
}

/// The settings a host passes under their older name; the same struct as
/// [`EngineConfig`].
pub type Settings = EngineConfig;

impl EngineConfig {
    /// Read the two environment variables that carry settings, empty or
    /// missing reading as `None`.
    ///
    /// `THINKTHEN_BASE_URL` sets the address and `THINKTHEN_CACHE` names a
    /// cache folder. The key variable is read by the engine at send time and
    /// never stored here.
    #[must_use]
    pub fn from_env() -> Self {
        let read = |name: &str| {
            std::env::var(name).ok().filter(|value| !value.trim().is_empty())
        };
        Self {
            address: read("THINKTHEN_BASE_URL"),
            cache: read("THINKTHEN_CACHE").map(std::path::PathBuf::from),
            ..Self::default()
        }
    }
}

/// Which verb a question holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestionKind {
    /// A yes-or-no question under a cut or a band.
    Decide,
    /// A pick from two to 255 named options.
    Choose,
    /// A placement on two to ten named levels.
    Score,
    /// Any number of labels from one to twenty, judged apart.
    Tag,
}

impl std::fmt::Display for QuestionKind {
    /// The verb's own word.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Decide => "decide",
            Self::Choose => "choose",
            Self::Score => "score",
            Self::Tag => "tag",
        })
    }
}

/// A question, built from parts or read from the file's grammar, asked many
/// times.
///
/// Building from parts and loading a file give the same value: the builder
/// writes the file's own JSON and reads it back through the one grammar, so
/// no second parser exists to drift. The digest is the core's own, over the
/// question and its threshold.
///
/// A question carries its members once. When a caller hands over a built
/// question and also names its members — `options`, `labels`, or `levels` —
/// the pair is ambiguous, and the call refuses with a usage error naming
/// both rather than guessing which one wins.
#[derive(Clone, Debug)]
pub struct Question {
    asked: CoreQuestion,
    kind: QuestionKind,
    threshold: Option<Threshold>,
    threshold_named: bool,
    model: String,
    members: Vec<String>,
}

impl Question {
    /// A `decide` question with no threshold, which the caller adds with
    /// [`DecideBuilder::cut`] or [`DecideBuilder::band`].
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the text is blank or breaks
    /// the grammar.
    pub fn decide(text: &str) -> Result<DecideBuilder, Error> {
        Self::check_text(text)?;
        Ok(DecideBuilder { text: text.to_owned(), model: None })
    }

    /// A `choose` question over named options, two to 255 of them.
    ///
    /// The winner is the option with the highest probability, returned when
    /// no cut was set or when the winner reaches the cut.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the text is blank or the
    /// options break the rule.
    pub fn choose(text: &str, options: &[&str]) -> Result<ChoiceBuilder, Error> {
        Self::check_text(text)?;
        Self::check_members(options, 2, 255, "options")?;
        Ok(ChoiceBuilder {
            text: text.to_owned(),
            options: options.iter().map(|option| (*option).to_owned()).collect(),
            model: None,
            cut: None,
        })
    }

    /// A `score` question over named levels, two to ten, lowest first.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the text is blank or the
    /// levels break the rule.
    pub fn score(text: &str, levels: &[&str]) -> Result<Question, Error> {
        Self::check_text(text)?;
        Self::check_members(levels, 2, 10, "levels")?;
        Self::from_built(
            Verb::Score,
            serde_json::json!({"score": text, "levels": levels}),
        )
    }

    /// A `tag` question over named labels, one to twenty.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the text is blank or the
    /// labels break the rule.
    pub fn tag(text: &str, labels: &[&str]) -> Result<Question, Error> {
        Self::check_text(text)?;
        Self::check_members(labels, 1, 20, "labels")?;
        Self::from_built(Verb::Tag, serde_json::json!({"tag": text, "labels": labels}))
    }

    /// Read a question from the file's grammar, any verb.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the text is not one question
    /// the grammar accepts.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let file = QuestionFile::parse(text).map_err(|error: QuestionFileError| Error::usage(error.to_string()))?;
        let members = members_of(text).unwrap_or_default();
        let named = names_threshold(text);
        Self::settle(file.verb(), file, members, named)
    }

    /// Read a question from a file the caller named.
    ///
    /// # Errors
    ///
    /// Returns an error of the local kind when the file cannot be read, and
    /// of the usage kind when its text breaks the grammar.
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| Error::local(format!("{}: {error}", path.display())))?;
        Self::from_json(&text)
    }

    /// The verb this question holds.
    #[must_use]
    pub const fn kind(&self) -> QuestionKind {
        self.kind
    }

    /// The question's text.
    #[must_use]
    pub fn text(&self) -> &str {
        match &self.asked {
            CoreQuestion::Decide { text, .. }
            | CoreQuestion::Choose { text, .. }
            | CoreQuestion::Score { text, .. }
            | CoreQuestion::Tag { text, .. } => text.as_str(),
        }
    }

    /// The options, labels, or levels, in the question's own order.
    #[must_use]
    pub fn members(&self) -> &[String] {
        &self.members
    }

    /// The model the question names, or the alias when it names none.
    #[must_use]
    pub fn model(&self) -> &str {
        &self.model
    }

    /// The core's own settled question, for the engine that builds a plan.
    #[must_use]
    pub const fn core(&self) -> &CoreQuestion {
        &self.asked
    }

    /// The rule the answer is read under, when the question holds one.
    #[must_use]
    pub const fn threshold(&self) -> Option<Threshold> {
        self.threshold
    }

    /// Whether the question itself named a threshold, against the default
    /// the grammar supplies. `rank` refuses a named one and takes the
    /// default as no rule at all.
    #[must_use]
    pub const fn threshold_named(&self) -> bool {
        self.threshold_named
    }

    /// The digest of this question with its threshold, the core's own.
    #[must_use]
    pub fn digest(&self) -> String {
        question_sha256(&self.asked, self.threshold)
            .unwrap_or_else(|error| format!("unrenderable: {error}"))
    }

    /// Reject a blank question text.
    fn check_text(text: &str) -> Result<(), Error> {
        if text.trim().is_empty() {
            return Err(Error::usage("the question text is blank"));
        }
        Ok(())
    }

    /// Reject a member list outside its verb's bounds or holding blanks.
    fn check_members(list: &[&str], low: usize, high: usize, name: &str) -> Result<(), Error> {
        if list.len() < low || list.len() > high {
            return Err(Error::usage(format!(
                "{name} takes {low} to {high} entries, and {count} came",
                count = list.len()
            )));
        }
        if list.iter().any(|member| member.trim().is_empty()) {
            return Err(Error::usage(format!("every {name} entry needs a name")));
        }
        Ok(())
    }

    /// Build through the one grammar, so parts and files agree.
    fn from_built(verb: Verb, body: serde_json::Value) -> Result<Self, Error> {
        let text = serde_json::to_string(&body)
            .map_err(|error| Error::defect(error.to_string()))?;
        let file = QuestionFile::parse(&text).map_err(|error: QuestionFileError| Error::usage(error.to_string()))?;
        let named = names_threshold(&text);
        Self::settle(verb, file, members_of(&text).unwrap_or_default(), named)
    }

    /// Resolve a parsed file into the settled question.
    fn settle(verb: Verb, file: QuestionFile, members: Vec<String>, threshold_named: bool) -> Result<Self, Error> {
        let resolved = resolve(verb, None, Some(&file), &Typed::default())
            .map_err(|error: QuestionFileError| Error::usage(error.to_string()))?;
        let asked = resolved
            .question()
            .cloned()
            .ok_or_else(|| Error::usage("the question names no text to ask"))?;
        let kind = match verb {
            Verb::Decide => QuestionKind::Decide,
            Verb::Choose => QuestionKind::Choose,
            Verb::Score => QuestionKind::Score,
            Verb::Tag => QuestionKind::Tag,
        };
        Ok(Self {
            asked,
            kind,
            threshold: resolved.threshold(),
            threshold_named,
            model: resolved.model().as_str().to_owned(),
            members,
        })
    }
}

/// Whether the question's own JSON named a `threshold` key.
fn names_threshold(text: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(text)
        .ok()
        .and_then(|value| value.get("threshold").cloned())
        .is_some()
}

/// The option, label, or level names a question's JSON carries, in order.
///
/// The grammar takes a list of names or an ordered map from name to
/// description; both give the same names in the same order.
fn members_of(text: &str) -> Option<Vec<String>> {
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let held = value.get("options").or_else(|| value.get("labels")).or_else(|| value.get("levels"))?;
    match held {
        serde_json::Value::Array(items) => Some(
            items.iter().filter_map(|item| item.as_str().map(str::to_owned)).collect(),
        ),
        serde_json::Value::Object(map) => {
            Some(map.keys().cloned().collect())
        }
        _ => None,
    }
}

/// A `decide` question waiting for its rule.
#[derive(Clone, Debug)]
pub struct DecideBuilder {
    text: String,
    model: Option<String>,
}

impl DecideBuilder {
    /// Finish with a single cut.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the cut is not a fraction
    /// from 0 to 1.
    pub fn cut(self, mark: f64) -> Result<Question, Error> {
        mark.to_string()
            .parse::<Threshold>()
            .map_err(|error| Error::usage(error.to_string()))?;
        self.finish(serde_json::json!(mark))
    }

    /// Finish with a band, and all three answers.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the band's sides are not
    /// fractions or its low side sits at or above its high side.
    pub fn band(self, low: f64, high: f64) -> Result<Question, Error> {
        format!("{low}:{high}")
            .parse::<Threshold>()
            .map_err(|error| Error::usage(error.to_string()))?;
        self.finish(serde_json::json!(format!("{low}:{high}")))
    }

    /// Name the model this question asks.
    #[must_use]
    pub fn model(mut self, name: &str) -> Self {
        self.model = Some(name.to_owned());
        self
    }

    /// Carry the held settings into the built form and parse it once.
    fn finish(self, threshold: serde_json::Value) -> Result<Question, Error> {
        let mut body = serde_json::json!({ "decide": self.text });
        body["threshold"] = threshold;
        if let Some(model) = self.model {
            body["model"] = serde_json::json!(model);
        }
        Question::from_built(Verb::Decide, body)
    }
}

/// A `choose` question waiting for its optional cut.
#[derive(Clone, Debug)]
pub struct ChoiceBuilder {
    text: String,
    options: Vec<String>,
    model: Option<String>,
    cut: Option<f64>,
}

impl ChoiceBuilder {
    /// Require the winner to reach this cut.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the cut is not a fraction
    /// from 0 to 1.
    pub fn cut(mut self, mark: f64) -> Result<Self, Error> {
        Threshold::from_str(&mark.to_string()).map_err(|error| Error::usage(error.to_string()))?;
        self.cut = Some(mark);
        Ok(self)
    }

    /// Name the model this question asks.
    #[must_use]
    pub fn model(mut self, name: &str) -> Self {
        self.model = Some(name.to_owned());
        self
    }

    /// Build the question.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the parts break the grammar.
    pub fn build(self) -> Result<Question, Error> {
        let mut body = serde_json::json!({ "choose": self.text, "options": self.options });
        if let Some(cut) = self.cut {
            body["threshold"] = serde_json::json!(cut);
        }
        if let Some(model) = self.model {
            body["model"] = serde_json::json!(model);
        }
        Question::from_built(Verb::Choose, body)
    }
}

/// A set of named questions, as `annotate` asks of one record.
///
/// The set's grammar is the specification's: a `version` of 1, an optional
/// top-level `threshold` that every `decide` member without its own
/// inherits, and `questions`, an object of named questions each with the
/// shape of one question file. `from_json` accepts exactly what the command
/// line's own parser accepts — that parser judges the text — and keeps the
/// members in file order, so the answer columns carry the order the file
/// names.
#[derive(Clone, Debug)]
pub struct QuestionSet {
    questions: Vec<Question>,
    names: Vec<String>,
}

impl QuestionSet {
    /// Read a set from its JSON grammar.
    ///
    /// The core's own question-set parser judges the text, so a set the
    /// command line refuses is refused here with the same words, and one it
    /// accepts is accepted. The members are then rebuilt in file order.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the text is not a set the
    /// grammar accepts.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        // One judge for acceptance: the command line's parser. The rebuild
        // below cannot widen what it accepts, only carry the questions into
        // this crate's own type in the file's order.
        thinkthen_core::QuestionSet::parse(text)
            .map_err(|error| Error::usage(error.to_string()))?;
        let value: serde_json::Value = serde_json::from_str::<serde_json::Value>(text)
            .map_err(|error| Error::usage(error.to_string()))?;
        let object = match &value {
            serde_json::Value::Object(map) => map,
            _ => return Err(Error::usage("a question set is a JSON object")),
        };
        let questions = object
            .get("questions")
            .and_then(|held| held.as_object())
            .ok_or_else(|| Error::usage("a question set holds a questions object"))?;
        let shared = object.get("threshold").cloned();
        let mut names = Vec::with_capacity(questions.len());
        let mut parsed = Vec::with_capacity(questions.len());
        for (name, member) in questions {
            let mut member = member.clone();
            if let (Some(shared), serde_json::Value::Object(fields)) = (&shared, &mut member) {
                if member_is_decide(fields) && !fields.contains_key("threshold") {
                    fields.insert("threshold".to_owned(), shared.clone());
                }
            }
            let text = serde_json::to_string(&member)
                .map_err(|error| Error::defect(error.to_string()))?;
            parsed.push(Question::from_json(&text)?);
            names.push(name.clone());
        }
        Ok(Self { questions: parsed, names })
    }

    /// Read a set from a file the caller named.
    ///
    /// # Errors
    ///
    /// Returns an error of the local kind when the file cannot be read, and
    /// of the usage kind when its text breaks the grammar.
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path)
            .map_err(|error| Error::local(format!("{}: {error}", path.display())))?;
        Self::from_json(&text)
    }

    /// The set's questions, in name order.
    #[must_use]
    pub fn questions(&self) -> &[Question] {
        &self.questions
    }

    /// The set's names, in the same order as [`QuestionSet::questions`].
    #[must_use]
    pub fn names(&self) -> &[String] {
        &self.names
    }
}

/// Whether a set member is a `decide` question, the one verb that inherits
/// the set's shared threshold.
fn member_is_decide(fields: &serde_json::Map<String, serde_json::Value>) -> bool {
    fields.contains_key("decide")
}

/// Builds an engine from a config: the seam every surface binds through,
/// so replacing the stand-in with the real engine changes the one line
/// that names the connector.
pub trait Connector: Send + Sync {
    /// Build an engine. Every field the config sets wins over the
    /// environment; a connector reads its own environment only for what
    /// the config leaves unset, and only its own knobs.
    ///
    /// # Errors
    ///
    /// The usage kind when the config cannot be honored (an unresolvable
    /// address, for example), and the local kind when the connector's own
    /// resources cannot be prepared.
    fn connect(&self, config: &EngineConfig) -> Result<std::sync::Arc<dyn Engine>, Error>;
}

/// The one engine every surface binds.
///
/// An implementation carries the settings it was built with, holds no
/// thread between calls, rebuilds its state after a fork, and keeps one
/// width gate for the process. The verbs take the question first, as text
/// the host resolved or as a built [`Question`], and every method without
/// `_opts` is the `_opts` form with [`Options::new`].
///
/// The rules that ride with the verbs:
///
/// - `filter` refuses a band, `rank` refuses any threshold, `choose` takes
///   a cut alone, and `score` and `tag` take none.
/// - `filter` and `rank` cross once at the engine's width and return every
///   judgment's placement; `filter` keeps the records that reached the mark
///   and `rank` orders most likely yes first with ties in input order.
/// - `find` sends every unit together in one request, two to 255 of them,
///   and the answer is relative.
/// - `annotate` crosses once per record and adds one field per question.
/// - A bulk call with more records than the request limit is refused before
///   its first request.
pub trait Engine: Send + Sync {
    /// Ask once, and wait for the answer.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage, backend, deadline, or cancelled kind.
    fn decide_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Answer, Error>;

    /// Ask of every record, once, at the engine's width, keeping every
    /// judgment in input order.
    ///
    /// The calling thread waits on a tick and runs the poll callback each
    /// tick, which is where a host hears its own interrupts.
    ///
    /// # Errors
    ///
    /// Returns the first failure any record meets, of the usage, backend,
    /// deadline, or cancelled kind.
    fn decide_many_opts(
        &self,
        question: &Question,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Judgment>, Error>;

    /// Pick the option the evidence fits best.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_opts`].
    fn choose_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Option<String>, Error>;

    /// Place the evidence on the question's levels.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_opts`].
    fn score_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Scored, Error>;

    /// Name the labels that held, in the question's order.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_opts`].
    fn tag_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Vec<String>, Error>;

    /// Keep the records whose evidence reached the mark, cut questions
    /// alone.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many_opts`], and the usage kind when
    /// the question holds a band.
    fn filter_opts(
        &self,
        question: &Question,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<usize>, Error>;

    /// Order the records most likely yes first, ties in input order,
    /// threshold-less questions alone.
    ///
    /// Each [`Ranked`] carries the ruled pair: the record's place in the
    /// input and the probability the backend gave it. No surface returns
    /// the bare record alone.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many_opts`], and the usage kind when
    /// the question holds a threshold.
    fn rank_opts(
        &self,
        question: &Question,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Ranked>, Error>;

    /// Pick the unit that best answers the question, out of two to 255 sent
    /// together.
    ///
    /// The [`Found`] carries the ruled pair: the winning unit's place in
    /// the input and its probability. No surface returns the bare unit
    /// alone.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_opts`], and the usage kind when the
    /// count is outside its bounds.
    fn find_opts(
        &self,
        question: &Question,
        units: &[&str],
        options: Options<'_>,
    ) -> Result<Found, Error>;

    /// Ask every question in the set of every record, once each, adding one
    /// field per question in the set's name order.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many_opts`].
    fn annotate_opts(
        &self,
        set: &QuestionSet,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<AnnotatedRecord>, Error>;

    /// One judgment plus the audit trail, with the sends that produced it.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_opts`].
    fn details_opts(
        &self,
        question: &Question,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Details, Error>;

    /// Find every name in one text, and the relations the rules allow.
    ///
    /// A text the recordings do not hold is a usage error naming the text;
    /// a rule the recordings do not hold is a usage error naming the rule.
    /// The stand-in answers from its replay table, so nothing is invented.
    ///
    /// # Errors
    ///
    /// The usage kind for a missing text, an unrecorded rule, a blank
    /// relation end, or a named end outside the asked kinds; otherwise the
    /// backend, deadline, or cancelled kind.
    fn recognize_opts(
        &self,
        ask: &Recognize,
        text: &str,
        options: Options<'_>,
    ) -> Result<Recognized, Error>;

    /// Say how every record relates to the others: one pick-one question
    /// per legal pair, all records crossing at once.
    ///
    /// More than [`MAX_RELATE_RECORDS`] records is a usage error before
    /// anything happens; [`relate_checked`] enforces that for every caller
    /// that enters through it.
    ///
    /// # Errors
    ///
    /// The usage kind past the record limit, for a missing text, an
    /// unrecorded rule, a kind field the recordings cannot honour, or a
    /// blank relation end; otherwise the backend, deadline, or cancelled
    /// kind.
    fn relate_opts(
        &self,
        ask: &Relate,
        records: &[&str],
        options: Options<'_>,
    ) -> Result<Vec<Edge>, Error>;

    /// The counters since the last reset.
    fn usage(&self) -> Usage;

    /// Ask once, with neither a token nor a deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_opts`].
    fn decide(&self, question: &Question, evidence: &str) -> Result<Answer, Error> {
        self.decide_opts(question, evidence, Options::new())
    }

    /// Ask of every record, with neither a token nor a deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many_opts`].
    fn decide_many(
        &self,
        question: &Question,
        records: &[&str],
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Judgment>, Error> {
        self.decide_many_opts(question, records, Options::new(), poll)
    }

    /// Ask once with an optional token, the form a database shim holds.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_opts`].
    fn decide_with(
        &self,
        question: &Question,
        evidence: &str,
        cancel: Option<&Cancel>,
    ) -> Result<Answer, Error> {
        self.decide_opts(question, evidence, Options::new().maybe_cancel(cancel))
    }

    /// Pick the best option, with neither a token nor a deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::choose_opts`].
    fn choose(&self, question: &Question, evidence: &str) -> Result<Option<String>, Error> {
        self.choose_opts(question, evidence, Options::new())
    }

    /// Place the evidence on its levels, with neither a token nor a
    /// deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::score_opts`].
    fn score(&self, question: &Question, evidence: &str) -> Result<Scored, Error> {
        self.score_opts(question, evidence, Options::new())
    }

    /// Name the labels that held, with neither a token nor a deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::tag_opts`].
    fn tag(&self, question: &Question, evidence: &str) -> Result<Vec<String>, Error> {
        self.tag_opts(question, evidence, Options::new())
    }

    /// Keep the records that reached the mark, with neither a token nor a
    /// deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::filter_opts`].
    fn filter(
        &self,
        question: &Question,
        records: &[&str],
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<usize>, Error> {
        self.filter_opts(question, records, Options::new(), poll)
    }

    /// Order the records most likely yes first, with neither a token nor a
    /// deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::rank_opts`].
    fn rank(
        &self,
        question: &Question,
        records: &[&str],
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Ranked>, Error> {
        self.rank_opts(question, records, Options::new(), poll)
    }

    /// Pick the unit that best answers the question, with neither a token
    /// nor a deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::find_opts`].
    fn find(&self, question: &Question, units: &[&str]) -> Result<Found, Error> {
        self.find_opts(question, units, Options::new())
    }

    /// Ask every question in the set of every record, with neither a token
    /// nor a deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::annotate_opts`].
    fn annotate(
        &self,
        set: &QuestionSet,
        records: &[&str],
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<AnnotatedRecord>, Error> {
        self.annotate_opts(set, records, Options::new(), poll)
    }

    /// One judgment plus its audit trail, with neither a token nor a
    /// deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::details_opts`].
    fn details(&self, question: &Question, evidence: &str) -> Result<Details, Error> {
        self.details_opts(question, evidence, Options::new())
    }

    /// Find every name in one text, with neither a token nor a deadline.
    ///
    /// # Errors
    ///
    /// The usage kind for a missing text, an unrecorded rule, a blank
    /// relation end, or a named end outside the asked kinds; otherwise the
    /// backend, deadline, or cancelled kind.
    fn recognize(&self, ask: &Recognize, text: &str) -> Result<Recognized, Error> {
        self.recognize_opts(ask, text, Options::new())
    }

    /// Say how every record relates to the others, with neither a token nor
    /// a deadline; the record limit is checked through [`relate_checked`].
    ///
    /// # Errors
    ///
    /// The usage kind past the record limit, for a missing text, an
    /// unrecorded rule, a kind field the recordings cannot honour, or a
    /// blank relation end; otherwise the backend, deadline, or cancelled
    /// kind.
    fn relate(&self, ask: &Relate, records: &[&str]) -> Result<Vec<Edge>, Error> {
        guard_relate_records(records.len())?;
        self.relate_opts(ask, records, Options::new())
    }
}

#[cfg(test)]
mod tests {
    use super::{Answer, ErrorKind, Options, Question, QuestionSet};

    /// Building from parts and loading a file give the same question.
    #[test]
    fn parts_and_files_agree() {
        let built = Question::decide("Does the customer ask for a refund?")
            .expect("the text is fine")
            .band(0.2, 0.8)
            .expect("the band is fine");
        let loaded = Question::from_json(
            r#"{"decide":"Does the customer ask for a refund?","threshold":"0.2:0.8"}"#,
        )
        .expect("the file parses");
        assert_eq!(built.digest(), loaded.digest());
        assert_eq!(built.kind(), loaded.kind());
        assert_eq!(built.text(), loaded.text());
        assert_eq!(built.model(), loaded.model());
    }

    /// The verbs' member bounds and blank checks are usage errors.
    #[test]
    fn the_bounds_hold() {
        let one_option = Question::choose("Which team?", &["billing"]);
        assert_eq!(one_option.unwrap_err().kind, ErrorKind::Usage);

        let blank_member = Question::choose("Which team?", &["billing", " "]);
        assert_eq!(blank_member.unwrap_err().kind, ErrorKind::Usage);

        let eleven_levels: Vec<&str> = (0..11).map(|_| "l").collect();
        let too_many = Question::score("How strong?", &eleven_levels);
        assert_eq!(too_many.unwrap_err().kind, ErrorKind::Usage);

        let blank = Question::decide("   ");
        assert_eq!(blank.unwrap_err().kind, ErrorKind::Usage);
    }

    /// A cut outside its fraction range is a usage error.
    #[test]
    fn the_cut_range_holds() {
        let bad = Question::decide("Refund?").expect("fine").cut(1.5);
        assert_eq!(bad.unwrap_err().kind, ErrorKind::Usage);
    }

    /// A set reads with its names in file order, and the verbs carry.
    #[test]
    fn a_set_reads_with_names() {
        let set = QuestionSet::from_json(
            r#"{"version":1,"questions":{
                "spam":{"decide":"Spam?","threshold":0.5},
                "kind":{"choose":"Which kind?","options":["bug","feature"]}}}"#,
        )
        .expect("the set parses");
        assert_eq!(set.names(), ["spam", "kind"], "file order, not name order");
        assert_eq!(set.questions()[1].members(), ["bug", "feature"]);
    }

    /// A missing file fails with the local kind.
    #[test]
    fn a_missing_file_is_local() {
        let missing = Question::from_file(std::path::Path::new("/no/such/file.json"));
        assert_eq!(missing.unwrap_err().kind, ErrorKind::Local);
    }

    /// A spent deadline carries its budget in the message, per the ruling.
    #[test]
    fn a_deadline_names_its_budget() {
        let budget = std::time::Duration::from_millis(1);
        let spent = Options::new().deadline_in(budget);
        std::thread::sleep(std::time::Duration::from_millis(3));
        let error = super::Error::guard(&spent).expect_err("the deadline is gone");
        assert_eq!(error.kind, ErrorKind::Deadline);
        assert!(error.message.contains("0.001"), "the message names the value: {error}");
        assert!(error.retryable);
    }

    /// A zero budget is legal: it is spent before the call starts, sends
    /// nothing, and returns the deadline kind naming the budget.
    #[test]
    fn a_zero_budget_is_spent_immediately() {
        let spent = Options::new().deadline_in(std::time::Duration::ZERO);
        assert!(spent.passed(), "a zero budget is already gone");
        let error = super::Error::guard(&spent).expect_err("the deadline is gone");
        assert_eq!(error.kind, ErrorKind::Deadline);
        assert!(error.message.contains("the deadline of 0 s"), "the message names the value: {error}");
        assert!(error.retryable, "a fresh budget may answer");
    }

    /// A host's deadline is converted, checked, at one door: a NaN, a
    /// negative, or an oversized budget is the usage kind, the sentinel
    /// means no deadline, and zero stays a spent deadline.
    #[test]
    fn a_hosts_deadline_is_converted_checked() {
        use std::time::Duration;
        assert_eq!(
            super::deadline_from_seconds(2.5),
            Ok(Some(Duration::from_secs_f64(2.5))),
            "plain seconds pass through"
        );
        assert_eq!(super::deadline_from_seconds(0.0), Ok(Some(Duration::ZERO)), "zero is a spent deadline");
        assert_eq!(super::deadline_from_seconds(super::NO_DEADLINE), Ok(None), "the sentinel means none");
        assert_eq!(
            super::deadline_from_seconds(f64::from(u32::MAX)),
            Ok(Some(Duration::from_secs(u64::from(u32::MAX)))),
            "the largest budget the engine holds is accepted"
        );
        assert_eq!(super::deadline_from_seconds(f64::NAN).unwrap_err().kind, ErrorKind::Usage, "NaN is refused");
        assert_eq!(super::deadline_from_seconds(-0.5).unwrap_err().kind, ErrorKind::Usage, "a negative other than the sentinel is refused");
        assert_eq!(super::deadline_from_seconds(f64::INFINITY).unwrap_err().kind, ErrorKind::Usage, "infinity is refused");
        assert_eq!(super::deadline_from_seconds(1e300).unwrap_err().kind, ErrorKind::Usage, "an oversized budget is refused");
        assert_eq!(
            super::deadline_from_millis(250.0),
            Ok(Some(Duration::from_millis(250))),
            "milliseconds convert"
        );
        assert_eq!(super::deadline_from_millis(0.0), Ok(Some(Duration::ZERO)), "zero milliseconds is spent");
        assert_eq!(super::deadline_from_millis(super::NO_DEADLINE), Ok(None), "the sentinel means none in milliseconds too");
        assert_eq!(super::deadline_from_millis(f64::NAN).unwrap_err().kind, ErrorKind::Usage, "NaN milliseconds are refused");
        assert_eq!(super::deadline_from_millis(-2.0).unwrap_err().kind, ErrorKind::Usage, "negative milliseconds are refused");
        assert_eq!(super::deadline_from_millis(1e300).unwrap_err().kind, ErrorKind::Usage, "oversized milliseconds are refused");
    }

    /// The Options helper is the one call a host door makes: it carries a
    /// checked budget, treats the sentinel as no deadline, and never
    /// panics on a hostile value.
    #[test]
    fn a_hosts_deadline_arms_options() {
        let armed = Options::new().with_deadline_seconds(Some(5.0)).expect("fine");
        assert!(!armed.passed(), "five seconds is ahead");
        assert!(armed.remaining().is_some(), "the deadline is carried");
        let spent = Options::new().with_deadline_seconds(Some(0.0)).expect("legal");
        assert!(spent.passed(), "zero is spent");
        let none = Options::new().with_deadline_seconds(Some(super::NO_DEADLINE)).expect("legal");
        assert!(!none.passed(), "the sentinel carries no deadline");
        assert!(none.remaining().is_none(), "nothing to run out");
        assert!(Options::new().with_deadline_seconds(None).expect("legal").remaining().is_none(), "None means no deadline");
        let hostile = Options::new().with_deadline_seconds(Some(f64::INFINITY));
        assert_eq!(hostile.err().expect("refused").kind, ErrorKind::Usage, "no panic, the usage kind");
        let millis = Options::new().with_deadline_millis(Some(250.0)).expect("fine");
        assert!(millis.remaining().is_some_and(|left| left <= std::time::Duration::from_millis(250)), "the millis budget is carried");
        assert!(Options::new().with_deadline_millis(Some(super::NO_DEADLINE)).expect("legal").remaining().is_none());
        let hostile = Options::new().with_deadline_millis(Some(f64::NAN));
        assert_eq!(hostile.err().expect("refused").kind, ErrorKind::Usage, "no panic, the usage kind");
    }

    /// The door JSON for `find` and `rank` is the ruled pair, one shape:
    /// the place in the input and the probability.
    #[test]
    fn the_find_and_rank_pair_is_the_door_json() {
        let ranked = super::Ranked { index: 3, probability: 0.97 };
        assert_eq!(
            serde_json::to_string(&ranked).expect("serializes"),
            r#"{"index":3,"probability":0.97}"#
        );
        let found = super::Found { index: Some(2), probability: 0.55 };
        assert_eq!(
            serde_json::to_string(&found).expect("serializes"),
            r#"{"index":2,"probability":0.55}"#
        );
        let none = super::Found { index: None, probability: 0.03 };
        assert_eq!(
            serde_json::to_string(&none).expect("serializes"),
            r#"{"index":null,"probability":0.03}"#
        );
    }

    /// The public word for the middle arm is unsure, on every page.
    #[test]
    fn the_public_word_is_unsure() {
        assert_eq!(Answer::Unsure.to_string(), "unsure");
        assert_eq!(Answer::Unsure.value(), None);
    }

    /// The design page's question file example reads whole.
    #[test]
    fn the_recognize_file_reads() {
        let ask = super::Recognize::from_json(
            r#"{"kinds":{"person":"A human being, by name.",
                 "organization":"A company.","place":"A city."},
                "relations":[
                  {"name":"works_for","source":"person","target":"organization","reads":"works for"},
                  {"name":"located_in","source":"*","target":"place"},
                  {"name":"married_to","source":"person","target":"person","either":true}],
                "threshold":0.4,"relation_threshold":0.6}"#,
        )
        .expect("the file parses");
        assert_eq!(
            ask.kinds,
            ["person", "organization", "place"],
            "an object of kinds carries the file's order; the list form always did"
        );
        assert_eq!(ask.relations.len(), 3);
        assert_eq!(ask.relations[1].from, super::Kind::Any);
        assert!(ask.relations[2].either);
        assert!((ask.threshold - 0.4).abs() < f64::EPSILON);
        assert!((ask.relation_threshold - 0.6).abs() < f64::EPSILON);
    }

    /// The old `from`/`to` spelling is refused, and the sentence names the
    /// ruled spelling; nothing has shipped, so no alias is kept.
    #[test]
    fn the_old_relation_spelling_is_refused() {
        let old = super::Recognize::from_json(
            r#"{"kinds":["person"],"relations":[{"name":"works_for","from":"person","to":"organization"}]}"#,
        )
        .expect_err("from/to is refused");
        assert_eq!(old.kind, ErrorKind::Usage);
        assert!(old.message.contains("source and target"), "{old}");

        let half = super::Relate::from_json(
            r#"{"relations":[{"name":"covers","source":"test","to":"requirement"}]}"#,
        )
        .expect_err("a to key with a source key is refused");
        assert_eq!(half.kind, ErrorKind::Usage);
        assert!(half.message.contains("source and target"), "{half}");
    }

    /// A missing end, a kind outside the asked kinds, and a threshold out of
    /// range are usage errors.
    #[test]
    fn the_rule_shape_holds() {
        let missing = super::RelationRule::new("works_for", super::Kind::named("person"), super::Kind::named(""))
            .expect_err("a blank end is refused");
        assert_eq!(missing.kind, ErrorKind::Usage);
        assert!(missing.message.contains("missing end"), "{missing}");

        let outside = super::Recognize::from_json(
            r#"{"kinds":["person"],"relations":[{"name":"works_for","source":"person","target":"vessel"}]}"#,
        )
        .expect_err("a kind outside the asked kinds is refused");
        assert!(outside.message.contains("vessel"), "{outside}");

        let range = super::Recognize::new().threshold(1.5).expect_err("out of range");
        assert_eq!(range.kind, ErrorKind::Usage);
    }

    /// The relate file reads bare names and either lists.
    #[test]
    fn the_relate_file_reads() {
        let ask = super::Relate::from_json(
            r#"{"relations":["caused_by",{"name":"covers","source":"test","target":"requirement"}],
                "either":["same_as"],"kind_field":"/type","threshold":0.9}"#,
        )
        .expect("the file parses");
        assert_eq!(ask.relations.len(), 3);
        assert_eq!(ask.relations[0].from, super::Kind::Any);
        assert!(ask.relations[2].either);
        assert_eq!(ask.kind_field.as_deref(), Some("/type"));
        assert!((ask.threshold - 0.9).abs() < f64::EPSILON);
    }

    /// The record limit refuses at 256 and passes at 255.
    #[test]
    fn the_relate_limit_holds() {
        assert!(super::guard_relate_records(255).is_ok());
        let over = super::guard_relate_records(256).expect_err("past the limit");
        assert_eq!(over.kind, ErrorKind::Usage);
        assert!(over.message.contains("255"), "{over}");
    }

    /// The host-facing JSON spells the ends source and target, and never
    /// from and to; the numbers carry their ruled names.
    #[test]
    fn the_door_json_spells_source_and_target() {
        let answer = super::Recognized {
            entities: vec![super::Entity {
                id: 1,
                text: "Maria Chen".into(),
                kind: "person".into(),
                start: 0,
                end: 10,
                strength: 0.98,
            }],
            relations: vec![super::Relation {
                name: "works_for".into(),
                source: 1,
                target: 2,
                probability: 0.94,
            }],
        };
        let json = answer.to_json();
        assert!(json.contains("\"source\":1"), "{json}");
        assert!(json.contains("\"target\":2"), "{json}");
        assert!(json.contains("\"probability\":0.94"), "{json}");
        assert!(json.contains("\"strength\":0.98"), "{json}");
        assert!(!json.contains("\"from\""), "{json}");
        assert!(!json.contains("\"to\""), "{json}");
        assert!(!json.contains("\"confidence\""), "the restricted word stays out: {json}");

        let edges = super::edges_json(&[super::Edge {
            name: "caused_by".into(),
            source: 1,
            target: 4,
            probability: 0.94,
            source_kind: None,
            target_kind: None,
        }]);
        assert!(edges.starts_with("{\"edges\":["), "{edges}");
        assert!(edges.contains("\"source\":1"), "{edges}");
        assert!(!edges.contains("kind"), "the ends' kinds stay out when a rule names none: {edges}");
    }

    /// A boundary passes a clean result through, panic or not.
    #[test]
    fn the_boundary_passes_clean_results_through() {
        let fine: Result<u8, super::Error> =
            super::catch_panic("the door", || Ok(7));
        assert_eq!(fine.expect("passes"), 7);

        let refused: Result<u8, super::Error> =
            super::catch_panic("the door", || Err(super::Error::usage("no")));
        let refused = refused.expect_err("refused");
        assert_eq!(refused.kind, ErrorKind::Usage);
    }

    /// A panic becomes the defect error, message and all.
    #[test]
    fn a_panic_becomes_the_defect_error() {
        let caught: Result<u8, super::Error> =
            super::catch_panic("the decide door", || panic!("the answer blew up"));
        let caught = caught.expect_err("the boundary catches the panic");
        assert_eq!(caught.kind, ErrorKind::Defect, "{caught}");
        assert!(
            caught.message.contains("the decide door"),
            "the message names the boundary: {caught}"
        );
        assert!(
            caught.message.contains("the answer blew up"),
            "the message carries the panic's own words: {caught}"
        );
    }

    /// The formatter reads both string payloads and names a non-text one.
    #[test]
    fn the_panic_text_formatter_reads_a_payload() {
        /// Catch one panicking body and hand back its payload.
        fn payload_of(body: impl FnOnce() + std::panic::UnwindSafe) -> Box<dyn std::any::Any + Send> {
            match std::panic::catch_unwind(body) {
                Ok(()) => unreachable!("the body panics"),
                Err(payload) => payload,
            }
        }

        let borrowed = payload_of(|| panic!("a borrowed phrase"));
        assert_eq!(super::panic_text(borrowed.as_ref()), "a borrowed phrase");

        let owned = payload_of(|| panic!("a {}", "formatted phrase"));
        assert_eq!(super::panic_text(owned.as_ref()), "a formatted phrase");

        let odd = payload_of(|| std::panic::panic_any(42_u8));
        assert_eq!(super::panic_text(odd.as_ref()), "a payload that is not text");
    }
}
