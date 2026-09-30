//! Portable SQL call settings beside the existing question-file grammar.

use thinkthen::{For, LoadedQuestion, Question, QuestionKind, Settings};

use crate::engines;
use crate::errors::RowError;

use super::{
    BridgeSettings, BridgeStop, BridgeText, Reply, asked, batch, copied_texts, probe,
    reply_boundary, run_detached, text,
};

fn usage(error: impl std::fmt::Display) -> String {
    RowError::usage(&error.to_string()).text
}

/// Keep a validated JSON question's exact member order while adding fields
/// validated and serialized by the shared settings parser. The original
/// question is parsed first, so this assembly never substitutes for its
/// duplicate-name, grammar, or file-origin checks.
fn combined(argument: &str, settings: &Settings) -> Result<String, String> {
    let source: serde_json::Value = serde_json::from_str(argument).map_err(usage)?;
    let object = source
        .as_object()
        .ok_or_else(|| RowError::usage("a question is one JSON object").text)?;
    let keys = object.keys().map(String::as_str).collect::<Vec<_>>();
    settings.conflicts(&keys, false).map_err(usage)?;
    let fields = settings
        .question_json(For::Decide, "settings placeholder")
        .map_err(usage)?;
    let tail = fields
        .strip_prefix("{\"decide\":\"settings placeholder\"")
        .and_then(|rest| rest.strip_suffix('}'))
        .ok_or_else(|| "thinkthen defect: the settings writer changed its shape".to_owned())?;
    let end = argument
        .rfind('}')
        .ok_or_else(|| "thinkthen defect: a checked question lost its object".to_owned())?;
    let mut joined = argument[..end].trim_end().to_owned();
    joined.push_str(tail);
    joined.push('}');
    Ok(joined)
}

pub(super) fn decide(
    argument: &str,
    from_file: bool,
    settings_text: &str,
    threshold: Option<&str>,
) -> Result<(LoadedQuestion, Settings), String> {
    let mut settings = Settings::parse(settings_text).map_err(usage)?;
    if let Some(threshold) = threshold {
        settings.conflicts(&["threshold"], false).map_err(usage)?;
        let text = settings_text.trim();
        let inside = text
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .ok_or_else(|| "thinkthen defect: checked settings lost their object".to_owned())?;
        let value = serde_json::to_string(threshold).map_err(usage)?;
        let join = if inside.trim().is_empty() { "" } else { "," };
        settings =
            Settings::parse(&format!("{{\"threshold\":{value}{join}{inside}}}")).map_err(usage)?;
    }
    settings.check(For::Decide).map_err(usage)?;
    let written = if from_file || argument.trim_start().starts_with('{') {
        super::question_typed(argument, from_file).map_err(|error| error.text)?;
        combined(argument, &settings)?
    } else {
        settings
            .question_json(For::Decide, argument)
            .map_err(usage)?
    };
    let question = Question::from_json(&written).map_err(|error| RowError::from(error).text)?;
    if question.kind() != QuestionKind::Decide {
        return Err(RowError::usage("the question has another kind").text);
    }
    Ok((question, settings))
}

pub(super) fn batch_word(settings: &Settings) -> Option<String> {
    if settings.batch_max() {
        Some("max".to_owned())
    } else {
        settings.batch_records().map(|value| value.to_string())
    }
}

pub(super) fn due(query: i64, call: Option<i64>) -> i64 {
    match call {
        None | Some(-1) => query,
        Some(value) if query < 0 => value,
        Some(value) => query.min(value),
    }
}

/// Refuse one complete portable call before the caller starts any group.
///
/// # Safety
/// The question, settings and threshold byte ranges remain live through this call.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_validate_portable_decide(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    settings: *const u8,
    settings_len: usize,
    threshold: *const u8,
    threshold_len: usize,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let settings = text(settings, settings_len)?;
        let threshold = (!threshold.is_null())
            .then(|| text(threshold, threshold_len))
            .transpose()?;
        decide(question, from_file != 0, settings, threshold).map(|_| Vec::new())
    })
}

/// One grouped portable decision through the existing engine and call scope.
///
/// # Safety
/// All byte ranges, text array and stop predicate remain live until return.
#[unsafe(no_mangle)]
pub(crate) unsafe extern "C" fn thinkthen_cpp_portable_decide_group(
    question: *const u8,
    question_len: usize,
    from_file: i32,
    texts: *const BridgeText,
    count: usize,
    settings: *const u8,
    settings_len: usize,
    threshold: *const u8,
    threshold_len: usize,
    query_deadline_ms: i64,
    session: BridgeSettings,
    stop: BridgeStop,
) -> Reply {
    reply_boundary(|| {
        let question = text(question, question_len)?;
        let settings_text = text(settings, settings_len)?;
        let threshold = (!threshold.is_null())
            .then(|| text(threshold, threshold_len))
            .transpose()?;
        let (question, call) = decide(question, from_file != 0, settings_text, threshold)?;
        let copied = copied_texts(texts, count)?;
        let session_batch = batch(&session)?;
        let call_batch = batch_word(&call);
        let asked = asked(&session)?;
        let engine = engines::engine_for(&asked, |path| probe(&session, path))?;
        let total = asked.max_requests_total;
        let deadline = due(query_deadline_ms, call.deadline_ms());
        let context = call.context().map(str::to_owned);
        run_detached(stop, move |token| {
            let options = engines::options_for(
                deadline,
                &token,
                total,
                call_batch.as_deref().or(session_batch.as_deref()),
                context.as_deref(),
            )?;
            let rows = engine
                .decide_many_with(&question, copied, options)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| engines::call_error(error, total).text)?;
            Ok(rows
                .into_iter()
                .map(|row| match row.value() {
                    thinkthen::Answer::No => 0,
                    thinkthen::Answer::Yes => 1,
                    thinkthen::Answer::Unsure => 2,
                })
                .collect())
        })
    })
}

#[cfg(test)]
mod tests {
    use super::decide;
    use thinkthen::Question;

    #[test]
    fn one_parser_merges_plain_and_structured_questions_without_losing_member_order() {
        let (plain, settings) = decide(
            "Refund?",
            false,
            r#"{"true":"Money back.","deadline_ms":0}"#,
            None,
        )
        .expect("settings");
        assert_eq!(settings.deadline_ms(), Some(0));
        let expected =
            Question::from_json(r#"{"decide":"Refund?","true":"Money back."}"#).expect("question");
        assert_eq!(format!("{plain:?}"), format!("{expected:?}"));

        let (structured, _) = decide(
            r#"{"decide":{"ask":"Refund?"},"false":null}"#,
            false,
            r#"{"true":"Money back."}"#,
            None,
        )
        .expect("structured");
        let expected = Question::from_json(
            r#"{"decide":{"ask":"Refund?"},"false":null,"true":"Money back."}"#,
        )
        .expect("question");
        assert_eq!(format!("{structured:?}"), format!("{expected:?}"));
        assert!(
            decide(
                r#"{"decide":"Refund?","true":null}"#,
                false,
                r#"{"true":"yes"}"#,
                None
            )
            .expect_err("duplicate field")
            .contains("settings repeats `true`")
        );
        assert!(
            decide("Refund?", false, r#"{"threshold":0.5}"#, Some("0.7"))
                .expect_err("duplicate threshold")
                .contains("settings repeats `threshold`")
        );
    }
}
