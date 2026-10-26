//! Choose the best fitting option, or report not sure.

use crate::core::{Answer, Ask, ChoiceOption, Error, ErrorKind, Outcome, Threshold, Verdict};

/// A choice question with ordered options.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub(crate) struct Choose {
    pub(crate) text: String,
    pub(crate) options: Vec<ChoiceOption>,
    pub(crate) threshold: Threshold,
}

impl Choose {
    /// A question and its options, in order.
    ///
    /// # Errors
    /// Returns `usage.question` for a blank question, fewer than two options,
    /// or duplicate option labels.
    pub(crate) fn new(
        text: impl Into<String>,
        options: impl IntoIterator<Item = ChoiceOption>,
    ) -> Result<Self, Error> {
        let text = text.into();
        let options: Vec<_> = options.into_iter().collect();
        if text.trim().is_empty() {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement("question", "nonblank text", "blank text"),
            ));
        }
        if options.len() < crate::core::questions::MIN_CHOICES {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement(
                    "choices",
                    &format!("at least {} choices", crate::core::questions::MIN_CHOICES),
                    &format!("{} choices", options.len()),
                ),
            ));
        }
        if !crate::core::ask::distinct_labels(options.iter().map(ChoiceOption::label)) {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement(
                    "choices",
                    "distinct labels",
                    "repeated labels (contents redacted)",
                ),
            ));
        }
        Ok(Self {
            text,
            options,
            threshold: Threshold::default(),
        })
    }
}

impl super::Function for Choose {
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        _limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let evidence = record.evidence();
        Ok(vec![Ask::Choice {
            instructions: super::quoted(evidence, &self.text),
            options: self.options.clone(),
        }])
    }

    #[expect(
        clippy::expect_used,
        reason = "validated choices have at least two options"
    )]
    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        let answers = context.answers();
        match super::single(answers) {
            Ok(answer) => {
                let (winner, p) = crate::core::ask::argmax(answer.probabilities());
                let option = question
                    .options
                    .get(winner)
                    .expect("decoded choice matches its ask");
                if question.threshold.verdict(p) == Verdict::Yes {
                    Outcome::Answered {
                        answer: Answer::Label(option.label().to_owned()),
                        p: Some(p),
                        detail: None,
                    }
                } else {
                    Outcome::Unsure { p, detail: None }
                }
            }
            Err(error) => Outcome::Failed { error },
        }
    }
}
