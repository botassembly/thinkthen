#![cfg(test)]
//! Saved-run comparison through the public Rust SDK.
#[path = "../../loopback/src/lib.rs"]
pub mod loopback;
use serde_json::{Value, json};
use thinkthen::analysis::{
    AuditOptions, Gold, GoldCase, Manifest, Metric, QuestionMapping, SavedRun, compare,
    compare_threshold,
};
use thinkthen::{BackendName, Decide, OffsetUnit, RecordId, Settings, Threshold};
fn saved(probability: f64) -> SavedRun {
    let question = Decide::new("Ready?").unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let sample = engine.run(&question, "ready").unwrap();
    let manifest = Manifest {
        version: 1,
        function: sample.function(),
        question: question.into(),
        name: None,
        question_identity: sample.source().question_identity().into(),
        backend: BackendName::Stub,
        model: sample.source().model().into(),
        input_representation: "text/v1".into(),
        offset_unit: OffsetUnit::Utf8,
        expected_cases: vec![RecordId::Integer(1), RecordId::Integer(2)],
    };
    let mut row = serde_json::to_value(sample).unwrap();
    row["id"] = json!(1);
    row["answer"] = json!(probability >= 0.5);
    row["probability"] = json!(probability);
    SavedRun {
        manifest,
        rows: vec![serde_json::from_value(row).unwrap()],
    }
}
#[test]
fn threshold_diff_uses_saved_probability_and_new_question_identity() {
    let run = saved(0.6);
    let gold = GoldCase {
        case_id: RecordId::Integer(1),
        question_identity: run.manifest.question_identity.clone(),
        gold: Some(Gold::Boolean(false)),
        candidates: vec![],
    };
    let report = compare_threshold(
        run.clone(),
        Threshold::cut(0.8).unwrap(),
        Some(vec![gold]),
        AuditOptions::default(),
    )
    .unwrap();
    assert_eq!(report.counts.matched, 1);
    assert_eq!(report.counts.gained_correct, 1);
    assert_eq!(report.counts.lost_correct, 0);
    assert_eq!(report.counts.changed_answer, 1);
    assert_eq!(report.correctness_change, Metric::Value(1.0));
    assert_ne!(
        report.left.manifest.question_identity,
        report.right.manifest.question_identity
    );
    assert_eq!(
        report.cases[0]
            .right
            .as_ref()
            .unwrap()
            .probability()
            .unwrap()
            .get(),
        0.6
    );
    let report = compare_threshold(
        run,
        Threshold::band(0.2, 0.8).unwrap(),
        None,
        AuditOptions::default(),
    )
    .unwrap();
    assert_eq!(report.counts.changed_status, 1);
    assert!(matches!(
        report.correctness_change,
        Metric::Unavailable { .. }
    ));
}
#[test]
fn actual_runs_keep_one_sided_cases_and_require_explicit_cross_question_mapping() {
    let left = saved(0.9);
    let mut right = saved(0.1);
    right.manifest.model = "different-model".into();
    let mut row = serde_json::to_value(&right.rows[0]).unwrap();
    row["source"]["model"] = json!("different-model");
    right.rows = vec![serde_json::from_value(row.clone()).unwrap()];
    row["id"] = json!(2);
    right.rows.push(serde_json::from_value(row).unwrap());
    let report = compare(left.clone(), right, None, &[], AuditOptions::default()).unwrap();
    assert_eq!(report.counts.matched, 1);
    assert_eq!(report.counts.right_only, 1);
    assert_eq!(report.counts.changed_answer, 1);
    let tuned = compare_threshold(
        left.clone(),
        Threshold::cut(0.8).unwrap(),
        None,
        AuditOptions::default(),
    )
    .unwrap();
    let right = SavedRun {
        manifest: tuned.right.manifest.clone(),
        rows: tuned
            .cases
            .into_iter()
            .filter_map(|case| case.right)
            .collect(),
    };
    assert!(
        compare(
            left.clone(),
            right.clone(),
            None,
            &[],
            AuditOptions::default()
        )
        .is_err()
    );
    let mapping = QuestionMapping {
        left: left.manifest.question_identity.clone(),
        right: right.manifest.question_identity.clone(),
    };
    assert!(
        compare(
            left.clone(),
            right.clone(),
            None,
            &[mapping.clone(), mapping.clone()],
            AuditOptions::default()
        )
        .is_err()
    );
    assert!(compare(left, right, None, &[mapping], AuditOptions::default()).is_ok());
}
#[test]
fn zero_pairs_duplicates_and_missing_probabilities_refuse_comparison() {
    let run = saved(0.6);
    let mut other = run.clone();
    other.rows.clear();
    assert!(compare(run.clone(), other, None, &[], AuditOptions::default()).is_err());
    let mut duplicate = run.clone();
    duplicate.rows.push(duplicate.rows[0].clone());
    assert!(compare(run.clone(), duplicate, None, &[], AuditOptions::default()).is_err());
    let mut row = serde_json::to_value(&run.rows[0]).unwrap();
    row.as_object_mut().unwrap().remove("probability");
    // The strict saved-row reader refuses the incomplete result before comparison.
    assert!(serde_json::from_value::<thinkthen::AnyRow>(row).is_err());
}

#[test]
fn actual_choose_and_find_comparisons_do_not_invent_unsure_winners() {
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    for function in ["choose", "find"] {
        let (question, sample): (thinkthen::Question, thinkthen::AnyRow) = if function == "choose" {
            let q = thinkthen::Choose::with_strings("Which?", ["a", "b"]).unwrap();
            let sample = engine.run(&q, "a").unwrap();
            (q.into(), thinkthen::AnyRow::Choose(sample))
        } else {
            let q = thinkthen::Find::new("Which line?").unwrap();
            let sample = engine.run(&q, "x").unwrap();
            (q.into(), thinkthen::AnyRow::Find(sample))
        };
        let mut left = saved(0.6);
        left.manifest.question = question;
        left.manifest.function = sample.function();
        left.manifest.question_identity = sample.source().question_identity().into();
        let mut row = serde_json::to_value(sample).unwrap();
        row["id"] = json!(1);
        row["status"] = json!("answered");
        row["probability"] = json!(0.9);
        row.as_object_mut().unwrap().remove("detail");
        row["answer"] = if function == "choose" {
            json!("a")
        } else {
            Value::Null
        };
        left.rows = vec![serde_json::from_value(row.clone()).unwrap()];
        if function == "find" {
            let mut old = row.clone();
            old["answer"] = json!({"text":"x","start":0,"end":1});
            let mut new = old.clone();
            new["answer"]["text"] = json!("y");
            let mut old_run = left.clone();
            old_run.rows = vec![serde_json::from_value(old).unwrap()];
            let mut new_run = left.clone();
            new_run.rows = vec![serde_json::from_value(new).unwrap()];
            let report = compare(old_run, new_run, None, &[], AuditOptions::default()).unwrap();
            assert_eq!(report.counts.changed_answer, 1);
        }
        let mut right = left.clone();
        row["status"] = json!("unsure");
        row.as_object_mut().unwrap().remove("answer");
        right.rows = vec![serde_json::from_value(row).unwrap()];
        let report = compare(left.clone(), right, None, &[], AuditOptions::default()).unwrap();
        assert_eq!(report.counts.changed_status, 1);
        assert!(
            compare_threshold(
                left,
                Threshold::cut(0.5).unwrap(),
                None,
                AuditOptions::default()
            )
            .is_err()
        );
    }
}

#[test]
fn derived_threshold_outcome_agrees_with_the_real_engine() {
    let server = loopback::Loopback::noul(0.6).unwrap();
    let engine = Settings::default()
        .url(server.base())
        .api_key(thinkthen::Key::new(loopback::KEY).unwrap())
        .engine(|_| None)
        .unwrap();
    for threshold in [
        Threshold::cut(0.8).unwrap(),
        Threshold::band(0.2, 0.8).unwrap(),
    ] {
        let actual = engine
            .run(
                &Decide::new("Ready?").unwrap().threshold(threshold),
                "ready",
            )
            .unwrap();
        let report =
            compare_threshold(saved(0.6), threshold, None, AuditOptions::default()).unwrap();
        let derived = report
            .cases
            .iter()
            .find_map(|case| case.right.as_ref())
            .unwrap();
        let thinkthen::AnyRow::Decide(derived) = derived else {
            panic!("wrong function")
        };
        assert_eq!(actual.outcome(), derived.outcome());
    }
    assert_eq!(server.count(), 2);
}

#[test]
fn find_comparison_refuses_incompatible_unicode_offset_units() {
    let question = thinkthen::Find::new("Which line?").unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let sample = engine.run(&question, "é").unwrap();
    let mut left = saved(0.6);
    left.manifest.question = question.into();
    left.manifest.function = sample.function();
    left.manifest.question_identity = sample.source().question_identity().into();
    let mut row = serde_json::to_value(sample).unwrap();
    row["id"] = json!(1);
    row["status"] = json!("answered");
    row["answer"] = json!({"text":"é","start":0,"end":2});
    left.rows = vec![serde_json::from_value(row.clone()).unwrap()];
    let mut right = left.clone();
    right.manifest.offset_unit = OffsetUnit::CodePoints;
    row["answer"]["end"] = json!(1);
    right.rows = vec![serde_json::from_value(row).unwrap()];
    let label = GoldCase {
        case_id: RecordId::Integer(1),
        question_identity: left.manifest.question_identity.clone(),
        gold: Some(Gold::Span { start: 0, end: 2 }),
        candidates: vec![],
    };
    for gold in [None, Some(vec![label.clone()])] {
        let error = compare(
            left.clone(),
            right.clone(),
            gold,
            &[],
            AuditOptions::default(),
        )
        .unwrap_err();
        assert!(error.to_string().contains("matching offset units"));
    }
    right.manifest.offset_unit = OffsetUnit::Utf8;
    right.manifest.input_representation = "text/v2".into();
    let mut moved = serde_json::to_value(&left.rows[0]).unwrap();
    moved["answer"]["start"] = json!(2);
    moved["answer"]["end"] = json!(4);
    right.rows = vec![serde_json::from_value(moved).unwrap()];
    let report = compare(
        left.clone(),
        right.clone(),
        None,
        &[],
        AuditOptions::default(),
    )
    .unwrap();
    assert_eq!(report.counts.changed_answer, 1);
    let error = compare(left, right, Some(vec![label]), &[], AuditOptions::default()).unwrap_err();
    assert!(error.to_string().contains("matching input representations"));
}
