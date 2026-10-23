//! `score`: a number from 0 to 1 on an ordered scale.

use crate::core::{Ask, Error, ErrorKind};

/// A question and its levels, from lowest to highest.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Score {
    text: String,
    levels: Vec<String>,
}

impl Score {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::Scale {
            levels: &self.levels,
            question: &self.text,
        }
    }

    /// Replace the levels, keeping the question's other settings.
    ///
    /// # Errors
    /// Returns `usage.question` when the replacement levels are invalid.
    pub fn levels(self, levels: Vec<String>) -> Result<Self, Error> {
        let replacement = Self::new(self.text, levels)?;
        Ok(replacement)
    }

    /// A question on an ordered scale of at least two distinct nonblank levels.
    ///
    /// # Errors
    ///
    /// `usage.question` when the question or its levels are invalid.
    pub fn new(
        text: impl Into<String>,
        levels: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, Error> {
        let text = text.into();
        let levels: Vec<String> = levels.into_iter().map(Into::into).collect();
        if text.trim().is_empty() {
            return Err(Error::new(ErrorKind::Question, "the question is blank"));
        }
        if levels.len() < 2
            || levels.iter().enumerate().any(|(place, level)| {
                level.trim().is_empty() || levels.iter().take(place).any(|earlier| earlier == level)
            })
        {
            return Err(Error::new(
                ErrorKind::Question,
                "a scale needs at least two distinct nonblank levels",
            ));
        }
        Ok(Self { text, levels })
    }

    fn ask(&self, evidence: &str) -> Ask {
        Ask::Scale {
            instructions: super::quoted(evidence, &self.text),
            levels: self.levels.clone(),
        }
    }
}

impl super::Function for Score {
    const NAME: &'static str = "score";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        _limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let evidence = record.evidence();
        Ok(vec![self.ask(evidence)])
    }

    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> crate::core::Outcome {
        let question = context.question();
        let answers = context.answers();
        use crate::core::{Answer, Detail, Outcome};

        let answer = match super::single(answers) {
            Ok(answer) => answer,
            Err(error) => return Outcome::Failed { error },
        };
        let probabilities = answer.probabilities();
        let total: f64 = probabilities.iter().map(|p| p.get()).sum();
        let weighted: f64 = probabilities
            .iter()
            .enumerate()
            .map(|(index, p)| index as f64 * p.get())
            .sum();
        let answer = weighted / total / (question.levels.len() - 1) as f64;
        Outcome::Answered {
            answer: Answer::Number(answer),
            p: None,
            detail: Some(Detail::Levels(probabilities.to_vec())),
        }
    }
}
