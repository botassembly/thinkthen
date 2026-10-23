//! `tag`: every label whose probability reaches the cut.

use std::collections::BTreeMap;

use crate::core::{Answer, Ask, Detail, Error, ErrorKind, Function, Outcome, Threshold, Verdict};

/// A question with independent labels and a single threshold cut.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Tag {
    text: String,
    labels: Vec<String>,
    threshold: Threshold,
}

impl Tag {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::Tags {
            labels: &self.labels,
            question: &self.text,
            threshold: self.threshold.canonical(),
        }
    }

    /// Replace the labels, keeping the question's other settings.
    ///
    /// # Errors
    /// Returns `usage.question` when the replacement labels are invalid.
    pub fn labels(self, labels: Vec<String>) -> Result<Self, Error> {
        let replacement = Self::new(self.text, labels)?;
        Ok(Self {
            threshold: self.threshold,
            ..replacement
        })
    }

    /// Ask `text` about each of the distinct, nonblank `labels`.
    ///
    /// # Errors
    ///
    /// `usage.question` when the question is blank or the labels are empty,
    /// blank, repeated or containing U+0000.
    pub fn new(
        text: impl Into<String>,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        let text = text.into();
        let labels: Vec<String> = labels.into_iter().map(Into::into).collect();
        if text.trim().is_empty() {
            return Err(Error::new(ErrorKind::Question, "the question is blank"));
        }
        for label in &labels {
            crate::core::ask::admit_name(label)?;
        }
        let mut seen = std::collections::HashSet::new();
        if labels.is_empty()
            || labels
                .iter()
                .any(|label| label.trim().is_empty() || !seen.insert(label))
        {
            return Err(Error::new(
                ErrorKind::Question,
                "labels must be nonblank, distinct and include at least one label",
            ));
        }
        Ok(Self {
            text,
            labels,
            threshold: Threshold::default(),
        })
    }

    /// Read each label's probability at this single cut.
    ///
    /// # Errors
    ///
    /// `usage.threshold` when `threshold` is a band.
    pub fn threshold(self, threshold: Threshold) -> Result<Self, Error> {
        Ok(Self {
            threshold: threshold.require_cut()?,
            ..self
        })
    }
}

impl Function for Tag {
    const NAME: &'static str = "tag";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        _limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let evidence = record.evidence();
        Ok(self
            .labels
            .iter()
            .map(|label| Ask::YesNo {
                instructions: super::quoted(
                    evidence,
                    &format!(
                        "{} Does the label {} apply?",
                        self.text,
                        serde_json::to_string(label).unwrap_or_default()
                    ),
                ),
                yes: None,
                no: None,
            })
            .collect())
    }

    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        let answers = context.answers();
        let mut labels = Vec::new();
        let mut detail = BTreeMap::new();
        for (label, answer) in question.labels.iter().zip(answers) {
            let p = match answer {
                Ok(answer) => answer.yes(),
                Err(error) => {
                    return Outcome::Failed {
                        error: error.clone(),
                    };
                }
            };
            if question.threshold.verdict(p) == Verdict::Yes {
                labels.push(label.clone());
            }
            detail.insert(label.clone(), p);
        }
        Outcome::Answered {
            answer: Answer::Labels(labels),
            p: None,
            detail: Some(Detail::Labels(detail)),
        }
    }
}
