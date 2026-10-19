//! Command setup observations over an already admitted authored question set.
use crate::{BatchSetting, InputDeclaration, QuestionSet};
use crate::failure::Failure;
use crate::saved_json::{Json, Kind};

pub(super) struct Settings {
    pub(super) profile: Option<String>,
    pub(super) batch: Option<BatchSetting>,
    pub(super) on: Vec<(String, Vec<String>)>,
    pub(super) context_schema: Option<InputDeclaration>,
}
impl Settings {
    pub(super) fn of(set: &QuestionSet, text: &str) -> Result<Self, Failure> {
        let document = Json::parse(text)?;
        let questions = document.member("questions")
            .ok_or(Failure::Defect("admitted annotation lost its questions"))?;
        let on = set.members().map(|(name, _)| {
            let pointers = match questions.member(name).and_then(|question| question.member("on")).map(Json::kind) {
                Some(Kind::String(pointer)) => vec![pointer.clone()],
                Some(Kind::Array(pointers)) => pointers.iter().filter_map(Json::as_str).map(str::to_owned).collect(),
                _ => Vec::new(),
            };
            (name.to_owned(), pointers)
        }).collect();
        let context_schema = match questions.kind() {
            Kind::Object(questions) => questions.first().map(|(_, question)| {
                crate::Question::from_json(&crate::render::json_line(question)?)
                    .map(|question| question.context_schema().cloned()).map_err(Failure::from)
            }).transpose()?.flatten(),
            _ => None,
        };
        let batch = document.member("batch").map(|value| {
            let setting: crate::RequestBatch = serde_json::from_str(&crate::render::json_line(value)?)
                .map_err(|_| Failure::Defect("admitted annotation batch lost its shape"))?;
            setting.native().map_err(Failure::from)
        }).transpose()?;
        Ok(Self {
            profile: document.member("profile").and_then(Json::as_str).map(str::to_owned),
            batch, on, context_schema,
        })
    }
    pub(super) fn first_part(&self) -> Option<&str> {
        self.on.iter().find(|(_, pointers)| pointers.iter().any(|pointer| !pointer.is_empty()))
            .map(|(name, _)| name.as_str())
    }
}
