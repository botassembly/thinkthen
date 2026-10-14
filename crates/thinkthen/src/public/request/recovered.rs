//! Independent native row outcomes without changing ordinary request packets.
use super::RequestValue;
use crate::Error;
use serde::{Serialize, Serializer, ser::SerializeStruct};

/// One completed input or whole-set invocation and its safe outcome.
#[derive(Debug)]
pub struct RequestRecoveredRow {
    pub(crate) ordinal: usize,
    pub(crate) outcome: Result<RequestValue, Error>,
}
impl RequestRecoveredRow {
    /// Original zero-based input ordinal; a whole-set aggregate uses zero.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Complete native records or the original safe row failure.
    /// # Errors
    /// Returns the retained recoverable failure for this occurrence.
    pub fn outcome(&self) -> Result<&RequestValue, &Error> {
        self.outcome.as_ref()
    }
}
impl Serialize for RequestRecoveredRow {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut document = serializer.serialize_struct("RequestRecoveredRow", 2)?;
        document.serialize_field("index", &self.ordinal)?;
        match &self.outcome {
            Ok(value) => document.serialize_field("results", value)?,
            Err(error) => document.serialize_field("failure", error.complete().error())?,
        }
        document.end()
    }
}

pub(crate) fn fatal(error: &Error) -> bool {
    matches!(
        error.kind(),
        crate::ErrorKind::Cancelled | crate::ErrorKind::Deadline | crate::ErrorKind::Defect
    )
}

pub(crate) fn row(
    ordinal: usize,
    outcome: Result<RequestValue, Error>,
) -> Result<RequestRecoveredRow, Error> {
    match outcome {
        Err(error) if fatal(&error) => Err(error),
        outcome => Ok(RequestRecoveredRow { ordinal, outcome }),
    }
}
