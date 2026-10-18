//! One native accumulator owns answer shape before rows leave the session worker.
use super::{RequestInput, RequestValue};
use crate::{Error, SettledAnswer};

pub(super) struct Settlement {
    answer: Option<SettledAnswer>,
    scalar: bool,
    failures: std::collections::BTreeMap<usize, crate::AnswerResolution>,
}
impl Settlement {
    pub(super) fn new(input: &RequestInput) -> Self {
        Self {
            answer: None,
            failures: std::collections::BTreeMap::new(),
            scalar: matches!(input, RequestInput::Text { .. } | RequestInput::Json { .. }),
        }
    }
    pub(super) fn record(&mut self, value: &RequestValue) {
        if let Some(answer) = SettledAnswer::of(value) {
            match &mut self.answer {
                Some(previous) => previous.append(answer),
                None => self.answer = Some(answer),
            }
        }
    }
    pub(super) fn recovered(&mut self, function: super::RequestFunction, row: &super::RequestRecoveredRow) {
        match row.outcome() {
            Ok(value) => self.record(value),
            Err(_) => {
                self.failures.insert(row.ordinal(), crate::AnswerResolution::Failed);
                let answer = SettledAnswer::failed(function);
                match &mut self.answer { Some(previous) => previous.append(answer), None => self.answer = Some(answer) }
            }
        }
    }
    pub(super) fn finish(mut self) -> Option<SettledAnswer> {
        if !self.failures.is_empty() && let Some(answer) = &mut self.answer {
            answer.failed_occurrences(self.failures);
        }
        if self.scalar
            && let Some(answer) = &mut self.answer
        {
            answer.scalar();
        }
        self.answer
    }
}
pub(super) fn answer(input: &RequestInput, value: &RequestValue) -> Result<SettledAnswer, Error> {
    let mut settlement = Settlement::new(input);
    settlement.record(value);
    settlement
        .finish()
        .ok_or_else(|| Error::defect("a native call has no settled answer"))
}

pub(super) fn recovered(request: &super::AdmittedRequest, call: crate::Call<Vec<super::RequestRecoveredRow>>) -> crate::Call<Vec<super::RequestRecoveredRow>> {
    let function = request.request.call.function();
    let mut settlement = Settlement::new(&request.request.call.arguments().input);
    for row in call.value() { settlement.recovered(function, row); }
    call.with_answer(settlement.finish().unwrap_or_else(|| SettledAnswer::empty(function)))
}
