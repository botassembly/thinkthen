use thinkthen::{Engine, Entity, Relate, RelationRule};

let tt = Engine::from_env()?;
let makes =
    RelationRule::one_way("makes", "company", "product")?;
let ask = Relate::builder().relation(makes)?.build()?;
let names = [
    Entity::new("Apple", "company")?,
    Entity::new("Amazon", "company")?,
    Entity::new("iPhone", "product")?,
    Entity::new("Kindle", "product")?,
];
let who_makes = tt.relate(&ask, names)?;
let pairs: Vec<_> = who_makes
    .value()
    .iter()
    .map(|edge| {
        (edge.source().name(), edge.target().name())
    })
    .collect();
let expected = [
    ("Apple", "iPhone"),
    ("Amazon", "Kindle"),
];
assert_eq!(pairs, expected);
