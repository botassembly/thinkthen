//! Set rank retains every ordered member and the final turns selection.
use super::Atomic;
use crate::core::image::InputFunction;
use crate::core::{AtomicReading, ModelName, RecordScope, RenderError, ResultIdentity};
use serde::Serialize;
use std::num::NonZeroUsize;

#[derive(Serialize)]
struct MemberScope<'a> {
    record: usize,
    member: &'a str,
    position: usize,
}
#[derive(Serialize)]
struct SetReading<'a> {
    questions_sha256: &'a str,
    question_name: &'a str,
    rank_position: NonZeroUsize,
}
impl Atomic {
    pub(crate) fn ranked_member(
        mut self,
        record: usize,
        name: &str,
        position: NonZeroUsize,
    ) -> Result<Self, RenderError> {
        self.legacy.threshold = None;
        self.rank_position = Some(position);
        self.identity = ResultIdentity::of(
            InputFunction::Rank,
            &MemberScope {
                record,
                member: name,
                position: 0,
            },
            self.identity.question_sources().to_vec(),
            self.identity.observations().to_vec(),
            &AtomicReading {
                question: &self.legacy.question,
                threshold: None,
                rank_position: Some(position),
            },
            &[],
        )?;
        Ok(self)
    }
    pub(crate) fn ranked_set(
        mut self,
        selection: (usize, &str, NonZeroUsize),
        digest: &str,
        members: &[&Self],
        requested_model: &ModelName,
    ) -> Result<Self, RenderError> {
        let (record, name, position) = selection;
        self.rank_position = Some(position);
        self.identity = ResultIdentity::of(
            InputFunction::Rank,
            &RecordScope { record },
            members
                .iter()
                .flat_map(|member| member.identity.question_sources().iter().cloned())
                .collect(),
            members
                .iter()
                .flat_map(|member| member.identity.observations().iter().cloned())
                .collect(),
            &SetReading {
                questions_sha256: digest,
                question_name: name,
                rank_position: position,
            },
            &members
                .iter()
                .map(|member| member.identity.answer_id().clone())
                .collect::<Vec<_>>(),
        )?;
        self.legacy.meta = crate::core::Meta::combined(
            digest.to_owned(),
            requested_model,
            &members
                .iter()
                .map(|member| &member.legacy.meta)
                .collect::<Vec<_>>(),
        )?;
        Ok(self)
    }
}
