//! Logical construction order retains actual accepted constituents before surface conversion.
use crate::core::image::InputFunction;
use crate::core::{Observation, Question, QuestionSource, RenderError, ResultIdentity};
use serde::Serialize;

#[derive(Clone, Debug, Default)]
pub(crate) struct LogicalTrace {
    pub(crate) sources: Vec<QuestionSource>,
    pub(crate) observations: Vec<Observation>,
    readings: Vec<Reading>,
}
#[derive(Clone, Debug, Serialize)]
struct Reading {
    stage: &'static str,
    position: usize,
    question: Question,
}
impl LogicalTrace {
    pub(crate) fn take(
        &mut self,
        stage: &'static str,
        question: &Question,
        sources: &[QuestionSource],
        observations: &[Observation],
    ) {
        let position = self
            .readings
            .iter()
            .filter(|read| read.stage == stage)
            .count();
        self.readings.push(Reading {
            stage,
            position,
            question: question.clone(),
        });
        self.sources.extend_from_slice(sources);
        self.observations.extend_from_slice(observations);
    }
    pub(crate) fn identity(
        &self,
        function: InputFunction,
        scope: &impl Serialize,
        resolved: &impl Serialize,
        children: &[crate::core::AnswerId],
    ) -> Result<ResultIdentity, RenderError> {
        ResultIdentity::of(
            function,
            scope,
            self.sources.clone(),
            self.observations.clone(),
            &(resolved, &self.readings),
            children,
        )
    }
}
