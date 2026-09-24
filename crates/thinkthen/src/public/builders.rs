//! The question builders: each step checks its value and the last step
//! closes the question.

use std::marker::PhantomData;

use crate::core::{self, Labels, LabelsError, Meaning, ModelName, QuestionText, Threshold, Verb};
use crate::public::error::Error;
use crate::public::question::{
    BandedQuestion, Choice, ChooseQuestion, Description, Kind, Question, TagQuestion, cut,
    model_of, text_of,
};

/// A `decide` question under construction.
#[derive(Debug)]
pub struct DecideBuilder {
    pub(super) text: QuestionText,
    pub(super) yes: Option<Meaning>,
    pub(super) no: Option<Meaning>,
    pub(super) model: Option<ModelName>,
}

impl DecideBuilder {
    /// Say what a yes means.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when it is already set.
    pub fn yes(mut self, value: Description) -> Result<Self, Error> {
        once(&mut self.yes, value.meaning()?, "yes")?;
        Ok(self)
    }

    /// Say what a no means.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when it is already set.
    pub fn no(mut self, value: Description) -> Result<Self, Error> {
        once(&mut self.no, value.meaning()?, "no")?;
        Ok(self)
    }

    /// Ask this model instead of the engine's.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank model or a second one.
    pub fn model(mut self, value: &str) -> Result<Self, Error> {
        model_of(&mut self.model, value)?;
        Ok(self)
    }

    /// Finish under the default cut of 0.5.
    #[must_use]
    pub fn cut(self) -> Question {
        self.finish(Threshold::default(), Kind::Decide)
    }

    /// Finish under this cut, above zero and at most one.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a cut outside that range.
    pub fn cut_at(self, value: f64) -> Result<Question, Error> {
        Ok(self.finish(cut(value)?, Kind::Decide))
    }

    /// Finish under a band: yes at or above `high`, no below `low`, unsure between.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for sides outside zero to one or out of order.
    pub fn band(self, low: f64, high: f64) -> Result<BandedQuestion, Error> {
        let rule = Threshold::band(low, high).map_err(|error| Error::usage(error.to_string()))?;
        Ok(BandedQuestion(self.finish(rule, Kind::Banded)))
    }

    fn finish(self, rule: Threshold, kind: Kind) -> Question {
        let mut question = Question::yes_no(self.text, self.yes, self.no, Some(rule), kind);
        question.model = self.model;
        question
    }
}

fn once<T>(held: &mut Option<T>, value: T, name: &str) -> Result<(), Error> {
    if held.is_some() {
        return Err(Error::usage(format!("{name} is already set")));
    }
    *held = Some(value);
    Ok(())
}

/// The labels and model a list question gathers before it closes.
#[derive(Debug)]
pub(super) struct Listing {
    pub(super) text: QuestionText,
    pub(super) labels: Vec<(String, Option<core::Description>)>,
    pub(super) model: Option<ModelName>,
}

impl Listing {
    pub(super) fn new(text: &str) -> Result<Self, Error> {
        Ok(Self {
            text: text_of(text)?,
            labels: Vec::new(),
            model: None,
        })
    }

    fn next<C: Choice>(
        mut self,
        value: &C,
        description: Option<Description>,
    ) -> Result<Self, Error> {
        let place = self.labels.len();
        if C::labels().get(place) != Some(&value.label()) {
            return Err(Error::usage(format!(
                "label {} of the choice comes next, in the choice's own order",
                place + 1
            )));
        }
        self.labels.push((
            value.label().to_owned(),
            description.map(|held| held.core()).transpose()?,
        ));
        Ok(self)
    }

    fn close<C: Choice>(self, verb: Verb, threshold: Option<Threshold>) -> Result<Question, Error> {
        if self.labels.len() != C::labels().len() {
            return Err(Error::usage("every label of the choice must be given"));
        }
        let usage = |error: LabelsError| Error::usage(error.to_string());
        let (core, kind) = match verb {
            Verb::Tag => (
                core::Question::Tag {
                    text: self.text,
                    labels: Labels::tags(self.labels).map_err(usage)?,
                },
                Kind::Tag,
            ),
            _ => (
                core::Question::Choose {
                    text: self.text,
                    options: Labels::described(self.labels).map_err(usage)?,
                },
                Kind::Choose,
            ),
        };
        Ok(Question {
            core,
            threshold,
            model: self.model,
            kind,
        })
    }
}

/// A `choose` question under construction, one option of `C` at a time.
pub struct ChooseBuilder<C: Choice>(pub(super) Listing, pub(super) PhantomData<fn() -> C>);

impl<C: Choice> ChooseBuilder<C> {
    /// Add the next option of `C`, in `C::labels()` order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for an option out of order or repeated.
    pub fn option(self, value: C, description: Option<Description>) -> Result<Self, Error> {
        Ok(Self(self.0.next(&value, description)?, PhantomData))
    }

    /// Ask this model instead of the engine's.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank model or a second one.
    pub fn model(mut self, value: &str) -> Result<Self, Error> {
        model_of(&mut self.0.model, value)?;
        Ok(self)
    }

    /// Finish with no cut: the leading option wins unless two tie.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when an option of `C` is missing.
    pub fn build(self) -> Result<ChooseQuestion<C>, Error> {
        Ok(ChooseQuestion(
            self.0.close::<C>(Verb::Choose, None)?,
            PhantomData,
        ))
    }

    /// Finish under a cut the leading option must reach.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a bad cut or a missing option.
    pub fn cut_at(self, value: f64) -> Result<ChooseQuestion<C>, Error> {
        let rule = cut(value)?;
        Ok(ChooseQuestion(
            self.0.close::<C>(Verb::Choose, Some(rule))?,
            PhantomData,
        ))
    }
}

/// A `tag` question under construction, one label of `C` at a time.
pub struct TagBuilder<C: Choice>(pub(super) Listing, pub(super) PhantomData<fn() -> C>);

impl<C: Choice> TagBuilder<C> {
    /// Add the next label of `C`, in `C::labels()` order.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a label out of order or repeated.
    pub fn label(self, value: C, description: Option<Description>) -> Result<Self, Error> {
        Ok(Self(self.0.next(&value, description)?, PhantomData))
    }

    /// Ask this model instead of the engine's.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank model or a second one.
    pub fn model(mut self, value: &str) -> Result<Self, Error> {
        model_of(&mut self.0.model, value)?;
        Ok(self)
    }

    /// Finish under the default cut of 0.5.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when a label of `C` is missing.
    pub fn cut(self) -> Result<TagQuestion<C>, Error> {
        Ok(TagQuestion(
            self.0.close::<C>(Verb::Tag, Some(Threshold::default()))?,
            PhantomData,
        ))
    }

    /// Finish under this cut.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a bad cut or a missing label.
    pub fn cut_at(self, value: f64) -> Result<TagQuestion<C>, Error> {
        let rule = cut(value)?;
        Ok(TagQuestion(
            self.0.close::<C>(Verb::Tag, Some(rule))?,
            PhantomData,
        ))
    }
}

/// A `score` question under construction, one level at a time, lowest first.
#[derive(Debug)]
pub struct ScoreBuilder(pub(super) Listing);

impl ScoreBuilder {
    /// Add the next level. Give every level a description, or none.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] when the levels mix described and bare.
    pub fn level(mut self, name: &str, description: Option<Description>) -> Result<Self, Error> {
        let described = description.is_some();
        if self
            .0
            .labels
            .first()
            .is_some_and(|(_, held)| held.is_some() != described)
        {
            return Err(Error::usage("give every level a description, or none"));
        }
        let description = description.map(|held| held.core()).transpose()?;
        self.0.labels.push((name.to_owned(), description));
        Ok(self)
    }

    /// Ask this model instead of the engine's.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for a blank model or a second one.
    pub fn model(mut self, value: &str) -> Result<Self, Error> {
        model_of(&mut self.0.model, value)?;
        Ok(self)
    }

    /// Finish the question.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Usage`] for fewer than 2 or more than 10 levels, or a repeated one.
    pub fn build(self) -> Result<Question, Error> {
        let levels =
            Labels::levels(self.0.labels).map_err(|error| Error::usage(error.to_string()))?;
        Ok(Question {
            core: core::Question::Score {
                text: self.0.text,
                levels,
            },
            threshold: None,
            model: self.0.model,
            kind: Kind::Score,
        })
    }
}
