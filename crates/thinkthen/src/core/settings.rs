//! The pure `thinkthen.settings/1` grammar (ADR 0105, item 1).
//!
//! Question fields go through the existing question-file parser. Call fields
//! stay outside request bytes and the request digest.

use std::fmt;

use thiserror::Error;

use crate::core::batch::Setting;
use crate::core::json::{Json, JsonError};
use crate::core::question_file::{QuestionFile, Verb, safe_key};
use crate::core::text::ModelName;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
/// A refused portable call settings value or conflict.
pub enum SettingsError {
    #[error("{0}")]
    /// The JSON text could not be decoded.
    Json(String),
    #[error("the settings argument is one JSON object")]
    /// The top-level value is not an object.
    NotAnObject,
    #[error("the settings key `{}` does not exist", safe_key(.0))]
    /// The object names a key outside the closed schema.
    UnknownKey(String),
    #[error("the settings key `{}` does not belong to this verb", safe_key(.0))]
    /// A recognized key does not apply to this verb.
    WrongVerb(String),
    #[error("the settings hold both `{0}` and `{1}`")]
    /// Two distinct member fields were supplied.
    TwoMemberKeys(String, String),
    #[error("`context` is text that is not blank")]
    /// Shared context is blank or is not text.
    BlankContext,
    #[error("`batch` is `max` or a whole number of at least 1")]
    /// The batch setting is not a positive count or `max`.
    BadBatch,
    #[error("`deadline_ms` is a whole number of milliseconds")]
    /// The deadline is not an integral number of milliseconds.
    DeadlineNotWhole,
    #[error("`deadline_ms` is -1, 0, or at most 4294967295000 milliseconds")]
    /// The integral deadline is outside the accepted range.
    DeadlineOutOfRange,
    #[error("`none` is true or false")]
    /// The find-only no-match setting is not Boolean.
    BadNone,
    #[error("settings repeats `{}` from the question or named arguments", safe_key(.0))]
    /// A caller argument and settings supply the same question field.
    RepeatedField(String),
    #[error("a members argument and settings both name members")]
    /// Both an explicit members argument and settings supply members.
    TwoMembers,
    #[error("{0}")]
    /// The shared question grammar refused the fields.
    Question(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// The judgment verb whose question fields are being checked.
pub enum For {
    /// Binary decision.
    Decide,
    /// Choice among named options.
    Choose,
    /// Score against named levels.
    Score,
    /// Tag among named labels.
    Tag,
    /// Nearest-match search.
    Find,
}

impl For {
    const fn question_verb(self) -> Option<Verb> {
        match self {
            Self::Decide => Some(Verb::Decide),
            Self::Choose => Some(Verb::Choose),
            Self::Score => Some(Verb::Score),
            Self::Tag => Some(Verb::Tag),
            Self::Find => None,
        }
    }

    const fn member_key(self) -> Option<&'static str> {
        match self {
            Self::Decide | Self::Find => None,
            Self::Choose => Some("options"),
            Self::Score => Some("levels"),
            Self::Tag => Some("labels"),
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq)]
/// Validated portable settings for one call.
pub struct Settings {
    question_fields: Vec<(String, Json)>,
    source_keys: Vec<String>,
    pub(crate) context: Option<String>,
    pub(crate) batch: Option<Setting>,
    /// `-1` means none and `0` is already spent.
    pub(crate) deadline_ms: Option<i64>,
    pub(crate) none: Option<bool>,
}

impl fmt::Debug for Settings {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Settings")
            .field(
                "question_keys",
                &self
                    .question_fields
                    .iter()
                    .map(|(key, _)| key)
                    .collect::<Vec<_>>(),
            )
            .field("context", &self.context.is_some())
            .field("batch", &self.batch)
            .field("deadline_ms", &self.deadline_ms)
            .field("none", &self.none)
            .finish()
    }
}

impl Settings {
    /// Read a portable settings object, preserving the source order of member maps.
    pub fn parse(text: &str) -> Result<Self, SettingsError> {
        let Json::Object(fields) =
            Json::parse(text).map_err(|error| SettingsError::Json(error.to_string()))?
        else {
            return Err(SettingsError::NotAnObject);
        };
        let mut result = Self::default();
        let mut member_key = None;
        for (key, value) in fields {
            result.source_keys.push(key.clone());
            match key.as_str() {
                "threshold" | "true" | "false" | "options" | "levels" | "labels" | "model" => {
                    result.question_key(key, value, &mut member_key)?;
                }
                "context" => match value {
                    Json::String(text) if !text.trim().is_empty() => result.context = Some(text),
                    _ => return Err(SettingsError::BlankContext),
                },
                "batch" => {
                    result.batch = Some(Setting::of_json(&value).ok_or(SettingsError::BadBatch)?)
                }
                "deadline_ms" => {
                    result.deadline_ms = Some(Self::deadline(value)?);
                }
                "none" => {
                    result.none = Some(match value {
                        Json::Bool(value) => value,
                        _ => return Err(SettingsError::BadNone),
                    })
                }
                _ => return Err(SettingsError::UnknownKey(key)),
            }
        }
        Ok(result)
    }

    fn question_key(
        &mut self,
        key: String,
        value: Json,
        member_key: &mut Option<String>,
    ) -> Result<(), SettingsError> {
        if matches!(key.as_str(), "options" | "levels" | "labels")
            && let Some(first) = member_key.replace(key.clone())
        {
            return Err(SettingsError::TwoMemberKeys(first, key));
        }
        self.question_fields.push((key, value));
        Ok(())
    }

    fn deadline(value: Json) -> Result<i64, SettingsError> {
        match value {
            Json::Number(number) => {
                let value = number.as_i64().ok_or(SettingsError::DeadlineNotWhole)?;
                valid_deadline_ms(value)
                    .then_some(value)
                    .ok_or(SettingsError::DeadlineOutOfRange)
            }
            _ => Err(SettingsError::DeadlineNotWhole),
        }
    }

    /// Refuse keys this verb cannot take, then reuse the question-file grammar.
    pub fn check(&self, verb: For) -> Result<(), SettingsError> {
        if self.none.is_some() && verb != For::Find {
            return Err(SettingsError::WrongVerb("none".into()));
        }
        if verb == For::Find {
            return self.check_find();
        }
        for (key, _) in &self.question_fields {
            if matches!(key.as_str(), "options" | "levels" | "labels")
                && verb.member_key() != Some(key.as_str())
            {
                return Err(SettingsError::WrongVerb(key.clone()));
            }
        }
        let question = self.question_file(verb, Json::String("settings validation".into()))?;
        let _ = question;
        Ok(())
    }

    fn check_find(&self) -> Result<(), SettingsError> {
        for (key, value) in &self.question_fields {
            if key != "model" {
                return Err(SettingsError::WrongVerb(key.clone()));
            }
            let Some(text) = value.as_str() else {
                return Err(SettingsError::WrongVerb("model".into()));
            };
            ModelName::new(text).map_err(|_| SettingsError::WrongVerb("model".into()))?;
        }
        Ok(())
    }

    /// Reject a settings key repeated in an explicit question/parameter, and
    /// a members argument beside settings members. A NULL members argument
    /// passes `false` here and leaves the settings member intact.
    pub fn conflicts(
        &self,
        explicit_keys: &[&str],
        explicit_members: bool,
    ) -> Result<(), SettingsError> {
        for key in &self.source_keys {
            if explicit_keys.contains(&key.as_str()) {
                return Err(SettingsError::RepeatedField(key.clone()));
            }
            if explicit_members && matches!(key.as_str(), "options" | "levels" | "labels") {
                return Err(SettingsError::TwoMembers);
            }
        }
        Ok(())
    }

    /// Build the same validated question grammar with the caller's text.
    /// The host still applies call keys separately.
    pub(crate) fn question_file(
        &self,
        verb: For,
        text: Json,
    ) -> Result<QuestionFile, SettingsError> {
        let verb = verb
            .question_verb()
            .ok_or(SettingsError::WrongVerb("question".into()))?;
        let mut fields = Vec::with_capacity(self.question_fields.len() + 1);
        fields.push((verb.word().into(), text));
        fields.extend(self.question_fields.iter().cloned());
        QuestionFile::parsed(Json::Object(fields))
            .map_err(|error| SettingsError::Question(error.to_string()))
    }

    /// The question-file JSON made from the settled settings and one caller
    /// question. Hosts feed this to the existing public question constructor.
    pub fn question_json(&self, verb: For, text: &str) -> Result<String, SettingsError> {
        self.check(verb)?;
        let verb = verb
            .question_verb()
            .ok_or(SettingsError::WrongVerb("question".into()))?;
        let mut fields = Vec::with_capacity(self.question_fields.len() + 1);
        fields.push((verb.word().into(), Json::String(text.into())));
        fields.extend(self.question_fields.iter().cloned());
        serde_json::to_string(&Json::Object(fields)).map_err(|_| {
            SettingsError::Question("the settings could not be written as JSON".into())
        })
    }

    /// Literal shared context, absent when the caller named none.
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    /// Milliseconds at the numeric host boundary; `-1` means none and `0`
    /// means spent. Absence leaves the engine's call option unchanged.
    pub const fn deadline_ms(&self) -> Option<i64> {
        self.deadline_ms
    }

    /// Whether the caller requested the fill-to-limit batch setting.
    pub const fn batch_max(&self) -> bool {
        matches!(self.batch, Some(Setting::Max))
    }

    /// The explicit record cap of a batch, absent for `max` or no setting.
    pub const fn batch_records(&self) -> Option<usize> {
        match self.batch {
            Some(Setting::Records(count)) => Some(count.get()),
            _ => None,
        }
    }

    /// Find's explicit no-match option, absent on other verbs.
    pub const fn none(&self) -> Option<bool> {
        self.none
    }
}

/// The accepted numeric host boundary, before any host converts the value to a clock.
pub(crate) const MAX_DEADLINE_MS: i64 = 4_294_967_295_000;

pub(crate) const fn valid_deadline_ms(value: i64) -> bool {
    value == -1 || (value >= 0 && value <= MAX_DEADLINE_MS)
}

/// Validate the engine constructor's closed JSON schema before any host
/// converts its values or captures an environment. In particular, this reader
/// retains duplicate names, which a generic JSON map would silently lose.
pub(crate) fn engine_settings(text: &str) -> Result<(), String> {
    let parsed = Json::parse(text).map_err(|error| match error {
        JsonError::Syntax { .. } => "settings JSON is one object".to_owned(),
        JsonError::DuplicateName { .. } => "settings JSON repeats a member name".to_owned(),
        JsonError::NotFinite => error.to_string(),
    })?;
    let Json::Object(fields) = parsed else {
        return Err("settings JSON is one object".into());
    };
    for (key, value) in fields {
        let number = || match &value {
            Json::Number(number) => number.as_u64(),
            _ => None,
        };
        let string = || matches!(value, Json::String(_));
        let valid = match key.as_str() {
            "base_url" | "model" | "record" | "replay" | "profile" => string(),
            "throttle" => number().is_some_and(|count| (1..=32).contains(&count)),
            "max_requests" => {
                matches!(value, Json::Null) || number().is_some_and(|count| count > 0)
            }
            "max_request_bytes" => number().is_some_and(|count| count > 0),
            "cache" => matches!(value, Json::Bool(false)) || string(),
            "timeout" => number().is_some(),
            "max_retries" => number().is_some_and(|count| u32::try_from(count).is_ok()),
            "batch" => Setting::of_json(&value).is_some(),
            "max_requests_total" | "max_estimated_input_tokens_total" => {
                matches!(value, Json::Null) || number().is_some()
            }
            _ => return Err(format!("settings JSON has unknown key {}", safe_key(&key))),
        };
        if !valid {
            return Err(format!("settings {key} has an invalid value"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{For, Settings, SettingsError, engine_settings};
    use crate::core::{Json, Setting, Threshold, Typed, Verb, resolve};
    use std::num::NonZeroUsize;

    #[test]
    fn accepted_settings_share_question_file_grammar() {
        let cases = [
            ("{}", For::Decide),
            (r#"{"threshold":0.7}"#, For::Decide),
            (r#"{"threshold":"0.3:0.7"}"#, For::Decide),
            (r#"{"true":"A yes means a refund request"}"#, For::Decide),
            (r#"{"true":{"meaning":"refund"}}"#, For::Decide),
            (r#"{"options":["billing","shipping"]}"#, For::Choose),
            (
                r#"{"options":{"billing":"Payments","shipping":"Parcels"}}"#,
                For::Choose,
            ),
            (r#"{"model":"jev-1.13.0"}"#, For::Decide),
            (r#"{"context":"One shared reference text"}"#, For::Decide),
            (r#"{"batch":"max"}"#, For::Decide),
            (r#"{"batch":64}"#, For::Decide),
            (r#"{"deadline_ms":5000}"#, For::Decide),
            (r#"{"none":true}"#, For::Find),
        ];
        for (object, verb) in cases {
            Settings::parse(object)
                .and_then(|settings| settings.check(verb))
                .expect(object);
        }
        let empty = Settings::parse("{}").expect("empty");
        assert_eq!(empty, Settings::default());
        assert_eq!(
            Settings::parse(r#"{"batch":64}"#).expect("batch").batch,
            Some(Setting::Records(NonZeroUsize::new(64).expect("positive")))
        );
        let band = Settings::parse(r#"{"threshold":"0.3:0.7"}"#).expect("band");
        let file = band
            .question_file(For::Decide, Json::String("q".into()))
            .expect("same grammar");
        let settled =
            resolve(Verb::Decide, None, Some(&file), &Typed::default()).expect("resolved question");
        assert_eq!(settled.threshold(), Threshold::band(0.3, 0.7).ok());
        let call = Settings::parse(r#"{"true":"A yes means a refund request","model":"jev-1.13.0","context":"One shared reference text","batch":"max","deadline_ms":5000}"#).expect("shared settings");
        assert_eq!(call.question_json(For::Decide, "Refund?"), Ok(r#"{"decide":"Refund?","true":"A yes means a refund request","model":"jev-1.13.0"}"#.into()));
        assert_eq!(
            (
                call.context(),
                call.batch_max(),
                call.batch_records(),
                call.deadline_ms(),
                call.none()
            ),
            (
                Some("One shared reference text"),
                true,
                None,
                Some(5000),
                None
            )
        );
    }

    #[test]
    fn invalid_corpus_and_cross_argument_conflicts_refuse() {
        let invalid = [
            (r#"{"options":["a"],"labels":["b"]}"#, For::Choose),
            (r#"{"threshold":1.5}"#, For::Decide),
            (r#"{"threshold":"0.7:0.3"}"#, For::Decide),
            (r#"{"context":"  "}"#, For::Decide),
            (r#"{"deadline_ms":1.5}"#, For::Decide),
            (r#"{"unknwon":1}"#, For::Decide),
            (r#"{"none":true}"#, For::Decide),
            ("[1]", For::Decide),
            ("nope", For::Decide),
        ];
        for (object, verb) in invalid {
            assert!(
                Settings::parse(object)
                    .and_then(|settings| settings.check(verb))
                    .is_err(),
                "{object}"
            );
        }
        let repeated = Settings::parse(r#"{"threshold":0.7}"#).expect("parsed");
        assert_eq!(
            repeated.conflicts(&["threshold"], false),
            Err(SettingsError::RepeatedField("threshold".into()))
        );
        let members = Settings::parse(r#"{"options":["a","b"]}"#).expect("parsed");
        assert_eq!(members.conflicts(&[], true), Err(SettingsError::TwoMembers));
        members
            .conflicts(&[], false)
            .expect("NULL members are absent");
        // V10's boundary is settled before any host can create a deadline.
        for value in [-1, 0, 4_294_967_295_000_i64] {
            let text = format!(r#"{{"deadline_ms":{value}}}"#);
            Settings::parse(&text)
                .and_then(|settings| settings.check(For::Decide))
                .expect(&text);
        }
        for value in [-2, 4_294_967_295_001_i64] {
            let text = format!(r#"{{"deadline_ms":{value}}}"#);
            assert_eq!(
                Settings::parse(&text),
                Err(SettingsError::DeadlineOutOfRange)
            );
        }
    }

    #[test]
    fn constructor_schema_keeps_duplicate_names_and_accepts_the_active_cap() {
        for (text, reason) in [
            (r#"{"timeout":1,"timeout":2}"#, "repeats"),
            (r#"{"max_requests_total":-1}"#, "max_requests_total"),
            (r#"{"batch":0}"#, "batch"),
            (r#"{"api_key":"secret"}"#, "api_key"),
        ] {
            assert!(engine_settings(text).expect_err(text).contains(reason));
        }
        for text in [
            "{}",
            r#"{"batch":"max","timeout":30,"cache":false}"#,
            r#"{"max_requests_total":0}"#,
            r#"{"max_requests_total":1}"#,
            r#"{"max_requests_total":null}"#,
        ] {
            engine_settings(text).expect(text);
        }
    }
}
