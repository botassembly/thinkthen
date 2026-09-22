//! The Series door, behind the `polars` feature, under the null backend.
//!
//! Run through `./check.sh`, which sets `ENGINE_NULL=1` and builds this
//! test with the 1.95 toolchain Polars requires. The equality expectation
//! — a column crosses at the same width as a slice — is proven here by
//! equal answers in equal order through the same engine and the same
//! request count; the width itself is the engine's process gate, which
//! experiment 213 measured at 32 in flight for both forms.
#![cfg(feature = "polars")]

use polars::prelude::{DataFrame, NamedFrom, Series};

use thinkthen::{Engine, Question, QuestionSet};

fn engine() -> Engine {
    Engine::from_env().expect("the stand-in never fails to build")
}

fn texts(values: &[&str]) -> Series {
    Series::new("body".into(), values.to_vec())
}

fn bools(series: &Series) -> Vec<Option<bool>> {
    series.bool().expect("a boolean column").iter().collect()
}

#[test]
fn the_decide_column_matches_the_slice() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let records = ["i want a refund now", "good morning", "maybe later"];
    let column = tt.decide_column("Is this a complaint?", &texts(&records))?;
    let listed = tt.decide_many("Is this a complaint?", &records)?;
    let from_slice: Vec<Option<bool>> = listed.iter().map(|one| one.answer.value()).collect();
    assert_eq!(bools(&column), from_slice);
    // Input order is kept, and the answers are the recorded numbers.
    assert_eq!(bools(&column), vec![Some(true), Some(false), Some(true)]);
    Ok(())
}

#[test]
fn a_band_column_carries_nulls() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let band = Question::decide("Does the customer ask for a refund?")?.band(0.2, 0.8)?;
    let column = tt.decide_column(
        &band,
        &texts(&["i want a refund now", "maybe later", "good morning"]),
    )?;
    assert_eq!(bools(&column), vec![Some(true), None, Some(false)]);
    Ok(())
}

#[test]
fn the_score_column_matches_one_call_at_a_time() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let levels = ["Routine.", "Soon.", "Immediate."];
    let question = Question::score("How urgent?", &levels)?;
    let records = ["i want a refund now", "maybe later", "good morning"];
    let column = tt.score_column(&question, &texts(&records))?;
    let numbers = column.f64().expect("a number column");
    let one_by_one: Vec<f64> = records
        .iter()
        .map(|text| {
            tt.score(&question, text)
                .expect("the null backend answers")
                .value
        })
        .collect();
    assert_eq!(
        numbers.iter().collect::<Vec<Option<f64>>>(),
        one_by_one.into_iter().map(Some).collect::<Vec<_>>()
    );
    Ok(())
}

#[test]
fn the_choose_and_tag_columns() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let options = ["the refund desk", "the maybe desk", "anywhere else"];
    let choose = Question::choose("Which desk owns this?", &options)?.build()?;
    let picked = tt.choose_column(
        &choose,
        &texts(&["please route this ticket", "i want a refund now"]),
    )?;
    assert_eq!(
        picked.str().expect("a text column").get(0),
        Some("the refund desk")
    );

    let tag = Question::tag("Which words appear?", &["refund", "shipping"])?;
    let tags = tt.tag_column(
        &tag,
        &texts(&["the refund and the shipping", "good morning"]),
    )?;
    let held = tags.list().expect("a list column");
    assert_eq!(held.len(), 2);
    Ok(())
}

#[test]
fn the_annotate_frame_keeps_the_callers_columns() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let set = QuestionSet::from_json(
        r#"{"version": 1, "questions": {
            "wants_refund": {"decide": "Is this a refund request?", "threshold": 0.5},
            "topic": {"decide": "Is this a billing problem?", "threshold": 0.5}
        }}"#,
    )?;
    let frame = DataFrame::new(
        3,
        vec![
            Series::new("id".into(), [1i64, 2, 3]).into(),
            texts(&["i want a refund now", "good morning", "maybe later"]).into(),
        ],
    )?;
    let out = tt.annotate_frame(&set, &frame, "body")?;
    // The caller's columns first, then one column a question in the set's
    // own name order (the set sorts its names).
    let names: Vec<String> = out
        .get_column_names()
        .into_iter()
        .map(|name| name.to_string())
        .collect();
    assert_eq!(&names[..2], &["id", "body"]);
    let appended: Vec<&str> = names[2..].iter().map(String::as_str).collect();
    let wanted: Vec<&str> = set.names().iter().map(String::as_str).collect();
    assert_eq!(appended, wanted);
    let wants = out
        .column("wants_refund")?
        .as_materialized_series()
        .bool()?;
    assert_eq!(wants.get(0), Some(true));
    assert_eq!(wants.get(1), Some(false));
    // The caller's own columns are untouched.
    assert_eq!(
        out.column("id")?.as_materialized_series().i64()?.get(2),
        Some(3)
    );
    Ok(())
}

#[test]
fn a_null_row_refuses_naming_the_row() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let column = Series::new("body".into(), [Some("i want a refund now"), None]);
    let refused = tt.decide_column("Is this a complaint?", &column);
    let error = refused.expect_err("a null row refuses");
    assert!(error.message.contains("row 1"), "{}", error.message);
    assert_eq!(error.kind, thinkthen::ErrorKind::Usage);
    Ok(())
}

#[test]
fn a_non_text_column_refuses_naming_its_dtype() -> Result<(), Box<dyn std::error::Error>> {
    let tt = engine();
    let column = Series::new("counts".into(), [1i64, 2, 3]);
    let error = tt
        .decide_column("Is this a complaint?", &column)
        .expect_err("a number column refuses");
    assert!(error.message.contains("counts"), "{}", error.message);
    Ok(())
}

#[test]
fn the_request_count_equals_the_slices() -> Result<(), Box<dyn std::error::Error>> {
    // The equality expectation's offline half: a column and a slice make
    // the same number of requests through the same batch spine.
    let tt = engine();
    let records = ["i want a refund now", "good morning", "maybe later"];
    let before = tt.usage().requests;
    let _ = tt.decide_column("Is this a complaint?", &texts(&records))?;
    let column_delta = tt.usage().requests - before;
    let before = tt.usage().requests;
    let _ = tt.decide_many("Is this a complaint?", &records)?;
    let slice_delta = tt.usage().requests - before;
    assert_eq!(column_delta, slice_delta);
    Ok(())
}
