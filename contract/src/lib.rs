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
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Details {
    /// The probability the backend gave the yes side.
    pub probability: f64,
    /// Yes, no, or unsure under the question's rule.
    pub answer: Answer,
    /// The model the request named.
    pub model: String,
    /// The digest of the question with its threshold, 64 hex figures.
    pub digest: String,
    /// The wire sends that produced this judgment.
    pub sends: u32,
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
}

/// One record's `annotate` answer: one field per question, in the set's
/// name order.
pub type AnnotatedRecord = Vec<(String, Annotated)>;

/// The settled settings an engine value carries, with one spelling each.
///
/// The defaults come from ADR 0017's settings table: the built-in address,
/// no key, the model alias, a width of 4, no request limit, the XDG cache
/// home, and a 100 MB cache cap. A setting the host passes always wins over
/// the environment, and a folder the user names always wins over the cache
/// home.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Settings {
    /// The backend address, overriding `THINKTHEN_BASE_URL`.
    pub address: Option<String>,
    /// The model, overriding the alias and the question's own name.
    pub model: Option<String>,
    /// How many requests one process has in flight at once.
    pub width: Option<usize>,
    /// The request limit: a bulk call with more records is refused before
    /// its first request.
    pub max_requests: Option<u64>,
    /// The cache folder, overriding `THINKTHEN_CACHE` and the XDG cache
    /// home. `None` leaves the engine's own default in place.
    pub cache: Option<std::path::PathBuf>,
    /// The cache cap in bytes, overriding the 100 MB default.
    pub cache_bytes: Option<u64>,
}

impl Settings {
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
/// The set's grammar is the specification's: an object of named questions,
/// each with the shape of one question file. The order is the set's own
/// name order.
#[derive(Clone, Debug)]
pub struct QuestionSet {
    questions: Vec<Question>,
    names: Vec<String>,
}

impl QuestionSet {
    /// Read a set from its JSON grammar.
    ///
    /// The grammar is the specification's: a `version`, an optional
    /// top-level `threshold` that every `decide` member without its own
    /// inherits, and `questions`, an object of named questions each with
    /// the shape of one question file.
    ///
    /// # Errors
    ///
    /// Returns an error of the usage kind when the text is not a set the
    /// grammar accepts.
    pub fn from_json(text: &str) -> Result<Self, Error> {
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
        let mut order: Vec<usize> = (0..names.len()).collect();
        order.sort_by(|one, two| names[*one].cmp(&names[*two]));
        let names = order.iter().map(|place| names[*place].clone()).collect();
        let questions = order.iter().map(|place| parsed[*place].clone()).collect();
        Ok(Self { questions, names })
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

    /// A set reads with its names in the set's order, and the verbs carry.
    #[test]
    fn a_set_reads_with_names() {
        let set = QuestionSet::from_json(
            r#"{"version":1,"questions":{
                "spam":{"decide":"Spam?","threshold":0.5},
                "kind":{"choose":"Which kind?","options":["bug","feature"]}}}"#,
        )
        .expect("the set parses");
        assert_eq!(set.names(), ["kind", "spam"]);
        assert_eq!(set.questions()[0].members(), ["bug", "feature"]);
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

    /// The public word for the middle arm is unsure, on every page.
    #[test]
    fn the_public_word_is_unsure() {
        assert_eq!(Answer::Unsure.to_string(), "unsure");
        assert_eq!(Answer::Unsure.value(), None);
    }
}
