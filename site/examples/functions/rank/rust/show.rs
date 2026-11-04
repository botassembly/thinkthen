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
    for ranked in engine.rows(&by_urgency, messages)? {
        let ranked = ranked?;
        let rank = ranked.answer()?;
        let (_, message) = ranked
            .record()
            .expect("a rank row keeps its record")
            .original();
        println!("{rank} {}", String::from_utf8_lossy(message));
    }
    Ok(())
}
