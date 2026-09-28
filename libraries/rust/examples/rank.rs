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
    for one in tt.rank(&urgent, inbox)?.into_value() {
        writeln!(out, "{:.2} {}", one.probability(), one.input())?;
    }
    Ok(())
}
