//! Native ordered complete annotation carriers, including failed members.

use serde::{Serialize, Serializer};
use std::fmt;

use crate::core::{self, AnswerId, FailureId, ResultIdentity};
use crate::public::{Error, FailureCause, Judgment, Probabilities};

/// One complete annotation with all ordered successful and failed members.
#[derive(Clone, PartialEq)]
pub struct CompleteAnnotated {
    pub(crate) canonical: core::CompleteAnnotation,
}

impl CompleteAnnotated {
    /// The stable root answer identity.
    #[must_use]
    pub const fn answer_id(&self) -> &AnswerId {
        self.canonical.identity.answer_id()
    }

    /// Actual sources and identities in logical question order.
    #[must_use]
    pub const fn identity(&self) -> &ResultIdentity {
        &self.canonical.identity
    }

    /// Every named member, including failures, in question-set order.
    pub fn members(&self) -> impl ExactSizeIterator<Item = CompleteAnnotationMember<'_>> {
        self.canonical
            .members
            .iter()
            .map(|(name, canonical)| CompleteAnnotationMember { name, canonical })
    }

    /// The complete canonical result/2 document.
    ///
    /// # Errors
    /// Returns a defect if a typed annotation cannot be serialized.
    pub fn to_json(&self) -> Result<String, Error> {
        core::json_line(&self.canonical)
            .map_err(|_| Error::defect("a complete annotation could not be written as JSON"))
    }
}

impl fmt::Debug for CompleteAnnotated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteAnnotated")
            .field("answer_id", self.answer_id())
            .finish_non_exhaustive()
    }
}

impl Serialize for CompleteAnnotated {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize(serializer)
    }
}

/// One borrowed named success or failure in a complete annotation.
pub struct CompleteAnnotationMember<'a> {
    name: &'a str,
    canonical: &'a core::CompleteAnnotationMember,
}

impl CompleteAnnotationMember<'_> {
    /// The member name in the authored question set.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name
    }

    /// The successful member's ID, including successful null.
    #[must_use]
    pub fn answer_id(&self) -> Option<&AnswerId> {
        match &self.canonical.identity {
            core::MemberIdentity::Answered(id) => Some(id),
            core::MemberIdentity::Failed(_) => None,
        }
    }

    /// The failed occurrence's ID, absent for successful null.
    #[must_use]
    pub fn failure_id(&self) -> Option<&FailureId> {
        match &self.canonical.identity {
            core::MemberIdentity::Failed(id) => Some(id),
            core::MemberIdentity::Answered(_) => None,
        }
    }

    /// A successful typed value; a failure never supplies a value.
    #[must_use]
    pub fn value(&self) -> Option<Judgment> {
        self.canonical.value().map(super::judgment)
    }

    /// All successful question probabilities, including rejected labels.
    #[must_use]
    pub fn probabilities(&self) -> Option<Probabilities> {
        self.canonical
            .legacy
            .answered()
            .map(|(_, answer, _, _)| super::complete::probabilities(answer))
    }

    /// Backend-provided confidence, when supplied for this successful member.
    #[must_use]
    pub fn confidence(&self) -> Option<f64> {
        self.canonical
            .legacy
            .answered()
            .and_then(|(_, answer, _, _)| answer.confidence())
            .map(|p| p.as_f64())
    }

    /// The typed failed-member cause, absent for a successful member.
    #[must_use]
    pub fn failure(&self) -> Option<FailureCause> {
        self.canonical.legacy.failed().map(|(_, failure, _)| {
            crate::public::annotated::cause(core::FailedValue::new(failure).cause())
        })
    }

    /// The saved question key associated with this logical member.
    #[must_use]
    pub fn request(&self) -> &str {
        self.canonical.request()
    }
}

impl fmt::Debug for CompleteAnnotationMember<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteAnnotationMember")
            .field("answer_id", &self.answer_id())
            .field("failure_id", &self.failure_id())
            .finish_non_exhaustive()
    }
}
