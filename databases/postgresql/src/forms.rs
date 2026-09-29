//! Portable settings and PostgreSQL's defaulted named conveniences.

use pgrx::datum::{Array, JsonB};
use thinkthen::{For, LoadedQuestion, Question, QuestionKind, Settings, SettingsError};

use crate::call::{self, Call, OrRaise as _, Refusal};
use crate::ffi::RawJson;
use crate::files::Given;

#[derive(Clone, Copy, Default)]
pub(crate) struct Named<'a> {
    pub(crate) threshold: Option<&'a str>,
    pub(crate) context: Option<&'a str>,
    pub(crate) model: Option<&'a str>,
    pub(crate) batch: Option<&'a str>,
    pub(crate) deadline_ms: Option<i64>,
}

fn settings(raw: Option<&RawJson>, named: Named<'_>) -> Result<Settings, Refusal> {
    let original = raw.map_or("{}", |value| value.0.as_str());
    // The engine constructor accepts this 0299 key, but this host has no
    // active reservation. Keep the portable parser's duplicate-name check
    // before classifying the unsupported host key.
    Settings::parse(original).map_err(|error| match error {
        SettingsError::UnknownKey(key) if key == "max_estimated_input_tokens_total" => {
            Refusal::usage(
                "max_estimated_input_tokens_total is not supported by this PostgreSQL host",
            )
        }
        other => Refusal::usage(other.to_string()),
    })?;
    let mut merged = original.trim().strip_suffix('}').unwrap_or("{").to_owned();
    for (key, value) in [
        ("threshold", named.threshold),
        ("context", named.context),
        ("model", named.model),
    ] {
        if let Some(value) = value {
            if !merged.trim_end().ends_with('{') {
                merged.push(',');
            }
            merged.push_str(&format!(
                "\"{key}\":{}",
                serde_json::Value::String(value.into())
            ));
        }
    }
    if let Some(value) = named.batch {
        if !merged.trim_end().ends_with('{') {
            merged.push(',');
        }
        let batch = value.parse::<u64>().map_or_else(
            |_| serde_json::Value::String(value.into()),
            |count| serde_json::Value::Number(count.into()),
        );
        merged.push_str(&format!("\"batch\":{batch}"));
    }
    if let Some(value) = named.deadline_ms {
        if !merged.trim_end().ends_with('{') {
            merged.push(',');
        }
        merged.push_str(&format!("\"deadline_ms\":{value}"));
    }
    merged.push('}');
    Settings::parse(&merged).map_err(|error| Refusal::usage(error.to_string()))
}

pub(crate) fn controls(raw: Option<&RawJson>, named: Named<'_>) -> (Settings, Call) {
    controls_result(raw, named).or_raise()
}

pub(crate) fn controls_result(
    raw: Option<&RawJson>,
    named: Named<'_>,
) -> Result<(Settings, Call), Refusal> {
    let settings = settings(raw, named)?;
    let call = call::read_result()?.with_settings(&settings)?;
    Ok((settings, call))
}

/// Aggregate questions keep their own question fields; only call controls
/// and the engine's default model can apply to the whole set.
pub(crate) fn aggregate_controls(raw: Option<&RawJson>) -> Call {
    let (_, mut call) = controls(raw, Named::default());
    let Some(raw) = raw else { return call };
    let value: serde_json::Value = serde_json::from_str(&raw.0)
        .unwrap_or_else(|_| call::raise(Refusal::usage("settings is one JSON object")));
    let Some(fields) = value.as_object() else {
        call::raise(Refusal::usage("settings is one JSON object"));
    };
    for key in fields.keys() {
        if !matches!(key.as_str(), "batch" | "deadline_ms" | "model") {
            call::raise(Refusal::usage(format!(
                "annotate does not take setting `{key}`"
            )));
        }
    }
    if let Some(model) = fields.get("model").and_then(serde_json::Value::as_str) {
        call = call.with_model(model);
    }
    call
}

pub(crate) fn question(
    argument: Option<&str>,
    members: Option<Array<'_, &str>>,
    verb: For,
    settings: &Settings,
) -> LoadedQuestion {
    question_result(argument, members, verb, settings).or_raise()
}

pub(crate) fn question_result(
    argument: Option<&str>,
    members: Option<Array<'_, &str>>,
    verb: For,
    settings: &Settings,
) -> Result<LoadedQuestion, Refusal> {
    let key = match verb {
        For::Decide => "",
        For::Choose => "options",
        For::Score => "levels",
        For::Tag => "labels",
        For::Find => return Err(Refusal::usage("find is not a judgment question")),
    };
    let list = members.map(|held| held.iter().flatten().map(str::to_owned).collect());
    let asked = Given::read_question(argument, key, call::file_directory().as_deref())
        .and_then(|given| given.with_members(key, list))
        .and_then(|given| given.with_settings(settings, verb))
        .and_then(|given| given.parse(Question::from_json))?;
    let kind = match &asked {
        LoadedQuestion::Banded(_) => QuestionKind::Decide,
        LoadedQuestion::Question(question) => question.kind(),
    };
    let wanted = match verb {
        For::Decide => QuestionKind::Decide,
        For::Choose => QuestionKind::Choose,
        For::Score => QuestionKind::Score,
        For::Tag => QuestionKind::Tag,
        For::Find => return Err(Refusal::usage("find is not a judgment question")),
    };
    if kind != wanted {
        return Err(Refusal::usage(format!(
            "a {} call takes a {} question",
            key_for(verb),
            key_for(verb)
        )));
    }
    Ok(asked)
}

const fn key_for(verb: For) -> &'static str {
    match verb {
        For::Decide => "decide",
        For::Choose => "choose",
        For::Score => "score",
        For::Tag => "tag",
        For::Find => "find",
    }
}

pub(crate) fn keyed(argument: Option<JsonB>) -> Vec<(String, String)> {
    let Some(JsonB(value)) = argument else {
        return Vec::new();
    };
    let fields = value
        .as_object()
        .ok_or_else(|| Refusal::usage("keyed input is one JSON object"))
        .or_raise();
    fields
        .iter()
        .map(|(key, value)| {
            value
                .as_str()
                .map(|text| (key.clone(), text.to_owned()))
                .ok_or_else(|| Refusal::usage(format!("keyed input value for {key} is text")))
                .or_raise()
        })
        .collect()
}

pub(crate) fn plan_verb(argument: &str, settings: &Settings) -> For {
    plan_verb_result(argument, settings).or_raise()
}

pub(crate) fn plan_verb_result(argument: &str, settings: &Settings) -> Result<For, Refusal> {
    if argument.starts_with('@') || argument.trim_start().starts_with('{') {
        let asked = Given::read_question(Some(argument), "", call::file_directory().as_deref())
            .and_then(|given| given.parse(Question::from_json))?;
        return Ok(match asked {
            LoadedQuestion::Banded(_) => For::Decide,
            LoadedQuestion::Question(question) => match question.kind() {
                QuestionKind::Decide => For::Decide,
                QuestionKind::Choose => For::Choose,
                QuestionKind::Score => For::Score,
                QuestionKind::Tag => For::Tag,
                _ => return Err(Refusal::usage("plan takes a judgment question")),
            },
        });
    }
    let choose = settings.question_json(For::Choose, argument).is_ok();
    if choose && settings.question_json(For::Decide, argument).is_err() {
        return Ok(For::Choose);
    }
    for verb in [For::Score, For::Tag] {
        if settings.question_json(verb, argument).is_ok()
            && settings.question_json(For::Decide, argument).is_err()
        {
            return Ok(verb);
        }
    }
    Ok(For::Decide)
}
