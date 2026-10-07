//! Canonical complete carriers, populated exclusively from actual native calls.
mod details;
mod execute;
mod inputs;
mod located;
mod metadata;
mod observations;
mod questions;
mod rank_set;
mod rows;
mod structured;
use crate::current::Storage;
use crate::ffi::carriers::{ObservationV1, RowObservationV1, SummaryV1};
pub(crate) use execute::ask;
use std::fmt;
/// Immutable result owner; every nested view allocation lives until this is freed.
pub(crate) struct ResultHandle {
    pub(crate) _storage: Storage,
    pub(crate) summary: SummaryV1,
    pub(crate) rows: Vec<RowObservationV1>,
    pub(crate) observations: Vec<ObservationV1>,
    pub(crate) authors: Vec<crate::ffi::carriers::QuestionAuthorV1>,
    pub(crate) member_authors: Vec<Vec<crate::ffi::carriers::QuestionAuthorV1>>,
    pub(crate) observation_authors: Vec<crate::ffi::carriers::QuestionAuthorV1>,
    pub(crate) rank_members: Vec<Vec<crate::ffi::carriers::RankViewV1>>,
    pub(crate) source_recognition: Vec<crate::ffi::carriers::SourceRecognitionV1>,
    pub(crate) source_relations: Vec<crate::ffi::carriers::SourceRelationsV1>,
    pub(crate) row_details: Vec<crate::ffi::carriers::DetailsV1>,
    pub(crate) observation_details: Vec<crate::ffi::carriers::DetailsV1>,
}
impl fmt::Debug for ResultHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResultHandle")
            .field("count", &self.rows.len())
            .finish_non_exhaustive()
    }
}
pub(crate) fn failure(
    snapshot: crate::failures::FailureSnapshot,
) -> Result<ResultHandle, crate::failures::Failure> {
    let mut storage = Storage::default();
    let facts = snapshot
        .facts
        .as_deref()
        .map(|f| metadata::facts(&mut storage, f))
        .transpose()?
        .unwrap_or_default();
    let attempts = storage.attempts(
        snapshot
            .facts
            .as_deref()
            .and_then(thinkthen::Facts::attempts),
    );
    let error = crate::ffi::carriers::OptionalErrorV1 {
        present: 1,
        value: crate::ffi::carriers::ErrorV1 {
            code: snapshot.code,
            message: storage.string(&snapshot.message),
            retryable: i32::from(snapshot.retryable),
            stopped: snapshot.stopped.map(metadata::stopped).unwrap_or_default(),
        },
    };
    Ok(ResultHandle {
        _storage: storage,
        summary: SummaryV1 {
            state: 2,
            facts,
            attempts,
            error,
            ..SummaryV1::default()
        },
        rows: Vec::new(),
        observations: Vec::new(),
        authors: Vec::new(),
        member_authors: Vec::new(),
        observation_authors: Vec::new(),
        rank_members: Vec::new(),
        source_recognition: Vec::new(),
        source_relations: Vec::new(),
        row_details: Vec::new(),
        observation_details: Vec::new(),
    })
}
