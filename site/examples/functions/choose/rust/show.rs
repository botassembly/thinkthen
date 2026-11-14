use thinkthen::{Choose, Engine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let about = Choose::with_strings(
        "What is the message about?",
        ["billing", "shipping", "other"],
    )?;
    let topic = engine
        .run(&about, "Please refund my order.")?;
    println!("{topic}");
    Ok(())
}
