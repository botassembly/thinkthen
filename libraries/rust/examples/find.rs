//! `find`: the one unit that best answers the question, or none.

use std::io::Write;

use thinkthen::{Engine, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let deadline = Question::find("Which line gives the refund deadline?")?;
    let policy = [
        "Refunds are issued within 30 days of purchase.",
        "Shipping is free on orders over $50.",
    ];
    let found = tt.find(&deadline, policy)?;
    writeln!(std::io::stdout().lock(), "{:?}", found.value().selected())?;
    Ok(())
}
