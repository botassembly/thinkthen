//! Finalize typed rank identity only after stable ordering and turns selection.
use super::Judged;
use crate::core::{self, CompleteAtomic, ModelName, Record};
use crate::failure::Failure;
use serde::{Serialize, Serializer};
use std::{num::NonZeroUsize, sync::Arc};

pub(crate) struct RankRow {
    pub(crate) value: RankValue,
    pub(crate) record: usize,
    pub(crate) documents: bool,
}
pub(crate) enum RankValue {
    Single(Box<CompleteAtomic>),
    Set { group: Arc<Group>, member: usize },
}
pub(crate) struct Group {
    original: Record,
    members: Vec<(String, CompleteAtomic)>,
    digest: String,
    requested_model: ModelName,
}
impl RankRow {
    pub(super) const fn is_set(&self) -> bool {
        matches!(self.value, RankValue::Set { .. })
    }
    pub(crate) fn bind_set(
        rows: &mut [Judged],
        set: &core::QuestionSet,
        requested_model: &ModelName,
    ) -> Result<(), Failure> {
        let mut original = None;
        let mut members = Vec::with_capacity(rows.len());
        let mut formats = Vec::with_capacity(rows.len());
        for (row, named) in rows.iter_mut().zip(set.questions()) {
            let rank = row
                .rank
                .take()
                .ok_or(Failure::Defect("a rank lost its detail"))?;
            let RankValue::Single(mut canonical) = rank.value else {
                return Err(Failure::Defect("a rank member was already bound"));
            };
            let input = canonical.take_input();
            if original.is_none() {
                original = input;
            }
            canonical.declarations = named.metadata().clone();
            members.push((named.name().to_owned(), *canonical));
            formats.push((rank.record, rank.documents));
        }
        if members.len() != set.questions().len() {
            return Err(Failure::Defect("a rank lost its member"));
        }
        let group = Arc::new(Group {
            original: original.ok_or(Failure::Defect("a rank lost its original"))?,
            members,
            digest: set.sha256()?,
            requested_model: requested_model.clone(),
        });
        for (member, (row, (record, documents))) in rows.iter_mut().zip(formats).enumerate() {
            row.rank = Some(RankRow {
                value: RankValue::Set {
                    group: Arc::clone(&group),
                    member,
                },
                record,
                documents,
            });
        }
        Ok(())
    }
}
struct Named<'a> {
    canonical: &'a CompleteAtomic,
    original: &'a Record,
    name: &'a str,
    members: Vec<core::RankMemberDocument<'a>>,
}
impl Serialize for Named<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.canonical.serialize_occurrence_members(
            Some(self.original),
            Some(self.name),
            None,
            Some(
                self.members
                    .iter()
                    .map(|member| core::RankMemberDocument {
                        name: member.name,
                        result: member.result,
                    })
                    .collect(),
            ),
            serializer,
        )
    }
}
fn position(at: usize) -> Result<NonZeroUsize, Failure> {
    at.checked_add(1)
        .and_then(NonZeroUsize::new)
        .ok_or(Failure::Defect("rank position overflow"))
}
impl Judged {
    pub(super) fn finish_rank(
        &mut self,
        at: usize,
        positions: Option<&[usize]>,
    ) -> Result<(), Failure> {
        let Some(rank) = self.rank.take() else {
            return Ok(());
        };
        let value = position(at)?;
        let mut printed = Some(match rank.value {
            RankValue::Single(canonical) => {
                core::json_line(&canonical.ranked(rank.record, value)?)?
            }
            RankValue::Set { group, member } => {
                let positions =
                    positions.ok_or(Failure::Defect("a rank lost its member positions"))?;
                let members = group
                    .members
                    .iter()
                    .zip(positions)
                    .map(|((name, canonical), at)| {
                        canonical
                            .clone()
                            .ranked_member(rank.record, name, position(*at)?)
                            .map_err(Failure::from)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if members.len() != group.members.len() {
                    return Err(Failure::Defect("a rank lost its member positions"));
                }
                let (name, _) = group
                    .members
                    .get(member)
                    .ok_or(Failure::Defect("a rank lost its winner"))?;
                let canonical = members
                    .get(member)
                    .ok_or(Failure::Defect("a rank lost its winner"))?
                    .clone()
                    .ranked_set(
                        (rank.record, name, value),
                        &group.digest,
                        &members.iter().collect::<Vec<_>>(),
                        &group.requested_model,
                    )?;
                core::json_line(&Named {
                    canonical: &canonical,
                    original: &group.original,
                    name,
                    members: group
                        .members
                        .iter()
                        .zip(&members)
                        .map(|((name, _), result)| core::RankMemberDocument { name, result })
                        .collect(),
                })?
            }
        });
        crate::cli::intake::locate(&mut printed, self.position.as_ref())?;
        crate::cli::intake::source_members(&mut printed, self.position.as_ref())?;
        if rank.documents && !self.position.as_ref().is_some_and(|p| p.located) {
            crate::cli::intake::document(&mut printed, self.position.as_ref(), true)?;
        }
        self.printed = printed;
        Ok(())
    }
}
