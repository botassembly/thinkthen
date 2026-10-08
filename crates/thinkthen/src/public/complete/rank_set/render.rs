//! Stable member ranks feed the existing saved-order turns merge.
use super::{
    CompleteRank, CompleteRankMember, CompleteRecord, CompleteSetRank, Engine, Error, Held,
    InputFunction, NonZeroUsize, Question, core, facade,
};
use crate::public::engine::Keyed;

fn position(at: usize) -> Result<NonZeroUsize, Error> {
    at.checked_add(1)
        .and_then(NonZeroUsize::new)
        .ok_or_else(|| Error::defect("set rank position overflow"))
}
#[expect(
    clippy::too_many_arguments,
    reason = "rendering receives the actual invocation, members and original occurrences"
)]
pub(super) fn ranked<T>(
    public: &Engine,
    engine: &facade::Engine,
    set: &core::QuestionSet,
    questions: &[Question],
    setting: core::Setting,
    held: Vec<Held<T>>,
    rows: Vec<Vec<Keyed>>,
    captured: bool,
) -> Result<Vec<CompleteRecord<T, CompleteSetRank>>, Error> {
    let lists = (0..questions.len())
        .map(|member| {
            let odds = rows
                .iter()
                .map(|row| {
                    row.get(member)
                        .and_then(|(judged, _)| judged.answer.yes())
                        .ok_or_else(super::super::wrong)
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(core::ranking(&odds, None))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let mut positions = vec![vec![0; questions.len()]; rows.len()];
    for (member, list) in lists.iter().enumerate() {
        for (at, index) in list.iter().enumerate() {
            *positions
                .get_mut(*index)
                .and_then(|row| row.get_mut(member))
                .ok_or_else(|| Error::defect("set rank lost a member position"))? = at;
        }
    }
    let selected = core::turns(&lists, None);
    let digest = set.sha256().map_err(|_| super::super::wrong())?;
    let mut rows: Vec<_> = held.into_iter().zip(rows).map(Some).collect();
    selected
        .into_iter()
        .enumerate()
        .map(|(at, (index, winner))| {
            let (held, rows) = rows
                .get_mut(index)
                .and_then(Option::take)
                .ok_or_else(|| Error::defect("set rank lost its original"))?;
            let members = members(
                public,
                engine,
                set,
                questions,
                setting,
                &held,
                index,
                rows,
                positions.get(index).ok_or_else(super::super::wrong)?,
                captured,
            )?;
            let winning = members.get(winner).ok_or_else(super::super::wrong)?;
            let value = position(at)?;
            let canonical = winning
                .result
                .canonical
                .clone()
                .ranked_set(
                    (index, &winning.name, value),
                    &digest,
                    &members
                        .iter()
                        .map(|member| &member.result.canonical)
                        .collect::<Vec<_>>(),
                    engine.backend().model(),
                )
                .map_err(|_| Error::defect("set rank identity could not be finalized"))?;
            Ok(CompleteRecord {
                original: held.original,
                ordinal: index,
                result: CompleteSetRank {
                    result: CompleteRank { canonical, value },
                    question_name: winning.name.clone(),
                    members,
                },
            })
        })
        .collect()
}
#[expect(
    clippy::too_many_arguments,
    reason = "each member retains actual metadata, controls and occurrence scope"
)]
fn members<T>(
    public: &Engine,
    engine: &facade::Engine,
    set: &core::QuestionSet,
    questions: &[Question],
    setting: core::Setting,
    held: &Held<T>,
    index: usize,
    rows: Vec<Keyed>,
    positions: &[usize],
    captured: bool,
) -> Result<Vec<CompleteRankMember>, Error> {
    if rows.len() != questions.len() {
        return Err(Error::defect("a set rank lost a member"));
    }
    rows.into_iter()
        .zip(questions)
        .zip(set.questions())
        .enumerate()
        .map(|(member, (((judged, keys), question), named))| {
            let mut run =
                super::super::batch_run(engine, question, public.profile.as_ref(), setting);
            run.context_sha256.clone_from(&held.context_sha256);
            let attempts = captured.then(|| judged.answered.attempts.clone());
            let mut canonical = super::super::atomic(
                run,
                &judged,
                super::super::spec(InputFunction::Rank, question, judged.value.clone(), index),
                keys,
                None,
                attempts,
            )
            .map_err(|_| super::super::wrong())?;
            canonical.source.clone_from(&held.source);
            let value = position(*positions.get(member).ok_or_else(super::super::wrong)?)?;
            let canonical = canonical
                .ranked_member(index, named.name(), value)
                .map_err(|_| super::super::wrong())?;
            Ok(CompleteRankMember {
                name: named.name().to_owned(),
                result: CompleteRank { canonical, value },
            })
        })
        .collect()
}
