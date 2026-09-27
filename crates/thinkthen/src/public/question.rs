//! Typed descriptions, questions, their builders, and the `Choice` binding.
//!
//! Each builder makes the same core value the question-file parser makes, so a
//! built question and its file ask the same request.

use std::fmt;
use std::marker::PhantomData;
use std::path::Path;

use crate::core::{
    self, Json, Meaning, ModelName, QuestionFile, QuestionText, Threshold, Typed, Verb, Withheld,
    json_line, resolve,
};
use crate::public::builders::{
    ChooseBuilder, DecideBuilder, LabelBuilder, Listing, ScoreBuilder, TagBuilder,
};
use crate::public::choice::Choice;
use crate::public::error::Error;

/// What a question asks and which calls accept it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Kind {
    Decide,
    Banded,
    Choose,
    Tag,
    Score,
    Rank,
    Find,
    FindNone,
}

impl Kind {
    const fn public(self) -> QuestionKind {
        match self {
            Self::Decide | Self::Banded => QuestionKind::Decide,
            Self::Choose => QuestionKind::Choose,
            Self::Tag => QuestionKind::Tag,
            Self::Score => QuestionKind::Score,
            Self::Rank => QuestionKind::Rank,
            Self::Find | Self::FindNone => QuestionKind::Find,
        }
    }
}

/// What a question asks. A banded question reads `Decide`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuestionKind {
    /// A yes or no question.
    Decide,
    /// A pick of one option.
    Choose,
    /// A test of each label on its own.
    Tag,
    /// A placement on named levels.
    Score,
    /// A question `rank` orders by.
    Rank,
    /// A question `find` selects with.
    Find,
}

impl QuestionKind {
    pub(super) const fn of(question: &core::Question) -> Self {
        match question {
            core::Question::Decide { .. } => Self::Decide,
            core::Question::Choose { .. } => Self::Choose,
            core::Question::Tag { .. } => Self::Tag,
            core::Question::Score { .. } => Self::Score,
        }
    }
}

/// A label, an option, a level, or a yes or no meaning: plain text or one
/// ordered JSON object. `Debug` withholds what it says.
#[derive(Clone, Eq, PartialEq)]
pub struct Description {
    value: Json,
    text: String,
}

impl fmt::Debug for Description {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Description")
            .field(&Withheld(self.text.len()))
            .finish()
    }
}

impl Description {
    /// A plain text description.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn text(value: &str) -> Result<Self, Error> {
        if value.trim().is_empty() {
            return Err(Error::usage("a description is text, not white space"));
        }
        Self::of(Json::String(value.to_owned()))
    }

    /// Build one ordered JSON object description.
    #[must_use]
    pub fn builder() -> DescriptionBuilder {
        DescriptionBuilder {
            members: Vec::new(),
            examples: None,
        }
    }

    /// The description as compact JSON.
    #[must_use]
    pub fn as_json(&self) -> &str {
        &self.text
    }

    fn of(value: Json) -> Result<Self, Error> {
        let text =
            json_line(&value).map_err(|_| Error::defect("a description could not be written"))?;
        Ok(Self { value, text })
    }

    pub(super) fn meaning(&self) -> Result<Meaning, Error> {
        match &self.value {
            Json::String(text) => Meaning::new(text.clone()).map_err(Error::refused),
            other => Meaning::structured(other)
                .ok_or_else(|| Error::usage("a meaning is text or an object")),
        }
    }

    pub(crate) fn core(&self) -> Result<core::Description, Error> {
        core::Description::of_json(&self.value)
            .ok_or_else(|| Error::usage("a description is text or an object"))
    }
}

/// One description object, with members in call order. `Debug` counts the
/// members and withholds what they say.
pub struct DescriptionBuilder {
    members: Vec<(String, Json)>,
    examples: Option<usize>,
}

impl fmt::Debug for DescriptionBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DescriptionBuilder")
            .field("members", &self.members.len())
            .finish_non_exhaustive()
    }
}

impl DescriptionBuilder {
    /// Add `what`: what the label means.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when `what` is already set.
    pub fn what(self, value: &str) -> Result<Self, Error> {
        self.member("what", Json::String(value.to_owned()))
    }

    /// Add `not_for`: what the label does not cover.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when `not_for` is already set.
    pub fn not_for(self, value: &str) -> Result<Self, Error> {
        self.member("not_for", Json::String(value.to_owned()))
    }

    /// Append one example. Every example joins one `examples` array, placed
    /// where the first example call falls.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when `field_json` already set `examples`.
    pub fn example(mut self, value: &str) -> Result<Self, Error> {
        let place = match self.examples {
            Some(place) => place,
            None => {
                self = self.member("examples", Json::Array(Vec::new()))?;
                let place = self.members.len() - 1;
                self.examples = Some(place);
                place
            }
        };
        if let Some((_, Json::Array(examples))) = self.members.get_mut(place) {
            examples.push(Json::String(value.to_owned()));
        }
        Ok(self)
    }

    /// Add one more member holding one JSON value.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when the value is not JSON or the member is already set.
    pub fn field_json(self, name: &str, compact_json_value: &str) -> Result<Self, Error> {
        let value = Json::parse(compact_json_value).map_err(|_| {
            Error::usage(format!(
                "the description member {name:?} is not one JSON value"
            ))
        })?;
        self.member(name, value)
    }

    /// The finished description.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Defect`] only when the object cannot be written.
    pub fn build(self) -> Result<Description, Error> {
        Description::of(Json::Object(self.members))
    }

    fn member(mut self, name: &str, value: Json) -> Result<Self, Error> {
        if self.members.iter().any(|(held, _)| held == name) {
            return Err(Error::usage(format!(
                "the description already holds {name:?}"
            )));
        }
        self.members.push((name.to_owned(), value));
        Ok(self)
    }
}

/// A question and its rule, ready for the calls its kind accepts.
///
/// `Debug` names the kind alone.
#[derive(Clone, PartialEq)]
pub struct Question {
    pub(crate) core: core::Question,
    pub(crate) threshold: Option<Threshold>,
    pub(crate) model: Option<ModelName>,
    pub(crate) kind: Kind,
}

impl fmt::Debug for Question {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Question")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

/// A yes or no question read under a band, which answers unsure between its sides.
#[derive(Clone, Debug, PartialEq)]
pub struct BandedQuestion(pub(crate) Question);

/// A `choose` question bound to the labels of `C`.
#[derive(Clone, PartialEq)]
pub struct ChooseQuestion<C: Choice>(pub(crate) Question, pub(super) PhantomData<fn() -> C>);

/// A `tag` question bound to the labels of `C`.
#[derive(Clone, PartialEq)]
pub struct TagQuestion<C: Choice>(pub(crate) Question, pub(super) PhantomData<fn() -> C>);

/// `Debug` for the types that carry a `Choice`, which need not be `Debug`.
macro_rules! debug_without_choice {
    ($($name:ident),+) => {$(
        impl<C: Choice> fmt::Debug for $name<C> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }
    )+};
}

debug_without_choice!(ChooseQuestion, TagQuestion, ChooseBuilder, TagBuilder);

/// A question read from a file: under one cut, or under a band.
#[derive(Clone, Debug, PartialEq)]
pub enum LoadedQuestion {
    /// Any question with no band.
    Question(Question),
    /// A `decide` question under a band.
    Banded(BandedQuestion),
}

pub(super) fn text_of(value: &str) -> Result<QuestionText, Error> {
    QuestionText::new(value).map_err(Error::refused)
}

pub(super) fn cut(value: f64) -> Result<Threshold, Error> {
    Threshold::cut(value).map_err(Error::refused)
}

pub(super) fn model_of(held: &mut Option<ModelName>, value: &str) -> Result<(), Error> {
    once(
        held,
        ModelName::new(value).map_err(Error::refused)?,
        "the model",
    )
}

pub(super) fn once<T>(held: &mut Option<T>, value: T, name: &str) -> Result<(), Error> {
    if held.is_some() {
        return Err(Error::usage(format!("{name} is already set")));
    }
    *held = Some(value);
    Ok(())
}

impl Question {
    /// Start a yes or no question.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn decide(text: &str) -> Result<DecideBuilder, Error> {
        Ok(DecideBuilder {
            text: text_of(text)?,
            yes: None,
            no: None,
            model: None,
        })
    }

    /// Start a pick from the labels of `C`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn choose<C: Choice>(text: &str) -> Result<ChooseBuilder<C>, Error> {
        Ok(ChooseBuilder(Listing::new(text)?, PhantomData))
    }

    /// Start a question testing each label of `C` on its own.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn tag<C: Choice>(text: &str) -> Result<TagBuilder<C>, Error> {
        Ok(TagBuilder(Listing::new(text)?, PhantomData))
    }

    /// Start a `choose` question from options known only at run time.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn choose_labels(text: &str) -> Result<LabelBuilder, Error> {
        LabelBuilder::new(text, Verb::Choose)
    }

    /// Start a `tag` question from labels known only at run time.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn tag_labels(text: &str) -> Result<LabelBuilder, Error> {
        LabelBuilder::new(text, Verb::Tag)
    }

    /// What this question asks. A banded question reads `Decide`.
    #[must_use]
    pub fn kind(&self) -> QuestionKind {
        self.kind.public()
    }

    /// Start a placement on named levels, lowest first.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn score(text: &str) -> Result<ScoreBuilder, Error> {
        Ok(ScoreBuilder(Listing::new(text)?))
    }

    /// A question `rank` orders records by. It reads no rule.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn rank(text: &str) -> Result<Self, Error> {
        Ok(Self::yes_no(text_of(text)?, None, None, None, Kind::Rank))
    }

    /// A question `find` selects one unit with.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for blank text.
    pub fn find(text: &str) -> Result<Self, Error> {
        Ok(Self::yes_no(text_of(text)?, None, None, None, Kind::Find))
    }

    /// This find question with a `none` candidate beside the units, as
    /// `find --none` asks. The model may then answer that no unit fits.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a question that is not a find question.
    pub fn offering_none(self) -> Result<Self, Error> {
        match self.kind {
            Kind::Find | Kind::FindNone => Ok(Self {
                kind: Kind::FindNone,
                ..self
            }),
            _ => Err(Error::usage("only a find question offers none")),
        }
    }

    /// Read one question file. A file whose `on` names a part of a record is
    /// refused, because a library call's evidence is one whole text.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming what the file breaks.
    pub fn from_json(value: &str) -> Result<LoadedQuestion, Error> {
        let file = QuestionFile::parse(value).map_err(Error::refused)?;
        let resolved =
            resolve(file.verb(), None, Some(&file), &Typed::default()).map_err(Error::refused)?;
        if resolved
            .on()
            .iter()
            .any(|pointer| !pointer.as_str().is_empty())
        {
            return Err(Error::usage(
                "a library question reads its evidence whole, so it takes no `on`",
            ));
        }
        let core = resolved
            .question()
            .cloned()
            .ok_or_else(|| Error::defect("a question file resolved no question"))?;
        let model = (!resolved.sources().model_is_default()).then(|| resolved.model().clone());
        let threshold = resolved.threshold();
        let kind = match file.verb() {
            Verb::Decide if threshold.is_some_and(|rule| !rule.is_cut()) => Kind::Banded,
            Verb::Decide => Kind::Decide,
            Verb::Choose => Kind::Choose,
            Verb::Tag => Kind::Tag,
            Verb::Score => Kind::Score,
        };
        let question = Self {
            core,
            threshold,
            model,
            kind,
        };
        Ok(if kind == Kind::Banded {
            LoadedQuestion::Banded(BandedQuestion(question))
        } else {
            LoadedQuestion::Question(question)
        })
    }

    /// Read one question file from disk.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Local`] when the file cannot be read or breaks a rule.
    pub fn load(path: impl AsRef<Path>) -> Result<LoadedQuestion, Error> {
        let text = std::fs::read_to_string(path)
            .map_err(|_| Error::local("the question file could not be read"))?;
        Self::from_json(&text).map_err(|error| Error::local(error.detail().message()))
    }

    /// Bind a `choose` question to `C`, whose labels must be its options in order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for another kind or another label list.
    pub fn into_choose<C: Choice>(self) -> Result<ChooseQuestion<C>, Error> {
        self.bound::<C>(Kind::Choose)
            .map(|question| ChooseQuestion(question, PhantomData))
    }

    /// Bind a `tag` question to `C`, whose labels must be its labels in order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for another kind or another label list.
    pub fn into_tag<C: Choice>(self) -> Result<TagQuestion<C>, Error> {
        self.bound::<C>(Kind::Tag)
            .map(|question| TagQuestion(question, PhantomData))
    }

    fn bound<C: Choice>(self, kind: Kind) -> Result<Self, Error> {
        let labels = match &self.core {
            core::Question::Choose {
                options: labels, ..
            }
            | core::Question::Tag { labels, .. } => labels,
            _ => {
                return Err(Error::usage(
                    "this question is not a choose or tag question",
                ));
            }
        };
        if self.kind != kind
            || !labels
                .names()
                .map(String::as_str)
                .eq(C::labels().iter().copied())
        {
            return Err(Error::usage(
                "the question's labels are not the choice's labels in order",
            ));
        }
        Ok(self)
    }

    pub(super) const fn yes_no(
        text: QuestionText,
        yes: Option<Meaning>,
        no: Option<Meaning>,
        threshold: Option<Threshold>,
        kind: Kind,
    ) -> Self {
        Self {
            core: core::Question::Decide { text, yes, no },
            threshold,
            model: None,
            kind,
        }
    }
}
