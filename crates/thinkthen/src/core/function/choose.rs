//! Choose the best fitting option, or report not sure.

use crate::core::{Answer, Ask, ChoiceOption, Error, ErrorKind, Outcome, Threshold, Verdict};

/// A choice question with ordered options.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Choose {
    text: String,
    options: Vec<ChoiceOption>,
    threshold: Threshold,
}

impl Choose {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::Choice {
            options: self.options.iter().map(Into::into).collect(),
            question: &self.text,
            threshold: self.threshold.canonical(),
        }
    }

    /// Replace the options, keeping the question's other settings.
    ///
    /// # Errors
    /// Returns `usage.question` when the replacement options are invalid.
    pub fn options(self, options: Vec<ChoiceOption>) -> Result<Self, Error> {
        let replacement = Self::new(self.text, options)?;
        Ok(Self {
            threshold: self.threshold,
            ..replacement
        })
    }

    /// A question and its options, in order.
    ///
    /// # Errors
    /// Returns `usage.question` for a blank question, fewer than two options,
    /// or duplicate option labels.
    pub fn new(
        text: impl Into<String>,
        options: impl IntoIterator<Item = ChoiceOption>,
    ) -> Result<Self, Error> {
        let text = text.into();
        let options: Vec<_> = options.into_iter().collect();
        if text.trim().is_empty() || options.len() < 2 {
            return Err(Error::new(
                ErrorKind::Question,
                "choose needs a question and at least two options",
            ));
        }
        for (place, option) in options.iter().enumerate() {
            if options
                .iter()
                .take(place)
                .any(|earlier| earlier.label() == option.label())
            {
                return Err(Error::new(
                    ErrorKind::Question,
                    "choose options must have distinct labels",
                ));
            }
        }
        Ok(Self {
            text,
            options,
            threshold: Threshold::default(),
        })
    }

    /// Accept the winning option only when its probability reaches this threshold.
    #[must_use]
    pub fn threshold(self, threshold: Threshold) -> Self {
        Self { threshold, ..self }
    }
}

impl super::Function for Choose {
    const NAME: &'static str = "choose";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
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
