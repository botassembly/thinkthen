//! One driver maps admitted questions onto the existing bounded worker engine.
use crate::core::{
    self,
    records::{Record, RecordId},
    results::{self, AnyRow, Outcome, Row, Source},
};
use crate::engine::RecordEvent;
use crate::{Engine, Error, OffsetUnit, Question, questions};
use std::collections::{BTreeMap, VecDeque};
use std::num::NonZeroU64;
use std::sync::{Mutex, PoisonError};

pub(super) fn drive(
    engine: &Engine,
    question: &Question,
    name: Option<&str>,
    records: impl Iterator<Item = RecordEvent<Record>> + Send,
    offsets: OffsetUnit,
    mut sink: impl FnMut(Result<AnyRow, Error>),
) -> Result<(), Error> {
    let source = Source {
        name: name.map(str::to_owned),
        question_identity: identity(question, name)?,
        backend: engine.backend().name().parse()?,
        model: engine.backend().model().to_owned(),
    };
    let context = Context { question, offsets };
    macro_rules! emit {
        ($variant:ident,$q:ident) => {
            ordinary(
                engine,
                &$q.inner,
                records,
                &context,
                |r| {
                    Row::<questions::$variant>::from_outcome(
                        r.position,
                        r.id,
                        source.clone(),
                        r.outcome,
                    )
                    .map(AnyRow::$variant)
                    .map(Some)
                },
                &mut sink,
            )
        };
    }
    match question {
        Question::Decide(q) => emit!(Decide, q),
        Question::Filter(q) => emit!(Filter, q),
        Question::Choose(q) => ordinary(
            engine,
            q,
            records,
            &context,
            |r| {
                Row::<questions::Choose>::from_outcome(r.position, r.id, source.clone(), r.outcome)
                    .map(AnyRow::Choose)
                    .map(Some)
            },
            &mut sink,
        ),
        Question::Tag(q) => emit!(Tag, q),
        Question::Score(q) => emit!(Score, q),
        Question::Find(q) => emit!(Find, q),
        Question::Recognize(q) => emit!(Recognize, q),
        Question::Relate(q) => emit!(Relate, q),
        Question::Annotate(q) => {
            let members = q
                .questions()
                .iter()
                .map(|(name, entry)| {
                    Ok((
                        name.clone(),
                        core::ToolboxQuestion::admitted(
                            entry.execution(),
                            entry
                                .evidence_override()
                                .unwrap_or(q.evidence_pointer())
                                .clone(),
                            stub_identity(&entry.as_question())?,
                        ),
                    ))
                })
                .collect::<Result<_, Error>>()?;
            let execution =
                core::Annotate::new(&core::Toolbox::admitted(members)).complete_choices();
            ordinary(
                engine,
                &execution,
                records,
                &context,
                |r| annotation(q, r, &source).map(Some),
                &mut sink,
            )
        }
        Question::Rank(q) => rank(engine, q, question, records, offsets, &source, &mut sink),
    }
}
// A stub answer depends on question content and already selected evidence.
// A selector or execution name cannot change the same question's answer.
fn stub_identity(question: &Question) -> Result<String, Error> {
    identity(&question.without_evidence(), None)
}
fn identity(question: &Question, name: Option<&str>) -> Result<String, Error> {
    use sha2::{Digest, Sha256};
    serde_json::to_vec(&(name, question))
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| results::mismatch())
}
struct Evaluated {
    position: NonZeroU64,
    id: Option<RecordId>,
    outcome: core::Outcome,
}
struct Context<'a> {
    question: &'a Question,
    offsets: OffsetUnit,
}
fn ordinary<F: core::Function>(
    engine: &Engine,
    execution: &F,
    records: impl Iterator<Item = RecordEvent<Record>> + Send,
    context: &Context<'_>,
    mut convert: impl FnMut(Evaluated) -> Result<Option<AnyRow>, Error>,
    sink: &mut impl FnMut(Result<AnyRow, Error>),
) -> Result<(), Error> {
    let Context { question, offsets } = *context;
    let metadata = Mutex::new(VecDeque::new());
    let mut ordinal = 0_u64;
    let mut failure = None;
    let records = records.map(|event| {
        event.map(|record| {
            ordinal = ordinal.saturating_add(1);
            let pointer = if matches!(question, Question::Annotate(_)) {
                &core::record::ROOT_POINTER
            } else {
                question.evidence_pointer()
            };
            let prepared = record.prepare(pointer);
            metadata
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .push_back((
                    NonZeroU64::new(ordinal).unwrap_or(NonZeroU64::MIN),
                    record.id().cloned(),
                    prepared.clone(),
                ));
            prepared
        })
    });
    let result = engine.call_contract(
        execution,
        records,
        |row| {
            let row = match row {
                Ok(row) => row,
                Err(error) => {
                    failure.get_or_insert_with(|| error.clone());
                    sink(Err(error));
                    return;
                }
            };
            let Some((position, id, record)) = metadata
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .pop_front()
            else {
                failure = Some(results::mismatch());
                return;
            };
            let mut outcome = row.outcome().clone();
            if let Question::Annotate(q) = question {
                if let core::Outcome::Answered {
                    answer: core::Answer::Annotations(entries),
                    ..
                } = &mut outcome
                {
                    for (name, outcome) in entries {
                        if let Some(entry) = q.questions().get(name)
                            && let Ok(text) = record.evidence_at(
                                entry.evidence_override().unwrap_or(q.evidence_pointer()),
                            )
                        {
                            core::spans::outcome(outcome, &text, offsets);
                        }
                    }
                }
            } else if let Ok(text) = &record.evidence {
                core::spans::outcome(&mut outcome, text, offsets);
            }
            match convert(Evaluated {
                position,
                id,
                outcome,
            }) {
                Ok(Some(row)) => sink(Ok(row)),
                Ok(None) => {}
                Err(error) => {
                    failure = Some(error.clone());
                    sink(Err(error));
                    engine.cancel.cancel();
                }
            }
        },
        stub_identity(question)?,
    );
    failure.map_or(result, Err)
}
fn annotation(
    question: &questions::Annotate,
    row: Evaluated,
    source: &Source,
) -> Result<AnyRow, Error> {
    if let core::Outcome::Answered {
        answer: core::Answer::Annotations(entries),
        ..
    } = row.outcome
    {
        let answers = entries
            .into_iter()
            .map(|(name, outcome)| {
                let member = question
                    .questions()
                    .get(&name)
                    .ok_or_else(results::mismatch)?;
                Ok((name, member.entry(outcome)?))
            })
            .collect::<Result<BTreeMap<_, _>, Error>>()?;
        Ok(AnyRow::Annotate(Row::new(
            row.position,
            row.id,
            source.clone(),
            Outcome::Answered {
                answer: answers,
                probability: None,
                detail: None,
            },
        )))
    } else {
        Row::<questions::Annotate>::from_outcome(row.position, row.id, source.clone(), row.outcome)
            .map(AnyRow::Annotate)
    }
}
fn rank(
    engine: &Engine,
    q: &questions::Rank,
    question: &Question,
    records: impl Iterator<Item = RecordEvent<Record>> + Send,
    offsets: OffsetUnit,
    source: &Source,
    sink: &mut impl FnMut(Result<AnyRow, Error>),
) -> Result<(), Error> {
    let mut successful = BTreeMap::new();
    let mut failed = Vec::new();
    ordinary(
        engine,
        &q.inner,
        records,
        &Context { question, offsets },
        |row| {
            if let core::Outcome::Answered {
                answer: core::Answer::Number(metric),
                ..
            } = &row.outcome
            {
                successful.insert(
                    (
                        std::cmp::Reverse(metric.to_bits() & !(1_u64 << 63)),
                        row.position,
                    ),
                    row,
                );
                if q.top.is_some_and(|top| successful.len() > top.get()) {
                    successful.pop_last();
                }
            } else {
                failed.push(row);
            }
            Ok(None)
        },
        &mut |_| {},
    )?;
    for (index, mut row) in successful.into_values().enumerate() {
        if let core::Outcome::Answered { answer, .. } = &mut row.outcome {
            *answer = core::Answer::Position(index + 1);
        }
        sink(
            Row::<questions::Rank>::from_outcome(row.position, row.id, source.clone(), row.outcome)
                .map(AnyRow::Rank),
        );
    }
    for row in failed {
        sink(
            Row::<questions::Rank>::from_outcome(row.position, row.id, source.clone(), row.outcome)
                .map(AnyRow::Rank),
        );
    }
    Ok(())
}
