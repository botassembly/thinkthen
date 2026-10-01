use thinkthen::{Engine, Entity, Relate, RelationRule};

let tt = Engine::from_env()?;
let sings =
    RelationRule::one_way("sings", "singer", "song")?;
let ask = Relate::builder().relation(sings)?.build()?;
let names = [
    Entity::new("Paul McCartney", "singer")?,
    Entity::new("Ringo Starr", "singer")?,
    Entity::new("Yesterday", "song")?,
    Entity::new("Octopus's Garden", "song")?,
];
let who_sings = tt.relate(&ask, names)?.into_value();
let pairs: Vec<_> = who_sings
    .iter()
    .map(|edge| {
        (edge.source().name(), edge.target().name())
    })
    .collect();
let expected = [
    ("Paul McCartney", "Yesterday"),
    ("Ringo Starr", "Octopus's Garden"),
];
assert_eq!(pairs, expected);
