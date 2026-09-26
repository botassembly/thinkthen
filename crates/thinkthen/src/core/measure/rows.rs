//! The row assembly: counts under a rule, the wrong-answer pairs, the coverage
//! curve, and the per-verb measures one audit row carries.

use std::cmp::Reverse;
use std::collections::BTreeMap;

use crate::core::measure::answer::{Answer, Rule, Said, Shown, Verb};
use crate::core::measure::audit::{Counts, Disagreement, Kind, Point, Row, Settings};
use crate::core::measure::items::tally;
use crate::core::measure::key::{Key, Outcome, Want, outcome};
use crate::core::measure::optimize::suggest;
use crate::core::measure::pairs::{curve, pairs, tie_share};
use crate::core::measure::{MeasureError, auc, calibration, python_float_text, python_sum, wilson};
use crate::core::threshold::Threshold;

/// A labeled answer and its key value.
pub(crate) type Graded<'a> = (&'a Answer, Want);

/// One row over a group's answers.
///
/// # Errors
///
/// Returns [`MeasureError`] for a group with two verbs, a rule the answers
/// cannot take, or a key value the answers cannot hold.
pub(crate) fn row(
    group: String,
    kind: Kind,
    members: &[&Answer],
    key: &Key,
    settings: &Settings,
) -> Result<Row, MeasureError> {
    let ok: Vec<&Answer> = members.iter().copied().filter(|a| !a.failed).collect();
    let verb = ok.first().map(|answer| answer.verb);
    if ok.iter().any(|answer| Some(answer.verb) != verb) {
        return Err(MeasureError::TwoVerbs);
    }
    let wants = ok
        .iter()
        .map(|answer| Ok(key.want(answer)?.map(|want| (*answer, want))));
    let labeled: Vec<Graded<'_>> = wants
        .filter_map(Result::transpose)
        .collect::<Result<_, MeasureError>>()?;
    let counts = count(&labeled, settings.rule)?;
    let yes_no = verb.is_some_and(Verb::yes_no);
    let set = verb.is_some_and(Verb::set);
    let pooled = kind == Kind::Pooled;
    let probabilities = !labeled.is_empty() && labeled.iter().all(|(a, _)| a.has_probability());
    let yes_pairs: Vec<(f64, bool)> = if yes_no && probabilities {
        labeled
            .iter()
            .filter_map(|(a, want)| Some((a.confidence()?, *want == Want::Yes)))
            .collect()
    } else {
        Vec::new()
    };
    let direction = |value: usize| (yes_no || set).then_some(value);
    let ties = matches!(verb, Some(Verb::Choose | Verb::Find));
    let shares: Vec<f64> = labeled.iter().map(|(a, want)| tie_share(a, want)).collect();
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
        tied_holding_key: ties.then(|| shares.iter().filter(|held| **held > 0.0).count()),
        tie_share: ties.then(|| python_sum(shares.iter().copied())),
        agreement: counts.agreement,
        interval: wilson(counts.right, counts.answered).filter(|_| !pooled && !set),
        true_yes: direction(counts.true_yes),
        false_yes: direction(counts.false_yes),
        true_no: yes_no.then_some(counts.true_no),
        false_no: direction(counts.false_no),
        yes_recall: counts.yes_recall,
        precision: counts.precision,
        f1: counts.f1,
        mean_probability: (!yes_pairs.is_empty() && !pooled)
            .then(|| python_sum(yes_pairs.iter().map(|pair| pair.0)) / yes_pairs.len() as f64),
        auc: auc(&yes_pairs).filter(|_| !pooled),
        r_precision: (verb == Some(Verb::Rank))
            .then(|| r_precision(&labeled))
            .flatten(),
        mean_level_distance: (verb == Some(Verb::Score))
            .then(|| mean_level_distance(&labeled, settings.rule))
            .transpose()?
            .flatten(),
        disagreements: if yes_no || set {
            None
        } else {
            Some(disagreements(&labeled, settings.rule)?)
        },
        calibration: None,
        coverage: None,
        curve: None,
        suggested: None,
        kind,
        name: ok.first().and_then(|answer| answer.name.clone()),
        band: ok.iter().any(|answer| answer.band),
    };
    if probabilities && !pooled && !set {
        if !matches!(verb, Some(Verb::Rank | Verb::Score)) {
            let pairs = pairs(&labeled)?;
            row.calibration = calibration(&pairs, settings.seed);
            row.curve = settings.curve.then(|| curve(&pairs));
        }
        if verb != Some(Verb::Score) && verb != Some(Verb::Find) {
            row.coverage = Some(coverage(&labeled, yes_no)?);
        }
    }
    if probabilities && !matches!(verb, Some(Verb::Rank | Verb::Find)) {
        row.suggested = suggest(&labeled, key, settings, verb, !pooled)?;
    }
    Ok(row)
}

/// Counts under one rule over labeled answers.
///
/// # Errors
///
/// Returns [`MeasureError`] for a rule an answer cannot take.
pub(crate) fn count(items: &[Graded<'_>], rule: Rule) -> Result<Counts, MeasureError> {
    let mut counts = Counts {
        n: items.len(),
        ..Counts::default()
    };
    for (answer, want) in items {
        if let (Some(said), Want::Items(key, matching)) = (&answer.items, want) {
            let [hit, extra, missed] = tally(said, key, *matching, rule)?;
            (counts.true_yes, counts.right) = (counts.true_yes + hit, counts.right + hit);
            counts.false_yes += extra;
            counts.false_no += missed;
            counts.wrong += extra + missed;
            continue;
        }
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
    measures(&mut counts, items.first().map(|(a, _)| a.verb));
    Ok(counts)
}

/// Two counts over separate answers, added, with every measure taken from the sums.
pub(crate) fn added(a: &Counts, b: &Counts, verb: Option<Verb>) -> Counts {
    let mut sum = Counts {
        n: a.n + b.n,
        right: a.right + b.right,
        wrong: a.wrong + b.wrong,
        unresolved: a.unresolved + b.unresolved,
        tied: a.tied + b.tied,
        true_yes: a.true_yes + b.true_yes,
        false_yes: a.false_yes + b.false_yes,
        true_no: a.true_no + b.true_no,
        false_no: a.false_no + b.false_no,
        ..Counts::default()
    };
    measures(&mut sum, verb);
    sum
}

/// The measures a count object derives from its counts. A `recognize` or
/// `relate` count has no true no, so it has no agreement or coverage.
fn measures(counts: &mut Counts, verb: Option<Verb>) {
    let set = verb.is_some_and(Verb::set);
    counts.answered = counts.right + counts.wrong;
    counts.agreement = share(counts.right, counts.answered).filter(|_| !set);
    counts.coverage = share(counts.answered, counts.n).filter(|_| !set);
    if set || verb.is_some_and(Verb::yes_no) {
        let (hit, false_yes, false_no) = (counts.true_yes, counts.false_yes, counts.false_no);
        counts.yes_recall = share(hit, hit + false_no);
        counts.precision = share(hit, hit + false_yes);
        counts.f1 = share(2 * hit, 2 * hit + false_yes + false_no);
    }
}

/// A part over a whole, or null over nothing.
pub(crate) fn share(part: usize, whole: usize) -> Option<f64> {
    (whole > 0).then(|| part as f64 / whole as f64)
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

/// The share of relevant records among the first R by probability, where R
/// counts the relevant labeled answers. A tie keeps file order.
fn r_precision(items: &[Graded<'_>]) -> Option<f64> {
    let mut order: Vec<&Graded<'_>> = items.iter().collect();
    order.sort_by(|a, b| {
        let p = |item: &Graded<'_>| item.0.confidence().unwrap_or(-1.0);
        p(b).total_cmp(&p(a))
    });
    let relevant = order.iter().filter(|(_, want)| *want == Want::Yes).count();
    let hits = order
        .iter()
        .take(relevant)
        .filter(|(_, want)| *want == Want::Yes)
        .count();
    share(hits, relevant)
}

/// The mean gap in levels between what each labeled `score` answer said and its key.
fn mean_level_distance(items: &[Graded<'_>], rule: Rule) -> Result<Option<f64>, MeasureError> {
    let mut gaps = Vec::new();
    for (answer, want) in items {
        if let (Said::Option(said), Some(key)) = (answer.said(rule)?, answer.level(want.text()))
            && let Some(level) = answer.level(said)
        {
            gaps.push(level.abs_diff(key) as f64);
        }
    }
    Ok((!gaps.is_empty()).then(|| python_sum(gaps.iter().copied()) / gaps.len() as f64))
}

/// The cut `k / 100`, which always lies in the range a cut takes.
pub(crate) fn cut(k: u32) -> Threshold {
    Threshold::cut(f64::from(k) / 100.0).unwrap_or_default()
}

/// A rule's number as the band text writes it: `0.45`, `0.3`, `0`, `1`.
pub(crate) fn number(hundredths: u32) -> String {
    if hundredths.is_multiple_of(100) {
        (hundredths / 100).to_string()
    } else {
        python_float_text(f64::from(hundredths) / 100.0)
    }
}

fn coverage(items: &[Graded<'_>], yes_no: bool) -> Result<Vec<Point>, MeasureError> {
    let first = if yes_no { 50 } else { 5 };
    (first..=100)
        .step_by(5)
        .map(|k| {
            let at = f64::from(k) / 100.0;
            let (rule, threshold) = if !yes_no || k == 50 {
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
