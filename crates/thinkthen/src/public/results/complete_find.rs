//! Typed complete find retains caller-owned originals without extra trait bounds.

use std::fmt;

use serde::{Serialize, Serializer};

use crate::core::{self, AnswerId, ResultIdentity};
use crate::public::{Candidate, Error, Found, Usage};

/// A complete whole-set find with every candidate and its probability.
#[derive(Clone, PartialEq)]
pub struct CompleteFound<T> {
    pub(crate) canonical: core::CompleteFind,
    pub(crate) found: Found<T>,
}

impl<T> CompleteFound<T> {
    /// Stable identity of this whole-set reading.
    #[must_use]
    pub const fn answer_id(&self) -> &AnswerId {
        self.canonical.identity.answer_id()
    }

    /// Actual response sources and observation identities.
    #[must_use]
    pub const fn identity(&self) -> &ResultIdentity {
        &self.canonical.identity
    }

    /// The selected original, or successful null.
    #[must_use]
    pub fn selected(&self) -> Option<&T> {
        self.found.selected()
    }

    /// Every original candidate, followed by the optional synthetic none.
    #[must_use]
    pub fn candidates(&self) -> &[Candidate<T>] {
        self.found.candidates()
    }

    /// Backend-reported confidence, when supplied.
    #[must_use]
    pub fn confidence(&self) -> Option<f64> {
        self.canonical.confidence()
    }

    /// Independently reported counts; omitted dimensions stay unknown.
    #[must_use]
    pub fn reported_usage(&self) -> Option<crate::public::ReportedUsage> {
        self.canonical.reported_usage()
    }

    /// Reported usage, retaining absence.
    #[must_use]
    pub fn usage(&self) -> Option<Usage> {
        self.canonical.usage().map(super::usage)
    }

    /// Consume the result and return its selected original.
    #[must_use]
    pub fn into_selected(self) -> Option<T> {
        self.found.into_selected()
    }

    /// Write the canonical result/2 document.
    ///
    /// # Errors
    /// Returns a defect if the typed result cannot be serialized.
    pub fn to_json(&self) -> Result<String, Error> {
        core::json_line(&self.canonical)
            .map_err(|_| Error::defect("a complete find could not be written as JSON"))
    }
}

impl<T> fmt::Debug for CompleteFound<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteFound")
            .field("answer_id", self.answer_id())
            .finish_non_exhaustive()
    }
}

impl<T> Serialize for CompleteFound<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize(serializer)
    }
}
