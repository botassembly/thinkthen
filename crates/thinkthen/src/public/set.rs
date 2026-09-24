//! Named question sets for `annotate`.

use std::fmt;
use std::path::Path;

use crate::core::{self, Pointer, Threshold};
use crate::public::choice::Choice;
use crate::public::error::Error;
use crate::public::question::{
    BandedQuestion, ChooseQuestion, Kind, Question, QuestionKind, TagQuestion,
};

/// Ordered, named questions that `annotate` asks of each record.
///
/// `Debug` names the members' count alone.
#[derive(Clone, PartialEq)]
pub struct QuestionSet(pub(crate) core::QuestionSet);

impl fmt::Debug for QuestionSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QuestionSet")
            .field("questions", &self.0.questions().len())
            .finish_non_exhaustive()
    }
}

impl QuestionSet {
    /// Read one version-one question set. A member whose `on` names a part
    /// of a record is refused, because a library call's evidence is one
    /// whole text.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming what the set breaks.
    pub fn from_json(value: &str) -> Result<Self, Error> {
        let set =
            core::QuestionSet::parse(value).map_err(|error| Error::usage(error.to_string()))?;
        let root = Pointer::new("").map_err(|_| Error::defect("the root pointer was refused"))?;
        if set
            .questions()
            .iter()
            .any(|member| member.on() != [root.clone()])
        {
            return Err(Error::usage(
                "a library question set reads each record whole, so no member takes `on`",
            ));
        }
        Ok(Self(set))
    }

    /// Read one question set from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Local`] when the file cannot be read or breaks a rule.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path)
            .map_err(|_| Error::local("the question set could not be read"))?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }

    /// Each member's name and kind, in set order. A banded member reads `Decide`.
    pub fn members(&self) -> impl ExactSizeIterator<Item = (&str, QuestionKind)> + '_ {
        self.0
            .questions()
            .iter()
            .map(|member| (member.name(), QuestionKind::of(member.question())))
    }

    /// Name questions one at a time.
    #[must_use]
    pub fn builder() -> QuestionSetBuilder {
        QuestionSetBuilder(Vec::new())
    }
}

/// A question set under construction, in member order.
#[derive(Debug)]
pub struct QuestionSetBuilder(Vec<(String, Question)>);

impl QuestionSetBuilder {
    /// Add a `decide`, `choose`, `tag`, or `score` question under this name.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a bad or repeated name, a `rank` or
    /// `find` question, or a question naming its own model.
    pub fn question(mut self, name: &str, value: Question) -> Result<Self, Error> {
        if matches!(value.kind, Kind::Rank | Kind::Find) {
            return Err(Error::usage(
                "a question set takes decide, choose, tag, and score questions",
            ));
        }
        if value.model.is_some() {
            return Err(Error::usage(
                "a question set member takes no model; the engine names it",
            ));
        }
        if self.0.iter().any(|(held, _)| held == name) {
            let error = core::QuestionSetError::Duplicate(name.to_owned());
            return Err(Error::usage(error.to_string()));
        }
        core::QuestionSet::from_parts(vec![(name.to_owned(), value.core.clone(), None)])
            .map_err(|error| Error::usage(error.to_string()))?;
        self.0.push((name.to_owned(), value));
        Ok(self)
    }

    /// Add a banded `decide` question under this name.
    ///
    /// # Errors
    ///
    /// As [`QuestionSetBuilder::question`].
    pub fn banded(self, name: &str, value: BandedQuestion) -> Result<Self, Error> {
        self.question(name, value.0)
    }

    /// Add a typed `choose` question under this name.
    ///
    /// # Errors
    ///
    /// As [`QuestionSetBuilder::question`].
    pub fn choose<C: Choice>(self, name: &str, value: ChooseQuestion<C>) -> Result<Self, Error> {
        self.question(name, value.0)
    }

    /// Add a typed `tag` question under this name.
    ///
    /// # Errors
    ///
    /// As [`QuestionSetBuilder::question`].
    pub fn tag<C: Choice>(self, name: &str, value: TagQuestion<C>) -> Result<Self, Error> {
        self.question(name, value.0)
    }

    /// The finished set.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a set with no question.
    pub fn build(self) -> Result<QuestionSet, Error> {
        let members: Vec<(String, core::Question, Option<Threshold>)> = self
            .0
            .into_iter()
            .map(|(name, question)| (name, question.core, question.threshold))
            .collect();
        core::QuestionSet::from_parts(members)
            .map(QuestionSet)
            .map_err(|error| Error::usage(error.to_string()))
    }
}
