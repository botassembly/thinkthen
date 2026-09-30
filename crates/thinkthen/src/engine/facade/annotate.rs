//! One record's grouped annotate requests and their ordered assembly.
//!
//! The command and the public library both answer a question set here. Each
//! renders the [`Annotation`] its own way.

use super::{Chunk, Engine};
use crate::core::adapters::built_in;
use crate::core::{
    AnnotateBatchMeta, AnnotatedAnswer, AnnotatedEntry, AnnotatedFailure, AnnotatedValue,
    AnswerOutcome, Batch, BatchError, BatchMeta, FailedValue, ModelName, Plan, QuestionSet, Reply,
    Setting, Usage,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::public::{Error as PublicError, ErrorKind};
use std::sync::atomic::{AtomicU64, Ordering};

mod batch;
use batch::{group_completion, project_group};

/// One sent chunk's reply and the set places its questions fill.
pub(crate) struct ChunkAnswer {
    places: Vec<usize>,
    reply: Reply,
    digest: String,
    requests_sent: u64,
    replayed: bool,
    parent_request: Option<String>,
    parent_sent: u64,
    batch: Option<AnnotateBatchMeta>,
    parent_batch: Option<AnnotateBatchMeta>,
    /// The question keys, when the question pipeline answered this chunk.
    keys: Option<Vec<String>>,
}

/// Every chunk of one question group, and the one model they reported.
pub(crate) struct GroupAnswer {
    answered: Vec<ChunkAnswer>,
}

impl GroupAnswer {
    /// One group's questions as the question pipeline answered them: each
    /// set place with its outcome, its keys, its attempts share, and whether
    /// the store answered all of it.
    pub(crate) fn of_questions(questions: Vec<QuestionAnswer>) -> Self {
        let answered = questions
            .into_iter()
            .map(|question| ChunkAnswer {
                places: vec![question.place],
                digest: question.keys.first().cloned().unwrap_or_default(),
                reply: question.reply,
                requests_sent: question.requests_sent,
                replayed: question.cached,
                parent_request: None,
                parent_sent: 0,
                batch: None,
                parent_batch: None,
                keys: Some(question.keys),
            })
            .collect();
        Self { answered }
    }
}

/// One set question's answer from the question pipeline.
pub(crate) struct QuestionAnswer {
    pub(crate) place: usize,
    pub(crate) reply: Reply,
    pub(crate) keys: Vec<String>,
    pub(crate) requests_sent: u64,
    pub(crate) cached: bool,
}

/// One group's chunks, prepared once, and the question places each chunk asks.
pub(crate) struct PreparedGroup {
    places: Vec<Vec<usize>>,
    pub(crate) chunks: Vec<Chunk>,
}

/// One record's named values and details, in set order, with the metadata.
pub(crate) struct Annotation {
    pub(crate) values: Vec<(String, AnnotatedValue)>,
    pub(crate) details: Vec<(String, AnnotatedEntry)>,
    pub(crate) receipts: Vec<MemberReceipt>,
    pub(crate) model: Option<ModelName>,
    pub(crate) usage: Option<Usage>,
    pub(crate) requests: Vec<String>,
    pub(crate) batches: Vec<AnnotateBatchMeta>,
    pub(crate) requests_sent: u64,
    pub(crate) replayed: bool,
    pub(crate) failed_questions: usize,
}

/// One set member's even share of the request chunk that answered it.
pub(crate) struct MemberReceipt {
    pub(crate) usage: Option<Usage>,
    pub(crate) requests_sent: u64,
    pub(crate) replayed: bool,
    pub(crate) parent_request: Option<String>,
}

struct ParentAttempt {
    digest: String,
    sent: u64,
    total: usize,
    closed: crate::core::batch::Closed,
    setting: Setting,
}

/// Retain the original cause until the command or public host maps it.
pub(crate) enum GroupBatchFailure {
    Engine(Error),
    Batch(BatchError),
    AllFailed,
    Defect(&'static str),
}

impl From<Error> for GroupBatchFailure {
    fn from(error: Error) -> Self {
        Self::Engine(error)
    }
}

fn public_failure(error: GroupBatchFailure) -> PublicError {
    match error {
        GroupBatchFailure::Engine(error) => error.into(),
        GroupBatchFailure::Batch(error) => PublicError::refused(error),
        GroupBatchFailure::AllFailed => {
            PublicError::of(ErrorKind::Backend, "a backend question failed in a batch")
        }
        GroupBatchFailure::Defect(message) => PublicError::defect(message),
    }
}

impl Engine {
    /// Send one packed group slice and return one bounded fragment per input row.
    pub(crate) fn answer_group_batch(
        &self,
        work: &super::GroupWork,
        cancel: &Cancel,
    ) -> Result<crate::engine::schedule::Completed<Vec<GroupAnswer>, PublicError>, PublicError>
    {
        let done = self
            .answer_group_batch_raw(work, cancel)
            .map_err(public_failure)?;
        Ok(crate::engine::schedule::Completed {
            value: done.value,
            records: done.records,
            replayed: done.replayed,
            partial_failure: done.partial_failure,
            stop: done.stop.map(public_failure),
        })
    }

    /// One group request before host-specific error conversion.
    #[expect(
        clippy::too_many_lines,
        reason = "the first and second half keep one ordered result and stop contract"
    )]
    pub(crate) fn answer_group_batch_raw(
        &self,
        work: &super::GroupWork,
        cancel: &Cancel,
    ) -> Result<
        crate::engine::schedule::Completed<Vec<GroupAnswer>, GroupBatchFailure>,
        GroupBatchFailure,
    > {
        let super::GroupRequest::Packed {
            batch,
            records,
            questions,
        } = &work.request
        else {
            let super::GroupRequest::Legacy(prepared) = &work.request else {
                return Err(GroupBatchFailure::Defect(
                    "an annotate group lost its request",
                ));
            };
            let prepared = prepared
                .lock()
                .map_err(|_| GroupBatchFailure::Defect("an annotate preparation was poisoned"))?
                .take()
                .ok_or(GroupBatchFailure::Defect(
                    "an annotate preparation ran twice",
                ))?;
            return Ok(group_completion(
                vec![self.answer_group(prepared, cancel)?],
                work.sole_group,
            ));
        };
        let places = &work.places;
        let sole_group = work.sole_group;
        let attempted = AtomicU64::new(0);
        match self.ask_batch_with_attempts(batch, cancel, Some(&attempted)) {
            Ok(answered) => Ok(group_completion(
                project_group(batch, places, answered, None, work.group, work.setting)?,
                sole_group,
            )),
            Err(error)
                if (error.too_large() || matches!(error, Error::ReplayMiss(_)))
                    && records.len() > 1 =>
            {
                let [left, right] =
                    crate::core::group_halves(self.backend(), self.profile(), questions, records)
                        .map_err(GroupBatchFailure::Batch)?;
                let parent = ParentAttempt {
                    digest: batch.digest.as_str().to_owned(),
                    sent: attempted.load(Ordering::Relaxed),
                    total: records.len(),
                    closed: batch.closed,
                    setting: work.setting,
                };
                let left_answer = self.ask_batch(&left, cancel)?;
                let left = group_completion(
                    project_group(
                        &left,
                        places,
                        left_answer,
                        Some((&parent, 0)),
                        work.group,
                        work.setting,
                    )?,
                    sole_group,
                );
                if left.stop.is_some() {
                    return Ok(left);
                }
                let mut value = left.value;
                if let Some(stop) = cancel.stop() {
                    return Ok(crate::engine::schedule::Completed {
                        value,
                        records: 0,
                        replayed: 0,
                        partial_failure: false,
                        stop: Some(stop.into()),
                    });
                }
                let right_answer = match self.ask_batch(&right, cancel) {
                    Ok(answered) => answered,
                    Err(error) => {
                        return Ok(crate::engine::schedule::Completed {
                            value,
                            records: 0,
                            replayed: 0,
                            partial_failure: false,
                            stop: Some(error.into()),
                        });
                    }
                };
                value.extend(project_group(
                    &right,
                    places,
                    right_answer,
                    Some((&parent, value.len())),
                    work.group,
                    work.setting,
                )?);
                Ok(group_completion(value, sole_group))
            }
            Err(error) => Err(error.into()),
        }
    }

    /// Split one group's plan under the backend limits and name each chunk's places.
    pub(crate) fn prepare_group(
        &self,
        plan: &Plan,
        places: Vec<usize>,
    ) -> Result<PreparedGroup, Error> {
        let chunks = self.split(plan)?;
        let places = chunks
            .iter()
            .scan(places.into_iter(), |places, chunk| {
                Some(places.by_ref().take(chunk.plan.questions().len()).collect())
            })
            .collect();
        Ok(PreparedGroup { places, chunks })
    }

    /// Send one group's chunks in order and keep each reply with its places.
    pub(crate) fn answer_group(
        &self,
        group: PreparedGroup,
        cancel: &Cancel,
    ) -> Result<GroupAnswer, Error> {
        let mut answered = Vec::with_capacity(group.chunks.len());
        let mut model = None;
        let mut places = group.places.into_iter();
        self.ask_chunks(group.chunks, cancel, |result| {
            check_model(&mut model, result.reply.model(), self.backend().model())?;
            answered.push(ChunkAnswer {
                places: places
                    .next()
                    .ok_or(Error::Defect("an annotate chunk has no question places"))?,
                reply: result.reply,
                digest: result.request.as_str().to_owned(),
                requests_sent: result.requests_sent,
                replayed: result.replayed,
                parent_request: None,
                parent_sent: 0,
                batch: None,
                parent_batch: None,
                keys: None,
            });
            Ok::<(), Error>(())
        })?;
        Ok(GroupAnswer { answered })
    }

    /// Keep the original single-record seam for its focused engine tests.
    #[cfg(test)]
    pub(crate) fn annotate(
        &self,
        set: &QuestionSet,
        plan: impl Fn(&[usize]) -> Result<Plan, Error>,
        cancel: &Cancel,
    ) -> Result<Annotation, Error> {
        let prepared = set
            .groups()
            .into_iter()
            .map(|places| self.prepare_group(&plan(&places)?, places))
            .collect::<Result<Vec<_>, _>>()?;
        let answered = prepared
            .into_iter()
            .map(|group| self.answer_group(group, cancel))
            .collect::<Result<Vec<_>, _>>()?;
        assemble(set, answered, self.backend().model())
    }
}

/// Fold every group's replies into one record's values, in set order.
pub(crate) fn assemble(
    set: &QuestionSet,
    answered: Vec<GroupAnswer>,
    requested: &ModelName,
) -> Result<Annotation, Error> {
    let size = set.questions().len();
    let mut values: Vec<Option<AnnotatedValue>> = vec![None; size];
    let mut details: Vec<Option<AnnotatedEntry>> = vec![None; size];
    let mut receipts: Vec<Option<MemberReceipt>> = (0..size).map(|_| None).collect();
    let mut annotation = Annotation {
        values: Vec::new(),
        details: Vec::new(),
        receipts: Vec::new(),
        model: None,
        usage: Some(Usage::new(0, 0)),
        requests: Vec::new(),
        batches: Vec::new(),
        requests_sent: 0,
        replayed: true,
        failed_questions: 0,
    };
    let mut stored_model = None;
    for chunk in answered.into_iter().flat_map(|group| group.answered) {
        // A stored answer's model takes no part in the check, by ADR 0111.
        if chunk.keys.is_some() && chunk.replayed {
            stored_model.get_or_insert_with(|| chunk.reply.model().clone());
        } else {
            check_model(&mut annotation.model, chunk.reply.model(), requested)?;
        }
        if let Some(parent) = &chunk.parent_request {
            annotation.requests.push(parent.clone());
            if let Some(meta) = &chunk.parent_batch {
                annotation.batches.push(meta.clone());
            }
        }
        match &chunk.keys {
            Some(keys) => annotation.requests.extend(keys.iter().cloned()),
            None => annotation.requests.push(chunk.digest.clone()),
        }
        if let Some(meta) = &chunk.batch {
            annotation.batches.push(meta.clone());
        }
        annotation.usage = match (annotation.usage, chunk.reply.usage()) {
            (Some(total), Some(next)) => {
                Some(total.checked_plus(next).ok_or(Error::UsageOverflow)?)
            }
            _ => None,
        };
        annotation.replayed &= chunk.replayed;
        annotation.requests_sent = annotation
            .requests_sent
            .checked_add(chunk.requests_sent)
            .and_then(|total| total.checked_add(chunk.parent_sent))
            .ok_or(Error::Defect("a request count overflowed"))?;
        annotation.failed_questions +=
            take_answers(set, &chunk, &mut values, &mut details, &mut receipts)?;
    }
    annotation.model = annotation.model.or(stored_model);
    annotation.values = pair(set, values, "a question has no value")?;
    annotation.details = pair(set, details, "a question has no detailed answer")?;
    annotation.receipts = receipts
        .into_iter()
        .map(|receipt| receipt.ok_or(Error::Defect("a question has no request receipt")))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(annotation)
}

fn take_answers(
    set: &QuestionSet,
    chunk: &ChunkAnswer,
    values: &mut [Option<AnnotatedValue>],
    details: &mut [Option<AnnotatedEntry>],
    receipts: &mut [Option<MemberReceipt>],
) -> Result<usize, Error> {
    if chunk.places.len() != chunk.reply.outcomes().len() {
        return Err(Error::Defect(
            "the adapter answered the wrong number of questions",
        ));
    }
    let mut failed = 0;
    for (position, (place, outcome)) in chunk.places.iter().zip(chunk.reply.outcomes()).enumerate()
    {
        let question = set
            .questions()
            .get(*place)
            .ok_or(Error::Defect("a group points outside its set"))?;
        let (Some(value_slot), Some(detail_slot)) =
            (values.get_mut(*place), details.get_mut(*place))
        else {
            return Err(Error::Defect("an answer points outside its set"));
        };
        let Some(receipt_slot) = receipts.get_mut(*place) else {
            return Err(Error::Defect("a receipt points outside its set"));
        };
        *receipt_slot = Some(MemberReceipt {
            usage: chunk
                .reply
                .usage()
                .map(|whole| whole.share(chunk.places.len(), position)),
            requests_sent: crate::core::share(chunk.requests_sent, chunk.places.len(), position)
                + crate::core::share(chunk.parent_sent, chunk.places.len(), position),
            replayed: chunk.replayed,
            parent_request: chunk.parent_request.clone(),
        });
        let digest = chunk.digest.clone();
        match outcome {
            AnswerOutcome::Answered(answer) => {
                let (value, _) = answer.read(question.threshold());
                *value_slot = Some(AnnotatedValue::Answered(value.clone()));
                *detail_slot = Some(AnnotatedEntry::Answered(AnnotatedAnswer::new(
                    value,
                    question.question().clone(),
                    answer.clone(),
                    question.threshold(),
                    digest,
                )));
            }
            AnswerOutcome::Failed(failure) => {
                failed += 1;
                *value_slot = Some(AnnotatedValue::Failed(FailedValue::new(*failure)));
                *detail_slot = Some(AnnotatedEntry::Failed(AnnotatedFailure::new(
                    question.question().clone(),
                    *failure,
                    digest,
                )));
            }
        }
    }
    Ok(failed)
}

fn pair<T>(
    set: &QuestionSet,
    values: Vec<Option<T>>,
    absent: &'static str,
) -> Result<Vec<(String, T)>, Error> {
    set.questions()
        .iter()
        .zip(values)
        .map(|(question, value)| {
            Ok((
                question.name().to_owned(),
                value.ok_or(Error::Defect(absent))?,
            ))
        })
        .collect()
}

/// Keep the first model a record's replies named, and refuse a second.
pub(crate) fn check_model(
    first: &mut Option<ModelName>,
    next: &ModelName,
    requested: &ModelName,
) -> Result<(), Error> {
    let Some(held) = first else {
        *first = Some(next.clone());
        return Ok(());
    };
    if held == next {
        return Ok(());
    }
    let safe = built_in::diagnostic_model(held.as_str(), requested.as_str())
        .then(|| held.as_str().to_owned())
        .zip(
            built_in::diagnostic_model(next.as_str(), requested.as_str())
                .then(|| next.as_str().to_owned()),
        )
        .map(|(first, second)| {
            if first <= second {
                (first, second)
            } else {
                (second, first)
            }
        });
    Err(Error::ModelsDiffer(safe))
}
