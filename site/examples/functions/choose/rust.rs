use thinkthen::{Description, Engine, Question};

thinkthen::choices! {
    enum Team {
        Billing => "billing",
        Shipping => "shipping",
        Account => "account",
    }
}

let tt = Engine::from_env()?;
let question = "Which team owns this?";
let billing =
    Description::text("Invoices, fees, and refunds.")?;
let shipping = Description::text("Parcels and delivery.")?;
let account = Description::text("Logins and passwords.")?;
let owner = Question::choose::<Team>(question)?
    .option(Team::Billing, Some(billing))?
    .option(Team::Shipping, Some(shipping))?
    .option(Team::Account, Some(account))?
    .cut_at(0.9)?;
let texts = [
    "Please refund the extra fee on my invoice.",
    "My parcel went to the wrong address.",
    "I cannot reset my password.",
    concat!(
        "My parcel never came, and now ",
        "I cannot log in to track it.",
    ),
];
let owners = texts
    .iter()
    .map(|text| {
        tt.choose(&owner, text)
            .map(|call| call.into_value())
    })
    .collect::<Result<Vec<_>, _>>()?;
let expected = [
    Some(Team::Billing),
    Some(Team::Shipping),
    Some(Team::Account),
    None,
];
assert_eq!(owners, expected);
