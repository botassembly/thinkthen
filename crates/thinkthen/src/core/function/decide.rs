//! `decide`: yes, no or not sure.

use crate::core::ask::Ask;
use crate::core::error::{Error, ErrorKind};
use crate::core::row::{Answer, Outcome};
use crate::core::threshold::{Threshold, Verdict};

/// A yes or no question and the threshold that reads its answer.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Decide {
    text: String,
    yes: Option<String>,
    no: Option<String>,
    threshold: Threshold,
}

impl Decide {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::YesNo {
            no: self.no.as_deref(),
            question: &self.text,
            threshold: self.threshold.canonical(),
            yes: self.yes.as_deref(),
        }
    }

    /// The question `text`, read at the default threshold.
    ///
    /// # Errors
    ///
    /// `usage.question` when `text` is blank.
    pub fn new(text: impl Into<String>) -> Result<Self, Error> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(Error::new(ErrorKind::Question, "the question is blank"));
        }
        Ok(Self {
            text,
            yes: None,
            no: None,
            threshold: Threshold::default(),
        })
    }

    /// This question read at `threshold`.
    #[must_use]
    pub fn threshold(self, threshold: Threshold) -> Self {
        Self { threshold, ..self }
    }

    /// This question with `description` saying what a yes means.
    #[must_use]
    pub fn yes(self, description: impl Into<String>) -> Self {
        Self {
            yes: Some(description.into()),
            ..self
        }
    }

    /// This question with `description` saying what a no means.
    #[must_use]
    pub fn no(self, description: impl Into<String>) -> Self {
        Self {
            no: Some(description.into()),
            ..self
        }
    }
}

impl super::Function for Decide {
    const NAME: &'static str = "decide";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
    /// The ask this question makes about `evidence`.
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        _limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let evidence = record.evidence();
        Ok(vec![Ask::YesNo {
            instructions: super::quoted(evidence, &self.text),
            yes: self.yes.clone(),
            no: self.no.clone(),
        }])
    }

    /// The outcome of the model's answer to this question's ask.
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        let answers = context.answers();
        match super::single(answers) {
            Ok(answer) => {
                let p = answer.yes();
                match question.threshold.verdict(p) {
                    Verdict::Yes => Outcome::Answered {
                        answer: Answer::Bool(true),
                        p: Some(p),
                        detail: None,
                    },
                    Verdict::No => Outcome::Answered {
                        answer: Answer::Bool(false),
                        p: Some(p),
                        detail: None,
                    },
                    Verdict::Unsure => Outcome::Unsure { p, detail: None },
                }
            }
            Err(error) => Outcome::Failed { error },
        }
    }
}
