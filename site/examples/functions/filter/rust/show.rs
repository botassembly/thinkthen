use thinkthen::{Engine, Filter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let messages = vec![
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    ];
    let refund = Filter::new("Does the customer request a refund?")?;
    // Only the messages that answer yes.
    // A failed message arrives as an Err in its place.
    for refund_request in engine.filter(&refund, messages, |message| *message)?
    {
        println!("{}", refund_request?);
    }
    Ok(())
}
