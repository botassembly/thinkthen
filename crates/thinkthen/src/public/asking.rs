//! One fixed question over caller texts on the question pipeline, by ADR
//! 0111 section 4. The public batches, the native vector and the single
//! calls share it, so one text makes the same questions on every path.

use crate::core::pack::{self, Ask, PackError};
use crate::core::{self, AnswerOutcome, Json, ModelName, quoted_plan_of};
use crate::engine::error::Error as EngineError;
use crate::engine::facade;
use crate::engine::pipeline::{self, Answered, Asker, Failed};
use crate::public::engine::evidence;
use crate::public::error::{Error, ErrorKind};
use crate::public::question::Question;
use crate::public::results::{Member, ObservedQuestion};

/// One caller text and its zero-based place.
pub(crate) struct Text {
    pub(crate) at: usize,
    pub(crate) input: super::QuestionInput,
}

impl Text {
    pub(crate) fn plain(&self, function: super::InputFunction) -> Result<&str, Error> {
        match &self.input {
            super::QuestionInput::Text(text) => Ok(text),
            super::QuestionInput::Record(record) if record.images().is_empty() => {
                Ok(record.plain())
            }
            super::QuestionInput::Record(_) | super::QuestionInput::Images(_) => {
                Err(Error::usage(format!(
                    "{} accepts text only; images are unsupported",
                    function.name()
                )))
            }
        }
    }
}

/// What one question needs of each text beside the text.
pub(crate) struct Decisions {
    metadata: crate::core::declaration::QuestionMetadata,
    prepared: Question,
    question: core::Question,
    asked: (ModelName, core::Descriptions),
    api: core::adapters::ApiType,
    url: core::Url,
    context: Option<core::Evidence>,
    profile: Option<core::BackendProfile>,
    kind: super::question::Kind,
    route: crate::core::adapters::built_in::images::ImageRoute,
}

impl Decisions {
    pub(crate) fn new(
        engine: &facade::Engine,
        question: &Question,
        context: Option<core::Evidence>,
    ) -> Self {
        Self {
            metadata: question.metadata.clone(),
            prepared: question.clone(),
            question: question.core.clone(),
            asked: engine.backend().asked(),
            api: engine.backend().api_type(),
            url: engine.backend().url().clone(),
            context,
            profile: engine.profile().cloned(),
            kind: question.kind,
            route: engine.backend().image_route(),
        }
    }
}

/// One text's answer, with the receipt its row reports.
pub(crate) struct Decided {
    input: std::sync::Arc<super::QuestionInput>,
    pub(crate) outcome: AnswerOutcome,
    pub(crate) answered: facade::Answered,
    pub(crate) keys: Vec<String>,
    span: (usize, usize),
}

/// Why one text has no answer.
pub(crate) enum Miss {
    /// The call refused the text before any send.
    Refused(Error),
    /// The backend failed its question; the receipt stays for observers.
    Failed(Box<Decided>),
}

impl From<Error> for Miss {
    fn from(error: Error) -> Self {
        Self::Refused(error)
    }
}

impl Asker for Decisions {
    type Input = Text;
    type Row = Decided;
    type Error = Miss;

    fn validates_batches(&self) -> bool {
        self.metadata.item_schema.is_some()
            || self.metadata.context_schema.is_some()
            || self
                .prepared
                .metadata
                .reading
                .on
                .iter()
                .any(|pointer| !pointer.is_empty())
    }

    fn label(&self, text: &Text) -> usize {
        text.at
    }

    fn asks(&self, text: &Text) -> Result<Vec<Ask>, Miss> {
        let input = self.prepared.selected_input(&text.input)?;
        self.metadata.validate_item(&input).map_err(Miss::Refused)?;
        let plan = match &input {
            super::QuestionInput::Text(text) => {
                let record = evidence(text).map_err(Miss::Refused)?;
                quoted_plan_of(
                    self.asked.clone(),
                    record,
                    &Json::String(text.clone()),
                    self.context.as_ref(),
                    vec![self.question.clone()],
                    self.profile.as_ref(),
                )
                .map_err(|error| Miss::Refused(planned(error)))?
            }
            super::QuestionInput::Record(record) => {
                let function = input_function(self.kind);
                super::images::guard(function, &text.input)?;
                if let Some(images) = record.image_state() {
                    core::image::plan(
                        self.asked.clone(),
                        images,
                        self.context.as_ref(),
                        vec![self.question.clone()],
                        self.profile.as_ref(),
                        self.route,
                    )
                } else {
                    quoted_plan_of(
                        self.asked.clone(),
                        record.evidence.clone(),
                        &record.value,
                        self.context.as_ref(),
                        vec![self.question.clone()],
                        self.profile.as_ref(),
                    )
                }
                .map_err(|error| Miss::Refused(planned(error)))?
            }
            super::QuestionInput::Images(images) => {
                let function = input_function(self.kind);
                super::images::guard(function, &text.input)?;
                core::image::plan(
                    self.asked.clone(),
                    images.state(),
                    self.context.as_ref(),
                    vec![self.question.clone()],
                    self.profile.as_ref(),
                    self.route,
                )
                .map_err(|error| Miss::Refused(planned(error)))?
            }
        };
        pack::asks_for(self.api, &self.url, &plan).map_err(|error| {
            if plan.images().is_some() {
                Miss::Refused(Error::usage(error.to_string()))
            } else {
                Miss::Refused(Error::defect("a request could not be written as JSON"))
            }
        })
    }

    fn row(&self, text: Text, answers: Vec<Answered>) -> Result<Decided, Miss> {
        let refused = |error: EngineError| Miss::Refused(Error::from(error));
        let outcomes = pipeline::read(&self.question, &answers)
            .map_err(|error| refused(EngineError::from(error)))?;
        let [outcome] = <[AnswerOutcome; 1]>::try_from(outcomes)
            .map_err(|_| Miss::Refused(Error::defect("a question read more than one outcome")))?;
        let decided = Decided {
            span: answers
                .iter()
                .find(|answer| answer.answer.is_err())
                .map_or((text.at, text.at), |answer| answer.span),
            input: std::sync::Arc::new(text.input),
            answered: pipeline::receipt(&answers, vec![outcome.clone()]).map_err(refused)?,
            keys: answers.iter().map(|answered| answered.key.hex()).collect(),
            outcome,
        };
        match decided.outcome {
            AnswerOutcome::Answered(_) => Ok(decided),
            AnswerOutcome::Failed(_) => Err(Miss::Failed(Box::new(decided))),
        }
    }
}

fn input_function(kind: super::question::Kind) -> super::InputFunction {
    use super::{InputFunction, question::Kind};
    match kind {
        Kind::Decide | Kind::Banded => InputFunction::Decide,
        Kind::Choose => InputFunction::Choose,
        Kind::Score => InputFunction::Score,
        Kind::Tag => InputFunction::Tag,
        Kind::Rank => InputFunction::Rank,
        Kind::Find | Kind::FindNone => InputFunction::Find,
    }
}

pub(crate) fn observed_function(question: &Question) -> super::InputFunction {
    input_function(question.kind)
}

impl Decided {
    /// The judgment under the question's threshold.
    pub(crate) fn judgment(&self, question: &Question) -> Option<facade::Judgment> {
        let AnswerOutcome::Answered(answer) = &self.outcome else {
            return None;
        };
        let (value, outcome) = answer.read(question.threshold);
        Some(facade::Judgment {
            answer: answer.clone(),
            value,
            outcome,
            answered: self.answered.clone(),
        })
    }

    /// The question event an observer sees.
    pub(crate) fn observed(
        &self,
        question: &Question,
        backend: &core::Backend,
    ) -> Result<ObservedQuestion, Error> {
        let mut observed = ObservedQuestion::from_reply(
            &question.core,
            question.threshold,
            question.profile.as_ref(),
            backend,
            &self.outcome,
            &self.answered.reply,
            self.answered.request.as_str(),
            self.answered.requests_sent,
            self.answered.replayed,
            1,
            0,
        )?;
        observed.requests.clone_from(&self.keys);
        Ok(observed
            .with_receipt(&self.answered)
            .with_declarations(&question.reading_metadata())
            .with_input(self.input.clone()))
    }

    /// The receipt a detailed row reports.
    pub(crate) fn member(&self, question: &Question) -> Result<Member, Error> {
        let judged = self
            .judgment(question)
            .ok_or_else(|| Error::defect("a failed question has no member"))?;
        Ok(Member::new(judged, self.keys.clone()))
    }
}

/// The public error for one text with no row.
pub(crate) fn failure(failed: Failed<Miss>) -> Error {
    match failed {
        Failed::Asker(Miss::Refused(error)) => error,
        Failed::Asker(Miss::Failed(decided)) => {
            backend_failed().with_diagnostic(super::error::diagnostic::Diagnostic::PartialReply {
                first: decided.span.0,
                last: decided.span.1,
            })
        }
        Failed::Pack { error, .. } => packed(error),
        Failed::Engine { error, first, last } => Error::from(error.clone()).with_diagnostic(
            super::error::diagnostic::Diagnostic::EngineRange {
                cause: error,
                first,
                last,
            },
        ),
        Failed::Stopped(error) => Error::from(error),
    }
}

/// A question the backend failed.
pub(crate) fn backend_failed() -> Error {
    Error::of(ErrorKind::Backend, "a backend question failed in a batch")
}

/// A text the core refused to quote.
fn planned(error: core::BatchError) -> Error {
    match error {
        core::BatchError::Profile(limit) => Error::from(EngineError::ProfileLimit(limit)),
        other => Error::refused(other),
    }
}

/// A question the packer cannot send even alone.
pub(crate) fn packed(error: PackError) -> Error {
    match error {
        PackError::Profile(limit) => Error::from(EngineError::ProfileLimit(limit)),
        PackError::Context {
            kind,
            limit,
            actual,
            ..
        } => Error::refused(core::BatchError::ContextOverLimit {
            kind,
            limit,
            actual,
        }),
    }
}
