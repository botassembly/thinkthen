//! One audit row per group: agreement, both directions, AUC, calibration,
//! coverage, and a suggested cut tuned on one part and checked on the other.

use serde::Serialize;

use crate::core::measure::answer::{Answer, Rule, Shown, Verb};
use crate::core::measure::key::Key;
use crate::core::measure::levels::LevelCuts;
use crate::core::measure::optimize::{Measure, Steady};
use crate::core::measure::rows::row;
use crate::core::measure::{Calibration, MeasureError};

/// One group per answer name, else question text, else verb; or one per verb.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum By {
    Question,
    Verb,
}

/// What one audit run was asked.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Settings {
    pub(crate) by: By,
    pub(crate) rule: Rule,
    pub(crate) shown: Shown,
    pub(crate) seed: u64,
    pub(crate) target: f64,
    pub(crate) optimize: Measure,
}

/// Which answers of a question a row pools.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Kind {
    /// Every answer of one question.
    #[default]
    Whole,
    /// Every label of a `tag` question, pooled.
    Pooled,
    /// One label of a `tag` question.
    Label,
}

/// Counts under one rule over labeled answers.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub(crate) struct Counts {
    pub(crate) n: usize,
    pub(crate) answered: usize,
    pub(crate) right: usize,
    pub(crate) wrong: usize,
    pub(crate) unresolved: usize,
    pub(crate) tied: usize,
    pub(crate) true_yes: usize,
    pub(crate) false_yes: usize,
    pub(crate) true_no: usize,
    pub(crate) false_no: usize,
    pub(crate) agreement: Option<f64>,
    pub(crate) coverage: Option<f64>,
    pub(crate) yes_recall: Option<f64>,
    pub(crate) precision: Option<f64>,
    pub(crate) f1: Option<f64>,
}

/// One wrong-answer pair and its count.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct Disagreement {
    pub(crate) key: String,
    pub(crate) said: String,
    pub(crate) count: usize,
}

/// One point of the coverage curve.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Point {
    pub(crate) cut: f64,
    pub(crate) threshold: Shown,
    pub(crate) answered: usize,
    pub(crate) coverage: Option<f64>,
    pub(crate) right: usize,
    pub(crate) accuracy: Option<f64>,
}

/// One part of the split, counted under the run's rule and under the cut.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Checked {
    pub(crate) n: usize,
    pub(crate) at_run: Counts,
    pub(crate) at_cut: Counts,
}

/// The suggested cut.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Suggested {
    pub(crate) cut: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) cuts: Option<LevelCuts>,
    pub(crate) objective: String,
    pub(crate) split: &'static str,
    pub(crate) seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tune: Option<Checked>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) held: Option<Checked>,
    /// Absent where the measure does not apply, else null or the count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) steady: Option<Option<Steady>>,
}

/// One printed audit row.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Row {
    pub(crate) group: String,
    pub(crate) verb: Option<&'static str>,
    pub(crate) rows: usize,
    pub(crate) failed: usize,
    pub(crate) labeled: usize,
    pub(crate) unlabeled: usize,
    pub(crate) threshold: Shown,
    pub(crate) right: usize,
    pub(crate) wrong: usize,
    pub(crate) unresolved: usize,
    pub(crate) tied: usize,
    pub(crate) agreement: Option<f64>,
    pub(crate) interval: Option<[f64; 2]>,
    pub(crate) true_yes: Option<usize>,
    pub(crate) false_yes: Option<usize>,
    pub(crate) true_no: Option<usize>,
    pub(crate) false_no: Option<usize>,
    pub(crate) yes_recall: Option<f64>,
    pub(crate) precision: Option<f64>,
    pub(crate) f1: Option<f64>,
    pub(crate) mean_probability: Option<f64>,
    pub(crate) auc: Option<f64>,
    pub(crate) r_precision: Option<f64>,
    pub(crate) mean_level_distance: Option<f64>,
    pub(crate) disagreements: Option<Vec<Disagreement>>,
    pub(crate) calibration: Option<Calibration>,
    pub(crate) coverage: Option<Vec<Point>>,
    pub(crate) suggested: Option<Suggested>,
    /// Which answers of the question the row pools.
    #[serde(skip)]
    pub(crate) kind: Kind,
    /// The answer name, for a member of a question set.
    #[serde(skip)]
    pub(crate) name: Option<String>,
    /// True when an answer ran under a band.
    #[serde(skip)]
    pub(crate) band: bool,
}

/// Grade every answer against the key, one row per group in first-seen order.
///
/// A `tag` question gives a pooled row and then one row per label; under
/// `--by verb` it gives the pooled row alone.
///
/// # Errors
///
/// Returns [`MeasureError`] for a group with two verbs, a rule the answers
/// cannot take, a key value the answers cannot hold, or mixed key parts.
pub(crate) fn audit(
    answers: &[Answer],
    key: &Key,
    settings: &Settings,
) -> Result<Vec<Row>, MeasureError> {
    let mut groups: Vec<(String, Vec<&Answer>)> = Vec::new();
    for answer in answers {
        let name = match settings.by {
            By::Verb => answer.verb.name().to_owned(),
            By::Question => [&answer.name, &answer.text]
                .into_iter()
                .flatten()
                .find(|name| !name.is_empty())
                .cloned()
                .unwrap_or_else(|| answer.verb.name().to_owned()),
        };
        match groups.iter_mut().find(|(held, _)| *held == name) {
            Some((_, members)) => members.push(answer),
            None => groups.push((name, vec![answer])),
        }
    }
    let mut rows = Vec::new();
    for (name, members) in groups {
        let labels = members
            .iter()
            .find(|answer| answer.verb == Verb::Tag && !answer.failed)
            .map(|answer| answer.options.clone());
        let Some(labels) = labels else {
            rows.push(row(name, Kind::Whole, &members, key, settings)?);
            continue;
        };
        rows.push(row(name.clone(), Kind::Pooled, &members, key, settings)?);
        if settings.by == By::Verb {
            continue;
        }
        for label in labels {
            let one: Vec<&Answer> = members
                .iter()
                .copied()
                .filter(|answer| answer.failed || answer.label.as_ref() == Some(&label))
                .collect();
            rows.push(row(
                format!("{name}/{label}"),
                Kind::Label,
                &one,
                key,
                settings,
            )?);
        }
    }
    Ok(rows)
}
