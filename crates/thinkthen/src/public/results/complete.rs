//! Typed complete judgment carriers over the canonical result/2 serializer.

use std::fmt;

use serde::{Serialize, Serializer};

use super::{NamedProbability, Probabilities, usage};
use crate::core::{self, AnswerId, ResultIdentity};
use crate::public::{Answer, Error, Usage};

macro_rules! complete {
    ($name:ident, $doc:literal, $value:ty) => {
        #[doc = $doc]
        #[derive(Clone, PartialEq)]
        pub struct $name {
            pub(crate) canonical: core::CompleteAtomic,
            pub(crate) value: $value,
        }

        impl $name {
            /// Borrow all actual result metadata without decoding JSON.
            #[must_use]
            pub fn meta(&self) -> super::ResultMetadata<'_> {
                super::ResultMetadata {
                    fields: self.canonical.metadata(),
                    identity: &self.canonical.identity,
                }
            }

            /// Stable identity of the resolved logical answer.
            #[must_use]
            pub const fn answer_id(&self) -> &AnswerId {
                self.canonical.identity.answer_id()
            }

            /// The aligned actual response sources and observation identities.
            #[must_use]
            pub const fn identity(&self) -> &ResultIdentity {
                &self.canonical.identity
            }

            /// Every declared probability, including unselected options.
            #[must_use]
            pub fn probabilities(&self) -> Probabilities {
                probabilities(self.canonical.answer())
            }

            /// Backend-supplied confidence; absence is retained.
            #[must_use]
            pub fn confidence(&self) -> Option<f64> {
                self.canonical.answer().confidence().map(|p| p.as_f64())
            }

            /// Independently reported counts; omitted dimensions stay unknown.
            #[must_use]
            pub fn reported_usage(&self) -> Option<crate::public::ReportedUsage> {
                self.canonical.reported_usage()
            }

            /// Reported usage for this result; missing usage stays absent.
            #[must_use]
            pub fn usage(&self) -> Option<Usage> {
                self.canonical.usage().map(usage)
            }

            /// Write the canonical complete result/2 document.
            ///
            /// # Errors
            /// Returns [`Error::Defect`] if a typed result cannot be serialized.
            pub fn to_json(&self) -> Result<String, Error> {
                core::json_line(&self.canonical)
                    .map_err(|_| Error::defect("a complete result could not be written as JSON"))
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("answer_id", self.answer_id())
                    .finish_non_exhaustive()
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.canonical.serialize(serializer)
            }
        }
    };
}

complete!(
    CompleteDecision,
    "One complete decide reading, with identity and all answer details.",
    Answer
);
complete!(
    CompleteChoice,
    "One complete choose reading, with the entire ordered distribution.",
    Option<String>
);
complete!(
    CompleteTags,
    "One complete tag reading, including every rejected label's probability.",
    Vec<String>
);
complete!(
    CompleteScore,
    "One complete score reading, retaining weighted value and distribution.",
    f64
);
complete!(
    CompleteFilter,
    "One filter occurrence, including rejected observations.",
    bool
);

impl CompleteDecision {
    /// The yes, no or unsure reading.
    #[must_use]
    pub const fn value(&self) -> Answer {
        self.value
    }
}

complete!(
    CompleteRank,
    "One ranked occurrence, with its one-based final position and retained judgment.",
    std::num::NonZeroUsize
);

impl CompleteRank {
    /// One-based final rank position, assigned before top truncation.
    #[must_use]
    pub const fn value(&self) -> usize {
        self.value.get()
    }
}

impl CompleteChoice {
    /// The selected label, or successful null.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

impl CompleteTags {
    /// Selected labels in declaration order.
    #[must_use]
    pub fn value(&self) -> &[String] {
        &self.value
    }
}

impl CompleteScore {
    /// The weighted position on the question's declared levels.
    #[must_use]
    pub const fn value(&self) -> f64 {
        self.value
    }
}

impl CompleteFilter {
    /// Whether this original occurrence was selected by the cut.
    #[must_use]
    pub const fn value(&self) -> bool {
        self.value
    }
}

pub(super) fn probabilities(answer: &core::Answer) -> Probabilities {
    match answer.yes() {
        Some(yes) => Probabilities::YesNo { yes },
        None => Probabilities::Named(
            answer
                .named()
                .unwrap_or_default()
                .into_iter()
                .map(|(name, probability)| NamedProbability {
                    name: name.to_owned(),
                    probability,
                })
                .collect(),
        ),
    }
}
