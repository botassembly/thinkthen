use thinkthen::{Engine, Kind, Recognize};

let tt = Engine::from_env()?;
let ask = Recognize::builder()
    .kind(Kind::new("person", None)?)?
    .kind(Kind::new("product", None)?)?
    .kind(Kind::new("place", None)?)?
    .build()?;
let text = concat!(
    "Sam Patel says his Kindle arrived broken ",
    "in Denver.",
);
let facts = tt.recognize(&ask, text)?.into_value();
let names: Vec<_> = facts
    .entities()
    .iter()
    .map(|one| (one.text(), one.kind()))
    .collect();
let expected = [
    ("Sam Patel", "person"),
    ("Kindle", "product"),
    ("Denver", "place"),
];
assert_eq!(names, expected);
