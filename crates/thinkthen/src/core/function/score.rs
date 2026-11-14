//! `score`: a number from 0 to 1 on an ordered scale.

use crate::core::{Ask, Error, ErrorKind};

/// A question and its levels, from lowest to highest.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(crate) struct Score {
    pub(crate) text: String,
    pub(crate) levels: Vec<crate::core::ChoiceOption>,
}

impl Score {
    pub(crate) fn with_options(
        text: impl Into<String>,
        levels: impl IntoIterator<Item = crate::core::ChoiceOption>,
    ) -> Result<Self, Error> {
        let text = text.into();
        let levels: Vec<_> = levels.into_iter().collect();
        if text.trim().is_empty() {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement("question", "nonblank text", "blank text"),
            ));
        }
        if levels.len() < crate::core::questions::MIN_LEVELS
            || !crate::core::ask::distinct_labels(
                levels.iter().map(crate::core::ChoiceOption::label),
            )
        {
            return Err(Error::new(
                ErrorKind::Question,
                crate::core::diagnostic::requirement(
                    "levels",
                    "the required number of distinct nonblank levels",
                    "an insufficient or invalid level domain (contents redacted)",
                ),
            ));
        }
        Ok(Self { text, levels })
    }

    fn ask(&self, field: &str) -> Ask {
        let mut text = self.text.clone();
        for level in &self.levels {
            if let Some(description) = level.description_text() {
                text.push_str(&format!(
                    "\nLevel {} means: {}",
                    serde_json::to_string(level.label()).unwrap_or_default(),
                    serde_json::to_string(description).unwrap_or_default()
                ));
            }
        }
        Ask::Scale {
            instructions: super::quoted(field, &text),
            levels: self
                .levels
                .iter()
                .map(|level| level.label().to_owned())
                .collect(),
        }
    }
}

impl super::Function for Score {
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        _limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let field = record.field();
        Ok(vec![self.ask(field)])
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
