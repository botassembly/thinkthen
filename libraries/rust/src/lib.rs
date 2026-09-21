//! The Rust surface: the engine itself, with no binding in between.
//!
//! This crate is the ruled Rust library of ADR 0017, pick 7: an [`Engine`]
//! value is built from the environment, plain calls return [`Result`], and
//! no async runtime comes with them. `Answer::Unsure` is a checked arm and
//! the host's own empty value is `None`, which [`Answer::value`] gives.
//!
//! Every call goes through the contract, never the engine beneath it: the
//! stand-in implements [`thinkthen_contract::Engine`] today, the real
//! engine implements it after the merge, and the move changes one
//! dependency in this crate's manifest. No rule, no retry, and no sending
//! lives here.
//!
//! The first argument of a yes-or-no verb is the question, as text or as a
//! built [`Question`]: `tt.decide("...", text)` and `tt.decide(&refund,
//! text)` ask the same thing, because the text form builds through the one
//! file grammar and inherits its default cut. `choose`, `score`, and `tag`
//! take a built question, because their members do not fit a plain
//! string.
//!
//! Errors are the contract's one shape, and Rust's own error class is
//! `Result<_, Error>`; the six kinds and the retryable signal ride the
//! [`Error`] value itself.

pub use thinkthen_contract::{
    Annotated, AnnotatedRecord, Answer, Cancel, Details, Error, ErrorKind, Found, Judgment,
    Options, Question, QuestionKind, QuestionSet, Ranked, Scored, Settings, Usage,
};

use thinkthen_contract::Engine as ContractEngine;
use thinkthen_standin::BlockingEngine;

/// The engine value every call goes through.
///
/// Built from the environment with [`Engine::from_env`], or from settled
/// settings. It holds no thread between calls, keeps one width gate for the
/// process, and rebuilds its state after a fork. Cloning shares the same
/// state, which is what a host embedding the library wants.
#[derive(Clone, Debug)]
pub struct Engine {
    inner: BlockingEngine,
}

impl Engine {
    /// Build an engine from the environment.
    ///
    /// The stand-in never fails here, and the signature is already the
    /// ruled one, so the real engine may refuse a broken environment
    /// without a page change.
    ///
    /// # Errors
    ///
    /// Never, on the stand-in. The real engine returns the local or usage
    /// kind when the environment it reads is broken.
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self { inner: BlockingEngine::from_env() })
    }

    /// Build an engine from settled settings, the form a host with its own
    /// configuration uses.
    #[must_use]
    pub fn from_settings(settings: Settings) -> Self {
        Self { inner: BlockingEngine::from_settings(settings) }
    }

    /// Ask once.
    ///
    /// # Errors
    ///
    /// Returns the usage, backend, deadline, or cancelled kind, per the
    /// contract.
    pub fn decide<Q: IntoQuestion>(&self, question: Q, evidence: &str) -> Result<Answer, Error> {
        self.decide_opts(question, evidence, Options::new())
    }

    /// Ask once, with the cancel token and the deadline.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`].
    pub fn decide_opts<Q: IntoQuestion>(
        &self,
        question: Q,
        evidence: &str,
        options: Options<'_>,
    ) -> Result<Answer, Error> {
        self.inner.decide_opts(&question.into_question()?, evidence, options)
    }

    /// Ask once with an optional token, the form a caller with one cancel
    /// for many calls holds.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`].
    pub fn decide_with<Q: IntoQuestion>(
        &self,
        question: Q,
        evidence: &str,
        cancel: Option<&Cancel>,
    ) -> Result<Answer, Error> {
        self.decide_opts(question, evidence, Options::new().maybe_cancel(cancel))
    }

    /// Ask of every record, once, keeping every judgment in input order.
    ///
    /// # Errors
    ///
    /// Returns the first failure any record meets, per the contract.
    pub fn decide_many<Q: IntoQuestion>(
        &self,
        question: Q,
        records: &[&str],
    ) -> Result<Vec<Judgment>, Error> {
        self.decide_many_opts(question, records, Options::new(), None)
    }

    /// Ask of every record, with the options and the poll callback the
    /// calling thread runs each tick while it waits.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many`].
    pub fn decide_many_opts<Q: IntoQuestion>(
        &self,
        question: Q,
        records: &[&str],
        options: Options<'_>,
        poll: Option<&mut dyn FnMut()>,
    ) -> Result<Vec<Judgment>, Error> {
        self.inner
            .decide_many_opts(&question.into_question()?, records, options, poll)
    }

    /// Pick the option the evidence fits best, from a built question.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`].
    pub fn choose(&self, question: &Question, evidence: &str) -> Result<Option<String>, Error> {
        self.inner.choose_opts(question, evidence, Options::new())
    }

    /// Place the evidence on the question's levels, from a built question.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`].
    pub fn score(&self, question: &Question, evidence: &str) -> Result<Scored, Error> {
        self.inner.score_opts(question, evidence, Options::new())
    }

    /// Name the labels that held, from a built question.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`].
    pub fn tag(&self, question: &Question, evidence: &str) -> Result<Vec<String>, Error> {
        self.inner.tag_opts(question, evidence, Options::new())
    }

    /// Keep the records whose evidence reached the mark, in input order.
    ///
    /// The returned records borrow from the input slice, so a bulk call
    /// crosses once and the kept evidence comes back with no clone.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many`], and the usage kind when the
    /// question holds a band.
    pub fn filter<'a, Q: IntoQuestion>(
        &self,
        question: Q,
        records: &'a [&str],
    ) -> Result<Vec<&'a str>, Error> {
        let kept = self.inner.filter_opts(
            &question.into_question()?,
            records,
            Options::new(),
            None,
        )?;
        Ok(kept.into_iter().map(|place| records[place]).collect())
    }

    /// Order the records most likely yes first, ties in input order.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many`], and the usage kind when the
    /// question names a threshold.
    pub fn rank<Q: IntoQuestion>(&self, question: Q, records: &[&str]) -> Result<Vec<Ranked>, Error> {
        self.inner.rank_opts(&question.into_question()?, records, Options::new(), None)
    }

    /// Pick the unit that best answers the question, out of the units sent
    /// together. The answer is relative.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`], and the usage kind when the count
    /// is outside its bounds.
    pub fn find<Q: IntoQuestion>(&self, question: Q, units: &[&str]) -> Result<Found, Error> {
        self.inner.find_opts(&question.into_question()?, units, Options::new())
    }

    /// Ask every question of the set of every record, once each, from a
    /// set or from a question-file path the caller names.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many`], and the local kind when a
    /// named file cannot be read.
    pub fn annotate<S: IntoSet>(
        &self,
        set: S,
        records: &[&str],
    ) -> Result<Vec<AnnotatedRecord>, Error> {
        self.inner
            .annotate_opts(&set.into_set()?, records, Options::new(), None)
    }

    /// One judgment plus the audit trail, with the sends that produced it.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`].
    pub fn details<Q: IntoQuestion>(&self, question: Q, evidence: &str) -> Result<Details, Error> {
        self.inner.details_opts(&question.into_question()?, evidence, Options::new())
    }

    /// Return the text unchanged, for testing.
    ///
    /// # Errors
    ///
    /// Returns no kind on the stand-in.
    pub fn probe(&self, text: &str) -> Result<String, Error> {
        self.inner.probe(text)
    }

    /// The counters since the last reset: sends, cache answers, tokens.
    #[must_use]
    pub fn usage(&self) -> Usage {
        self.inner.usage()
    }
}

/// A first argument that is the question, as text or as a built question.
///
/// The text form is a yes-or-no question under the grammar's default cut:
/// `tt.filter("Is this a complaint?", &reviews)` asks what
/// `Question::decide("Is this a complaint?")` with the file's default
/// would. This crate implements it for exactly the two ruled spellings,
/// `&Question` and `&str`, and no other implementation is supported.
pub trait IntoQuestion {
    /// Resolve to the built question.
    ///
    /// # Errors
    ///
    /// Returns the usage kind when the text is blank or breaks the
    /// grammar.
    fn into_question(self) -> Result<Question, Error>;
}


impl IntoQuestion for &Question {
    fn into_question(self) -> Result<Question, Error> {
        Ok(self.clone())
    }
}

impl IntoQuestion for &str {
    fn into_question(self) -> Result<Question, Error> {
        Question::from_json(&serde_json::json!({ "decide": self }).to_string())
    }
}

/// An `annotate` first argument: a built set, or a question-file path the
/// caller names. This crate implements it for exactly `&QuestionSet` and
/// `&str`, and no other implementation is supported.
pub trait IntoSet {
    /// Resolve to the built set.
    ///
    /// # Errors
    ///
    /// Returns the local kind when a named file cannot be read, and the
    /// usage kind when its text breaks the grammar.
    fn into_set(self) -> Result<QuestionSet, Error>;
}

impl IntoSet for &QuestionSet {
    fn into_set(self) -> Result<QuestionSet, Error> {
        Ok(self.clone())
    }
}

impl IntoSet for &str {
    fn into_set(self) -> Result<QuestionSet, Error> {
        QuestionSet::from_file(std::path::Path::new(self))
    }
}
