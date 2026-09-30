//! Bounded, request-aligned planning for annotation groups.

use std::collections::{BTreeMap, VecDeque};
use std::sync::Mutex;

use crate::core::batch::BatchError;
use crate::core::{
    Batch, BatchRecord, GroupBatcher, PartError, Question, QuestionSet, Setting, quoted_plan,
};
use crate::engine::error::Error;
use crate::engine::facade::{Engine, PreparedGroup};

pub(crate) enum GroupRequest {
    Packed {
        batch: Box<Batch>,
        records: Vec<BatchRecord>,
        questions: Vec<Question>,
    },
    Legacy(Mutex<Option<PreparedGroup>>),
}

/// One closed request and the input rows its logical members answer.
pub(crate) struct GroupWork {
    pub(crate) request: GroupRequest,
    pub(crate) places: Vec<usize>,
    pub(crate) rows: Vec<usize>,
    pub(crate) group: usize,
    pub(crate) sole_group: bool,
    pub(crate) setting: Setting,
}

/// Preserve caller selection and profile refusals across the private planner.
pub(crate) enum GroupPlanError {
    Part(PartError),
    Batch(BatchError),
    Defect(&'static str),
    Engine(Error),
}

struct Group {
    places: Vec<usize>,
    questions: Vec<Question>,
    planner: GroupBatcher,
    pending: VecDeque<(usize, BatchRecord)>,
}

/// One open pure planner for each normalized `on` group.
pub(crate) struct GroupPlanner {
    groups: Vec<Group>,
    ready: BTreeMap<(usize, usize, usize), GroupWork>,
    sequence: usize,
    setting: Setting,
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
                    questions.clone(),
                    setting,
                )
                .map_err(GroupPlanError::Batch)?;
                Ok(Group {
                    places,
                    questions,
                    planner,
                    pending: VecDeque::new(),
                })
            })
            .collect::<Result<Vec<_>, GroupPlanError>>()?;
        Ok(Self {
            groups,
            ready: BTreeMap::new(),
            sequence: 0,
            setting,
        })
    }

    /// Validate every selection before adding any group work for this row.
    pub(crate) fn push(
        &mut self,
        engine: &Engine,
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
                let selected = BatchRecord {
                    value: evidence.as_json(),
                    evidence,
                };
                let prepared = if engine.profile().is_some() {
                    let plan = quoted_plan(
                        engine.backend().model().clone(),
                        selected.evidence.clone(),
                        None,
                        group.questions.clone(),
                        engine.profile(),
                    )
                    .map_err(|error| match error {
                        BatchError::Profile(_) => GroupPlanError::Batch(error),
                        _ => GroupPlanError::Defect("an annotate group asks nothing"),
                    })?;
                    let prepared = engine
                        .prepare_group(&plan, group.places.clone())
                        .map_err(GroupPlanError::Engine)?;
                    (prepared.chunks.len() > 1).then_some(prepared)
                } else {
                    None
                };
                Ok((selected, prepared))
            })
            .collect::<Result<Vec<_>, GroupPlanError>>()?;
        for (group, (selected, prepared)) in selected.into_iter().enumerate() {
            self.add(engine, group, selected, prepared, row)?;
        }
        Ok(())
    }

    fn add(
        &mut self,
        engine: &Engine,
        group: usize,
        selected: BatchRecord,
        prepared: Option<PreparedGroup>,
        row: usize,
    ) -> Result<(), GroupPlanError> {
        if let Some(prepared) = prepared {
            let held = self
                .groups
                .get_mut(group)
                .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?;
            let closed = held.planner.finish().map_err(GroupPlanError::Batch)?;
            if let Some(batch) = closed {
                self.queue(group, batch).map_err(GroupPlanError::Defect)?;
            }
            let held = self
                .groups
                .get_mut(group)
                .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?;
            held.planner = GroupBatcher::new(
                engine.backend().clone(),
                engine.profile().cloned(),
                held.questions.clone(),
                self.setting,
            )
            .map_err(GroupPlanError::Batch)?;
            self.ready.insert(
                (row, group, self.sequence),
                GroupWork {
                    request: GroupRequest::Legacy(Mutex::new(Some(prepared))),
                    places: held.places.clone(),
                    rows: vec![row],
                    group,
                    sole_group: self.groups.len() == 1,
                    setting: self.setting,
                },
            );
            self.sequence += 1;
        } else {
            let held = self
                .groups
                .get_mut(group)
                .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?;
            held.pending.push_back((row, selected.clone()));
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
        let sole_group = self.groups.len() == 1;
        let members = batch
            .group_members
            .as_ref()
            .ok_or("a group request has no members")?
            .len();
        let held = self
            .groups
            .get_mut(group)
            .ok_or("an annotate group disappeared")?;
        let (rows, records): (Vec<_>, Vec<_>) = held.pending.drain(..members).unzip();
        let first = *rows.first().ok_or("a group request has no input row")?;
        if rows.len() != members {
            return Err("a group request lost an input row");
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
                group,
                sole_group,
                setting: self.setting,
            },
        );
        self.sequence += 1;
        Ok(())
    }

    /// Close older open group fragments before dispatching work starting at a later row.
    pub(crate) fn pop_at_frontier(
        &mut self,
        oldest: usize,
    ) -> Result<Option<GroupWork>, GroupPlanError> {
        if self
            .ready
            .first_key_value()
            .is_some_and(|((first, _, _), _)| *first > oldest)
        {
            self.close_oldest(oldest)?;
        }
        Ok(self.ready.pop_first().map(|(_, work)| work))
    }

    fn close_oldest(&mut self, oldest: usize) -> Result<(), GroupPlanError> {
        for group in 0..self.groups.len() {
            let held = self
                .groups
                .get_mut(group)
                .ok_or(GroupPlanError::Defect("an annotate group disappeared"))?;
            let closed = if held.pending.front().is_some_and(|(row, _)| *row <= oldest) {
                held.planner.finish().map_err(GroupPlanError::Batch)?
            } else {
                None
            };
            if let Some(batch) = closed {
                self.queue(group, batch).map_err(GroupPlanError::Defect)?;
            }
        }
        Ok(())
    }

    pub(crate) fn ready(&self) -> bool {
        !self.ready.is_empty()
    }

    pub(crate) fn groups(&self) -> usize {
        self.groups.len()
    }
}
