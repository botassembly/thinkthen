use thinkthen::{Decide, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?;
    let is_refund: bool = engine
        .run(&refund, "Please refund my order.")?
        .into_answer()?;
    println!("{is_refund}");
    Ok(())
}
