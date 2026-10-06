//! Saved whole-set find readings use the ordinary ordered parser and field validators.
use super::{Json, ModelName, Pointer, ProfileName, QuestionFile, QuestionFileError, QuestionText};

pub(crate) struct FindFile {
    pub(crate) text: QuestionText,
    pub(crate) on: Vec<Pointer>,
    pub(crate) model: Option<ModelName>,
    pub(crate) profile: Option<ProfileName>,
}
impl QuestionFile {
    pub(crate) fn parse_find(text: &str) -> Result<FindFile, QuestionFileError> {
        let value = Json::parse(text)?;
        let Json::Object(members) = &value else {
            return Err(QuestionFileError::NotAnObject);
        };
        for (key, _) in members {
            if !["find", "on", "model", "profile"].contains(&key.as_str()) {
                return Err(QuestionFileError::UnknownKey(key.clone()));
            }
        }
        Ok(FindFile {
            text: super::fields::question_text(&value, "find")?,
            on: super::fields::pointers_in(&value)?.unwrap_or_default(),
            model: super::fields::model_in(&value)?,
            profile: super::profile::profile_in(&value)?,
        })
    }
}
