//! Saved runs use audit admission and compare explicit outcomes without inference.
use super::{
    AuditOptions, AuditReport, Cases, Gold, GoldCase, Manifest, Metric, admit_cases, correctness,
    evaluate, id_key, invalid,
};
use crate::{AnyRow, Error, Outcome, Question, RecordId, Threshold};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// One saved group, retaining every status and original probability.
#[derive(Debug, Clone)]
pub struct SavedRun {
    /// Full original execution metadata and expected inventory.
    pub manifest: Manifest,
    /// Saved current typed rows, including failed and unsure outcomes.
    pub rows: Vec<AnyRow>,
}
/// Explicit authorization to pair two question versions by case ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionMapping {
    /// Left engine question identity.
    pub left: String,
    /// Corresponding right engine question identity.
    pub right: String,
}
/// Case-level transitions preserve missing rows rather than synthesizing none.
#[derive(Debug, Clone, Serialize)]
pub struct CaseDiff {
    /// Exact caller case identity.
    pub case_id: RecordId,
    /// Original left result, absent when missing.
    pub left: Option<AnyRow>,
    /// Original right result, absent when missing.
    pub right: Option<AnyRow>,
    /// True only for a known correct labeled answer; absent without a label.
    pub left_correct: Option<bool>,
    /// True only for a known correct labeled answer; absent without a label.
    pub right_correct: Option<bool>,
}
/// Counts separate paired, missing, status and answer changes.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DiffCounts {
    /// Saved rows present on both sides with matching case identities.
    pub matched: usize,
    /// A saved row exists only on the left.
    pub left_only: usize,
    /// A saved row exists only on the right.
    pub right_only: usize,
    /// Expected case with no saved row on either side.
    pub missing_both: usize,
    /// Paired rows with changed statuses.
    pub changed_status: usize,
    /// Paired answered rows with different typed selections.
    pub changed_answer: usize,
    /// Paired rows with the same status and selection.
    pub unchanged: usize,
    /// Cases with labels on both sides, including missing results.
    pub paired_labeled: usize,
    /// Previously incorrect or unproduced cases that now have correct answers.
    pub gained_correct: usize,
    /// Previously correct cases now incorrect, unproduced, unsure or failed.
    pub lost_correct: usize,
}
/// An offline comparison; per-side quality reports retain audit denominators.
#[derive(Debug, Clone, Serialize)]
pub struct DiffReport {
    /// Report version.
    pub version: u32,
    /// Left quality report and original metadata.
    pub left: AuditReport,
    /// Right quality report and original metadata.
    pub right: AuditReport,
    /// Transition counts.
    pub counts: DiffCounts,
    /// Net correctness change divided by cases labeled on both sides.
    /// Unavailable when no key or paired labels exist.
    pub correctness_change: Metric,
    /// Exact saved outcomes for the union of expected case identities.
    pub cases: Vec<CaseDiff>,
}

/// Compare two actual saved runs; no settings or input files are changed.
/// # Errors
/// Applies all audit admission rules, requires an explicit one-to-one map for
/// differing question identities, and refuses a comparison with no paired rows.
/// Gold remains attached to its declared question identity: a question mapping
/// does not silently transfer truth labels between different questions.
pub fn compare(
    left: SavedRun,
    right: SavedRun,
    gold: Option<Vec<GoldCase>>,
    mapping: &[QuestionMapping],
    options: AuditOptions,
) -> Result<DiffReport, Error> {
    if left.manifest.function == crate::FunctionName::Find
        && right.manifest.function == crate::FunctionName::Find
        && left.manifest.offset_unit != right.manifest.offset_unit
    {
        return Err(invalid("find comparison requires matching offset units"));
    }
    if left.manifest.function == crate::FunctionName::Find
        && right.manifest.function == crate::FunctionName::Find
        && left.manifest.question_identity == right.manifest.question_identity
        && left.manifest.input_representation != right.manifest.input_representation
        && gold.is_some()
    {
        return Err(invalid(
            "labeled find comparison requires matching input representations or distinct mapped question identities with per-side labels",
        ));
    }
    check_mapping(&left.manifest, &right.manifest, mapping)?;
    let (left_gold, right_gold) = partition_gold(&left.manifest, &right.manifest, gold, options)?;
    let left_cases = admit_cases(&left.manifest, left.rows, left_gold, options)?;
    let right_cases = admit_cases(&right.manifest, right.rows, right_gold, options)?;
    report(
        &left.manifest,
        &left_cases,
        &right.manifest,
        &right_cases,
        options,
    )
}
fn check_mapping(
    left: &Manifest,
    right: &Manifest,
    mapping: &[QuestionMapping],
) -> Result<(), Error> {
    let mut left_seen = BTreeSet::new();
    let mut right_seen = BTreeSet::new();
    for pair in mapping {
        if !left_seen.insert(&pair.left) || !right_seen.insert(&pair.right) {
            return Err(invalid("question mapping must be one-to-one"));
        }
    }
    if left.question_identity != right.question_identity
        && !mapping.iter().any(|pair| {
            pair.left == left.question_identity && pair.right == right.question_identity
        })
    {
        return Err(invalid(
            "different question identities require an explicit question mapping",
        ));
    }
    Ok(())
}
fn partition_gold(
    left: &Manifest,
    right: &Manifest,
    gold: Option<Vec<GoldCase>>,
    options: AuditOptions,
) -> Result<(Vec<GoldCase>, Vec<GoldCase>), Error> {
    if left.expected_cases.len() > options.max_cases.get()
        || right.expected_cases.len() > options.max_cases.get()
    {
        return Err(invalid(
            "comparison inventory exceeds the configured case limit",
        ));
    }
    let left_ids = left
        .expected_cases
        .iter()
        .map(id_key)
        .collect::<Result<BTreeSet<_>, _>>()?;
    let right_ids = right
        .expected_cases
        .iter()
        .map(id_key)
        .collect::<Result<BTreeSet<_>, _>>()?;
    let mut seen = BTreeSet::new();
    let mut left_gold = Vec::new();
    let mut right_gold = Vec::new();
    for label in gold.unwrap_or_default() {
        if seen.len() >= options.max_cases.get().saturating_mul(2) {
            return Err(invalid("comparison gold exceeds the configured case limit"));
        }
        let id = id_key(&label.case_id)?;
        if !seen.insert((label.question_identity.clone(), id.clone())) {
            return Err(invalid(
                "comparison gold contains duplicate question/case identities",
            ));
        }
        let on_left = label.question_identity == left.question_identity && left_ids.contains(&id);
        let on_right =
            label.question_identity == right.question_identity && right_ids.contains(&id);
        if !on_left && !on_right {
            return Err(invalid(
                "gold entry belongs to neither compared inventory/question",
            ));
        }
        if on_left {
            left_gold.push(label.clone());
        }
        if on_right {
            right_gold.push(label);
        }
    }
    Ok((left_gold, right_gold))
}
fn report(
    left: &Manifest,
    left_cases: &Cases,
    right: &Manifest,
    right_cases: &Cases,
    options: AuditOptions,
) -> Result<DiffReport, Error> {
    let mut counts = DiffCounts::default();
    let keys = left_cases
        .keys()
        .chain(right_cases.keys())
        .collect::<BTreeSet<_>>();
    let mut cases = Vec::new();
    for key in keys {
        let (left_row, left_gold) = left_cases
            .get(key)
            .map(|(row, gold)| (row.as_ref(), gold.as_ref()))
            .unwrap_or_default();
        let (right_row, right_gold) = right_cases
            .get(key)
            .map(|(row, gold)| (row.as_ref(), gold.as_ref()))
            .unwrap_or_default();
        match (left_row, right_row) {
            (Some(a), Some(b)) => {
                counts.matched += 1;
                if a.status() != b.status() {
                    counts.changed_status += 1;
                } else if selection(a) != selection(b) {
                    counts.changed_answer += 1;
                } else {
                    counts.unchanged += 1;
                }
            }
            (Some(_), None) => counts.left_only += 1,
            (None, Some(_)) => counts.right_only += 1,
            (None, None) => counts.missing_both += 1,
        }
        let labeled = |row: Option<&AnyRow>, gold: Option<&Gold>| -> Result<Option<bool>, Error> {
            gold.map(|gold| {
                row.map(|row| correctness(row, gold))
                    .transpose()
                    .map(|value| value.flatten().unwrap_or(false))
            })
            .transpose()
        };
        let left_correct = labeled(left_row, left_gold)?;
        let right_correct = labeled(right_row, right_gold)?;
        if let (Some(a), Some(b)) = (left_correct, right_correct) {
            counts.paired_labeled += 1;
            counts.gained_correct += usize::from(!a && b);
            counts.lost_correct += usize::from(a && !b);
        }
        cases.push(CaseDiff {
            case_id: serde_json::from_str(key)
                .map_err(|_| invalid("a saved case identity could not be decoded"))?,
            left: left_row.cloned(),
            right: right_row.cloned(),
            left_correct,
            right_correct,
        });
    }
    if counts.matched == 0 {
        return Err(invalid(
            "comparison requires at least one matching saved case",
        ));
    }
    let correctness_change = if counts.paired_labeled == 0 {
        Metric::Unavailable {
            reason: "no cases labeled on both sides",
        }
    } else {
        Metric::Value(
            (counts.gained_correct as f64 - counts.lost_correct as f64)
                / counts.paired_labeled as f64,
        )
    };
    Ok(DiffReport {
        version: 1,
        left: evaluate(left, left_cases, options)?,
        right: evaluate(right, right_cases, options)?,
        counts,
        correctness_change,
        cases,
    })
}
#[derive(PartialEq)]
enum Selection<'a> {
    Boolean(bool),
    Label(&'a str),
    Span(Option<&'a crate::Span>),
}
fn selection(row: &AnyRow) -> Option<Selection<'_>> {
    match row {
        AnyRow::Decide(row) => row.answer().ok().map(Selection::Boolean),
        AnyRow::Filter(row) => row.answer().ok().map(Selection::Boolean),
        AnyRow::Choose(row) => row.answer().ok().map(Selection::Label),
        AnyRow::Find(row) => row.answer().ok().map(Selection::Span),
        _ => None,
    }
}

/// Compare the saved decide/filter probabilities with one new cut or band.
/// # Errors
/// Refuses choose/find and missing nonfailed probabilities; no backend fallback
/// exists. The full question and shared engine threshold rule determine new rows.
pub fn compare_threshold(
    run: SavedRun,
    threshold: Threshold,
    gold: Option<Vec<GoldCase>>,
    options: AuditOptions,
) -> Result<DiffReport, Error> {
    run.manifest.validate(options)?;
    if run.rows.len() > options.max_cases.get() {
        return Err(invalid("comparison rows exceed the configured case limit"));
    }
    let mut right = run.manifest.clone();
    right.question = match right.question {
        Question::Decide(question) => question.threshold(threshold).into(),
        Question::Filter(question) => question.threshold(threshold.filter_cut()?).into(),
        _ => {
            return Err(invalid(
                "threshold-only diff supports decide/filter; saved unsure choose/find rows do not identify their winner",
            ));
        }
    };
    right.question_identity =
        crate::contract_driver::identity(&right.question, right.name.as_deref())?;
    let rows = run
        .rows
        .iter()
        .cloned()
        .map(|row| retune(row, &right.question_identity, threshold))
        .collect::<Result<_, _>>()?;
    let mapped = QuestionMapping {
        left: run.manifest.question_identity.clone(),
        right: right.question_identity.clone(),
    };
    let mut gold = gold;
    if let Some(labels) = &mut gold
        && right.question_identity != run.manifest.question_identity
    {
        let copies = labels
            .iter()
            .map(|label| {
                if label.question_identity != run.manifest.question_identity {
                    return Err(invalid("threshold gold belongs to a different question"));
                }
                let mut copy = label.clone();
                copy.question_identity = right.question_identity.clone();
                Ok(copy)
            })
            .collect::<Result<Vec<_>, _>>()?;
        labels.extend(copies);
    }
    compare(
        run,
        SavedRun {
            manifest: right,
            rows,
        },
        gold,
        &[mapped],
        options,
    )
}
fn retune(row: AnyRow, identity: &str, threshold: Threshold) -> Result<AnyRow, Error> {
    match row {
        AnyRow::Decide(row) => retune_row(row, identity, threshold).map(AnyRow::Decide),
        AnyRow::Filter(row) => retune_row(row, identity, threshold).map(AnyRow::Filter),
        _ => Err(invalid("threshold-only diff supports decide/filter")),
    }
}
fn retune_row<F: crate::results::Function<Answer = bool, Detail = crate::results::NoDetail>>(
    row: crate::Row<F>,
    identity: &str,
    threshold: Threshold,
) -> Result<crate::Row<F>, Error> {
    let outcome = if let Outcome::Failed { error } = row.outcome() {
        Outcome::Failed {
            error: error.clone(),
        }
    } else {
        let probability = row.probability().ok_or_else(|| {
            invalid("threshold-only diff requires each nonfailed row's saved probability")
        })?;
        match threshold.verdict(probability) {
            crate::core::Verdict::Yes => Outcome::Answered {
                answer: true,
                probability: Some(probability),
                detail: None,
            },
            crate::core::Verdict::No => Outcome::Answered {
                answer: false,
                probability: Some(probability),
                detail: None,
            },
            crate::core::Verdict::Unsure => Outcome::Unsure {
                probability,
                detail: None,
            },
        }
    };
    let mut source = row.source().clone();
    source.question_identity = identity.into();
    Ok(crate::Row::new(
        std::num::NonZeroU64::new(row.position()).unwrap_or(std::num::NonZeroU64::MIN),
        row.id().cloned(),
        source,
        outcome,
    ))
}
