//! Bounded, request-aligned planning for annotation groups.

use std::collections::{BTreeMap, VecDeque};

use crate::core::batch::BatchError;
use crate::core::{Batch, BatchRecord, GroupBatcher, PartError, QuestionSet, Setting};
use crate::engine::facade::Engine;

/// One closed request and the input rows its logical members answer.
pub(crate) struct GroupWork {
    pub(crate) batch: Batch,
    pub(crate) places: Vec<usize>,
    pub(crate) rows: Vec<usize>,
    pub(crate) group: usize,
}

/// Preserve caller selection and profile refusals across the private planner.
pub(crate) enum GroupPlanError {
    Part(PartError),
    Batch(BatchError),
    Defect(&'static str),
}

struct Group {
    places: Vec<usize>,
    planner: GroupBatcher,
    pending: VecDeque<usize>,
}

/// One open pure planner for each normalized `on` group.
pub(crate) struct GroupPlanner {
    groups: Vec<Group>,
    ready: BTreeMap<(usize, usize, usize), GroupWork>,
    sequence: usize,
}

impl GroupPlanner {
    pub(crate) fn new(
        engine: &Engine,
        set: &QuestionSet,
        setting: Setting,
    ) -> Result<Self, GroupPlanError> {
        let groups = set
            .groups()
            .into_iter()
            .map(|places| {
                let questions = places
                    .iter()
                    .map(|&place| {
                        set.questions()
                            .get(place)
                            .map(|named| named.question().clone())
                            .ok_or(GroupPlanError::Defect(
                                "an annotate group names no question",
                            ))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let planner = GroupBatcher::new(
                    engine.backend().clone(),
                    engine.profile().cloned(),
                    questions,
                    setting,
                )
                .map_err(GroupPlanError::Batch)?;
                Ok(Group {
                    places,
                    planner,
                    pending: VecDeque::new(),
                })
            })
            .collect::<Result<Vec<_>, GroupPlanError>>()?;
        Ok(Self {
            groups,
            ready: BTreeMap::new(),
            sequence: 0,
        })
    }

    /// Validate every selection before adding any group work for this row.
    pub(crate) fn push(
        &mut self,
        set: &QuestionSet,
        record: &BatchRecord,
        row: usize,
    ) -> Result<(), GroupPlanError> {
        let selected = self
            .groups
            .iter()
            .map(|group| {
                let evidence = set
                    .group_evidence(&group.places, record)
                    .map_err(GroupPlanError::Part)?;
                Ok(BatchRecord {
                    value: evidence.as_json(),
                    evidence,
                })
            })
            .collect::<Result<Vec<_>, GroupPlanError>>()?;
        for (group, selected) in selected.into_iter().enumerate() {
            let held = self
                .groups
                .get_mut(group)
                .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?;
            held.pending.push_back(row);
            let mut closed = Vec::new();
            held.planner
                .push(selected, &mut closed)
                .map_err(GroupPlanError::Batch)?;
            for batch in closed {
                self.queue(group, batch).map_err(GroupPlanError::Defect)?;
            }
        }
        Ok(())
    }

    /// Close all valid earlier fragments when input ends or a later row refuses.
    pub(crate) fn finish(&mut self) -> Result<(), GroupPlanError> {
        for group in 0..self.groups.len() {
            let closed = self
                .groups
                .get_mut(group)
                .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?
                .planner
                .finish()
                .map_err(GroupPlanError::Batch)?;
            if let Some(batch) = closed {
                self.queue(group, batch).map_err(GroupPlanError::Defect)?;
            }
        }
        Ok(())
    }

    fn queue(&mut self, group: usize, batch: Batch) -> Result<(), &'static str> {
        let members = batch
            .group_members
            .as_ref()
            .ok_or("a group request has no members")?
            .len();
        let held = self
            .groups
            .get_mut(group)
            .ok_or("an annotate group disappeared")?;
        let rows = held.pending.drain(..members).collect::<Vec<_>>();
        let first = *rows.first().ok_or("a group request has no input row")?;
        if rows.len() != members {
            return Err("a group request lost an input row");
        }
        self.ready.insert(
            (first, group, self.sequence),
            GroupWork {
                batch,
                places: held.places.clone(),
                rows,
                group,
            },
        );
        self.sequence += 1;
        Ok(())
    }

    pub(crate) fn pop(&mut self) -> Option<GroupWork> {
        self.ready.pop_first().map(|(_, work)| work)
    }

    pub(crate) fn ready(&self) -> bool {
        !self.ready.is_empty()
    }

    pub(crate) fn groups(&self) -> usize {
        self.groups.len()
    }
}
