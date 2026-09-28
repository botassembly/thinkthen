//! Profile slices of one normalized annotation group.

use super::{GroupPlanError, GroupPlanner, GroupRequest, GroupWork, Slice};
use crate::core::{Batch, BatchRecord, GroupBatcher, Plan, Question, QuestionSet};
use crate::engine::facade::Engine;
use std::collections::VecDeque;

impl GroupPlanner {
    /// Plan every singleton profile slice before admitting this CLI row.
    /// Each returned slot belongs to the row's own layout, even if an earlier
    /// row held a different layout in the same normalized group.
    pub(crate) fn push_sliced(
        &mut self,
        engine: &Engine,
        set: &QuestionSet,
        record: &BatchRecord,
        row: usize,
    ) -> Result<Vec<usize>, GroupPlanError> {
        let selected = self
            .groups
            .iter()
            .map(|group| {
                let evidence = set
                    .group_evidence(&group.places, record)
                    .map_err(GroupPlanError::Part)?;
                let selected = BatchRecord {
                    value: evidence.as_json(),
                    evidence,
                };
                let plan = Plan::new(
                    selected.evidence.clone(),
                    engine.backend().model().clone(),
                    group.questions.clone(),
                )
                .map_err(|_| GroupPlanError::Defect("an annotate group asks nothing"))?;
                let layout = engine
                    .prepare_group(&plan, group.places.clone())
                    .map_err(GroupPlanError::Engine)?
                    .slices();
                Ok((selected, layout))
            })
            .collect::<Result<Vec<_>, GroupPlanError>>()?;
        let mut next_slot = 0;
        let mut groups = Vec::new();
        for (group, (record, layout)) in selected.into_iter().enumerate() {
            let count = layout.len();
            self.add_slices(engine, group, record, layout, row, next_slot)?;
            next_slot += count;
            groups.extend(std::iter::repeat_n(group, count));
        }
        Ok(groups)
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "the slice location, selected record, and layout are distinct planner inputs"
    )]
    fn add_slices(
        &mut self,
        engine: &Engine,
        group: usize,
        record: BatchRecord,
        layout: Vec<(Vec<usize>, Vec<Question>)>,
        row: usize,
        first_slot: usize,
    ) -> Result<(), GroupPlanError> {
        let same_layout = self.groups.get(group).is_some_and(|held| {
            held.slices.len() == layout.len()
                && held
                    .slices
                    .iter()
                    .zip(&layout)
                    .all(|(slice, (places, _))| slice.places == *places)
        });
        if !same_layout {
            self.close_slices(group, false)?;
            let rebuilt = layout
                .into_iter()
                .map(|(places, questions)| {
                    let planner = GroupBatcher::new(
                        engine.backend().clone(),
                        engine.profile().cloned(),
                        questions.clone(),
                        self.setting,
                    )
                    .map_err(GroupPlanError::Batch)?;
                    Ok(Slice {
                        places,
                        questions,
                        planner,
                        pending: VecDeque::new(),
                    })
                })
                .collect::<Result<Vec<_>, GroupPlanError>>()?;
            self.groups
                .get_mut(group)
                .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?
                .slices = rebuilt;
        }
        let count = self
            .groups
            .get(group)
            .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?
            .slices
            .len();
        for slice in 0..count {
            let mut closed = Vec::new();
            {
                let held = self
                    .groups
                    .get_mut(group)
                    .and_then(|held| held.slices.get_mut(slice))
                    .ok_or(GroupPlanError::Defect("an annotate slice disappeared"))?;
                held.pending
                    .push_back((row, record.clone(), first_slot + slice));
                held.planner
                    .push(record.clone(), &mut closed)
                    .map_err(GroupPlanError::Batch)?;
            }
            for batch in closed {
                self.queue_slice(group, slice, batch)
                    .map_err(GroupPlanError::Defect)?;
            }
        }
        Ok(())
    }
    /// A real upstream pause closes each open slice without inventing a
    /// scheduler or window close reason.
    pub(crate) fn pause(&mut self) -> Result<(), GroupPlanError> {
        for group in 0..self.groups.len() {
            self.close_slices(group, true)?;
        }
        Ok(())
    }

    #[expect(
        clippy::excessive_nesting,
        reason = "one slice closes while its planner borrow is held"
    )]
    pub(super) fn close_slices(&mut self, group: usize, pause: bool) -> Result<(), GroupPlanError> {
        let count = self
            .groups
            .get(group)
            .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?
            .slices
            .len();
        for slice in 0..count {
            let closed = {
                let held = self
                    .groups
                    .get_mut(group)
                    .and_then(|held| held.slices.get_mut(slice))
                    .ok_or(GroupPlanError::Defect("an annotate slice disappeared"))?;
                if pause {
                    held.planner.pause()
                } else {
                    held.planner.finish()
                }
                .map_err(GroupPlanError::Batch)?
            };
            if let Some(batch) = closed {
                self.queue_slice(group, slice, batch)
                    .map_err(GroupPlanError::Defect)?;
            }
        }
        Ok(())
    }

    fn queue_slice(
        &mut self,
        group: usize,
        slice: usize,
        batch: Batch,
    ) -> Result<(), &'static str> {
        let sole_group = self.groups.len() == 1
            && self
                .groups
                .get(group)
                .is_some_and(|held| held.slices.len() == 1);
        let members = batch
            .group_members
            .as_ref()
            .ok_or("an annotate slice request has no members")?
            .len();
        let held = self
            .groups
            .get_mut(group)
            .and_then(|held| held.slices.get_mut(slice))
            .ok_or("an annotate slice disappeared")?;
        let (rows, pairs): (Vec<_>, Vec<_>) = held
            .pending
            .drain(..members)
            .map(|(row, record, slot)| (row, (record, slot)))
            .unzip();
        let (records, slots): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
        let first = *rows.first().ok_or("an annotate slice has no input row")?;
        if rows.len() != members {
            return Err("an annotate slice lost an input row");
        }
        self.ready.insert(
            (first, group, self.sequence),
            GroupWork {
                request: GroupRequest::Packed {
                    batch: Box::new(batch),
                    records,
                    questions: held.questions.clone(),
                },
                places: held.places.clone(),
                rows,
                slots,
                group,
                sole_group,
                setting: self.setting,
            },
        );
        self.sequence += 1;
        Ok(())
    }
    /// Only a request starting at the oldest unfinished row may dispatch.
    /// A later closed group waits for an accepted close event in that row.
    pub(crate) fn ready_at(&self, oldest: usize) -> bool {
        self.ready
            .first_key_value()
            .is_some_and(|((first, _, _), _)| *first <= oldest)
    }

    pub(crate) fn pop_sliced_at(&mut self, oldest: usize) -> Option<GroupWork> {
        self.ready_at(oldest)
            .then(|| self.ready.pop_first().map(|(_, work)| work))
            .flatten()
    }
}
