//! Canonical question identities use the same native operations as completed results.
use super::RequestDefinition;
use crate::{Error, LoadedQuestion, core};

/// The original relation input interpretation retained by canonical question identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationIdentityMode {
    /// Records select entity names and kinds through the saved fields.
    Records,
    /// Lines treat each original line as an entity name, omitting record fields.
    Lines,
}

impl RequestDefinition {
    /// Compute the canonical identity of this admitted question, without I/O or sends.
    /// Atomic identities include the resolved reading threshold and profile;
    /// ordered sets and recognition reuse their native canonical operations.
    /// Find retains its dedicated identity, including its explicit none reading.
    ///
    /// # Errors
    /// Relate requires an explicit [`RelationIdentityMode`] through
    /// [`Self::relation_sha256`]. Per-record choose has no complete identity
    /// before its actual options arrive. Encoding an admitted question can fail
    /// only as an ordinary defect. No template or wire-request digest is substituted.
    pub fn question_sha256(&self) -> Result<String, Error> {
        let encoded = match self {
            Self::Atomic(LoadedQuestion::Question(question)) | Self::Rank(question) => {
                return atomic(question);
            }
            Self::Atomic(LoadedQuestion::Banded(question)) => return atomic(&question.0),
            Self::Find(question) => return atomic(question.question()),
            Self::Annotate(set) | Self::DecodedSet { annotation: set, .. } => set.0.sha256(),
            Self::RankSet(set) => set.0.sha256(),
            Self::Recognize(question) => core::recognize_sha256(&question.question().0),
            Self::Recognition(question) => core::recognize_sha256(&question.0),
            Self::Relate(_) => return Err(Error::usage("relate identity requires its record or line mode")),
            Self::DynamicChoose(_) => return Err(Error::usage("a per-record choose question has no identity before its options")),
        };
        encoded.map_err(|_| Error::defect("question identity could not be written"))
    }

    /// Compute a relation question's existing record- or line-mode identity.
    /// This uses the saved fields in record mode and native null fields in line
    /// mode. The input entities, selected model and runtime controls stay outside it.
    /// # Errors
    /// Refuses another definition kind, or returns Defect for an encoding failure.
    pub fn relation_sha256(&self, mode: RelationIdentityMode) -> Result<String, Error> {
        let Self::Relate(question) = self else {
            return Err(Error::usage("relation identity requires a relate question"));
        };
        question.0.question(mode == RelationIdentityMode::Lines).sha256()
            .map_err(|_| Error::defect("question identity could not be written"))
    }
}

fn atomic(question: &crate::Question) -> Result<String, Error> {
    if matches!(question.kind, crate::public::question::Kind::Find | crate::public::question::Kind::FindNone) {
        let core::Question::Decide { text, .. } = &question.core else {
            return Err(Error::defect("a find preparation lost its question"));
        };
        return core::Find::identity(text, question.kind == crate::public::question::Kind::FindNone, question.profile.as_ref())
            .map_err(|_| Error::defect("question identity could not be written"));
    }
    core::question_sha256_with_profile(&question.core, question.threshold, question.profile.as_ref())
        .map_err(|_| Error::defect("question identity could not be written"))
}
