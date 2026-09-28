use thinkthen::{Engine, Kind, Recognize, RelationRule};

let tt = Engine::from_env()?;
let text = concat!(
    "Maria Chen joined Northwind Freight, ",
    "a company in Chicago.",
);
let works_for = RelationRule::one_way(
    "works_for",
    "person",
    "organization",
)?;
let based_in = RelationRule::one_way(
    "based_in",
    "organization",
    "place",
)?;
let ask = Recognize::builder()
    .kind(Kind::new("person", None)?)?
    .kind(Kind::new("organization", None)?)?
    .kind(Kind::new("place", None)?)?
    .relation(works_for)?
    .relation(based_in)?
    .build()?;
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

let links: Vec<_> = facts
    .relations()
    .unwrap_or_default()
    .iter()
    .map(|one| {
        let source = one.source().text();
        let target = one.target().text();
        (one.relation(), source, target)
    })
    .collect();
let expected = [
    ("works_for", "Maria Chen", "Northwind Freight"),
    ("based_in", "Northwind Freight", "Chicago"),
];
assert_eq!(links, expected);
