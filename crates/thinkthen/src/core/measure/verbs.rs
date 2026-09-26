//! How the answer of each verb reads from a saved line: a `tag` answer's
//! labels, a question's names, a probability, and a distribution's top.

use crate::core::json::Json;
use crate::core::measure::MeasureError;
use crate::core::probability::Probability;

use super::{Answer, Printed, Top, Verb, verb_named};

impl Answer {
    /// One yes/no answer per label of a `tag` answer, in the order the probabilities give.
    pub(super) fn labels(
        self,
        question: Option<&Json>,
        value: Option<&Json>,
        distribution: Option<&Json>,
    ) -> Result<Vec<Self>, MeasureError> {
        let line = self.line;
        let said: Option<Vec<String>> = match value {
            Some(Json::Array(items)) => Some(
                items
                    .iter()
                    .map(|item| item.as_str().map(str::to_owned))
                    .collect::<Option<_>>()
                    .ok_or(MeasureError::Ungradable(line))?,
            ),
            None | Some(Json::Null) => None,
            Some(_) => return Err(MeasureError::Ungradable(line)),
        };
        let pairs: Vec<(String, Option<Probability>)> = match distribution {
            Some(Json::Object(members)) => members
                .iter()
                .map(|(label, p)| Ok((label.clone(), Some(probability(line, p)?))))
                .collect::<Result<_, MeasureError>>()?,
            Some(_) => return Err(MeasureError::Probability(line)),
            None => names(question.and_then(|held| held.member("labels")))
                .ok_or(MeasureError::Ungradable(line))?
                .into_iter()
                .map(|label| (label, None))
                .collect(),
        };
        let options: Vec<String> = pairs.iter().map(|(label, _)| label.clone()).collect();
        Ok(pairs
            .into_iter()
            .map(|(label, p)| Self {
                value: said
                    .as_ref()
                    .map(|said| Printed::Bool(said.contains(&label))),
                probability: p,
                options: options.clone(),
                label: Some(label),
                ..self.clone()
            })
            .collect())
    }
}

/// A list of names, or the member names of a map, as a question lists its labels or levels.
pub(super) fn names(value: Option<&Json>) -> Option<Vec<String>> {
    match value? {
        Json::Array(items) => items
            .iter()
            .map(|item| item.as_str().map(str::to_owned))
            .collect(),
        Json::Object(members) => Some(members.iter().map(|(name, _)| name.clone()).collect()),
        _ => None,
    }
}

pub(super) fn probability(line: usize, value: &Json) -> Result<Probability, MeasureError> {
    let Json::Number(number) = value else {
        return Err(MeasureError::Probability(line));
    };
    number
        .as_f64()
        .and_then(|held| Probability::new(held).ok())
        .ok_or(MeasureError::Probability(line))
}

pub(super) fn top(line: usize, value: &Json) -> Result<Top, MeasureError> {
    let Json::Object(members) = value else {
        return Err(MeasureError::Probability(line));
    };
    let mut found: Option<Top> = None;
    for (option, held) in members {
        let p = probability(line, held)?.as_f64();
        match &mut found {
            Some(top) if p == top.top => top.tied = true,
            Some(top) if p < top.top => {}
            _ => {
                found = Some(Top {
                    top: p,
                    pick: option.clone(),
                    tied: false,
                });
            }
        }
    }
    found.ok_or(MeasureError::Probability(line))
}

/// The verb a saved line answers, from its question's verb and its value.
pub(super) fn verb(
    line: usize,
    entry: &Json,
    audit: bool,
    failed: bool,
) -> Result<Verb, MeasureError> {
    let value = entry.member("value");
    Ok(match verb_named(entry) {
        Some("decide")
            if audit
                && entry.member("threshold") == Some(&Json::Null)
                && matches!(value, Some(Json::Null)) =>
        {
            Verb::Rank
        }
        Some("decide") => Verb::Decide,
        Some("choose") => Verb::Choose,
        Some("tag") if audit => Verb::Tag,
        Some("score") if audit => Verb::Score,
        Some("find") if audit => Verb::Find,
        Some(other) if !other.is_empty() => return Err(MeasureError::Ungradable(line)),
        _ if matches!(value, None | Some(Json::Null | Json::Bool(_))) => Verb::Decide,
        _ if audit && !matches!(value, Some(Json::String(_))) && !failed => {
            return Err(MeasureError::NoQuestion(line));
        }
        _ => Verb::Choose,
    })
}
