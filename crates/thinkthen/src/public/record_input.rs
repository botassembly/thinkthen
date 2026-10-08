//! Independent per-record context and ordered choose candidates.

use std::fmt;

use super::{Description, Error, Question, RecordContext};
use crate::core::{self, Labels, Pointer};

/// One original item with optional context and replacement choose candidates.
/// `None` context falls back to the call context; explicit empty text suppresses it.
#[derive(Clone, PartialEq)]
pub struct RecordInput<T> {
    /// The original evidence, retained without cloning or serialization.
    pub original: T,
    /// Exact per-record context, or the call's fallback when absent.
    pub context: Option<RecordContext>,
    /// The entire ordered replacement shortlist, or fixed question options.
    pub options: Option<RecordOptions>,
    /// Replacement recognition examples; an empty list suppresses shared examples.
    pub examples: Option<Vec<super::RecognitionExample>>,
}

impl<T> RecordInput<T> {
    /// Transform the original while retaining every independent per-record control.
    #[must_use]
    pub fn map_original<U>(self, map: impl FnOnce(T) -> U) -> RecordInput<U> {
        RecordInput {
            original: map(self.original),
            context: self.context,
            options: self.options,
            examples: self.examples,
        }
    }
}

impl<T> fmt::Debug for RecordInput<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordInput")
            .field("context", &self.context.as_ref().map(|_| "<withheld>"))
            .field("options", &self.options)
            .field("examples", &self.examples.as_ref().map(Vec::len))
            .finish_non_exhaustive()
    }
}

/// One runtime choose option with caller-authored description content.
#[derive(Clone, PartialEq)]
pub struct RecordOption {
    /// The exact option name.
    pub name: String,
    /// Optional description; accepted JSON content is preserved in order.
    pub description: Option<Description>,
}

impl fmt::Debug for RecordOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordOption").finish_non_exhaustive()
    }
}

/// A validated, nonempty ordered shortlist for one choose record.
#[derive(Clone, PartialEq)]
pub struct RecordOptions {
    options: Vec<RecordOption>,
}

impl fmt::Debug for RecordOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecordOptions")
            .field("len", &self.options.len())
            .finish()
    }
}

impl RecordOptions {
    /// Validate names, descriptions, order, uniqueness and choose limits.
    ///
    /// # Errors
    /// Returns [`Error::Usage`] for an invalid shortlist.
    pub fn new(options: Vec<RecordOption>) -> Result<Self, Error> {
        let mut builder = Question::choose_labels("record options")?;
        for option in &options {
            builder = builder.label(&option.name, option.description.clone())?;
        }
        builder.build()?;
        Ok(Self { options })
    }

    /// Project a shortlist from JSON at an existing record pointer.
    ///
    /// # Errors
    /// Returns [`Error::Usage`] for invalid JSON, pointer or candidates.
    pub fn project(record: &str, pointer: &str) -> Result<Self, Error> {
        let pointer = Pointer::new(pointer).map_err(Error::refused)?;
        let labels =
            core::record_input::project_options(record, &pointer).map_err(Error::refused)?;
        Self::from_labels(&labels)
    }

    pub(crate) fn replacing(&self, question: &Question) -> Result<Question, Error> {
        let core::Question::Choose { text, .. } = &question.core else {
            return Err(Error::usage("record options are admitted only for choose"));
        };
        let mut question = question.clone();
        question.core = core::Question::Choose {
            text: text.clone(),
            options: self.labels()?,
        };
        Ok(question)
    }

    pub(crate) fn labels(&self) -> Result<Labels, Error> {
        let labels = self
            .options
            .iter()
            .map(|option| {
                Ok((
                    option.name.clone(),
                    option
                        .description
                        .as_ref()
                        .map(Description::core)
                        .transpose()?,
                ))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Labels::described(labels).map_err(Error::refused)
    }

    /// Borrow all options in the order they will be sent.
    #[must_use]
    pub fn options(&self) -> &[RecordOption] {
        &self.options
    }

    pub(crate) fn from_labels(labels: &Labels) -> Result<Self, Error> {
        let options = labels
            .descriptions()
            .map(option_from_label)
            .collect::<Result<Vec<_>, Error>>()?;
        Self::new(options)
    }
}

impl Description {
    /// Parse caller-authored description JSON using the shared ordered parser.
    ///
    /// # Errors
    /// Returns [`Error::Usage`] unless the value is text, an object, a list or null.
    pub fn from_json(text: &str) -> Result<Self, Error> {
        let value = core::Json::parse(text).map_err(Error::refused)?;
        if core::Description::of_json(&value).is_none() {
            return Err(Error::usage(
                "a description is text, an object, a list or null",
            ));
        }
        Self::of(value)
    }
}

fn option_from_label(
    (name, description): (&String, Option<&core::Description>),
) -> Result<RecordOption, Error> {
    Ok(RecordOption {
        name: name.clone(),
        description: description.map(description_from_core).transpose()?,
    })
}

fn description_from_core(description: &core::Description) -> Result<Description, Error> {
    Description::of(description.as_json().clone())
}
