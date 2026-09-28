//! Fixed, evidence-free diagnostics for a shared context.

use std::io;

use crate::core::{BackendProfile, BatchError, LimitKind, ProfileName};
use crate::failure::Failure;

#[derive(Debug)]
pub(crate) enum Error {
    Open(io::Error),
    NotUtf8,
    Empty,
    SingleDocument,
    StructuredQuestion,
    OverLimit {
        initial: bool,
        kind: LimitKind,
        limit: usize,
        actual: usize,
        profile: Option<ProfileName>,
    },
}

/// The limits that classify a planner refusal without storing evidence.
#[derive(Clone)]
pub(crate) struct Limits {
    profile: Option<BackendProfile>,
}

impl Limits {
    pub(crate) fn new(profile: Option<&BackendProfile>) -> Self {
        Self {
            profile: profile.cloned(),
        }
    }

    pub(crate) fn refused(&self, error: BatchError, initial: bool) -> Failure {
        match error {
            BatchError::ContextOverLimit {
                kind,
                limit,
                actual,
            } => {
                let profile = self.profile.as_ref().filter(|held| match kind {
                    LimitKind::EvidenceBytes => held.max_evidence_bytes == Some(limit),
                    LimitKind::RequestBytes => held.max_request_bytes == Some(limit),
                    LimitKind::Questions => held.max_questions == Some(limit),
                    LimitKind::Options => false,
                });
                let profile = profile.map(|held| held.name().clone());
                Failure::Context(Error::OverLimit {
                    initial,
                    kind,
                    limit,
                    actual,
                    profile,
                })
            }
            BatchError::StructuredQuestionWithContext => {
                Failure::Context(Error::StructuredQuestion)
            }
            BatchError::Profile(limit) => Failure::ProfileLimit(limit),
            BatchError::Defect(what) => Failure::Defect(what),
        }
    }
}

pub(super) fn message(failure: &Failure) -> Option<(u8, String)> {
    let Failure::Context(error) = failure else {
        return None;
    };
    Some(match error {
        Error::Open(error) => (5, format!("--context could not be opened: {error}")),
        Error::NotUtf8 => (5, "--context is not UTF-8 text".to_owned()),
        Error::Empty => (2, "--context names an empty file".to_owned()),
        Error::SingleDocument => (2, "--context shares one text across the records of a stream, and a single text is one record".to_owned()),
        Error::StructuredQuestion => (2, "--context needs a question written as text; a question written as JSON cannot quote a record".to_owned()),
        Error::OverLimit { initial, kind, limit, actual, profile } => {
            let message = if let Some(profile) = profile {
                if *initial {
                    format!("--context: profile {} allows at most {limit} {}; the context's request has {actual}", profile.as_str(), kind.words())
                } else {
                    format!("--context: profile {} allows at most {limit} {}; this record and the context make {actual}", profile.as_str(), kind.words())
                }
            } else if *initial {
                format!("--context: the context and the question make a request of {actual} bytes, over the request size of {limit} bytes; raise --max-request-bytes or shorten the context")
            } else {
                format!("--context: this record and the context make a request of {actual} bytes, over the request size of {limit} bytes; raise --max-request-bytes, or shorten the context or the record")
            };
            (2, message)
        }
    })
}
