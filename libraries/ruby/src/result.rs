//! Owned call accounting copied before the Ruby worker hands its result over.

use std::fmt;

use thinkthen::{Facts, FailureCause, Judgment, Probabilities, RecordObservation, Usage};

use crate::call::Output;

#[derive(Clone)]
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

impl fmt::Debug for Detail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Detail")
            .field("index", &self.index)
            .field("answered", &self.answer.is_some())
            .field("failed", &self.failed.is_some())
            .finish()
    }
}

impl Detail {
    pub(crate) fn copy(event: RecordObservation<'_>) -> Option<Self> {
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
            index,
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

#[derive(Debug)]
pub(crate) struct Completed {
    pub(crate) value: Output,
    pub(crate) facts: Facts,
    pub(crate) details: Vec<Detail>,
}
