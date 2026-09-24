//! One audit row per group: agreement, both directions, AUC, calibration,
//! coverage, and a suggested cut tuned on one part and checked on the other.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Serialize, Serializer};

use crate::core::measure::answer::{Answer, Rule, Said, Verb};
use crate::core::measure::key::{Key, Outcome, Part, Want, outcome};
use crate::core::measure::{
    Calibration, MeasureError, SplitMix64, auc, calibration, python_float_text, python_sum,
    shuffle, wilson,
};
use crate::core::threshold::Threshold;

/// One group per answer name, else question text, else verb; or one per verb.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum By {
    Question,
    Verb,
}

/// A rule as the output prints it: `"as run"`, a cut as a number, or a band as text.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Shown {
    AsRun,
    Cut(f64),
    Band(String),
}

impl Serialize for Shown {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AsRun => serializer.serialize_str("as run"),
            Self::Cut(cut) => serializer.serialize_f64(*cut),
            Self::Band(band) => serializer.serialize_str(band),
        }
    }
}

/// What one audit run was asked.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Settings {
    pub(crate) by: By,
    pub(crate) rule: Rule,
    pub(crate) shown: Shown,
    pub(crate) seed: u64,
    pub(crate) target: f64,
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
    pub(crate) objective: String,
    pub(crate) split: &'static str,
    pub(crate) seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) tune: Option<Checked>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) held: Option<Checked>,
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
    pub(crate) mean_probability: Option<f64>,
    pub(crate) auc: Option<f64>,
    pub(crate) disagreements: Option<Vec<Disagreement>>,
    pub(crate) calibration: Option<Calibration>,
    pub(crate) coverage: Option<Vec<Point>>,
    pub(crate) suggested: Option<Suggested>,
}

/// A labeled answer and its key value.
type Graded<'a> = (&'a Answer, Want);

/// Grade every answer against the key, one row per group in first-seen order.
///
/// # Errors
///
/// Returns [`MeasureError`] for a group with two verbs, a rule the answers
/// cannot take, a `choose` key value that is not text, or mixed key parts.
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
    groups
        .into_iter()
        .map(|(name, members)| row(name, &members, key, settings))
        .collect()
}

fn row(
    group: String,
    members: &[&Answer],
    key: &Key,
    settings: &Settings,
) -> Result<Row, MeasureError> {
    let ok: Vec<&Answer> = members.iter().copied().filter(|a| !a.failed).collect();
    let verb = ok.first().map(|answer| answer.verb);
    if ok.iter().any(|answer| Some(answer.verb) != verb) {
        return Err(MeasureError::TwoVerbs);
    }
    let mut labeled: Vec<Graded<'_>> = Vec::new();
    for answer in &ok {
        if let Some(want) = key.want(answer)? {
            labeled.push((answer, want));
        }
    }
    let counts = count(&labeled, settings.rule)?;
    let decide = verb == Some(Verb::Decide);
    let probabilities = !labeled.is_empty() && labeled.iter().all(|(a, _)| a.has_probability());
    let yes_pairs: Vec<(f64, bool)> = if decide && probabilities {
        labeled
            .iter()
            .filter_map(|(a, want)| Some((a.confidence()?, *want == Want::Yes)))
            .collect()
    } else {
        Vec::new()
    };
    let direction = |value: usize| decide.then_some(value);
    let mut row = Row {
        group,
        verb: verb.map(Verb::name),
        rows: ok.len(),
        failed: members.len() - ok.len(),
        labeled: labeled.len(),
        unlabeled: ok.len() - labeled.len(),
        threshold: settings.shown.clone(),
        right: counts.right,
        wrong: counts.wrong,
        unresolved: counts.unresolved,
        tied: counts.tied,
        agreement: counts.agreement,
        interval: wilson(counts.right, counts.answered),
        true_yes: direction(counts.true_yes),
        false_yes: direction(counts.false_yes),
        true_no: direction(counts.true_no),
        false_no: direction(counts.false_no),
        yes_recall: counts.yes_recall.filter(|_| decide),
        mean_probability: (!yes_pairs.is_empty())
            .then(|| python_sum(yes_pairs.iter().map(|pair| pair.0)) / yes_pairs.len() as f64),
        auc: auc(&yes_pairs),
        disagreements: if decide {
            None
        } else {
            Some(disagreements(&labeled, settings.rule)?)
        },
        calibration: None,
        coverage: None,
        suggested: None,
    };
    if probabilities {
        let pairs: Vec<(f64, bool)> = if decide {
            yes_pairs
        } else {
            labeled
                .iter()
                .filter_map(|(a, want)| a.top.as_ref().map(|top| (top, want)))
                .filter(|(top, _)| !top.tied)
                .map(|(top, want)| (top.top, top.pick == want.text()))
                .collect()
        };
        row.calibration = calibration(&pairs, settings.seed);
        row.coverage = Some(coverage(&labeled, decide)?);
        row.suggested = suggest(&labeled, key, settings, decide)?;
    }
    Ok(row)
}

fn count(items: &[Graded<'_>], rule: Rule) -> Result<Counts, MeasureError> {
    let mut counts = Counts {
        n: items.len(),
        ..Counts::default()
    };
    for (answer, want) in items {
        let said = answer.said(rule)?;
        let graded = outcome(&said, want);
        match graded {
            Outcome::Right => counts.right += 1,
            Outcome::Wrong => counts.wrong += 1,
            Outcome::Unresolved => counts.unresolved += 1,
            Outcome::Tied => counts.tied += 1,
        }
        match (graded, said) {
            (Outcome::Right, Said::Yes) => counts.true_yes += 1,
            (Outcome::Right, Said::No) => counts.true_no += 1,
            (Outcome::Wrong, Said::Yes) => counts.false_yes += 1,
            (Outcome::Wrong, Said::No) => counts.false_no += 1,
            _ => {}
        }
    }
    counts.answered = counts.right + counts.wrong;
    let share = |part: usize, whole: usize| (whole > 0).then(|| part as f64 / whole as f64);
    counts.agreement = share(counts.right, counts.answered);
    counts.coverage = share(counts.answered, counts.n);
    let decide = items.first().is_some_and(|(a, _)| a.verb == Verb::Decide);
    counts.yes_recall =
        share(counts.true_yes, counts.true_yes + counts.false_no).filter(|_| decide);
    Ok(counts)
}

fn disagreements(items: &[Graded<'_>], rule: Rule) -> Result<Vec<Disagreement>, MeasureError> {
    let mut counts: BTreeMap<(String, String), usize> = BTreeMap::new();
    for (answer, want) in items {
        let said = answer.said(rule)?;
        if outcome(&said, want) == Outcome::Wrong {
            *counts
                .entry((want.text().to_owned(), said.text().to_owned()))
                .or_default() += 1;
        }
    }
    let mut pairs: Vec<Disagreement> = counts
        .into_iter()
        .map(|((key, said), count)| Disagreement { key, said, count })
        .collect();
    pairs.sort_by_key(|pair| Reverse(pair.count));
    Ok(pairs)
}

/// The cut `k / 100`, which always lies in the range a cut takes.
fn cut(k: u32) -> Threshold {
    Threshold::cut(f64::from(k) / 100.0).unwrap_or_default()
}

/// A rule's number as the band text writes it: `0.45`, `0.3`, `0`, `1`.
fn number(hundredths: u32) -> String {
    if hundredths.is_multiple_of(100) {
        (hundredths / 100).to_string()
    } else {
        python_float_text(f64::from(hundredths) / 100.0)
    }
}

fn coverage(items: &[Graded<'_>], decide: bool) -> Result<Vec<Point>, MeasureError> {
    let first = if decide { 50 } else { 5 };
    (first..=100)
        .step_by(5)
        .map(|k| {
            let at = f64::from(k) / 100.0;
            let (rule, threshold) = if !decide || k == 50 {
                (cut(k), Shown::Cut(at))
            } else {
                let low = f64::from(100 - k) / 100.0;
                let band = Threshold::band(low, at).unwrap_or_default();
                (
                    band,
                    Shown::Band(format!("{}:{}", number(100 - k), number(k))),
                )
            };
            let counts = count(items, Rule::Threshold(rule))?;
            Ok(Point {
                cut: at,
                threshold,
                answered: counts.answered,
                coverage: counts.coverage,
                right: counts.right,
                accuracy: counts.agreement,
            })
        })
        .collect()
}

/// The split: from the key's parts, or seeded when the key gives none.
fn split(
    items: &[Graded<'_>],
    key: &Key,
    seed: u64,
) -> Result<(&'static str, BTreeSet<String>), MeasureError> {
    let ids: BTreeSet<&str> = items.iter().map(|(a, _)| a.id.as_str()).collect();
    let parts: Vec<Option<Part>> = ids.iter().map(|id| key.part(id)).collect();
    if parts.iter().all(Option::is_some) {
        let tune = ids
            .iter()
            .zip(&parts)
            .filter(|(_, part)| **part == Some(Part::Tune))
            .map(|(id, _)| (*id).to_owned());
        return Ok(("key", tune.collect()));
    }
    if parts.iter().any(Option::is_some) {
        return Err(MeasureError::MixedParts);
    }
    let mut ids: Vec<&str> = ids.into_iter().collect();
    shuffle(&mut ids, &mut SplitMix64::new(seed));
    let half = ids.len() / 2;
    let tune = ids.iter().take(half).map(|id| (*id).to_owned());
    Ok(("seeded", tune.collect()))
}

fn suggest(
    items: &[Graded<'_>],
    key: &Key,
    settings: &Settings,
    decide: bool,
) -> Result<Option<Suggested>, MeasureError> {
    let (how, tune_ids) = split(items, key, settings.seed)?;
    if tune_ids.is_empty() {
        return Ok(None);
    }
    let (tune, held): (Vec<Graded<'_>>, Vec<Graded<'_>>) = items
        .iter()
        .cloned()
        .partition(|(a, _)| tune_ids.contains(&a.id));
    let chosen = if decide {
        let mut best = None;
        for k in 1..=99 {
            let right = count(&tune, Rule::Threshold(cut(k)))?.right;
            let score = (right, Reverse(k.abs_diff(50)), Reverse(k));
            if best.as_ref().is_none_or(|held| score > *held) {
                best = Some(score);
            }
        }
        best.map(|(_, _, Reverse(k))| k)
    } else {
        let mut lowest = None;
        for k in 1..=100 {
            let agreement = count(&tune, Rule::Threshold(cut(k)))?.agreement;
            if agreement.unwrap_or(0.0) >= settings.target {
                lowest = Some(k);
                break;
            }
        }
        lowest
    };
    let objective = if decide {
        "most agreement on the tuning part".to_owned()
    } else {
        format!(
            "lowest cut with agreement >= {}",
            python_float_text(settings.target)
        )
    };
    let seed = (how == "seeded").then_some(settings.seed);
    let Some(k) = chosen else {
        return Ok(Some(Suggested {
            cut: None,
            objective,
            split: how,
            seed,
            tune: None,
            held: None,
        }));
    };
    let checked = |part: &[Graded<'_>]| -> Result<Checked, MeasureError> {
        Ok(Checked {
            n: part.len(),
            at_run: count(part, settings.rule)?,
            at_cut: count(part, Rule::Threshold(cut(k)))?,
        })
    };
    Ok(Some(Suggested {
        cut: Some(f64::from(k) / 100.0),
        objective,
        split: how,
        seed,
        tune: Some(checked(&tune)?),
        held: Some(checked(&held)?),
    }))
}
