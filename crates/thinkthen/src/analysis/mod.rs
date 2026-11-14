//! Offline grading of saved rows. No engine, credentials or transport is created.
mod admission;
mod diff;
pub use diff::{
    CaseDiff, DiffCounts, DiffReport, QuestionMapping, SavedRun, compare, compare_threshold,
};
mod report;
use crate::{AnyRow, Code, Error, Outcome, RecordId, Status};
pub use admission::{CandidateSpan, Gold, GoldCase, Manifest, decode_row};
pub use report::{AuditReport, Bin, Counts, Metric};
use std::collections::BTreeMap;

/// Resource bounds for an offline audit's in-memory case join.
#[derive(Debug, Clone, Copy)]
pub struct AuditOptions {
    /// Number of equal-width probability bins. The default is ten.
    pub bins: std::num::NonZeroUsize,
    /// Maximum expected cases, saved rows and key entries, separately.
    /// The default is 100,000; increase explicitly for larger saved runs.
    pub max_cases: std::num::NonZeroUsize,
    /// Maximum bytes per input document for front ends using bounded reads.
    /// The default is 64 MiB; the typed audit itself accepts admitted values.
    pub max_input_bytes: std::num::NonZeroUsize,
}
impl Default for AuditOptions {
    fn default() -> Self {
        Self {
            bins: std::num::NonZeroUsize::new(10).unwrap_or(std::num::NonZeroUsize::MIN),
            max_input_bytes: std::num::NonZeroUsize::new(64 * 1024 * 1024)
                .unwrap_or(std::num::NonZeroUsize::MIN),
            max_cases: std::num::NonZeroUsize::new(100_000).unwrap_or(std::num::NonZeroUsize::MIN),
        }
    }
}
fn invalid(message: &str) -> Error {
    Error::from_contract(Code::UsageQuestion, message)
}
fn id_key(id: &RecordId) -> Result<String, Error> {
    serde_json::to_string(id).map_err(|_| invalid("the case identity could not be encoded"))
}

/// Grade one explicit question/backend/model group without making a model call.
///
/// Saved rows and labels must belong to the manifest's expected case inventory.
/// Missing results remain in the coverage denominator. Strings and integer IDs
/// are distinct; input order never supplies a missing identity.
/// # Errors
/// Refuses incompatible metadata, duplicate IDs, unsupported functions, invalid
/// gold domains and input that exceeds the configured resource bounds.
pub fn audit(
    manifest: &Manifest,
    rows: impl IntoIterator<Item = AnyRow>,
    gold: impl IntoIterator<Item = GoldCase>,
    options: AuditOptions,
) -> Result<AuditReport, Error> {
    let cases = admit_cases(manifest, rows, gold, options)?;
    evaluate(manifest, &cases, options)
}
type Cases = BTreeMap<String, (Option<AnyRow>, Option<Gold>)>;
fn admit_cases(
    manifest: &Manifest,
    rows: impl IntoIterator<Item = AnyRow>,
    gold: impl IntoIterator<Item = GoldCase>,
    options: AuditOptions,
) -> Result<Cases, Error> {
    manifest.validate(options)?;
    let mut cases = BTreeMap::new();
    for id in &manifest.expected_cases {
        if cases.insert(id_key(id)?, (None, None)).is_some() {
            return Err(invalid(
                "the expected case inventory contains duplicate IDs",
            ));
        }
    }
    let mut seen_rows = 0;
    for row in rows {
        seen_rows += 1;
        if seen_rows > options.max_cases.get() {
            return Err(invalid("saved rows exceed the configured audit case limit"));
        }
        manifest.check_row(&row)?;
        let id = row
            .id()
            .ok_or_else(|| invalid("audit requires stable IDs; supply IDs in the original run"))?;
        let case = cases
            .get_mut(&id_key(id)?)
            .ok_or_else(|| invalid("a saved row is absent from the expected inventory"))?;
        if case.0.replace(row).is_some() {
            return Err(invalid("saved rows contain duplicate case IDs"));
        }
    }
    let mut seen_gold = std::collections::BTreeSet::new();
    for label in gold {
        if seen_gold.len() >= options.max_cases.get() {
            return Err(invalid(
                "key entries exceed the configured audit case limit",
            ));
        }
        if label.question_identity != manifest.question_identity {
            return Err(invalid("a key entry belongs to a different question"));
        }
        let key = id_key(&label.case_id)?;
        if !seen_gold.insert(key.clone()) {
            return Err(invalid("key entries contain duplicate case IDs"));
        }
        let case = cases
            .get_mut(&key)
            .ok_or_else(|| invalid("a key entry is absent from the expected inventory"))?;
        if let Some(gold) = label.resolve(manifest)? {
            gold.validate(manifest)?;
            case.1 = Some(gold);
        }
    }
    Ok(cases)
}
fn evaluate(
    manifest: &Manifest,
    cases: &Cases,
    options: AuditOptions,
) -> Result<AuditReport, Error> {
    let mut report = AuditReport::new(manifest.clone(), options.bins.get());
    let mut binary = Vec::new();
    for (row, label) in cases.values() {
        report.counts.expected_cases += 1;
        if row.is_none() {
            report.counts.missing_results += 1;
        }
        let Some(label) = label else {
            if let Some(row) = row {
                report.counts.unlabeled_results += 1;
                match row.status() {
                    Status::Answered => report.counts.unlabeled_answered += 1,
                    Status::Unsure => report.counts.unlabeled_unsure += 1,
                    Status::Failed => report.counts.unlabeled_failed += 1,
                }
            }
            continue;
        };
        report.counts.labeled_cases += 1;
        if matches!(label, Gold::None(())) {
            report.counts.gold_none += 1;
        }
        let Some(row) = row else { continue };
        let correct = correctness(row, label)?;
        match row.status() {
            Status::Answered => {
                report.counts.answered_labeled += 1;
                if correct == Some(true) {
                    report.counts.correct += 1;
                } else {
                    report.counts.wrong += 1;
                }
                if matches!(row, AnyRow::Find(row) if matches!(row.outcome(), Outcome::Answered { answer: None, .. }))
                {
                    if correct == Some(true) {
                        report.counts.correct_none += 1;
                    } else {
                        report.counts.false_none += 1;
                    }
                }
            }
            Status::Unsure => report.counts.unsure_labeled += 1,
            Status::Failed => report.counts.failed_labeled += 1,
        }
        let Some(probability) = row.probability().map(crate::Probability::get) else {
            report.counts.excluded_missing_probability += 1;
            continue;
        };
        let observed = if let Gold::Boolean(value) = label {
            binary.push((probability, *value));
            Some(*value)
        } else {
            correct
        };
        if let Some(observed) = observed {
            report.add_probability(probability, observed);
        } else {
            report.counts.excluded_unknown_winner += 1;
        }
    }
    report.finish(binary);
    Ok(report)
}
fn correctness(row: &AnyRow, gold: &Gold) -> Result<Option<bool>, Error> {
    if row.status() != Status::Answered {
        return Ok(None);
    }
    match (row, gold) {
        (AnyRow::Decide(row), Gold::Boolean(gold)) => {
            Ok(row.answer().ok().map(|answer| answer == *gold))
        }
        (AnyRow::Filter(row), Gold::Boolean(gold)) => {
            Ok(row.answer().ok().map(|answer| answer == *gold))
        }
        (AnyRow::Choose(row), Gold::Choice(gold)) => {
            Ok(row.answer().ok().map(|answer| answer == gold))
        }
        (AnyRow::Find(row), Gold::None(())) => Ok(row.answer().ok().map(|answer| answer.is_none())),
        (AnyRow::Find(row), Gold::Span { start, end }) => Ok(row.answer().ok().map(|answer| {
            answer
                .as_ref()
                .is_some_and(|answer| answer.start() == *start && answer.end() == *end)
        })),
        _ => Err(invalid(
            "the gold value is incompatible with the row function",
        )),
    }
}
