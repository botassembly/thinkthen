//! The suggested bar: the split, the four measures and their tie rules, the
//! search on the tuning part, and how steady the bar stays across splits.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Serialize, Serializer};

use crate::core::measure::answer::{Rule, Verb};
use crate::core::measure::audit::{Checked, Counts, Settings, Suggested};
use crate::core::measure::key::{Key, Part};
use crate::core::measure::levels::{LevelCuts, search};
use crate::core::measure::rows::{Graded, count, cut, share};
use crate::core::measure::{MeasureError, SplitMix64, python_float_text, shuffle};

/// How many seeded splits `steady` reads.
const SPLITS: u64 = 20;

/// The measure a suggested bar maximizes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Measure {
    /// The most right answers.
    Accuracy,
    /// The most right among the answers that said yes.
    Precision,
    /// The most yes answers found among the records the key calls yes.
    Recall,
    /// The balance of precision and recall.
    F1,
}

impl Measure {
    /// The measure's name as `--optimize` spells it.
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Accuracy => "accuracy",
            Self::Precision => "precision",
            Self::Recall => "recall",
            Self::F1 => "f1",
        }
    }

    /// The measure over counts, or null on a zero denominator.
    pub(crate) fn of(self, counts: &Counts) -> Option<f64> {
        match self {
            Self::Accuracy => share(counts.right, counts.n),
            Self::Precision => counts.precision,
            Self::Recall => counts.yes_recall,
            Self::F1 => counts.f1,
        }
    }

    /// True when cut `k` wins a tie on the measure against cut `held`.
    fn prefers(self, k: u32, held: u32) -> bool {
        match self {
            Self::Recall => k > held,
            Self::Precision => k < held,
            Self::Accuracy | Self::F1 => (k.abs_diff(50), k) < (held.abs_diff(50), held),
        }
    }
}

/// One tuned bar: a cut in hundredths, or level cuts for `score`.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum Bar {
    Cut(u32),
    Levels(LevelCuts),
}

impl Bar {
    /// The rule the bar reads answers under.
    fn rule(self) -> Rule {
        match self {
            Self::Cut(k) => Rule::Threshold(cut(k)),
            Self::Levels(cuts) => Rule::Levels(cuts),
        }
    }

    /// The bar's distance from 0.5 in hundredths; level cuts have none.
    const fn distance(self) -> u32 {
        match self {
            Self::Cut(k) => k.abs_diff(50),
            Self::Levels(_) => 0,
        }
    }

    /// The cut as a number, for a bar that is one.
    pub(crate) fn value(self) -> Option<f64> {
        match self {
            Self::Cut(k) => Some(f64::from(k) / 100.0),
            Self::Levels(_) => None,
        }
    }
}

impl Serialize for Bar {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cut(k) => serializer.serialize_f64(f64::from(*k) / 100.0),
            Self::Levels(cuts) => cuts.serialize(serializer),
        }
    }
}

/// One bar the splits tuned, with how many tuned it.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Tally {
    pub(crate) cut: Bar,
    pub(crate) count: usize,
}

/// How steady the suggested bar stays across splits.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub(crate) struct Steady {
    pub(crate) splits: usize,
    pub(crate) cut: Bar,
    pub(crate) counts: Vec<Tally>,
    pub(crate) better: usize,
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

type Parts<'a> = (Vec<Graded<'a>>, Vec<Graded<'a>>);

fn parts<'a>(items: &[Graded<'a>], tune: &BTreeSet<String>) -> Parts<'a> {
    items
        .iter()
        .cloned()
        .partition(|(a, _)| tune.contains(&a.id))
}

/// The suggested bar for one row, tuned on split 0 and counted over every split.
///
/// # Errors
///
/// Returns [`MeasureError`] for mixed key parts or a rule an answer cannot take.
pub(crate) fn suggest(
    items: &[Graded<'_>],
    key: &Key,
    settings: &Settings,
    verb: Option<Verb>,
) -> Result<Option<Suggested>, MeasureError> {
    let Some(verb) = verb else {
        return Ok(None);
    };
    let (how, tune_ids) = split(items, key, settings.seed)?;
    if tune_ids.is_empty() {
        return Ok(None);
    }
    let measure = settings.optimize;
    let applies = verb.yes_no() || measure == Measure::Accuracy;
    let mut suggested = Suggested {
        cut: None,
        cuts: None,
        objective: objective(verb, measure, settings.target),
        split: how,
        seed: (how == "seeded").then_some(settings.seed),
        tune: None,
        held: None,
        steady: None,
    };
    if !applies {
        suggested.objective = format!("{} does not apply to {}", measure.name(), verb.name());
        return Ok(Some(suggested));
    }
    let mut splits = vec![tune_ids];
    if how == "seeded" {
        for step in 1..SPLITS {
            splits.push(split(items, key, settings.seed.wrapping_add(step))?.1);
        }
    }
    let tuned = splits
        .iter()
        .map(|tune_ids| tune(&parts(items, tune_ids).0, verb, settings))
        .collect::<Result<Vec<_>, _>>()?;
    let (tune_part, held_part) = parts(items, splits.first().unwrap_or(&BTreeSet::new()));
    if let Some(Some(bar)) = tuned.first() {
        let checked = |part: &[Graded<'_>]| -> Result<Checked, MeasureError> {
            Ok(Checked {
                n: part.len(),
                at_run: count(part, settings.rule)?,
                at_cut: count(part, bar.rule())?,
            })
        };
        suggested.cut = bar.value();
        if let Bar::Levels(cuts) = bar {
            suggested.cuts = Some(*cuts);
        }
        suggested.tune = Some(checked(&tune_part)?);
        suggested.held = Some(checked(&held_part)?);
    }
    let mut tallies: BTreeMap<Bar, usize> = BTreeMap::new();
    for bar in tuned.into_iter().flatten() {
        *tallies.entry(bar).or_default() += 1;
    }
    let chosen = tallies
        .iter()
        .max_by_key(|(bar, count)| (**count, Reverse(bar.distance()), Reverse(**bar)))
        .map(|(bar, _)| *bar);
    let mut steady = None;
    if let Some(bar) = chosen {
        let mut better = 0;
        for tune_ids in &splits {
            better += usize::from(beats(&parts(items, tune_ids).1, bar, verb, settings)?);
        }
        let counts = tallies.into_iter().map(|(cut, count)| Tally { cut, count });
        steady = Some(Steady {
            splits: splits.len(),
            cut: bar,
            counts: counts.collect(),
            better,
        });
    }
    suggested.steady = Some(steady);
    Ok(Some(suggested))
}

fn objective(verb: Verb, measure: Measure, target: f64) -> String {
    match verb {
        Verb::Choose => format!("lowest cut with agreement >= {}", python_float_text(target)),
        Verb::Score => "most exact levels on the tuning part".to_owned(),
        _ if measure == Measure::Accuracy => "most agreement on the tuning part".to_owned(),
        _ => format!("most {} on the tuning part", measure.name()),
    }
}

/// The bar one tuning part picks, or none.
fn tune(part: &[Graded<'_>], verb: Verb, settings: &Settings) -> Result<Option<Bar>, MeasureError> {
    if part.is_empty() {
        return Ok(None);
    }
    match verb {
        Verb::Score => {
            let levels = part.first().map_or(0, |(a, _)| a.options.len());
            let cuts = search(levels, |cuts| Ok(count(part, Rule::Levels(*cuts))?.right))?;
            Ok(Some(Bar::Levels(cuts)))
        }
        Verb::Choose => {
            for k in 1..=100 {
                let agreement = count(part, Rule::Threshold(cut(k)))?.agreement;
                if agreement.unwrap_or(0.0) >= settings.target {
                    return Ok(Some(Bar::Cut(k)));
                }
            }
            Ok(None)
        }
        _ => {
            let measure = settings.optimize;
            let mut best: Option<(f64, u32)> = None;
            for k in 1..=99 {
                let Some(score) = measure.of(&count(part, Rule::Threshold(cut(k)))?) else {
                    continue;
                };
                if best.is_none_or(|(held, at)| {
                    score > held || (score == held && measure.prefers(k, at))
                }) {
                    best = Some((score, k));
                }
            }
            Ok(best.map(|(_, k)| Bar::Cut(k)))
        }
    }
}

/// True when the bar scores strictly better than the run's rule on a held part.
fn beats(
    held: &[Graded<'_>],
    bar: Bar,
    verb: Verb,
    settings: &Settings,
) -> Result<bool, MeasureError> {
    let at_bar = count(held, bar.rule())?;
    let at_run = count(held, settings.rule)?;
    Ok(match verb {
        Verb::Score => at_bar.right > at_run.right,
        Verb::Choose => {
            let reach = |counts: &Counts| counts.agreement.is_some_and(|a| a >= settings.target);
            if reach(&at_bar) == reach(&at_run) {
                at_bar.right > at_run.right && at_bar.agreement >= at_run.agreement
            } else {
                reach(&at_bar)
            }
        }
        _ => {
            let measure = settings.optimize;
            let theirs = measure.of(&at_run);
            measure
                .of(&at_bar)
                .is_some_and(|ours| theirs.is_none_or(|theirs| ours > theirs))
        }
    })
}
