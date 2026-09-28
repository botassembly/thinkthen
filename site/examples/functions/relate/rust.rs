use thinkthen::{Engine, Entity, Relate, RelationRule};

let tt = Engine::from_env()?;
let rules = [
    "Book economy class for every flight under six hours.",
    "Submit receipts within 30 days of the trip.",
    "Hotel stays are capped at 200 dollars a night.",
    "Employees may book business class on any flight.",
    "Rental cars need a manager's approval.",
    concat!(
        "Receipts may be submitted at any time, ",
        "with no deadline.",
    ),
    "Meals are reimbursed up to 60 dollars a day.",
    "Use the company travel portal for all bookings.",
];
let contradicts =
    RelationRule::both_ways("contradicts", "*", "*")?;
let ask = Relate::builder()
    .relation(contradicts)?
    .threshold(0.5)?
    .build()?;
let entities = rules
    .iter()
    .map(|rule| Entity::new(rule, "rule"))
    .collect::<Result<Vec<_>, _>>()?;
let contradictions = tt.relate(&ask, entities)?
    .into_value();

let pairs: Vec<_> = contradictions
    .iter()
    .map(|edge| {
        let source = edge.source().name();
        let target = edge.target().name();
        (source, target, edge.probability())
    })
    .collect();
let expected = [
    (rules[0], rules[3], 0.84),
    (rules[1], rules[5], 0.99),
];
assert_eq!(pairs, expected);
