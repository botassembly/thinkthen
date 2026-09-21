//! The slide sample, run exactly as drawn, under the null backend.
//!
//! The sample is the Rust slide in
//! `repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`.
//! The setup above each sample (`ticket`, `reviews`, the two piles) is the
//! slide's context, not its code. Run through `./check.sh`, which sets
//! `ENGINE_NULL=1`; the wire twin lives in `tests/wire.rs`.

use thinkthen::{Answer, Engine, Question};

struct Ticket {
    body: String,
}

#[test]
fn the_slide_runs_as_drawn() -> Result<(), Box<dyn std::error::Error>> {
    // The slide's context.
    let ticket = Ticket { body: "I want a refund for order 9".to_owned() };
    let mut refunds: Vec<Ticket> = Vec::new();
    let mut review: Vec<Ticket> = Vec::new();
    let reviews: Vec<&str> =
        vec!["I want a refund for order 9", "just saying hi", "maybe a refund"];

    // The sample, as drawn.
    let tt = Engine::from_env()?;
    // FINDING 1, filed in NOTES.md: as drawn the slide chains .band on
    // Question::decide's Result, which does not compile; the one-character
    // fix is the ? after the paren, applied here.
    let refund = Question::decide(
        "Does the customer ask for a refund?")?
        .band(0.2, 0.8)?;

    match tt.decide(&refund, &ticket.body)? {
        Answer::Yes => refunds.push(ticket),
        Answer::No => {}
        Answer::Unsure => review.push(ticket),
    }

    let complaints = tt.filter("Is this a complaint?",
        &reviews)?;

    // The answers the stub's rule gives, pinned so a change is a finding.
    // "refund" evidence answers 0.97, which the band calls Yes and the
    // default cut keeps; "just saying hi" answers 0.03 and is dropped.
    assert_eq!(refunds.len(), 1, "the banded refund question answers Yes");
    assert!(review.is_empty(), "no record lands in the unsure pile");
    assert_eq!(complaints, vec!["I want a refund for order 9", "maybe a refund"]);
    Ok(())
}
