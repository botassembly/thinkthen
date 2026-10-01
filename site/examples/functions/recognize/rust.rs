use thinkthen::{Engine, Kind, Recognize};

let tt = Engine::from_env()?;
let ask = Recognize::builder()
    .kind(Kind::new("person", None)?)?
    .kind(Kind::new("organization", None)?)?
    .kind(Kind::new("place", None)?)?
    .build()?;
let text = concat!(
    "Maria Chen joined Northwind Freight, ",
    "a company in Chicago.",
);
let facts = tt.recognize(&ask, text)?.into_value();
let names: Vec<_> = facts
    .entities()
    .iter()
    .map(|one| (one.text(), one.kind()))
    .collect();
let expected = [
    ("Maria Chen", "person"),
    ("Northwind Freight", "organization"),
    ("Chicago", "place"),
];
assert_eq!(names, expected);
