#![cfg(test)]
//! Saved-answer analysis through the public Rust SDK.
use thinkthen::analysis::{AuditOptions, Gold, GoldCase, Manifest, Metric, audit, decode_row};
use thinkthen::{BackendName, Decide, OffsetUnit, Question, RecordId, Settings};

fn manifest() -> Manifest {
    let question: Question = Decide::new("Is it a refund?").unwrap().into();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let row = engine
        .run(&Decide::new("Is it a refund?").unwrap(), "refund")
        .unwrap();
    Manifest {
        version: 1,
        function: question.function(),
        question,
        name: None,
        question_identity: row.source().question_identity().into(),
        backend: BackendName::Stub,
        model: row.source().model().into(),
        input_representation: "text/v1".into(),
        offset_unit: OffsetUnit::Utf8,
        expected_cases: (1..=6).map(RecordId::Integer).collect(),
    }
}
fn row(manifest: &Manifest, id: i64, status: &str, rest: &str) -> thinkthen::AnyRow {
    decode_row(&format!(r#"{{"function":"decide","position":1,"id":{id},"source":{{"question_identity":"{}","backend":"stub","model":"{}"}},"status":"{status}"{rest}}}"#,manifest.question_identity,manifest.model)).unwrap()
}
fn gold(manifest: &Manifest, id: i64, value: bool) -> GoldCase {
    GoldCase {
        case_id: RecordId::Integer(id),
        question_identity: manifest.question_identity.clone(),
        gold: Some(Gold::Boolean(value)),
        candidates: Vec::new(),
    }
}
#[test]
fn offline_audit_keeps_denominators_and_binary_probability_meaning() {
    let manifest = manifest();
    let rows = vec![
        row(&manifest, 3, "unsure", ",\"probability\":0.5"),
        row(
            &manifest,
            2,
            "answered",
            ",\"answer\":false,\"probability\":0.2",
        ),
        row(
            &manifest,
            1,
            "answered",
            ",\"answer\":true,\"probability\":0.9",
        ),
        row(
            &manifest,
            4,
            "failed",
            ",\"error\":{\"code\":\"record.missing\",\"message\":\"missing\"}",
        ),
        row(
            &manifest,
            6,
            "answered",
            ",\"answer\":true,\"probability\":1.0",
        ),
    ];
    let labels = vec![
        gold(&manifest, 1, true),
        gold(&manifest, 2, false),
        gold(&manifest, 3, true),
        gold(&manifest, 4, false),
        gold(&manifest, 5, true),
    ];
    let report = audit(&manifest, rows, labels, AuditOptions::default()).unwrap();
    assert_eq!(report.counts.labeled_cases, 5);
    assert_eq!(report.counts.correct, 2);
    assert_eq!(report.counts.missing_results, 1);
    assert_eq!(report.counts.unlabeled_results, 1);
    assert_eq!(report.counts.unsure_labeled, 1);
    assert_eq!(report.counts.failed_labeled, 1);
    assert_eq!(report.coverage, Metric::Value(0.4));
    assert_eq!(report.answered_accuracy, Metric::Value(1.0));
    assert_eq!(report.auc, Metric::Value(1.0));
    assert_eq!(report.bins[2].observed_rate, Metric::Value(0.0));
    assert_eq!(report.bins[5].observed_rate, Metric::Value(1.0));
}
#[test]
fn exact_ids_duplicates_and_question_identity_are_checked() {
    let manifest = manifest();
    let a = row(
        &manifest,
        1,
        "answered",
        ",\"answer\":true,\"probability\":0.9",
    );
    assert!(audit(&manifest, [a.clone(), a], [], AuditOptions::default()).is_err());
    let mut wrong = manifest.clone();
    wrong.question_identity = "wrong".into();
    assert!(audit(&wrong, [], [], AuditOptions::default()).is_err());
    let a = row(
        &manifest,
        1,
        "answered",
        ",\"answer\":true,\"probability\":0.9",
    );
    let mut label = gold(&manifest, 1, true);
    label.case_id = RecordId::String("1".into());
    assert!(audit(&manifest, [a], [label], AuditOptions::default()).is_err());
}
#[test]
fn explicit_null_gold_is_not_an_absent_label() {
    let label: GoldCase =
        serde_json::from_str(r#"{"case_id":1,"question_identity":"q","gold":null}"#).unwrap();
    assert_eq!(label.gold, Some(Gold::None(())));
    let missing: GoldCase =
        serde_json::from_str(r#"{"case_id":1,"question_identity":"q"}"#).unwrap();
    assert_eq!(missing.gold, None);
}

#[test]
fn binary_auc_gives_half_credit_to_ties_and_no_value_for_one_class() {
    let manifest = manifest();
    let rows = [
        row(
            &manifest,
            1,
            "answered",
            ",\"answer\":true,\"probability\":0.5",
        ),
        row(
            &manifest,
            2,
            "answered",
            ",\"answer\":true,\"probability\":0.5",
        ),
    ];
    let report = audit(
        &manifest,
        rows.clone(),
        [gold(&manifest, 1, true), gold(&manifest, 2, false)],
        AuditOptions::default(),
    )
    .unwrap();
    assert_eq!(report.auc, Metric::Value(0.5));
    let report = audit(
        &manifest,
        rows,
        [gold(&manifest, 1, true), gold(&manifest, 2, true)],
        AuditOptions::default(),
    )
    .unwrap();
    assert!(matches!(report.auc, Metric::Unavailable { .. }));
    let report = audit(&manifest, [], [], AuditOptions::default()).unwrap();
    assert!(matches!(report.coverage, Metric::Unavailable { .. }));
}

#[test]
fn find_none_false_none_and_unknown_winners_remain_distinct() {
    let question = thinkthen::Find::new("Which line fits?").unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let sample = engine.run(&question, "first\nsecond").unwrap();
    let mut manifest = manifest();
    manifest.question = question.into();
    manifest.function = manifest.question.function();
    manifest.question_identity = sample.source().question_identity().into();
    // Find's explicit-null answer is intentionally distinct from missing answer.
    let raw = |id, status, rest| {
        decode_row(&format!(r#"{{"function":"find","position":1,"id":{id},"source":{{"question_identity":"{}","backend":"stub","model":"{}"}},"status":"{status}"{rest}}}"#,manifest.question_identity,manifest.model)).unwrap()
    };
    let rows = [
        raw(1, "answered", ",\"answer\":null,\"probability\":0.9"),
        raw(2, "answered", ",\"answer\":null,\"probability\":0.8"),
        raw(3, "unsure", ",\"probability\":0.4"),
    ];
    let label = |id, gold| GoldCase {
        case_id: RecordId::Integer(id),
        question_identity: manifest.question_identity.clone(),
        gold: Some(gold),
        candidates: Vec::new(),
    };
    let report = audit(
        &manifest,
        rows,
        [
            label(1, Gold::None(())),
            label(2, Gold::Span { start: 0, end: 5 }),
            label(3, Gold::None(())),
        ],
        AuditOptions::default(),
    )
    .unwrap();
    assert_eq!(report.counts.gold_none, 2);
    assert_eq!(report.counts.correct_none, 1);
    assert_eq!(report.counts.false_none, 1);
    assert_eq!(report.counts.unsure_labeled, 1);
    assert_eq!(report.counts.excluded_unknown_winner, 1);
    assert_eq!(report.bins[8].observed_rate, Metric::Value(0.0));
    assert_eq!(report.bins[9].observed_rate, Metric::Value(1.0));
}

#[test]
fn caller_items_are_admitted_and_retired_filter_envelopes_are_refused() {
    let question = thinkthen::Filter::new("Keep it?").unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let request = thinkthen::Request::new(question);
    let call = engine
        .open_request(&request, thinkthen::OffsetUnit::Utf8)
        .unwrap();
    let sender = call.sender();
    sender.send(thinkthen::Record::text("first")).unwrap();
    sender.close();
    let sample = call.rows().next().unwrap().unwrap();
    let kept = matches!(&sample, thinkthen::AnyRow::Filter(row) if row.answer().is_ok_and(|answer| answer));
    let row = serde_json::to_string(&sample).unwrap();
    let envelope = format!(
        "{},\"item\":{{\"text\":\"first\"}},\"kept\":{}}}",
        row.strip_suffix('}').unwrap(),
        kept
    );
    assert!(decode_row(&envelope).is_err());
    assert!(serde_json::from_str::<thinkthen::AnyRow>(&envelope).is_err());
    let with_item = format!(
        "{},\"item\":{{\"text\":\"first\"}}}}",
        row.strip_suffix('}').unwrap()
    );
    assert!(decode_row(&with_item).is_ok());
    assert!(serde_json::from_str::<thinkthen::AnyRow>(&with_item).is_ok());
}

#[test]
fn explicit_candidate_map_grades_identity_without_matching_display_text() {
    let question = thinkthen::Find::new("Which line fits?").unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let sample = engine.run(&question, "same\nsame").unwrap();
    let mut manifest = manifest();
    manifest.question = question.into();
    manifest.function = manifest.question.function();
    manifest.question_identity = sample.source().question_identity().into();
    let json = format!(
        r#"{{"function":"find","position":1,"id":1,"source":{{"question_identity":"{}","backend":"stub","model":"{}"}},"status":"answered","answer":{{"text":"same","start":5,"end":9}},"probability":1.0}}"#,
        manifest.question_identity, manifest.model
    );
    let row = decode_row(&json).unwrap();
    let key = GoldCase {
        case_id: RecordId::Integer(1),
        question_identity: manifest.question_identity.clone(),
        gold: Some(Gold::Candidate {
            candidate_id: RecordId::String("second".into()),
        }),
        candidates: vec![
            thinkthen::analysis::CandidateSpan {
                id: RecordId::String("first".into()),
                start: 0,
                end: 4,
            },
            thinkthen::analysis::CandidateSpan {
                id: RecordId::String("second".into()),
                start: 5,
                end: 9,
            },
        ],
    };
    let report = audit(
        &manifest,
        [row.clone()],
        [key.clone()],
        AuditOptions::default(),
    )
    .unwrap();
    assert_eq!(report.counts.correct, 1);
    assert_eq!(report.bins[9].count, 1);
    let mut ambiguous = key.clone();
    ambiguous.candidates[1].start = ambiguous.candidates[0].start;
    ambiguous.candidates[1].end = ambiguous.candidates[0].end;
    assert!(
        audit(
            &manifest,
            [row.clone()],
            [ambiguous],
            AuditOptions::default()
        )
        .is_err()
    );
    let mut duplicate = key;
    duplicate.candidates.push(duplicate.candidates[0].clone());
    assert!(audit(&manifest, [row], [duplicate], AuditOptions::default()).is_err());
    let duplicate_field = json.replace("\"id\":1", "\"id\":2,\"id\":1");
    assert!(decode_row(&duplicate_field).is_err());
}

#[test]
fn choose_grades_only_labels_in_the_admitted_domain() {
    let question = thinkthen::Choose::with_strings("Which?", ["first", "second"]).unwrap();
    let engine = Settings::default()
        .with_backend(BackendName::Stub)
        .engine(|_| None)
        .unwrap();
    let sample = engine.run(&question, "first").unwrap();
    let mut manifest = manifest();
    manifest.question = question.into();
    manifest.function = manifest.question.function();
    manifest.question_identity = sample.source().question_identity().into();
    let mut value = serde_json::to_value(&sample).unwrap();
    value["id"] = serde_json::json!(1);
    value["answer"] = serde_json::json!("first");
    value["probability"] = serde_json::json!(0.9);
    let row = decode_row(&value.to_string()).unwrap();
    let mut label = GoldCase {
        case_id: RecordId::Integer(1),
        question_identity: manifest.question_identity.clone(),
        gold: Some(Gold::Choice("first".into())),
        candidates: Vec::new(),
    };
    let report = audit(
        &manifest,
        [row.clone()],
        [label.clone()],
        AuditOptions::default(),
    )
    .unwrap();
    assert_eq!(report.counts.correct, 1);
    assert_eq!(report.bins[9].observed_rate, Metric::Value(1.0));
    assert!(matches!(report.auc, Metric::Unavailable { .. }));
    label.gold = Some(Gold::Choice("not-an-option".into()));
    assert!(audit(&manifest, [row], [label], AuditOptions::default()).is_err());
}
