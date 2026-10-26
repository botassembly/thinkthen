//! `decide`: yes, no or not sure.

use crate::core::ask::Ask;
use crate::core::error::{Error, ErrorKind};
use crate::core::row::{Answer, Outcome};
use crate::core::threshold::{Threshold, Verdict};

/// A yes or no question and the threshold that reads its answer.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub(crate) struct Decide {
    pub(crate) text: String,
    yes: Option<String>,
    no: Option<String>,
    pub(crate) threshold: Threshold,
}

impl Decide {
    /// The question `text`, read at the default threshold.
    ///
    /// # Errors
    ///
    /// `usage.question` when `text` is blank.
    pub(crate) fn new(text: impl Into<String>) -> Result<Self, Error> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement("question", "nonblank text", "blank text"),
            ));
        }
        Ok(Self {
            text,
            yes: None,
            no: None,
            threshold: Threshold::default(),
        })
    }
}

impl super::Function for Decide {
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
