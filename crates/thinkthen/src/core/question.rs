//! The question a judgment asks, as one verb over the text it was given.

use serde::{Serialize, Serializer};
use thiserror::Error;

use crate::core::text::{Description, Meaning, QuestionText, Withheld};

/// Why a list of options or levels is not one the verb takes.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub(crate) enum LabelsError {
    /// Fewer than two options, or more than the verb's ceiling.
    #[error("`choose` takes 2 to 255 options")]
    OptionCount,
    /// Fewer than two levels, or more than the verb's ceiling.
    #[error("`score` takes 2 to 10 levels, lowest first")]
    LevelCount,
    /// Fewer than one tag, or more than the public ceiling.
    #[error("`tag` takes 1 to 20 labels")]
    TagCount,
    /// An option is empty or holds only white space.
    #[error("an option is text, not white space")]
    OptionBlank,
    /// A tag label is empty or holds only white space.
    #[error("a label is text, not white space")]
    TagBlank,
    /// A level is empty or holds only white space.
    #[error("a level is text, not white space")]
    LevelBlank,
    /// One option was given twice.
    #[error("a list holds each option once")]
    OptionDuplicate,
    /// One tag label was given twice.
    #[error("a list holds each label once")]
    TagDuplicate,
    /// One level was given twice.
    #[error("a list holds each level once")]
    LevelDuplicate,
    /// An option holds a control character.
    #[error("an option is one line of printable text")]
    OptionControl,
    /// A tag label holds a control character.
    #[error("a label is one line of printable text")]
    TagControl,
    /// A level holds a control character.
    #[error("a level is one line of printable text")]
    LevelControl,
}

#[derive(Clone, Copy)]
enum LabelKind {
    Option,
    Tag,
    Level,
}

impl LabelKind {
    const fn blank(self) -> LabelsError {
        match self {
            Self::Option => LabelsError::OptionBlank,
            Self::Tag => LabelsError::TagBlank,
            Self::Level => LabelsError::LevelBlank,
        }
    }

    const fn duplicate(self) -> LabelsError {
        match self {
            Self::Option => LabelsError::OptionDuplicate,
            Self::Tag => LabelsError::TagDuplicate,
            Self::Level => LabelsError::LevelDuplicate,
        }
    }

    const fn control(self) -> LabelsError {
        match self {
            Self::Option => LabelsError::OptionControl,
            Self::Tag => LabelsError::TagControl,
            Self::Level => LabelsError::LevelControl,
        }
    }
}

/// The most options `choose` picks between, by ADR 0007.
const MOST_OPTIONS: usize = 255;

/// The most levels `score` places on, by ADR 0007.
const MOST_LEVELS: usize = 10;

/// The most independent labels one `tag` request carries.
const MOST_TAGS: usize = 20;

/// One label, with the description that rides beside it on the wire.
///
/// The command line carries labels alone, so a positional option has no
/// description. A record that holds a map from label to description gives one.
#[derive(Clone, Eq, PartialEq)]
struct Label {
    name: String,
    description: Option<Description>,
}

/// The options `choose` picks from, or the levels `score` places on.
///
/// The order is the user's own. The tool never reorders a list, because option
/// order moves the odds and a run with a changed list is a different
/// measurement.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[cfg_attr(test, schemars(inline, with = "Vec<String>"))]
pub(crate) struct Labels(Vec<Label>);

/// `choose --options` reads labels from a record, and a record is evidence.
/// A list does not know where it came from, so `Debug` withholds every name.
impl std::fmt::Debug for Labels {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let length = self.names().map(String::len).sum();
        formatter
            .debug_tuple("Labels")
            .field(&Withheld(length))
            .finish()
    }
}

impl Serialize for Labels {
    /// Write the names alone, so a result names the labels it was asked with.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.names())
    }
}

impl Labels {
    /// Take 2 to 255 option labels, in the order the user gave them.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] when the list is too short or too long, when a
    /// label is blank, or when one label was given twice.
    pub(crate) fn options(values: Vec<String>) -> Result<Self, LabelsError> {
        Self::checked(
            bare(values),
            2,
            MOST_OPTIONS,
            LabelsError::OptionCount,
            LabelKind::Option,
        )
    }

    /// Take 2 to 255 option labels, each with the description a record gave it.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] on the same four counts `options` does.
    ///
    /// A description that is blank is no description, so a list of names, a map
    /// whose values are `null`, and a map whose values are white space all name
    /// the same question. `question-file.md` writes that rule out.
    pub(crate) fn described(
        values: Vec<(String, Option<Description>)>,
    ) -> Result<Self, LabelsError> {
        let labels = values
            .into_iter()
            .map(|(name, description)| Label {
                name,
                description: description.filter(|held| !held.blank()),
            })
            .collect();
        Self::checked(
            labels,
            2,
            MOST_OPTIONS,
            LabelsError::OptionCount,
            LabelKind::Option,
        )
    }

    /// Validate an internal complete choice menu without a public count ceiling.
    pub(crate) fn recognition_menu(
        values: Vec<(String, Option<Description>)>,
    ) -> Result<Self, LabelsError> {
        let labels = values
            .into_iter()
            .map(|(name, description)| Label {
                name,
                description: description.filter(|held| !held.blank()),
            })
            .collect();
        Self::checked(
            labels,
            2,
            usize::MAX,
            LabelsError::OptionCount,
            LabelKind::Option,
        )
    }

    /// Take 1 to 20 tag labels with their optional descriptions.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] when the count or one label is invalid.
    pub(crate) fn tags(values: Vec<(String, Option<Description>)>) -> Result<Self, LabelsError> {
        let labels = values
            .into_iter()
            .map(|(name, description)| Label {
                name,
                description: description.filter(|held| !held.blank()),
            })
            .collect();
        Self::checked(labels, 1, MOST_TAGS, LabelsError::TagCount, LabelKind::Tag)
    }

    /// Take 2 to 10 levels, lowest first, with the description each map member
    /// held. A `null` the map named stays a description of `null`, so the model
    /// reads the null and never the level's name.
    ///
    /// # Errors
    ///
    /// Returns [`LabelsError`] when the list is too short or too long, when a
    /// level is blank, or when one level was given twice.
    pub(crate) fn levels(values: Vec<(String, Option<Description>)>) -> Result<Self, LabelsError> {
        let labels = values
            .into_iter()
            .map(|(name, description)| Label { name, description })
            .collect();
        Self::checked(
            labels,
            2,
            MOST_LEVELS,
            LabelsError::LevelCount,
            LabelKind::Level,
        )
    }

    /// Take a list that is long enough, short enough, filled, and unrepeated.
    fn checked(
        values: Vec<Label>,
        least: usize,
        most: usize,
        count: LabelsError,
        kind: LabelKind,
    ) -> Result<Self, LabelsError> {
        if values.len() < least || values.len() > most {
            return Err(count);
        }
        if values.iter().any(|value| value.name.trim().is_empty()) {
            return Err(kind.blank());
        }
        // `--raw` prints a label byte for byte, and `--options` lets a record
        // the tool did not write supply one. A label carrying a line feed or
        // an escape would then write a line of its own into the caller's
        // output. The message never quotes the label, because a record is
        // evidence.
        if values
            .iter()
            .any(|value| value.name.chars().any(char::is_control))
        {
            return Err(kind.control());
        }
        for (place, value) in values.iter().enumerate() {
            if values
                .iter()
                .skip(place + 1)
                .any(|other| other.name == value.name)
            {
                return Err(kind.duplicate());
            }
        }
        Ok(Self(values))
    }

    /// Read the names back, in the order they were given.
    pub(crate) fn names(&self) -> impl Iterator<Item = &String> {
        self.0.iter().map(|label| &label.name)
    }

    /// Read each name with its description, in the order they were given.
    pub(crate) fn descriptions(&self) -> impl Iterator<Item = (&String, Option<&Description>)> {
        self.0
            .iter()
            .map(|label| (&label.name, label.description.as_ref()))
    }

    /// True when every label carries a description, as a `score` map does.
    pub(crate) fn fully_described(&self) -> bool {
        self.0.iter().all(|label| label.description.is_some())
    }

    /// How many labels the list holds.
    pub(crate) fn count(&self) -> usize {
        self.0.len()
    }
}

/// Take labels the command line gave, which carry no description.
fn bare(values: Vec<String>) -> Vec<Label> {
    values
        .into_iter()
        .map(|name| Label {
            name,
            description: None,
        })
        .collect()
}

/// What the judgment was asked, and which of the three shapes it takes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "verb", rename_all = "snake_case")]
#[cfg_attr(test, derive(schemars::JsonSchema), schemars(rename = "question"))]
pub(crate) enum Question {
    /// Ask whether the question holds for the evidence.
    Decide {
        /// The question the model receives.
        text: QuestionText,
        /// What a yes means, when the user said so. Absent by default.
        #[serde(rename = "true", skip_serializing_if = "Option::is_none")]
        yes: Option<Meaning>,
        /// What a no means, when the user said so. Absent by default.
        #[serde(rename = "false", skip_serializing_if = "Option::is_none")]
        no: Option<Meaning>,
    },
    /// Ask which of a fixed list of labels fits the evidence.
    Choose {
        /// The question the model receives.
        text: QuestionText,
        /// The labels to pick between, in the order the user gave them.
        options: Labels,
    },
    /// Ask which of several independent labels apply.
    Tag {
        /// The question that frames the labels.
        text: QuestionText,
        /// The labels tested independently, in user order.
        labels: Labels,
    },
    /// Ask where on a list of named levels the evidence sits.
    Score {
        /// The question the model receives.
        text: QuestionText,
        /// The levels, lowest first.
        levels: Labels,
    },
}

#[cfg(test)]
mod tests;
