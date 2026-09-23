//! Every verb in the ruled shape, under the null backend.
//!
//! The null backend answers with the conformance file's own numbers, so
//! these assertions are the same expectations the conformance runner
//! checks. Run through `./check.sh`, which sets `ENGINE_NULL=1`; a
//! bare `cargo test` names no backend and fails each test by name.

use thinkthen::{
    Annotated, Answer, Engine, ErrorKind, Options, Question, QuestionSet, Row, failed_questions,
    rows_json,
};

mod common;

fn engine() -> Engine {
    Engine::from_env().expect("the stand-in never fails to build")
}

fn default_cut(text: &str) -> Result<Question, thinkthen::Error> {
    // The grammar's default cut, built the way the text form does, because
    // DecideBuilder finishes only with a named cut or band.
    Question::from_json(&format!(r#"{{"decide": "{text}"}}"#))
}

fn refund_text() -> &'static str {
    "I want a refund for order 9"
}

#[test]
fn decide_cut_and_band() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let cut = Question::decide("Does the customer ask for a refund?")?.cut(0.8)?;
    assert_eq!(tt.decide(&cut, refund_text())?, Answer::Yes);
    let band = Question::decide("Does the customer ask for a refund?")?.band(0.2, 0.8)?;
    assert_eq!(tt.decide(&band, refund_text())?, Answer::Yes);
    let mid = Question::decide("Does the customer ask for a refund?")?.band(0.05, 0.99)?;
    assert_eq!(tt.decide(&mid, "maybe later")?, Answer::Unsure);
    Ok(())
}

#[test]
fn text_form_uses_the_grammar_default() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    assert_eq!(
        tt.decide("Does the customer ask for a refund?", refund_text())?,
        Answer::Yes
    );
    assert_eq!(
        tt.decide("Does the customer ask for a refund?", "good morning")?,
        Answer::No
    );
    Ok(())
}

#[test]
fn decide_many_keeps_order() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = default_cut("Does the customer ask for a refund?")?;
    let records = vec![refund_text(), "good morning", "maybe later"];
    let judgments = tt.decide_many(&question, &records)?;
    let probabilities: Vec<f64> = judgments
        .iter()
        .map(|judgment| judgment.probability)
        .collect();
    assert_eq!(probabilities, vec![0.97, 0.03, 0.55]);
    let answers: Vec<Answer> = judgments.iter().map(|judgment| judgment.answer).collect();
    assert_eq!(answers, vec![Answer::Yes, Answer::No, Answer::Yes]);
    Ok(())
}

#[test]
fn choose_picks_the_top_option() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = Question::choose(
        "Which team should handle this?",
        &["the refund desk", "support", "legal"],
    )?
    .build()?;
    let picked = tt.choose(&question, refund_text())?;
    // The null rule weights an option by its own name: a refund-named
    // option carries 0.62 raw and wins the normalized distribution.
    assert_eq!(picked.as_deref(), Some("the refund desk"));
    Ok(())
}

#[test]
fn score_places_on_the_levels() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = Question::score("How strong is the refund claim?", &["low", "mid", "high"])?;
    let scored = tt.score(&question, refund_text())?;
    // The null rule answers refund evidence on three levels with
    // 0.05/0.20/0.75, so the weighted position is 1.70 and the top level
    // is the nearest.
    assert_eq!(scored.nearest, "high");
    assert!((scored.value - 1.70).abs() < 1e-9, "got {}", scored.value);
    Ok(())
}

#[test]
fn tag_names_the_labels_that_held() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = Question::tag("What is this message about?", &["refund", "greeting"])?;
    let labels = tt.tag(&question, refund_text())?;
    assert!(
        labels.contains(&"refund".to_owned()),
        "the refund label holds at 0.97"
    );
    Ok(())
}

#[test]
fn filter_keeps_the_marked_records() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = default_cut("Does the writer ask for a refund?")?;
    let records = vec!["i want a refund now", "good morning", "refund, please"];
    let kept = tt.filter(&question, &records)?;
    assert_eq!(kept, vec!["i want a refund now", "refund, please"]);
    let empty: Vec<&str> = Vec::new();
    assert!(
        tt.filter("Does the writer ask for a refund?", &empty)?
            .is_empty()
    );
    Ok(())
}

#[test]
fn rank_orders_most_likely_first() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = default_cut("Does the writer ask for a refund?")?;
    let records = vec!["good morning", "refund, please", "maybe later"];
    let ranked = tt.rank(&question, &records)?;
    let order: Vec<usize> = ranked.iter().map(|place| place.index).collect();
    assert_eq!(
        order,
        vec![1, 2, 0],
        "0.97, then 0.55, then 0.03, ties aside"
    );
    Ok(())
}

#[test]
fn find_picks_a_unit() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = default_cut("Which line asks for a refund?")?;
    let units = vec!["good morning", "refund, please"];
    let found = tt.find(&question, &units)?;
    assert_eq!(found.index, Some(1), "the refund unit wins");
    Ok(())
}

#[test]
fn annotate_adds_one_field_a_question() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let set = QuestionSet::from_json(
        r#"{
        "version": 1,
        "questions": {
          "refund": {"decide": "Does the customer ask for a refund?"},
          "heat": {"score": "How strong is the claim?", "levels": ["low", "mid", "high"]}
        }
      }"#,
    )?;
    let records = vec![refund_text()];
    let annotated = tt.annotate(&set, &records)?;
    let fields: Vec<&str> = annotated[0].iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(
        fields,
        vec!["refund", "heat"],
        "the set answers in file order (the core parser's own rule)"
    );
    let refund = annotated[0]
        .iter()
        .find(|(name, _)| name == "refund")
        .map(|(_, held)| held.clone());
    assert!(matches!(refund, Some(Annotated::Decision(Answer::Yes))));
    Ok(())
}

#[test]
fn details_carries_the_audit_trail() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let question = default_cut("Does the customer ask for a refund?")?;
    let details = tt.details(&question, refund_text())?;
    assert_eq!(details.answer, Answer::Yes);
    assert!((details.probability - 0.97).abs() < 1e-9);
    assert_eq!(details.model, "jev-latest");
    assert_eq!(details.digest.len(), 64, "the digest is 64 hex figures");
    assert_eq!(details.sends, 1, "one send produced this judgment");
    // 0053: one 64-figure digest a logical request, in construction order.
    assert_eq!(details.requests.len(), 1);
    assert_eq!(details.requests[0].len(), 64);
    // 0054: always present, zero for one good question.
    assert_eq!(details.failed_questions, 0);
    Ok(())
}

#[test]
fn annotate_preserves_the_good_answers_and_marks_the_failed_one()
-> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    // The failed marker needs the compile-time fixture the gate arms.
    assert!(
        fixture_compiled(),
        "the synthetic-partial fixture is not compiled; ./check.sh builds it"
    );
    // The stand-in's one synthesized partial failure (0054): the reply
    // answers one question and omits the last in name order, so its field
    // carries the ruled typed marker while its neighbour answers, and the
    // count helper sees one.
    let tt = engine();
    let set = QuestionSet::from_json(
        r#"{"version":1,"questions":{"refund":{"decide":"Is this a refund request?","threshold":0.5},"topic":{"decide":"Is this a billing problem?","threshold":0.5}}}"#,
    )?;
    let annotated = tt.annotate(&set, &["order 4471: charged twice, please refund"])?;
    let topic = annotated[0]
        .iter()
        .find(|(name, _)| name == "topic")
        .map(|(_, held)| held);
    match topic {
        Some(Annotated::Failed(failed)) => {
            let ruled = serde_json::to_string(&Annotated::Failed(*failed))?;
            assert_eq!(
                ruled,
                r#"{"failed":{"kind":"backend","cause":"missing_answer"}}"#
            );
        }
        other => panic!("expected the failed marker, got {other:?}"),
    }
    let refund = annotated[0]
        .iter()
        .find(|(name, _)| name == "refund")
        .map(|(_, held)| held);
    assert!(matches!(refund, Some(Annotated::Decision(Answer::Yes))));
    assert_eq!(failed_questions(&annotated), 1);
    let clean = tt.annotate(&set, &["I want a refund for order 4471"])?;
    assert_eq!(failed_questions(&clean), 0);
    Ok(())
}

#[test]
fn the_record_row_is_the_ruled_shape() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    // The ruled `{"input","value"}` row (go-ahead item 4) in this host's
    // own type, serialized by the contract's `rows_json`.
    let rows = vec![
        Row { input: "refund now".to_owned(), value: Some(true) },
        Row { input: "good morning".to_owned(), value: Some(false) },
    ];
    assert_eq!(
        rows_json(&rows),
        r#"[{"input":"refund now","value":true},{"input":"good morning","value":false}]"#
    );
    Ok(())
}

#[test]
fn usage_counts_the_sends() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();
    let before = tt.usage().requests;
    let question = default_cut("Does the customer ask for a refund?")?;
    tt.decide(&question, refund_text())?;
    // The counter is process-wide and the suite runs in parallel, so the
    // honest assertion is that this call's send was counted at all.
    assert!(
        tt.usage().requests > before,
        "the send was not counted"
    );
    Ok(())
}

#[test]
fn the_error_kinds_reach_the_caller() -> Result<(), Box<dyn std::error::Error>> {
    common::require_backend();
    let tt = engine();

    let band = Question::decide("Does the writer ask for a refund?")?.band(0.2, 0.8)?;
    let error = tt.filter(&band, &["one", "two"]).unwrap_err();
    assert_eq!(
        error.kind,
        ErrorKind::Usage,
        "a band on filter is a usage error"
    );

    let named = Question::from_json(
        r#"{"decide": "Does the writer ask for a refund?", "threshold": 0.9}"#,
    )?;
    let error = tt.rank(&named, &["one"]).unwrap_err();
    assert_eq!(
        error.kind,
        ErrorKind::Usage,
        "a named threshold on rank is refused"
    );

    let error = tt.annotate("no-such-file.json", &["one"]).unwrap_err();
    assert_eq!(
        error.kind,
        ErrorKind::Local,
        "a missing file is a local error"
    );

    let question = default_cut("Does the customer ask for a refund?")?;
    let spent = Options::new().deadline_in(std::time::Duration::ZERO);
    let error = tt.decide_opts(&question, refund_text(), spent).unwrap_err();
    assert_eq!(
        error.kind,
        ErrorKind::Deadline,
        "a past deadline is its own kind"
    );
    assert!(error.retryable, "a second try carries a fresh budget");
    assert!(
        error.message.contains("the deadline of 0"),
        "the message names the limit that ran out: {}",
        error.message
    );

    let token = thinkthen::Cancel::new();
    token.cancel();
    let error = tt
        .decide_with(&question, refund_text(), Some(&token))
        .unwrap_err();
    assert_eq!(
        error.kind,
        ErrorKind::Cancelled,
        "a fired token cancels the call"
    );
    Ok(())
}

/// Whether this build carries the stand-in's synthesized partial failure.
fn fixture_compiled() -> bool {
    cfg!(feature = "synthetic-partial")
}
