//! Located calls shared by the JSON door and Python, using the native reader.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use thinkthen::{
    CallOptions, DetailQuestion, Engine, Entity, Error, Facts, LoadedQuestion, Question,
    QuestionSet, ReaderOptions, Recognize, Relate, SourceRecord, SourceUnit, Tally,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Selection {
    pub(crate) paths: Vec<String>,
    #[serde(default)]
    pub(crate) unit: SourceUnit,
    #[serde(default)]
    pub(crate) window: Option<usize>,
}

impl Selection {
    pub(crate) fn read(&self) -> Result<thinkthen::SourceRecords, Error> {
        thinkthen::read_files(
            &self.paths,
            ReaderOptions {
                unit: self.unit,
                window: self.window,
            },
        )
    }
}

pub(crate) fn parse(text: &str) -> Result<Selection, Error> {
    serde_json::from_str(text).map_err(|_| usage("source takes paths, unit, and optional window"))
}

fn value<T: Serialize>(held: T) -> Result<Value, Error> {
    serde_json::to_value(held).map_err(|_| defect("a located result could not be written"))
}

fn decoded(text: &str) -> Result<Value, Error> {
    serde_json::from_str(text).map_err(|_| defect("a native result could not be read"))
}

fn located(source: &SourceRecord<String>, answer: Value) -> Result<Value, Error> {
    let mut row = value(source)?;
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
            let records = selection.read()?.collect::<Result<Vec<_>, _>>()?;
            let mut batch = engine.filter_with(&asked, records, options);
            let rows = batch
                .by_ref()
                .map(|row| located(&row?, json!(true)))
                .collect::<Result<Vec<_>, _>>()?;
            let facts = batch
                .facts()
                .cloned()
                .ok_or_else(|| defect("completed filter has no facts"))?;
            (value(rows)?, facts)
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
            let records = selection.read()?.collect::<Result<Vec<_>, _>>()?;
            let call = engine.rank_with(&asked, records, options)?;
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
            (value(rows)?, call.facts().clone())
        }
        "find" => find(engine, question, selection, options)?,
        "annotate" => {
            let set = QuestionSet::from_json(question)?;
            let records = selection.read()?.collect::<Result<Vec<_>, _>>()?;
            let mut batch =
                engine.annotate_with(&set, records.iter().map(|r| r.record.as_str()), options);
            let answers = batch
                .by_ref()
                .map(|row| decoded(&row?.value_json()))
                .collect::<Result<Vec<_>, _>>()?;
            let facts = batch
                .facts()
                .cloned()
                .ok_or_else(|| defect("completed annotate has no facts"))?;
            let rows = records
                .iter()
                .zip(answers)
                .map(|(s, a)| located(s, a))
                .collect::<Result<Vec<_>, _>>()?;
            (value(rows)?, facts)
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
    let records = selection.read()?.collect::<Result<Vec<_>, _>>()?;
    let mut batch =
        engine.details_many_with(asked, records.iter().map(|r| r.record.as_str()), options);
    let answers = batch
        .by_ref()
        .map(|row| {
            let row = row?;
            if detailed {
                return decoded(&row.value().to_json());
            }
            match row.value().value() {
                thinkthen::Judgment::Decision(answer) => Ok(match answer {
                    thinkthen::Answer::Yes => json!(true),
                    thinkthen::Answer::No => json!(false),
                    thinkthen::Answer::Unsure => Value::Null,
                }),
                thinkthen::Judgment::Choice(pick) => value(pick),
                thinkthen::Judgment::Score(score) => value(score),
                thinkthen::Judgment::Tags(tags) => value(tags),
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let facts = batch
        .facts()
        .cloned()
        .ok_or_else(|| defect("completed judgments have no facts"))?;
    let rows = records
        .iter()
        .zip(answers)
        .map(|(s, a)| located(s, a))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((value(rows)?, facts))
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
    let records = selection.read()?.collect::<Result<Vec<_>, _>>()?;
    let call = engine.find_with(&asked, records, options)?;
    let answer =
        if let (Some(source), Some(picked)) = (call.value().selected(), call.value().picked()) {
            located(source, value(picked)?)?
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
    let asked = Recognize::from_json(question)?;
    let tally = Tally::new();
    let mut rows = Vec::new();
    for source in selection.read()? {
        let source = source?;
        let call = tally.run(|| engine.recognize_with(&asked, &source.record, options))?;
        let mut answer = decoded(&call.value().to_json())?;
        if let Some(entities) = answer.get_mut("entities").and_then(Value::as_array_mut) {
            for (entity, span) in entities.iter_mut().zip(call.value().entities()) {
                let (first, last) = source.span_lines(span.start(), span.end())?;
                entity["file"] = json!(source.file);
                entity["first_line"] = json!(first);
                entity["last_line"] = json!(last);
            }
        }
        rows.push(located(&source, answer)?);
    }
    Ok((value(rows)?, tally.facts()))
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
    for source in selection.read()? {
        let source = source?;
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
                let mut held = decoded(&edge.to_json())?;
                let source = sources
                    .get(s)
                    .ok_or_else(|| defect("missing source occurrence"))?;
                let target = sources
                    .get(t)
                    .ok_or_else(|| defect("missing target occurrence"))?;
                let source_value = located(
                    source,
                    json!({"name": edge.source().name(), "kind": edge.source().kind()}),
                )?;
                let target_value = located(
                    target,
                    json!({"name": edge.target().name(), "kind": edge.target().kind()}),
                )?;
                put(&mut held, "source", source_value)?;
                put(&mut held, "target", target_value)?;
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
