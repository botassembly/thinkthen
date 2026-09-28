//! Project one packed group request into its ordered record fragments.

use super::{
    AnnotateBatchMeta, AnswerOutcome, Batch, BatchMeta, ChunkAnswer, Error, GroupAnswer,
    GroupBatchFailure, ParentAttempt, Reply, Setting,
};

pub(super) fn project_group(
    batch: &Batch,
    places: &[usize],
    answered: super::super::Answered,
    parent: Option<(&ParentAttempt, usize)>,
    group: usize,
    setting: Setting,
) -> Result<Vec<GroupAnswer>, Error> {
    let members = batch
        .group_members
        .as_ref()
        .ok_or(Error::Defect("an annotate batch has no group members"))?;
    let mut fragments = Vec::with_capacity(members.len());
    for (position, member) in members.iter().enumerate() {
        if member.outcomes.len() != places.len() {
            return Err(Error::Defect("an annotate batch lost its group slice"));
        }
        let outcomes = member
            .outcomes
            .iter()
            .map(|&at| {
                answered
                    .reply
                    .outcomes()
                    .get(at)
                    .cloned()
                    .ok_or(Error::Defect("an annotate batch lost an answer"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let reply = Reply::new(
            answered.reply.model().clone(),
            outcomes,
            answered
                .reply
                .usage()
                .map(|usage| usage.share(members.len(), position)),
        );
        let parent_sent = parent.map_or(0, |(attempt, offset)| {
            crate::core::share(attempt.sent, attempt.total, offset + position)
        });
        let batch_meta = BatchMeta::new(
            setting,
            members.len(),
            position + 1,
            batch.closed,
            answered.reply.usage(),
            answered.requests_sent,
        );
        let batch_meta = if parent.is_some() {
            batch_meta.with_split()
        } else {
            batch_meta
        };
        let parent_batch = parent.map(|(attempt, offset)| {
            AnnotateBatchMeta::new(
                group + 1,
                attempt.digest.clone(),
                BatchMeta::new(
                    attempt.setting,
                    attempt.total,
                    offset + position + 1,
                    attempt.closed,
                    None,
                    attempt.sent,
                ),
            )
        });
        fragments.push(GroupAnswer {
            answered: vec![ChunkAnswer {
                places: places.to_vec(),
                reply,
                digest: answered.request.as_str().to_owned(),
                requests_sent: crate::core::share(answered.requests_sent, members.len(), position),
                replayed: answered.replayed,
                parent_request: parent.map(|(attempt, _)| attempt.digest.clone()),
                parent_sent,
                batch: Some(AnnotateBatchMeta::new(
                    group + 1,
                    answered.request.as_str().to_owned(),
                    batch_meta,
                )),
                parent_batch,
            }],
            model: Some(answered.reply.model().clone()),
        });
    }
    Ok(fragments)
}

pub(super) fn group_completion(
    mut value: Vec<GroupAnswer>,
    sole_group: bool,
) -> crate::engine::schedule::Completed<Vec<GroupAnswer>, GroupBatchFailure> {
    let stop = if sole_group {
        value
            .iter()
            .position(|one| {
                one.answered.iter().all(|chunk| {
                    chunk
                        .reply
                        .outcomes()
                        .iter()
                        .all(|answer| matches!(answer, AnswerOutcome::Failed(_)))
                })
            })
            .map(|index| {
                value.truncate(index + 1);
                GroupBatchFailure::AllFailed
            })
    } else {
        None
    };
    crate::engine::schedule::Completed {
        value,
        records: 0,
        replayed: 0,
        partial_failure: false,
        stop,
    }
}
