//! Recording metadata is private to the unexposed store.
use crate::Error;
use serde_json::{Value, json};
use thinkthen::{Question, Request};

pub(crate) struct QuestionDefinition {
    question: Question,
    function: String,
    domain: Value,
    wording: Value,
}
impl QuestionDefinition {
    pub(crate) fn admit(request: &str) -> Result<Self, Error> {
        let request = Request::from_json(request).map_err(|_| Error::Question)?;
        if !request.records().is_empty()
            || request.question().function() == thinkthen::FunctionName::Annotate
        {
            return Err(Error::Question);
        }
        let question = request.question().clone();
        let mut wording = serde_json::to_value(&question)?;
        let fields = wording.as_object_mut().ok_or(Error::Question)?;
        let function = fields
            .remove("function")
            .and_then(|v| v.as_str().map(str::to_owned))
            .ok_or(Error::Question)?;
        fields.remove("threshold");
        fields.remove("evidence");
        fields.remove("top");
        let mut domain = serde_json::Map::new();
        for key in ["choices", "labels", "levels", "kinds", "rules"] {
            if let Some(value) = fields.get(key) {
                let mut value = value.clone();
                if let Some(values) = value.as_array_mut() {
                    if key == "rules" {
                        for value in values.iter_mut() {
                            if let Some(rule) = value.as_object_mut() {
                                rule.remove("reads");
                            }
                        }
                    }
                    if key != "levels" {
                        values.sort_by_key(Value::to_string);
                    }
                }
                domain.insert(key.to_owned(), value);
            }
        }
        Ok(Self {
            question,
            function,
            domain: json!(domain),
            wording,
        })
    }
    pub(crate) fn function(&self) -> &str {
        &self.function
    }
    pub(crate) fn domain(&self) -> &Value {
        &self.domain
    }
    pub(crate) fn wording(&self) -> &Value {
        &self.wording
    }
    pub(crate) fn validate_answer(&self, answer: &Value, evidence: &str) -> Result<(), Error> {
        thinkthen::native::validate_recording_answer(
            &self.question,
            &serde_json::to_string(answer)?,
            evidence,
        )
        .map_err(|_| Error::Question)
    }
}
