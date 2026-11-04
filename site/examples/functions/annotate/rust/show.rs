use thinkthen::{AnnotatableQuestion, Annotate, Choose, Decide, Engine, Score};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let messages = vec![
        "Please refund my order.",
        "Where is my parcel?",
        "I was charged twice for one invoice.",
    ];
    let triage = Annotate::new([
        (
            "refund",
            AnnotatableQuestion::from(Decide::new(
                "Does the customer request a refund?",
            )?),
        ),
        (
            "topic",
            Choose::new(
                "What is the message about?",
                ["billing", "shipping", "other"],
            )?
            .into(),
        ),
        (
            "urgency",
            Score::new("How urgent is the request?", ["routine", "urgent"])?
                .into(),
        ),
    ])?;
    for annotated in engine.rows(&triage, messages)? {
        // Each answer prints bare, or as unsure or failed with its code.
        let answers = annotated?.into_answer()?;
        println!(
            "{} {} {}",
            answers["refund"], answers["topic"], answers["urgency"]
        );
    }
    Ok(())
}
