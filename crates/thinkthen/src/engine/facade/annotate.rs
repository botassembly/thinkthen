//! One record's annotate answers and their ordered assembly.
//!
//! The command and the public library both answer a question set here. Each
//! renders the [`Annotation`] its own way.

use crate::core::adapters::built_in::{self, DecodeError};
use crate::core::pack;
use crate::core::{
    AnnotatedAnswer, AnnotatedEntry, AnnotatedFailure, AnnotatedValue, AnswerOutcome, FailedValue,
    ModelName, Question, QuestionSet, Reply, Usage,
};
use crate::engine::error::Error;
use crate::engine::pipeline::Answered;

/// One set question's reply and the place it fills.
struct ChunkAnswer {
    places: Vec<usize>,
    reply: Reply,
    requests_sent: u64,
    replayed: bool,
    keys: Vec<String>,
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
                reply: question.reply,
                requests_sent: question.requests_sent,
                replayed: question.cached,
                keys: question.keys,
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

impl QuestionAnswer {
    /// One set question's outcome from its wire answers.
    pub(crate) fn read(place: usize, question: Question, own: &[Answered]) -> Result<Self, Error> {
        let model = own.first().map_or("", |answered| &*answered.answered_by);
        let stored: Vec<_> = own
            .iter()
            .map(|answered| answered.answer.as_deref().map_err(DecodeError::cause))
            .collect();
        let outcomes = pack::read(std::slice::from_ref(&question), &stored, model)?;
        let usage = own.iter().try_fold(Usage::new(0, 0), |total, answered| {
            total.checked_plus(answered.usage?)
        });
        let model = ModelName::reported(model).map_err(|_| Error::Defect("a reply named no model"))?;
        Ok(Self {
            place,
            reply: Reply::new(model, outcomes, usage),
            keys: own.iter().map(|answered| answered.key.hex()).collect(),
            requests_sent: own.iter().map(|answered| answered.requests_sent).sum(),
            cached: own.iter().all(|answered| answered.cached),
        })
    }
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
    let mut stored_model = None;
    for chunk in answered.into_iter().flat_map(|group| group.answered) {
        // A stored answer's model takes no part in the check, by ADR 0111.
        if chunk.replayed {
            stored_model.get_or_insert_with(|| chunk.reply.model().clone());
        } else {
            check_model(&mut annotation.model, chunk.reply.model(), requested)?;
        }
        annotation.requests.extend(chunk.keys.iter().cloned());
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
            requests_sent: crate::core::share(chunk.requests_sent, chunk.places.len(), position),
            replayed: chunk.replayed,
        });
        let digest = chunk.keys.first().cloned().unwrap_or_default();
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
