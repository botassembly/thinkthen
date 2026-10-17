//! `filter`: the records whose answer is yes, lazily, in input order.

use std::io::Write;

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let complaint = Question::decide("Is this a complaint?")?.cut();
    let reviews = [
        "Arrived a day early. Thank you!",
        "The zipper broke the first time I used it.",
    ];
    let kept = tt.filter(&complaint, reviews)?;
    let mut out = std::io::stdout().lock();
    for row in kept.into_value() {
        writeln!(out, "{row}")?;
    }
    Ok(())
}
