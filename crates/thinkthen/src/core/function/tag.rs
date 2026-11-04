//! `tag`: every label whose probability reaches the cut.

use std::collections::BTreeMap;

use crate::core::{Answer, Ask, Detail, Error, ErrorKind, Function, Outcome, Threshold, Verdict};

/// A question with independent labels and a single threshold cut.
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct Tag {
    pub(crate) text: String,
    pub(crate) labels: Vec<crate::core::ChoiceOption>,
    pub(crate) threshold: Threshold,
}

impl Tag {
    pub(crate) fn with_options(
        text: impl Into<String>,
        labels: impl IntoIterator<Item = crate::core::ChoiceOption>,
    ) -> Result<Self, Error> {
        let text = text.into();
        let labels: Vec<_> = labels.into_iter().collect();
        if text.trim().is_empty() {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement("question", "nonblank text", "blank text"),
            ));
        }
        if labels.len() < crate::core::questions::MIN_LABELS
            || !crate::core::ask::distinct_labels(
                labels.iter().map(crate::core::ChoiceOption::label),
            )
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
        let field = record.field();
        Ok(self
            .labels
            .iter()
            .map(|label| Ask::YesNo {
                instructions: super::quoted(
                    field,
                    &format!(
                        "{} Does the label {} apply?",
                        self.text,
                        serde_json::to_string(label.label()).unwrap_or_default()
                    ),
                ),
                yes: label.description_text().map(str::to_owned),
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
                labels.push(label.label().to_owned());
            }
            detail.insert(label.label().to_owned(), p);
        }
        Outcome::Answered {
            answer: Answer::Labels(labels),
            p: None,
            detail: Some(Detail::Labels(detail)),
        }
    }
}
