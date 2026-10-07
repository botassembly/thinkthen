//! Native preparation, lossless author serialization and selected record admission.
use crate::core::{self, Json};
use crate::public::{
    Error, LoadedQuestion, Question, QuestionInput, RawRecord, RecordReading, SourceLocation,
};
fn encoded<T: serde::Serialize>(value: &T) -> Result<Json, Error> {
    let text = core::json_line(value)
        .map_err(|_| Error::defect("question preparation could not be written"))?;
    Json::parse(&text).map_err(|_| Error::defect("question preparation could not be read"))
}
impl Question {
    pub(crate) fn reading_metadata(&self) -> core::declaration::QuestionMetadata {
        let mut metadata = self.metadata.clone();
        metadata.reading.model = self.model.as_ref().map(|value| value.as_str().to_owned());
        metadata.reading.profile = self.profile.as_ref().map(|value| value.as_str().to_owned());
        metadata.reading.batch = self.batch.as_ref().and_then(core::Setting::of_json);
        metadata
    }
    /// Serialize native preparation in the ordinary single-question file grammar.
    /// Author declarations, descriptions, reading and explicitly selected route settings remain intact.
    /// # Errors
    /// Returns Usage for partly described score levels that the file grammar cannot represent,
    /// or Defect when validated native preparation cannot be encoded.
    pub fn to_json(&self) -> Result<String, Error> {
        let Json::Object(mut fields) = encoded(&self.core)? else {
            return Err(Error::defect("a question is not an object"));
        };
        let mut verb = fields
            .iter()
            .find(|(key, _)| key == "verb")
            .and_then(|(_, value)| value.as_str())
            .ok_or_else(|| Error::defect("a question lost its verb"))?
            .to_owned();
        if matches!(
            self.kind,
            super::question::Kind::Find | super::question::Kind::FindNone
        ) {
            verb = "find".to_owned();
            fields.push((
                "none".to_owned(),
                Json::Bool(self.kind == super::question::Kind::FindNone),
            ));
        }
        fields.retain(|(key, _)| key != "verb");
        for (key, _) in &mut fields {
            if key == "text" {
                *key = verb.clone();
            }
        }
        let labels = match &self.core {
            core::Question::Choose { options, .. } => Some(("options", options)),
            core::Question::Tag { labels, .. } => Some(("labels", labels)),
            core::Question::Score { levels, .. } => Some(("levels", levels)),
            core::Question::Decide { .. } => None,
        };
        if let Some(("levels", labels)) = labels
            && !labels.fully_described()
            && labels
                .descriptions()
                .any(|(_, description)| description.is_some())
        {
            return Err(Error::usage(
                "partly described score levels cannot be written as a question file",
            ));
        }
        if let Some((key, labels)) = labels
            && labels
                .descriptions()
                .any(|(_, description)| description.is_some())
        {
            let described = descriptions(labels);
            if let Some((_, value)) = fields.iter_mut().find(|(name, _)| name == key) {
                *value = described;
            }
        }
        if let Some(value) = self.threshold {
            fields.push(("threshold".to_owned(), encoded(&value)?));
        }
        if let Some(value) = &self.model {
            fields.push(("model".to_owned(), Json::String(value.as_str().to_owned())));
        }
        if let Some(value) = &self.profile {
            fields.push((
                "profile".to_owned(),
                Json::String(value.as_str().to_owned()),
            ));
        }
        if let Some(value) = &self.batch {
            fields.push(("batch".to_owned(), value.clone()));
        }
        if !self.metadata.reading.on.is_empty() {
            fields.push(("on".to_owned(), encoded(&self.metadata.reading.on)?));
        }
        if let Json::Object(metadata) = encoded(&self.metadata)? {
            fields.extend(metadata);
        }
        core::json_line(&Json::Object(fields))
            .map_err(|_| Error::defect("question preparation could not be written"))
    }
    pub(crate) fn selected_input(&self, input: &QuestionInput) -> Result<QuestionInput, Error> {
        let on = &self.metadata.reading.on;
        if !on.iter().any(|pointer| !pointer.is_empty()) {
            return Ok(input.clone());
        }
        let original = match input {
            QuestionInput::Record(record) if record.images().is_empty() => {
                record.original().clone()
            }
            QuestionInput::Text(text) => RawRecord::text(text)?,
            _ => {
                return Err(Error::usage(
                    "authored on pointers require a text or JSON record",
                ));
            }
        };
        let fields = on.iter().map(String::as_str).collect::<Vec<_>>();
        let mut selected = RecordReading::new(&fields, None, None)?
            .compose(original)?
            .original;
        if let QuestionInput::Record(record) = input
            && let Some(location) = record.location()
        {
            selected = selected.with_location(location.clone());
        }
        Ok(QuestionInput::Record(selected))
    }
    pub(crate) fn admit_input(&self, input: &QuestionInput) -> Result<(), Error> {
        self.metadata.validate_item(&self.selected_input(input)?)
    }
}
impl LoadedQuestion {
    /// Preserve the complete loaded preparation, including a banded reading.
    /// # Errors
    /// As Question::to_json.
    pub fn to_json(&self) -> Result<String, Error> {
        match self {
            Self::Question(question) => question.to_json(),
            Self::Banded(question) => question.0.to_json(),
        }
    }
}
impl QuestionInput {
    /// Read an annotation document structurally when valid JSON, otherwise as literal text.
    /// Physical location stays separate from every model request and identity.
    /// # Errors
    /// Returns Usage for the existing native document bounds or invalid blank evidence.
    pub fn annotation_text(text: &str, location: SourceLocation) -> Result<Self, Error> {
        Ok(Self::Record(
            annotation_record(text)?.with_location(location),
        ))
    }
    /// Read an annotation document without inventing a physical source location.
    /// Valid JSON stays structural; syntax-invalid document text stays literal.
    /// # Errors
    /// Preserves native duplicate, depth, size and blank-evidence refusals.
    pub fn annotation_document(text: &str) -> Result<Self, Error> {
        Ok(Self::Record(annotation_record(text)?))
    }
}

fn annotation_record(text: &str) -> Result<super::RecordEvidence, Error> {
    let reading =
        core::Reading::new(core::Framing::Document, Vec::new()).map_err(Error::refused)?;
    let original = RawRecord(std::sync::Arc::new(
        reading
            .annotation_record(text.as_bytes())
            .map_err(Error::refused)?,
    ));
    let selected = RecordReading::new(&[], None, None)?
        .compose(original)?
        .original;
    Ok(selected)
}
fn descriptions(labels: &core::Labels) -> Json {
    Json::Object(
        labels
            .descriptions()
            .map(|(name, description)| {
                (
                    name.clone(),
                    description.map_or(Json::Null, |value| value.as_json().clone()),
                )
            })
            .collect(),
    )
}
