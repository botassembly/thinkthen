//! One native accumulator owns answer shape before rows leave the session worker.
use super::{RequestInput, RequestValue};
use crate::{Error, SettledAnswer};

pub(super) struct Settlement {
    answer: Option<SettledAnswer>,
    scalar: bool,
}
impl Settlement {
    pub(super) fn new(input: &RequestInput) -> Self {
        Self {
            answer: None,
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
    pub(super) fn finish(mut self) -> Option<SettledAnswer> {
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
