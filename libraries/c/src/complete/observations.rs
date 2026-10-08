//! Immutable observer snapshots; failures never turn into successful nulls.
use super::{metadata, rows};
use crate::current::Storage;
use crate::failures::Failure;
use crate::ffi::carriers::{
    MemberFailureV1, ObservationDataV1, ObservationSuccessV1, ObservationV1,
    OptionalDiscriminatorV1, QuestionObservationDataV1, QuestionObservationV1, RowObservationV1,
};
use crate::ffi::values as abi;
use thinkthen::{FailureCause, Observation, OwnedRecordObservation, QuestionDetail};
pub(super) const fn cause(cause: FailureCause) -> u32 {
    match cause {
        FailureCause::MissingAnswer => abi::THINKTHEN_MEMBER_MISSING_ANSWER_V1,
        FailureCause::WrongKind => abi::THINKTHEN_MEMBER_WRONG_KIND_V1,
        FailureCause::MissingProbability => abi::THINKTHEN_MEMBER_MISSING_PROBABILITY_V1,
        FailureCause::InvalidProbability => abi::THINKTHEN_MEMBER_INVALID_PROBABILITY_V1,
        FailureCause::InvalidDistribution => abi::THINKTHEN_MEMBER_INVALID_DISTRIBUTION_V1,
        FailureCause::UnexpectedProbability => abi::THINKTHEN_MEMBER_UNEXPECTED_PROBABILITY_V1,
    }
}
pub(super) fn raw(
    events: &[OwnedRecordObservation],
    index: usize,
    member: Option<&str>,
) -> Option<String> {
    events.iter().find_map(|event| match event {
        OwnedRecordObservation::Question {
            index: at,
            member: name,
            detail,
            ..
        } if *at == index && name.as_deref() == member => {
            detail.detail().raw_pick().map(str::to_owned)
        }
        _ => None,
    })
}
pub(super) fn convert(
    s: &mut Storage,
    events: &[OwnedRecordObservation],
    rows: &[RowObservationV1],
) -> Result<Vec<ObservationV1>, Failure> {
    events
        .iter()
        .map(|event| {
            let mut data = ObservationDataV1::default();
            let kind = match event {
                OwnedRecordObservation::Question {
                    index,
                    member,
                    stage,
                    position,
                    detail,
                } => {
                    data.question = question(
                        s,
                        (*index, member.as_deref(), *stage, *position),
                        detail.detail(),
                    )?;
                    abi::THINKTHEN_EVENT_QUESTION_V1
                }
                OwnedRecordObservation::Row { index, .. } => {
                    data.row = *rows.iter().find(|row| row.index == *index).ok_or_else(|| {
                        Failure::defect("a native row event lost its complete original occurrence")
                    })?;
                    abi::THINKTHEN_EVENT_ROW_V1
                }
            };
            Ok(ObservationV1 { kind, data })
        })
        .collect()
}
fn question(
    s: &mut Storage,
    (index, member, stage, position): (usize, Option<&str>, Option<&str>, usize),
    d: QuestionDetail<'_>,
) -> Result<QuestionObservationV1, Failure> {
    let stage = stage
        .map(|stage| match stage {
            "boundary" => Ok(abi::THINKTHEN_STAGE_BOUNDARY_V1),
            "kind" => Ok(abi::THINKTHEN_STAGE_KIND_V1),
            "edge" => Ok(abi::THINKTHEN_STAGE_EDGE_V1),
            "relation" => Ok(abi::THINKTHEN_STAGE_RELATION_V1),
            _ => Err(Failure::defect(
                "native observation supplied an unknown stage",
            )),
        })
        .transpose()?;
    let mut data = QuestionObservationDataV1::default();
    let state = if let Some(failure) = d.failure() {
        data.failure = MemberFailureV1 {
            failure_id: s.string(
                d.failure_id()
                    .ok_or_else(|| Failure::defect("native failed observation lost its identity"))?
                    .as_str(),
            ),
            cause: cause(failure),
        };
        abi::THINKTHEN_MEMBER_FAILURE_V1
    } else {
        let observation_id = match d.observations() {
            [Observation::Answered { observation_id }] => Some(observation_id),
            _ => None,
        };
        data.success = ObservationSuccessV1 {
            answer_id: s.string(
                d.answer_id()
                    .ok_or_else(|| {
                        Failure::defect("native observed reading lost its answer identity")
                    })?
                    .as_str(),
            ),
            observation_id: s
                .optional_string(observation_id.map(|id| id.as_str()))
                .value,
            value: rows::member_value(
                s,
                d.value()
                    .ok_or_else(|| Failure::defect("native success lost its value"))?,
                d.question(),
            )?,
            probabilities: s.observed_probabilities(
                d.probabilities()
                    .ok_or_else(|| Failure::defect("native success lost its probabilities"))?,
            ),
            confidence: metadata::double(d.confidence()),
        };
        abi::THINKTHEN_MEMBER_SUCCESS_V1
    };
    Ok(QuestionObservationV1 {
        index,
        member: s.optional_string(member),
        stage: OptionalDiscriminatorV1 {
            present: i32::from(stage.is_some()),
            value: stage.unwrap_or(0),
        },
        position,
        question_sha256: s.string(d.question_sha256()),
        model: s.string(d.model()),
        url: s.string(d.url()),
        requests: s.strings(d.requests()),
        requests_sent: d.requests_sent(),
        cached: i32::from(d.cached()),
        failed_questions: d.failed_questions(),
        usage: s.usage(d.reported_usage()),
        question_sources: s.sources(d.question_sources()),
        state,
        data,
    })
}

pub(super) fn authors(
    s: &mut Storage,
    events: &[OwnedRecordObservation],
    rows: &[RowObservationV1],
    authors: &[crate::ffi::carriers::QuestionAuthorV1],
) -> Result<Vec<crate::ffi::carriers::QuestionAuthorV1>, Failure> {
    use crate::current::author::native_author;
    events
        .iter()
        .map(|event| match event {
            OwnedRecordObservation::Question { detail, .. } => {
                let detail = detail.detail();
                Ok(s.author(&native_author!(detail.question())))
            }
            OwnedRecordObservation::Row { index, .. } => rows
                .iter()
                .position(|row| row.index == *index)
                .and_then(|at| authors.get(at))
                .copied()
                .ok_or_else(|| Failure::defect("native row event lost its author snapshot")),
        })
        .collect()
}
