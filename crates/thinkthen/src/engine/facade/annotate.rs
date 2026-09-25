//! One record's grouped annotate requests and their ordered assembly.
//!
//! The command and the public library both answer a question set here. Each
//! renders the [`Annotation`] its own way.

use super::{Chunk, Engine};
use crate::core::adapters::built_in;
use crate::core::{
    AnnotatedAnswer, AnnotatedEntry, AnnotatedFailure, AnnotatedValue, AnswerOutcome, FailedValue,
    ModelName, Plan, QuestionSet, Reply, Usage,
};
use crate::engine::Cancel;
use crate::engine::error::Error;

/// One sent chunk's reply and the set places its questions fill.
pub(crate) struct ChunkAnswer {
    places: Vec<usize>,
    reply: Reply,
    digest: String,
    requests_sent: u64,
    replayed: bool,
}

/// Every chunk of one question group, and the one model they reported.
pub(crate) struct GroupAnswer {
    answered: Vec<ChunkAnswer>,
    pub(crate) model: Option<ModelName>,
}

/// One group's chunks, prepared once, and the question places each chunk asks.
pub(crate) struct PreparedGroup {
    places: Vec<Vec<usize>>,
    chunks: Vec<Chunk>,
}

/// One record's named values and details, in set order, with the metadata.
pub(crate) struct Annotation {
    pub(crate) values: Vec<(String, AnnotatedValue)>,
    pub(crate) details: Vec<(String, AnnotatedEntry)>,
    pub(crate) model: Option<ModelName>,
    pub(crate) usage: Option<Usage>,
    pub(crate) requests: Vec<String>,
    pub(crate) requests_sent: u64,
    pub(crate) replayed: bool,
    pub(crate) failed_questions: usize,
}

impl Engine {
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
            });
            Ok::<(), Error>(())
        })?;
        Ok(GroupAnswer { answered, model })
    }

    /// Answer every group of one text in set order and assemble the record.
    pub(crate) fn annotate(
        &self,
        set: &QuestionSet,
        plan: impl Fn(&[usize]) -> Result<Plan, Error>,
        cancel: &Cancel,
    ) -> Result<Annotation, Error> {
        let mut answered = Vec::new();
        let mut model = None;
        for places in set.groups() {
            let group = self.prepare_group(&plan(&places)?, places)?;
            let group = self.answer_group(group, cancel)?;
            if let Some(reported) = &group.model {
                check_model(&mut model, reported, self.backend().model())?;
            }
            answered.push(group);
        }
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
    let mut annotation = Annotation {
        values: Vec::new(),
        details: Vec::new(),
        model: None,
        usage: Some(Usage::new(0, 0)),
        requests: Vec::new(),
        requests_sent: 0,
        replayed: true,
        failed_questions: 0,
    };
    for chunk in answered.into_iter().flat_map(|group| group.answered) {
        check_model(&mut annotation.model, chunk.reply.model(), requested)?;
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
            .ok_or(Error::Defect("a request count overflowed"))?;
        annotation.failed_questions += take_answers(set, &chunk, &mut values, &mut details)?;
    }
    annotation.values = pair(set, values, "a question has no value")?;
    annotation.details = pair(set, details, "a question has no detailed answer")?;
    Ok(annotation)
}

fn take_answers(
    set: &QuestionSet,
    chunk: &ChunkAnswer,
    values: &mut [Option<AnnotatedValue>],
    details: &mut [Option<AnnotatedEntry>],
) -> Result<usize, Error> {
    if chunk.places.len() != chunk.reply.outcomes().len() {
        return Err(Error::Defect(
            "the adapter answered the wrong number of questions",
        ));
    }
    let mut failed = 0;
    for (place, outcome) in chunk.places.iter().zip(chunk.reply.outcomes()) {
        let question = set
            .questions()
            .get(*place)
            .ok_or(Error::Defect("a group points outside its set"))?;
        let (Some(value_slot), Some(detail_slot)) =
            (values.get_mut(*place), details.get_mut(*place))
        else {
            return Err(Error::Defect("an answer points outside its set"));
        };
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
