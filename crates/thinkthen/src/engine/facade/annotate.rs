//! One record's grouped annotate requests and their ordered assembly.
//!
//! The command and the public library both answer a question set here. Each
//! renders the [`Annotation`] its own way.

use super::{Chunk, Engine};
use crate::core::adapters::built_in;
use crate::core::{
    AnnotatedAnswer, AnnotatedEntry, AnnotatedFailure, AnnotatedValue, AnswerOutcome, Batch,
    FailedValue, ModelName, Plan, QuestionSet, Reply, Usage,
};
use crate::engine::Cancel;
use crate::engine::error::Error;
use crate::public::{Error as PublicError, ErrorKind};
use std::sync::atomic::{AtomicU64, Ordering};

/// One sent chunk's reply and the set places its questions fill.
pub(crate) struct ChunkAnswer {
    places: Vec<usize>,
    reply: Reply,
    digest: String,
    requests_sent: u64,
    replayed: bool,
    parent_request: Option<String>,
    parent_sent: u64,
}

/// Every chunk of one question group, and the one model they reported.
pub(crate) struct GroupAnswer {
    answered: Vec<ChunkAnswer>,
    pub(crate) model: Option<ModelName>,
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
}

impl Engine {
    /// Send one packed group slice and return one bounded fragment per input row.
    pub(crate) fn answer_group_batch(
        &self,
        work: &super::GroupWork,
        cancel: &Cancel,
    ) -> Result<crate::engine::schedule::Completed<Vec<GroupAnswer>, PublicError>, PublicError>
    {
        let super::GroupRequest::Packed {
            batch,
            records,
            questions,
        } = &work.request
        else {
            let super::GroupRequest::Legacy(prepared) = &work.request else {
                return Err(PublicError::defect("an annotate group lost its request"));
            };
            let prepared = prepared
                .lock()
                .map_err(|_| PublicError::defect("an annotate preparation was poisoned"))?
                .take()
                .ok_or_else(|| PublicError::defect("an annotate preparation ran twice"))?;
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
                project_group(batch, places, answered, None)?,
                sole_group,
            )),
            Err(error) if error.too_large() && records.len() > 1 => {
                let [left, right] =
                    crate::core::group_halves(self.backend(), self.profile(), questions, records)
                        .map_err(PublicError::refused)?;
                let parent = ParentAttempt {
                    digest: batch.digest.as_str().to_owned(),
                    sent: attempted.load(Ordering::Relaxed),
                    total: records.len(),
                };
                let left_answer = self.ask_batch(&left, cancel)?;
                let left = group_completion(
                    project_group(&left, places, left_answer, Some((&parent, 0)))?,
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
            });
            Ok::<(), Error>(())
        })?;
        Ok(GroupAnswer { answered, model })
    }

    /// Prepare every group of one text before sending any, as the command
    /// does, then answer them in set order and assemble the record.
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
        let mut answered = Vec::new();
        let mut model = None;
        for group in prepared {
            let group = self.answer_group(group, cancel)?;
            if let Some(reported) = &group.model {
                check_model(&mut model, reported, self.backend().model())?;
            }
            answered.push(group);
        }
        assemble(set, answered, self.backend().model())
    }
}

fn project_group(
    batch: &Batch,
    places: &[usize],
    answered: super::Answered,
    parent: Option<(&ParentAttempt, usize)>,
) -> Result<Vec<GroupAnswer>, Error> {
    let members = batch
        .group_members
        .as_ref()
        .ok_or(Error::Defect("an annotate batch has no group members"))?;
    let mut fragments = Vec::with_capacity(members.len());
    for (position, member) in members.iter().enumerate() {
        if member.outcomes.len() != places.len() {
            return Err(Error::Defect("an annotate batch lost its group slice"));
        }
        let outcomes = member
            .outcomes
            .iter()
            .map(|&at| {
                answered
                    .reply
                    .outcomes()
                    .get(at)
                    .cloned()
                    .ok_or(Error::Defect("an annotate batch lost an answer"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let reply = Reply::new(
            answered.reply.model().clone(),
            outcomes,
            answered
                .reply
                .usage()
                .map(|usage| usage.share(members.len(), position)),
        );
        let parent_sent = parent.map_or(0, |(attempt, offset)| {
            crate::core::share(attempt.sent, attempt.total, offset + position)
        });
        fragments.push(GroupAnswer {
            answered: vec![ChunkAnswer {
                places: places.to_vec(),
                reply,
                digest: answered.request.as_str().to_owned(),
                requests_sent: crate::core::share(answered.requests_sent, members.len(), position),
                replayed: answered.replayed,
                parent_request: parent.map(|(attempt, _)| attempt.digest.clone()),
                parent_sent,
            }],
            model: Some(answered.reply.model().clone()),
        });
    }
    Ok(fragments)
}

fn group_completion(
    mut value: Vec<GroupAnswer>,
    sole_group: bool,
) -> crate::engine::schedule::Completed<Vec<GroupAnswer>, PublicError> {
    let stop = if sole_group {
        value
            .iter()
            .position(|one| {
                one.answered.iter().all(|chunk| {
                    chunk
                        .reply
                        .outcomes()
                        .iter()
                        .all(|answer| matches!(answer, AnswerOutcome::Failed(_)))
                })
            })
            .map(|index| {
                value.truncate(index + 1);
                PublicError::of(ErrorKind::Backend, "a backend question failed in a batch")
            })
    } else {
        None
    };
    crate::engine::schedule::Completed {
        value,
        records: 0,
        replayed: 0,
        partial_failure: false,
        stop,
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
        requests_sent: 0,
        replayed: true,
        failed_questions: 0,
    };
    for chunk in answered.into_iter().flat_map(|group| group.answered) {
        check_model(&mut annotation.model, chunk.reply.model(), requested)?;
        if let Some(parent) = &chunk.parent_request {
            annotation.requests.push(parent.clone());
        }
        annotation.requests.push(chunk.digest.clone());
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
