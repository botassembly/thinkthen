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

use std::ops::Range;

pub use thinkthen_contract::{
    Annotated, AnnotatedRecord, Answer, Cancel, Cause, Details, Edge, Entity, Error, ErrorKind,
    Failed, FailureKind, Found, Judgment, Kind, MAX_RELATE_RECORDS, Options, Question,
    QuestionKind, QuestionSet, Ranked, Recognize, Recognized, Relate, Relation, RelationRule, Row,
    Scored, Settings, Usage, failed_questions, rows_json,
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
        Ok(Self {
            inner: BlockingEngine::from_env(),
        })
    }

    /// Build an engine from settled settings, the form a host with its own
    /// configuration uses.
    #[must_use]
    pub fn from_settings(settings: Settings) -> Self {
        Self {
            inner: BlockingEngine::from_settings(settings),
        }
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
        self.inner
            .decide_opts(&question.into_question()?, evidence, options)
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
        let kept =
            self.inner
                .filter_opts(&question.into_question()?, records, Options::new(), None)?;
        Ok(kept.into_iter().map(|place| records[place]).collect())
    }

    /// Order the records most likely yes first, ties in input order.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide_many`], and the usage kind when the
    /// question names a threshold.
    pub fn rank<Q: IntoQuestion>(
        &self,
        question: Q,
        records: &[&str],
    ) -> Result<Vec<Ranked>, Error> {
        self.inner
            .rank_opts(&question.into_question()?, records, Options::new(), None)
    }

    /// Pick the unit that best answers the question, out of the units sent
    /// together. The answer is relative.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::decide`], and the usage kind when the count
    /// is outside its bounds.
    pub fn find<Q: IntoQuestion>(&self, question: Q, units: &[&str]) -> Result<Found, Error> {
        self.inner
            .find_opts(&question.into_question()?, units, Options::new())
    }

    /// Find every name in one text, and, when a rule was given, the
    /// relations between the names.
    ///
    /// The answer is the contract's [`Recognized`]: typed [`Entity`] and
    /// [`Relation`] records. Each name's `start` and `end` count code
    /// points of the text the caller gave; Rust slices bytes, so slice a
    /// name with [`name_in`] or [`byte_range`] rather than the raw
    /// offsets, or an accent or an emoji before the name cuts the wrong
    /// bytes. The emoji case in the tests is the strict proof.
    ///
    /// The number on an entity is `strength`: a number we compute — the
    /// least of the word probabilities behind the name times the mean of
    /// the kind probabilities — defined once in the manual, with its parts
    /// under details, per the ruling of
    /// `sdlc/issues/2026-09-21-one-rule-for-every-number-the-tool-prints.md`.
    /// The vendor's `confidence` passes through under details only, under
    /// its own name, and nothing gates on it. The number on a relation is
    /// `probability`, passed through unchanged.
    ///
    /// # Errors
    ///
    /// The usage kind for a missing text, an unrecorded rule, a blank
    /// relation end, or a named end outside the asked kinds; otherwise
    /// the backend, deadline, or cancelled kind, per the contract.
    pub fn recognize(&self, ask: &Recognize, text: &str) -> Result<Recognized, Error> {
        self.inner.recognize(ask, text)
    }

    /// Say how every record relates to the others: one pick-one question
    /// per legal pair, every record crossing at once.
    ///
    /// More than [`MAX_RELATE_RECORDS`] records is a usage error before
    /// anything happens; the check lives in the contract, so every surface
    /// inherits it. The answer is a list of typed [`Edge`] records, with
    /// `probability` on each.
    ///
    /// # Errors
    ///
    /// The usage kind past the record limit, for a missing text, an
    /// unrecorded rule, a kind field the recordings cannot honour, or a
    /// blank relation end; otherwise the backend, deadline, or cancelled
    /// kind.
    pub fn relate(&self, ask: &Relate, records: &[&str]) -> Result<Vec<Edge>, Error> {
        self.inner.relate(ask, records)
    }

    /// Find every name in one text, with the options: the cancel token and
    /// the deadline ride every call.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::recognize`].
    pub fn recognize_opts(
        &self,
        ask: &Recognize,
        text: &str,
        options: Options<'_>,
    ) -> Result<Recognized, Error> {
        self.inner.recognize_opts(ask, text, options)
    }

    /// Say how every record relates to the others, with the options.
    ///
    /// # Errors
    ///
    /// Same kinds as [`Engine::relate`].
    pub fn relate_opts(
        &self,
        ask: &Relate,
        records: &[&str],
        options: Options<'_>,
    ) -> Result<Vec<Edge>, Error> {
        self.inner.relate_opts(ask, records, options)
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
        self.inner
            .details_opts(&question.into_question()?, evidence, Options::new())
    }

    /// The counters since the last reset: sends, cache answers, tokens.
    #[must_use]
    pub fn usage(&self) -> Usage {
        self.inner.usage()
    }
}

/// The byte range of one entity's name in the text it was found in.
///
/// The contract counts `start` and `end` in code points, the one unit a
/// user can check against the text they typed. Rust slices strings by
/// bytes, and this converts once: the range sits on the text's own char
/// boundaries, so `&text[range]` is the name. Both offsets saturate at
/// the end of the text, which only a broken engine can ask for.
#[must_use]
pub fn byte_range(entity: &Entity, text: &str) -> Range<usize> {
    Range {
        start: byte_of(text, entity.start),
        end: byte_of(text, entity.end),
    }
}

/// The name as a slice of the original text, in Rust's byte indexing.
///
/// Use it instead of `&text[entity.start..entity.end]`: the contract's
/// offsets count code points, so the raw slice cuts the wrong bytes the
/// moment an accent or an emoji sits before the name; the test
/// `the_emoji_case_slices_in_bytes` proves both halves. A broken engine's
/// offsets give the empty string, never a panic.
#[must_use]
pub fn name_in<'t>(entity: &Entity, text: &'t str) -> &'t str {
    text.get(byte_range(entity, text)).unwrap_or("")
}

/// The byte offset of the `codepoints`-th code point, saturating at the
/// end of the text.
fn byte_of(text: &str, codepoints: usize) -> usize {
    text.char_indices()
        .nth(codepoints)
        .map_or(text.len(), |(offset, _)| offset)
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
