//! Located calls shared by the JSON door and Python, using the native reader.

mod relate_output;

use serde_json::{Value, json};
use thinkthen::{
    CallOptions, DetailQuestion, Engine, Entity, Error, Evidence, Facts, LoadedQuestion, Question,
    QuestionSet, ReaderOptions, Recognize, Relate, SourceRecord, Tally,
};

/// Keep provenance with the original input while details serializes only evidence.
struct DetailInput(SourceRecord<String>);

impl Evidence for DetailInput {
    fn evidence(&self) -> &str {
        &self.0.record
    }
}

impl serde::Serialize for DetailInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(&self.0.record, serializer)
    }
}

pub(crate) struct Selection {
    paths: Vec<String>,
    options: ReaderOptions,
}

impl Selection {
    pub(crate) fn read(&self) -> Result<thinkthen::SourceRecords, Error> {
        thinkthen::read_files(&self.paths, self.options)
    }
}

pub(crate) fn parse(text: &str) -> Result<Selection, Error> {
    let mut body: serde_json::Map<String, Value> = serde_json::from_str(text)
        .map_err(|_| usage("source takes paths, unit, and optional window"))?;
    let paths = serde_json::from_value(
        body.remove("paths")
            .ok_or_else(|| usage("source requires paths"))?,
    )
    .map_err(|_| usage("source paths are an array of strings"))?;
    let options: ReaderOptions = serde_json::from_value(Value::Object(body))
        .map_err(|_| usage("source takes paths, unit, and optional window"))?;
    Ok(Selection {
        paths,
        options: options.validate()?,
    })
}

fn decoded(text: &str) -> Result<Value, Error> {
    serde_json::from_str(text).map_err(|_| defect("a native result could not be read"))
}

fn located(source: &SourceRecord<String>, answer: Value) -> Result<Value, Error> {
    let mut row = json!({"record":source.record,"file":source.file,"first_line":source.first_line,"last_line":source.last_line});
    put(&mut row, "value", answer)?;
    Ok(row)
}

pub(crate) fn execute(
    engine: &Engine,
    verb: &str,
    question: &str,
    selection: &Selection,
    options: CallOptions<'_>,
    detailed: bool,
) -> Result<(String, Facts), Error> {
    // Validate the question before admitting any file content.
    let result = match verb {
        "decide" | "choose" | "score" | "tag" => match Question::from_json(question)? {
            LoadedQuestion::Question(asked) => {
                judgments(engine, &asked, selection, options, detailed)?
            }
            LoadedQuestion::Banded(asked) => {
                judgments(engine, &asked, selection, options, detailed)?
            }
        },
        "filter" => {
            let LoadedQuestion::Question(asked) = Question::from_json(question)? else {
                return Err(usage("filter keeps a record at a cut, not a band"));
            };
            let mut batch = engine.try_filter_with(&asked, selection.read()?, options);
            let rows = batch
                .by_ref()
                .map(|row| located(&row?, json!(true)))
                .collect::<Result<Vec<_>, _>>()?;
            let facts = batch
                .facts()
                .cloned()
                .ok_or_else(|| defect("completed filter has no facts"))?;
            (Value::Array(rows), facts)
        }
        "rank" => {
            let body: std::collections::BTreeMap<String, String> =
                serde_json::from_str(question)
                    .map_err(|_| usage("rank takes its question text alone"))?;
            let text = body
                .get("rank")
                .filter(|_| body.len() == 1)
                .ok_or_else(|| usage("rank takes its question text alone"))?;
            let asked = Question::rank(text)?;
            let mut bytes = 0usize;
            let records = selection.read()?.map(|record| {
                let record = record?;
                bytes = bytes
                    .checked_add(record.record.len())
                    .filter(|&bytes| bytes <= 16 * 1024 * 1024)
                    .ok_or_else(|| usage("source rank input exceeds 16 MiB"))?;
                Ok(record)
            });
            let call = engine.try_rank_with(&asked, records, options)?;
            let rows = call
                .value()
                .iter()
                .map(|row| {
                    let mut held = located(row.input(), json!(row.probability()))?;
                    put(&mut held, "index", json!(row.index()))?;
                    put(&mut held, "probability", json!(row.probability()))?;
                    Ok(held)
                })
                .collect::<Result<Vec<_>, Error>>()?;
            (Value::Array(rows), call.facts().clone())
        }
        "find" => find(engine, question, selection, options)?,
        "annotate" => {
            let set = QuestionSet::from_json(question)?;
            let mut batch = engine.try_annotate_with(&set, selection.read()?, options);
            let rows = batch
                .by_ref()
                .map(|row| {
                    let row = row?;
                    located(row.input(), decoded(&row.value_json())?)
                })
                .collect::<Result<Vec<_>, Error>>()?;
            let facts = batch
                .facts()
                .cloned()
                .ok_or_else(|| defect("completed annotate has no facts"))?;
            (Value::Array(rows), facts)
        }
        "recognize" => recognize(engine, question, selection, options)?,
        "relate" => relate(engine, question, selection, options)?,
        _ => return Err(usage("source requires a judging verb")),
    };
    let text = serde_json::to_string(&result.0)
        .map_err(|_| defect("a located result could not be written"))?;
    Ok((text, result.1))
}

fn judgments<Q: DetailQuestion + ?Sized>(
    engine: &Engine,
    asked: &Q,
    selection: &Selection,
    options: CallOptions<'_>,
    detailed: bool,
) -> Result<(Value, Facts), Error> {
    let records = selection.read()?.map(|record| record.map(DetailInput));
    let mut batch = engine.try_details_many_with(asked, records, options);
    let rows = batch
        .by_ref()
        .map(|row| {
            let row = row?;
            let answer = if detailed {
                decoded(&row.value().to_json())?
            } else {
                match row.value().value() {
                    thinkthen::Judgment::Decision(answer) => match answer {
                        thinkthen::Answer::Yes => json!(true),
                        thinkthen::Answer::No => json!(false),
                        thinkthen::Answer::Unsure => Value::Null,
                    },
                    thinkthen::Judgment::Choice(pick) => json!(pick),
                    thinkthen::Judgment::Score(score) => json!(score),
                    thinkthen::Judgment::Tags(tags) => json!(tags),
                }
            };
            located(&row.input().0, answer)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| defect("completed judgments have no facts"))?;
    Ok((Value::Array(rows), facts))
}

fn find(
    engine: &Engine,
    question: &str,
    selection: &Selection,
    options: CallOptions<'_>,
) -> Result<(Value, Facts), Error> {
    let body: Value =
        serde_json::from_str(question).map_err(|_| usage("find requires a question"))?;
    let text = body
        .get("find")
        .and_then(Value::as_str)
        .ok_or_else(|| usage("find requires question text"))?;
    if body
        .as_object()
        .is_none_or(|o| o.keys().any(|k| k != "find" && k != "none"))
    {
        return Err(usage("find takes its question text and none alone"));
    }
    let mut asked = Question::find(text)?;
    match body.get("none") {
        Some(Value::Bool(true)) => asked = asked.offering_none()?,
        Some(Value::Bool(false)) | None => (),
        _ => return Err(usage("find takes none as true or false")),
    }
    let call = engine.try_find_with(&asked, selection.read()?, options)?;
    let answer =
        if let (Some(source), Some(picked)) = (call.value().selected(), call.value().picked()) {
            located(
                source,
                serde_json::to_value(picked)
                    .map_err(|_| defect("a found result could not be written"))?,
            )?
        } else {
            Value::Null
        };
    Ok((answer, call.facts().clone()))
}

fn recognize(
    engine: &Engine,
    question: &str,
    selection: &Selection,
    options: CallOptions<'_>,
) -> Result<(Value, Facts), Error> {
    let options = options.started()?;
    let asked = Recognize::from_json(question)?;
    let tally = Tally::new();
    let mut rows = Vec::new();
    let records = selection.read()?;
    let result = (|| {
        for (at, source) in records.enumerate() {
            let source = source?;
            engine.check_record_limit(at)?;
            let call = tally.run(|| engine.recognize_with(&asked, &source.record, options))?;
            let mut answer = decoded(&call.value().to_json())?;
            locate_recognition(&source, &mut answer)?;
            rows.push(located(&source, answer)?);
        }
        Ok::<(), Error>(())
    })();
    result.map_err(|error| error.with_facts(tally.facts()))?;
    Ok((Value::Array(rows), tally.facts()))
}

fn relate(
    engine: &Engine,
    question: &str,
    selection: &Selection,
    options: CallOptions<'_>,
) -> Result<(Value, Facts), Error> {
    let asked = Relate::from_json(question)?;
    let mut sources = Vec::new();
    let mut entities = Vec::new();
    let mut occurrences = Vec::new();
    let mut bytes = 0usize;
    for source in selection.read()? {
        let source = source?;
        if sources.len() == 255 {
            return Err(usage("source relate takes at most 255 source records"));
        }
        bytes = bytes
            .checked_add(source.record.len())
            .filter(|&bytes| bytes <= 16 * 1024 * 1024)
            .ok_or_else(|| usage("source relate input exceeds 16 MiB"))?;
        let entity = Entity::new(&source.record, "*")?;
        let index = if let Some(at) = entities.iter().position(|e| e == &entity) {
            at
        } else {
            if entities.len() == 255 {
                return Err(usage("relate takes at most 255 distinct entities"));
            }
            entities.push(entity);
            entities.len() - 1
        };
        sources.push(source);
        occurrences.push(index);
    }
    let call = engine.relate_with(&asked, entities.clone(), options)?;
    let mut edges = Vec::new();
    let mut budget = relate_output::Budget::new();
    for edge in call.value() {
        for (s, _) in occurrences
            .iter()
            .enumerate()
            .filter(|(_, i)| entities.get(**i) == Some(edge.source()))
        {
            for (t, _) in occurrences
                .iter()
                .enumerate()
                .filter(|(_, i)| entities.get(**i) == Some(edge.target()))
            {
                let source = sources
                    .get(s)
                    .ok_or_else(|| defect("missing source occurrence"))?;
                let target = sources
                    .get(t)
                    .ok_or_else(|| defect("missing target occurrence"))?;
                let held =
                    relate_output::edge(edge, source, target, &mut budget, !edges.is_empty())
                        .map_err(|error| error.with_facts(call.facts().clone()))?;
                edges.push(held);
            }
        }
    }
    Ok((json!({"edges": edges}), call.facts().clone()))
}

fn usage(message: &str) -> Error {
    Error::new(thinkthen::ErrorKind::Usage, message)
}
fn defect(message: &str) -> Error {
    Error::new(thinkthen::ErrorKind::Defect, message)
}

fn put(row: &mut Value, key: &str, value: Value) -> Result<(), Error> {
    row.as_object_mut()
        .ok_or_else(|| defect("located result is not an object"))?
        .insert(key.to_owned(), value);
    Ok(())
}

/// Select one of the ten existing question grammars, with no evidence members.
pub(crate) fn dispatch(
    engine: &Engine,
    question: &str,
    selection: &str,
    options: CallOptions<'_>,
) -> Result<(String, Facts), Error> {
    let mut body: serde_json::Map<String, Value> =
        serde_json::from_str(question).map_err(|_| usage("files takes one question object"))?;
    if [
        "source", "evidence", "records", "units", "call", "attempts", "usage",
    ]
    .iter()
    .any(|key| body.contains_key(*key))
    {
        return Err(usage(
            "files takes question members alone; source replaces evidence, records, and units",
        ));
    }
    let named: Vec<_> = [
        "decide",
        "choose",
        "tag",
        "score",
        "filter",
        "rank",
        "find",
        "annotate",
        "recognize",
        "relate",
    ]
    .into_iter()
    .filter(|verb| body.contains_key(*verb))
    .collect();
    let [verb] = named.as_slice() else {
        return Err(usage("files takes exactly one judging verb"));
    };
    let detailed = match body.remove("details") {
        None | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) if matches!(*verb, "decide" | "choose" | "tag" | "score") => true,
        _ => {
            return Err(usage(
                "details requires a scalar judging verb and a boolean",
            ));
        }
    };
    if *verb == "filter"
        && let Some(text) = body.remove("filter")
    {
        body.insert("decide".into(), text);
    }
    let question = if *verb == "annotate" {
        if body.len() != 1 {
            return Err(usage("annotate takes its question set alone"));
        }
        body.remove("annotate")
            .ok_or_else(|| usage("annotate requires a question set"))?
    } else {
        Value::Object(body)
    };
    execute(
        engine,
        verb,
        &question.to_string(),
        &parse(selection)?,
        options,
        detailed,
    )
}

fn locate_recognition(source: &SourceRecord<String>, answer: &mut Value) -> Result<(), Error> {
    if let Some(entities) = answer.get_mut("entities").and_then(Value::as_array_mut) {
        for entity in entities {
            locate_span(source, entity)?;
        }
    }
    if let Some(relations) = answer.get_mut("relations").and_then(Value::as_array_mut) {
        for relation in relations {
            locate_relation(source, relation)?;
        }
    }
    Ok(())
}

fn locate_span(source: &SourceRecord<String>, entity: &mut Value) -> Result<(), Error> {
    let offset = |key| {
        entity
            .get(key)
            .and_then(Value::as_u64)
            .and_then(|n| usize::try_from(n).ok())
            .ok_or_else(|| defect("a native recognition span held no offset"))
    };
    let (first, last) = source.span_lines(offset("start")?, offset("end")?)?;
    put(entity, "file", json!(source.file))?;
    put(entity, "first_line", json!(first))?;
    put(entity, "last_line", json!(last))?;
    Ok(())
}

fn locate_relation(source: &SourceRecord<String>, relation: &mut Value) -> Result<(), Error> {
    for endpoint in ["source", "target"] {
        if let Some(entity) = relation.get_mut(endpoint) {
            locate_span(source, entity)?;
        }
    }
    Ok(())
}
