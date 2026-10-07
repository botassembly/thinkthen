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
pub struct ResolvedQuestion<'a>(
    pub(crate) &'a core::Question,
    pub(crate) Option<&'a core::declaration::QuestionMetadata>,
);
impl ResolvedQuestion<'_> {
    /// Explicit authored model; absent when the engine supplied its default.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        self.1
            .and_then(|metadata| metadata.reading.model.as_deref())
    }
    /// Explicit authored calibration profile.
    #[must_use]
    pub fn profile(&self) -> Option<&str> {
        self.1
            .and_then(|metadata| metadata.reading.profile.as_deref())
    }
    /// Explicit authored request batch setting.
    #[must_use]
    pub fn batch(&self) -> Option<crate::public::BatchSetting> {
        self.1
            .and_then(|metadata| metadata.reading.batch)
            .map(batch_setting)
    }
    /// Authored evidence pointers in their original order.
    pub fn on(&self) -> impl Iterator<Item = &str> {
        self.1
            .into_iter()
            .flat_map(|metadata| metadata.reading.on.iter())
            .map(String::as_str)
    }
    /// Optional author name; independent of accepted observation provenance.
    #[must_use]
    pub fn name(&self) -> Option<&crate::public::QuestionName> {
        self.1.and_then(|m| m.name.as_ref())
    }
    /// Optional author wording version.
    #[must_use]
    pub fn wording_version(&self) -> Option<crate::public::WordingVersion> {
        self.1.and_then(|m| m.wording_version)
    }
    /// Optional declaration of the effective typed item.
    #[must_use]
    pub fn item_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.1.and_then(|m| m.item_schema.as_ref())
    }
    /// Optional declaration of explicit per-item context.
    #[must_use]
    pub fn context_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.1.and_then(|m| m.context_schema.as_ref())
    }

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
    pub(crate) metadata: &'a core::declaration::QuestionMetadata,
    pub(crate) text: &'a core::QuestionText,
    pub(crate) none: bool,
    pub(crate) profile: Option<&'a str>,
}
impl FindReading<'_> {
    /// Explicit authored model, independent of the answering model.
    #[must_use]
    pub fn model(&self) -> Option<&str> {
        self.metadata.reading.model.as_deref()
    }
    /// Original authored evidence pointers.
    pub fn on(&self) -> impl Iterator<Item = &str> {
        self.metadata.reading.on.iter().map(String::as_str)
    }

    /// Optional author name; independent of accepted observation provenance.
    #[must_use]
    pub fn name(&self) -> Option<&crate::public::QuestionName> {
        self.metadata.name.as_ref()
    }
    /// Optional author wording version.
    #[must_use]
    pub fn wording_version(&self) -> Option<crate::public::WordingVersion> {
        self.metadata.wording_version
    }
    /// Optional declaration of the effective typed item.
    #[must_use]
    pub fn item_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.metadata.item_schema.as_ref()
    }
    /// Optional declaration of explicit per-item context.
    #[must_use]
    pub fn context_schema(&self) -> Option<&crate::public::InputDeclaration> {
        self.metadata.context_schema.as_ref()
    }

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

fn batch_setting(value: core::Setting) -> crate::public::BatchSetting {
    match value {
        core::Setting::Max => crate::public::BatchSetting::Max,
        core::Setting::Records(count) => crate::public::BatchSetting::Records(count),
    }
}
