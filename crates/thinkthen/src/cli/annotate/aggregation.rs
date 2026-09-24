//! Ordered aggregation for grouped and split annotation requests.

use crate::core::adapters::built_in;
use crate::core::{
    AnnotateMeta, AnnotateResult, AnnotatedAnswer, AnnotatedEntry, AnnotatedFailure,
    AnnotatedValue, AnswerOutcome, FailedValue, ModelName, NamedValues, Outcome, QuestionSet,
    Record, RecordValue, Reply, RequestMeta, Usage, json_line,
};
use crate::failure::Failure;
use crate::schedule::Judged;

use super::Judging;

pub(crate) struct ChunkAnswer {
    pub(crate) places: Vec<usize>,
    pub(crate) reply: Reply,
    pub(crate) digest: String,
    pub(crate) requests_sent: u64,
    pub(crate) replayed: bool,
}

pub(crate) struct GroupAnswer {
    pub(crate) answered: Vec<ChunkAnswer>,
    pub(crate) model: Option<ModelName>,
}

pub(super) fn finish(
    judging: &Judging<'_>,
    record: Record,
    answered: Vec<GroupAnswer>,
) -> Result<Judged, Failure> {
    let mut values: Vec<Option<AnnotatedValue>> = vec![None; judging.set.questions().len()];
    let mut details: Vec<Option<AnnotatedEntry>> = vec![None; judging.set.questions().len()];
    let mut model = None;
    let mut requests = Vec::new();
    let mut usage = Some(Usage::new(0, 0));
    let mut requests_sent = 0_u64;
    let mut replayed = true;
    let mut failed_questions = 0;
    for group in answered {
        for chunk in group.answered {
            check_model(
                &mut model,
                chunk.reply.model(),
                judging.engine.backend().model(),
            )?;
            requests.push(chunk.digest.clone());
            usage = add_usage(usage, chunk.reply.usage())?;
            replayed &= chunk.replayed;
            requests_sent = requests_sent
                .checked_add(chunk.requests_sent)
                .ok_or(Failure::Defect("a request count overflowed"))?;
            failed_questions += take_answers(
                &judging.set,
                &chunk.places,
                &chunk.reply,
                &chunk.digest,
                &mut values,
                &mut details,
            )?;
        }
    }
    let named_values = pair(&judging.set, values, "a question has no value")?;
    let printed = if judging.common.details {
        let named_details = pair(&judging.set, details, "a question has no detailed answer")?;
        let meta = AnnotateMeta::new(
            env!("CARGO_PKG_VERSION"),
            judging.set.sha256()?,
            judging.engine.backend().url().clone(),
            model.ok_or(Failure::Defect("no group reported a model"))?,
            usage,
            RequestMeta::new(replayed, requests_sent, requests)
                .with_failed_questions(failed_questions)
                .with_profile_warning(judging.mismatch.warning()),
        );
        json_line(&AnnotateResult::new(
            record,
            named_values,
            named_details,
            meta,
        ))?
    } else if judging.streams && !record.is_object() {
        json_line(&RecordValue::new(record, NamedValues::new(named_values)))?
    } else {
        json_line(&record.annotated(named_values))?
    };
    Ok(Judged {
        printed: Some(printed),
        outcome: Outcome::Yes,
        replayed,
        probability: None,
        partial_failure: failed_questions > 0,
        profile_mismatch: judging.mismatch.notice(),
    })
}

fn add_usage(total: Option<Usage>, next: Option<Usage>) -> Result<Option<Usage>, Failure> {
    match (total, next) {
        (Some(total), Some(next)) => Ok(Some(
            total.checked_plus(next).ok_or(Failure::UsageOverflow)?,
        )),
        _ => Ok(None),
    }
}

fn take_answers(
    set: &QuestionSet,
    group: &[usize],
    reply: &Reply,
    digest: &str,
    values: &mut [Option<AnnotatedValue>],
    details: &mut [Option<AnnotatedEntry>],
) -> Result<usize, Failure> {
    if group.len() != reply.outcomes().len() {
        return Err(Failure::Defect(
            "the adapter answered the wrong number of questions",
        ));
    }
    let mut failed = 0;
    for (place, outcome) in group.iter().zip(reply.outcomes()) {
        let question = set
            .questions()
            .get(*place)
            .ok_or(Failure::Defect("a group points outside its set"))?;
        let value_slot = values
            .get_mut(*place)
            .ok_or(Failure::Defect("an answer points outside its set"))?;
        let detail_slot = details
            .get_mut(*place)
            .ok_or(Failure::Defect("an answer points outside its set"))?;
        match outcome {
            AnswerOutcome::Answered(answer) => {
                let (value, _) = answer.read(question.threshold());
                *value_slot = Some(AnnotatedValue::Answered(value.clone()));
                *detail_slot = Some(AnnotatedEntry::Answered(AnnotatedAnswer::new(
                    value,
                    question.question().clone(),
                    answer.clone(),
                    question.threshold(),
                    digest.to_owned(),
                )));
            }
            AnswerOutcome::Failed(failure) => {
                failed += 1;
                *value_slot = Some(AnnotatedValue::Failed(FailedValue::new(*failure)));
                *detail_slot = Some(AnnotatedEntry::Failed(AnnotatedFailure::new(
                    question.question().clone(),
                    *failure,
                    digest.to_owned(),
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
) -> Result<Vec<(String, T)>, Failure> {
    set.questions()
        .iter()
        .zip(values)
        .map(|(question, value)| {
            Ok((
                question.name().to_owned(),
                value.ok_or(Failure::Defect(absent))?,
            ))
        })
        .collect()
}

pub(crate) fn check_model(
    first: &mut Option<ModelName>,
    next: &ModelName,
    requested: &ModelName,
) -> Result<(), Failure> {
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
    Err(Failure::ModelsDiffer(safe))
}
