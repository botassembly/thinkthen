//! Native turns ranking retains its selecting member and every member judgment.
use super::{inputs::Original, metadata, observations, rows};
use crate::current::{Storage, author::native_author};
use crate::failures::Failure;
use crate::ffi::carriers::{RankViewV1, RowObservationDataV1, RowObservationV1};
use thinkthen::{CompleteRecord, CompleteSetRank, OwnedRecordObservation};
pub(super) fn row(
    s: &mut Storage,
    row: &CompleteRecord<Original, CompleteSetRank>,
    events: &[OwnedRecordObservation],
) -> Result<RowObservationV1, Failure> {
    let set = row.result();
    let r = set.result();
    let raw = observations::raw(events, row.ordinal(), Some(set.question_name()));
    let answer = s.answer(
        rows::question_kind(r.question()),
        &r.probabilities(),
        raw.as_deref(),
        r.confidence(),
    )?;
    let common = s.atomic_row(
        row.original(),
        r.meta(),
        r.question(),
        r.threshold(),
        answer,
    )?;
    let mut authors = Vec::new();
    let mut members = Vec::new();
    let mut details = Vec::new();
    for member in set.members() {
        let result = member.result();
        authors.push(s.author(&native_author!(result.question())));
        let raw = observations::raw(events, row.ordinal(), Some(member.name()));
        let answer = s.answer(
            rows::question_kind(result.question()),
            &result.probabilities(),
            raw.as_deref(),
            result.confidence(),
        )?;
        let common = s.atomic_row(
            row.original(),
            result.meta(),
            result.question(),
            result.threshold(),
            answer,
        )?;
        details.push(s.details(
            common,
            result.meta(),
            std::iter::once(&row.original().native),
            raw.as_deref(),
        )?);
        members.push(RankViewV1 {
            common,
            value: metadata::size(Some(result.value())),
            question_name: s.optional_string(Some(member.name())),
        });
    }
    s.row_author(&native_author!(r.question()), authors);
    s.5.push(members);
    s.8.push(details);
    s.row_details(
        common,
        r.meta(),
        std::iter::once(&row.original().native),
        raw.as_deref(),
    )?;
    let mut data = RowObservationDataV1::default();
    data.rank = RankViewV1 {
        common,
        value: metadata::size(Some(set.value())),
        question_name: s.optional_string(Some(set.question_name())),
    };
    Ok(RowObservationV1 {
        index: row.ordinal(),
        function: 6,
        data,
    })
}
