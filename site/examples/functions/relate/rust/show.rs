use thinkthen::{Engine, Relate, RelationRule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let message = "Morgan works for Example Group.\nPlease refund my order.";
    let works_for = Relate::new([RelationRule::new(
        "person",
        "organization",
        "works_for",
    )?])?;
    let relations = engine.run(&works_for, message)?.into_answer()?;
    for relation in &relations {
        println!(
            "{} --{}--> {}",
            relation.source().text(),
            relation.relation(),
            relation.target().text()
        );
    }
    Ok(())
}
