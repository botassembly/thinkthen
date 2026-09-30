//! Batches: runs of consecutive records that share one request, by ADR 0048
//! items 1 and 2.
//!
//! Every batch sends the context, or `QUOTED` without one, as its state, and
//! each distinct record gets its own quoted questions, a batch of one
//! included, by ADR 0111 section 1. A question written as JSON cannot take the
//! quote, so its record goes alone as the state. The batcher keeps running
//! byte counts, so it encodes each record once and each batch once, and it
//! checks every closed body against those counts.

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroUsize;

use thiserror::Error;

use crate::core::adapters::built_in;
use crate::core::backend::Backend;
use crate::core::backend_profile::{BackendProfile, LimitKind, ProfileLimit};
use crate::core::json::Json;
use crate::core::plan::Plan;
use crate::core::question::Question;
use crate::core::recording::{Digest, Exchange};
use crate::core::text::{Evidence, QuestionText};

mod groups;
mod questions;
pub(crate) use groups::{GroupBatcher, GroupMember, group_halves};
#[cfg(test)]
pub(crate) use questions::halves;
pub(crate) use questions::{halves_with_questions, quoted, quoted_plan, quoted_plan_of};

/// A record closes its batch when its content hash is 0 mod this.
const CUT: u64 = 4_096;

/// A batch closes when it holds this many records, repeats included.
const MEMBERS: usize = 4_096;

/// The state of every quoted request without a context, by ADR 0111.
pub(crate) const QUOTED: &str = "Each question quotes the text it asks about.";

/// How many records a batch may hold: as many as fit, or at most `N`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Setting {
    Max,
    Records(NonZeroUsize),
}

impl Setting {
    /// `max`, or a decimal whole number of at least 1, and nothing else.
    pub(crate) fn parse(text: &str) -> Option<Self> {
        if text == "max" {
            return Some(Self::Max);
        }
        text.bytes()
            .all(|byte| byte.is_ascii_digit())
            .then(|| text.parse().ok().map(Self::Records))
            .flatten()
    }

    /// A question file's `batch`: the string `max` or a whole number of at least 1.
    pub(crate) fn of_json(value: &Json) -> Option<Self> {
        match value {
            Json::String(text) if text == "max" => Some(Self::Max),
            Json::Number(number) => Self::parse(&number.to_string()),
            _ => None,
        }
    }
}

/// One record as a batch reads it: today's evidence for a batch of one, and
/// the JSON value a batch quotes, lists, hashes, and compares for copies.
#[derive(Clone)]
pub(crate) struct BatchRecord {
    pub(crate) evidence: Evidence,
    pub(crate) value: Json,
}

/// Why a batch closed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Closed {
    Content,
    Size,
    Limit,
    Pause,
    End,
}

/// One closed batch: the one request it sends and the records it answers.
pub(crate) struct Batch {
    pub(crate) plan: Plan,
    pub(crate) body: Vec<u8>,
    pub(crate) digest: Digest,
    /// Each record's first wire question, in input order. Copies share one.
    pub(crate) questions: Vec<usize>,
    /// Each record's logical decoded outcome. Copies share one.
    pub(crate) outcomes: Vec<usize>,
    /// The complete question of each logical row, before record quoting.
    pub(crate) row_questions: Vec<Question>,
    /// Complete logical slices for annotate members, absent on ordinary batches.
    pub(crate) group_members: Option<Vec<GroupMember>>,
    pub(crate) closed: Closed,
}

impl fmt::Debug for Batch {
    /// Counts, the digest, and the reason, and never a record or body byte.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Batch")
            .field("records", &self.questions.len())
            .field("body_bytes", &self.body.len())
            .field("digest", &self.digest.as_str())
            .field("closed", &self.closed)
            .finish()
    }
}

/// Why records could not be planned. No variant holds record or context text.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub(crate) enum BatchError {
    #[error("a request passes a profile limit")]
    Profile(ProfileLimit),
    #[error("a question written as JSON cannot quote a record beside a context")]
    StructuredQuestionWithContext,
    #[error("the context's request passes its limit of {limit} {}: {actual}", .kind.words())]
    ContextOverLimit {
        kind: LimitKind,
        limit: usize,
        actual: usize,
    },
    #[error("{0}")]
    Defect(&'static str),
}

/// One distinct record in the open batch.
struct Joined {
    record: BatchRecord,
    base: Vec<Question>,
    question: Option<Vec<Question>>,
    wire: usize,
    share: usize,
}

/// Plans batches from records in input order.
pub(crate) struct Batcher {
    backend: Backend,
    profile: Option<BackendProfile>,
    question: Vec<Question>,
    grouped: bool,
    size: Option<usize>,
    context: Option<Evidence>,
    /// The state every quoted request sends: the context or `QUOTED`.
    shared: Evidence,
    /// The batched body's bytes with no record in it.
    skeleton: usize,
    /// The bytes of the shared state.
    evidence: usize,
    open: Open,
}

/// The open batch: its distinct records, each record's place among them, and
/// running totals of the distinct records' batched shares.
#[derive(Default)]
struct Open {
    distinct: Vec<Joined>,
    seen: BTreeMap<(String, String), usize>,
    members: Vec<usize>,
    bytes: usize,
    wires: usize,
}

impl Batcher {
    /// Check the context alone, before any record, and size the empty batch.
    pub(crate) fn new(
        backend: Backend,
        profile: Option<BackendProfile>,
        question: Question,
        setting: Setting,
        context: Option<Evidence>,
    ) -> Result<Self, BatchError> {
        Self::new_slice(backend, profile, vec![question], setting, context, false)
    }

    fn new_slice(
        backend: Backend,
        profile: Option<BackendProfile>,
        question: Vec<Question>,
        setting: Setting,
        context: Option<Evidence>,
        grouped: bool,
    ) -> Result<Self, BatchError> {
        let quotable = question.iter().all(|question| {
            let mut question = question.clone();
            text(&mut question).as_json().as_str().is_some()
        });
        if context.is_some() && !quotable {
            return Err(BatchError::StructuredQuestionWithContext);
        }
        let size = match setting {
            _ if !quotable => Some(1),
            Setting::Max => None,
            Setting::Records(most) => Some(most.get()),
        };
        let shared = match &context {
            Some(context) => context.clone(),
            None => Evidence::new(QUOTED).map_err(|_| defect())?,
        };
        let evidence = shared.as_text().map_err(|_| defect())?.len();
        let mut batcher = Self {
            backend,
            profile,
            question,
            grouped,
            size,
            context,
            shared,
            skeleton: 0,
            evidence,
            open: Open::default(),
        };
        if quotable {
            batcher.skeleton = batcher.skeleton()?;
        }
        if batcher.context.is_some() {
            batcher.context_fits(0, 0)?;
        }
        Ok(batcher)
    }

    /// Add one record, and add the batches that close to `closed`: zero, one,
    /// or two. A batch that closes before a refusal stays in `closed`, and a
    /// record that fails the profile alone is refused on its own push.
    pub(crate) fn push(
        &mut self,
        record: BatchRecord,
        closed: &mut Vec<Batch>,
    ) -> Result<(), BatchError> {
        self.push_slice(record, self.question.clone(), closed)
    }

    /// Close the open batch at the end of input, if it holds a record.
    pub(crate) fn finish(&mut self) -> Result<Option<Batch>, BatchError> {
        self.close_open(Closed::End)
    }

    /// Close the open batch because input paused, if it holds a record.
    pub(crate) fn pause(&mut self) -> Result<Option<Batch>, BatchError> {
        self.close_open(Closed::Pause)
    }

    fn close_open(&mut self, closed: Closed) -> Result<Option<Batch>, BatchError> {
        (!self.open.members.is_empty())
            .then(|| self.close(closed))
            .transpose()
    }

    fn add(&mut self, joined: Joined, key: (String, String)) {
        self.open.bytes += joined.share + self.join_bytes(joined.wire);
        self.open.wires += joined.wire;
        self.open.seen.insert(key, self.open.distinct.len());
        self.open.members.push(self.open.distinct.len());
        self.open.distinct.push(joined);
    }

    /// The bytes a record adds beside its share: its separators and the
    /// extra digits its wire names take past `q1`, `q2`, and so on.
    fn join_bytes(&self, wire: usize) -> usize {
        if self.open.distinct.is_empty() {
            0
        } else {
            1 + names(self.open.wires, wire)
        }
    }

    /// Whether the open batch and this record fit every limit in the batched form.
    fn fits(&self, joined: &Joined) -> bool {
        let body = self.skeleton + self.open.bytes + joined.share + self.join_bytes(joined.wire);
        self.over(body, self.evidence, self.open.wires + joined.wire)
            .is_none()
    }

    /// The first limit these counts pass, with its value and the count.
    fn over(
        &self,
        body: usize,
        evidence: usize,
        wires: usize,
    ) -> Option<(LimitKind, usize, usize)> {
        let held = self.profile.as_ref();
        let request = Some(
            held.and_then(|held| held.max_request_bytes)
                .map_or(self.backend.ceiling(), |profile| {
                    profile.min(self.backend.ceiling())
                }),
        );
        let evidence_limit = held.and_then(|held| held.max_evidence_bytes);
        let limits = [
            (LimitKind::EvidenceBytes, evidence_limit, evidence),
            (LimitKind::RequestBytes, request, body),
            (
                LimitKind::Questions,
                held.and_then(|held| held.max_questions),
                wires,
            ),
        ];
        limits.into_iter().find_map(|(kind, most, actual)| {
            most.filter(|&most| actual > most)
                .map(|most| (kind, most, actual))
        })
    }

    /// Refuse a context whose request with one such record, or none, passes a limit.
    fn context_fits(&self, share: usize, wire: usize) -> Result<(), BatchError> {
        self.over(self.skeleton + share, self.evidence, wire)
            .map_or(Ok(()), |(kind, limit, actual)| {
                Err(BatchError::ContextOverLimit {
                    kind,
                    limit,
                    actual,
                })
            })
    }

    /// Encode the open batch once, check it, and start the next one.
    fn close(&mut self, closed: Closed) -> Result<Batch, BatchError> {
        let (plan, body) = self.built()?;
        let Open {
            distinct, members, ..
        } = std::mem::take(&mut self.open);
        let firsts: Vec<usize> = distinct
            .iter()
            .scan(0, |at, held| Some(std::mem::replace(at, *at + held.wire)))
            .collect();
        let logical: Vec<usize> = distinct
            .iter()
            .scan(0, |at, held| {
                Some(std::mem::replace(at, *at + held.base.len()))
            })
            .collect();
        let group_members = self.grouped.then(|| {
            members
                .iter()
                .filter_map(|&place| {
                    let held = distinct.get(place)?;
                    let first = *logical.get(place)?;
                    Some(GroupMember {
                        outcomes: (first..first + held.base.len()).collect(),
                    })
                })
                .collect()
        });
        Ok(Batch {
            digest: Exchange::new(self.backend.url(), &body).digest(),
            questions: members
                .iter()
                .filter_map(|&place| firsts.get(place).copied())
                .collect(),
            outcomes: members
                .iter()
                .filter_map(|&place| logical.get(place).copied())
                .collect(),
            row_questions: members
                .iter()
                .filter_map(|&place| distinct.get(place)?.base.first().cloned())
                .collect(),
            group_members,
            plan,
            body,
            closed,
        })
    }

    /// The open batch's plan and body, encoded once and checked against its
    /// counts and the profile.
    fn built(&self) -> Result<(Plan, Vec<u8>), BatchError> {
        let Open {
            distinct, bytes, ..
        } = &self.open;
        let batched = distinct.first().is_some_and(|held| held.question.is_some());
        let plan = match distinct.first() {
            Some(only) if !batched => Plan::new(
                only.record.evidence.clone(),
                self.model(),
                only.base.clone(),
            )
            .map_err(|_| defect())?,
            _ => self.plan(
                &distinct
                    .iter()
                    .filter_map(|held| held.question.as_deref())
                    .collect::<Vec<_>>(),
            )?,
        };
        let body = built_in::encode(&plan).map_err(|_| defect())?;
        let evidence = plan.evidence().as_text().map_err(|_| defect())?;
        if batched && (body.len(), evidence.len()) != (self.skeleton + *bytes, self.evidence) {
            return Err(BatchError::Defect(
                "a batch's body differs from its counted bytes",
            ));
        }
        if let Some(profile) = &self.profile {
            profile
                .check(&plan, &evidence, &body)
                .map_err(BatchError::Profile)?;
        }
        Ok((plan, body))
    }

    /// The batched plan of these records' quoted questions under the shared state.
    fn plan(&self, quoted: &[&[Question]]) -> Result<Plan, BatchError> {
        let questions = quoted.iter().flat_map(|asked| asked.iter().cloned());
        Plan::new(self.shared.clone(), self.model(), questions.collect()).map_err(|_| defect())
    }

    /// The wire questions and encoded bytes of the batched plan of these records.
    fn measured(&self, quoted: &[&[Question]]) -> Result<(usize, usize), BatchError> {
        let plan = self.plan(quoted)?;
        let body = built_in::encode(&plan).map_err(|_| defect())?;
        Ok((plan.wire_question_count(), body.len()))
    }

    /// The batched body with no record: a probe record alone, less its share,
    /// which two copies of it side by side reveal.
    fn skeleton(&self) -> Result<usize, BatchError> {
        let asked = questions::quote("\"0\"", &self.question)?.ok_or_else(defect)?;
        let (wire, one) = self.measured(&[&asked])?;
        let (_, two) = self.measured(&[&asked, &asked])?;
        let share = two
            .checked_sub(one + 1 + names(wire, wire))
            .ok_or_else(defect)?;
        one.checked_sub(share).ok_or_else(defect)
    }

    fn model(&self) -> crate::core::text::ModelName {
        self.backend.model().clone()
    }
}

/// The text every question kind asks, where the quote prefix goes.
fn text(question: &mut Question) -> &mut QuestionText {
    match question {
        Question::Decide { text, .. }
        | Question::Choose { text, .. }
        | Question::Tag { text, .. }
        | Question::Score { text, .. } => text,
    }
}

/// The extra digits `wire` names take from `q{from + 1}` on, past `q1` on.
fn names(from: usize, wire: usize) -> usize {
    let digits = |count: usize| count.to_string().len();
    (1..=wire).map(|at| digits(from + at) - digits(at)).sum()
}

fn defect() -> BatchError {
    BatchError::Defect("a batch could not be encoded")
}

#[cfg(test)]
mod tests;
