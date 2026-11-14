use thinkthen::{Engine, Score};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let how_urgent =
        Score::with_strings("How urgent is the request?", ["routine", "urgent"])?;
    // 0 is routine, 1 is urgent.
    let urgency: f64 = engine
        .run(&how_urgent, "Please refund my order.")?
        .into_answer()?;
    println!("{urgency}");
    Ok(())
}
