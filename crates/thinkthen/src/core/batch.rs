//! Batches: runs of consecutive records that share one request, by ADR 0048
//! items 1 and 2.
//!
//! A batch without a context sends `{"records":[…]}` as its evidence and one
//! quoted question per distinct record. A batch of one distinct record without
//! a context sends today's request of that record, byte for byte. The batcher
//! keeps running byte counts, so it encodes each record once and each batch
//! once, and it checks every closed body against those counts.

#![cfg_attr(
    not(test),
    expect(dead_code, reason = "ticket B4 puts batches on the command")
)]

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroUsize;

use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::core::adapters::built_in;
use crate::core::backend::Backend;
use crate::core::backend_profile::{BackendProfile, LimitKind, ProfileLimit};
use crate::core::json::Json;
use crate::core::plan::Plan;
use crate::core::question::Question;
use crate::core::recording::{Digest, Exchange};
use crate::core::render::json_line;
use crate::core::text::{Evidence, QuestionText};

/// A record closes its batch when its content hash is 0 mod this.
const CUT: u64 = 4_096;

/// How many records a batch may hold: as many as fit, or at most `N`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Setting {
    Max,
    Records(NonZeroUsize),
}

/// One record as a batch reads it: today's evidence for a batch of one, and
/// the JSON value a batch quotes, lists, hashes, and compares for copies.
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
    End,
}

/// One closed batch: the one request it sends and the records it answers.
pub(crate) struct Batch {
    pub(crate) plan: Plan,
    pub(crate) body: Vec<u8>,
    pub(crate) digest: Digest,
    /// Each record's first wire question, in input order. Copies share one.
    pub(crate) questions: Vec<usize>,
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
    question: Option<Question>,
    wire: usize,
    share: usize,
    line: usize,
}

/// Plans batches from records in input order.
pub(crate) struct Batcher {
    backend: Backend,
    profile: Option<BackendProfile>,
    question: Question,
    /// The question's text, or `None` for a question written as JSON.
    text: Option<String>,
    size: Option<usize>,
    context: Option<Evidence>,
    /// The batched body's bytes with no record in it.
    skeleton: usize,
    /// The bytes of `{"records":[]}`, or of the context.
    evidence: usize,
    distinct: Vec<Joined>,
    seen: BTreeMap<String, usize>,
    members: Vec<usize>,
    /// Running totals of the distinct records' batched shares.
    bytes: usize,
    lines: usize,
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
        let mut asked = question.clone();
        let text = text(&mut asked).as_json().as_str().map(str::to_owned);
        let quotable = text.is_some();
        if context.is_some() && !quotable {
            return Err(BatchError::StructuredQuestionWithContext);
        }
        let size = match setting {
            _ if !quotable => Some(1),
            Setting::Max => None,
            Setting::Records(most) => Some(most.get()),
        };
        let evidence = match &context {
            Some(context) => context.as_text().map_err(|_| defect())?.len(),
            None => json_line(&records(Vec::new())).map_err(|_| defect())?.len(),
        };
        let mut batcher = Self {
            backend,
            profile,
            question,
            text,
            size,
            context,
            skeleton: 0,
            evidence,
            distinct: Vec::new(),
            seen: BTreeMap::new(),
            members: Vec::new(),
            bytes: 0,
            lines: 0,
            wires: 0,
        };
        if quotable {
            batcher.skeleton = batcher.skeleton()?;
        }
        if batcher.context.is_some() {
            batcher.context_fits(0, 0)?;
        }
        Ok(batcher)
    }

    /// Add one record, and return the batches that closed: zero, one, or two.
    pub(crate) fn push(&mut self, record: BatchRecord) -> Result<Vec<Batch>, BatchError> {
        let line = json_line(&record.value).map_err(|_| defect())?;
        let cut = Sha256::digest(line.as_bytes())
            .first_chunk::<8>()
            .is_some_and(|head| u64::from_be_bytes(*head) % CUT == 0);
        let mut closed = Vec::new();
        if let Some(&place) = self.seen.get(&line) {
            self.members.push(place);
        } else {
            let joined = self.joined(record, line.len())?;
            if self.context.is_some() {
                self.context_fits(joined.share, joined.wire)?;
            }
            if !self.distinct.is_empty() && !self.fits(&joined) {
                closed.push(self.close(Closed::Limit)?);
            }
            self.add(joined, line);
        }
        if cut {
            closed.push(self.close(Closed::Content)?);
        } else if self.size == Some(self.members.len()) {
            closed.push(self.close(Closed::Size)?);
        }
        Ok(closed)
    }

    /// Close the open batch at the end of input, if it holds a record.
    pub(crate) fn finish(&mut self) -> Result<Option<Batch>, BatchError> {
        (!self.members.is_empty())
            .then(|| self.close(Closed::End))
            .transpose()
    }

    /// Quote one record and measure its batched share from one encode.
    fn joined(&self, record: BatchRecord, line: usize) -> Result<Joined, BatchError> {
        let Some(question) = self.quoted(&record.value)? else {
            return Ok(Joined {
                record,
                question: None,
                wire: 1,
                share: 0,
                line,
            });
        };
        let wire = Plan::new(
            record.evidence.clone(),
            self.model(),
            vec![question.clone()],
        )
        .map_err(|_| defect())?
        .wire_question_count();
        let alone = self.encoded(&[(&record.value, &question)])?.len();
        let share = alone.checked_sub(self.skeleton).ok_or_else(defect)?;
        Ok(Joined {
            record,
            question: Some(question),
            wire,
            share,
            line,
        })
    }

    fn add(&mut self, joined: Joined, line: String) {
        let (share, wire) = (joined.share, joined.wire);
        self.bytes += share + self.join_bytes(wire);
        self.lines += joined.line + usize::from(!self.distinct.is_empty());
        self.wires += wire;
        self.seen.insert(line, self.distinct.len());
        self.members.push(self.distinct.len());
        self.distinct.push(joined);
    }

    /// The bytes a record adds beside its share: its separators and the
    /// extra digits its wire names take past `q1`, `q2`, and so on.
    fn join_bytes(&self, wire: usize) -> usize {
        if self.distinct.is_empty() {
            return 0;
        }
        let separators = if self.context.is_some() { 1 } else { 2 };
        separators
            + (0..wire)
                .map(|at| digits(self.wires + at + 1) - digits(at + 1))
                .sum::<usize>()
    }

    /// Whether the open batch and this record fit every limit in the batched form.
    fn fits(&self, joined: &Joined) -> bool {
        let body = self.skeleton + self.bytes + joined.share + self.join_bytes(joined.wire);
        let evidence = match self.context {
            Some(_) => self.evidence,
            None => self.evidence + self.lines + joined.line + 1,
        };
        self.over(body, evidence, self.wires + joined.wire)
            .is_none()
    }

    /// The first limit these counts pass, with its value and the count.
    fn over(
        &self,
        body: usize,
        evidence: usize,
        wires: usize,
    ) -> Option<(LimitKind, usize, usize)> {
        let profile = self.profile.as_ref();
        let request = profile
            .and_then(|held| held.max_request_bytes)
            .or(self.backend.ceiling());
        [
            (
                LimitKind::EvidenceBytes,
                profile.and_then(|held| held.max_evidence_bytes),
                evidence,
            ),
            (LimitKind::RequestBytes, request, body),
            (
                LimitKind::Questions,
                profile.and_then(|held| held.max_questions),
                wires,
            ),
        ]
        .into_iter()
        .find_map(|(kind, limit, actual)| {
            limit
                .filter(|&most| actual > most)
                .map(|most| (kind, most, actual))
        })
    }

    /// Refuse a context whose request with one such record, or none, passes a limit.
    fn context_fits(&self, share: usize, wire: usize) -> Result<(), BatchError> {
        match self.over(self.skeleton + share, self.evidence, wire) {
            Some((kind, limit, actual)) => Err(BatchError::ContextOverLimit {
                kind,
                limit,
                actual,
            }),
            None => Ok(()),
        }
    }

    /// Encode the open batch once, check it, and start the next one.
    fn close(&mut self, closed: Closed) -> Result<Batch, BatchError> {
        let distinct = std::mem::take(&mut self.distinct);
        let members = std::mem::take(&mut self.members);
        let batched = self.context.is_some() || distinct.len() > 1;
        let plan = match (batched, distinct.first()) {
            (false, Some(only)) => Plan::new(
                only.record.evidence.clone(),
                self.model(),
                vec![self.question.clone()],
            ),
            _ => Plan::new(
                self.evidence_of(&distinct)?,
                self.model(),
                distinct
                    .iter()
                    .filter_map(|held| held.question.clone())
                    .collect(),
            ),
        }
        .map_err(|_| defect())?;
        let body = built_in::encode(&plan).map_err(|_| defect())?;
        if batched && body.len() != self.skeleton + self.bytes {
            return Err(BatchError::Defect(
                "a batch's body differs from its counted bytes",
            ));
        }
        if let Some(profile) = &self.profile {
            let evidence = plan.evidence().as_text().map_err(|_| defect())?;
            profile
                .check(&plan, &evidence, &body)
                .map_err(BatchError::Profile)?;
        }
        let firsts: Vec<usize> = distinct
            .iter()
            .scan(0, |at, held| Some(std::mem::replace(at, *at + held.wire)))
            .collect();
        (self.bytes, self.lines, self.wires) = (0, 0, 0);
        self.seen.clear();
        Ok(Batch {
            digest: Exchange::new(self.backend.url(), &body).digest(),
            questions: members
                .iter()
                .filter_map(|&place| firsts.get(place).copied())
                .collect(),
            plan,
            body,
            closed,
        })
    }

    /// The batched evidence: the context, or the distinct records' values.
    fn evidence_of(&self, distinct: &[Joined]) -> Result<Evidence, BatchError> {
        match &self.context {
            Some(context) => Ok(context.clone()),
            None => Evidence::structured(records(
                distinct
                    .iter()
                    .map(|held| held.record.value.clone())
                    .collect(),
            ))
            .map_err(|_| defect()),
        }
    }

    /// The batched body of these records, encoded.
    fn encoded(&self, held: &[(&Json, &Question)]) -> Result<Vec<u8>, BatchError> {
        let evidence = match &self.context {
            Some(context) => context.clone(),
            None => Evidence::structured(records(
                held.iter().map(|(value, _)| (*value).clone()).collect(),
            ))
            .map_err(|_| defect())?,
        };
        let questions = held
            .iter()
            .map(|(_, question)| (*question).clone())
            .collect();
        let plan = Plan::new(evidence, self.model(), questions).map_err(|_| defect())?;
        built_in::encode(&plan).map_err(|_| defect())
    }

    /// The batched body with no record: one probe record alone, less its
    /// share, which two copies of it side by side reveal.
    fn skeleton(&self) -> Result<usize, BatchError> {
        let probe = Json::String("0".to_owned());
        let question = self.quoted(&probe)?.ok_or_else(defect)?;
        let wire = Plan::new(
            Evidence::new("0").map_err(|_| defect())?,
            self.model(),
            vec![question.clone()],
        )
        .map_err(|_| defect())?
        .wire_question_count();
        let one = self.encoded(&[(&probe, &question)])?.len();
        let two = self
            .encoded(&[(&probe, &question), (&probe, &question)])?
            .len();
        let separators = if self.context.is_some() { 1 } else { 2 };
        let names: usize = (0..wire)
            .map(|at| digits(wire + at + 1) - digits(at + 1))
            .sum();
        let share = two
            .checked_sub(one + separators + names)
            .ok_or_else(defect)?;
        one.checked_sub(share).ok_or_else(defect)
    }

    /// The question with `The text is `, the record's JSON, and `. ` before its text.
    /// `None` when the question is written as JSON and cannot be quoted.
    fn quoted(&self, value: &Json) -> Result<Option<Question>, BatchError> {
        let Some(asked) = &self.text else {
            return Ok(None);
        };
        let line = json_line(value).map_err(|_| defect())?;
        let mut question = self.question.clone();
        *text(&mut question) =
            QuestionText::new(format!("The text is {line}. {asked}")).map_err(|_| defect())?;
        Ok(Some(question))
    }

    fn model(&self) -> crate::core::text::ModelName {
        self.backend.model().clone()
    }
}

/// `{"records":[…]}` over these values.
fn records(values: Vec<Json>) -> Json {
    Json::Object(vec![("records".to_owned(), Json::Array(values))])
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

/// The decimal digits of a count.
fn digits(count: usize) -> usize {
    count.to_string().len()
}

fn defect() -> BatchError {
    BatchError::Defect("a batch could not be encoded")
}

#[cfg(test)]
mod tests;
