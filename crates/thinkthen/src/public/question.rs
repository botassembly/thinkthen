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
use crate::public::builders::{ChooseBuilder, DecideBuilder, Listing, ScoreBuilder, TagBuilder};
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
            Json::String(text) => {
                Meaning::new(text.clone()).map_err(|error| Error::usage(error.to_string()))
            }
            other => Meaning::structured(other)
                .ok_or_else(|| Error::usage("a meaning is text or an object")),
        }
    }

    pub(crate) fn core(&self) -> Result<core::Description, Error> {
        core::Description::of_json(&self.value)
            .ok_or_else(|| Error::usage("a description is text or an object"))
    }
}

/// One description object, with members in call order.
#[derive(Debug)]
pub struct DescriptionBuilder {
    members: Vec<(String, Json)>,
    examples: Option<usize>,
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
    QuestionText::new(value).map_err(|error| Error::usage(error.to_string()))
}

pub(super) fn cut(value: f64) -> Result<Threshold, Error> {
    Threshold::cut(value).map_err(|error| Error::usage(error.to_string()))
}

pub(super) fn model_of(held: &mut Option<ModelName>, value: &str) -> Result<(), Error> {
    if held.is_some() {
        return Err(Error::usage("the model is already set"));
    }
    *held = Some(ModelName::new(value).map_err(|error| Error::usage(error.to_string()))?);
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

    /// Read one question file. A file whose `on` names a part of a record is
    /// refused, because a library call's evidence is one whole text.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] naming what the file breaks.
    pub fn from_json(value: &str) -> Result<LoadedQuestion, Error> {
        let usage = |error: &dyn fmt::Display| Error::usage(error.to_string());
        let file = QuestionFile::parse(value).map_err(|error| usage(&error))?;
        let resolved = resolve(file.verb(), None, Some(&file), &Typed::default())
            .map_err(|error| usage(&error))?;
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

/// A closed set of labels a typed question answers with. Write one with [`choices!`](crate::choices).
pub trait Choice: Clone + Eq + Send + Sync + 'static {
    /// The label this value sends.
    fn label(&self) -> &'static str;
    /// Every label, in declared order.
    fn labels() -> &'static [&'static str];
    /// The value a label names.
    fn from_label(value: &str) -> Option<Self>;
}

/// Declare an enum of labels and its [`Choice`] implementation.
///
/// ```
/// thinkthen::choices! {
///     /// How a ticket is routed.
///     pub enum Route { Billing => "billing", Outage => "outage" }
/// }
/// assert_eq!(Route::from_label("outage"), Some(Route::Outage));
/// ```
#[macro_export]
macro_rules! choices {
    ($(#[$meta:meta])* $vis:vis enum $name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        $vis enum $name { $(#[doc = $label] $variant),+ }

        impl $name {
            /// The label this value sends.
            #[must_use]
            $vis const fn label(&self) -> &'static str {
                match self { $(Self::$variant => $label),+ }
            }

            /// Every label, in declared order.
            #[must_use]
            $vis const fn labels() -> &'static [&'static str] {
                &[$($label),+]
            }

            /// The value a label names.
            #[must_use]
            $vis fn from_label(value: &str) -> ::core::option::Option<Self> {
                #[deny(unreachable_patterns)]
                match value {
                    $($label => ::core::option::Option::Some(Self::$variant),)+
                    _ => ::core::option::Option::None,
                }
            }
        }

        // A lint in another crate's macro is silent, so a duplicate label
        // fails as a constant instead.
        #[allow(clippy::indexing_slicing, reason = "each index stays below its checked length")]
        const _: () = {
            let labels: &[&str] = &[$($label),+];
            let mut i = 0;
            while i < labels.len() {
                let mut j = i + 1;
                while j < labels.len() {
                    let (a, b) = (labels[i].as_bytes(), labels[j].as_bytes());
                    let (mut same, mut k) = (a.len() == b.len(), 0);
                    while same && k < a.len() {
                        same = a[k] == b[k];
                        k += 1;
                    }
                    assert!(!same, "choices! labels must differ");
                    j += 1;
                }
                i += 1;
            }
        };

        impl $crate::Choice for $name {
            fn label(&self) -> &'static str {
                $name::label(self)
            }
            fn labels() -> &'static [&'static str] {
                $name::labels()
            }
            fn from_label(value: &str) -> ::core::option::Option<Self> {
                $name::from_label(value)
            }
        }
    };
}
