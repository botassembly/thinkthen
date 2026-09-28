//! `relate`: the edges among entities you already hold, in rule order.

use std::io::Write;

use thinkthen::{Engine, Entity, Relate, RelationRule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tt = Engine::from_env()?;
    let ask = Relate::builder()
        .relation(RelationRule::both_ways("contradicts", "*", "*")?)?
        .build()?;
    let rules = [
        Entity::new("Book economy class for every flight.", "rule")?,
        Entity::new("Employees may book business class on any flight.", "rule")?,
    ];
    let mut out = std::io::stdout().lock();
    for edge in tt.relate(&ask, rules)? {
        let (name, probability) = (edge.relation(), edge.probability());
        let (source, target) = (edge.source().name(), edge.target().name());
        writeln!(out, "{name} {probability:.2} {source} | {target}")?;
    }
    Ok(())
}
