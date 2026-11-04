use thinkthen::{Engine, Tag};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let engine = Engine::from_env()?;
    let appear = Tag::new("Which topics appear?", ["billing", "shipping"])?;
    let topics: Vec<String> = engine
        .run(&appear, "I was charged twice and my parcel is late again.")?
        .into_answer()?;
    println!("{topics:?}");
    Ok(())
}
