//! `choose`: one team for each text, or a person when the pick is unresolved.

use std::io::Write;

use thinkthen::{Engine, Question};

thinkthen::choices! { enum Team { Billing => "billing", Shipping => "shipping", Account => "account" } }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let owner = Question::choose::<Team>("Which team owns this?")?
        .option(Team::Billing, None)?
        .option(Team::Shipping, None)?
        .option(Team::Account, None)?
        .build()?;
    let mut out = std::io::stdout().lock();
    for text in [
        "My card was charged twice.",
        "The courier lost my parcel.",
        "I cannot reset my password.",
    ] {
        // The compiler makes you handle "not sure".
        match tt.choose(&owner, text)? {
            Some(team) => writeln!(out, "{}: {text}", team.label())?,
            None => writeln!(out, "a person: {text}")?,
        }
    }
    Ok(())
}
