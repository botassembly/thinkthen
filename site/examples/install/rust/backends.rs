use thinkthen::{EngineBuilder, Question};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let typesafe = EngineBuilder::from_env()?
        .backend("typesafe")?
        .build()?;
    let liquid = EngineBuilder::from_env()?
        .backend("liquid")?
        .build()?;
    let ollama = EngineBuilder::from_env()?
        .backend("ollama")?
        .base_url("http://localhost:11535/v1")?
        .build()?;

    let question = "Does the customer ask for a refund?";
    let refund = Question::decide(question)?.cut();
    let broken =
        "Please refund my order. It arrived broken.";
    let thanks = "Thanks for the quick help yesterday!";
    for tt in [typesafe, liquid, ollama] {
        let yes = tt.decide(&refund, broken)?;
        let no = tt.decide(&refund, thanks)?;
        assert_eq!(yes.value(), &Some(true));
        assert_eq!(no.value(), &Some(false));
    }
    Ok(())
}
