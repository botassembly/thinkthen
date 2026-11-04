use thinkthen::{Engine, Recognize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let people_and_groups = Recognize::new(["person", "organization"])?;
    let names = engine
        .run(&people_and_groups, "Morgan works for Example Group.")?
        .into_answer()?;
    for name in &names {
        println!("{}: {}", name.kind(), name.text());
    }
    Ok(())
}
