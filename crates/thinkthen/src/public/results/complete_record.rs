//! Original occurrences remain associated with their complete readings.

use std::fmt;

/// One original occurrence and its complete result; no original trait bounds.
#[derive(Clone, PartialEq)]
pub struct CompleteRecord<T, R> {
    pub(crate) original: T,
    pub(crate) ordinal: usize,
    pub(crate) result: R,
}

impl<T, R> CompleteRecord<T, R> {
    /// The exact original item retained by this occurrence.
    #[must_use]
    pub const fn original(&self) -> &T {
        &self.original
    }

    /// The original zero-based ordinal, before filtering, sorting or splitting.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// The complete typed judgment for this occurrence.
    #[must_use]
    pub const fn result(&self) -> &R {
        &self.result
    }

    /// Retain ownership of both the original and complete result.
    #[must_use]
    pub fn into_parts(self) -> (T, R) {
        (self.original, self.result)
    }
}

impl<T, R> fmt::Debug for CompleteRecord<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompleteRecord")
            .field("ordinal", &self.ordinal)
            .finish_non_exhaustive()
    }
}

macro_rules! serialize_atomic {
    ($($result:ident),+ $(,)?) => { $(
        impl<T: serde::Serialize> serde::Serialize for CompleteRecord<T, super::$result> {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.result.canonical.serialize_with_input(Some(&self.original), serializer)
            }
        }
    )+ };
}
serialize_atomic!(
    CompleteDecision,
    CompleteChoice,
    CompleteTags,
    CompleteScore,
    CompleteFilter,
    CompleteRank,
    CompleteRecognized
);
impl<T: serde::Serialize> serde::Serialize for CompleteRecord<T, super::CompleteAnnotated> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.result
            .canonical
            .serialize_with_input(&self.original, serializer)
    }
}

impl<T: serde::Serialize> serde::Serialize for CompleteRecord<T, super::CompleteRelated> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.result
            .serialize_input(Some(&self.original), serializer)
    }
}
