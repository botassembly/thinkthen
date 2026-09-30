//! The plan door's body: one closed `thinkthen.plan-input/1` object,
//! previewed through the public `Engine::plan_with` (ticket 0291).
//!
//! The object holds `verb`, `question`, `input`, and optional `settings`.
//! A question in text form is built from the settings through the one core
//! settings parser, as the SQL hosts build theirs. A question object takes
//! call controls alone from the settings. The preview reads no key and no
//! cache, sends nothing, and serializes the crate's `PlanEstimate`.

use std::collections::BTreeMap;
use std::num::NonZeroUsize;

use serde::Deserialize;
use serde_json::value::RawValue;
use thinkthen::{
    BatchSetting, CallOptions, Engine, For, LoadedQuestion, Question, QuestionKind, Settings,
};

use crate::failures::Failure;

/// The closed input object. Serde refuses an unknown or repeated member.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanInput<'a> {
    verb: String,
    #[serde(borrow)]
    question: &'a RawValue,
    input: Records,
    #[serde(borrow, default)]
    settings: Option<&'a RawValue>,
}

/// One text, or an ordered array of texts.
#[derive(Deserialize)]
#[serde(untagged)]
enum Records {
    One(String),
    Many(Vec<String>),
}

/// The settings keys a question object leaves to the call.
const CONTROLS: [&str; 3] = ["batch", "context", "deadline_ms"];

/// Preview one plan input as `plan` JSON text.
pub(crate) fn plan(engine: &Engine, text: &str) -> Result<String, Failure> {
    let input: PlanInput<'_> = serde_json::from_str(text).map_err(|_| {
        Failure::usage(
            "the plan input is one JSON object of verb, question, input, and optional settings, \
             with no other or repeated member, and input is one text or an array of texts",
        )
    })?;
    let verb = match input.verb.as_str() {
        "decide" => (For::Decide, QuestionKind::Decide),
        "choose" => (For::Choose, QuestionKind::Choose),
        "score" => (For::Score, QuestionKind::Score),
        "tag" => (For::Tag, QuestionKind::Tag),
        _ => {
            return Err(Failure::usage(
                "plan takes the verb decide, choose, score, or tag",
            ));
        }
    };
    let source = input.settings.map_or("{}", RawValue::get);
    let settings = Settings::parse(source).map_err(usage)?;
    let question = question(input.question.get(), &settings, source, verb)?;
    let records = match input.input {
        Records::One(text) => vec![text],
        Records::Many(texts) => texts,
    };
    let mut options = CallOptions::new();
    if settings.batch_max() {
        options = options.batch(BatchSetting::Max);
    } else if let Some(count) = settings.batch_records().and_then(NonZeroUsize::new) {
        options = options.batch(BatchSetting::Records(count));
    }
    if let Some(context) = settings.context() {
        options = options.context(context);
    }
    let estimate = match &question {
        LoadedQuestion::Question(question) => engine.plan_with(question, records, options)?,
        LoadedQuestion::Banded(question) => engine.plan_with(question, records, options)?,
    };
    serde_json::to_string(&estimate).map_err(|_| Failure::defect("a plan could not be written"))
}

fn usage(error: impl ToString) -> Failure {
    Failure::usage(error.to_string())
}

/// The question the input names, checked against its verb.
fn question(
    raw: &str,
    settings: &Settings,
    source: &str,
    (verb, expected): (For, QuestionKind),
) -> Result<LoadedQuestion, Failure> {
    if raw.starts_with('"') {
        let text: String = serde_json::from_str(raw).map_err(usage)?;
        return Ok(Question::from_json(
            &settings.question_json(verb, &text).map_err(usage)?,
        )?);
    }
    let fields: BTreeMap<String, &RawValue> = serde_json::from_str(raw)
        .map_err(|_| Failure::usage("the question is one text or one JSON object"))?;
    let keys: Vec<&str> = fields.keys().map(String::as_str).collect();
    settings.check(verb).map_err(usage)?;
    settings.conflicts(&keys, false).map_err(usage)?;
    let named: BTreeMap<String, &RawValue> =
        serde_json::from_str(source).map_err(|_| Failure::defect("parsed settings failed"))?;
    if let Some(key) = named.keys().find(|key| !CONTROLS.contains(&key.as_str())) {
        return Err(Failure::usage(format!(
            "the settings key `{key}` belongs in the question object"
        )));
    }
    let question = Question::from_json(raw)?;
    let kind = match &question {
        LoadedQuestion::Banded(_) => QuestionKind::Decide,
        LoadedQuestion::Question(asked) => asked.kind(),
    };
    if kind != expected {
        return Err(Failure::usage("the question object asks another verb"));
    }
    Ok(question)
}
