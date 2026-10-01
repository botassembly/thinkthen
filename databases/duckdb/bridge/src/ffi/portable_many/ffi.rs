//! Keyed SQL calls keep caller keys beside one packed engine result.
#![allow(
    unsafe_code,
    reason = "the bridge copies borrowed DuckDB bytes before its worker starts"
)]

use std::collections::HashSet;
use std::fmt;

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::json;
use serde_json::value::RawValue;
use thinkthen::{
    Answer, For, Judgment, LoadedQuestion, Probabilities, Question, QuestionKind, Settings,
};

use crate::engines;
use crate::errors::RowError;

use super::{
    BridgeSettings, BridgeStop, Reply, asked, batch, probe, reply_boundary, run_detached, text,
};

struct Keyed(Vec<(String, String)>);

pub(super) fn keyed_texts(source: &str) -> Result<Vec<String>, String> {
    let keyed = serde_json::from_str::<Keyed>(source)
        .map_err(|error| RowError::usage(&error.to_string()).text)?;
    Ok(keyed.0.into_iter().map(|(_, text)| text).collect())
}

struct Fields(Vec<(String, Box<RawValue>)>);

struct Ordered;

impl<'de> Visitor<'de> for Ordered {
    type Value = Fields;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one JSON object")
    }
    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Fields, M::Error> {
        let mut fields = Vec::new();
        while let Some(field) = map.next_entry()? {
            fields.push(field);
        }
        Ok(Fields(fields))
    }
}

impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        decoder.deserialize_map(Ordered)
    }
}

struct Entries;

impl<'de> Visitor<'de> for Entries {
    type Value = Keyed;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one keyed JSON object of text values")
    }
    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Keyed, M::Error> {
        let mut pairs = Vec::new();
        let mut names = HashSet::new();
        while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
            if !names.insert(key.clone()) {
                return Err(serde::de::Error::custom("a keyed input repeats a key"));
            }
            let Some(value) = value.as_str() else {
                return Err(serde::de::Error::custom("each keyed input value is text"));
            };
            pairs.push((key, value.to_owned()));
        }
        Ok(Keyed(pairs))
    }
}

impl<'de> Deserialize<'de> for Keyed {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        decoder.deserialize_map(Entries)
    }
}

pub(super) fn parse(
    question: &str,
    from_file: bool,
    settings: &str,
    kind: i32,
) -> Result<(String, LoadedQuestion, Settings), String> {
    if kind == RANK {
        return ranked(question, settings);
    }
    let verb = match kind {
        0 => For::Decide,
        4 => For::Choose,
        5 => For::Score,
        6 => For::Tag,
        _ => return Err("thinkthen defect: unknown keyed kind".into()),
    };
    let settings_value =
        Settings::parse(settings).map_err(|error| RowError::usage(&error.to_string()).text)?;
    let written = if from_file || question.trim_start().starts_with('{') {
        super::question_typed(question, from_file).map_err(|error| error.text)?;
        let source: serde_json::Value = serde_json::from_str(question)
            .map_err(|error| RowError::usage(&error.to_string()).text)?;
        let keys: Vec<&str> = source
            .as_object()
            .ok_or_else(|| RowError::usage("a question is one JSON object").text)?
            .keys()
            .map(String::as_str)
            .collect();
        settings_value
            .conflicts(&keys, false)
            .map_err(|error| RowError::usage(&error.to_string()).text)?;
        if settings_value.none().is_some() {
            return Err(
                RowError::usage("the settings key `none` does not belong to this verb").text,
            );
        }
        let Fields(fields) = serde_json::from_str::<Fields>(settings)
            .map_err(|error| RowError::usage(&error.to_string()).text)?;
        let end = question
            .rfind('}')
            .ok_or_else(|| "thinkthen defect: a checked question lost its object".to_owned())?;
        let mut joined = question[..end].trim_end().to_owned();
        for (key, value) in fields {
            if matches!(key.as_str(), "context" | "batch" | "deadline_ms" | "none") {
                continue;
            }
            joined.push(',');
            joined.push_str(
                &serde_json::to_string(&key)
                    .map_err(|_| "thinkthen defect: a settings key could not be written")?,
            );
            joined.push(':');
            joined.push_str(value.get());
        }
        joined.push('}');
        joined
    } else {
        settings_value
            .question_json(verb, question)
            .map_err(|error| RowError::usage(&error.to_string()).text)?
    };
    let parsed = Question::from_json(&written).map_err(|error| RowError::from(error).text)?;
    let expected = match kind {
        0 => QuestionKind::Decide,
        4 => QuestionKind::Choose,
        5 => QuestionKind::Score,
        6 => QuestionKind::Tag,
        _ => return Err("thinkthen defect: unknown keyed kind".into()),
    };
    if parsed.kind() != expected {
        return Err(RowError::usage("the question has another kind").text);
    }
    Ok((written, parsed, settings_value))
}

/// The keyed kind `thinkthen_rank` sends; its question is literal text.
const RANK: i32 = 8;

/// A rank question is literal text, as find's is, and takes only `model`
/// among question fields.
fn ranked(question: &str, settings: &str) -> Result<(String, LoadedQuestion, Settings), String> {
    let call = Settings::parse(settings)
        .and_then(|value| {
            value.check(For::Rank)?;
            Ok(value)
        })
        .map_err(|error| RowError::usage(&error.to_string()).text)?;
    let asked = Question::rank(question).and_then(|asked| match call.model() {
        Some(model) => asked.with_model(model),
        None => Ok(asked),
    });
    let asked = asked.map_err(|error| RowError::from(error).text)?;
    Ok((String::new(), LoadedQuestion::Question(asked), call))
}

struct KeyedCall {
    question: LoadedQuestion,
    input: Keyed,
    call: Settings,
    query: i64,
    session: BridgeSettings,
    stop: BridgeStop,
    kind: i32,
}

fn choice_probability(chosen: Option<&str>, probabilities: &Probabilities) -> Option<f64> {
    match (chosen, probabilities) {
        (Some(name), Probabilities::Named(named)) => named
            .iter()
            .find(|entry| entry.name() == name)
            .map(|entry| entry.probability()),
        _ => None,
    }
}

fn row_value(
    value: &Judgment,
    probabilities: &Probabilities,
) -> Result<(serde_json::Value, Option<f64>), String> {
    match value {
        Judgment::Choice(chosen) => Ok((
            json!(chosen),
            choice_probability(chosen.as_deref(), probabilities),
        )),
        Judgment::Score(score) => Ok((json!(score), None)),
        Judgment::Tags(labels) => Ok((json!(labels), None)),
        _ => Err("thinkthen defect: a keyed call returned another kind".into()),
    }
}

fn run(
    KeyedCall {
        question,
        input,
        call,
        query,
        session,
        stop,
        kind,
    }: KeyedCall,
) -> Result<Vec<u8>, String> {
    let batch_word = super::portable::batch_word(&call);
    let session_batch = batch(&session)?;
    let asked = asked(&session)?;
    let engine = engines::engine_for(&asked, |path| probe(&session, path))?;
    let total = asked.max_requests_total;
    let context = call.context().map(str::to_owned);
    let texts: Vec<String> = input.0.iter().map(|(_, value)| value.clone()).collect();
    run_detached(stop, move |token| {
        let options = engines::options_for(
            super::portable::due(query, call.deadline_ms()),
            &token,
            total,
            batch_word.as_deref().or(session_batch.as_deref()),
            context.as_deref(),
        )?;
        let mut rows = Vec::new();
        if kind == RANK {
            let LoadedQuestion::Question(question) = &question else {
                return Err("thinkthen defect: a rank question was banded".into());
            };
            if texts.is_empty() {
                return Ok(b"[]".to_vec());
            }
            let ranked = engine
                .rank_with(question, texts, options)
                .map_err(|error| engines::call_error(error, total).text)?;
            for (place, row) in ranked.value().iter().enumerate() {
                let (key, _) = input
                    .0
                    .get(row.index())
                    .ok_or("thinkthen defect: a ranked row lost its record")?;
                rows.push(json!({"key":key,"rank":place + 1,"probability":row.probability()}));
            }
        } else if kind == 0 {
            let decided = engine
                .decide_many_with(&question, texts, options)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| engines::call_error(error, total).text)?;
            for ((key, _), value) in input.0.iter().zip(decided) {
                rows.push(json!({"key":key,"value":match value.value() {
                    Answer::Yes => Some(true), Answer::No => Some(false), Answer::Unsure => None,
                },"probability":value.probability()}));
            }
        } else {
            let answered = engine
                .details_many_with(&question, texts, options)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| engines::call_error(error, total).text)?;
            for ((key, _), value) in input.0.iter().zip(answered) {
                let details = value.value();
                let (value, probability) = row_value(details.value(), details.probabilities())?;
                rows.push(json!({"key":key,"value":value,"probability":probability}));
            }
        }
        serde_json::to_vec(&rows)
            .map_err(|_| "thinkthen defect: keyed rows could not be written".into())
    })
}

/// Validate a complete keyed call before another member of its DuckDB chunk sends.
///
/// # Safety
/// All byte ranges remain live during the call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_portable_many(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    keyed: *const u8,
    keyed_len: usize,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let keyed = text(keyed, keyed_len)?;
        let settings = text(settings, settings_len)?;
        parse(question, from_file != 0, settings, kind)?;
        serde_json::from_str::<Keyed>(keyed)
            .map_err(|error| RowError::usage(&error.to_string()).text)?;
        Ok(Vec::new())
    })
}

/// Ask one keyed object under the existing detached statement scope.
///
/// # Safety
/// All byte ranges and the stop callback remain live until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_portable_many(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    keyed: *const u8,
    keyed_len: usize,
    settings: *const u8,
    settings_len: usize,
    kind: i32,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let keyed = text(keyed, keyed_len)?;
        let settings = text(settings, settings_len)?;
        let (_, question, call) = parse(question, from_file != 0, settings, kind)?;
        let input = serde_json::from_str::<Keyed>(keyed)
            .map_err(|error| RowError::usage(&error.to_string()).text)?;
        run(KeyedCall {
            question,
            input,
            call,
            query: query_deadline_ms,
            session,
            stop,
            kind,
        })
    })
}
