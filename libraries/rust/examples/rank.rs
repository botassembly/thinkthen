//! `rank`: every record, most likely yes first. Ties keep input order.

use std::io::Write;

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let urgent = Question::rank("Is this urgent?")?;
    let inbox = [
        "Newsletter: our autumn catalog is here.",
        "Our checkout page is down and customers cannot pay.",
    ];
    let mut out = std::io::stdout().lock();
    let ranked = tt.rank(&urgent, inbox)?;
    for one in ranked.results() {
        writeln!(
            out,
            "{:.2} {}",
            one.result().probability().ok_or("missing probability")?,
            one.original()
        )?;
    }
    Ok(())
}
