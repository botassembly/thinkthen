//! Owned, bounded call evidence carried from the worker to R's main thread.

use std::sync::{Mutex, PoisonError};
use std::time::Instant;

use thinkthen::{
    Batch, Error, Facts, FailureCause, Judgment, Probabilities, RecordObservation, Usage,
};

use crate::defect;

#[derive(Clone, Debug)]
pub(crate) struct Detail {
    pub(crate) index: usize,
    pub(crate) member: Option<String>,
    pub(crate) stage: Option<&'static str>,
    pub(crate) position: usize,
    pub(crate) question_sha256: String,
    pub(crate) answer: Option<Judgment>,
    pub(crate) failed: Option<FailureCause>,
    pub(crate) probabilities: Option<Probabilities>,
    pub(crate) confidence: Option<f64>,
    pub(crate) model: String,
    pub(crate) url: String,
    pub(crate) requests: Vec<String>,
    pub(crate) requests_sent: u64,
    pub(crate) usage: Option<Usage>,
    pub(crate) cached: bool,
    pub(crate) failed_questions: usize,
}

impl Detail {
    fn copy(event: RecordObservation<'_>, original: Option<usize>) -> Option<Self> {
        let RecordObservation::Question {
            index,
            member,
            stage,
            position,
            detail,
        } = event
        else {
            return None;
        };
        Some(Self {
            index: original.unwrap_or(index),
            member: member.map(str::to_owned),
            stage,
            position,
            question_sha256: detail.question_sha256().to_owned(),
            answer: detail.value().cloned(),
            failed: detail.failure(),
            probabilities: detail.probabilities().cloned(),
            confidence: detail.confidence(),
            model: detail.model().to_owned(),
            url: detail.url().to_owned(),
            requests: detail.requests().to_vec(),
            requests_sent: detail.requests_sent(),
            usage: detail.usage(),
            cached: detail.cached(),
            failed_questions: detail.failed_questions(),
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Counts {
    pub(crate) records: u64,
    pub(crate) requests_sent: u64,
    pub(crate) cache_answers: u64,
    pub(crate) input_tokens: Option<u64>,
    pub(crate) output_tokens: Option<u64>,
    pub(crate) seconds: f64,
    pub(crate) model: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Account {
    started: Instant,
    positions: Option<Vec<usize>>,
    counts: Mutex<Option<Counts>>,
    details: Mutex<Vec<Detail>>,
}

impl Account {
    pub(crate) fn new(positions: Option<Vec<usize>>) -> Self {
        Self {
            started: Instant::now(),
            positions,
            counts: Mutex::new(None),
            details: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn observe(&self, event: RecordObservation<'_>, original: Option<usize>) {
        let original = original.or_else(|| match &event {
            RecordObservation::Question { index, .. } => self
                .positions
                .as_ref()
                .and_then(|positions| positions.get(*index).copied()),
            _ => None,
        });
        if let Some(detail) = Detail::copy(event, original) {
            self.details
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push(detail);
        }
    }

    pub(crate) fn add(&self, facts: &Facts) -> Result<(), String> {
        const MAX_EXACT: u64 = (1 << 53) - 1;
        let mut counts = self.counts.lock().unwrap_or_else(PoisonError::into_inner);
        if [
            facts.records(),
            facts.requests_sent(),
            facts.cache_answers(),
        ]
        .into_iter()
        .any(|value| value > MAX_EXACT)
            || [facts.input_tokens(), facts.output_tokens()]
                .into_iter()
                .flatten()
                .any(|value| value > MAX_EXACT)
        {
            return Err(defect("the R call's facts exceeded their exact range"));
        }
        let Some(current) = counts.as_mut() else {
            *counts = Some(Counts {
                records: facts.records(),
                requests_sent: facts.requests_sent(),
                cache_answers: facts.cache_answers(),
                input_tokens: facts.input_tokens(),
                output_tokens: facts.output_tokens(),
                seconds: 0.0,
                model: facts.model().map(str::to_owned),
            });
            return Ok(());
        };
        let overflow = || defect("the R call's facts exceeded their exact range");
        let mut next = current.clone();
        next.records = next
            .records
            .checked_add(facts.records())
            .ok_or_else(overflow)?;
        next.requests_sent = next
            .requests_sent
            .checked_add(facts.requests_sent())
            .ok_or_else(overflow)?;
        next.cache_answers = next
            .cache_answers
            .checked_add(facts.cache_answers())
            .ok_or_else(overflow)?;
        if [next.records, next.requests_sent, next.cache_answers]
            .into_iter()
            .any(|value| value > MAX_EXACT)
        {
            return Err(overflow());
        }
        next.input_tokens = match (next.input_tokens, facts.input_tokens()) {
            (Some(one), Some(two)) => Some(one.checked_add(two).ok_or_else(overflow)?),
            _ => None,
        };
        next.output_tokens = match (next.output_tokens, facts.output_tokens()) {
            (Some(one), Some(two)) => Some(one.checked_add(two).ok_or_else(overflow)?),
            _ => None,
        };
        if [next.input_tokens, next.output_tokens]
            .into_iter()
            .flatten()
            .any(|value| value > MAX_EXACT)
        {
            return Err(overflow());
        }
        if next.model.as_deref() != facts.model() {
            next.model = None;
        }
        *current = next;
        Ok(())
    }

    pub(crate) fn failed(&self, error: &Error) -> Result<String, String> {
        if let Some(facts) = error.facts() {
            self.add(facts)?;
        }
        Ok(crate::carry(error))
    }

    pub(crate) fn no_work(&self) {
        let mut counts = self.counts.lock().unwrap_or_else(PoisonError::into_inner);
        if counts.is_none() {
            *counts = Some(Counts {
                records: 0,
                requests_sent: 0,
                cache_answers: 0,
                input_tokens: None,
                output_tokens: None,
                seconds: 0.0,
                model: None,
            });
        }
    }

    pub(crate) fn finish(self, no_work: bool) -> Snapshot {
        let mut counts = self
            .counts
            .into_inner()
            .unwrap_or_else(PoisonError::into_inner);
        if no_work && counts.is_none() {
            counts = Some(Counts {
                records: 0,
                requests_sent: 0,
                cache_answers: 0,
                input_tokens: None,
                output_tokens: None,
                seconds: 0.0,
                model: None,
            });
        }
        if let Some(ref mut facts) = counts {
            facts.seconds = self.started.elapsed().as_secs_f64();
        }
        Snapshot {
            facts: counts,
            details: self
                .details
                .into_inner()
                .unwrap_or_else(PoisonError::into_inner),
        }
    }
}

/// Exhaust a lazy batch through its one terminal error, then read its final account.
pub(crate) fn collect<T>(batch: &mut Batch<'_, T>, account: &Account) -> Result<Vec<T>, String> {
    let mut values = Vec::new();
    let mut failure = None;
    for row in batch.by_ref() {
        match row {
            Ok(value) => values.push(value),
            Err(error) => {
                failure = Some(crate::carry(&error));
                break;
            }
        }
    }
    if let Some(facts) = batch.facts() {
        account.add(facts)?;
    }
    if let Some(error) = failure {
        return Err(error);
    }
    Ok(values)
}

#[derive(Clone, Debug)]
pub(crate) struct Snapshot {
    pub(crate) facts: Option<Counts>,
    pub(crate) details: Vec<Detail>,
}

pub(crate) struct Completed<T> {
    pub(crate) result: Result<T, String>,
    pub(crate) snapshot: Snapshot,
}

impl<T> Completed<T> {
    pub(crate) fn map<U>(self, convert: impl FnOnce(T) -> Result<U, String>) -> Completed<U> {
        Completed {
            result: self.result.and_then(convert),
            snapshot: self.snapshot,
        }
    }
}
