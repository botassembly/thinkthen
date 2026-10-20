//! `rank`: every record, most likely yes first. Ties keep input order.

use std::io::Write;

mod support;
use thinkthen::{RequestEnvironment, RequestValue};

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let urgent = Question::rank("Is this urgent?")?;
    let inbox = [
        "Newsletter: our autumn catalog is here.",
        "Our checkout page is down and customers cannot pay.",
    ];
    let mut out = std::io::stdout().lock();
    let ranked = tt.call(
        "rank",
        support::arguments(urgent, support::records(inbox)),
        RequestEnvironment::default(),
    )?;
    let RequestValue::Ranked(rows) = ranked.value() else {
        return Err("expected ranked records".into());
    };
    for one in rows {
        writeln!(
            out,
            "{:.2} {}",
            one.result().probability().ok_or("missing probability")?,
            support::original(one.original())?
        )?;
    }
    Ok(())
}
