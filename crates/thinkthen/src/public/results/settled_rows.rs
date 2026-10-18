//! Native row projections share the settled contract with collecting and streamed calls.
use super::settled::{
    AnswerResolution as Resolution, AnswerResolutions, AnswerShape, Settled, SettledAnswer,
    SettledDecision, SettledJudgment, SettledOriginal,
};
use crate::{Answer, CompleteAnnotated, CompleteRecord, Judgment, QuestionInput, RequestValue};
use std::collections::BTreeMap;

impl<T, P> Settled<T, P> {
    pub(crate) fn one(
        value: T,
        probability: P,
        pick: Option<String>,
        index: Option<usize>,
        resolution: AnswerResolutions,
    ) -> Self {
        Self {
            value: AnswerShape::Scalar(value),
            probability: AnswerShape::Scalar(probability),
            pick: AnswerShape::Scalar(pick),
            index: AnswerShape::Scalar(index),
            resolution: AnswerShape::Scalar(resolution),
        }
    }
    pub(crate) fn rows(rows: impl Iterator<Item = Self>) -> Self {
        let mut answer = Self {
            value: AnswerShape::Collection(Vec::new()),
            probability: AnswerShape::Collection(Vec::new()),
            pick: AnswerShape::Collection(Vec::new()),
            index: AnswerShape::Collection(Vec::new()),
            resolution: AnswerShape::Collection(Vec::new()),
        };
        for row in rows {
            answer.append(row);
        }
        answer
    }
    pub(crate) fn append(&mut self, other: Self) {
        self.value.append(other.value);
        self.probability.append(other.probability);
        self.pick.append(other.pick);
        self.index.append(other.index);
        self.resolution.append(other.resolution);
    }
    pub(crate) fn scalar(&mut self) {
        self.value.scalar();
        self.probability.scalar();
        self.pick.scalar();
        self.index.scalar();
        self.resolution.scalar();
    }
}
impl<T> AnswerShape<T> {
    fn append(&mut self, other: Self) {
        if let Self::Collection(rows) = self {
            match other {
                Self::Scalar(row) => rows.push(row),
                Self::Collection(more) => rows.extend(more),
            }
        }
    }
    fn scalar(&mut self) {
        if let Self::Collection(rows) = self
            && rows.len() == 1
            && let Some(value) = rows.pop()
        {
            *self = Self::Scalar(value);
        }
    }
}
fn resolved() -> AnswerResolutions {
    AnswerResolutions::State(Resolution::Resolved)
}
fn nullable(present: bool) -> AnswerResolutions {
    AnswerResolutions::State(if present {
        Resolution::Resolved
    } else {
        Resolution::Unresolved
    })
}
fn decision(value: Answer) -> Option<bool> {
    match value {
        Answer::Yes => Some(true),
        Answer::No => Some(false),
        Answer::Unsure => None,
    }
}
fn annotation(
    row: &CompleteRecord<QuestionInput, CompleteAnnotated>,
) -> Settled<BTreeMap<String, SettledJudgment>, Option<f64>> {
    let mut values = BTreeMap::new();
    let mut resolutions = BTreeMap::new();
    for member in row.result().members() {
        let value = match member.value() {
            Some(Judgment::Decision(value)) => SettledJudgment::Decision(decision(value)),
            Some(Judgment::Choice(value)) => SettledJudgment::Choice(value),
            Some(Judgment::Score(value)) => SettledJudgment::Score(value),
            Some(Judgment::Tags(value)) => SettledJudgment::Tags(value),
            None => {
                resolutions.insert(member.name().to_owned(), Resolution::Failed);
                continue;
            }
        };
        let resolution = match &value {
            SettledJudgment::Decision(None) | SettledJudgment::Choice(None) => {
                Resolution::Unresolved
            }
            _ => Resolution::Resolved,
        };
        resolutions.insert(member.name().to_owned(), resolution);
        values.insert(member.name().to_owned(), value);
    }
    Settled::one(
        values,
        None,
        None,
        None,
        AnswerResolutions::Members(resolutions),
    )
}
impl SettledAnswer {
    pub(crate) fn rows(value: &RequestValue) -> Option<Self> {
        Some(match value {
            RequestValue::Decisions(rows) => Self::Decide(Settled::rows(rows.iter().map(|row| {
                let value = decision(row.result().value());
                let meaning = match row.result().canonical.decide_value(value) {
                    crate::core::DecideValue::Authored(meaning) => {
                        Some(meaning.clone())
                    }
                    crate::core::DecideValue::Primitive(_) => None,
                };
                Settled::one(
                    SettledDecision { value, meaning },
                    row.result().probability(),
                    None,
                    None,
                    nullable(value.is_some()),
                )
            }))),
            RequestValue::Choices(rows) => Self::Choose(Settled::rows(rows.iter().map(|row| {
                let result = row.result();
                Settled::one(
                    result.value().map(str::to_owned),
                    result.probability(),
                    result.raw_pick().map(str::to_owned),
                    None,
                    nullable(result.value().is_some()),
                )
            }))),
            RequestValue::Tags(rows) => Self::Tag(Settled::rows(rows.iter().map(|row| {
                Settled::one(row.result().value().to_vec(), None, None, None, resolved())
            }))),
            RequestValue::Scores(rows) => Self::Score(Settled::rows(rows.iter().map(|row| {
                Settled::one(
                    row.result().value(),
                    None,
                    row.result().raw_level().map(str::to_owned),
                    None,
                    resolved(),
                )
            }))),
            RequestValue::Filtered(rows) => Self::Filter(Settled::one(
                rows.iter()
                    .filter(|row| row.result().value())
                    .map(|row| SettledOriginal(row.original().clone()))
                    .collect(),
                rows.iter()
                    .filter(|row| row.result().value())
                    .map(|row| row.result().probability())
                    .collect(),
                None,
                None,
                resolved(),
            )),
            RequestValue::Annotations(rows) => {
                Self::Annotate(Settled::rows(rows.iter().map(annotation)))
            }
            _ => return None,
        })
    }
}
