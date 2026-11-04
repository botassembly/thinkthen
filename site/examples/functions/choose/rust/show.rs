use thinkthen::{Choose, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let about = Choose::new(
        "What is the message about?",
        ["billing", "shipping", "other"],
    )?;
    let topic: String = engine
        .run(&about, "Please refund my order.")?
        .into_answer()?;
    println!("{topic}");
    Ok(())
}
