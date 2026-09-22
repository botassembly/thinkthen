//! The slide sample on the wire, plus the shapes only the wire proves.
//!
//! Run by `./check.sh` when the stub is up on the surface's port, with
//! `ENGINE_BASE_URL` pointing at it, the same pattern as the stand-in's
//! wire suite. When neither `ENGINE_NULL` nor `ENGINE_BASE_URL` is set the
//! tests skip with a note, so a bare `cargo test` passes.

use thinkthen::{Answer, Cancel, Engine, ErrorKind, Options, Question};

fn engine() -> Engine {
    Engine::from_env().expect("the stand-in never fails to build")
}

fn stub_is_configured() -> bool {
    std::env::var_os("ENGINE_BASE_URL").is_some()
        || std::env::var_os("THINKTHEN_BASE_URL").is_some()
}

struct Ticket {
    body: String,
}

#[test]
fn the_slide_runs_as_drawn_on_the_wire() -> Result<(), Box<dyn std::error::Error>> {
    if !stub_is_configured() {
        eprintln!("wire test skipped: no stub is named; run ./check.sh with the stub up");
        return Ok(());
    }
    let ticket = Ticket {
        body: "I want a refund for order 9".to_owned(),
    };
    let mut refunds: Vec<Ticket> = Vec::new();
    let mut review: Vec<Ticket> = Vec::new();
    let reviews: Vec<&str> = vec![
        "I want a refund for order 9",
        "just saying hi",
        "maybe a refund",
    ];

    let tt = engine();
    // FINDING 1, filed in NOTES.md: as drawn the slide chains .band on
    // Question::decide's Result, which does not compile; the one-character
    // fix is the ? after the paren, applied here.
    let refund = Question::decide("Does the customer ask for a refund?")?.band(0.2, 0.8)?;

    match tt.decide(&refund, &ticket.body)? {
        Answer::Yes => refunds.push(ticket),
        Answer::No => {}
        Answer::Unsure => review.push(ticket),
    }

    let complaints = tt.filter("Is this a complaint?", &reviews)?;

    assert_eq!(refunds.len(), 1);
    assert!(review.is_empty());
    assert_eq!(
        complaints,
        vec!["I want a refund for order 9", "maybe a refund"]
    );
    Ok(())
}

#[test]
fn a_cancelled_bulk_call_returns_the_cancelled_kind() -> Result<(), Box<dyn std::error::Error>> {
    if !stub_is_configured() {
        eprintln!("wire test skipped: no stub is named; run ./check.sh with the stub up");
        return Ok(());
    }
    let tt = engine();
    let question = Question::from_json(r#"{"decide": "Does the writer ask for a refund?"}"#)?;
    let records: Vec<&str> = vec!["refund now"; 40];
    let token = Cancel::new();
    let ticks = std::cell::Cell::new(0u32);
    let mut polls = 0u32;
    let error = tt
        .decide_many_opts(
            &question,
            &records,
            Options::new().maybe_cancel(Some(&token)),
            Some(&mut || {
                polls += 1;
                ticks.set(ticks.get() + 1);
                if ticks.get() == 2 {
                    token.cancel();
                }
            }),
        )
        .unwrap_err();
    assert_eq!(error.kind, ErrorKind::Cancelled);
    assert!(polls >= 2, "the poll callback ran on the calling thread");
    Ok(())
}

#[test]
fn a_refused_request_is_a_backend_error_that_is_not_retryable()
-> Result<(), Box<dyn std::error::Error>> {
    if !stub_is_configured() {
        eprintln!("wire test skipped: no stub is named; run ./check.sh with the stub up");
        return Ok(());
    }
    // The stub refuses "malformed" evidence with HTTP 422, which is a
    // backend answer a second try cannot fix. The dead-address shape needs
    // an engine value that honors Settings.address; the stand-in does not,
    // and that gap is filed in NOTES.md.
    let tt = engine();
    let question = Question::from_json(r#"{"decide": "Does the customer ask for a refund?"}"#)?;
    let error = tt.decide(&question, "malformed").unwrap_err();
    assert_eq!(error.kind, ErrorKind::Backend);
    assert!(!error.retryable, "a refusal is not worth a second try");
    Ok(())
}
