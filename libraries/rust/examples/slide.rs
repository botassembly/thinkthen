//! The deck's Rust slide: build a question once, and the band's middle is unsure.

use std::io::Write;

use thinkthen::{Answer, Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let refund = Question::decide("Does the customer ask for a refund?")?.band(0.2, 0.8)?;
    let mut out = std::io::stdout().lock();
    match tt.decide(&refund, "I want to send this back.")?.into_value() {
        Answer::Unsure => writeln!(out, "a person reads it")?,
        other => writeln!(out, "{other:?}")?,
    }
    Ok(())
}
