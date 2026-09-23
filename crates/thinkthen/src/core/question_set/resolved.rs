use serde::Serialize;
use serde::ser::{SerializeMap, Serializer};

use super::{NamedQuestion, QuestionSet};
use crate::core::question::{Labels, Question};
use crate::core::render::{RenderError, json_line};

pub(super) fn json(set: &QuestionSet) -> Result<String, RenderError> {
    json_line(&ResolvedSet(set))
}

struct ResolvedSet<'a>(&'a QuestionSet);

impl Serialize for ResolvedSet<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("version", &1)?;
        if let Some(profile) = &self.0.profile {
            map.serialize_entry("profile", profile.as_str())?;
        }
        map.serialize_entry("questions", &ResolvedQuestions(&self.0.questions))?;
        map.end()
    }
}

struct ResolvedQuestions<'a>(&'a [NamedQuestion]);

impl Serialize for ResolvedQuestions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for question in self.0 {
            map.serialize_entry(question.name(), &ResolvedQuestion(question))?;
        }
        map.end()
    }
}

struct ResolvedQuestion<'a>(&'a NamedQuestion);

impl Serialize for ResolvedQuestion<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let question = self.0;
        let mut map = serializer.serialize_map(None)?;
        match question.question() {
            Question::Decide { text, yes, no } => {
                map.serialize_entry("decide", text.as_json())?;
                if let Some(yes) = yes {
                    map.serialize_entry("true", yes.as_json())?;
                }
                if let Some(no) = no {
                    map.serialize_entry("false", no.as_json())?;
                }
            }
            Question::Choose { text, options } => {
                map.serialize_entry("choose", text.as_json())?;
                map.serialize_entry("options", &ResolvedDescriptions(options))?;
            }
            Question::Tag { text, labels } => {
                map.serialize_entry("tag", text.as_json())?;
                map.serialize_entry("labels", &ResolvedDescriptions(labels))?;
            }
            Question::Score { text, levels } => {
                map.serialize_entry("score", text.as_json())?;
                if levels.fully_described() {
                    map.serialize_entry("levels", &ResolvedDescriptions(levels))?;
                } else {
                    map.serialize_entry("levels", levels)?;
                }
            }
        }
        if let Some(threshold) = question.threshold() {
            map.serialize_entry("threshold", &threshold)?;
        }
        map.serialize_entry("on", question.on())?;
        map.end()
    }
}

struct ResolvedDescriptions<'a>(&'a Labels);

impl Serialize for ResolvedDescriptions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self.0.descriptions())
    }
}
