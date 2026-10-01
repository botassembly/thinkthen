//! One old request-level entry read as question entries, by ADR 0111 section 9.
//!
//! The request's state, model and questions are written again as compact JSON
//! and must join back into the recorded body byte for byte, so each key hashes
//! the bytes that were sent. The reply is split with one decoder per wire
//! question, and each good answer keeps its even share of the reply's usage.

use super::{Entry, EntryError, SCHEMA, place};
use crate::core::adapters::built_in;
use crate::core::batch::{QUOTED, quoted};
use crate::core::json::Json;
use crate::core::pack::{QuestionKey, shares, split};
use crate::core::result::Usage;
use crate::core::text::Url;

/// Where a converted answer came from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Origin {
    /// Read under the form it was asked in.
    Converted,
    /// Asked in the old form and written under the quoted form.
    Quoted,
}

impl Origin {
    /// The `origin` column's word.
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Converted => "converted",
            Self::Quoted => "quoted",
        }
    }
}

/// One good answer of an old entry, with every part of its question entry.
#[derive(Debug)]
pub(crate) struct Converted {
    pub(crate) key: QuestionKey,
    pub(crate) url: String,
    pub(crate) model: String,
    pub(crate) state: String,
    pub(crate) question: String,
    pub(crate) answer: String,
    pub(crate) answered_by: String,
    pub(crate) usage: Option<Usage>,
    pub(crate) origin: Origin,
    /// Written only in the old form, which a replay on this version misses.
    pub(crate) unquoted: bool,
}

/// Why an old entry gave no question entry.
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Converting {
    /// The file is no entry this version reads.
    Unreadable(EntryError),
    /// The request's parts written again do not rejoin into its body.
    Unjoined,
    /// A wire question is of no kind this version decodes.
    Unasked,
    /// The reply holds no answer that decodes.
    Unanswered,
}

impl Converting {
    /// The warning a skipped entry gets, naming nothing it holds.
    pub(crate) const fn reason(&self) -> &'static str {
        match self {
            Self::Unreadable(_) => "it is not a recording entry",
            Self::Unjoined => "its request does not rejoin byte for byte from its parts",
            Self::Unasked => "a question in its request is not one this version decodes",
            Self::Unanswered => "its reply holds no answer that decodes",
        }
    }
}

/// Read one old entry's good answers. The `{"records":[…]}` state becomes the
/// fixed sentence, because its questions already quote their records. With
/// `quote`, a single-record exchange is also written in the quoted form.
pub(crate) fn convert(bytes: &[u8], quote: bool) -> Result<Vec<Converted>, Converting> {
    let entry: Entry =
        serde_json::from_slice(bytes).map_err(|error| Converting::Unreadable(place(error)))?;
    if entry.schema != SCHEMA || entry.adapter != built_in::NAME {
        return Err(Converting::Unreadable(EntryError::Schema));
    }
    let url = Url::new(&entry.url).map_err(|_| Converting::Unreadable(EntryError::Mismatched))?;
    let body = entry.request.get();
    let (state, model, questions) = parts(body).ok_or(Converting::Unjoined)?;
    let written = |value: &Json| serde_json::to_string(value).map_err(|_| Converting::Unjoined);
    let state_json = written(&state)?;
    let model_json = serde_json::to_string(&model).map_err(|_| Converting::Unjoined)?;
    let question_json = questions
        .iter()
        .map(written)
        .collect::<Result<Vec<_>, _>>()?;
    let joined = built_in::join(
        &state_json,
        &model_json,
        question_json.iter().map(String::as_str),
    );
    if joined != body.as_bytes() {
        return Err(Converting::Unjoined);
    }
    let decoders = questions
        .iter()
        .map(built_in::decoder)
        .collect::<Option<Vec<_>>>()
        .ok_or(Converting::Unasked)?;
    let reply =
        split(&decoders, entry.response.get().as_bytes()).map_err(|_| Converting::Unanswered)?;
    let usage = shares(reply.usage, decoders.len());
    let fixed = serde_json::to_string(QUOTED).map_err(|_| Converting::Unjoined)?;
    let own_state = if records_form(&state) {
        fixed.clone()
    } else {
        state_json.clone()
    };
    let origin = if entry.quoted {
        Origin::Quoted
    } else {
        Origin::Converted
    };
    // An old-form exchange: quote it now, or it already quotes its records,
    // or it cannot take the quote.
    let form = (own_state != fixed && !entry.quoted).then(|| requote(&state_json, &questions));
    let requoted = match &form {
        Some(Requote::Quoted(all)) if quote => Some(all),
        _ => None,
    };
    let unquoted = matches!(form, Some(Requote::Quoted(_) | Requote::Unquotable)) && requoted.is_none();
    let mut converted = Vec::new();
    for (place, answer) in reply.answers.into_iter().enumerate() {
        let (Ok(answer), Some(question), Some(usage)) =
            (answer, question_json.get(place), usage.get(place))
        else {
            continue;
        };
        let row = |state: &str, question: &str, origin: Origin| Converted {
            key: QuestionKey::of(&url, &model_json, state, question),
            url: entry.url.clone(),
            model: model.clone(),
            state: state.to_owned(),
            question: question.to_owned(),
            answer: answer.clone(),
            answered_by: reply.model.as_str().to_owned(),
            usage: *usage,
            origin,
            unquoted: unquoted && origin == Origin::Converted,
        };
        converted.push(row(&own_state, question, origin));
        if let Some(quoted) = requoted.and_then(|all| all.get(place)) {
            converted.push(row(&fixed, quoted, Origin::Quoted));
        }
    }
    if converted.is_empty() {
        return Err(Converting::Unanswered);
    }
    Ok(converted)
}

/// The request's state, model and wire questions, in order.
fn parts(body: &str) -> Option<(Json, String, Vec<Json>)> {
    let Json::Object(members) = Json::parse(body).ok()? else {
        return None;
    };
    let member = |name: &str| {
        members
            .iter()
            .find(|(held, _)| held == name)
            .map(|(_, value)| value)
    };
    let Some(Json::String(model)) = member("model") else {
        return None;
    };
    let Some(Json::Object(questions)) = member("questions") else {
        return None;
    };
    let questions = questions
        .iter()
        .map(|(_, question)| question.clone())
        .collect();
    Some((member("state")?.clone(), model.clone(), questions))
}

/// Whether a state is the old `{"records":[…]}` form.
fn records_form(state: &Json) -> bool {
    matches!(state, Json::Object(members)
        if matches!(members.as_slice(), [(name, Json::Array(_))] if name == "records"))
}

/// What `--quote` can do with one old single-record exchange.
#[derive(Debug, Eq, PartialEq)]
enum Requote {
    /// Each question with the record quoted into its string instructions.
    Quoted(Vec<String>),
    /// Its questions already quote their records, as a context's questions do.
    Already,
    /// An instruction is JSON, which cannot take the quote.
    Unquotable,
}

/// Quote `line`, the single record the state carried, into each question's
/// string instructions.
fn requote(line: &str, questions: &[Json]) -> Requote {
    let mut written = Vec::new();
    for question in questions {
        let Json::Object(members) = question else {
            return Requote::Unquotable;
        };
        let mut members = members.clone();
        let Some((_, instructions)) = members
            .iter_mut()
            .find(|(name, _)| name == "instructions")
        else {
            return Requote::Unquotable;
        };
        let Json::String(asked) = instructions else {
            return Requote::Unquotable;
        };
        if quotes_a_record(line, asked) {
            return Requote::Already;
        }
        *instructions = Json::String(quoted(line, asked));
        match serde_json::to_string(&Json::Object(members)) {
            Ok(text) => written.push(text),
            Err(_) => return Requote::Unquotable,
        }
    }
    Requote::Quoted(written)
}

/// Whether an instruction already opens with a quoted record: `The text is `,
/// one complete JSON value, and `. `. The value is the state's own record, or
/// a string, list or object, as a context exchange quotes its records. A bare
/// scalar that is not the state, as in "The text is true. Is it?", is the
/// question's own words. No old entry tells a question that opens with a
/// quoted word from a quoted record, so such a question is left unquoted.
fn quotes_a_record(line: &str, asked: &str) -> bool {
    let Some(rest) = asked.strip_prefix("The text is ") else {
        return false;
    };
    rest.match_indices(". ").any(|(end, _)| {
        let value = &rest[..end];
        value == line
            || matches!(
                Json::parse(value),
                Ok(Json::String(_) | Json::Array(_) | Json::Object(_))
            )
    })
}

#[cfg(test)]
#[path = "convert/tests.rs"]
mod tests;
