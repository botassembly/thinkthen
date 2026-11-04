use thinkthen::{Engine, Find};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let message = "Morgan works for Example Group.\nPlease refund my order.";
    let requested = Find::new("Which line states the requested action?")?;
    // None when no line answers the question.
    match engine.run(&requested, message)?.into_answer()? {
        Some(action) => println!("{}", action.text()),
        None => println!("no line answers"),
    }
    Ok(())
}
