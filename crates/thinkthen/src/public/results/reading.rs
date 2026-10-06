//! Typed borrowed resolved questions and authored content, independent of JSON protocols.
use crate::core;
use crate::public::{Error, QuestionKind};
use serde::{Serialize, Serializer};
use std::fmt;

/// Caller-authored content in its original ordered native representation.
#[derive(Clone, Copy)]
pub struct QuestionContent<'a>(pub(crate) &'a core::Json);
impl QuestionContent<'_> {
    /// Literal text, absent when the caller authored another admitted shape.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.0.as_str()
    }
    /// Whether this is an explicitly authored null.
    #[must_use]
    pub const fn is_null(&self) -> bool {
        matches!(self.0, core::Json::Null)
    }
    /// Preserve arbitrary authored content and map order as JSON.
    /// # Errors
    /// Returns a defect if native content cannot be serialized.
    pub fn to_json(&self) -> Result<String, Error> {
        core::json_line(self.0).map_err(|_| Error::defect("authored content could not be written"))
    }
}
impl Serialize for QuestionContent<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// The effective reading rule; null is represented by an absent rule.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResolvedThreshold {
    /// Accept at or above this probability.
    Cut(f64),
    /// Below low is no, at or above high is yes, and the middle is unsure.
    Band {
        /// Lower inclusive unsure edge.
        low: f64,
        /// Upper inclusive yes edge.
        high: f64,
    },
}
impl ResolvedThreshold {
    pub(crate) const fn of(value: core::Threshold) -> Self {
        match value.bounds() {
            (low, Some(high)) => Self::Band { low, high },
            (cut, None) => Self::Cut(cut),
        }
    }
}

/// A declared label or level and its optional authored description.
#[derive(Clone, Copy)]
pub struct ResolvedOption<'a> {
    pub(crate) name: &'a str,
    pub(crate) description: Option<QuestionContent<'a>>,
}
impl ResolvedOption<'_> {
    /// Exact declared name.
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }
    /// Original description, distinguishing absence from authored null.
    #[must_use]
    pub const fn description(&self) -> Option<QuestionContent<'_>> {
        self.description
    }
}

/// The normalized atomic question that actually supplied this reading.
#[derive(Clone, Copy)]
pub struct ResolvedQuestion<'a>(pub(crate) &'a core::Question);
impl ResolvedQuestion<'_> {
    /// The actual primitive question kind, including score used by rank.
    #[must_use]
    pub fn kind(&self) -> QuestionKind {
        QuestionKind::of(self.0)
    }
    /// Original question content, including structured authored questions.
    #[must_use]
    pub fn text(&self) -> QuestionContent<'_> {
        let text = match self.0 {
            core::Question::Decide { text, .. }
            | core::Question::Choose { text, .. }
            | core::Question::Tag { text, .. }
            | core::Question::Score { text, .. } => text,
        };
        QuestionContent(text.as_json())
    }
    /// Declared options or levels in original order; decide has none.
    pub fn options(&self) -> impl Iterator<Item = ResolvedOption<'_>> {
        let labels = match self.0 {
            core::Question::Decide { .. } => None,
            core::Question::Choose { options, .. } => Some(options),
            core::Question::Tag { labels, .. } => Some(labels),
            core::Question::Score { levels, .. } => Some(levels),
        };
        labels
            .into_iter()
            .flat_map(core::Labels::descriptions)
            .map(|(name, description)| ResolvedOption {
                name,
                description: description.map(|d| QuestionContent(d.as_json())),
            })
    }
    /// What yes means, distinguishing absent from explicitly authored null.
    #[must_use]
    pub fn yes(&self) -> Option<QuestionContent<'_>> {
        match self.0 {
            core::Question::Decide { yes, .. } => {
                yes.as_ref().map(|v| QuestionContent(v.as_json()))
            }
            _ => None,
        }
    }
    /// What no means, distinguishing absent from explicitly authored null.
    #[must_use]
    pub fn no(&self) -> Option<QuestionContent<'_>> {
        match self.0 {
            core::Question::Decide { no, .. } => no.as_ref().map(|v| QuestionContent(v.as_json())),
            _ => None,
        }
    }
}

/// Whole-set find's actual normalized question.
#[derive(Clone, Copy)]
pub struct FindReading<'a> {
    pub(crate) text: &'a core::QuestionText,
    pub(crate) none: bool,
    pub(crate) profile: Option<&'a str>,
}
impl FindReading<'_> {
    /// Authored calibration profile, independent of runtime route limits.
    #[must_use]
    pub const fn profile(&self) -> Option<&str> {
        self.profile
    }
    /// Authored selection question.
    #[must_use]
    pub fn text(&self) -> QuestionContent<'_> {
        QuestionContent(self.text.as_json())
    }
    /// Whether the actual candidate set included the explicit none option.
    #[must_use]
    pub const fn offers_none(&self) -> bool {
        self.none
    }
}

macro_rules! withheld {
    ($($name:ident),+) => { $(impl fmt::Debug for $name<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct(stringify!($name)).finish_non_exhaustive() }
    })+ };
}
withheld!(
    QuestionContent,
    ResolvedOption,
    ResolvedQuestion,
    FindReading
);
