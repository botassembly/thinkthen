//! Find the line that best answers a question.

use std::collections::BTreeMap;

use crate::core::{
    Answer, Ask, ChoiceOption, Detail, Error, ErrorKind, Outcome, Threshold, Verdict,
};

/// A question selecting one nonblank line of each record.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Find {
    text: String,
    threshold: Threshold,
}

// Schema derives emit metadata maps in sibling impls, scoped to result values.
#[allow(
    clippy::disallowed_types,
    reason = "schema derivation builds metadata; evidence remains typed"
)]
mod output {

    /// A chosen line and its Unicode scalar offsets in the record.
    #[derive(Debug, Clone, PartialEq, serde::Serialize, schemars::JsonSchema)]
    pub struct Found {
        /// The line without its line ending.
        pub text: String,
        /// Inclusive start, counted in Unicode scalar values.
        pub start: usize,
        /// Exclusive end, counted in Unicode scalar values.
        pub end: usize,
    }
}
pub use output::Found;

impl Find {
    pub(crate) fn definition(&self) -> super::identity::Definition<'_> {
        super::identity::Definition::Find {
            question: &self.text,
            threshold: self.threshold.canonical(),
        }
    }

    /// Whether a curated selection is one whole admitted nonblank line.
    #[must_use]
    pub fn admits_found(evidence: &str, found: &Found) -> bool {
        units(evidence).iter().any(|unit| unit == found)
    }

    /// Create a nonblank question.
    ///
    /// # Errors
    /// Returns `usage.question` for a blank question.
    pub fn new(text: impl Into<String>) -> Result<Self, Error> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(Error::new(ErrorKind::Question, "find needs a question"));
        }
        Ok(Self {
            text,
            threshold: Threshold::default(),
        })
    }

    /// Accept a selected line only at this probability cut.
    ///
    /// # Errors
    /// Returns `usage.threshold` for a band.
    pub fn threshold(self, threshold: Threshold) -> Result<Self, Error> {
        Ok(Self {
            threshold: threshold.require_cut()?,
            ..self
        })
    }
}

fn units(text: &str) -> Vec<Found> {
    let mut start = 0;
    text.split_inclusive('\n')
        .filter_map(|part| {
            let line = part.strip_suffix('\n').unwrap_or(part);
            let line = if part.ends_with('\n') {
                line.strip_suffix('\r').unwrap_or(line)
            } else {
                line
            };
            let unit = Found {
                text: line.to_owned(),
                start,
                end: start + line.chars().count(),
            };
            start += part.chars().count();
            (!line.trim().is_empty()).then_some(unit)
        })
        .collect()
}

impl super::Function for Find {
    const NAME: &'static str = "find";
    fn identity_as(&self, function: &str) -> String {
        super::identity(function, &self.definition())
    }
    #[expect(clippy::expect_used, reason = "generated option labels are nonblank")]
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let evidence = record.evidence();
        let units = units(evidence);
        if let Some(limit) = limits.options.map(std::num::NonZeroUsize::get)
            && units.len() >= limit
        {
            return Err(Error::new(
                ErrorKind::BackendLimit,
                format!(
                    "the backend supports at most {} nonblank lines for find",
                    limit - 1
                ),
            ));
        }
        if units.is_empty() {
            return Ok(Vec::new());
        }
        let mut options: Vec<_> = units
            .into_iter()
            .enumerate()
            .map(|(i, unit)| {
                ChoiceOption::new(format!("u{}", i + 1))
                    .expect("generated label")
                    .description(unit.text)
            })
            .collect();
        options.push(
            ChoiceOption::new("none")
                .expect("nonblank label")
                .description("No line answers the question."),
        );
        Ok(vec![Ask::Choice {
            instructions: format!(
                "Select the line that contains the answer to this question. Select none only if no line contains the answer.\n\nQuestion: {}",
                self.text
            ),
            options,
        }])
    }

    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        _limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        let record = context.record();
        let answers = context.answers();
        let evidence = record.evidence();
        let units = units(evidence);
        if units.is_empty() {
            return Outcome::Answered {
                answer: Answer::Found(None),
                p: None,
                detail: None,
            };
        }
        let answer = match super::single(answers) {
            Ok(answer) => answer,
            Err(error) => return Outcome::Failed { error },
        };
        let probabilities = answer.probabilities();
        let detail = Detail::Labels(
            probabilities
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    (
                        if i == units.len() {
                            "none".to_owned()
                        } else {
                            format!("u{}", i + 1)
                        },
                        *p,
                    )
                })
                .collect::<BTreeMap<_, _>>(),
        );
        let (winner, p) = crate::core::ask::argmax(probabilities);
        if probabilities.last().is_some_and(|none| *none == p) {
            return Outcome::Answered {
                answer: Answer::Found(None),
                p: Some(p),
                detail: Some(detail),
            };
        }
        if question.threshold.verdict(p) != Verdict::Yes {
            return Outcome::Unsure {
                p,
                detail: Some(detail),
            };
        }
        Outcome::Answered {
            answer: Answer::Found(units.into_iter().nth(winner)),
            p: Some(p),
            detail: Some(detail),
        }
    }
}
