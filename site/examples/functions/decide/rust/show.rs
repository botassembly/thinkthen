use thinkthen::{Decide, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?;
    // into_answer() fails on not sure or a failed record, so neither
    // reads as false.
    let is_refund: bool = engine
        .run(&refund, "Please refund my order.")?
        .into_answer()?;
    println!("{is_refund}");
    Ok(())
}
