//! Reading a recorded wire question back as the logical question that
//! decodes its answer alone, for `cache convert`.

use crate::core::json::Json;
use crate::core::question::{Labels, Question};
use crate::core::text::QuestionText;

/// Validate the concrete wire type and serialize its owned fields in adapter order.
pub(crate) fn canonical_question(text: &str) -> Option<(String, Question)> {
    // Json rejects duplicate keys, including options, before the typed reader.
    let original = Json::parse(text).ok()?;
    let decoded = decoder(&original)?;
    let question: super::request::RequestQuestion = serde_json::from_str(text).ok()?;
    let canonical = serde_json::to_string(&question).ok()?;
    Some((canonical, decoded))
}

/// The member one wire question holds under `name`.
fn field<'a>(question: &'a Json, name: &str) -> Option<&'a Json> {
    let Json::Object(members) = question else {
        return None;
    };
    members
        .iter()
        .find(|(held, _)| held == name)
        .map(|(_, value)| value)
}

/// The logical question that reads one wire question's answer alone. Only
/// the kind and the options or the level count matter to a decoder.
pub(crate) fn decoder(question: &Json) -> Option<Question> {
    let text = QuestionText::new("converted").ok()?;
    match field(question, "type")? {
        Json::String(kind) if kind == "noul" => Some(Question::Decide {
            text,
            yes: None,
            no: None,
        }),
        Json::String(kind) if kind == "choice" => {
            let Json::Object(criteria) = field(question, "criteria")? else {
                return None;
            };
            let options = criteria.iter().map(|(option, _)| option.clone()).collect();
            Some(Question::Choose {
                text,
                options: Labels::options(options).ok()?,
            })
        }
        Json::String(kind) if kind == "score" => {
            let Json::Array(criteria) = field(question, "criteria")? else {
                return None;
            };
            let levels = (0..criteria.len())
                .map(|level| (level.to_string(), None))
                .collect();
            Some(Question::Score {
                text,
                levels: Labels::levels(levels).ok()?,
            })
        }
        _ => None,
    }
}
