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
            Choose::with_strings(
                "What is the message about?",
                ["billing", "shipping", "other"],
            )?
            .into(),
        ),
        (
            "urgency",
            Score::with_strings("How urgent is the request?", ["routine", "urgent"])?
                .into(),
        ),
    ])?;
    for annotated in engine.rows(&triage, messages)? {
        let entries = annotated?.err_if_failed()?.into_answer()?;
        let is_refund = entries.decide("refund")?;
        let topic = entries.choose::<String>("topic")?;
        let urgency = entries.score("urgency")?;
        println!(
            "{} {} {}",
            is_refund, topic, urgency
        );
    }
    Ok(())
}
