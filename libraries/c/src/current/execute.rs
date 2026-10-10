//! Run existing native doors with owned inputs; no parser, reader or scheduler here.
use super::private::CurrentSummaryV1;
use super::{
    Input, QuestionHandle, ResultHandle, SourceHandle, Storage,
    question::Native,
    views::{self, Row},
};
use crate::failures::Failure;
use crate::ffi::values::{
    THINKTHEN_FUNCTION_RANK_V1 as RANK, THINKTHEN_FUNCTION_RECOGNIZE_V1 as RECOGNIZE,
};
use std::sync::{
    Mutex, PoisonError,
    atomic::{AtomicUsize, Ordering},
};
use thinkthen::{CallOptions, Engine, Evidence, Facts, LoadedQuestion, RecordObservation, Tally};

pub(crate) fn ask(
    engine: &Engine,
    kind: u32,
    question: &QuestionHandle,
    source: &SourceHandle,
    options: CallOptions<'_>,
    attempts: bool,
) -> Result<ResultHandle, Failure> {
    if !question.fits(kind) {
        return Err(Failure::usage(
            "the question kind does not match the named call",
        ));
    }
    let rank_set = if kind == RANK && matches!(question.native, Native::Set(_)) {
        Some(super::parse(RANK, question.json.clone())?)
    } else {
        None
    };
    let question = rank_set.as_ref().unwrap_or(question);
    let input_index = AtomicUsize::new(0);
    let collected = Mutex::new(Vec::new());
    let observer = |event: RecordObservation<'_>| {
        if let RecordObservation::Question {
            index,
            member,
            stage,
            position,
            detail,
        } = event
        {
            let mut held = collected.lock().unwrap_or_else(PoisonError::into_inner);
            let q = views::SavedQuestion::new(
                if kind == RECOGNIZE {
                    input_index.load(Ordering::Relaxed)
                } else {
                    index
                },
                member,
                stage,
                position,
                detail,
            );
            held.push(q);
        }
    };
    let attempted = Mutex::new(Vec::new());
    let collect_attempt = |event| {
        attempted
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(event);
    };
    let options = options.observe(&observer);
    let options = if attempts {
        options.observe_attempt(&collect_attempt)
    } else {
        options
    };
    let mut storage = Storage::default();
    let (rows, facts) = execute(
        engine,
        kind,
        question,
        source,
        options,
        (&mut storage, &input_index),
    )?;
    let questions = collected
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner)
        .iter()
        .map(|q| q.view(&mut storage))
        .collect::<Vec<_>>();
    let mut attempts_list = attempted
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner);
    attempts_list.sort_by_key(thinkthen::AttemptObservation::ordinal);
    let attempt_views = attempts_list
        .iter()
        .map(|a| views::attempt(&mut storage, a))
        .collect::<Vec<_>>();
    let summary = CurrentSummaryV1 {
        function: kind,
        count: rows.len(),
        question_count: questions.len(),
        facts: views::facts(&mut storage, &facts),
        attempts_present: i32::from(attempts),
        attempt_count: attempt_views.len(),
    };
    Ok(ResultHandle {
        _storage: storage,
        summary,
        rows,
        questions,
        attempts: attempt_views,
    })
}
fn execute(
    engine: &Engine,
    kind: u32,
    question: &QuestionHandle,
    source: &SourceHandle,
    options: CallOptions<'_>,
    output: (&mut Storage, &AtomicUsize),
) -> Result<(Vec<Row>, Facts), Failure> {
    let (storage, input_index) = output;
    match &source.0 {
        super::Source::Files(_, options) if options.media == thinkthen::ReaderMedia::Image => {
            return Err(Failure::usage(
                "private result/1 projection accepts text only",
            ));
        }
        super::Source::Records(records)
            if records.iter().any(|record| {
                !record.images.is_empty() || record.context.is_some() || !record.options.is_empty()
            }) =>
        {
            return Err(Failure::usage(
                "private result/1 projection cannot discard context, options or images",
            ));
        }
        _ => {}
    }
    let records = source.read()?;
    match &question.native {
        Native::Atomic(loaded) => match loaded {
            LoadedQuestion::Question(q) => {
                atomic(engine, q, records, options, storage, (kind, question))
            }
            LoadedQuestion::Banded(q) => {
                atomic(engine, q, records, options, storage, (kind, question))
            }
        },
        Native::DynamicChoose(_) => Err(Failure::usage(
            "private legacy projections take fixed choose questions",
        )),
        Native::Set(set) => {
            let mut batch = engine.try_annotate_with(set, records, options);
            let rows = batch
                .by_ref()
                .map(|r| {
                    let r = r?;
                    Ok(Row::Annotation(views::annotation(
                        storage,
                        r.input(),
                        r.values(),
                        &question.json,
                    )?))
                })
                .collect::<Result<Vec<_>, Failure>>()?;
            let facts = batch
                .facts()
                .cloned()
                .ok_or_else(|| Failure::defect("completed annotation has no facts"))?;
            Ok((rows, facts))
        }
        Native::Rank(q) => {
            let call = engine.try_rank_with(q, records, options)?;
            let rows = call
                .value()
                .iter()
                .map(|r| {
                    let mut v = views::row(storage, Some(r.input()));
                    v.probability = views::double(Some(r.probability()));
                    Row::Original(v)
                })
                .collect();
            Ok((rows, call.facts().clone()))
        }
        Native::RankSet(set) => {
            let records = records.collect::<Result<Vec<_>, _>>()?;
            let call = engine.rank_set_with(set, records, options)?;
            let rows = call
                .value()
                .iter()
                .map(|r| {
                    let mut v = views::row(storage, Some(r.input()));
                    v.probability = views::double(Some(r.probability()));
                    v.question_name = storage.optional_string(Some(r.question_name()));
                    Row::Original(v)
                })
                .collect();
            Ok((rows, call.facts().clone()))
        }
        Native::Find(q) => find(engine, q, records, options, storage),
        Native::Recognize(_) | Native::Relate(_) => {
            entities(engine, question, records, options, storage, input_index)
        }
    }
}
fn atomic<'a, Q: thinkthen::DetailQuestion + ?Sized>(
    engine: &'a Engine,
    question: &'a Q,
    records: Box<dyn Iterator<Item = Result<Input, thinkthen::Error>> + 'a>,
    options: CallOptions<'a>,
    storage: &mut Storage,
    selection: (u32, &QuestionHandle),
) -> Result<(Vec<Row>, Facts), Failure> {
    if selection.0 == 5 {
        // Native filter admits only a single cut; use its actual admission.
        let Native::Atomic(LoadedQuestion::Question(q)) = &selection.1.native else {
            return Err(Failure::usage("filter keeps a record at a cut, not a band"));
        };
        let mut batch = engine.try_filter_with(q, records, options);
        let rows = batch
            .by_ref()
            .map(|r| {
                let r = r?;
                Ok(Row::Original(views::row(storage, Some(&r))))
            })
            .collect::<Result<Vec<_>, thinkthen::Error>>()?;
        let facts = batch
            .facts()
            .cloned()
            .ok_or_else(|| Failure::defect("completed filter has no facts"))?;
        return Ok((rows, facts));
    }
    let mut batch = engine.try_details_many_with(question, records, options);
    let rows = batch
        .by_ref()
        .map(|r| {
            let r = r?;
            let mut view = views::atomic(storage, r.input(), r.value());
            let meaning = meaning(&selection.1.json, r.value().value())?;
            views::authored(storage, &mut view.value, meaning.as_ref());
            Ok(Row::Atomic(Box::new(view)))
        })
        .collect::<Result<Vec<_>, Failure>>()?;
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| Failure::defect("completed typed details have no facts"))?;
    Ok((rows, facts))
}
pub(super) fn meaning(
    json: &str,
    value: &thinkthen::Judgment,
) -> Result<Option<super::Content>, Failure> {
    let key = match value {
        thinkthen::Judgment::Decision(thinkthen::Answer::Yes) => "true",
        thinkthen::Judgment::Decision(thinkthen::Answer::No) => "false",
        _ => return Ok(None),
    };
    let fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(json)
            .map_err(|_| Failure::defect("saved input grammar cannot be read"))?;
    let Some(raw) = fields.get(key) else {
        return Ok(None);
    };
    if let Ok(text) = serde_json::from_str::<String>(raw.get()) {
        Ok(Some(super::Content::Text(text)))
    } else {
        Ok(Some(super::Content::Json(raw.clone())))
    }
}

fn entities(
    engine: &Engine,
    question: &QuestionHandle,
    records: super::Inputs<'_>,
    options: CallOptions<'_>,
    storage: &mut Storage,
    input_index: &AtomicUsize,
) -> Result<(Vec<Row>, Facts), Failure> {
    match &question.native {
        Native::Recognize(q) => {
            let options = options.started()?;
            let tally = Tally::new();
            let mut rows = Vec::new();
            let result = (|| {
                for (at, input) in records.enumerate() {
                    let input = input?;
                    engine.check_record_limit(at)?;
                    input_index.store(input.index, Ordering::Relaxed);
                    let call = tally.run(|| engine.recognize_with(q, input.evidence(), options))?;
                    rows.push(Row::Recognition(views::recognition(
                        storage,
                        &input,
                        call.value(),
                    )));
                }
                Ok::<_, thinkthen::Error>(())
            })();
            result.map_err(|e| e.with_facts(tally.facts()))?;
            Ok((rows, tally.facts()))
        }
        Native::Relate(q) => {
            // Native relation entities require caller-authored JSON records;
            // text file records use the existing wildcard entity spelling.
            let mut entities = Vec::new();
            let mut inputs = Vec::new();
            for input in records {
                let input = input?;
                engine.check_record_limit(inputs.len())?;
                thinkthen::Relate::admit_record_count(inputs.len() + 1)
                    .map_err(|_| Failure::usage("relate takes at most 255 records"))?;
                let entity = match input.original {
                    Some(super::Content::Json(ref raw)) => {
                        thinkthen::Entity::from_record(raw.get())?
                    }
                    Some(super::Content::Text(ref text)) => thinkthen::Entity::new(text, "*")?,
                    None => return Err(Failure::usage("relate accepts text only")),
                };
                if input.position.is_none() || !entities.contains(&entity) {
                    entities.push(entity);
                }
                inputs.push(views::row(storage, Some(&input)));
            }
            let call = engine.relate_with(q, entities, options)?;
            let mut result = views::relations(storage, call.value());
            let (data, len) = storage.array(inputs);
            result.inputs = super::private::CurrentRowsV1 { data, len };
            Ok((vec![Row::Relations(result)], call.facts().clone()))
        }
        _ => Err(Failure::defect("entity dispatch received another kind")),
    }
}

fn find(
    engine: &Engine,
    q: &thinkthen::Question,
    records: super::Inputs<'_>,
    options: CallOptions<'_>,
    storage: &mut Storage,
) -> Result<(Vec<Row>, Facts), Failure> {
    let call = engine.try_find_with(q, records, options)?;
    let mut row = views::row(storage, call.value().selected());
    if let Some(selected) = call.value().selected() {
        row.probability = views::double(
            call.value()
                .candidates()
                .iter()
                .find(|c| c.input().is_some_and(|i| i.index == selected.index))
                .map(thinkthen::Candidate::probability),
        );
    }
    let candidates = call
        .value()
        .candidates()
        .iter()
        .map(|c| super::private::CurrentCandidateV1 {
            common: views::row(storage, c.input()),
            probability: c.probability(),
        })
        .collect();
    let (data, len) = storage.array(candidates);
    let find = super::private::CurrentFindV1 {
        selected: row,
        candidates: super::private::CurrentCandidatesV1 { data, len },
    };
    Ok((vec![Row::Find(find)], call.facts().clone()))
}
