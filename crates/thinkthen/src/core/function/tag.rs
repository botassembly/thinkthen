//! `tag`: every label whose probability reaches the cut.

use std::collections::BTreeMap;

use crate::core::{Answer, Ask, Detail, Error, ErrorKind, Function, Outcome, Threshold, Verdict};

/// A question with independent labels and a single threshold cut.
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Tag {
    pub(crate) text: String,
    pub(crate) labels: Vec<String>,
    pub(crate) threshold: Threshold,
}

impl Tag {
    /// Ask `text` about each of the distinct, nonblank `labels`.
    ///
    /// # Errors
    ///
    /// `usage.question` when the question is blank or the labels are empty,
    /// blank, repeated or containing U+0000.
    pub(crate) fn new(
        text: impl Into<String>,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        let text = text.into();
        let labels: Vec<String> = labels.into_iter().map(Into::into).collect();
        if text.trim().is_empty() {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement("question", "nonblank text", "blank text"),
            ));
        }
        for label in &labels {
            crate::core::ask::admit_name(label)?;
        }
        if labels.len() < crate::core::questions::MIN_LABELS
            || labels.iter().any(|label| label.trim().is_empty())
            || !crate::core::ask::distinct_labels(labels.iter().map(String::as_str))
        {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement(
                    "labels",
                    "a nonempty domain of distinct nonblank labels",
                    "an empty or invalid label domain (contents redacted)",
                ),
            ));
        }
        Ok(Self {
            text,
            labels,
            threshold: Threshold::default(),
        })
    }
}

impl Function for Tag {
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
