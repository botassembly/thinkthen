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

struct Fields(Vec<(String, Box<RawValue>)>);

impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
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
        decoder.deserialize_map(Ordered)
    }
}

impl<'de> Deserialize<'de> for Keyed {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
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
        decoder.deserialize_map(Entries)
    }
}

pub(super) fn parse(
    question: &str,
    from_file: bool,
    settings: &str,
    kind: i32,
) -> Result<(String, LoadedQuestion, Settings), String> {
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
    if !matches!(&parsed, LoadedQuestion::Question(value) if value.kind() == expected)
        && !(kind == 0 && matches!(&parsed, LoadedQuestion::Banded(_)))
    {
        return Err(RowError::usage("the question has another kind").text);
    }
    Ok((written, parsed, settings_value))
}

fn batch_word(settings: &Settings) -> Option<String> {
    if settings.batch_max() {
        Some("max".into())
    } else {
        settings.batch_records().map(|value| value.to_string())
    }
}

fn due(query: i64, call: Option<i64>) -> i64 {
    match call {
        None | Some(-1) => query,
        Some(value) if query < 0 => value,
        Some(value) => query.min(value),
    }
}

fn run(
    question: LoadedQuestion,
    input: Keyed,
    call: Settings,
    query: i64,
    session: BridgeSettings,
    stop: BridgeStop,
    kind: i32,
) -> Result<Vec<u8>, String> {
    let batch_word = batch_word(&call);
    let session_batch = batch(&session)?;
    let asked = asked(&session)?;
    let engine = engines::engine_for(&asked, |path| probe(&session, path))?;
    let total = asked.max_requests_total;
    let context = call.context().map(str::to_owned);
    let texts: Vec<String> = input.0.iter().map(|(_, value)| value.clone()).collect();
    run_detached(stop, move |token| {
        let options = engines::options_for(
            due(query, call.deadline_ms()),
            &token,
            total,
            batch_word.as_deref().or(session_batch.as_deref()),
            context.as_deref(),
        )?;
        let mut rows = Vec::new();
        if kind == 0 {
            let decided = match &question {
                LoadedQuestion::Question(held) => engine
                    .decide_many_with(held, texts, options)
                    .collect::<Result<Vec<_>, _>>(),
                LoadedQuestion::Banded(held) => engine
                    .decide_many_with(held, texts, options)
                    .collect::<Result<Vec<_>, _>>(),
            }
            .map_err(|error| engines::call_error(error, total).text)?;
            for ((key, _), value) in input.0.iter().zip(decided) {
                rows.push(json!({"key":key,"value":match value.value() {
                    Answer::Yes => Some(true), Answer::No => Some(false), Answer::Unsure => None,
                },"probability":value.probability()}));
            }
        } else {
            let LoadedQuestion::Question(held) = &question else {
                return Err(RowError::usage("the question has another kind").text);
            };
            let answered = engine
                .details_many_with(held, texts, options)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| engines::call_error(error, total).text)?;
            for ((key, _), value) in input.0.iter().zip(answered) {
                let details = value.value();
                let (value, probability) = match details.value() {
                    Judgment::Choice(chosen) => {
                        let probability = match (chosen, details.probabilities()) {
                            (Some(name), Probabilities::Named(named)) => named
                                .iter()
                                .find(|entry| entry.name() == name)
                                .map(|entry| entry.probability()),
                            _ => None,
                        };
                        (json!(chosen), probability)
                    }
                    Judgment::Score(score) => (json!(score), None),
                    Judgment::Tags(labels) => (json!(labels), None),
                    _ => return Err("thinkthen defect: a keyed call returned another kind".into()),
                };
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
        run(
            question,
            input,
            call,
            query_deadline_ms,
            session,
            stop,
            kind,
        )
    })
}
