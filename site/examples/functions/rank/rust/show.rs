use thinkthen::{Engine, Rank};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let messages = vec![
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    ];
    let by_urgency =
        Rank::new("How urgent is the request?", ["routine", "urgent"])?;
    // Rank 1 is the most urgent.
    for ranked in engine.rank(&by_urgency, messages, |message| *message)? {
        let (message, row) = ranked?;
        println!("{} {message}", row.answer()?);
    }
    Ok(())
}
