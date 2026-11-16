//! Saved question toolboxes share the command's entries envelope.
use super::collection::RawNamed;
use super::{Error, RawSimpleQuestion, SimpleQuestion, TOOLBOX_LIMIT, Toolbox, question_error};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(super) struct RawSavedToolbox {
    #[serde(deserialize_with = "crate::core::diagnostic::entries")]
    entries: RawNamed<RawSavedQuestion>,
}
#[derive(Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct RawSavedQuestion {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: RawSimpleQuestion,
}
#[derive(Serialize)]
struct SavedQuestions<'a> {
    entries: BTreeMap<&'a str, SavedQuestion<'a>>,
}
#[derive(Serialize)]
struct SavedQuestion<'a> {
    question: &'a SimpleQuestion,
}
impl Toolbox {
    /// Load named questions in the command's saved toolbox format.
    /// # Errors
    /// Returns the shared usage error for invalid JSON, names or questions.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        if json.len() > TOOLBOX_LIMIT {
            return Err(question_error(crate::core::diagnostic::requirement(
                "toolbox",
                "JSON within the toolbox byte limit",
                "oversized JSON",
            )));
        }
        let saved: RawSavedToolbox = serde_json::from_str(json).map_err(|error| {
            question_error(crate::core::diagnostic::context(
                "toolbox",
                &error.to_string(),
            ))
        })?;
        saved.try_into()
    }
    /// Save named questions for another SDK or the command's run operation.
    /// # Errors
    /// Returns the shared usage error for an unsavable name or oversized toolbox.
    pub fn to_json(&self) -> Result<String, Error> {
        let json = serde_json::to_string(&saved_questions(self)?).map_err(|error| {
            question_error(crate::core::diagnostic::context(
                "toolbox",
                &error.to_string(),
            ))
        })?;
        if json.len() > TOOLBOX_LIMIT {
            return Err(question_error(crate::core::diagnostic::requirement(
                "toolbox",
                "JSON within the toolbox byte limit",
                "oversized JSON",
            )));
        }
        Ok(json)
    }
    /// Export flat admitted question definitions for native host hydration.
    /// # Errors
    /// Returns the shared question error if JSON cannot be serialized.
    #[doc(hidden)]
    pub fn definitions_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self.iter().collect::<BTreeMap<_, _>>()).map_err(|error| {
            question_error(crate::core::diagnostic::context(
                "toolbox",
                &error.to_string(),
            ))
        })
    }
}
impl Serialize for Toolbox {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        saved_questions(self)
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Toolbox {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RawSavedToolbox::deserialize(deserializer)?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}
impl TryFrom<RawSavedToolbox> for Toolbox {
    type Error = Error;
    fn try_from(saved: RawSavedToolbox) -> Result<Self, Error> {
        let questions = saved
            .entries
            .0
            .into_iter()
            .map(|(name, entry)| {
                validate_saved_name(&name)?;
                Ok((name, entry.question.try_into()?))
            })
            .collect::<Result<_, Error>>()?;
        Self::new(questions)
    }
}
pub(crate) fn validate_saved_name(name: &str) -> Result<(), Error> {
    if name.is_empty() || name.contains(['/', '\\', ':', '\0']) || matches!(name, "." | "..") {
        return Err(Error::new(
            super::ErrorKind::Name,
            crate::core::diagnostic::requirement(
                "name",
                "one filename without path separators",
                "an invalid entry name (contents redacted)",
            ),
        ));
    }
    Ok(())
}

fn saved_questions(toolbox: &Toolbox) -> Result<SavedQuestions<'_>, Error> {
    for (name, _) in toolbox.iter() {
        validate_saved_name(name)?;
    }
    Ok(SavedQuestions {
        entries: toolbox
            .iter()
            .map(|(name, question)| (name, SavedQuestion { question }))
            .collect(),
    })
}
