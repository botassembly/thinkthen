//! `recognize`: the names of each kind in a text, and the relations among them.

use std::io::Write;

use thinkthen::{Engine, Kind, Recognize, RelationRule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let works_for = RelationRule::one_way("works_for", "person", "organization")?;
    let ask = Recognize::builder()
        .kind(Kind::new("person", None)?)?
        .kind(Kind::new("organization", None)?)?
        .relation(works_for)?
        .build()?;
    let text = "Maria Chen joined Northwind Freight last spring.";
    let found = tt.recognize(&ask, text)?;
    let mut out = std::io::stdout().lock();
    for name in found.entities() {
        writeln!(out, "{} {}", name.text(), name.kind())?;
    }
    for link in found.relations().unwrap_or_default() {
        let (source, target) = (link.source().text(), link.target().text());
        writeln!(out, "{} {source} {target}", link.relation())?;
    }
    Ok(())
}
