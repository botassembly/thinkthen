//! Complete native recognition with all staged probability tables.

use crate::core::{self, AnswerId, ResultIdentity};
use crate::public::{Error, NamedProbability, Recognized, Usage};
use serde::{Serialize, Serializer};
use std::fmt;

/// A complete recognized value and the distributions behind it.
#[derive(Clone, PartialEq)]
pub struct CompleteRecognized {
    pub(crate) canonical: core::CompleteRecognition,
    pub(crate) value: Recognized,
}

impl CompleteRecognized {
    /// Borrow actual metadata without decoding a result document.
    #[must_use]
    pub fn meta(&self) -> super::ResultMetadata<'_> {
        super::ResultMetadata {
            fields: self.canonical.metadata(),
            identity: &self.canonical.identity,
        }
    }

    /// Stable identity of the complete staged reading.
    #[must_use]
    pub const fn answer_id(&self) -> &AnswerId {
        self.canonical.identity.answer_id()
    }

    /// The actionable names and optional relations.
    #[must_use]
    pub const fn value(&self) -> &Recognized {
        &self.value
    }

    /// Sources in stage and logical question order.
    #[must_use]
    pub const fn identity(&self) -> &ResultIdentity {
        &self.canonical.identity
    }

    /// All piece, name and relation probabilities retained by execution.
    #[must_use]
    pub fn probabilities(&self) -> RecognitionProbabilities<'_> {
        RecognitionProbabilities(&self.canonical.answer)
    }

    /// Independently reported counts; omitted dimensions stay unknown.
    #[must_use]
    pub fn reported_usage(&self) -> Option<crate::public::ReportedUsage> {
        self.canonical.reported_usage()
    }

    /// Backend-reported usage, retaining absence.
    #[must_use]
    pub fn usage(&self) -> Option<Usage> {
        self.canonical.usage().map(super::usage)
    }

    /// Write the complete canonical result/2 document.
    ///
    /// # Errors
    /// Returns a defect if the typed result cannot be serialized.
    pub fn to_json(&self) -> Result<String, Error> {
        core::json_line(&self.canonical)
            .map_err(|_| Error::defect("a complete recognition could not be written as JSON"))
    }
}

impl fmt::Debug for CompleteRecognized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteRecognized")
            .field("answer_id", self.answer_id())
            .finish_non_exhaustive()
    }
}

impl Serialize for CompleteRecognized {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize(serializer)
    }
}

/// Borrowed probability tables in their original stage order.
pub struct RecognitionProbabilities<'a>(&'a core::RecognitionOdds);

impl RecognitionProbabilities<'_> {
    /// Every input piece's boundary tag distribution.
    pub fn pieces(&self) -> impl ExactSizeIterator<Item = PieceProbabilities<'_>> {
        self.0.pieces.iter().map(PieceProbabilities)
    }

    /// Every step-one name's kind and edge distributions.
    pub fn names(&self) -> impl ExactSizeIterator<Item = NameProbabilities<'_>> {
        self.0.names.iter().map(NameProbabilities)
    }

    /// Every asked relation pair, including rejected pairs.
    pub fn pairs(&self) -> impl ExactSizeIterator<Item = PairProbability<'_>> {
        self.0.pairs.iter().map(PairProbability)
    }
}

/// One piece's scalar offsets and full tag distribution.
pub struct PieceProbabilities<'a>(&'a core::PieceOdds);

impl PieceProbabilities<'_> {
    /// Half-open Unicode scalar range.
    #[must_use]
    pub fn range(&self) -> std::ops::Range<usize> {
        self.0.start..self.0.end
    }

    /// All declared tags, in table order.
    #[must_use]
    pub fn tags(&self) -> Vec<NamedProbability> {
        named(&self.0.tags)
    }
}

/// One found name's complete kind and edge option distributions.
pub struct NameProbabilities<'a>(&'a core::NameOdds);

impl NameProbabilities<'_> {
    /// Half-open Unicode scalar range before edge adjustment.
    #[must_use]
    pub fn range(&self) -> std::ops::Range<usize> {
        self.0.start..self.0.end
    }

    /// Declared kinds, absent when none were asked.
    #[must_use]
    pub fn kinds(&self) -> Option<Vec<NamedProbability>> {
        self.0.kinds.as_ref().map(named)
    }

    /// Full edge options, absent when no edge question was asked.
    #[must_use]
    pub fn edges(&self) -> Option<Vec<NamedProbability>> {
        self.0.edges.as_ref().map(named)
    }
}

/// One relation pair's name, scalar endpoints and probability.
pub struct PairProbability<'a>(&'a core::PairOdds);

impl PairProbability<'_> {
    /// The authored relation name.
    #[must_use]
    pub fn relation(&self) -> &str {
        &self.0.relation
    }

    /// Source name's half-open scalar range.
    #[must_use]
    pub fn source(&self) -> std::ops::Range<usize> {
        self.0.source.start..self.0.source.end
    }

    /// Target name's half-open scalar range.
    #[must_use]
    pub fn target(&self) -> std::ops::Range<usize> {
        self.0.target.start..self.0.target.end
    }

    /// Probability of the asked relation, including a rejected pair.
    #[must_use]
    pub fn probability(&self) -> f64 {
        self.0.probability
    }
}

fn named(odds: &core::Odds) -> Vec<NamedProbability> {
    odds.0
        .iter()
        .map(|(name, probability)| NamedProbability {
            name: name.clone(),
            probability: *probability,
        })
        .collect()
}

macro_rules! withheld {
    ($($name:ident),+ $(,)?) => { $(
        impl fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name)).finish_non_exhaustive()
            }
        }
    )+ };
}

withheld!(
    RecognitionProbabilities,
    PieceProbabilities,
    NameProbabilities,
    PairProbability
);
