use thinkthen::{Decide, Engine, NoAnswer, Threshold};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let refund = Decide::new("Does the customer request a refund?")?
        .threshold(Threshold::band(0.2, 0.8)?);
    let message = "I was charged twice for one invoice.";
    match engine.run(&refund, message)?.into_answer() {
        Ok(is_refund) => println!("{is_refund}"),
        Err(NoAnswer::Unsure { probability }) => println!(
            "Not sure ({}): ask a person.",
            probability.get()
        ),
        Err(NoAnswer::Failed(error)) => {
            println!("{}: {error}", error.code())
        }
    }
    Ok(())
}
