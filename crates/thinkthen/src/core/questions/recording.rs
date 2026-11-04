//! Shared typed answer admission for the private record store.
use super::*;
use crate::core::results;

fn read<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json).map_err(|_| {
        question_error(crate::core::diagnostic::requirement(
            "answer",
            "a value in the admitted function domain",
            "an incompatible answer (contents redacted)",
        ))
    })
}
fn span(field: &str, text: &str, start: usize, end: usize) -> bool {
    start < end
        && end <= field.chars().count()
        && field.chars().skip(start).take(end - start).eq(text.chars())
}
fn names(question: &Recognize, answer: Vec<results::Name>, field: &str) -> bool {
    answer
        .windows(2)
        .all(|pair| matches!(pair, [left,right] if left.end <= right.start))
        && answer.iter().all(|name| {
            span(field, &name.text, name.start, name.end)
                && question
                    .inner
                    .kinds
                    .iter()
                    .any(|kind| kind.label() == name.kind)
        })
}
fn relations(question: &Relate, answer: Vec<results::Relation>, field: &str) -> bool {
    answer.iter().enumerate().all(|(position, relation)| {
        let source = &relation.source;
        let target = &relation.target;
        (source.text != target.text || source.kind != target.kind)
            && (!relation.either || source.start < target.start)
            && (source.end <= target.start || target.end <= source.start)
            && span(field, &source.text, source.start, source.end)
            && span(field, &target.text, target.start, target.end)
            && !answer.iter().take(position).any(|prior| {
                prior.relation == relation.relation
                    && prior.source.text == source.text
                    && prior.source.kind == source.kind
                    && prior.target.text == target.text
                    && prior.target.kind == target.kind
            })
            && question.inner.rules.iter().any(|rule| {
                rule.name == relation.relation
                    && rule.either == relation.either
                    && ((rule.source == source.kind && rule.target == target.kind)
                        || (relation.either
                            && rule.source == target.kind
                            && rule.target == source.kind))
            })
    })
}
pub(crate) fn validate(question: &Question, json: &str, field: &str) -> Result<(), Error> {
    let valid = match question {
        Question::Decide(_) | Question::Filter(_) => {
            let _: bool = read(json)?;
            true
        }
        Question::Choose(question) => {
            let answer: String = read(json)?;
            question
                .inner
                .options
                .iter()
                .any(|option| option.label() == answer)
        }
        Question::Tag(question) => {
            let answer: Vec<String> = read(json)?;
            answer.iter().enumerate().all(|(index, label)| {
                question
                    .inner
                    .labels
                    .iter()
                    .any(|option| option.label() == label)
                    && !answer.iter().take(index).any(|prior| prior == label)
            })
        }
        Question::Score(_) => {
            let answer: f64 = read(json)?;
            answer.is_finite() && (0.0..=1.0).contains(&answer)
        }
        Question::Rank(_) => {
            let _: std::num::NonZeroU64 = read(json)?;
            true
        }
        Question::Find(_) => {
            let answer: Option<results::Found> = read(json)?;
            answer.is_none_or(|answer| {
                crate::core::function::find::units(field)
                    .iter()
                    .any(|found| {
                        found.text == answer.text
                            && found.start == answer.start
                            && found.end == answer.end
                    })
            })
        }
        Question::Recognize(question) => names(question, read(json)?, field),
        Question::Relate(question) => relations(question, read(json)?, field),
        // The record store admits annotation members separately because selectors differ.
        Question::Annotate(_) => false,
    };
    if valid {
        Ok(())
    } else {
        Err(question_error(crate::core::diagnostic::requirement(
            "answer",
            "a value in the admitted function domain",
            "an incompatible answer (contents redacted)",
        )))
    }
}
