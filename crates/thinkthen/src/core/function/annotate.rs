//! Named questions with independent outcomes for one record.

use std::collections::BTreeMap;

use crate::core::{
    Answer, Ask, AskAnswer, Error, Function, Outcome, Pointer, Record, Toolbox, ToolboxQuestion,
};

/// Every question in a toolbox, answered independently for each record.
#[derive(Debug, serde::Serialize)]
pub(crate) struct Annotate {
    questions: Vec<(String, ToolboxQuestion)>,
    #[serde(skip)]
    complete_choices: bool,
}

impl Annotate {
    /// Use every named question in the toolbox, in name order.
    #[must_use]
    pub(crate) fn new(toolbox: &Toolbox) -> Self {
        Self {
            complete_choices: false,
            questions: toolbox
                .questions()
                .map(|(name, question)| (name.to_owned(), question.clone()))
                .collect(),
        }
    }

    pub(crate) fn complete_choices(mut self) -> Self {
        self.complete_choices = true;
        self
    }

    // The engine concatenates rounds. Replay each round's ask counts to recover
    // each question's own history, including its record-specific evidence.
    #[expect(
        clippy::expect_used,
        reason = "the packer delivers one answer per prepared ask in round order"
    )]
    fn route(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
        answers: &[Result<AskAnswer, Error>],
    ) -> Vec<Entry<'_>> {
        let mut entries: Vec<_> = self
            .questions
            .iter()
            .map(|(name, question)| {
                let prepared = evidence(question, record.record()).and_then(|text| {
                    question
                        .try_asks_record(&crate::core::AdmittedRecord::text(&text), limits)
                        .map(|asks| (crate::core::AdmittedRecord::text(text), asks))
                });
                let (text, pending, error) = match prepared {
                    Ok((text, asks)) => (text, asks, None),
                    Err(error) => (
                        crate::core::AdmittedRecord::text(""),
                        Vec::new(),
                        Some(error),
                    ),
                };
                Entry {
                    name,
                    question,
                    text,
                    pending,
                    error,
                    history: Vec::new(),
                }
            })
            .collect();
        if answers.is_empty() {
            return entries;
        }
        let mut answers = answers.iter().peekable();
        loop {
            for entry in &mut entries {
                for _ in std::mem::take(&mut entry.pending) {
                    entry.history.push(
                        answers
                            .next()
                            .expect("the packer delivers every ask's answer")
                            .clone(),
                    );
                }
                if entry.error.is_none() {
                    match Function::followup_record(
                        crate::core::FunctionAnswers::new(
                            entry.question,
                            &entry.text,
                            &entry.history,
                        ),
                        limits,
                    ) {
                        Ok(asks) => entry.pending = asks,
                        Err(error) => entry.error = Some(error),
                    }
                }
            }
            if answers.peek().is_none() {
                return entries;
            }
        }
    }
}

struct Entry<'a> {
    name: &'a str,
    question: &'a ToolboxQuestion,
    text: crate::core::AdmittedRecord,
    pending: Vec<Ask>,
    error: Option<Error>,
    history: Vec<Result<AskAnswer, Error>>,
}

fn evidence(question: &ToolboxQuestion, record: &Record) -> Result<String, Error> {
    record.evidence_at(question.on().unwrap_or(&Pointer::default()))
}

impl Function for Annotate {
    #[expect(
        clippy::expect_used,
        reason = "the packer passes a prepared ask's position"
    )]
    fn stub_key(
        context: crate::core::FunctionAnswers<'_, Self>,
        _identity: &str,
        limits: crate::core::Limits,
        mut position: usize,
    ) -> [u8; 32] {
        let entries = context
            .question()
            .route(context.record(), limits, context.answers());
        entries
            .into_iter()
            .find_map(|entry| {
                if position < entry.pending.len() {
                    Some(super::stub_key(
                        entry.question.stub_identity(),
                        entry.text.evidence(),
                        entry.history.len(),
                        position,
                    ))
                } else {
                    position -= entry.pending.len();
                    None
                }
            })
            .expect("prepared annotation ask")
    }
    fn try_asks_record(
        &self,
        record: &crate::core::AdmittedRecord,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        Ok(self
            .questions
            .iter()
            .flat_map(|(_, question)| {
                evidence(question, record.record())
                    .and_then(|text| {
                        question.try_asks_record(&crate::core::AdmittedRecord::text(text), limits)
                    })
                    .unwrap_or_default()
            })
            .collect())
    }

    fn followup_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Result<Vec<Ask>, Error> {
        let question = context.question();
        let record = context.record();
        let answers = context.answers();
        Ok(question
            .route(record, limits, answers)
            .into_iter()
            .flat_map(|entry| entry.pending)
            .collect())
    }

    fn read_record(
        context: crate::core::FunctionAnswers<'_, Self>,
        limits: crate::core::Limits,
    ) -> Outcome {
        let question = context.question();
        let record = context.record();
        let answers = context.answers();
        let entries: BTreeMap<_, _> = question
            .route(record, limits, answers)
            .into_iter()
            .map(|entry| {
                let outcome = match entry.error {
                    Some(error) => Outcome::Failed { error },
                    None => match (question.complete_choices, entry.question.function()) {
                        (true, crate::core::ToolboxFunction::Choose(choice)) => {
                            crate::core::questions::complete_choice(
                                crate::core::FunctionAnswers::new(
                                    choice,
                                    &entry.text,
                                    &entry.history,
                                ),
                                limits,
                            )
                        }
                        _ => Function::read_record(
                            crate::core::FunctionAnswers::new(
                                entry.question,
                                &entry.text,
                                &entry.history,
                            ),
                            limits,
                        ),
                    },
                };
                (entry.name.to_owned(), outcome)
            })
            .collect();
        Outcome::Answered {
            answer: Answer::Annotations(entries),
            p: None,
            detail: None,
        }
    }
}
