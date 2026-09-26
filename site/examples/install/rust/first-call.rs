use thinkthen::{Engine, Question};

thinkthen::choices! {
    enum Team {
        Billing => "billing",
        Shipping => "shipping",
        Account => "account",
    }
}

let tt = Engine::from_env()?;
let question = "Which team owns this?";
let owner = Question::choose::<Team>(question)?
    .option(Team::Billing, None)?
    .option(Team::Shipping, None)?
    .option(Team::Account, None)?
    .build()?;

let texts = [
    "My card was charged twice.",
    "The courier lost my parcel.",
    "I cannot reset my password.",
];
let teams = texts
    .iter()
    .map(|text| tt.choose(&owner, text))
    .collect::<Result<Vec<_>, _>>()?;
let expected = [
    Some(Team::Billing),
    Some(Team::Shipping),
    Some(Team::Account),
];
assert_eq!(teams, expected);
