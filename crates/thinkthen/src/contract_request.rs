//! Strict requests carry an admitted question separately from client settings.
use crate::{Error, ErrorKind, Question};
use serde_json::Value;

/// A canonical question and its initial input records.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(try_from = "Input")]
#[schemars(with = "Input")]
pub struct Request {
    question: Question,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default)]
    records: Vec<Value>,
}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
struct Input {
    #[serde(deserialize_with = "crate::core::diagnostic::question")]
    question: crate::core::questions::RawQuestion,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::name")]
    name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "crate::core::diagnostic::records")]
    records: Vec<Value>,
}
impl Request {
    /// Start a request with a typed question and no initial records.
    #[must_use]
    pub fn new(question: impl Into<Question>) -> Self {
        Self {
            question: question.into(),
            name: None,
            records: Vec::new(),
        }
    }
    /// Parse and admit a request without reading files, settings or credentials.
    /// # Errors
    /// Returns the shared question, domain, evidence or name admission error.
    pub fn from_json(json: &str) -> Result<Self, Error> {
        serde_json::from_str::<Input>(json)
            .map_err(|error| {
                Error::new(
                    ErrorKind::Question,
                    crate::core::diagnostic::context("request", &error.to_string()),
                )
            })?
            .try_into()
    }
    /// Set a validated execution name without resolving a reference.
    /// # Errors
    /// Returns a name error for invalid text.
    pub fn named(mut self, name: impl Into<String>) -> Result<Self, Error> {
        let name = name.into();
        crate::core::ask::admit_name(&name)?;
        if name.trim().is_empty() {
            return Err(Error::new(
                ErrorKind::Name,
                crate::core::diagnostic::requirement(
                    "name",
                    "a nonblank execution name",
                    "blank text",
                ),
            ));
        }
        self.name = Some(name);
        Ok(self)
    }
    /// Supply initial JSON records; record admission belongs to execution.
    #[must_use]
    pub fn with_records(mut self, records: impl IntoIterator<Item = Value>) -> Self {
        self.records = records.into_iter().collect();
        self
    }
    /// The admitted function and its options.
    #[must_use]
    pub const fn question(&self) -> &Question {
        &self.question
    }
    /// The optional execution name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    /// Initial records in input order.
    #[must_use]
    pub fn records(&self) -> &[Value] {
        &self.records
    }
}
impl TryFrom<Input> for Request {
    type Error = Error;
    fn try_from(value: Input) -> Result<Self, Error> {
        let request = Self::new(Question::try_from(value.question)?).with_records(value.records);
        match value.name {
            Some(name) => request.named(name),
            None => Ok(request),
        }
    }
}
